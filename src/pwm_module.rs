use crate::LogicLevel;
use once_cell::sync::Lazy;
use pyo3::{pyclass, pymethods, Py, PyErr, PyResult, Python};
use rppal::pwm::{Channel, Polarity, Pwm};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;


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
/// pwm_manager.setup_pwm_channel(0, frequency_hz=1000, duty_cycle=0.5, polarity=pwm_manager.PWMPolarity.NORMAL)
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
    /// - `duty_cycle` (int): The duty cycle (0 to 100).
    /// - `polarity` (PWMPolarity): The polarity of the PWM signal.
    ///
    /// Example usage:
    /// ```python
    /// pwm_manager.setup_pwm_channel(0, frequency_hz=100, duty_cycle=0.5, polarity=pwm_manager.PWMPolarity.NORMAL)
    /// ```
    #[pyo3(signature = (channel_num, frequency_hz = 0f64, duty_cycle = 0f64, period_ms = 0f64, pulse_width_ms = 0f64, logic_level = LogicLevel::HIGH)
    )]
    fn setup_pwm_channel(&self, channel_num: u8, frequency_hz: f64, duty_cycle: f64, period_ms: f64, pulse_width_ms: f64, logic_level: LogicLevel) -> PyResult<()> {
        let mut pwm_channels = self.pwm_channels.lock().unwrap();

        if pwm_channels.contains_key(&channel_num) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PWM channel already initialized"));
        }

        let channel = match channel_num {
            0 => Channel::Pwm0,
            1 => Channel::Pwm1,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid PWM channel number")),
        };

        if duty_cycle > 100f64 || duty_cycle < 0f64 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Duty cycle must be between 0 and 100, the current value {} does not meet this condition", duty_cycle)));
        }

        if period_ms < 0f64 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Period must be greater than 0, the current value {} does not meet this condition", period_ms)));
        }
        if pulse_width_ms > period_ms || pulse_width_ms < 0f64 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Pulse width must be less than the period and greater than 0, the current value {} does not meet this condition", pulse_width_ms)));
        }

        let frequency_hz = if frequency_hz > 0f64 { frequency_hz } else if period_ms > 0f64 {
            1f64 / period_ms * 1000f64
        } else { 1000f64 };

        let duty_cycle = if duty_cycle > 0f64 { duty_cycle } else {
            pulse_width_ms / (1f64 / frequency_hz) * 100f64
        };

        let polarity = match logic_level {
            LogicLevel::HIGH => Polarity::Normal,
            LogicLevel::LOW => Polarity::Inverse,
        };

        let pwm = Pwm::with_frequency(channel, frequency_hz, duty_cycle / 100f64, polarity, false)
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
    #[pyo3(signature = (channel_num))]
    fn reset_pwm_channel(&self, channel_num: u8) -> PyResult<()> {
        self.stop_pwm_channel(channel_num)?;

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
    /// - `duty_cycle` (float): The new duty cycle (0 to 100).
    ///
    /// Example usage:
    /// ```python
    /// pwm_manager.set_duty_cycle(0, 75)
    /// ```
    #[pyo3(signature = (channel_num, duty_cycle))]
    fn set_duty_cycle(&self, channel_num: u8, duty_cycle: f64) -> PyResult<()> {
        if duty_cycle > 100f64 || duty_cycle < 0f64 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Duty cycle must be between 0 and 100, current value {} does not meet this condition", duty_cycle)));
        }

        let pwm_channels = self.pwm_channels.lock().unwrap();
        if let Some(pwm_arc) = pwm_channels.get(&channel_num) {
            let pwm = pwm_arc.lock().unwrap();

            pwm.set_duty_cycle(duty_cycle / 100f64).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
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
    /// pwm_manager.set_frequency(0, 500)
    /// ```
    #[pyo3(signature = (channel_num, frequency_hz))]
    fn set_frequency(&self, channel_num: u8, frequency_hz: f64) -> PyResult<()> {
        if frequency_hz <= 0f64 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Frequency must be greater than 0"));
        }
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


    #[pyo3(signature = (channel_num, period_ms))]
    fn set_period(&self, channel_num: u8, period_ms: f64) -> PyResult<()> {
        let pwm_channels = self.pwm_channels.lock().unwrap();
        if period_ms < 0f64 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Period must be greater than 0"));
        }
        if let Some(pwm_arc) = pwm_channels.get(&channel_num) {
            let pwm = pwm_arc.lock().unwrap();
            pwm.set_period(Duration::from_secs_f64(period_ms * 1000f64)).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PWM channel not initialized"))
        }
    }

    #[pyo3(signature = (channel_num, pulse_width_ms))]
    fn set_pulse_width(&self, channel_num: u8, pulse_width_ms: f64) -> PyResult<()> {
        let pwm_channels = self.pwm_channels.lock().unwrap();
        if let Some(pwm_arc) = pwm_channels.get(&channel_num) {
            let pwm = pwm_arc.lock().unwrap();
            let current_period = pwm.period().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
            let pulse_width = Duration::from_secs_f64(pulse_width_ms * 1000f64);
            if pulse_width > current_period {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Pulse width must be less than the period"));
            }
            pwm.set_pulse_width(pulse_width).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
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

    #[pyo3(signature = (channel_num))]
    fn get_period(&self, channel_num: u8) -> PyResult<f64> {
        let pwm_channels = self.pwm_channels.lock().unwrap();

        if let Some(pwm_arc) = pwm_channels.get(&channel_num) {
            let pwm = pwm_arc.lock().unwrap();
            if let Ok(period) = pwm.period() {
                Ok(period.as_secs_f64() * 1000f64)
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to get PWM frequency"))
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PWM channel not initialized"))
        }
    }


    #[pyo3(signature = (channel_num))]
    fn get_pulse_width(&self, channel_num: u8) -> PyResult<f64> {
        let pwm_channels = self.pwm_channels.lock().unwrap();

        if let Some(pwm_arc) = pwm_channels.get(&channel_num) {
            let pwm = pwm_arc.lock().unwrap();
            if let Ok(pulse_width) = pwm.pulse_width() {
                Ok(pulse_width.as_secs_f64())
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
    /// - `float`: The current duty cycle (0 to 100).
    ///
    /// Example usage:
    /// ```python
    /// duty_cycle = pwm_manager.get_duty_cycle(0)
    /// ```
    #[pyo3(signature = (channel_num))]
    fn get_duty_cycle(&self, channel_num: u8) -> PyResult<f64> {
        let pwm_channels = self.pwm_channels.lock().unwrap();

        if let Some(pwm_arc) = pwm_channels.get(&channel_num) {
            let pwm = pwm_arc.lock().unwrap();
            if let Ok(duty_cycle) = pwm.duty_cycle() {
                Ok(duty_cycle * 100f64)
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to get PWM duty cycle"))
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PWM channel not initialized"))
        }
    }


    #[pyo3(signature = ())]
    fn cleanup(&self) -> PyResult<()> {
        let pwm_channels = self.pwm_channels.lock().unwrap();
        let channel_nums: Vec<u8> = pwm_channels.keys().cloned().collect();
        drop(pwm_channels);

        // Stop all PWM channels that are active
        for pin_num in channel_nums {
            self.reset_pwm_channel(pin_num)?;
        }

        let mut pwm_channels = self.pwm_channels.lock().unwrap();
        pwm_channels.clear();
        Ok(())
    }
}
