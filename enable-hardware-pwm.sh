#!/bin/bash

# Define the lines to be added
line="#Enable both pwm outputs\ndtoverlay=pwm-2chan"

# Backup the original config.txt file
sudo cp /boot/firmware/config.txt /boot/firmware/config.txt.bak

# Insert the line above the [cm4] section
sudo awk -v add="$line" '/^\[cm4\]/ && !x {print add; x=1} 1' /boot/firmware/config.txt > temp_config.txt && sudo mv temp_config.txt /boot/firmware/config.txt

echo "please reboot to enable hardware pwm :)"
