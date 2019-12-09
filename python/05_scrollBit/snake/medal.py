from scrollbit import set_pixel
from scrollbit import show
from microbit import sleep

def win():
    icon = [
        (0,3),
        (1,2), (1,4),
        (2,1), (2,5),
        (3,0), (3,1), (3,2), (3,6),
        (4,0), (4,1), (4,2), (4,6),
        (5,1), (5,5),
        (6,2), (6,4),
        (7,3)
    ]

    def draw(x, y, b): set_pixel((x + shift) % 17, y, b)

    shift = 0

    while True:
        for x,y in icon: draw(x,y, 0)
        shift = 0 if shift == 16 else shift + 1
        for x,y in icon: draw(x,y, 200)
        show()
        sleep(100)