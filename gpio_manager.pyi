class GPIOManager:
    """GPIOManager provides methods to manage GPIO pins and register callbacks."""

    def __init__(self) -> None:
        """Initializes a new GPIOManager instance."""
        ...

    def add_input_pin(self, pin_num: int, pull_resistor_state: InternPullResistorState = InternPullResistorState.AUTO, logic_level: LogicLevel = LogicLevel.HIGH) -> None:
        """
        Sets up an input pin but does not assign a callback yet.

        :param pin_num: The GPIO pin to configure as input.
        :param pull_resistor_state: The pin state (set it by using gpio_manager.InternPullResistorState.[PULLUP, PULLDOWN, EXTERNAL, or AUTO]).
        :param logic_level: The logic level of the pin (set it by using gpio_manager.LogicLevel.[HIGH or LOW]).
        """
        ...

    def assign_callback(self, pin_num: int, trigger_edge: TriggerEdge, callback: Callable, args: Optional[Tuple] = None, debounce_time_ms: int = 2) -> None:
        """
        Assigns a callback to an input pin.

        :param pin_num: The GPIO pin.
        :param trigger_edge: The edge trigger type (set using gpio_manager.TriggerEdge.[RISING, FALLING, BOTH]).
        :param callback: The callback function to be invoked on pin change.
        :param args: The arguments to pass to the callback function.
        :param debounce_time_ms: The debounce time in milliseconds.
        """
        ...

    def add_output_pin(self, pin_num: int, pin_state: OPinState = OPinState.LOW, logic_level: LogicLevel = LogicLevel.HIGH) -> None:
        """
        Sets up an output pin.

        :param pin_num: The GPIO pin to configure as output.
        :param pin_state: The initial state of the pin (set it by using gpio_manager.OPinState.[HIGH or LOW]).
        :param logic_level: The logic level of the pin (set it by using gpio_manager.LogicLevel.[HIGH or LOW]).
        """
        ...

    def set_output_pin(self, pin_num: int, pin_state: OPinState) -> None:
        """
        Sets the state of an output pin.

        :param pin_num: The GPIO pin.
        :param pin_state: The desired state (set it by using gpio_manager.OPinState.[HIGH or LOW]).
        """
        ...

    def get_pin(self, pin_num: int) -> OPinState:
        """
        Polls the current state of an input pin.

        :param pin_num: The GPIO pin to get.
        :return: The current state of the pin (check it by using gpio_manager.OPinState.[HIGH or LOW]).
        """
        ...

    def unassign_callback(self, pin_num: int) -> None:
        """
        Unassigns a callback from an input pin.

        :param pin_num: The GPIO pin whose callback is to be reset.
        """
        ...

    def wait_for_edge(self, pin_num: int, trigger_edge: TriggerEdge = TriggerEdge.BOTH, timeout_ms: int = -1) -> None:
        """
        Waits for an edge on the assigned pin. This function block for the given timeout, or waits forever if it is set to a negative number.

        :param pin_num: The GPIO pin.
        :param trigger_edge: The trigger type (set using gpio_manager.TriggerEdge.[RISING, FALLING, BOTH]).
        :param timeout_ms: Timeout in milliseconds.
        """
        ...

    def setup_pwm(self, pin_num, frequency_hz: int = 60, duty_cycle: int = 0, logic_level: LogicLevel = LogicLevel.HIGH) -> None:
        """
        Sets up a PWM signal on the given pin. If The pin must be set up as an output pin before calling this
        function, the values for the logic level and current state will be preserved otherwise the default values
        will be used when setting up pwm for the pin (initial output low and logic high).

        :param pin_num: The GPIO pin.
        :param frequency_hz: The period of the pwm signal in hertz.
        :param duty_cycle: The pulse width of the pwm signal as a percentage of the frequency (Duty cycle must be between 0 and 100).
        :param logic_level: The logic level of the pin (set it by using gpio_manager.LogicLevel.[HIGH or LOW]).
        """
        ...

    def set_pwm_duty_cycle(self, pin_num: int, duty_cycle: int) -> None:
        """
        Sets the PWM signal's duty cycle.
        :param pin_num: The GPIO pin.
        :param duty_cycle: The pulse width of the pwm signal as a percentage of the frequency (Duty cycle must be between 0 and 100).
        """
        ...

    def set_pwm_frequency(self, pin_num: int, frequency_hz: int) -> None:
        """
        Sets the PWM signal's frequency.
        :param pin_num: The GPIO pin.
        :param frequency_hz: The period of the pwm signal in hertz.
        """
        ...

    def start_pwm(self, pin_num: int) -> None:
        """
        Starts the PWM signal.
        :param pin_num: The GPIO pin.
        """
        ...

    def stop_pwm(self, pin_num: int) -> None:
        """
        Stops the PWM signal.
        :param pin_num: The GPIO pin.
        """
        ...

    def reset_pin(self, pin_num: int) -> None:
        """
        Resets the given pin so it can set to either input or output.
        :param pin_num: The GPIO pin.
        """
    ...

    def cleanup(self) -> None:
        """
        Cleans up the GPIO pins by setting all output pins to low and clearing all interrupts.
        """
        ...


class PWMManager:
    """PWMManager provides methods to manage PWM channels."""

    def __init__(self) -> None:
        """Initializes a new PWMManager instance."""
        ...

    def setup_pwm_channel(self, channel_num: int, frequency_hz: float = 60.0, duty_cycle: float = 0.5, polarity: 'PWMPolarity' = 'PWMPolarity.NORMAL') -> None:
        """
        Sets up a PWM channel with the specified parameters.

        :param channel_num: The PWM channel number (0 or 1).
        :param frequency_hz: The frequency in Hertz.
        :param duty_cycle: The duty cycle (0.0 to 1.0).
        :param polarity: The polarity of the PWM signal (set using PWMPolarity.[NORMAL or INVERSE]).
        """
        ...

    def start_pwm_channel(self, channel_num: int) -> None:
        """
        Starts the PWM signal on the specified channel.

        :param channel_num: The PWM channel number (0 or 1).
        """
        ...

    def stop_pwm_channel(self, channel_num: int) -> None:
        """
        Stops the PWM signal on the specified channel.

        :param channel_num: The PWM channel number (0 or 1).
        """
        ...

    def remove_pwm_channel(self, channel_num: int) -> None:
        """
        Removes the PWM channel from the manager.

        :param channel_num: The PWM channel number (0 or 1).
        """
        ...

    def set_duty_cycle(self, channel_num: int, duty_cycle: float) -> None:
        """
        Sets the duty cycle for the specified PWM channel.

        :param channel_num: The PWM channel number (0 or 1).
        :param duty_cycle: The new duty cycle (0.0 to 1.0).
        """
        ...

    def set_frequency(self, channel_num: int, frequency_hz: float) -> None:
        """
        Sets the frequency for the specified PWM channel.

        :param channel_num: The PWM channel number (0 or 1).
        :param frequency_hz: The new frequency in Hertz.
        """
        ...

    def get_frequency(self, channel_num: int) -> float:
        """
        Gets the current frequency of the specified PWM channel.

        :param channel_num: The PWM channel number (0 or 1).
        :return: The current frequency in Hertz.
        """
        ...

    def get_duty_cycle(self, channel_num: int) -> float:
        """
        Gets the current duty cycle of the specified PWM channel.

        :param channel_num: The PWM channel number (0 or 1).
        :return: The current duty cycle (0.0 to 1.0).
        """
        ...
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


class PWMPolarity:
    """Enum representing the PWM polarity options."""
    NORMAL: 'PWMPolarity'
    """
    Normal polarity (default).
    """
    INVERSE: 'PWMPolarity'
    """
    Inverse polarity.
    """

class InternPullResistorState:
    """Enum representing the GPIO pin state types for input pins."""
    PULLUP: 'InternPullResistorState'
    """
    Pulls the pin up to VCC.
    """
    PULLDOWN: 'InternPullResistorState'
    """
    Pulls the pin down to ground.
    """
    EXTERNAL: 'InternPullResistorState'
    """
    Don't use the internal pull resistor.
    """
    AUTO: 'InternPullResistorState'
    """
    Automatically picks the pull resistor based on the pin logic level.
    """

class OPinState:
    """Enum representing the GPIO pin state types for output pins. The state represents the logic state of the pin. The voltage will be set based on the logic level."""
    HIGH: 'OPinState'
    """
    Sets the pin to Logic HIGH.
    """
    LOW: 'OPinState'
    """
    Sets the pin to Logic LOW.
    """

class LogicLevel:
    """Enum representing the logic levels of the pins."""
    HIGH: 'LogicLevel'
    LOW: 'LogicLevel'

class TriggerEdge:
    """Enum representing the trigger edge types. Triggers are based off logic level changes"""
    RISING: 'TriggerEdge'
    """
    Trigger on the rising edge. (from Logic LOW to Logic HIGH)
    """
    FALLING: 'TriggerEdge'
    """
    Trigger on the falling edge. (from Logic HIGH to Logic LOW)
    """
    BOTH: 'TriggerEdge'
    """
    Trigger on both edges.
    """