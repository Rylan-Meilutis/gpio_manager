mod gpio_module;
mod pwm_module;
mod i2c_module;

use pyo3::prelude::*;
use pyo3::PyObject;
use rppal::gpio::{InputPin, OutputPin};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct PinManager {
    input_pins: HashMap<u8, Arc<Mutex<Pin>>>,
    output_pins: HashMap<u8, Arc<Mutex<Pin>>>,
    callbacks: HashMap<u8, PyObject>,
    async_interrupts: HashMap<u8, bool>,
    pwm_setup: HashMap<u8, PwmConfig>,
}
struct PwmConfig {
    frequency: u64,
    duty_cycle: u64,
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

