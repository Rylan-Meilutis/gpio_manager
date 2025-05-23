use crate::pwm_module::PWMManager;
use crate::{check_pwm_values, compute_pwm_values, Callback, InternPullResistorState, LogicLevel, Pin, PinManager, PinState, PinType, PwmConfig, TriggerEdge};
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::PyObject;
use pyo3::{pyclass, pymethods, Py, PyErr, PyResult, Python};
use rppal::gpio::{Gpio, Trigger};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, SystemTime, UNIX_EPOCH};


// Singleton instance of GPIOManager
static GPIO_MANAGER: Lazy<Arc<Mutex<GPIOManager>>> = Lazy::new(|| {
    Arc::new(Mutex::new(GPIOManager::new_singleton().expect("Failed to initialize GPIOManager")))
});


#[pyclass]
/// GPIOManager provides methods to manage GPIO pins and register callbacks.
///
/// Example usage in Python:
///
///
/// ```manager = gpio_manager.GPIOManager()```
///
/// ```manager.add_input_pin(18, gpio_manager.IPinState.PULLUP)```
///
/// ```manager.assign_callback(18, gpio_manager.TriggerEdge.FALLING, button_callback)```
///
/// ```manager.set_output_pin(25, gpio_manager.OPinState.HIGH)```
///
pub struct GPIOManager {
    gpio: Arc<Mutex<PinManager>>,
}


impl GPIOManager {
    /// Internal method to initialize the GPIOManager singleton.
    fn new_singleton() -> PyResult<Self> {
        Ok(Self {
            gpio: Arc::new(Mutex::new(PinManager {
                input_pins: HashMap::new(),
                output_pins: HashMap::new(),
                callbacks: HashMap::new(),
                pwm_setup: HashMap::new(),
            })),
        })
    }
    pub fn get_manager(&self) -> Arc<Mutex<PinManager>> {
        Arc::clone(&self.gpio)
    }

    fn shared(py: Python) -> PyResult<Py<GPIOManager>> {
        let manager = GPIO_MANAGER.lock().unwrap();

        Py::new(py, GPIOManager {
            gpio: Arc::clone(&manager.gpio),
        })
    }

    pub fn new_rust_reference() -> GPIOManager {
        let manager = GPIO_MANAGER.lock().unwrap();
        GPIOManager {
            gpio: Arc::clone(&manager.gpio),
        }
    }

    pub fn is_input_pin(&self, pin_num: u8, manager: &MutexGuard<PinManager>) -> bool {
        manager.input_pins.get(&pin_num).is_some()
    }

    pub fn is_output_pin(&self, pin_num: u8, manager: &MutexGuard<PinManager>) -> bool {
        manager.output_pins.get(&pin_num).is_some()
    }

    fn set_pwm(&self, pwm_pin: u8) -> PyResult<()> {
        let manager = self.gpio.lock().unwrap();
        if let Some(pwm_config) = manager.pwm_setup.get(&pwm_pin) {
            let pin;
            let opc = manager.output_pins.get(&pwm_pin).unwrap().lock().unwrap().pin.clone();
            if let PinType::Output(output_pin_arc) = opc {
                pin = output_pin_arc;
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in output pins (something bad must have have happened to call this function with an invalid pin)"));
            }
            let mut pin = pin.lock().unwrap();
            if !pwm_config.is_active {
                pin.clear_pwm().expect("Failed to set pwm");
                if pwm_config.logic_level == LogicLevel::LOW {
                    pin.set_high();
                } else {
                    pin.set_low();
                }
                return Ok(());
            }
            let duty_cycle = if pwm_config.logic_level == LogicLevel::LOW
            {
                100f64 - pwm_config.duty_cycle
            } else {
                pwm_config.duty_cycle
            };

            pin.set_pwm_frequency(pwm_config.frequency, duty_cycle / 100f64).expect("Failed to set pwm frequency");
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not setup for pwm"))
        }
    }

    fn is_pin_pwm(&self, pin_num: u8) -> bool {
        let pwm = PWMManager::new_rust_reference();
        let pwm = pwm.lock().unwrap();
        pwm.is_pin_pwm(pin_num)
    }

    fn ms_to_duration(&self, ms: Option<f64>) -> Option<Duration> {
         match ms {
            None => None,
            Some(ms) => {
                if ms < 0f64 {
                    None
                } else {
                    Some(Duration::from_secs_f64(ms / 1000f64))
                }
            }
        }
    }


    fn input_callback(&self, pin_num: u8, event: rppal::gpio::Event) {
        let callbacks = {
            let manager = self.gpio.lock().unwrap();
            manager
                .callbacks
                .get(&pin_num)
                .cloned() // Clones the Vec<Callback> to avoid holding the lock
                .unwrap_or_else(|| Vec::new()) // Creates a new Vec if None
        };
        let edge = match event.trigger {
            Trigger::RisingEdge => TriggerEdge::RISING,
            Trigger::FallingEdge => TriggerEdge::FALLING,
            _ => {
                eprintln!("Unknown trigger event");
                return;
            }
        };
        let trigger_time = {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("System time should be after Unix epoch");
            let boot_time = now.checked_sub(event.timestamp)
                               .expect("Failed to calculate boot time");
            let event_unix_time = boot_time + event.timestamp; // Add duration since boot to boot time
            event_unix_time.as_secs_f64()
        };

        // Re-acquire the GIL for calling the Python callback
        Python::with_gil(|py| {
            
            for callback in callbacks {
                let manager = self.gpio.lock().unwrap();
                if callback.trigger_edge != TriggerEdge::BOTH && callback.trigger_edge != edge {
                    continue;
                }
                let cb = callback.callable.lock().unwrap().clone_ref(py);
                let args = &callback.args.lock().unwrap();

                // Prepare new arguments
                let mut new_args: Vec<PyObject> = Vec::new();

                if callback.send_time {
                    new_args.push(trigger_time.to_object(py)); // Add timestamp as the first argument
                }
                if callback.send_edge {
                    new_args.push(edge.into_py(py)); // Add edge as the second argument
                }
                if let Ok(py_tuple) = args.downcast_bound::<PyTuple>(py) {
                    for item in py_tuple.iter() {
                        new_args.push(item.to_object(py));
                    }
                }

                let new_args_tuple = PyTuple::new_bound(py, new_args);
                drop(manager);
                // Call the Python callback
                if let Err(e) = cb.call1(py, new_args_tuple) {
                    e.print(py);
                }
            }
        });
    }
}


#[pymethods]
impl GPIOManager {
    #[new]
    /// Initializes a new GPIOManager instance.
    ///
    /// Example usage:
    /// ```manager = gpio_manager.GPIOManager()```
    ///
    fn new(py: Python) -> PyResult<Py<GPIOManager>> {
        GPIOManager::shared(py)
    }


    /// Sets up an input pin but does not assign a callback yet.
    ///
    /// Parameters:
    /// - ```pin_num``` (int): The GPIO pin to configure as input.
    ///
    /// Example usage:
    /// ```manager.add_input_pin(18)```
    ///
    #[pyo3(signature = (pin_num, pull_resistor_state = InternPullResistorState::AUTO, logic_level = LogicLevel::HIGH, reset_on_exit = true)
    )]
    fn add_input_pin(&self, pin_num: u8, pull_resistor_state: InternPullResistorState, logic_level: LogicLevel, reset_on_exit: bool) -> PyResult<()> {
        if self.is_pin_pwm(pin_num) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin configured for hardware PWM, please reset the pin to use as regular input pin"));
        }
        let mut manager = self.gpio.lock().unwrap();
        if self.is_output_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in output pins (pin is already setup as an output pin"));
        }
        let gpio = Gpio::new().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
        let mut input_pin = match pull_resistor_state {
            InternPullResistorState::PULLUP =>
                gpio
                    .get(pin_num)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?
                    .into_input_pullup(), // Return the input_pullup pin

            InternPullResistorState::PULLDOWN => gpio
                .get(pin_num)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?
                .into_input_pulldown(), // Return the input_pulldown pin

            InternPullResistorState::EXTERNAL => gpio.get(pin_num)
                                                     .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?
                .into_input(),
            InternPullResistorState::AUTO => if logic_level == LogicLevel::HIGH {
                gpio.get(pin_num)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?
                    .into_input_pulldown()
            } else {
                gpio.get(pin_num)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?
                    .into_input_pullup()
            },
        };
        input_pin.set_reset_on_drop(reset_on_exit);
        let input_pin = Pin {
            pin: PinType::Input(Arc::new(Mutex::new(input_pin))),
            logic_level,
        };

        manager.input_pins.insert(pin_num, Arc::new(Mutex::new(input_pin)));

        Ok(())
    }

    /// Assigns a callback to an input pin, specifying the edge trigger.
    ///
    /// Parameters:
    /// - ```pin_num``` (int): The GPIO pin.
    /// - ```trigger_edge``` (str): The edge trigger type ("rising", "falling", or "both").
    /// - ```callback``` (function): The callback function to be invoked on pin change.
    /// - ```args``` (tuple): The arguments to pass to the callback function.
    /// - ```debounce_time_ms``` (int): The debounce time in milliseconds.
    ///
    /// Example usage:
    /// ```manager.assign_callback(18, gpio_manager.TriggerEdge.FALLING, button_callback)```
    ///
    #[pyo3(signature = (pin_num, callback, trigger_edge = TriggerEdge::BOTH, debounce_time_ms = 2f64, args = None, include_trigger_time = false,
    include_trigger_edge = false))]
    fn assign_callback(
        &self,
        py: Python,
        pin_num: u8,
        callback: PyObject,
        trigger_edge: TriggerEdge,
        debounce_time_ms: f64,
        args: Option<&Bound<'_, PyTuple>>, // Using Option to allow args to be None
        include_trigger_time: bool,
        include_trigger_edge: bool,
    ) -> PyResult<()> {
        let manager = self.gpio.lock().unwrap();

        if !self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
        }
        drop(manager);
        let callable: &Bound<PyAny> = callback.bind(py);
        if !callable.is_callable() {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Object is not callable"));
        }
        let empty_tuple = PyTuple::empty_bound(py);
        let args = args.unwrap_or_else(|| &empty_tuple);

        let args_arc = Arc::new(Mutex::new(args.to_object(py)));

        let manager_clone = Arc::clone(&self.gpio);
        let callable = Python::with_gil(|py| {
            callback.clone_ref(py)
        });

        let trigger = {
            let pin_logic_level = manager_clone.lock().unwrap().input_pins.get(&pin_num).unwrap().lock().unwrap().logic_level;
            let trigger_event = match trigger_edge {
                TriggerEdge::RISING => if pin_logic_level == LogicLevel::HIGH {
                    TriggerEdge::RISING
                } else {
                    TriggerEdge::FALLING
                },
                TriggerEdge::FALLING => if pin_logic_level == LogicLevel::HIGH {
                    TriggerEdge::FALLING
                } else {
                    TriggerEdge::RISING
                },
                TriggerEdge::BOTH => TriggerEdge::BOTH,
            };

            trigger_event
        };

        let pin_arc = {
            let manager = manager_clone.lock().unwrap();
            let pin = manager.input_pins.get(&pin_num).unwrap().clone().lock().unwrap().pin.clone();
            if let PinType::Input(pin_arc) = pin {
                pin_arc
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
            }
        };

        let callable = Arc::new(Mutex::new(callable));
        let callback = Callback {
            callable,
            trigger_edge: trigger,
            args: args_arc,
            send_time: include_trigger_time,
            send_edge: include_trigger_edge,
        };

        let mut manager = manager_clone.lock().unwrap();
        let callbacks_set = manager.callbacks.get(&pin_num).is_some();

        if let Some(callback_vec) = manager.callbacks.get_mut(&pin_num) {
            callback_vec.push(callback);
        } else {
            manager.callbacks.insert(pin_num, vec![callback]);
        }
        if !callbacks_set {
            let mut pin = pin_arc.lock().unwrap();
            pin.set_async_interrupt(Trigger::Both, Some(Duration::from_secs_f64(debounce_time_ms / 1000f64)), move |event| {
                let manager = GPIOManager::new_rust_reference();
                // Call input_callback using the locked manager
                manager.input_callback(pin_num, event);
            }).expect("Error setting up async interrupt");
        }
        drop(manager);
        Ok(())
    }

    /// Sets up an output pin.
    ///
    /// Parameters:
    /// - ```pin_num``` (int): The GPIO pin to configure as output.
    ///
    /// Example usage:
    /// ```manager.add_output_pin(25)```
    ///
    #[pyo3(signature = (pin_num, pin_state = PinState::LOW, logic_level = LogicLevel::HIGH, reset_on_exit = true))]
    fn add_output_pin(&self, pin_num: u8, pin_state: PinState, logic_level: LogicLevel, reset_on_exit: bool) -> PyResult<()> {
        if self.is_pin_pwm(pin_num) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin configured for hardware PWM, please reset the pin to use as regular input pin"));
        }
        let mut manager = self.gpio.lock().unwrap();
        if self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in input pins (pin is already setup as an input pin)"));
        }
        let gpio = Gpio::new().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
        let mut output_pin = gpio.get(pin_num)
                                 .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?
            .into_output_high();

        match pin_state {
            PinState::HIGH => if logic_level == LogicLevel::HIGH {
                output_pin.set_high();
            } else {
                output_pin.set_low();
            },
            PinState::LOW => if logic_level == LogicLevel::HIGH {
                output_pin.set_low();
            } else {
                output_pin.set_high();
            },
        };
        output_pin.set_reset_on_drop(reset_on_exit);

        let output_pin = Pin {
            pin: PinType::Output(Arc::new(Mutex::new(output_pin))),
            logic_level,
        };

        manager.output_pins.insert(pin_num, Arc::new(Mutex::new(output_pin)));

        Ok(())
    }

    /// Sets up a PWM output pin.
    ///
    /// Parameters:
    /// - ```pin_num``` (int): The GPIO pin to configure as output.
    /// - ```period_ms``` (int): The period of the PWM signal in milliseconds.
    /// - ```pulse_width_us``` (int): The pulse width of the PWM signal in microseconds.
    ///
    /// Example usage:
    /// ```manager.set_pwm(25, 20, 1200)```
    #[pyo3(signature = (pin_num, frequency_hz = None, duty_cycle = None, period_ms = None, pulse_width_ms = None, logic_level = LogicLevel::HIGH, reset_on_exit = true)
    )]
    fn setup_pwm(&self, pin_num: u8, frequency_hz: Option<f64>, duty_cycle: Option<f64>, period_ms: Option<f64>, pulse_width_ms: Option<f64>, logic_level:
    LogicLevel, reset_on_exit: bool) -> PyResult<()> {
        if self.is_pin_pwm(pin_num) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin configured for hardware PWM, please reset the pin to use as regular input pin"));
        }
        check_pwm_values(&frequency_hz, &duty_cycle, &period_ms, &pulse_width_ms)?;

        let mut manager = self.gpio.lock().unwrap();

        if let Some(_) = manager.pwm_setup.get(&pin_num) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin already configured for PWM"));
        } else if self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in input pins (pin is already setup as an input pin)"));
        } else if self.is_output_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in output pins (pin is already setup as an output pin)"));
        } else {
            drop(manager);
            match logic_level {
                LogicLevel::LOW => {
                    self.add_output_pin(pin_num, PinState::LOW, logic_level, reset_on_exit)?;
                }
                LogicLevel::HIGH => {
                    self.add_output_pin(pin_num, PinState::LOW, logic_level, reset_on_exit)?;
                }
            }

            manager = self.gpio.lock().unwrap();
        }

        let (frequency, duty_cycle_percent) = compute_pwm_values(&frequency_hz, &duty_cycle, &period_ms, &pulse_width_ms);

        if pulse_width_ms.is_some() && pulse_width_ms.unwrap() / 1000f64 > 1f64 / frequency {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pulse width must be less than period (pwm not setup"));
        }

        if self.is_output_pin(pin_num, &manager) {
            manager.pwm_setup.insert(pin_num, PwmConfig {
                frequency,
                duty_cycle: duty_cycle_percent,
                logic_level,
                is_active: false,
            });
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in output pins (pin is either input or not setup)"))
        }
    }

    #[pyo3(signature = (pin_num, reset_on_exit))]
    fn set_reset_on_exit(&self, pin_num: u8, reset_on_exit: bool) -> PyResult<()> {
        let manager = self.gpio.lock().unwrap();
        let output_pins = manager.output_pins.get(&pin_num);
        let input_pins = manager.input_pins.get(&pin_num);
        let pin_match = if let Some(_) = output_pins {
            output_pins
        } else {
            input_pins
        };
        if let Some(pin) =  pin_match{
            let pin = pin.lock().unwrap();
            if let PinType::Output(out_pin) = &pin.pin {
                let mut out_pin = out_pin.lock().unwrap();
                out_pin.set_reset_on_drop(reset_on_exit);
            }
            else if let PinType::Input(in_pin) = &pin.pin {
                let mut in_pin = in_pin.lock().unwrap();
                in_pin.set_reset_on_drop(reset_on_exit);
            }
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input or output pins (pin is not setup)"))
        }

    }

    #[pyo3(signature = (pin_num, duty_cycle = 0f64))]
    fn set_pwm_duty_cycle(&self, pin_num: u8, duty_cycle: f64) -> PyResult<()> {
        if duty_cycle > 100f64 || duty_cycle < 0f64 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Duty cycle must be between 0 and 100, The value {} does not meet this condition", duty_cycle)));
        }

        let mut manager = self.gpio.lock().unwrap();
        if let Some(_) = manager.pwm_setup.get(&pin_num) {
            manager.pwm_setup.get_mut(&pin_num).unwrap().duty_cycle = duty_cycle;
            drop(manager);
            self.set_pwm(pin_num)?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not setup for PWM"))
        }
    }

    #[pyo3(signature = (pin_num, frequency_hz))]
    fn set_pwm_frequency(&self, pin_num: u8, frequency_hz: f64) -> PyResult<()> {
        if frequency_hz < 0f64 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Frequency must be greater than 0, The value {} does not meet this condition", frequency_hz)));
        }
        let mut manager = self.gpio.lock().unwrap();
        if let Some(_) = manager.pwm_setup.get(&pin_num) {
            manager.pwm_setup.get_mut(&pin_num).unwrap().frequency = frequency_hz;
            drop(manager);
            self.set_pwm(pin_num)?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not setup for pwm"))
        }
    }

    #[pyo3(signature = (pin_num, period_ms))]
    fn set_pwm_period(&self, pin_num: u8, period_ms: f64) -> PyResult<()> {
        if period_ms < 0f64 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("period must be greater than 0, The value {} does not meet this condition", period_ms)));
        }
        let mut manager = self.gpio.lock().unwrap();
        if let Some(_) = manager.pwm_setup.get(&pin_num) {
            let frequency_hz = 1f64 / (period_ms / 1000f64);
            manager.pwm_setup.get_mut(&pin_num).unwrap().frequency = frequency_hz;
            drop(manager);
            self.set_pwm(pin_num)?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not setup for pwm"))
        }
    }


    #[pyo3(signature = (pin_num, pulse_width_ms))]
    fn set_pwm_pulse_width(&self, pin_num: u8, pulse_width_ms: f64) -> PyResult<()> {
        if pulse_width_ms < 0f64 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("period must be greater than 0, The value {} does not meet this condition", pulse_width_ms)));
        }
        let mut manager = self.gpio.lock().unwrap();
        if let Some(pin) = manager.pwm_setup.get(&pin_num) {
            let frequency = pin.frequency;
            if pulse_width_ms / 1000f64 > 1f64 / frequency {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pulse width must be less than period"));
            }
            let duty_cycle = pulse_width_ms / ((1f64 / frequency) * 1000f64) * 100f64;
            manager.pwm_setup.get_mut(&pin_num).unwrap().duty_cycle = duty_cycle;
            drop(manager);
            self.set_pwm(pin_num)?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not setup for pwm"))
        }
    }


    #[pyo3(signature = (pin_num))]
    fn start_pwm(&self, pin_num: u8) -> PyResult<()> {
        let mut manager = self.gpio.lock().unwrap();
        if let Some(_) = manager.pwm_setup.get(&pin_num) {
            manager.pwm_setup.get_mut(&pin_num).unwrap().is_active = true;
            drop(manager);
            self.set_pwm(pin_num)?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in output pins (pin is either input or not setup)"))
        }
    }


    #[pyo3(signature = (pin_num))]
    fn stop_pwm(&self, pin_num: u8) -> PyResult<()> {
        let mut manager = self.gpio.lock().unwrap();
        if let Some(_) = manager.pwm_setup.get(&pin_num) {
            manager.pwm_setup.get_mut(&pin_num).unwrap().is_active = false;
            drop(manager);
            self.set_pwm(pin_num)?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in output pins (pin is either input or not setup)"))
        }
    }


    /// Sets the state of an output pin.
    ///
    /// Parameters:
    /// - ```pin_num``` (int): The GPIO pin.
    /// - ```state``` (bool): The desired state (True for high, False for low).
    ///
    /// Example usage:
    /// ```manager.set_output_pin(25, True)```
    #[pyo3(signature = (pin_num, pin_state))]
    fn set_output_pin(&self, pin_num: u8, pin_state: PinState) -> PyResult<()> {
        let manager = self.gpio.lock().unwrap();
        if self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in input pins (pin is setup as an input pin)"));
        }
        if let Some(_) = manager.pwm_setup.get(&pin_num) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin configured for PWM, please reset the pin to use as regular output pin"));
        }
        if let Some(output_pin) = manager.output_pins.get(&pin_num) {
            let output_pin = output_pin.lock().unwrap();
            let mut pin;
            if let PinType::Output(out_pin) = &output_pin.pin {
                pin = out_pin.lock().unwrap();
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in output pins (pin is either input or not setup)"));
            }
            match pin_state {
                PinState::HIGH => if output_pin.logic_level == LogicLevel::HIGH {
                    pin.set_high();
                } else {
                    pin.set_low();
                },
                PinState::LOW => if output_pin.logic_level == LogicLevel::HIGH {
                    pin.set_low();
                } else {
                    pin.set_high();
                },
            }
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in output pins (pin is either input or not setup)"))
        }
    }

    /// Polls the current state of an input pin.
    ///
    /// Parameters:
    /// - ```pin_num``` (int): The GPIO pin to poll.
    ///
    /// Returns:
    /// - ```bool```: The current state of the pin (True for high, False for low).
    ///
    /// Example usage:
    /// ```state = manager.poll_pin(18)```
    ///

    #[pyo3(signature = (pin_num))]
    fn get_pin(&self, pin_num: u8) -> PyResult<PinState> {
        let manager = self.gpio.lock().unwrap();

        if self.is_output_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in output pins (pin is already setup as an output pin)"));
        }
        if let Some(pin_arc) = manager.input_pins.get(&pin_num) {
            let pin_arc = pin_arc.lock().unwrap();
            let pin;
            if let PinType::Input(pin_arc) = &pin_arc.pin {
                pin = pin_arc.lock().unwrap();
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
            }
            if pin.is_high() {
                if pin_arc.logic_level == LogicLevel::HIGH {
                    Ok(PinState::HIGH)
                } else {
                    Ok(PinState::LOW)
                }
            } else {
                if pin_arc.logic_level == LogicLevel::HIGH {
                    Ok(PinState::LOW)
                } else {
                    Ok(PinState::HIGH)
                }
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"))
        }
    }

    /// Unassigns a callback from an input pin.
    ///
    /// Parameters:
    /// - ```pin_num``` (int): The GPIO pin whose callback is to be reset.
    ///
    /// Example usage:
    /// ```state = manager.poll_pin(18)```
    ///
    #[pyo3(signature = (pin_num))]
    fn unassign_callbacks(&self, pin_num: u8) -> PyResult<()> {
        let mut manager = self.gpio.lock().unwrap();

        if !self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
        }
        if let Some(pin_arc) = manager.input_pins.get(&pin_num) {
            let pin_arc = pin_arc.lock().unwrap();
            if let PinType::Input(pin_arc) = &pin_arc.pin {
                let mut pin = pin_arc.lock().unwrap();
                pin.clear_async_interrupt().expect("failed to clear interrupt");
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
            }
        }

        manager.callbacks.remove(&pin_num);
        Ok(())
    }


    #[pyo3(signature = (pin_num, callback))]
    fn unassign_callback(&self, py: Python, pin_num: u8, callback: PyObject) -> PyResult<()> {
        let mut manager = self.gpio.lock().unwrap();
        let callable: &Bound<PyAny> = callback.bind(py);
        if !callable.is_callable() {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Object is not callable"));
        }

        if !self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
        }
        let callbacks = manager.callbacks.get_mut(&pin_num).unwrap();
        let mut index = 0;
        for (i, callable) in callbacks.iter().enumerate() {
            let cb = callable.callable.lock().unwrap(); // Unlock the mutex to access the PyObject
            Python::with_gil(|_| {
                if cb.is(&callback) {
                    index = i;
                    return;
                }
            });
        }
        callbacks.remove(index);
        if callbacks.is_empty() {
            drop(manager);
            self.unassign_callbacks(pin_num)?;
        }

        Ok(())
    }

    /// wait for an edge on the assigned pin
    #[pyo3(signature = (pin_num, trigger_edge = TriggerEdge::BOTH, timeout_ms = None, debounce_ms = 2f64))]
    fn wait_for_edge(&self, pin_num: u8, trigger_edge: TriggerEdge, timeout_ms: Option<f64>, debounce_ms: Option<f64>) -> PyResult<()> {
        let manager = self.gpio.lock().unwrap();

        if !self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
        }

        let timeout = self.ms_to_duration(timeout_ms);

        let debounce = self.ms_to_duration(debounce_ms);

        if let Some(pin_arc) = manager.input_pins.get(&pin_num) {
            let pin_arc = pin_arc.lock().unwrap();
            let trigger = match trigger_edge {
                TriggerEdge::RISING => if pin_arc.logic_level == LogicLevel::HIGH {
                    Trigger::RisingEdge
                } else {
                    Trigger::FallingEdge
                },
                TriggerEdge::FALLING => if pin_arc.logic_level == LogicLevel::HIGH {
                    Trigger::FallingEdge
                } else {
                    Trigger::RisingEdge
                },
                TriggerEdge::BOTH => Trigger::Both,
            };
            if let PinType::Input(pin_arc) = &pin_arc.pin {
                let mut pin = pin_arc.lock().unwrap();
                pin.set_interrupt(trigger, debounce).expect("failed to setup interrupt");
                pin.poll_interrupt(false, timeout).expect("failed to poll interrupt");
                pin.clear_interrupt().expect("failed to clear interrupt");
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
            }
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
        }
        Ok(())
    }

    /// Reset the gpio_pin allowing it to be remapped to input or output
    #[pyo3(signature = (pin_num))]
    fn reset_pin(&self, pin_num: u8) -> PyResult<()> {
        // Lock the manager to start
        let manager = self.gpio.lock().unwrap();

        // Temporary variable to hold the pin if it's found
        let input_pin_arc = manager.input_pins.get(&pin_num).cloned();
        let output_pin_arc = manager.output_pins.get(&pin_num).cloned();
        // Unlock manager before working with pins
        drop(manager);

        // Handle input pins
        if let Some(_) = input_pin_arc {
            self.unassign_callbacks(pin_num)?;
            self.set_reset_on_exit(pin_num, true)?;
            // Re-lock manager to remove the input pin
            let mut manager = self.gpio.lock().unwrap();
            manager.input_pins.remove(&pin_num);
        }
        // Handle output pins
        else if let Some(pin_arc) = output_pin_arc {
            self.set_reset_on_exit(pin_num, true)?;
            let pin_arc = pin_arc.lock().unwrap();
            // Check if this pin has a PWM setup and reset PWM if necessary
            let pwm_exists = {
                let manager = self.gpio.lock().unwrap();
                manager.pwm_setup.get(&pin_num).is_some()
            };
            if pwm_exists {
                if let PinType::Output(out_pin) = &pin_arc.pin {
                    let mut pin = out_pin.lock().unwrap();
                    pin.clear_pwm().expect("Failed to clear pwm");
                    drop(pin);
                }
                drop(pin_arc);

                // Stop PWM and reset it
                self.stop_pwm(pin_num)?;

                // Re-lock the manager to remove the pin from PWM setup
                let mut manager = self.gpio.lock().unwrap();
                manager.pwm_setup.remove(&pin_num);
            } else {
                let pin = &pin_arc.pin;
                if let PinType::Output(_) = pin {
                    drop(pin_arc);
                    self.set_output_pin(pin_num, PinState::LOW)?;
                } else {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in output pins (Something really bad happened to get to this point)"));
                }
            }

            // Re-lock manager to remove the output pin
            let mut manager = self.gpio.lock().unwrap();
            manager.output_pins.remove(&pin_num);
        }

        Ok(())
    }

    /// Cleanup the GPIO pins by setting all outputs to low and clearing all interrupts
    #[pyo3(signature = ())]
    fn cleanup(&self) -> PyResult<()> {
        // Lock the manager and collect the necessary data

        let manager = self.gpio.lock().unwrap();

        // Clone the `Arc<Mutex<Pin>>` references, so we can release the manager lock
        let output_pins: Vec<(u8, Arc<Mutex<Pin>>)> = manager
            .output_pins
            .iter()
            .map(|(&pin_num, pin_arc)| (pin_num, Arc::clone(pin_arc)))
            .collect();

        let input_pins: Vec<(u8, Arc<Mutex<Pin>>)> = manager
            .input_pins
            .iter()
            .map(|(&pin_num, pin_arc)| (pin_num, Arc::clone(pin_arc)))
            .collect();


        drop(manager); // Release the lock on manager

        // Iterate over input pins and reset them
        for (pin_num, _pin_arc) in input_pins {
            self.reset_pin(pin_num)?;
        }

        // Iterate over output pins and reset them
        for (pin_num, _pin_arc) in output_pins {
            // If the pin is configured for PWM, set PWM to zero
            self.reset_pin(pin_num)?;
        }

        Ok(())
    }
}
