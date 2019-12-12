from microbit import *
import neopixel
from random import randint
import neopixel

np = neopixel.NeoPixel(pin0, 64)

def np_plot(x, y, r, g, b):
    np[x+(y*8)] = (r, g, b)

while True:
    for x in range(8):
        for y in range(8):
            np_plot(x,y,randint(1,2),randint(1,2),randint(1,2))
    np.show()