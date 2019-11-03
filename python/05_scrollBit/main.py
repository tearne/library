from scrollbit import set_pixel as setpx
from scrollbit import show
from random import getrandbits

UP = 0
DOWN = 1
LEFT = 2
RIGHT = 3

start = (3,5,UP)
snake = [start]
setpx(start[0], start[1], 40)
show()

TOP_EDGE = 0
BOTTOM_EDGE = 6
LEFT_EDGE = 0
RIGHT_EDGE = 16
    

def go_on(pt):
    d = pt[2]
    if d == UP: return go_up(pt)
    elif d == DOWN: return go_down(pt)
    elif d == RIGHT: return go_right(pt)
    else: return go_left(pt) 

def go_up(pt):
    return (pt[0], pt[1] - 1, UP)

def go_down(pt):
    return (pt[0], pt[1] + 1, DOWN)

def go_left(pt):
    return (pt[0] - 1, pt[1], LEFT)

def go_right(pt):
    return (pt[0] + 1, pt[1], RIGHT)

def up_down_random(pt):
    return go_up(pt) if bool(getrandbits(1)) else go_down(pt)

def left_right_random(pt):
    return go_left(pt) if bool(getrandbits(1)) else go_right(pt)


def appendNext(snakeArr):
    # if microbit.button_a.was_pressed():
    #     turn_clock(pt)
    # elif microbit.button_b.is_pressed():
    #     turn_anti(pt)
    # else:
    #     go_on(pt)

    def nextMove(pt):
        x = pt[0]
        y = pt[1]
        d = pt[2]

        if y == TOP_EDGE:
            if x == LEFT_EDGE:
                if d == UP:
                    return go_right(pt)
                else:  # d == LEFT:
                    return go_down(pt)
            elif x == RIGHT_EDGE:
                if d == UP:
                    return go_left(pt)
                else: # d == RIGHT:
                    return go_down(pt)
            else:     # middle of top edge
                return left_right_random(pt)
        elif y == BOTTOM_EDGE:
            if x == LEFT_EDGE:
                if d == DOWN:
                    return go_right(pt)
                else: # d == LEFT
                    return go_up(pt)
            elif x == RIGHT_EDGE:
                if d == DOWN:
                    return go_left(pt)
                else: # d == RIGHT
                    return go_up(pt)
            else:     # middle of bottom edge
                return left_right_random(pt)
        # Now know not on top or bottom edges
        elif x == LEFT_EDGE:
            return up_down_random(pt)
        elif x == RIGHT_EDGE:
            return up_down_random(pt)
        else:
            return go_on(pt)

    prev = snakeArr[-1]
    return snakeArr + [nextMove(prev)]

while True:
    snake = appendNext(snake)
    newPt = snake[-1]
    oldPt = snake[0]
    snake = snake[1:]
    
    setpx(newPt[0], newPt[1], 40)
    setpx(oldPt[0], oldPt[1], 2)
    show()
