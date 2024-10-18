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

    def assign_callback(self, pin_num: int, callback: Callable, trigger_edge: TriggerEdge = TriggerEdge.BOTH, args: Optional[Tuple] = None, debounce_time_ms: int = 2) -> None:
        """
        Assigns a callback to an input pin.

        :param pin_num: The GPIO pin.
        :param callback: The callback function to be invoked on pin change.
        :param trigger_edge: The edge trigger type (set using gpio_manager.TriggerEdge.[RISING, FALLING, BOTH]).
        :param args: The arguments to pass to the callback function.
        :param debounce_time_ms: The debounce time in milliseconds.
        """
        ...

    def add_output_pin(self, pin_num: int, pin_state: PINState = PINState.LOW, logic_level: LogicLevel = LogicLevel.HIGH) -> None:
        """
        Sets up an output pin.

        :param pin_num: The GPIO pin to configure as output.
        :param pin_state: The initial state of the pin (set it by using gpio_manager.PINState.[HIGH or LOW]).
        :param logic_level: The logic level of the pin (set it by using gpio_manager.LogicLevel.[HIGH or LOW]).
        """
        ...

    def set_output_pin(self, pin_num: int, pin_state: PINState) -> None:
        """
        Sets the state of an output pin.

        :param pin_num: The GPIO pin.
        :param pin_state: The desired state (set it by using gpio_manager.PINState.[HIGH or LOW]).
        """
        ...

    def get_pin(self, pin_num: int) -> PINState:
        """
        Polls the current state of an input pin.

        :param pin_num: The GPIO pin to get.
        :return: The current state of the pin (check it by using gpio_manager.PINState.[HIGH or LOW]).
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
