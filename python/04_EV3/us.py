#!/usr/bin/env python3
from ev3dev2.sensor.lego import TouchSensor, UltrasonicSensor
from ev3dev2.led import Leds
from time import sleep
from ev3dev2.motor import MediumMotor

# Connect ultrasonic and touch sensors to any sensor port
us = UltrasonicSensor()
motor = MediumMotor()

def main():
    leds = Leds()

    leds.all_off() # stop the LEDs flashing (as well as turn them off)
    while True:
        dist = us.distance_centimeters
        if  dist < 20: # to detect objects closer than 40cm
            # In the above line you can also use inches: us.distance_inches < 16
            leds.set_color('LEFT',  'RED')
            leds.set_color('RIGHT', 'RED')
        else:
            leds.set_color('LEFT',  'GREEN')
            leds.set_color('RIGHT', 'GREEN')

        print(dist)

        motor.on_to_position(50, int(dist))

        sleep (0.1) # Give the CPU a rest

try:
    main()
except:
    us.distance_centimeters_ping