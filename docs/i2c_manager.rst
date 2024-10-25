I2C Manager
===========

.. automodule:: gpio_manager.I2CManager
   :members:
   :undoc-members:

I2CManager Class
----------------
The `I2CManager` class provides methods to manage I2C communication with slave devices.

Methods
-------
- **open**:
   Opens the I2C bus.

   **Parameters**:
   - `bus` (int): The I2C bus number to open (default is 1).

- **close**:
   Closes the I2C bus.

- **write_byte**:
   Writes a single byte to the I2C slave device.

   **Parameters**:
   - `addr` (int): The I2C slave address.
   - `data` (int): The byte to write.

- **block_write_byte**:
   Writes a single byte with a command to the I2C slave device.

   **Parameters**:
   - `addr` (int): The I2C slave address.
   - `command` (int): The command to send.
   - `data` (int): The byte to write.

- **read_byte**:
   Reads a single byte from the I2C slave device.

   **Parameters**:
   - `addr` (int): The I2C slave address.

   **Returns**:
   - (int): The byte read.

- **block_read_byte**:
   Reads a single byte with a command from the I2C slave device.

   **Parameters**:
   - `addr` (int): The I2C slave address.
   - `command` (int): The command to send before reading.

   **Returns**:
   - (int): The byte read.

- **write**:
   Writes data to the I2C slave device.

   **Parameters**:
   - `addr` (int): The I2C slave address.
   - `data` (bytes): The bytes to write.

- **block_write**:
   Writes data with a command to the I2C slave device.

   **Parameters**:
   - `addr` (int): The I2C slave address.
   - `command` (int): The command to send.
   - `data` (bytes): The bytes to write.

- **read**:
   Reads data from the I2C slave device.

   **Parameters**:
   - `addr` (int): The I2C slave address.
   - `length` (int): The number of bytes to read.

   **Returns**:
   - (bytes): The bytes read.

- **block_read**:
   Reads data with a command from the I2C slave device.

   **Parameters**:
   - `addr` (int): The I2C slave address.
   - `command` (int): The command to send before reading.
   - `length` (int): The number of bytes to read.

   **Returns**:
   - (bytes): The bytes read.

- **write_read**:
   Performs a write followed by a read operation.

   **Parameters**:
   - `addr` (int): The I2C slave address.
   - `write_data` (bytes): The bytes to write.
   - `read_length` (int): The number of bytes to read.

   **Returns**:
   - (bytes): The bytes read.

- **block_write_read**:
   Performs a block write followed by a block read operation.

   **Parameters**:
   - `addr` (int): The I2C slave address.
   - `command` (int): The command to send.
   - `write_data` (bytes): The bytes to write.
   - `read_length` (int): The number of bytes to read.

   **Returns**:
   - (bytes): The bytes read.
