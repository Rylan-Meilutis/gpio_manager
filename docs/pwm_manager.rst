PWM Manager
===========

.. automodule:: gpio_manager.PWMManager
   :members:
   :undoc-members:

Enums
-----
The following enums are used in this class. See the `Enums <enums.html>`_ page for details:

- `LogicLevel`

PWMManager Class
----------------
The `PWMManager` class provides methods to control the hardware PWM channels (BCM Pin 18, and Pin 19).

Methods
-------
- **Constructor**:
   Initializes the PWMManager class, the PWM pins are in an unknown state until they are setup.

   **Example**::

       PWM_manager = gpio_manager.PWMManager()


- **setup_pwm_channel**:
   Sets up a PWM channel with the specified parameters.
    The value of frequency_hz and duty_cycle overwrites period_ms and pulse_width_ms if they are set.
    If neither frequency_hz or period_ms are set, the default value of 1000 hz is used.
    if neither duty_cycle or pulse_width_ms are set, the default value of 0% is used.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   - `frequency_hz` (Optional[float]): Frequency in Hertz. **Default**: 1000.

   - `duty_cycle` (Optional[float]): Duty cycle as a percentage (0-100). **Default**: 0.

   - `period_ms` (Optional[float]): Period in milliseconds. **Default**: 1.

   - `pulse_width_ms` (Optional[float]): Pulse width in milliseconds. **Default**: 0.

   - `logic_level` (Optional[LogicLevel]): Logic level of the PWM signal (HIGH, LOW). **Default**: HIGH.

   **Example**::

        PWM_manager.setup_pwm_channel(channel_num=0, frequency_hz=1000, duty_cycle=50)
        PWM_manager.setup_pwm_channel(channel_num=1, period_ms=1000, pulse_width_ms=500)

- **start_pwm_channel**:
   Starts the PWM signal on the specified channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   **Example**::

        PWM_manager.start_pwm_channel(channel_num=0)

- **stop_pwm_channel**:
   Stops the PWM signal.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   **Example**::

        PWM_manager.stop_pwm_channel(channel_num=0)

- **reset_pwm_channel**:
   Resets the PWM channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   **Example**::

        PWM_manager.reset_pwm_channel(channel_num=0)

- **set_duty_cycle**:
   Sets the duty cycle for the specified PWM channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   - `duty_cycle` (float): Duty cycle as a percentage (0-100).

   **Example**::

        PWM_manager.set_duty_cycle(channel_num=0, duty_cycle=75)

- **set_frequency**:
   Sets the frequency for the specified PWM channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   - `frequency_hz` (float): Frequency in Hertz.

   **Example**::

        PWM_manager.set_frequency(channel_num=0, frequency_hz=2000)

- **get_frequency**:
   Gets the current frequency of the specified PWM channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   **Returns**:
   - (float): The current frequency in Hertz.

   **Example**::

        current_frequency = PWM_manager.get_frequency(channel_num=0)

- **get_duty_cycle**:
   Gets the current duty cycle of the specified PWM channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   **Returns**:
   - (float): The current duty cycle as a percentage (0-100).

   **Example**::

        current_duty_cycle = PWM_manager.get_duty_cycle(channel_num=0)


- **set_pulse_width**:
   Sets the pulse width for the specified PWM channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   - `pulse_width_ms` (float): Pulse width in milliseconds.

   **Example**::

        PWM_manager.set_pulse_width(channel_num=0, pulse_width_ms=500)


- **get_pulse_width**:
    Gets the current pulse width of the specified PWM channel.

    **Parameters**:

    - `channel_num` (int): The PWM channel number (0 or 1).

    **Returns**:
    - (float): The current pulse width in milliseconds.

    **Example**::

          current_pulse_width = PWM_manager.get_pulse_width(channel_num=0)


- **set_period**:
    Sets the period for the specified PWM channel.

    **Parameters**:

    - `channel_num` (int): The PWM channel number (0 or 1).

    - `period_ms` (float): Period in milliseconds.

    **Example**::

          PWM_manager.set_period(channel_num=0, period_ms=20)


- **get_period**:
    Gets the current period of the specified PWM channel.

    **Parameters**:

    - `channel_num` (int): The PWM channel number (0 or 1).

    **Returns**:
    - (float): The current period in milliseconds.

    **Example**::

          current_period = PWM_manager.get_period(channel_num=0)


- **cleanup**:
   Cleans up all PWM channels.

   **Example**::

        PWM_manager.cleanup()

