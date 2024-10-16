## Description

- These are rust binding around
  the [RPPAL - Raspberry Pi Peripheral Access Library](https://github.com/golemparts/rppal) crate that gives access to
  gpio, pwm, spi, and more.
  As of current, the gpio and pwm portions are tested and working, i2c will be tested soon as well.
  Support for the other functions may come later on.
- These bindings allow you to call the rust code from python in a way that looks like any other python object

## Features

- Able to make multiple objects that can call the gpio without generating errors
- Support for event driven io using callbacks
- Supports software pwm
- Allows for setting up pins as input or output
- Works with multiple platforms

## Usage
### GPIO

- To use, you first need to create a `GPIOManager` object, which can be done by calling:

  ```python
  GPIO_manager = gpio_manager.GPIOManager()
  ```

- To set up a pin as input with the pull-up resistor enabled on the pin:

  ```python
  GPIO_manager.add_input_pin(BUTTON_PIN, gpio_manager.InternPullResistorState.AUTO, gpio_manager.LogicLevel.HIGH)
  ```

- To set up a pin as an output pin, run:

  ```python
  GPIO_manager.add_output_pin(LED_PIN)
  ```

- To set an output pin to the low state, run:

  ```python
  GPIO_manager.set_output_pin(LED_PIN, gpio_manager.OPinState.LOW)
  ```

- To assign a callback to an input pin for a falling edge trigger:

  ```python
  GPIO_manager.assign_callback(BUTTON_PIN, button_callback, gpio_manager.TriggerEdge.FALLING)
  ```

- To wait for an edge on an input pin:

  ```python
  GPIO_manager.wait_for_edge(BUTTON_PIN, gpio_manager.TriggerEdge.FALLING)
  ```

- To set up PWM on an output pin:

  ```python
  GPIO_manager.setup_pwm(PWM_PIN, FREQUENCY, PULSE_WIDTH, gpio_manager.LogicLevel.HIGH)
  ```

  **Note**: If the pin is already set up as an output, the values for the logic level and current state will be preserved.


- To set the duty cycle of a PWM pin:

  ```python
  GPIO_manager.set_pwm_duty_cycle(PWM_PIN, DUTY_CYCLE)
  ```

- To set the frequency of a PWM pin:

  ```python
  GPIO_manager.set_pwm_frequency(PWM_PIN, FREQUENCY)
  ```

- To start the PWM:

  ```python
  GPIO_manager.start_pwm(PWM_PIN)
  ```

- To stop the PWM:

  ```python
  GPIO_manager.stop_pwm(PWM_PIN)
  ```

- To reset a pin to its default state:

  ```python
  GPIO_manager.reset_pin(PIN)
  ```

- To set all output pins to low and clear all interrupts:

  ```python
  GPIO_manager.cleanup()
  ```
---

### PWM

- To use PWM functionality, you need to create a `PWMManager` object:

  ```python
  pwm_manager = gpio_manager.PWMManager()
  ```

- To set up a PWM channel:

  ```python
  pwm_manager.setup_pwm_channel(CHANNEL_NUM, frequency_hz=60.0, duty_cycle=0, logic_level=gpio_manager.LogicLevel.NORMAL)
  ```

  - `CHANNEL_NUM`: The PWM channel number (either 0 or 1).
  - `frequency_hz`: The frequency of the PWM signal in Hertz.
  - `duty_cycle`: The duty cycle of the PWM signal as a percentage (from 0 to 100).
  - `logic_level`: The logic level of the PWM signal (can be set to `LogicLevel.HIGH` or `LogicLevel.LOW`).

- To start the PWM signal on a specified channel:

  ```python
  pwm_manager.start_pwm_channel(CHANNEL_NUM)
  ```

- To stop the PWM signal on a specified channel:

  ```python
  pwm_manager.stop_pwm_channel(CHANNEL_NUM)
  ```

- To reset a PWM channel:

  ```python
  pwm_manager.reset_pwm_channel(CHANNEL_NUM)
  ```

- To set the duty cycle for a PWM channel:

  ```python
  pwm_manager.set_duty_cycle(CHANNEL_NUM, duty_cycle=75)
  ```

  - `CHANNEL_NUM`: The PWM channel number (either 0 or 1).
  - `duty_cycle`: The new duty cycle (from 0 to 100).

- To set the frequency for a PWM channel:

  ```python
  pwm_manager.set_frequency(CHANNEL_NUM, frequency_hz=60.0)
  ```

  - `CHANNEL_NUM`: The PWM channel number (either 0 or 1).
  - `frequency_hz`: The frequency of the PWM signal in Hertz.

- To get the current frequency of a PWM channel:

  ```python
  frequency = pwm_manager.get_frequency(CHANNEL_NUM)
  ```

- To get the current duty cycle of a PWM channel:

  ```python
  duty_cycle = pwm_manager.get_duty_cycle(CHANNEL_NUM)
  ```

- To cleanup the pwm channels:
  ```python
  duty_cycle = pwm_manager.cleanup()
  ```
---

### I2C

- To use I2C functionality, you first need to create an `I2CManager` object:

  ```python
  i2c_manager = gpio_manager.I2CManager()
  ```

- To open the I2C bus:

  ```python
   i2c_manager.open(bus=1)
   ```
  - `bus`: The I2C bus number to open (default is 1).

- To close the I2C bus:

  ```python
   i2c_manager.close()
   ```
- To write a single byte to an I2C slave device:

  ```python
   i2c_manager.write_byte(0x20, 0x01, 0xFF)
   ```
  - `0x20`: The I2C slave address.
  - `0x01`: The command to send to the slave device.
  - `0xFF`: The byte to write.

- To read a single byte from an I2C slave device:

  ```python
   data = i2c_manager.read_byte(0x20, 0x01)
   ```
  - `0x20`: The I2C slave address.
  - `0x01`: The command to send to the slave device before reading.
  - `data`: The byte read.

- To write data to an I2C slave device:

  ```python
   i2c_manager.write(0x20, 0x01, b'\x01\x02\x03')
   ```
  - `0x20`: The I2C slave address.
  - `0x01`: The command to send to the slave device.
  - `b'\x01\x02\x03'`: The bytes to write.

- To read data from an I2C slave device:

  ```python
   data = i2c_manager.read(0x20, 0x01, 3)
  ```
  - `0x20`: The I2C slave address.
  - `0x01`: The command to send to the slave device before reading.
  - `3`: The number of bytes to read.
  - `data`: The bytes read.

- To perform a write followed by a read operation:

   ```python
   data = i2c_manager.write_read(0x20, 0x01, b'\x01\x02', 3)
   ```
  - `0x20`: The I2C slave address.
  - `0x01`: The command to send to the slave device.
  - `b'\x01\x02'`: The bytes to write.
  - `3`: The number of bytes to read.
  - `data`: The bytes read.
---

## Warranty

- This library is provided as is and is not guaranteed to work in all cases.

## Coming Soon

- Support for UART
- support for SPI
- Documentation on readthedocs.org
