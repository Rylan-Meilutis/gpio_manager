from typing import Optional, Tuple, Callable


class GPIOManager:
    """GPIOManager provides methods to manage GPIO pins and register callbacks."""

    def __init__(self) -> None:
        """Initializes a new GPIOManager instance."""
        ...

    def add_input_pin(self, pin_num: int,
                      pull_resistor_state: Optional[InternPullResistorState] = InternPullResistorState.AUTO,
                      logic_level: Optional[LogicLevel] = LogicLevel.HIGH) -> None:
        """
        Sets up an input pin but does not assign a callback yet.

        :param pin_num: The GPIO pin to configure as input.
        :param pull_resistor_state: The pin state (set it by using gpio_manager.InternPullResistorState.[PULLUP, PULLDOWN, EXTERNAL, or AUTO]).
        :param logic_level: The logic level of the pin (set it by using gpio_manager.LogicLevel.[HIGH or LOW]).
        """
        ...

    def assign_callback(self, pin_num: int, callback: Callable[..., None], trigger_edge: Optional[TriggerEdge] =
    TriggerEdge.BOTH, debounce_time_ms: Optional[float] = 2, args: Optional[Tuple] = None, include_trigger_time:
    Optional[bool] = False, include_trigger_edge: Optional[bool] = False) -> None:
        """
        Assigns a callback to an input pin. If enabled, TriggerTime is a float representing the time the trigger occurred since unix time epoch. TriggerEdge is an enum representing the edge that triggered the
        callback (gpio_manager.TriggerEdge.[RISING, FALLING]). You can assign more than one callback to each pin by calling this function multiple times with different 
        callbacks. (Note) The debounce time is only assigned the first time a callback is assigned to a pin. If you want to change the debounce time, you must unassign the 
        callback and reassign it with the new debounce time.

        :param pin_num: The GPIO pin.
        :param callback: The callback function to be invoked on pin change.
        :param trigger_edge: The edge trigger type (set using gpio_manager.TriggerEdge.[RISING, FALLING, BOTH]).
        :param args: The arguments to pass to the callback function.
        :param debounce_time_ms: The debounce time in milliseconds.
        :param include_trigger_time: Whether to include the trigger time in the callback. (Will be the first argument)
        :param include_trigger_edge: Whether to include the trigger edge in the callback. (Will be the second argument if include_trigger_time is True, otherwise the first
        argument)
        """
        ...

    def add_output_pin(self, pin_num: int, pin_state: Optional[PinState] = PinState.LOW,
                       logic_level: Optional[LogicLevel] = LogicLevel.HIGH) -> None:
        """
        Sets up an output pin.

        :param pin_num: The GPIO pin to configure as output.
        :param pin_state: The initial state of the pin (set it by using gpio_manager.PINState.[HIGH or LOW]).
        :param logic_level: The logic level of the pin (set it by using gpio_manager.LogicLevel.[HIGH or LOW]).
        """
        ...

    def set_output_pin(self, pin_num: int, pin_state: PinState) -> None:
        """
        Sets the state of an output pin.

        :param pin_num: The GPIO pin.
        :param pin_state: The desired state (set it by using gpio_manager.PINState.[HIGH or LOW]).
        """
        ...

    def get_pin(self, pin_num: int) -> PinState:
        """
        Polls the current state of an input pin.

        :param pin_num: The GPIO pin to get.
        :return: The current state of the pin (check it by using gpio_manager.PINState.[HIGH or LOW]).
        """
        ...

    def unassign_callbacks(self, pin_num: int) -> None:
        """
        Unassigns all callbacks from an input pin.

        :param pin_num: The GPIO pin whose callback is to be reset.
        """
        ...

    def unassign_callback(self, pin_num: int, callback: Callable[..., None]) -> None:
        """
        Unassigns a specific callback from an input pin.

        :param pin_num: The GPIO pin whose callback is to be reset.
        :param callback: The callback function to be removed from the input pin.
        """
    ...

    def wait_for_edge(self, pin_num: int, trigger_edge: Optional[TriggerEdge] = TriggerEdge.BOTH, timeout_ms:
    Optional[float] = None, debounce_ms: Optional[float] = 2) -> None:
        """
        Waits for an edge on the assigned pin. This function block for the given timeout, or waits forever if it is 
        set to a negative number or None.

        :param pin_num: The GPIO pin.
        :param trigger_edge: The trigger type (set using gpio_manager.TriggerEdge.[RISING, FALLING, BOTH]).
        :param timeout_ms: Timeout in milliseconds.
        :param debounce_ms: Debounce time in milliseconds.
        """
        ...

    def setup_pwm(self, pin_num, frequency_hz: Optional[float] = None, duty_cycle: Optional[float] = None,
                  period_ms: Optional[float] = None,
                  pulse_width_ms: Optional[float] = None, logic_level: Optional[LogicLevel] = LogicLevel.HIGH) -> None:
        """
        Sets up a PWM signal on the given pin. If The pin must be set up as an output pin before calling this
        function, the values for the logic level and current state will be preserved otherwise the default values
        will be used when setting up pwm for the pin (initial output low and logic high).

        The value of frequency_hz and duty_cycle overwrites period_ms and pulse_width_ms if they are set.
        If neither frequency_hz and duty_cycle nor period_ms and pulse_width_ms are set, the default value of 1000 hz
        and a duty_cycle of 0 are used.

        :param pin_num: The GPIO pin.
        :param frequency_hz: The period of the pwm signal in hertz.
        :param duty_cycle: The pulse width of the pwm signal as a percentage of the frequency (Duty cycle must be between 0 and 100).
        :param period_ms: The period in milliseconds.
        :param pulse_width_ms: The pulse width in milliseconds.
        :param logic_level: The logic level of the pin (set it by using gpio_manager.LogicLevel.[HIGH or LOW]).
        """
        ...

    def set_pwm_duty_cycle(self, pin_num: int, duty_cycle: float) -> None:
        """
        Sets the PWM signal's duty cycle.
        :param pin_num: The GPIO pin.
        :param duty_cycle: The pulse width of the pwm signal as a percentage of the frequency (Duty cycle must be between 0 and 100).
        """
        ...

    def set_pwm_frequency(self, pin_num: int, frequency_hz: float) -> None:
        """
        Sets the PWM signal's frequency.
        :param pin_num: The GPIO pin.
        :param frequency_hz: The period of the pwm signal in hertz.
        """
        ...

    def set_pwm_period(self, pin_num: int, period_ms: float) -> None:
        """
        Sets the PWM signal's period.
        :param pin_num: The GPIO pin.
        :param period_ms: The period in milliseconds.
        """
        ...

    def set_pwm_pulse_width(self, pin_num: int, pulse_width_ms: float) -> None:
        """
        Sets the PWM signal's pulse width.
        :param pin_num: The GPIO pin.
        :param pulse_width_ms: The pulse width in milliseconds.
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
