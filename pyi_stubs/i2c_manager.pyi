class I2CManager:
    """I2CManager provides methods to manage I2C communication."""

    def __init__(self) -> None:
        """Initializes a new I2CManager instance."""
        ...

    def open(self, bus: int = 1) -> None:
        """
        Opens the I2C bus.

        :param bus: The I2C bus number to open (default is 1).
        """
        ...

    def close(self) -> None:
        """
        Closes the I2C bus.
        """
        ...

    def write_byte(self, addr: int, command: int, data: int) -> None:
        """
        Writes a single byte to the I2C slave device.

        :param addr: The I2C slave address.
        :param command: The command to send.
        :param data: The byte to write.
        """
        ...

    def read_byte(self, addr: int, command: int) -> int:
        """
        Reads a single byte from the I2C slave device.

        :param addr: The I2C slave address.
        :param command: The command to send before reading.
        :return: The byte read.
        """
        ...

    def write(self, addr: int, command: int, data: bytes) -> None:
        """
        Writes data to the I2C slave device.

        :param addr: The I2C slave address.
        :param command: The command to send.
        :param data: The bytes to write.
        """
        ...

    def read(self, addr: int, command: int, length: int) -> bytes:
        """
        Reads data from the I2C slave device.

        :param addr: The I2C slave address.
        :param command: The command to send before reading.
        :param length: The number of bytes to read.
        :return: The bytes read.
        """
        ...

    def write_read(self, addr: int, command: int, write_data: bytes, read_length: int) -> bytes:
        """
        Performs a write followed by a read operation.

        :param addr: The I2C slave address.
        :param command: The command to send.
        :param write_data: The bytes to write.
        :param read_length: The number of bytes to read.
        :return: The bytes read.
        """
        ...