class PWMManager:
    """PWMManager provides methods to manage PWM channels."""

    def __init__(self) -> None:
        """Initializes a new PWMManager instance."""
        ...

    def setup_pwm_channel(self, channel_num: int, frequency_hz: float = 60.0, duty_cycle: float = 0,
                          period_ms=0, pulse_width_ms=0, logic_level: 'LogicLevel' = LogicLevel.HIGH) -> None:
        """
        Sets up a PWM channel with the specified parameters.
        The value of frequency_hz and duty_cycle overwrites period_ms and pulse_width_ms if they are set.
        If neither frequency_hz and duty_cycle nor period_ms and pulse_width_ms are set, the default value of 1000 hz
        and a duty_cycle of 0 are used.

        :param channel_num: The PWM channel number (0 or 1).
        :param frequency_hz: The frequency in Hertz.
        :param duty_cycle: The duty cycle (0 to 100).
        :param period_ms: The period in milliseconds.
        :param pulse_width_ms: The pulse width in milliseconds.
        :param logic_level: The Logic level of the PWM signal (set using LogicLevel.[NORMAL or INVERSE]).
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

    def reset_pwm_channel(self, channel_num: int) -> None:
        """
        resets the PWM channel and removes it from the manager.

        :param channel_num: The PWM channel number (0 or 1).
        """
        ...

    def set_duty_cycle(self, channel_num: int, duty_cycle: int) -> None:
        """
        Sets the duty cycle for the specified PWM channel.

        :param channel_num: The PWM channel number (0 or 1).
        :param duty_cycle: The new duty cycle (0 to 100).
        """
        ...

    def set_frequency(self, channel_num: int, frequency_hz: float) -> None:
        """
        Sets the frequency for the specified PWM channel.

        :param channel_num: The PWM channel number (0 or 1).
        :param frequency_hz: The new frequency in Hertz.
        """
        ...

    def set_period(self, channel_num: int, period_ms: float) -> None:
        """
        Sets the period for the specified PWM channel in milliseconds.

        :param channel_num: The PWM channel number (0 or 1).
        :param period_ms: The new period in seconds.
        """

    ...

    def set_pulse_width(self, channel_num: int, pulse_width_ms: float) -> None:
        """
        Sets the pulse width for the specified PWM channel in milliseconds.

        :param channel_num: The PWM channel number (0 or 1).
        :param pulse_width_ms: The new pulse width in ms.
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
        :return: The current duty cycle (0 to 100).
        """
        ...

    def get_period(self, channel_num: int) -> float:
        """
        Gets the current period of the specified PWM channel.

        :param channel_num: The PWM channel number (0 or 1).
        :return: The current period in milliseconds.
        """
        ...

    def get_pulse_width(self, channel_num: int) -> float:
        """
        Gets the current pulse width of the specified PWM channel.

        :param channel_num: The PWM channel number (0 or 1).
        :return: The current pulse width in milliseconds.
        """
        ...

    def cleanup(self) -> None:
        """
        Sets all PWM channels to the disabled state and clears them from the set list
        """
