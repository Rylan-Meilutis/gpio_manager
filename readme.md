# Python GPIO Library

## Installation

``` bash
pip install --upgrade gpio-manager
``` 
or if you have installed uv
``` bash
uv pip install --upgrade gpio-manager
```

## Usage and Documentation
- for examples and documentation, see the [documentation](https://gpio-manager.readthedocs.io/en/latest/index.html)


## Description

- These are rust bindings around
  the [RPPAL - Raspberry Pi Peripheral Access Library](https://github.com/golemparts/rppal) crate that gives access to
  gpio, pwm, spi, and more.
  As of current, the gpio, i2c, and pwm portions are tested and working.
  Support for the other functions may come later on.
- These bindings allow you to call the rust code from python in a way that looks like any other python object

## Features

- Able to make multiple objects that can call the gpio without generating errors
- Support for event driven io using callbacks
- Supports software pwm
- Allows for setting up pins as input or output
- Supports hardware PWM
- Supports I2C
- Works with multiple pi versions and multiple OS's

## Warranty

- This library is provided as is and is not guaranteed to work in all cases.

## Support

- If you have any issues with the library please reach out in the discussions tab, or open an issue if you find a bug.

## Coming Soon

- Support for UART
- support for SPI