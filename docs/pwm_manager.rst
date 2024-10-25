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
The `PWMManager` class provides methods to control PWM channels.

Methods
-------
- **setup_pwm_channel**:
   Sets up a PWM channel with the specified parameters.
    The value of frequency_hz and duty_cycle overwrites period_ms and pulse_width_ms if they are set.
    If neither frequency_hz and duty_cycle nor period_ms and pulse_width_ms are set, the default value of 1000 hz
    and a duty_cycle of 0 are used.


   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   - `frequency_hz` (float): Frequency in Hertz.

   - `duty_cycle` (float): Duty cycle as a percentage (0-100).

   - `period_ms` (float): Period in milliseconds.

   - `pulse_width_ms` (float): Pulse width in milliseconds.

   - `logic_level` (LogicLevel): Logic level of the PWM signal (HIGH, LOW).

- **start_pwm_channel**:
   Starts the PWM signal on the specified channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

- **stop_pwm_channel**:
   Stops the PWM signal.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

- **reset_pwm_channel**:
   Resets the PWM channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

- **set_duty_cycle**:
   Sets the duty cycle for the specified PWM channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   - `duty_cycle` (float): Duty cycle as a percentage (0-100).

- **set_frequency**:
   Sets the frequency for the specified PWM channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   - `frequency_hz` (float): Frequency in Hertz.

- **get_frequency**:
   Gets the current frequency of the specified PWM channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   **Returns**:
   - (float): The current frequency in Hertz.

- **get_duty_cycle**:
   Gets the current duty cycle of the specified PWM channel.

   **Parameters**:

   - `channel_num` (int): The PWM channel number (0 or 1).

   **Returns**:
   - (float): The current duty cycle as a percentage (0-100).

- **cleanup**:
   Cleans up all PWM channels.
