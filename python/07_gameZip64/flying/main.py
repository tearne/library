from microbit import pin0, pin1, pin2, pin8, pin14, pin15, pin16, sleep, display
import neopixel, music, utime
import constants
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

def np_plot(x, y, rgb):
    np[x+(y*8)] = (rgb[0], rgb[1], rgb[2])

grid = [ [0] * 8 for i in range(8)]
ship = 3

def plot(g):
    for x, column in enumerate(g):
        for y, v in enumerate(column):
            if v == constants.ASTEROID:  np_plot(x, y, constants.ASTEROID_COLOUR)
            elif v == constants.POWER:   np_plot(x, y, constants.POWER_COLOUR)
            elif v == constants.MISSILE: np_plot(x, y, constants.MISSILE_COLOUR)
            else:                        np_plot(x, y, constants.BLACK)

def update_grid(g):
    updated = [ [0] * 8 for i in range(8)]

    for x, column in enumerate(g):
        for y, value in enumerate(column):
            if not value == constants.EMPTY:
                if x > 0: updated[x-1][y] = value 

    # updated[7][randrange(8)] = ASTEROID
    updated[7][randrange(8)] = constants.ASTEROID if random() > 0.3 else constants.POWER
    if random() < 0.1:
        updated[7][randrange(8)] = constants.MISSILE

    return updated


def update_ship(s):
    if pin14.read_digital() == 0 and s < 7:
        return s + 1
    elif pin8.read_digital() == 0 and s > 0:
        return s - 1
    else: return s

def hit(points):
    pin1.write_digital(1)
    sleep(10)
    pin1.write_digital(0)
    points = max(points - 1, 0)
    redraw_points(points)
    return points

def power_up(points):
    points = min(25, points + 1)
    for i in range(3):
        music.pitch(200 + i * 10 + 10 * points, duration=20, pin=pin2)
    redraw_points(points)
    return points

def redraw_points(points):
    for i in range(25):
        display.set_pixel(i % 5, int(i / 5), 9 if points > i else 0)


prev = utime.ticks_ms()

points = 2
redraw_points(points)

def get_delay(points):
    return 600 - (points - 5) * 15

got_missile = False


def plot_ship(ship, missile):
    colour = constants.MISSILE_COLOUR if missile.got_missile() else constants.SHIP_COLOUR
    np_plot(0,ship, colour)

class Missile:
    NONE = 0
    HOLDING = 1
    AWAY = 2

    def __init__(self):
        self._status = Missile.NONE

    def acquire(self):
        self._status = Missile.HOLDING
        music.pitch(200, duration=50, pin=pin2)
        music.pitch(400, duration=300, wait=False, pin=pin2)

    def got_missile(self):
        return self._status == Missile.HOLDING

    def fire(self, y_pos):
        self._status = Missile.AWAY
        self._x = 0
        self._y = y_pos

    def is_away(self):
        return self._status == Missile.AWAY

    def pos(self):
        return (self._x, self._y)

    def step_and_plot(self, grid):
        newGrid = grid

        if self._x > 7:
            self._status = Missile.NONE
        elif grid[self._x][self._y] == constants.ASTEROID:
            sleep(50)
            np_plot(self._x,self._y, constants.FIRE_COLOUR)
            newGrid = [row[:] for row in grid]
            newGrid[self._x][self._y] = constants.EMPTY
        else:
            np_plot(self._x, self._y, constants.MISSILE_COLOUR)
            sleep(50)
        
        self._x = self._x + 1

        return newGrid


missile = Missile()

while True:
    delay = get_delay(points)

    if utime.ticks_diff(utime.ticks_ms(), prev) > delay:
        prev = utime.ticks_ms()
        grid = update_grid(grid)
        plot(grid)
        
        ship_pos = grid[0][ship]
        if ship_pos == constants.POWER: 
            points = power_up(points)
        if ship_pos == constants.MISSILE:
            missile.acquire()
            music.pitch(200, duration=50, pin=pin2)
            music.pitch(400, duration=300, wait=False, pin=pin2)
        elif grid[0][ship] == constants.ASTEROID: 
            points = hit(points)

    new_ship = update_ship(ship)

    if missile.got_missile() and pin15.read_digital() == 0:
        missile.fire(ship)

    if missile.is_away():
        grid = missile.step_and_plot(grid)

    if not new_ship == ship:
        ship = new_ship
        sleep(100)

    plot_ship(ship, missile)

    np.show()
