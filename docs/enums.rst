
Enums
=====

InternPullResistorState
-----------------------
Enum representing the GPIO pin state types for input pins.

- **PULLUP**:
  Pulls the pin up to VCC.

- **PULLDOWN**:
  Pulls the pin down to ground.

- **EXTERNAL**:
  Doesn't use the internal pull resistor.

- **AUTO**:
  Automatically picks the pull resistor based on the pin logic level (Default).

PinState
--------
Enum representing the GPIO pin state types for output pins. The state represents the logic state of the pin.

- **HIGH**:
  Sets the pin to Logic HIGH.

- **LOW**:
  Sets the pin to Logic LOW.

LogicLevel
----------
Enum representing the logic levels of the pins.

- **HIGH**:
  Logic high, when the voltage is close to VCC (Default).

- **LOW**:
  Logic high, when the voltage is close to ground.

TriggerEdge
-----------
Enum representing the trigger edge types for GPIO pins. Triggers are based on logic level changes.

- **RISING**:
  Trigger on the rising edge (from Logic LOW to Logic HIGH).

- **FALLING**:
  Trigger on the falling edge (from Logic HIGH to Logic LOW).

- **BOTH**:
  Trigger on both edges (Default).
