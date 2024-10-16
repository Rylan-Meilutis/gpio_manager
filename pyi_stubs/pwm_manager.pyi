class PWMManager:
    """PWMManager provides methods to manage PWM channels."""

    def __init__(self) -> None:
        """Initializes a new PWMManager instance."""
        ...

    def setup_pwm_channel(self, channel_num: int, frequency_hz: float = 60.0, duty_cycle: int = 00, polarity: 'PWMPolarity' = 'PWMPolarity.NORMAL') -> None:
        """
        Sets up a PWM channel with the specified parameters.

        :param channel_num: The PWM channel number (0 or 1).
        :param frequency_hz: The frequency in Hertz.
        :param duty_cycle: The duty cycle (0 to 100).
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

    def get_frequency(self, channel_num: int) -> float:
        """
        Gets the current frequency of the specified PWM channel.

        :param channel_num: The PWM channel number (0 or 1).
        :return: The current frequency in Hertz.
        """
        ...

    def get_duty_cycle(self, channel_num: int) -> int:
        """
        Gets the current duty cycle of the specified PWM channel.

        :param channel_num: The PWM channel number (0 or 1).
        :return: The current duty cycle (0 to 100).
        """
        ...

    def reset(self, channel_num) -> None:
        """
        :param channel_num: The PWM channel number (0 or 1).
        """

    def cleanup(self) -> None:
        """
        Sets all PWM channels to the disabled state and clears them from the set list
        """