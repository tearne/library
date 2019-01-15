from microbit import *
import neopixel

np = neopixel.NeoPixel(pin0, 24)

while True:
    needle = ((15 - compass.heading()) // 30) % 12
    display.show(Image.ALL_CLOCKS[needle])
    
    led = (-compass.heading() // 15) % 24
    
    for i in range(24):
        np[i] = (0,0,0)
    
    np[led] = (20, 20, 20)
    np.show()    
    
