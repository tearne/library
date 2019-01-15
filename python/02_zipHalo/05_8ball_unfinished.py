from microbit import *
import neopixel
from math import sqrt, pow, ceil
import random

answers = [
    "It is certain",
    "It is decidedly so",
    "Without a doubt",
    "Yes, definitely",
    "You may rely on it",
    "As I see it, yes",
    "Most likely",
    "Outlook good",
    "Yes",
    "Signs point to yes",
    "Reply hazy try again",
    "Ask again later",
    "Better not tell you now",
    "Cannot predict now",
    "Concentrate and ask again",
    "Don't count on it",
    "My reply is no",
    "My sources say no",
    "Outlook not so good",
    "Very doubtful",
]


display.show('8')

numLEDs = 24
np = neopixel.NeoPixel(pin0, numLEDs)
while True:
    readingx = accelerometer.get_x()
    readingy = accelerometer.get_y()
    readingz = accelerometer.get_z()
    
    if accelerometer.was_gesture('shake'):
        display.clear()
        for i in range(24):
            np[i] = (0,0,0)
        np.show()
        sleep(2000)
        display.scroll(random.choice(answers))

    for i in range(24):
        np[i] = (
            min(abs(readingx//10),255),
            min(abs(readingy//10), 255), 
            min(abs(readingz//10),255)
            )
        np.show()

#
