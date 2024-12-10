"""
gpio_manager

A comprehensive library for managing GPIO operations, including input/output control,
PWM management, I2C communication, and support for edge-triggered callbacks with debounce.
Suitable for all Raspberry Pi models.

Classes:

- GPIOManager: Manages GPIO pins, including input and output configurations, and supports callback assignments.

- PWMManager: Controls Pulse Width Modulation (PWM) functionality for GPIO pins.

- I2CManager: Provides I2C communication functions for interacting with I2C devices.

- Enums: Defines enums such as PinState, LogicLevel, InternPullResistorState, and TriggerEdge for easy configuration
of pin states and edge triggers.

Example usage:
    from gpio_manager import GPIOManager
    gpio = GPIOManager()
    gpio.add_input_pin(pin_num=4)
    gpio.assign_callback(pin_num=4, callback=my_callback)
"""


class InternPullResistorState:
    """Enum representing the GPIO pin state types for input pins."""
    PULLUP: 'InternPullResistorState'
    """
    Pulls the pin up to VCC.
    """
    PULLDOWN: 'InternPullResistorState'
    """
    Pulls the pin down to ground.
    """
    EXTERNAL: 'InternPullResistorState'
    """
    Don't use the internal pull resistor.
    """
    AUTO: 'InternPullResistorState'
    """
    Automatically picks the pull resistor based on the pin logic level (Default).
    """


class PinState:
    """Enum representing the GPIO pin state types for output pins. The state represents the logic state of the pin. The voltage will be set based on the logic level."""
    HIGH: 'PinState'
    """
    Pin logic level high.
    """
    LOW: 'PinState'
    """
    Pin logic level low.
    """


class LogicLevel:
    """Enum representing the logic levels of the pins."""
    HIGH: 'LogicLevel'
    """
    Logic high, when the voltage is close to VCC (Default).
    """
    LOW: 'LogicLevel'
    """
    Logic high, when the voltage is close to ground.
    """


class TriggerEdge:
    """Enum representing the trigger edge types. Triggers are based off logic level changes"""
    RISING: 'TriggerEdge'
    """
    Trigger on the rising edge. (from Logic LOW to Logic HIGH)
    """
    FALLING: 'TriggerEdge'
    """
    Trigger on the falling edge. (from Logic HIGH to Logic LOW)
    """
    BOTH: 'TriggerEdge'
    """
    Trigger on both edges (Default).
    """
