from scrollbit import set_pixel as setpx
from scrollbit import show
from random import getrandbits, randint
import microbit
from time import sleep

UP = 0
DOWN = 1
LEFT = 2
RIGHT = 3

start = (3,5,UP)
snake = [start, (3,4,0), (3,3,0), (3,2,0), (3,1,0)]
setpx(start[0], start[1], 40)
show()

TOP_EDGE = 0
BOTTOM_EDGE = 6
LEFT_EDGE = 0
RIGHT_EDGE = 16
    
def turn_clock(pt):
    d = pt[2]
    if d == UP: return go_right(pt)
    elif d == DOWN: return go_left(pt)
    elif d == RIGHT: return go_down(pt)
    else: return go_up(pt) 

def turn_anti(pt):
    d = pt[2]
    if d == UP: return go_left(pt)
    elif d == DOWN: return go_right(pt)
    elif d == RIGHT: return go_up(pt)
    else: return go_down(pt) 

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

def hit_edge(pt):
    x = pt[0]
    y = pt[1]
    d = pt[2]

    return (y == TOP_EDGE and d == UP) or \
        (y == BOTTOM_EDGE and d == DOWN) or \
        (x == LEFT_EDGE and d == LEFT) or \
        (x == RIGHT_EDGE and d == RIGHT)


def appendNext(snakeArr):

    def nextMove(pt):
        if microbit.button_a.was_pressed():
            return turn_clock(pt)
        elif microbit.button_b.was_pressed():
            return turn_anti(pt)
        elif hit_edge(pt):
            return turn_clock(pt)
        else:
            return go_on(pt)

    prev = snakeArr[-1]
    return snakeArr + [nextMove(prev)]

    # def nextMove(pt):
    #     x = pt[0]
    #     y = pt[1]
    #     d = pt[2]

    #     if y == TOP_EDGE:
    #         if x == LEFT_EDGE:
    #             if d == UP:
    #                 return go_right(pt)
    #             else:  # d == LEFT:
    #                 return go_down(pt)
    #         elif x == RIGHT_EDGE:
    #             if d == UP:
    #                 return go_left(pt)
    #             else: # d == RIGHT:
    #                 return go_down(pt)
    #         else:     # middle of top edge
    #             return left_right_random(pt)
    #     elif y == BOTTOM_EDGE:
    #         if x == LEFT_EDGE:
    #             if d == DOWN:
    #                 return go_right(pt)
    #             else: # d == LEFT
    #                 return go_up(pt)
    #         elif x == RIGHT_EDGE:
    #             if d == DOWN:
    #                 return go_left(pt)
    #             else: # d == RIGHT
    #                 return go_up(pt)
    #         else:     # middle of bottom edge
    #             return left_right_random(pt)
    #     # Now know not on top or bottom edges
    #     elif x == LEFT_EDGE:
    #         return up_down_random(pt)
    #     elif x == RIGHT_EDGE:
    #         return up_down_random(pt)
    #     else:
    #         return go_on(pt)
    #    
    # prev = snakeArr[-1]
    # return snakeArr + [nextMove(prev)]

food = (randint(0,16), randint(0,6))
setpx(food[0], food[1], 255)

def doFood(food, snake):
    snakePosition = snake[-1][0:2]
    if snakePosition == food:
        snake = [snake[0]] + snake
        food = (randint(0,16), randint(0,6))
        setpx(food[0], food[1], 255)
        return (food, snake)
    else:
        return (food, snake)


while True:
    snake = appendNext(snake)
    newPt = snake[-1]
    oldPt = snake[0]
    snake = snake[1:]

    food, snake = doFood(food, snake)
    
    setpx(newPt[0], newPt[1], 30)
    setpx(oldPt[0], oldPt[1], 0)
    show()
    sleep(0.1)
