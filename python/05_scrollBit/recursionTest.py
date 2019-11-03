from microbit import *

a = 0
fail = False
def foo():
    global a, fail
    a += 1
    if not fail:
        try:
            foo()
        except:
            fail = True

foo()
display.show(a)