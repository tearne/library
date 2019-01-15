from microbit import *
import neopixel
from random import randrange

np = neopixel.NeoPixel(pin0, 24)

display.show(Image.ALL_CLOCKS, loop=True, delay=100, wait=False)

def reset():
    for i in range(24):
           np[i] = (0,0,0)


while True:
    if button_a.was_pressed() or button_b.was_pressed():
        reset()
    sleep(100)
    np[randrange(0,24)] = (randrange(20), randrange(20), randrange(20))
    np.show()

#
