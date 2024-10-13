class GPIOManager:
    """GPIOManager provides methods to manage GPIO pins and register callbacks."""

    def __init__(self) -> None:
        """Initializes a new GPIOManager instance."""
        ...

    def add_input_pin(self, pin_num: int, pin_state: IPinState = IPinState.NONE) -> None:
        """
        Sets up an input pin but does not assign a callback yet.

        :param pin_num: The GPIO pin to configure as input.
        :param pin_state: The pin state (set it by using gpio_manager.IPinState.[PULLUP, PULLDOWN, or NONE]).
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

    def set_pwm(self, pin_num: int, period_ms: int = 20, pulse_with_us: int = 0) -> None:
        """
        Sets up a PWM signal on the given pin.
        :param pin_num: The GPIO pin.
        :param period_ms: The period of the pwm signal in milliseconds.
        :param pulse_with_us: The pulse width of the pwm signal in microseconds.
        """
        ...

    def cleanup(self) -> None:
        """
        Cleans up the GPIO pins by setting all output pins to low and clearing all interrupts.
        """
        ...

class IPinState:
    """Enum representing the GPIO pin state types."""
    PULLUP: 'IPinState'
    PULLDOWN: 'IPinState'
    NONE: 'IPinState'

class OPinState:
    """Enum representing the GPIO pin state types."""
    HIGH: 'OPinState'
    LOW: 'OPinState'

class LogicLevel:
    """Enum representing the logic levels."""
    HIGH: 'LogicLevel'
    LOW: 'LogicLevel'

class TriggerEdge:
    """Enum representing the trigger edge types."""
    RISING: 'TriggerEdge'
    FALLING: 'TriggerEdge'
    BOTH: 'TriggerEdge'
