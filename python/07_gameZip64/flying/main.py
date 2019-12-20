from microbit import pin0, pin1, pin2, pin8, pin14, sleep, display
import neopixel, music, utime
from random import randint, randrange, random

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

EMPTY = 0
ASTEROID = 1
POWER = 2


def plot(g):
    for x, column in enumerate(g):
        for y, v in enumerate(column):
            if v == ASTEROID: np_plot(x, y, 5, 1, 1)
            elif v == POWER:  np_plot(x, y, 1, 5, 1)
            else:             np_plot(x, y, 0, 0, 0)
    np.show()

def update_grid(g):
    updated = [ [0] * 8 for i in range(8)]

    for x, column in enumerate(g):
        for y, value in enumerate(column):
            if not value == EMPTY:
                if x > 0: updated[x-1][y] = value 

    updated[7][randrange(8)] = ASTEROID
    updated[7][randrange(8)] = ASTEROID if random() > 0.1 else POWER

    return updated

def update_ship(s):
    if pin14.read_digital() == 0 and s < 7:
        return s + 1
    elif pin8.read_digital() == 0 and s > 0:
        return s - 1
    else: return s

def hit(points):
    pin1.write_digital(1)
    sleep(50)
    pin1.write_digital(0)
    points = points -1
    redraw_points(points)
    return points

def power_up(points):
    for i in range(3):
        music.pitch(300 + i * 100, duration=20, pin=pin2)
    points = points + 1
    redraw_points(points)
    return points

def redraw_points(points):
    for i in range(25):
        display.set_pixel(i % 5, int(i / 5), 9 if points >= i else 0)


prev = utime.ticks_ms()
delay = 500
sub_delay = delay / 3

points = 5
redraw_points(points)

while True:
    new_ship = update_ship(ship)
    if not new_ship == ship:
        ship = new_ship
        music.pitch(100, duration=10, pin=pin2, wait=False)
        sleep(sub_delay)

    if utime.ticks_diff(utime.ticks_ms(), prev) > delay:
        prev = utime.ticks_ms()
        # music.pitch(50, duration=100, pin=pin2, wait=False)
        grid = update_grid(grid)
        plot(grid)
        
        if grid[0][ship] == POWER: 
            points = power_up(points)
        elif grid[0][ship] == ASTEROID: 
            points = hit(points)

    np_plot(0,ship, 5, 5, 10)
    np.show()

