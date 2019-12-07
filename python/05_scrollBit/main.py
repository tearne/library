from scrollbit import set_pixel, get_pixel
from scrollbit import show
from random import getrandbits, randint
from microbit import sleep, i2c, button_a, button_b, display

FOOD_BRIGHT = 255
SNAKE_BRIGHT = 40

UP = 0
DOWN = 1
LEFT = 2
RIGHT = 3

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

def turn_random(pt):
    return turn_clock(pt) if bool(getrandbits(1)) else turn_anti(pt)

def hit_edge(pt):
    x = pt[0]
    y = pt[1]
    d = pt[2]

    return (y == TOP_EDGE and d == UP) or \
        (y == BOTTOM_EDGE and d == DOWN) or \
        (x == LEFT_EDGE and d == LEFT) or \
        (x == RIGHT_EDGE and d == RIGHT)

def is_ded(pt):
    x = pt[0]
    y = pt[1]
    return x == -1 or x == 17 or y == -1 or y == 7 or get_pixel(x,y) == SNAKE_BRIGHT

def append_next_move(snakeArr):
    def nextMove(pt):
        if button_a.was_pressed():
            return turn_clock(pt)
        elif button_b.was_pressed():
            return turn_anti(pt)
        elif hit_edge(pt):
            return turn_clock(pt)
        else:
            return go_on(pt)

    prev = snakeArr[-1]
    return snakeArr + [nextMove(prev)]

def new_food(snake):
    def doesnt_intersect_snake(pt):
        for s in snake:
            if s[0:2] == pt: return False
        return True

    while True:
        candidate = (randint(0,16), randint(0,6))
        if doesnt_intersect_snake(candidate):
            break

    return candidate

def eval_food(food, snake):
    snakePosition = snake[-1][0:2]
    if snakePosition == food:
        snake = [snake[0]] + snake
        food = new_food(snake)
        set_pixel(food[0], food[1], FOOD_BRIGHT)

        return (food, snake, True)
    else:
        return (food, snake, False)

def end():
    while True:
        for x in range(10):
            set_pixel(randint(0,16), randint(0,6), randint(0,3)*10)
        show()

def win():
    b = [2,4,8,16,32,64,128,255]
    while True:
        for i in b:
            for x in range(17):
                for y in range(7):
                    set_pixel(x, y, i)
                    show()

def start():
    start = (3,5,UP)
    snake = [start, start]
    set_pixel(start[0], start[1], SNAKE_BRIGHT)
    show()
    display.set_pixel(0,0,9)

    food = new_food(snake)
    set_pixel(food[0], food[1], FOOD_BRIGHT)

    while True:
        snake = append_next_move(snake)
        newPt = snake[-1]
        oldPt = snake[0]
        snake = snake[1:]

        if is_ded(newPt): end()

        food, snake, ate = eval_food(food, snake)

        if ate:
            points = len(snake) - 2
            display.set_pixel(points % 5, int(points / 5), 9)
            if points >= 24: win()
        
        set_pixel(newPt[0], newPt[1], SNAKE_BRIGHT)
        set_pixel(oldPt[0], oldPt[1], 0)
        show()

        sleep(70)

start()
