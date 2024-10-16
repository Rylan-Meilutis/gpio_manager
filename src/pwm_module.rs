use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use pyo3::{pyclass, pymethods, Py, PyErr, PyResult, Python};
use rppal::pwm::{Pwm, Channel, Polarity};

#[pyclass(eq, eq_int)]
#[derive(Clone, Copy, Eq, PartialEq)]
/// Enumeration for PWM Polarity.
pub enum PWMPolarity {
    NORMAL,
    INVERSE,
}

impl From<PWMPolarity> for Polarity {
    fn from(polarity: PWMPolarity) -> Self {
        match polarity {
            PWMPolarity::NORMAL => Polarity::Normal,
            PWMPolarity::INVERSE => Polarity::Inverse,
        }
    }
}

// Singleton instance of PWMManager
static PWM_MANAGER: Lazy<Arc<Mutex<PWMManager>>> = Lazy::new(|| {
    Arc::new(Mutex::new(PWMManager::new_singleton().expect("Failed to initialize PWMManager")))
});

#[pyclass]
/// PWMManager provides methods to manage PWM channels.
///
/// Example usage in Python:
///
/// ```python
/// pwm_manager = pwm_manager.PWMManager()
/// pwm_manager.setup_pwm_channel(0, frequency_hz=1000.0, duty_cycle=0.5, polarity=pwm_manager.PWMPolarity.NORMAL)
/// pwm_manager.set_duty_cycle(0, 0.75)
/// pwm_manager.stop_pwm_channel(0)
/// pwm_manager.remove_pwm_channel(0)
/// ```
pub struct PWMManager {
    pwm_channels: Arc<Mutex<HashMap<u8, Arc<Mutex<Pwm>>>>>,
}

impl PWMManager {
    /// Internal method to initialize the PWMManager singleton.
    fn new_singleton() -> PyResult<Self> {
        Ok(Self {
            pwm_channels: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    fn shared(py: Python) -> PyResult<Py<PWMManager>> {
        let manager = PWM_MANAGER.lock().unwrap();
        Py::new(py, PWMManager {
            pwm_channels: Arc::clone(&manager.pwm_channels),
        })
    }
}

#[pymethods]
impl PWMManager {
    #[new]
    /// Initializes a new PWMManager instance.
    ///
    /// Example usage:
    /// ```python
    /// pwm_manager = pwm_manager.PWMManager()
    /// ```
    fn new(py: Python) -> PyResult<Py<PWMManager>> {
        PWMManager::shared(py)
    }

    /// Sets up a PWM channel with the specified parameters.
    ///
    /// Parameters:
    /// - `channel_num` (int): The PWM channel number (0 or 1).
    /// - `frequency_hz` (float): The frequency in Hertz.
    /// - `duty_cycle` (float): The duty cycle (0.0 to 1.0).
    /// - `polarity` (PWMPolarity): The polarity of the PWM signal.
    ///
    /// Example usage:
    /// ```python
    /// pwm_manager.setup_pwm_channel(0, frequency_hz=1000.0, duty_cycle=0.5, polarity=pwm_manager.PWMPolarity.NORMAL)
    /// ```
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (channel_num, frequency_hz = 60.0, duty_cycle = 0.5, polarity = PWMPolarity::NORMAL))]
    fn setup_pwm_channel(&self, channel_num: u8, frequency_hz: f64, duty_cycle: f64, polarity: PWMPolarity) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }

    #[cfg(target_os = "linux")]
    #[pyo3(signature = (channel_num, frequency_hz = 60.0, duty_cycle = 0.5, polarity = PWMPolarity::NORMAL))]
    fn setup_pwm_channel(&self, channel_num: u8, frequency_hz: f64, duty_cycle: f64, polarity: PWMPolarity) -> PyResult<()> {
        let mut pwm_channels = self.pwm_channels.lock().unwrap();

        if pwm_channels.contains_key(&channel_num) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PWM channel already initialized"));
        }

        let channel = match channel_num {
            0 => Channel::Pwm0,
            1 => Channel::Pwm1,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid PWM channel number")),
        };

        if duty_cycle < 0.0 || duty_cycle > 1.0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Duty cycle must be between 0.0 and 1.0"));
        }

        let polarity: Polarity = polarity.into();

        let pwm = Pwm::with_frequency(channel, frequency_hz, duty_cycle, polarity, true)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;

        pwm_channels.insert(channel_num, Arc::new(Mutex::new(pwm)));

        Ok(())
    }

    /// Starts the PWM signal on the specified channel.
    ///
    /// Parameters:
    /// - `channel_num` (int): The PWM channel number (0 or 1).
    ///
    /// Example usage:
    /// ```python
    /// pwm_manager.start_pwm_channel(0)
    /// ```
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (channel_num))]
    fn start_pwm_channel(&self, channel_num: u8) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }

    #[cfg(target_os = "linux")]
    #[pyo3(signature = (channel_num))]
    fn start_pwm_channel(&self, channel_num: u8) -> PyResult<()> {
        let pwm_channels = self.pwm_channels.lock().unwrap();

        if let Some(pwm_arc) = pwm_channels.get(&channel_num) {
            let pwm = pwm_arc.lock().unwrap();
            pwm.enable().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PWM channel not initialized"))
        }
    }

    /// Stops the PWM signal on the specified channel.
    ///
    /// Parameters:
    /// - `channel_num` (int): The PWM channel number (0 or 1).
    ///
    /// Example usage:
    /// ```python
    /// pwm_manager.stop_pwm_channel(0)
    /// ```
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (channel_num))]
    fn stop_pwm_channel(&self, channel_num: u8) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }

    #[cfg(target_os = "linux")]
    #[pyo3(signature = (channel_num))]
    fn stop_pwm_channel(&self, channel_num: u8) -> PyResult<()> {
        let pwm_channels = self.pwm_channels.lock().unwrap();

        if let Some(pwm_arc) = pwm_channels.get(&channel_num) {
            let pwm = pwm_arc.lock().unwrap();
            pwm.disable().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PWM channel not initialized"))
        }
    }

    /// Removes the PWM channel from the manager.
    ///
    /// Parameters:
    /// - `channel_num` (int): The PWM channel number (0 or 1).
    ///
    /// Example usage:
    /// ```python
    /// pwm_manager.remove_pwm_channel(0)
    /// ```
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (channel_num))]
    fn remove_pwm_channel(&self, channel_num: u8) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }

    #[cfg(target_os = "linux")]
    #[pyo3(signature = (channel_num))]
    fn remove_pwm_channel(&self, channel_num: u8) -> PyResult<()> {
        let mut pwm_channels = self.pwm_channels.lock().unwrap();

        if pwm_channels.remove(&channel_num).is_some() {
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PWM channel not initialized"))
        }
    }

    /// Sets the duty cycle for the specified PWM channel.
    ///
    /// Parameters:
    /// - `channel_num` (int): The PWM channel number (0 or 1).
    /// - `duty_cycle` (float): The new duty cycle (0.0 to 1.0).
    ///
    /// Example usage:
    /// ```python
    /// pwm_manager.set_duty_cycle(0, 0.75)
    /// ```
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (channel_num, duty_cycle))]
    fn set_duty_cycle(&self, channel_num: u8, duty_cycle: f64) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }

    #[cfg(target_os = "linux")]
    #[pyo3(signature = (channel_num, duty_cycle))]
    fn set_duty_cycle(&self, channel_num: u8, duty_cycle: f64) -> PyResult<()> {
        if duty_cycle < 0.0 || duty_cycle > 1.0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Duty cycle must be between 0.0 and 1.0"));
        }

        let pwm_channels = self.pwm_channels.lock().unwrap();

        if let Some(pwm_arc) = pwm_channels.get(&channel_num) {
            let pwm = pwm_arc.lock().unwrap();
            pwm.set_duty_cycle(duty_cycle).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PWM channel not initialized"))
        }
    }

    /// Sets the frequency for the specified PWM channel.
    ///
    /// Parameters:
    /// - `channel_num` (int): The PWM channel number (0 or 1).
    /// - `frequency_hz` (float): The new frequency in Hertz.
    ///
    /// Example usage:
    /// ```python
    /// pwm_manager.set_frequency(0, 500.0)
    /// ```
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (channel_num, frequency_hz))]
    fn set_frequency(&self, channel_num: u8, frequency_hz: f64) -> PyResult<()> {
        unimplemented!("This function is only available on Linux");
    }

    #[cfg(target_os = "linux")]
    #[pyo3(signature = (channel_num, frequency_hz))]
    fn set_frequency(&self, channel_num: u8, frequency_hz: f64) -> PyResult<()> {
        let pwm_channels = self.pwm_channels.lock().unwrap();

        if let Some(pwm_arc) = pwm_channels.get(&channel_num) {
            let pwm = pwm_arc.lock().unwrap();
            let current_duty_cycle = pwm.duty_cycle().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
            pwm.set_frequency(frequency_hz, current_duty_cycle).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PWM channel not initialized"))
        }
    }

    /// Gets the current frequency of the specified PWM channel.
    ///
    /// Parameters:
    /// - `channel_num` (int): The PWM channel number (0 or 1).
    ///
    /// Returns:
    /// - `float`: The current frequency in Hertz.
    ///
    /// Example usage:
    /// ```python
    /// frequency = pwm_manager.get_frequency(0)
    /// ```
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (channel_num))]
    fn get_frequency(&self, channel_num: u8) -> PyResult<f64> {
        unimplemented!("This function is only available on Linux");
    }

    #[cfg(target_os = "linux")]
    #[pyo3(signature = (channel_num))]
    fn get_frequency(&self, channel_num: u8) -> PyResult<f64> {
        let pwm_channels = self.pwm_channels.lock().unwrap();

        if let Some(pwm_arc) = pwm_channels.get(&channel_num) {
            let pwm = pwm_arc.lock().unwrap();
            if let Ok(frequency) = pwm.frequency() {
                Ok(frequency)
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to get PWM frequency"))
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PWM channel not initialized"))
        }
    }

    /// Gets the current duty cycle of the specified PWM channel.
    ///
    /// Parameters:
    /// - `channel_num` (int): The PWM channel number (0 or 1).
    ///
    /// Returns:
    /// - `float`: The current duty cycle (0.0 to 1.0).
    ///
    /// Example usage:
    /// ```python
    /// duty_cycle = pwm_manager.get_duty_cycle(0)
    /// ```
    #[cfg(not(target_os = "linux"))]
    #[pyo3(signature = (channel_num))]
    fn get_duty_cycle(&self, channel_num: u8) -> PyResult<f64> {
        unimplemented!("This function is only available on Linux");
    }

    #[cfg(target_os = "linux")]
    #[pyo3(signature = (channel_num))]
    fn get_duty_cycle(&self, channel_num: u8) -> PyResult<f64> {
        let pwm_channels = self.pwm_channels.lock().unwrap();

        if let Some(pwm_arc) = pwm_channels.get(&channel_num) {
            let pwm = pwm_arc.lock().unwrap();
            if let Ok(duty_cycle) = pwm.duty_cycle() {
                Ok(duty_cycle)
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to get PWM duty cycle"))
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PWM channel not initialized"))
        }
    }
}
