Examples
========

Here are example use cases for `GPIOManager`, `PWMManager`, and `I2CManager`.

GPIOManager Examples
--------------------

- **Basic GPIO Input Pin Setup with Callback**::

    import gpio_manager
    import time

    def button_pressed_callback():
        print("Button was pressed on pin 17")

    # Set up GPIO Manager
    gpio = gpio_manager.GPIOManager()

    # Configure pin 17 as input with a pull-up resistor
    gpio.add_input_pin(17, pull_resistor_state=gpio_manager.InternPullResistorState.PULLUP)

    # Assign callback to the pin for falling edge detection
    gpio.assign_callback(17, button_pressed_callback, trigger_edge=gpio_manager.TriggerEdge.FALLING)

    while True:
        time.sleep(1)
        pass

- **GPIO Output Pin Setup and State Change**::

    import gpio_manager
    import time
    # Set up GPIO Manager
    gpio = gpio_manager.GPIOManager()

    # Configure pin 18 as an output pin with an initial LOW state
    gpio.add_output_pin(18, pin_state=gpio_manager.PinState.LOW)

    # Set pin 18 to HIGH
    gpio.set_output_pin(18, gpio_manager.PinState.HIGH)
    time.sleep(2)
    gpio.cleanup()

- **Sounding an Alarm on Pin 4 When an object gets to close to an ultrasonic sensor**::

    import time

    import gpio_manager

    start_time = 0
    end_time = 0
    BUZZER_PIN = 4
    TRIGGER_PIN = 23
    ECHO_PIN = 24

    SPEED_OF_SOUND_MS = 34300

    def milliseconds_to_seconds(miliseconds: float) -> float:
        return miliseconds / 1000


    def activate_warning(distance):
        gpio = gpio_manager.GPIOManager()
        repeat_count = 3

        if 10 < distance < 25:
            duration_time = milliseconds_to_seconds(250)

        elif distance < 10:
            duration_time = milliseconds_to_seconds(150)
        else:
            return
        for _ in range(repeat_count):
            gpio.set_pwm_duty_cycle(BUZZER_PIN, 90)
            time.sleep(duration_time)
            gpio.set_pwm_duty_cycle(BUZZER_PIN, 0)
            time.sleep(duration_time)


    def callback(t_time, edge):
        global start_time
        global end_time
        if edge == gpio_manager.TriggerEdge.RISING:
            start_time = t_time
        else:
            end_time = t_time


    def microseconds_to_seconds(microseconds: float) -> float:
        return microseconds / 1000000


    def setup_gpio() -> gpio_manager.GPIOManager:
        gpio = gpio_manager.GPIOManager()
        gpio.add_output_pin(TRIGGER_PIN)
        gpio.add_input_pin(TRIGGER_PIN)
        gpio.setup_pwm(BUZZER_PIN, 100)
        gpio.start_pwm(BUZZER_PIN)
        return gpio


    def send_trigger_pulse():
        gpio = gpio_manager.GPIOManager()
        gpio.set_output_pin(TRIGGER_PIN, gpio_manager.PinState.HIGH)
        time.sleep(microseconds_to_seconds(10))
        gpio.set_output_pin(TRIGGER_PIN, gpio_manager.PinState.LOW)


    def loop():
        gpio = gpio_manager.GPIOManager()
        gpio.assign_callback(ECHO_PIN, callback, gpio_manager.TriggerEdge.BOTH, include_trigger_time=True,
                             include_trigger_edge=True, debounce_time_ms=0)
        time.sleep(0.5)

        while True:
            send_trigger_pulse()
            time.sleep(0.1)
            duration = end_time - start_time
            if duration == 0:
                print("timeout")
                time.sleep(3)
                continue
            distance = duration * SPEED_OF_SOUND_MS / 2
            activate_warning(distance)
            print(f"Distance: {distance:.2f} cm")
            time.sleep(0.1)



    def main():
        gpio = setup_gpio()

        print("Ultrasonic Measurement")
        print("**************** PROGRAM IS RUNNING **************** ")
        print("Press CTRL-C to end the program.")
        try:
            loop()

        except KeyboardInterrupt:
            print("\nCTRL-C detected.")
        finally:
            gpio.cleanup()

            print("GPIO Port has been cleaned up.")
            print("**************** PROGRAM TERMINATED ****************")
            print()


    if __name__ == "__main__":
        main()



PWMManager Examples
-------------------

- **PWM Setup and Basic LED Brightness Control**::

    import gpio_manager

    # Set up PWM Manager
    pwm = gpio_manager.PWMManager()

    # Set up PWM on channel 0, 1000 Hz frequency, and 50% duty cycle
    pwm.setup_pwm_channel(0, frequency_hz=1000, duty_cycle=50)

    # Start PWM on channel 0
    pwm.start_pwm_channel(0)

    # Change duty cycle to 75%
    pwm.set_duty_cycle(0, 75)

    # Stop PWM on channel 0
    pwm.stop_pwm_channel(0)

- **RGB LED with PWM Control**::

    import gpio_manager
    import time

    # Set up PWM Manager
    pwm = gpio_manager.PWMManager()

    # Set up RGB LED pins with PWM (channels 0 and 1)
    pwm.setup_pwm_channel(0, frequency_hz=1000)
    pwm.setup_pwm_channel(1, frequency_hz=1000)

    # Function to cycle RGB colors
    def cycle_rgb():
        for duty_cycle in range(0, 101, 5):
            pwm.set_duty_cycle(0, duty_cycle)  # Red
            pwm.set_duty_cycle(1, 100 - duty_cycle)  # Green
            time.sleep(0.05)

    # Start PWM on channels 0 and 1
    pwm.start_pwm_channel(0)
    pwm.start_pwm_channel(1)

    # Cycle through colors
    cycle_rgb()

    # Stop PWM on all channels
    pwm.cleanup()


- **Servo motor control on PWM channel 0 (gpio pin 18)**::

    import gpio_manager

    Period_ms = 20
    PWM_CHANNEL = 0
    default_pulse_width = 1.5
    max_rotation = 90


    def setup_gpio():
        pwm = gpio_manager.PWMManager()
        pwm.setup_pwm_channel(PWM_CHANNEL, period_ms=Period_ms, pulse_width_ms=default_pulse_width)
        pwm.start_pwm_channel(PWM_CHANNEL)


    def degrees_to_pulse_with(degrees):
        if degrees < -max_rotation or degrees > max_rotation:
            raise ValueError("Angle must be between -90 and 90 degrees.")

        # Mapping -90 degrees to 0.5 ms and 90 degrees to 2.5 ms
        max_pulse = 2.5  # Pulse width for 90 degrees
        neutral_pulse = 1.5  # Pulse width for 0 degrees

        # Slope of the linear equation
        pulse_width = neutral_pulse + (-degrees / max_rotation) * (max_pulse - neutral_pulse)

        return pulse_width


    def get_user_input():
        while True:
            val = input("Please enter an angle in degrees (-90 - 90) or exit to exit: ")

            if val.lower() == "exit":
                return None

            try:
                int_data = float(val)
                if -max_rotation <= int_data <= max_rotation:
                    return int_data
                else:
                    raise ValueError
            except ValueError:
                print("invalid value, please try again \n")
                continue


    def loop():
        pwm = gpio_manager.PWMManager()
        while True:
            user_input = get_user_input()
            if user_input is None:
                exit()
            pwm.set_pulse_width(PWM_CHANNEL, degrees_to_pulse_with(user_input))


    def main():
        try:
            setup_gpio()
            loop()

        except KeyboardInterrupt:
            print("\nCTRL-C detected.")

        finally:
            pwm = gpio_manager.PWMManager()
            pwm.cleanup()
            print("GPIO Port has been cleaned up.")
            print("**************** PROGRAM TERMINATED ****************")
            print()


    if __name__ == "__main__":
        main()


I2CManager Examples
-------------------

- **Basic I2C Communication**::

    import gpio_manager

    # Set up I2C Manager
    i2c = gpio_manager.I2CManager()

    # Open I2C bus
    i2c.open(bus=1)

    # Write a byte to a slave device at address 0x20
    i2c.write_byte(0x20, 0xFF)

    # Read a byte from the slave device
    data = i2c.read_byte(0x20)
    print("Received byte:", data)

    # Close the I2C bus
    i2c.close()

- **I2C Block Read and Write**::

    import gpio_manager

    # Set up I2C Manager
    i2c = gpio_manager.I2CManager()

    # Open I2C bus
    i2c.open(bus=1)

    # Write a block of bytes with a command
    i2c.block_write(0x20, 0x01, b'\x01\x02\x03')

    # Read a block of data with a command
    data = i2c.block_read(0x20, 0x01, 3)
    print("Received block:", data)

    # Close the I2C bus
    i2c.close()

