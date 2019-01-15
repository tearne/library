from microbit import *
import neopixel
from math import sqrt, pow, ceil
from random import randint

numLEDs = 24
np = neopixel.NeoPixel(pin0, numLEDs)

SEC = 1000
MIN = 60 * SEC
HOUR = 60 * MIN

start = running_time()

class FadeSpinner:
    def __init__(self, brightness, msPerRevolution, offset):
        self.brightness = brightness
        self.period = msPerRevolution
        self.offset = offset
        self.percentStart = self.offset / self.period

    def colourAt(self, timeElapsed, position):
        percentElapsed = (timeElapsed % self.period) / self.period
        percent = (1 - self.percentStart + percentElapsed) % 1
        ledIdxFloat = percent * 24
        fractionBetween = ledIdxFloat - int(ledIdxFloat)
        ledIdx = int(ledIdxFloat)

        if(position == ledIdx):
            return int(self.brightness * (1 - fractionBetween))
        elif(position == ledIdx+1 or (ledIdx == 23 and position == 0)):
            return int(self.brightness * fractionBetween)
        else:
            return 0

    def timesUp(self, timeElapsed):
        return self.offset != 0 and timeElapsed >= self.offset

    def reset(self):
        self.timing = True

minutes = FadeSpinner(20, HOUR, 5 * MIN)
seconds = FadeSpinner(100, MIN, 0)

def report():
    msg = str(int(minutes.offset / MIN))
    if(len(msg) == 1):
        display.show(msg, wait=False, clear=True)
    else:
        display.scroll(msg, wait=False)

def restart(deltaMins):
    global start
    global minutes
    start = running_time()
    minutes = FadeSpinner(20, HOUR, minutes.offset + deltaMins * MIN)
    sleep(100)


def doAlarm():
    goingUp = True
    display.scroll("Alarm!", wait=False)
    while True:
        for i in range(100):
            if button_a.was_pressed() or button_b.was_pressed():
                return
            sleep(15)
            for j in range(24):
                brightness = i if goingUp else (100 - i)
                np[j] = (brightness, brightness, brightness)
            np.show()
        goingUp = not goingUp


def doTiming():
    while True:
        if button_a.is_pressed() and minutes.offset > 1:
            restart(-1)
            report()
        if button_b.is_pressed() and minutes.offset < HOUR:
            restart(1)
            report()

        elapsed = running_time() - start
        ledSubSec = int(24 * (elapsed % SEC) / SEC)

        for i in range(24):
            np[i] = (
                20 if i == ledSubSec else 0,
                minutes.colourAt(elapsed, i),
                seconds.colourAt(elapsed, i)
            )
        np.show()

        if minutes.timesUp(elapsed):
            return


while True:
    doTiming()
    doAlarm()

#
