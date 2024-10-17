use crate::{InternPullResistorState, LogicLevel, OPinState, Pin, PinManager, PinType, PwmConfig, TriggerEdge};
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::PyObject;
use pyo3::{pyclass, pymethods, Py, PyErr, PyResult, Python};
use rppal::gpio::{Gpio, Trigger};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;


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
                async_interrupts: HashMap::new(),
                pwm_setup: HashMap::new(),
            })),
        })
    }

    fn shared(py: Python) -> PyResult<Py<GPIOManager>> {
        let manager = GPIO_MANAGER.lock().unwrap();

        Py::new(py, GPIOManager {
            gpio: Arc::clone(&manager.gpio),
        })
    }

    fn is_input_pin(&self, pin_num: u8, manager: &MutexGuard<PinManager>) -> bool {
        manager.input_pins.get(&pin_num).is_some()
    }

    fn is_output_pin(&self, pin_num: u8, manager: &MutexGuard<PinManager>) -> bool {
        manager.output_pins.get(&pin_num).is_some()
    }


    fn is_callback_setup(&self, pin_num: u8, manager: &MutexGuard<PinManager>) -> bool {
        manager.async_interrupts.get(&pin_num).is_some()
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
            let period = if pwm_config.frequency == 0 {
                Duration::from_millis(0)
            } else {
                Duration::from_millis(1000 / pwm_config.frequency)
            };
            if pwm_config.duty_cycle > 100 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Duty cycle must be between 0 and 100, The value {} does not meet this condition", pwm_config.duty_cycle)));
            }
            let pulse_width = if pwm_config.logic_level == LogicLevel::LOW {
                Duration::from_micros((period.as_micros() as f64 * ((100f64 - pwm_config.duty_cycle as f64) / 100.0)) as u64)
            } else {
                Duration::from_micros((period.as_micros() as f64 * (pwm_config.duty_cycle as f64 / 100.0)) as u64)
            };
            pin.set_pwm(period, pulse_width).expect("Failed to set pwm");
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not setup for pwm"))
        }
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
    #[pyo3(signature = (pin_num, pull_resistor_state = InternPullResistorState::AUTO, logic_level = LogicLevel::HIGH)
    )]
    fn add_input_pin(&self, pin_num: u8, pull_resistor_state: InternPullResistorState, logic_level: LogicLevel) -> PyResult<()> {
        let mut manager = self.gpio.lock().unwrap();
        if self.is_output_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in output pins (pin is already setup as an output pin"));
        }
        let gpio = Gpio::new().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
        let input_pin = match pull_resistor_state {
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
    #[pyo3(signature = (pin_num, callback, trigger_edge = TriggerEdge::BOTH, args = None, debounce_time_ms = 2))]
    fn assign_callback(
        &self,
        py: Python,
        pin_num: u8,
        callback: PyObject,
        trigger_edge: TriggerEdge,
        args: Option<&Bound<'_, PyTuple>>, // Using Option to allow args to be None
        debounce_time_ms: u64,
    ) -> PyResult<()> {

        // check if the pin has an async interrupt already
        //
        let manager = self.gpio.lock().unwrap();

        if !self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
        }
        if self.is_callback_setup(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Callback already assigned to pin"));
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
        Python::with_gil(|py| {
            let py_callback_clone = callback.clone_ref(py);
            let mut manager = manager_clone.lock().unwrap();
            manager.callbacks.insert(pin_num, py_callback_clone);
            drop(manager);
        });

        let trigger = {
            let pin_logic_level = manager_clone.lock().unwrap().input_pins.get(&pin_num).unwrap().lock().unwrap().logic_level;
            let trigger_event = match trigger_edge {
                TriggerEdge::RISING => if pin_logic_level == LogicLevel::HIGH {
                    Trigger::RisingEdge
                } else {
                    Trigger::FallingEdge
                },
                TriggerEdge::FALLING => if pin_logic_level == LogicLevel::HIGH {
                    Trigger::FallingEdge
                } else {
                    Trigger::RisingEdge
                },
                TriggerEdge::BOTH => Trigger::Both,
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

        let py_callback_clone = Python::with_gil(|py| {
            let manager = manager_clone.lock().unwrap();
            manager.callbacks.get(&pin_num).unwrap().clone_ref(py)
        });

        let mut pin = pin_arc.lock().unwrap();
        pin.set_async_interrupt(trigger, Some(Duration::from_millis(debounce_time_ms)), move |_event| {
            // Re-acquire the GIL for calling the Python callback
            Python::with_gil(|py| {
                let cb = py_callback_clone.clone_ref(py);
                let args = args_arc.lock().unwrap();

                if let Err(e) = cb.call1(py, args.bind(py).downcast::<PyTuple>().unwrap()) {
                    e.print(py);
                }
            });
        }).expect("Error setting up async interrupt");
        let mut manager = manager_clone.lock().unwrap();
        manager.async_interrupts.insert(pin_num, true);
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
    #[pyo3(signature = (pin_num, pin_state = OPinState::LOW, logic_level = LogicLevel::HIGH))]
    fn add_output_pin(&self, pin_num: u8, pin_state: OPinState, logic_level: LogicLevel) -> PyResult<()> {
        let mut manager = self.gpio.lock().unwrap();
        if self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in input pins (pin is already setup as an input pin)"));
        }
        let gpio = Gpio::new().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
        let mut output_pin = gpio.get(pin_num)
                                 .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?
            .into_output_high();

        match pin_state {
            OPinState::HIGH => if logic_level == LogicLevel::HIGH {
                output_pin.set_high();
            } else {
                output_pin.set_low();
            },
            OPinState::LOW => if logic_level == LogicLevel::HIGH {
                output_pin.set_low();
            } else {
                output_pin.set_high();
            },
        };
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
    #[pyo3(signature = (pin_num, frequency_hz = 60, duty_cycle = 0, logic_level = LogicLevel::HIGH)
    )]
    fn setup_pwm(&self, pin_num: u8, frequency_hz: u64, duty_cycle: u64, logic_level: LogicLevel) -> PyResult<()> {
        if duty_cycle > 100 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Duty cycle must be between 0 and 100, The value {} does not meet this condition", duty_cycle)));
        }
        let mut manager = self.gpio.lock().unwrap();
        if self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in input pins (pin is already setup as an input pin)"));
        }
        if !self.is_output_pin(pin_num, &manager) {
            drop(manager);
            if logic_level == LogicLevel::LOW {
                self.add_output_pin(pin_num, OPinState::HIGH, logic_level)?;
            } else {
                self.add_output_pin(pin_num, OPinState::LOW, logic_level)?;
            }

            manager = self.gpio.lock().unwrap();
        }
        if let Some(_) = manager.pwm_setup.get(&pin_num) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin already configured for PWM"));
        }
        if self.is_output_pin(pin_num, &manager) {
            manager.pwm_setup.insert(pin_num, PwmConfig {
                frequency: frequency_hz,
                duty_cycle,
                logic_level,
                is_active: false,
            });
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in output pins (pin is either input or not setup)"))
        }
    }


    #[pyo3(signature = (pin_num, duty_cycle = 0))]
    fn set_pwm_duty_cycle(&self, pin_num: u8, duty_cycle: u64) -> PyResult<()> {
        if duty_cycle > 100 {
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

    #[pyo3(signature = (pin_num, frequency_hz = 60))]
    fn set_pwm_frequency(&self, pin_num: u8, frequency_hz: u64) -> PyResult<()> {
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
    fn set_output_pin(&self, pin_num: u8, pin_state: OPinState) -> PyResult<()> {
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
                OPinState::HIGH => if output_pin.logic_level == LogicLevel::HIGH {
                    pin.set_high();
                } else {
                    pin.set_low();
                },
                OPinState::LOW => if output_pin.logic_level == LogicLevel::HIGH {
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
    fn get_pin(&self, pin_num: u8) -> PyResult<OPinState> {
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
                    Ok(OPinState::HIGH)
                } else {
                    Ok(OPinState::LOW)
                }
            } else {
                if pin_arc.logic_level == LogicLevel::HIGH {
                    Ok(OPinState::LOW)
                } else {
                    Ok(OPinState::HIGH)
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
    fn unassign_callback(&self, pin_num: u8) -> PyResult<()> {
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

        manager.async_interrupts.remove(&pin_num);
        manager.callbacks.remove(&pin_num);
        Ok(())
    }

    /// wait for an edge on the assigned pin
    #[pyo3(signature = (pin_num, trigger_edge = TriggerEdge::BOTH, timeout_ms = -1))]
    fn wait_for_edge(&self, pin_num: u8, trigger_edge: TriggerEdge, timeout_ms: i64) -> PyResult<()> {
        let manager = self.gpio.lock().unwrap();

        if !self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
        }


        let timeout = if timeout_ms < 0 {
            None
        } else {
            Some(Duration::from_millis(timeout_ms as u64))
        };
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
                pin.set_interrupt(trigger, timeout).expect("failed to setup interrupt");
                pin.poll_interrupt(false, timeout).expect("failed to poll interrupt");
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
            self.unassign_callback(pin_num)?;
            // Re-lock manager to remove the input pin
            let mut manager = self.gpio.lock().unwrap();
            manager.input_pins.remove(&pin_num);
        }
        // Handle output pins
        else if let Some(pin_arc) = output_pin_arc {
            let pin_arc = pin_arc.lock().unwrap();
            // Check if this pin has a PWM setup and reset PWM if necessary
            let pwm_exists = {
                let manager = self.gpio.lock().unwrap();
                manager.pwm_setup.get(&pin_num).is_some()
            };
            if pwm_exists {
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
                    self.set_output_pin(pin_num, OPinState::LOW)?;
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
