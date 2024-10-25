GPIO Manager
============

.. automodule:: gpio_manager.GPIOManager
   :members:
   :undoc-members:

Enums
-----
The following enums are used in this class. See the `Enums <enums.html>`_ page for details:
- `InternPullResistorState`
- `PinState`
- `LogicLevel`
- `TriggerEdge`

GPIOManager Class
-----------------
The `GPIOManager` class provides methods to manage GPIO pins, register callbacks, and handle PWM signals.

Methods
-------
- **add_input_pin**:
   Sets up an input pin without assigning a callback.

   **Parameters**:

   - `pin_num` (int): The GPIO pin to configure as input.

   - `pull_resistor_state` (InternPullResistorState): Pull resistor state (PULLUP, PULLDOWN, EXTERNAL, AUTO).

   - `logic_level` (LogicLevel): The logic level of the pin (HIGH, LOW).

- **assign_callback**:
   Assigns a callback function to an input pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.

   - `callback` (Callable): The function to invoke on pin change.

   - `trigger_edge` (TriggerEdge): The edge trigger (RISING, FALLING, BOTH).

   - `args` (Optional[Tuple]): Arguments to pass to the callback function.

   - `debounce_time_ms` (int): Debounce time in milliseconds.

- **add_output_pin**:
   Sets up an output pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin to configure as output.

   - `pin_state` (PinState): Initial state of the pin (HIGH, LOW).

   - `logic_level` (LogicLevel): Logic level of the pin (HIGH, LOW).

- **set_output_pin**:
   Sets the state of an output pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.

   - `pin_state` (PinState): Desired pin state (HIGH, LOW).

- **get_pin**:
   Polls the current state of an input pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin to get.

   **Returns**:
   - (PinState): The current state of the pin.

- **unassign_callback**:
   Unassigns a callback from an input pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin whose callback is to be reset.

- **wait_for_edge**:
   Waits for an edge trigger on an input pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.

   - `trigger_edge` (TriggerEdge): Trigger type (RISING, FALLING, BOTH).

   - `timeout_ms` (int): Timeout in milliseconds.

- **setup_pwm**:
   Sets up a PWM signal on the given pin. If The pin must be set up as an output pin before calling this
    function, the values for the logic level and current state will be preserved otherwise the default values
    will be used when setting up pwm for the pin (initial output low and logic high).

    The value of frequency_hz and duty_cycle overwrites period_ms and pulse_width_ms if they are set.
    If neither frequency_hz and duty_cycle nor period_ms and pulse_width_ms are set, the default value of 1000 hz
    and a duty_cycle of 0 are used.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.

   - `frequency_hz` (float): Frequency of the PWM signal in Hertz.

   - `duty_cycle` (float): Duty cycle as a percentage.

   - `period_ms` (float): PWM period in milliseconds.

   - `pulse_width_ms` (float): Pulse width in milliseconds.

   - `logic_level` (LogicLevel): The logic level of the PWM signal (HIGH, LOW).

- **set_pwm_duty_cycle**:
   Sets the PWM signal's duty cycle.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.

   - `duty_cycle` (float): Duty cycle as a percentage (0-100).

- **set_pwm_frequency**:
   Sets the PWM signal's frequency.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.

   - `frequency_hz` (float): Frequency in Hertz.

- **start_pwm**:
   Starts the PWM signal on the specified pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.

- **stop_pwm**:
   Stops the PWM signal.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.

- **reset_pin**:
   Resets the given pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.

- **cleanup**:
   Cleans up the GPIO pins by setting all output pins to low and clearing all interrupts.
