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
- **Constructor**:
    Initializes the I2CManager class, the pins are in an unknown state until the bus is opened.

    **Example**::

          I2C_manager = gpio_manager.I2CManager()

- **open**:
   Opens the I2C bus.

   **Parameters**:

   - `bus` (Optional[int]): The I2C bus number to open (default is 1).

   **Example**::

        I2C_manager.open(bus=1)

- **close**:
   Closes the I2C bus.

   **Example**::

        I2C_manager.close()

- **write_byte**:
   Writes a single byte to the I2C slave device.

   **Parameters**:

   - `addr` (int): The I2C slave address.

   - `data` (int): The byte to write.

   **Example**::

        I2C_manager.write_byte(addr=0x1A, data=0xFF)

- **block_write_byte**:
   Writes a single byte with a command to the I2C slave device.

   **Parameters**:

   - `addr` (int): The I2C slave address.

   - `command` (int): The command to send.

   - `data` (int): The byte to write.

   **Example**::

        I2C_manager.block_write_byte(addr=0x1A, command=0x02, data=0xFF)

- **read_byte**:
   Reads a single byte from the I2C slave device.

   **Parameters**:

   - `addr` (int): The I2C slave address.

   **Returns**:
   - (int): The byte read.

   **Example**::

        data = I2C_manager.read_byte(addr=0x1A)

- **block_read_byte**:
   Reads a single byte with a command from the I2C slave device.

   **Parameters**:

   - `addr` (int): The I2C slave address.

   - `command` (int): The command to send before reading.

   **Returns**:
   - (int): The byte read.

   **Example**::

        data = I2C_manager.block_read_byte(addr=0x1A, command=0x02)

- **write**:
   Writes data to the I2C slave device.

   **Parameters**:

   - `addr` (int): The I2C slave address.

   - `data` (bytes): The bytes to write.

   **Example**::

        I2C_manager.write(addr=0x1A, data=b'\x01\x02\x03')

- **block_write**:
   Writes data with a command to the I2C slave device.

   **Parameters**:

   - `addr` (int): The I2C slave address.

   - `command` (int): The command to send.

   - `data` (bytes): The bytes to write.

   **Example**::

        I2C_manager.block_write(addr=0x1A, command=0x02, data=b'\x01\x02')

- **read**:
   Reads data from the I2C slave device.

   **Parameters**:

   - `addr` (int): The I2C slave address.

   - `length` (int): The number of bytes to read.

   **Returns**:
   - (bytes): The bytes read.

   **Example**::

        data = I2C_manager.read(addr=0x1A, length=3)

- **block_read**:
   Reads data with a command from the I2C slave device.

   **Parameters**:

   - `addr` (int): The I2C slave address.

   - `command` (int): The command to send before reading.

   - `length` (int): The number of bytes to read.

   **Returns**:
   - (bytes): The bytes read.

   **Example**::

        data = I2C_manager.block_read(addr=0x1A, command=0x02, length=3)

- **write_read**:
   Performs a write followed by a read operation.

   **Parameters**:

   - `addr` (int): The I2C slave address.

   - `write_data` (bytes): The bytes to write.

   - `read_length` (int): The number of bytes to read.

   **Returns**:
   - (bytes): The bytes read.

   **Example**::

        data = I2C_manager.write_read(addr=0x1A, write_data=b'\x01', read_length=3)

- **block_write_read**:
   Performs a block write followed by a block read operation.

   **Parameters**:

   - `addr` (int): The I2C slave address.

   - `command` (int): The command to send.

   - `write_data` (bytes): The bytes to write.

   - `read_length` (int): The number of bytes to read.

   **Returns**:
   - (bytes): The bytes read.

   **Example**::

        data = I2C_manager.block_write_read(addr=0x1A, command=0x02, write_data=b'\x01', read_length=3)

