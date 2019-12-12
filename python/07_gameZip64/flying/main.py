from microbit import pin0, pin2, pin8, pin14, sleep, display
import neopixel, music, utime
from random import randint, randrange

# Pin0:  8x8 ZIP LED Display
# Pin1:  Vibration Motor for Haptic Feedback
# Pin2:  Piezo Buzzer for Audio Feedback
# Pin8:  Joypad Up Button
# Pin12: Joypad Left Button
# Pin13: Joypad Right Button
# Pin14: Joypad Down Button
# Pin15: Fire 1 Button
# Pin16: Fire 2 Button
# Pin20: Button Expansion Point 1
# Pin19: Button Expansion Point 2


np = neopixel.NeoPixel(pin0, 64)

def np_plot(x, y, r, g, b):
    np[x+(y*8)] = (r, g, b)

grid = [ [0] * 8 for i in range(8)]
ship = 3


def plot(g):
    for x, column in enumerate(g):
        for y, v in enumerate(column):
            if v > 0: np_plot(x, y, 10, 5, 5)
            else: np_plot(x,y,0,0,0)
    np.show()

def update_grid(g):
    newGrid = g[1:]
    newCol = [0] * 8
    newCol[randrange(8)] = 1
    newGrid = newGrid + [newCol]
    return newGrid

def update_ship(s):
    if pin14.read_digital() == 0 and s < 7:
        return s + 1
    elif pin8.read_digital() == 0 and s > 0:
        return s - 1
    else: return s

prev = utime.ticks_ms()
while True:
    new_ship = update_ship(ship)
    if not new_ship == ship:
        ship = new_ship
        music.pitch(100, duration=100, pin=pin2, wait=False)
        sleep(150)

    if utime.ticks_diff(utime.ticks_ms(), prev) > 500:
        prev = utime.ticks_ms()
        # music.pitch(50, duration=100, pin=pin2, wait=False)
        grid = update_grid(grid)
        plot(grid)

    np_plot(0,ship, 5, 10, 5)
    np.show()