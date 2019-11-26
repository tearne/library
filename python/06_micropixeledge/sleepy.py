from microbit import *
import neopixel
from random import randrange

num_px = 10

np = neopixel.NeoPixel(pin0, num_px)

display.scroll("You are feeling sleepy    veeery sleeeeepy       ", loop=True, delay=100, wait=False)

def reset():
    for i in range(num_px):
           np[i] = (0,0,0)


reset()

i = 0
direction = 1
while True:
    if button_a.was_pressed() or button_b.was_pressed():
        reset()
    sleep(70)
    for j in range(10):
        rgb = (200,200,200) if j == i else (40,20,0)
        np[j] = rgb
    np.show()
    i = i + direction
    i = i % 10 
    if i == 9 or i == 0:
        direction = -direction