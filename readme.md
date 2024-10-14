# Python GPIO Library

## Installation
``` bash
pip install --break-system-packages --upgrade gpio-manager
``` 


## Usage
- To use you need to first create a GPIOManager object, this can be done by calling ```manager = gpio_manager.GPIOManager()```.
- To set up a pin as input with the pull-up resistor enabled on the pin ```manager.add_input_pin(BUTTON_PIN, gpio_manager.IPinState.PULLUP)```.
- To set up a pin as output run ```manager.add_output_pin(LED_PIN)```.
- To set an output pin to the low state run ```manager.set_output_pin(LED_PIN, gpio_manager.OPinState.LOW)```.
- To assign a callback to an input pin ```manager.assign_callback(BUTTON_PIN, gpio_manager.TriggerEdge.FALLING, button_callback)```.
- To wait on an input pin to trigger an edge ```manager.wait_for_edge(BUTTON_PIN, gpio_manager.TriggerEdge.FALLING)```.
- To set pwm on an output pin run ```manager.set_pwm(PWM_PIN, PERIOD_MS, PULSE_WIDTH_US)```.
- To set all outputs to low and clear all interrupts run ```manager.cleanup()```.

## Description
- These are rust binding around the [RPPAL - Raspberry Pi Peripheral Access Library](https://github.com/golemparts/rppal) crate that gives access to gpio, pwm, spi, and more. 
As of current, the only provided bindings are for the gpio library. 
Support for the other functions may come later on.
- These bindings allow you to call the rust code from python in a way that looks like any other python object

## Features
- Able to make multiple objects that can call the gpio without generating errors
- Support for event driven io using callbacks

## Warranty
- This library is provided as is and is not guaranteed to work in all cases.

## Support
- If you have any issues with the library please contact me in class or via the discussion board on brightspace.