mod gpio_module;
mod pwm_module;
mod i2c_module;
mod pinctrl;


use pyo3::prelude::*;
use pyo3::PyObject;
use rppal::gpio::{InputPin, OutputPin};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn compute_pwm_values(frequency_hz: &Option<f64>, duty_cycle: &Option<f64>, period_ms: &Option<f64>, pulse_width_ms: &Option<f64>) -> (f64, f64) {
    let frequency = match period_ms {
        Some(period_ms) => {
            1f64 / (period_ms / 1000f64)
        }
        None => { -1f64 }
    };

    let frequency = match frequency_hz {
        Some(frequency) => {
            frequency.clone()
        }
        None => {
            if period_ms.is_some() {
                frequency.clone()
            } else {
                1000f64
            }
        }
    };

    let duty_cycle_percent = match pulse_width_ms {
        Some(pulse_width) => {
            if frequency > 0f64 {
                (pulse_width / (1f64 / frequency * 1000f64)) * 100f64
            } else { 0f64 }
        }
        None => { -1f64 }
    };


    let duty_cycle_percent = match duty_cycle {
        Some(duty_cycle) => {
            duty_cycle.clone()
        }
        None => {
            if pulse_width_ms.is_some() {
                duty_cycle_percent
            } else {
                0f64
            }
        }
    };

    (frequency, duty_cycle_percent)
}
pub fn check_pwm_values(frequency_hz: &Option<f64>, duty_cycle: &Option<f64>, period_ms: &Option<f64>, pulse_width_ms: &Option<f64>) -> PyResult<()> {
    if duty_cycle.is_some() && (duty_cycle.unwrap() > 100f64 || duty_cycle.unwrap() < 0f64) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Duty cycle must be between 0 and 100, The value {} does not meet this condition", duty_cycle.unwrap())));
    }
    if period_ms.is_some() && period_ms.unwrap() < 0f64 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Period must be greater than 0, The value {} does not meet this condition", period_ms.unwrap())));
    }
    if pulse_width_ms.is_some() && pulse_width_ms.unwrap() < 0f64 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Pulse width must be greater than 0, The value {} does not meet this condition", pulse_width_ms.unwrap())));
    }
    if pulse_width_ms.is_some() && period_ms.is_some() && pulse_width_ms.unwrap() > period_ms.unwrap() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Pulse width must be less than the period, The value {} does not meet this condition", pulse_width_ms.unwrap())));
    }
    if frequency_hz.is_some() && frequency_hz.unwrap() < 0f64 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Frequency must be greater than 0, The value {} does not meet this condition", frequency_hz.unwrap())));
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct Callback {
    callable: Arc<Mutex<PyObject>>,
    trigger_edge: TriggerEdge,
    args: Arc<Mutex<PyObject>>,
    send_time: bool,
    send_edge: bool,
}

pub struct PinManager {
    input_pins: HashMap<u8, Arc<Mutex<Pin>>>,
    output_pins: HashMap<u8, Arc<Mutex<Pin>>>,
    callbacks: HashMap<u8, Vec<Callback>>,
    pwm_setup: HashMap<u8, PwmConfig>,
}


struct PwmConfig {
    frequency: f64,
    duty_cycle: f64,
    logic_level: LogicLevel,
    is_active: bool,
}


#[derive(Clone)]
enum PinType {
    Input(Arc<Mutex<InputPin>>),
    Output(Arc<Mutex<OutputPin>>),
}


#[derive(Clone)]
struct Pin {
    pin: PinType,
    logic_level: LogicLevel,
}


#[pyclass(eq)]
#[derive(Clone, Copy, Debug, PartialEq)]
/// Enum representing the GPIO pin state types.
pub enum InternPullResistorState {
    PULLUP,
    PULLDOWN,
    EXTERNAL,
    AUTO,
}


#[pyclass(eq)]
#[derive(Clone, Copy, Debug, PartialEq)]
/// Enum representing the GPIO pin state types.
pub enum PinState {
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


#[pymodule]
fn gpio_manager(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<gpio_module::GPIOManager>()?;
    m.add_class::<pwm_module::PWMManager>()?;
    m.add_class::<i2c_module::I2CManager>()?;
    m.add_class::<InternPullResistorState>()?;
    m.add_class::<PinState>()?;
    m.add_class::<LogicLevel>()?;
    m.add_class::<TriggerEdge>()?;
    Ok(())
}

