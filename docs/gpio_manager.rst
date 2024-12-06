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
- **Constructor**:
   Initializes the GPIOManager class. The GPIO will be in an unknown state until a pin is set up.

   **Example**::

       GPIO_manager = gpio_manager.GPIOManager()

- **add_input_pin**:
   Sets a pin as an input pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin to configure as input.
   - `pull_resistor_state` (Optional[InternPullResistorState]): Pull resistor state (PULLUP, PULLDOWN, EXTERNAL, AUTO). **Default**: AUTO.
   - `logic_level` (Optional[LogicLevel]): The logic level of the pin (HIGH, LOW). **Default**: HIGH.

   **Example**::

       GPIO_manager.add_input_pin(pin_num=17, pull_resistor_state=gpio_manager.InternPullResistorState.PULLUP, logic_level=gpio_manager.LogicLevel.HIGH)
       GPIO_manager.add_input_pin(pin_num=18)

- **assign_callback**:
   Assigns a callback function to an input pin. If enabled, TriggerTime is a float representing the time the trigger occurred since unix time epoch. TriggerEdge is an enum
   representing the edge that triggered the callback (gpio_manager.TriggerEdge.[RISING, FALLING]). You can assign multiple callbacks to the same pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.
   - `callback` (Callable): The function to invoke on pin change.
   - `trigger_edge` (Optional[TriggerEdge]): The edge trigger (RISING, FALLING, BOTH). **Default**: BOTH.
   - `debounce_time_ms` (Optional[int]): Debounce time in milliseconds. **Default**: 2.
   - `args` (Optional[Tuple]): Arguments to pass to the callback function. **Default**: None.
   - `include_trigger_time` (Optional[bool]): Whether to include the trigger time in the callback arguments. **Default**: False. (Note: parameter will be the first one passed to the  function.)
   - `include_previous_state` (Optional[bool]): Whether to include the previous state in the callback arguments. **Default**: False. (Note: parameter will be the second one passed to the function if include_trigger_time is true. Otherwise, it will be the first parameter.)


   **Example**::

         GPIO_manager.assign_callback(pin_num=15, callback=button_callback, trigger_edge=gpio_manager.TriggerEdge.FALLING, args=(15,), debounce_time_ms=50)
         GPIO_manager.assign_callback(pin_num=16, callback=button_callback, trigger_edge=gpio_manager.TriggerEdge.RISING)


   **Example Callback**::

      def button_callback(trigger_time: float, edge: gpio_manager.TriggerEdge, pin_num: int):
          if edge == gpio_manager.TriggerEdge.RISING:
              print(f"Button {pin_num} triggered at {trigger_time} on a rising edge")
          else:
              print(f"Button {pin_num} triggered at {trigger_time} on a falling edge")


   **This callback was setup with the following function call to trigger on pin 15**::

          GPIO_manager.assign_callback(15, button_callback, gpio_manager.TriggerEdge.BOTH, args=(15,), include_trigger_time=True, include_trigger_edge=True)


- **add_output_pin**:
   Sets up an output pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin to configure as output.
   - `pin_state` (Optional[PinState]): Initial state of the pin (HIGH, LOW). **Default**: LOW.
   - `logic_level` (Optional[LogicLevel]): Logic level of the pin (HIGH, LOW). **Default**: HIGH.

   **Example**::

       GPIO_manager.add_output_pin(pin_num=12, pin_state=gpio_manager.PinState.LOW, logic_level=gpio_manager.LogicLevel.HIGH)
       GPIO_manager.add_output_pin(pin_num=11)

- **set_output_pin**:
   Sets the state of an output pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.
   - `pin_state` (PinState): Desired pin state (HIGH, LOW).

   **Example**::

       GPIO_manager.set_output_pin(pin_num=12, pin_state=gpio_manager.PinState.HIGH)

- **get_pin**:
   Polls the current state of an input pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin to get.

   **Returns**:
   - (PinState): The current state of the pin.

   **Example**::

       current_state = GPIO_manager.get_pin(pin_num=12)

- **unassign_callback**:
   Unassigns the provided callback from an input pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin whose callback is to be reset.\
   - `callback` (Callable): The function to remove from the pin.

   **Example**::

       GPIO_manager.unassign_callback(pin_num=15, callback=button_callback)


- **unassign_callbacks**:
   Unassigns all callbacks from an input pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin whose callback is to be reset.\

   **Example**::

       GPIO_manager.unassign_callback(pin_num=15)


- **wait_for_edge**:
   Waits for an edge trigger on an input pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.
   - `trigger_edge` (Optional[TriggerEdge]): Trigger type (RISING, FALLING, BOTH). **Default**: BOTH.
   - `timeout_ms` (Optional[int]): Timeout in milliseconds. **Default**: None.

   **Example**::

       GPIO_manager.wait_for_edge(pin_num=15, trigger_edge=gpio_manager.TriggerEdge.FALLING, timeout_ms=1000)
       GPIO_manager.wait_for_edge(pin_num=16, trigger_edge=gpio_manager.TriggerEdge.RISING)

- **setup_pwm**:
   Sets up a PWM signal on the given pin. The pin must be set up as an output pin before calling this function.

   (Note) frequency_hz and period_ms are mutually exclusive. duty_cycle and pulse_width_ms are also mutually exclusive.
   If frequency_hz is set, period_ms will be ignored. If duty_cycle is set, pulse_width_ms will be ignored.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.
   - `frequency_hz` (Optional[float]): Frequency of the PWM signal in Hertz. **Default**: 1000.
   - `duty_cycle` (Optional[float]): Duty cycle as a percentage. **Default**: 0.
   - `period_ms` (Optional[float]): PWM period in milliseconds. **Default**: 1.
   - `pulse_width_ms` (Optional[float]): Pulse width in milliseconds. **Default**: 0.
   - `logic_level` (Optional[LogicLevel]): The logic level of the PWM signal (HIGH, LOW). **Default**: HIGH.

   **Example**::

       GPIO_manager.setup_pwm(pin_num=12, frequency_hz=1000, duty_cycle=50)
       GPIO_manager.setup_pwm(pin_num=11, period_ms=1000, pulse_width_ms=500)

- **set_pwm_duty_cycle**:
   Sets the PWM signal's duty cycle.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.
   - `duty_cycle` (float): Duty cycle as a percentage (0-100).

   **Example**::

       GPIO_manager.set_pwm_duty_cycle(pin_num=12, duty_cycle=75)

- **set_pwm_frequency**:
   Sets the PWM signal's frequency.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.
   - `frequency_hz` (float): Frequency in Hertz.

   **Example**::

       GPIO_manager.set_pwm_frequency(pin_num=12, frequency_hz=1000)

- **start_pwm**:
   Starts the PWM signal on the specified pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.

   **Example**::

       GPIO_manager.start_pwm(pin_num=12)

- **stop_pwm**:
   Stops the PWM signal.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.

   **Example**::

       GPIO_manager.stop_pwm(pin_num=12)

- **reset_pin**:
   Resets the given pin.

   **Parameters**:

   - `pin_num` (int): The GPIO pin.

   **Example**::

       GPIO_manager.reset_pin(pin_num=12)

- **cleanup**:
   Cleans up the GPIO pins by setting all output pins to low and clearing all interrupts.

   **Example**::

       GPIO_manager.cleanup()

