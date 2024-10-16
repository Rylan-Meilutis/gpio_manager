class PWMPolarity:
    """Enum representing the PWM polarity options."""
    NORMAL: 'PWMPolarity'
    """
    Normal polarity (default).
    """
    INVERSE: 'PWMPolarity'
    """
    Inverse polarity.
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
    Automatically picks the pull resistor based on the pin logic level.
    """

class OPinState:
    """Enum representing the GPIO pin state types for output pins. The state represents the logic state of the pin. The voltage will be set based on the logic level."""
    HIGH: 'OPinState'
    """
    Sets the pin to Logic HIGH.
    """
    LOW: 'OPinState'
    """
    Sets the pin to Logic LOW.
    """

class LogicLevel:
    """Enum representing the logic levels of the pins."""
    HIGH: 'LogicLevel'
    LOW: 'LogicLevel'

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
    Trigger on both edges.
    """