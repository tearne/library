from microbit import sleep, i2c, Image as I

WIDTH = 17
HEIGHT = 7

orientation = 0
NORMAL = 0
INVERT = 1

_f = 0
_b = bytearray(145)
_i = [getattr(I,x) for x in dir(I) if hasattr(getattr(I,x),'getpx')]

def _w(*args):
    if len(args) == 1: args = args[0]
    i2c.write(116, bytes(args))

def clear():
    global _b
    del _b
    _b = bytearray(145)

def show():
    global _f
    _f = not _f
    _w(253, _f)
    _b[0] = 36
    _w(_b)
    _w(253, 11)
    _w(1, _f)

def set_pixel(col, row, brightness):
    _b[_pixel_addr(col, row)] = brightness

def get_pixel(col, row):
    return _b[_pixel_addr(col, row)]

def _pixel_addr(x, y):
    y =  (7 - (y + 1))*(1 - orientation) + orientation*y
    x = (17 - (x + 1))*orientation + (1 - orientation)*x

    if x > 8:
        x = x - 8
        y = 6 - (y + 8)
    else:
        x = 8 - x

    return (x * 16 + y) + 1

_w(253, 11)
sleep(100)
_w(10, 0)
sleep(100)
_w(10, 1)
sleep(100)
_w(0, 0)
_w(6, 0)
for bank in [1,0]:
    _w(253, bank)
    _w([0] + [255] * 17)
clear()
show()
