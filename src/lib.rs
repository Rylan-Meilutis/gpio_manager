use pyo3::prelude::*;
use pyo3::PyObject;
#[cfg(target_os = "linux")]
use rppal::gpio::{Gpio, InputPin, OutputPin, Trigger};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use once_cell::sync::Lazy;
use pyo3::types::PyTuple;

#[cfg(target_os = "linux")]
struct PinManager {
    input_pins: HashMap<u8, Arc<Mutex<InputPin>>>,
    output_pins: HashMap<u8, Arc<Mutex<OutputPin>>>,
    callbacks: HashMap<u8, PyObject>,
    async_interrupts: HashMap<u8, bool>,
    pwm_setup: HashMap<u8, PwmConfig>,
}
struct PwmConfig {
    frequency: u64,
    duty_cycle: u64,
    is_active: bool,
}


#[pyclass(eq)]
#[derive(Clone, Copy, Debug, PartialEq)]
/// Enum representing the GPIO pin state types.
pub enum IPinState {
    PULLUP,
    PULLDOWN,
    NONE,
}

#[pyclass(eq)]
#[derive(Clone, Copy, Debug, PartialEq)]
/// Enum representing the GPIO pin state types.
pub enum OPinState {
    HIGH,
    LOW,
}

#[pyclass(eq)]
#[derive(Clone, Copy, Debug, PartialEq)]
/// Enum representing the GPIO pin state types.
pub enum LogicLevel {
    HIGH,
    LOW,
}


#[pyclass(eq)]
#[derive(Clone, Copy, Debug, PartialEq)]
/// Enum representing the GPIO pin state types.
pub enum TriggerEdge {
    RISING,
    FALLING,
    BOTH,
}

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
/// ```manager.add_input_pin(18, gpio_manager.IPinState.PULLUP))```
///
/// ```manager.assign_callback(18, gpio_manager.TriggerEdge.FALLING, button_callback)```
///
/// ```manager.set_output_pin(25, gpio_manager.OPinState.HIGH)```
///
pub struct GPIOManager {
    #[cfg(target_os = "linux")]

    gpio: Arc<Mutex<PinManager>>,
}


impl GPIOManager {
    /// Internal method to initialize the GPIOManager singleton.
    fn new_singleton() -> PyResult<Self> {
        #[cfg(not(target_os = "linux"))]
        unimplemented!("This function is only available on Linux");

        #[cfg(target_os = "linux")]
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
        #[cfg(not(target_os = "linux"))]
        unimplemented!("This function is only available on Linux");

        #[cfg(target_os = "linux")]
        Py::new(py, GPIOManager {
            gpio: Arc::clone(&manager.gpio),
        })
    }
    #[cfg(not(target_os = "linux"))]
    fn is_input_pin(&self, pin_num: u8) -> bool {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    fn is_input_pin(&self, pin_num: u8, manager:&MutexGuard<PinManager>) -> bool {
        manager.input_pins.get(&pin_num).is_some()
    }
    #[cfg(not(target_os = "linux"))]
    fn is_output_pin(&self, pin_num: u8) -> bool {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    fn is_output_pin(&self, pin_num: u8, manager:&MutexGuard<PinManager>) -> bool {
        manager.output_pins.get(&pin_num).is_some()
    }


    #[cfg(not(target_os = "linux"))]
    fn is_callback_setup(&self, pin_num: u8) -> bool {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    fn is_callback_setup(&self, pin_num: u8, manager:&MutexGuard<PinManager>) -> bool {
        manager.async_interrupts.get(&pin_num).is_some()
    }

    #[cfg(target_os = "linux")]
    fn set_pwm(&self, pin:u8) -> PyResult<()> {
        let manager = self.gpio.lock().unwrap();
        if let Some(pwm_config) = manager.pwm_setup.get(&pin) {
            let mut pin = manager.output_pins.get(&pin).unwrap().lock().unwrap();
            if !pwm_config.is_active{
                pin.set_pwm(Duration::from_millis(0), Duration::from_millis(0)).expect("Failed to set pwm");
                return Ok(());
            }
            let period = Duration::from_millis(1000 / pwm_config.frequency);
            //pulse with is a percentage of the period
            if pwm_config.duty_cycle == 0 {
                pin.set_low();
                return Ok(());
            }
            if pwm_config.duty_cycle == 100 {
                pin.set_high();
                return Ok(());
            }
            if pwm_config.duty_cycle > 100 || pwm_config.duty_cycle < 0 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Duty cycle must be between 0 and 100, The value {} does not meet this condition", pwm_config.duty_cycle)));
            }
            let pulse_width = Duration::from_micros((period.as_micros() as f64 * (pwm_config.duty_cycle as f64 / 100.0)) as u64);
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
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num, pin_state = IPinState::NONE))]
    fn add_input_pin(&self, pin_num: u8, pin_state: IPinState) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num, pin_state = IPinState::NONE))]
    fn add_input_pin(&self, pin_num: u8, pin_state: IPinState) -> PyResult<()> {
        let mut manager = self.gpio.lock().unwrap();
        if self.is_output_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in output pins (pin is already setup as an output pin"));
        }
        let gpio = Gpio::new().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
        let input_pin = match pin_state {
            IPinState::PULLUP =>
                gpio
                    .get(pin_num)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?
                    .into_input_pullup(), // Return the input_pullup pin

            IPinState::PULLDOWN => gpio
                .get(pin_num)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?
                .into_input_pulldown(), // Return the input_pulldown pin

            IPinState::NONE => gpio.get(pin_num)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?
                .into_input(),
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
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num, trigger_edge, callback, args = None, debounce_time_ms = 2))]
    fn assign_callback(
        &self,
        py: Python,
        pin_num: u8,
        trigger_edge: TriggerEdge,
        callback: PyObject,
        args: Option<&Bound<'_, PyTuple>>, // Using Option to allow args to be None
        debounce_time_ms: u64,
    ) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num, trigger_edge, callback, args = None, debounce_time_ms = 2))]
    fn assign_callback(
        &self,
        py: Python,
        pin_num: u8,
        trigger_edge: TriggerEdge,
        callback: PyObject,
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

        let trigger = match trigger_edge {
            TriggerEdge::RISING => Trigger::RisingEdge,
            TriggerEdge::FALLING => Trigger::FallingEdge,
            TriggerEdge::BOTH => Trigger::Both,
        };

        let manager_clone = Arc::clone(&self.gpio);
        Python::with_gil(|py| {
            let py_callback_clone = callback.clone_ref(py);
            let mut manager = manager_clone.lock().unwrap();
            manager.callbacks.insert(pin_num, py_callback_clone);
            drop(manager);
        });

        let pin_arc = {
            let manager = manager_clone.lock().unwrap();
            manager.input_pins.get(&pin_num).unwrap().clone()
        };

        let py_callback_clone = Python::with_gil(|py| {
            let manager = manager_clone.lock().unwrap();
            manager.callbacks.get(&pin_num).unwrap().clone_ref(py)
        });

        // Use rppal's async interrupt handler without spawning a thread
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
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num, pin_state = OPinState::LOW, logic_level = LogicLevel::HIGH))]
    fn add_output_pin(&self, pin_num: u8, pin_state: OPinState, logic_level: LogicLevel) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num, pin_state = OPinState::LOW, logic_level = LogicLevel::HIGH))]
    fn add_output_pin(&self, pin_num: u8, pin_state: OPinState, logic_level: LogicLevel) -> PyResult<()> {
        let mut manager = self.gpio.lock().unwrap();
        if self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in input pins (pin is already setup as an input pin)"));
        }
        let gpio = Gpio::new().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
        let mut output_pin = match logic_level {
            LogicLevel::HIGH => {
                gpio
                    .get(pin_num)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?
                    .into_output_high()
            }
            LogicLevel::LOW => {
                gpio
                    .get(pin_num)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?
                    .into_output_low()
            }
        };
        match pin_state {
            OPinState::HIGH => output_pin.set_high(),
            OPinState::LOW => output_pin.set_low(),
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
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num, frequency_hz = 60, duty_cycle = 0, logic_level = LogicLevel::HIGH))]
    fn setup_pwm(&self, pin_num: u8, frequency_hz: u64, duty_cycle: u64, logic_level:LogicLevel) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num, frequency_hz = 60, duty_cycle = 0, logic_level = LogicLevel::HIGH))]
    fn setup_pwm(&self, pin_num: u8, frequency_hz: u64, duty_cycle: u64, logic_level:LogicLevel) -> PyResult<()> {
        if duty_cycle > 100 || duty_cycle < 0{
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Duty cycle must be between 0 and 100, The value {} does not meet this condition", pwm_config.duty_cycle)));
        }
        let mut manager = self.gpio.lock().unwrap();
        if self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in input pins (pin is already setup as an input pin)"));
        }
        if !self.is_output_pin(pin_num, &manager) {
            drop(manager);
            self.add_output_pin(pin_num, OPinState::LOW, logic_level)?;
            manager = self.gpio.lock().unwrap();
        }
        if let Some(_) = manager.pwm_setup.get(&pin_num) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin already configured for PWM"));
        }
        if self.is_output_pin(pin_num, &manager) {
            manager.pwm_setup.insert(pin_num, PwmConfig {
                frequency: frequency_hz,
                duty_cycle,
                is_active: false,
            });
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in output pins (pin is either input or not setup)"))
        }
    }


    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num, duty_cycle = 0))]
    fn set_pwm_duty_cycle(&self, pin_num: u8, duty_cycle: u64) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num, duty_cycle = 0))]
    fn set_pwm_duty_cycle(&self, pin_num: u8, duty_cycle: u64) -> PyResult<()> {
        if duty_cycle > 100 || duty_cycle < 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Duty cycle must be between 0 and 100, The value {} does not meet this condition", pwm_config.duty_cycle)));
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

    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num, frequency_hz = 60))]
    fn set_pwm_frequency(&self, pin_num: u8, frequency_hz: u64) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num, frequency_hz = 60))]
    fn set_pwm_frequency(&self, pin_num: u8, frequency_hz: u64) -> PyResult<()> {
        let mut manager = self.gpio.lock().unwrap();
        if let Some(_) = manager.pwm_setup.get(&pin_num) {
            manager.pwm_setup.get_mut(&pin_num).unwrap().frequency = frequency_hz;
            self.set_pwm(pin_num)?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not setup for pwm"))
        }
    }


    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num))]
    fn start_pwm(&self, pin_num: u8) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num))]
    fn start_pwm(&self, pin_num: u8) -> PyResult<()> {
        let mut manager = self.gpio.lock().unwrap();
        if let Some(_) = manager.output_pins.get(&pin_num) {
            manager.pwm_setup.get_mut(&pin_num).unwrap().is_active = true;
            drop(manager);
            self.set_pwm(pin_num)?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in output pins (pin is either input or not setup)"))
        }
    }


    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num))]
    fn stop_pwm(&self, pin_num: u8) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num))]
    fn stop_pwm(&self, pin_num: u8) -> PyResult<()> {
        let mut manager = self.gpio.lock().unwrap();
        if let Some(_) = manager.output_pins.get(&pin_num) {
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
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num, pin_state))]
    fn set_output_pin(&self, pin_num: u8, pin_state: OPinState) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num, pin_state))]
    fn set_output_pin(&self, pin_num: u8, pin_state: OPinState) -> PyResult<()> {
        let manager = self.gpio.lock().unwrap();
        if self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in input pins (pin is setup as an input pin)"));
        }
        if let Some(_) = manager.pwm_setup.get(&pin_num) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin configured for PWM, please reset the pin to use as regular output pin"));
        }
        if let Some(pin) = manager.output_pins.get(&pin_num) {
            let mut pin = pin.lock().unwrap();
            match pin_state {
                OPinState::HIGH => pin.set_high(),
                OPinState::LOW => pin.set_low(),
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
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num))]
    fn get_pin(&self, pin_num: u8) -> PyResult<OPinState> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num))]
    fn get_pin(&self, pin_num: u8) -> PyResult<OPinState> {
        let manager = self.gpio.lock().unwrap();

        if self.is_output_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin found in output pins (pin is already setup as an output pin)"));
        }
        if let Some(pin_arc) = manager.input_pins.get(&pin_num) {
            let pin = pin_arc.lock().unwrap();
            if pin.is_high() {
                Ok(OPinState::HIGH)
            } else {
                Ok(OPinState::LOW)
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
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num))]
    fn unassign_callback(&self, pin_num: u8) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num))]
    fn unassign_callback(&self, pin_num: u8) -> PyResult<()> {
        let manager = self.gpio.lock().unwrap();

        if !self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
        }
        if let Some(pin_arc) = manager.input_pins.get(&pin_num) {
            let mut pin = pin_arc.lock().unwrap();
            pin.clear_async_interrupt().expect("failed to clear interrupt");
        }
        Ok(())
    }

    /// wait for an edge on the assigned pin
    ///
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num, trigger_edge = TriggerEdge::BOTH, timeout_ms = -1))]
    fn wait_for_edge(&self, pin_num: u8, trigger_edge: TriggerEdge, timeout_ms: i64) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num, trigger_edge = TriggerEdge::BOTH, timeout_ms = -1))]
    fn wait_for_edge(&self, pin_num: u8, trigger_edge: TriggerEdge, timeout_ms: i64) -> PyResult<()> {
        let manager = self.gpio.lock().unwrap();

        if !self.is_input_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
        }

        let trigger = match trigger_edge {
            TriggerEdge::RISING => Trigger::RisingEdge,
            TriggerEdge::FALLING => Trigger::FallingEdge,
            TriggerEdge::BOTH => Trigger::Both,
        };
        let timeout = if timeout_ms < 0 {
            None
        } else {
            Some(Duration::from_millis(timeout_ms as u64))
        };
        if let Some(pin_arc) = manager.input_pins.get(&pin_num) {
            let mut pin = pin_arc.lock().unwrap();
            pin.set_interrupt(trigger, timeout).expect("failed to setup interrupt");
            pin.poll_interrupt(false, timeout).expect("failed to poll interrupt");
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins (pin is either output or not setup)"));
        }
        Ok(())
    }

    /// Reset the gpio_pin allowing it to be remapped to input or output
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (pin_num))]
    fn reset_pin(&self, pin_num: u8) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }

    #[cfg(target_os = "linux")]
    #[pyo3(signature = (pin_num))]
    fn reset_pin(&self, pin_num: u8) -> PyResult<()> {
        let manager = self.gpio.lock().unwrap();

        if !self.is_input_pin(pin_num, &manager) && !self.is_output_pin(pin_num, &manager) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pin not found in input pins or output pins (Can't reset a pin that isn't setup)"));
        }
        if let Some(pin_arc) = manager.input_pins.get(&pin_num) {
            let mut manager = self.gpio.lock().unwrap();
            let mut pin = pin_arc.lock().unwrap();
            pin.clear_async_interrupt().expect("failed to clear interrupt");
            manager.input_pins.remove(&pin_num);
        } else if let Some(pin_arc) = manager.output_pins.get(&pin_num) {
            let mut manager = self.gpio.lock().unwrap();
            let mut pin = pin_arc.lock().unwrap();
            pin.set_low();
            if let Some(pwm_config) = manager.pwm_setup.get(&pin_num) {
                if pwm_config.is_active {
                    pin.set_pwm(Duration::from_millis(0), Duration::from_millis(0)).expect("Failed to set pwm");
                }
                manager.pwm_setup.remove(&pin_num);
            }
            manager.output_pins.remove(&pin_num);
        }
        Ok(())
    }

    /// Cleanup the GPIO pins by setting all outputs to low and clearing all interrupts
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = ())]
    fn cleanup(&self) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }
    #[cfg(target_os = "linux")]
    #[pyo3(signature = ())]
    fn cleanup(&self) -> PyResult<()> {
        // Lock the manager and collect the necessary data
        let manager = self.gpio.lock().unwrap();
        let output_pins: Vec<(u8, Arc<Mutex<OutputPin>>)> = manager
            .output_pins
            .iter()
            .map(|(&pin_num, pin_arc)| (pin_num, Arc::clone(pin_arc)))
            .collect();
        let input_pins: Vec<Arc<Mutex<InputPin>>> = manager.input_pins.values().cloned().collect();
        let pwm_pins: Vec<u8> = manager.pwm_setup.keys().cloned().collect();
        drop(manager); // Release the lock on manager

        // Iterate over output pins
        for (pin_num, pin_arc) in output_pins {
            let mut pin = pin_arc.lock().unwrap();

            // If the pin is configured for PWM, set PWM to zero
            if pwm_pins.contains(&pin_num) {
                pin.set_pwm(Duration::from_millis(0), Duration::from_millis(0))
                    .expect("Failed to set PWM to zero");
            }

            // Set the pin to low
            pin.set_low();
        }

        // Iterate over input pins
        for pin_arc in input_pins {
            let mut pin = pin_arc.lock().unwrap();
            pin.clear_async_interrupt().expect("Failed to clear interrupt");
        }

        Ok(())
    }
}


#[pymodule]
fn gpio_manager(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GPIOManager>()?;
    m.add_class::<IPinState>()?;
    m.add_class::<OPinState>()?;
    m.add_class::<LogicLevel>()?;
    m.add_class::<TriggerEdge>()?;
    Ok(())
}
