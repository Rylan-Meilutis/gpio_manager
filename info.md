## Description

- These are rust binding around
  the [RPPAL - Raspberry Pi Peripheral Access Library](https://github.com/golemparts/rppal) crate that gives access to
  gpio, pwm, spi, and more.
  As of current, the only provided bindings are for the gpio library.
  Support for the other functions may come later on.
- These bindings allow you to call the rust code from python in a way that looks like any other python object

## Features

- Able to make multiple objects that can call the gpio without generating errors
- Support for event driven io using callbacks
- Supports software pwm
- Allows for setting up pins as input or output
- Works with multiple platforms

## Usage

- To use you need to first create a GPIOManager object, this can be done by calling
  ```manager = gpio_manager.GPIOManager()```.
- To set up a pin as input with the pull-up resistor enabled on the pin
  ```manager.add_input_pin(BUTTON_PIN, gpio_manager.IPinState.PULLUP)```.
- To set up a pin as output run ```manager.add_output_pin(LED_PIN)```.
- To set an output pin to the low state run ```manager.set_output_pin(LED_PIN, gpio_manager.OPinState.LOW)```.
- To assign a callback to an input pin
  ```manager.assign_callback(BUTTON_PIN, gpio_manager.TriggerEdge.FALLING, button_callback)```.
- To wait on an input pin to trigger an edge ```manager.wait_for_edge(BUTTON_PIN, gpio_manager.TriggerEdge.FALLING)```.
- To setup pwm on an output pin run ```manager.setup_pwm(PWM_PIN, FREQUENCY, PULSE_WIDTH, gpio_manager.LogicLevel.HIGH)```. (Note that if the pin is already setup as an output, The values for the logic level and current state will be preserved.)
- To set the duty cycle of a pwm pin run ```manager.set_pwm_duty_cycle(PWM_PIN, DUTY_CYCLE)```.
- To set the frequency of a pwm pin run ```manager.set_pwm_frequency(PWM_PIN, FREQUENCY)```.
- To start the pwm run ```manager.start_pwm(PWM_PIN)```.
- To stop the pwm run ```manager.stop_pwm(PWM_PIN)```.
- To reset a pin to its default state run ```manager.reset_pin(PIN)```.
- To set all outputs to low and clear all interrupts run ```manager.cleanup()```.

## Warranty

- This library is provided as is and is not guaranteed to work in all cases.

## Coming Soon

- Support for UART
- support for SPI
- Support for hardware PWM
- Support for I2C
- Documentation on readthedocs.io
