from scrollbit import set_pixel, get_pixel, show
from random import getrandbits, randint
from microbit import sleep, i2c, button_a, button_b, display

from operations import turn_clock, turn_anti, go_on, go_up, go_down, go_left, go_right, turn_random, hit_edge, snake_init
from medal import win

FOOD_BRIGHT = 255
SNAKE_BRIGHT = 20

def is_ded(pt):
    x = pt[0]
    y = pt[1]
    return get_pixel(x,y) == SNAKE_BRIGHT or x == -1 or x == 17 or y == -1 or y == 7

def append_next_move(snk):
    def nextMove(pt):
        if button_a.was_pressed():
            return turn_clock(pt)
        elif button_b.was_pressed():
            return turn_anti(pt)
        elif hit_edge(pt):
            return turn_clock(pt)
        else:
            return go_on(pt)

    prev = snk[-1]
    return snk + [nextMove(prev)]

def new_food(snk):
    def doesnt_intersect_snk(pt):
        for s in snk:
            if s[0:2] == pt: return False
        return True

    while True:
        candidate = (randint(1,15), randint(1,5))
        if doesnt_intersect_snk(candidate):
            break

    return candidate

def eval_food(food, snk):
    snkPosition = snk[-1][0:2]
    if snkPosition == food:
        snk = [snk[0]] + snk
        food = new_food(snk)
        set_pixel(food[0], food[1], FOOD_BRIGHT)

        return (food, snk, True)
    else:
        return (food, snk, False)

def end():
    while True:
        for x in range(10):
            set_pixel(randint(0,16), randint(0,6), randint(0,3)*10)
        show()

def start():
    snk = snake_init()

    display.set_pixel(0,0,9)

    food = new_food(snk)
    set_pixel(food[0], food[1], FOOD_BRIGHT)

    while True:
        snk = append_next_move(snk)
        newPt = snk[-1]
        oldPt = snk[0]
        snk = snk[1:]

        if is_ded(newPt): end()

        food, snk, ate = eval_food(food, snk)

        if ate:
            points = len(snk) - 2
            display.set_pixel(points % 5, int(points / 5), 9)
            if points >= 24: win()

        set_pixel(newPt[0], newPt[1], SNAKE_BRIGHT)
        set_pixel(oldPt[0], oldPt[1], 0)
        show()

        sleep(100)

start()
