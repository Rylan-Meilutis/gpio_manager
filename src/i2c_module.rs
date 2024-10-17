use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::{pyclass, pymethods, Py, PyErr, PyResult, Python};
use rppal::i2c::I2c;
use std::sync::{Arc, Mutex};

static I2C_MANAGER: Lazy<Arc<Mutex<I2CManager>>> = Lazy::new(|| {
    Arc::new(Mutex::new(I2CManager::new_singleton().expect("Failed to initialize I2CManager")))
});

#[pyclass]
/// I2CManager provides methods to manage I2C communication.
///
/// Example usage in Python:
///
/// ```python
/// i2c_manager = i2c_manager.I2CManager()
/// i2c_manager.open(bus=1)
/// i2c_manager.write_byte(0x20, 0xFF)
/// data = i2c_manager.read_byte(0x20)
/// i2c_manager.close()
/// ```
pub struct I2CManager {
    i2c: Arc<Mutex<Option<I2c>>>,
}

impl I2CManager {
    /// Internal method to initialize the I2CManager singleton.
    fn new_singleton() -> PyResult<Self> {
        Ok(Self {
            i2c: Arc::new(Mutex::new(None)),
        })
    }

    fn shared(py: Python) -> PyResult<Py<I2CManager>> {
        let manager = I2C_MANAGER.lock().unwrap();
        Py::new(py, I2CManager {
            i2c: Arc::clone(&manager.i2c),
        })
    }
}

#[pymethods]
impl I2CManager {
    #[new]
    /// Initializes a new I2CManager instance.
    ///
    /// Example usage:
    /// ```python
    /// i2c_manager = i2c_manager.I2CManager()
    /// ```
    fn new(py: Python) -> PyResult<Py<I2CManager>> {
        I2CManager::shared(py)
    }

    /// Opens the I2C bus.
    ///
    /// Parameters:
    /// - `bus` (int): The I2C bus number to open (default is 1).
    ///
    /// Example usage:
    /// ```python
    /// i2c_manager.open(bus=1)
    /// ```
    #[pyo3(signature = (bus = 1))]
    fn open(&self, bus: u8) -> PyResult<()> {
        let mut i2c_lock = self.i2c.lock().unwrap();
        if i2c_lock.is_some() {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("I2C bus already opened"));
        }

        let i2c = I2c::with_bus(bus)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to open I2C bus {}: {:?}", bus, e)))?;
        *i2c_lock = Some(i2c);
        Ok(())
    }

    /// Closes the I2C bus.
    ///
    /// Example usage:
    /// ```python
    /// i2c_manager.close()
    /// ```
    fn close(&self) -> PyResult<()> {
        let mut i2c_lock = self.i2c.lock().unwrap();
        if i2c_lock.is_none() {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("I2C bus is not opened"));
        }
        *i2c_lock = None;
        Ok(())
    }

    /// Writes a single byte to the I2C slave device.
    ///
    /// Parameters:
    /// - `addr` (int): The I2C slave address.
    /// - `data` (int): The byte to write.
    ///
    /// Example usage:
    /// ```python
    /// i2c_manager.write_byte(0x20, 0xFF)
    /// ```
    #[pyo3(signature = (addr, data))]
    fn write_byte(&self, addr: u16, data: u8) -> PyResult<()> {
        let mut i2c_lock = self.i2c.lock().unwrap();
        if let Some(ref mut i2c) = *i2c_lock {
            i2c.set_slave_address(addr)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to set slave address: {:?}", e)))?;

            // Send command and data
            i2c.write(&[data])
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to write byte: {:?}", e)))?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("I2C bus is not opened"))
        }
    }

    #[pyo3(signature = (addr, command, data))]
    fn block_write_byte(&self, addr: u16, command: u8, data: u8) -> PyResult<()> {
        let mut i2c_lock = self.i2c.lock().unwrap();
        if let Some(ref mut i2c) = *i2c_lock {
            i2c.set_slave_address(addr)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to set slave address: {:?}", e)))?;

            // Send command and data
            i2c.block_write(command, &[data])
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to write byte: {:?}", e)))?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("I2C bus is not opened"))
        }
    }

    /// Reads a single byte from the I2C slave device.
    ///
    /// Parameters:
    /// - `addr` (int): The I2C slave address.
    ///
    /// Returns:
    /// - `int`: The byte read.
    ///
    /// Example usage:
    /// ```python
    /// data = i2c_manager.read_byte(0x20)
    /// ```
    #[pyo3(signature = (addr, command))]
    fn block_read_byte(&self, addr: u16, command: u8) -> PyResult<u8> {
        let mut i2c_lock = self.i2c.lock().unwrap();
        if let Some(ref mut i2c) = *i2c_lock {
            i2c.set_slave_address(addr)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to set slave address: {:?}", e)))?;
            let mut buf = [0u8; 1];
            i2c.block_read(command, &mut buf)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to read byte: {:?}", e)))?;
            Ok(buf[0])
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("I2C bus is not opened"))
        }
    }

    #[pyo3(signature = (addr))]
    fn read_byte(&self, addr: u16) -> PyResult<u8> {
        let mut i2c_lock = self.i2c.lock().unwrap();
        if let Some(ref mut i2c) = *i2c_lock {
            i2c.set_slave_address(addr)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to set slave address: {:?}", e)))?;
            let mut buf = [0u8; 1];
            i2c.read(&mut buf)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to read byte: {:?}", e)))?;
            Ok(buf[0])
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("I2C bus is not opened"))
        }
    }

    /// Writes data to the I2C slave device.
    ///
    /// Parameters:
    /// - `addr` (int): The I2C slave address.
    /// - `data` (bytes): The data to write.
    ///
    /// Example usage:
    /// ```python
    /// i2c_manager.write( b'\x01\x02\x03')
    /// ```
    #[pyo3(signature = (addr, data))]
    fn write(&self, addr: u16, data: &Bound<'_, PyBytes>) -> PyResult<()> {
        let mut i2c_lock = self.i2c.lock().unwrap();
        if let Some(ref mut i2c) = *i2c_lock {
            i2c.set_slave_address(addr)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to set slave address: {:?}", e)))?;
            i2c.write(data.as_bytes())
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to write data: {:?}", e)))?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("I2C bus is not opened"))
        }
    }


    #[pyo3(signature = (addr, command, data))]
    fn block_write(&self, addr: u16, command: u8, data: &Bound<'_, PyBytes>) -> PyResult<()> {
        let mut i2c_lock = self.i2c.lock().unwrap();
        if let Some(ref mut i2c) = *i2c_lock {
            i2c.set_slave_address(addr)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to set slave address: {:?}", e)))?;
            i2c.block_write(command, data.as_bytes())
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to write data: {:?}", e)))?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("I2C bus is not opened"))
        }
    }


    /// Reads data from the I2C slave device.
    ///
    /// Parameters:
    /// - `addr` (int): The I2C slave address.
    /// - `length` (int): The number of bytes to read.
    ///
    /// Returns:
    /// - `bytes`: The data read.
    ///
    /// Example usage:
    /// ```python
    /// data = i2c_manager.read(0x20, 3)
    /// ```
    #[pyo3(signature = (addr, command, length))]
    fn block_read<'py>(&self, py: Python<'py>, addr: u16, command: u8, length: usize) -> PyResult<Bound<'py, PyBytes>> {
        let mut i2c_lock = self.i2c.lock().unwrap();
        if let Some(ref mut i2c) = *i2c_lock {
            i2c.set_slave_address(addr)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to set slave address: {:?}", e)))?;

            let mut buf = vec![0u8; length];
            i2c.block_read(command, &mut buf)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to read data: {:?}", e)))?;

            Ok(PyBytes::new_bound(py, &buf))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("I2C bus is not opened"))
        }
    }


    #[pyo3(signature = (addr, length))]
    fn read<'py>(&self, py: Python<'py>, addr: u16, length: usize) -> PyResult<Bound<'py, PyBytes>> {
        let mut i2c_lock = self.i2c.lock().unwrap();
        if let Some(ref mut i2c) = *i2c_lock {
            i2c.set_slave_address(addr)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to set slave address: {:?}", e)))?;

            let mut buf = vec![0u8; length];
            i2c.read(&mut buf)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to read data: {:?}", e)))?;

            Ok(PyBytes::new_bound(py, &buf))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("I2C bus is not opened"))
        }
    }

    /// Performs a write followed by a read operation.
    ///
    /// Parameters:
    /// - `addr` (int): The I2C slave address.
    /// - `write_data` (bytes): The data to write.
    /// - `read_length` (int): The number of bytes to read.
    ///
    /// Returns:
    /// - `bytes`: The data read.
    ///
    /// Example usage:
    /// ```python
    /// data = i2c_manager.write_read(0x20, b'\x01\x02', 3)
    /// ```
    #[pyo3(signature = (addr, write_data, read_length))]
    fn write_read<'py>(&self, py: Python<'py>, addr: u16, write_data: &Bound<'py, PyBytes>, read_length: usize) -> PyResult<Bound<'py, PyBytes>> {
        let mut i2c_lock = self.i2c.lock().unwrap();
        if let Some(ref mut i2c) = *i2c_lock {
            i2c.set_slave_address(addr)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to set slave address: {:?}", e)))?;
            let mut buf = vec![0u8; read_length];
            i2c.write_read(write_data.as_bytes(), &mut buf)
               .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to write data: {:?}", e)))?;
            Ok(PyBytes::new_bound(py, &buf))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("I2C bus is not opened"))
        }
    }

    #[pyo3(signature = (addr, command, write_data, read_length))]
    fn block_write_read<'py>(&self, py: Python<'py>, addr: u16, command: u8, write_data: &Bound<'py, PyBytes>, read_length: usize) -> PyResult<Bound<'py,
        PyBytes>> {
        self.block_write(addr, command, write_data)?;
        self.block_read(py, addr, write_data.as_bytes()[0], read_length)
    }
}
