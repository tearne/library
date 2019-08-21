import bibliopixel
# causes frame timing information to be output
bibliopixel.log.setLogLevel(bibliopixel.log.DEBUG)

# Load driver for the AllPixel
from bibliopixel.drivers.serial import *
# set number of pixels & LED type here
driver = Serial(num = 300, ledtype = LEDTYPE.APA102, c_order = bibliopixel.drivers.ChannelOrder.BGR)

#import the bibliopixel base classes
from bibliopixel import *
from bibliopixel.animation import *
class BasicAnimTest(BaseStripAnim):
    def __init__(self, led):
        super(BasicAnimTest, self).__init__(led)
        #do any initialization here
  
    def step(self, amt = 1):
        for i in range(300):
            self._led.set(i, colors.hue2rgb((i*4+self._step)%256))
            self._led.set_brightness(10)
        self._step += amt*2


#Now try with Strip
led = Strip(driver)

try:
	anim = BasicAnimTest(led)
	anim.run(fps=45)
except KeyboardInterrupt:
	#turn everything off if Ctrl+C is pressed
	led.all_off()
	led.update()
