from microbit import *
import neopixel
from math import sqrt, pow
import random

numLEDs = 24

np = neopixel.NeoPixel(pin0, numLEDs)

class Swirl:
    def __init__(self, resolution, shader):
        self.setup(resolution)
        self.shader = shader

    def intShader(self, x):
        rgb = self.shader(x)
        return (int(rgb[0]), int(rgb[1]), int(rgb[2]))

    def setup(self, resolution):
        self.resolution = abs(resolution)
        self.currentTick = random.randint(0, abs(resolution))
        self.focalPoint = numLEDs * self.currentTick / self.resolution
        self.delta = 1 if resolution > 0 else -1
        self.kernelDist = 2

    def tick(self):
        self.currentTick += self.delta
        if self.currentTick >= self.resolution:
            self.currentTick = 0
        elif self.currentTick < 0:
            self.currentTick = self.resolution - 1

        self.focalPoint = numLEDs * self.currentTick / self.resolution

    def longer(self):
        if self.kernelDist < 9:
            self.kernelDist += 1

    def shorter(self):
        if self.kernelDist > 2:
            self.kernelDist -= 1

    def calcIntensity(self, led):
        diff = self.focalPoint - led
        dist = min(
                abs(diff - 24),
                abs(diff),
                abs(diff + 24)
        )

        if dist < self.kernelDist:
            closeness = (self.kernelDist - dist) / self.kernelDist
            intensity = int(
                min(255, 255 * closeness * closeness)
            )
            return self.intShader(intensity)
        else:
            return (0, 0, 0)

swirls = [
    # Swirl(Resolution, RGB colour shader)
    Swirl(24 * 2, lambda x: (x, 0, 0)),
    Swirl(24 * 3, lambda x: (0, x, 0)),
    Swirl(24 * 4, lambda x: (0, 0, x))
]

def mix(swirls, led):
    maxBrightness = 25  # 0 to 255
    result = (0, 0, 0)

    for s in swirls:
        next = s.calcIntensity(led)
        result = (result[0]+next[0], result[1]+next[1], result[2]+next[2])

    maximum = max(result)
    if maximum > maxBrightness:
        factor = maxBrightness / maximum
        result = (
            int(result[0] * factor),
            int(result[1] * factor),
            int(result[2] * factor)
        )

    return result

def displayKernelLength():
    display.show(str(swirls[0].kernelDist), wait=False, clear=True)

while True:
    for led in range(0, numLEDs):
        np[led] = mix(swirls, led)

    np.show()

    for s in swirls:
        s.tick()

    if button_a.was_pressed():
        for s in swirls:
            s.shorter()
        displayKernelLength()

    if button_b.was_pressed():
        for s in swirls:
            s.longer()
        displayKernelLength()

#
