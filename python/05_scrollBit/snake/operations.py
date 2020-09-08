UP = 0
DOWN = 1
LEFT = 2
RIGHT = 3

TOP_EDGE = 0
BOTTOM_EDGE = 6
LEFT_EDGE = 0
RIGHT_EDGE = 16

def snake_init():
    start = (3,5,UP)
    return [start, start]

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
