#!/usr/bin/env pybricks-micropython

from pybricks import ev3brick as brick
from pybricks.ev3devices import (Motor, TouchSensor, ColorSensor,
                                 InfraredSensor, UltrasonicSensor, GyroSensor)
from pybricks.parameters import (Port, Stop, Direction, Button, Color,
                                 SoundFile, ImageFile, Align)
from pybricks.tools import print, wait, StopWatch
from pybricks.robotics import DriveBase

left_motor = Motor(Port.D)
right_motor = Motor(Port.B)
ir = InfraredSensor(Port.S4)

balls = 3

wheel_diameter = 30
axel_track = 125

tracks = DriveBase(left_motor, right_motor, wheel_diameter, axel_track)

touch_sensor = TouchSensor(Port.S1)
colour = ColorSensor(Port.S3)
# us = UltrasonicSensor(Port.S2)
shooter = Motor(Port.C)

#
# Start
#
brick.sound.beep(2000)
brick.sound.beep(1000)
brick.sound.beep(5000)

useful_buttons = [
    Button.LEFT_UP,
    Button.LEFT_DOWN,
    Button.RIGHT_UP,
    Button.RIGHT_DOWN,
]

while not touch_sensor.pressed():
    (dist, angle) = ir.beacon(1)
    buttons = ir.buttons(1)
    print(dist, angle, buttons)
    # if angle is not None and dist > 15:
    #     tracks.drive(2 * dist, 2 * angle)
    if useful_buttons in buttons:
        if Button.LEFT_UP in buttons and Button.RIGHT_UP in buttons:
            print("forward")
            tracks.drive(200,0)
        elif Button.LEFT_UP in buttons:
            print("left")
            tracks.drive(200,-30)
        elif Button.RIGHT_UP in buttons:
            print("right")
            tracks.drive(200,30)
        elif Button.LEFT_DOWN in buttons and Button.RIGHT_DOWN in buttons:
            print("shoot")
            brick.sound.beep(200)
            brick.sound.beep(100)
            brick.sound.beep(200)
            brick.sound.beep(100)
            brick.sound.beep(200)
            shooter.run_time(-3600, 1500)
    else:
        tracks.stop()
    # if us.distance() < 500 and balls > 0:
    #     brick.sound.beep(200)
    #     brick.sound.beep(100)
    #     brick.sound.beep(200)
    #     brick.sound.beep(100)
    #     brick.sound.beep(200)
    #     shooter.run_time(-3600, 1500)
    #     balls = balls - 1

brick.sound.beep(2000)
brick.sound.beep(5000)
brick.sound.beep(1000)

tracks.stop()

