## Setup dev env
* `pip3 install uflash` 
  * https://uflash.readthedocs.io
* `pip3 install microfs`
  * https://microfs.readthedocs.io

## Load onto microbit
* Flash a fresh micropython using a blank source file 
  * `uflash`
* Copy on libs 
  * `ufs put scrollbit.py`
* Copy on a main 
  * `ufs put main.py`
* If you need REPL
  * `screen /dev/cu.usbmodem14202 115200` (on mac)

## Moar linkage

* https://microbit-micropython.readthedocs.io/en/latest/tutorials/storage.html
* https://github.com/magopian/microbit-fun/blob/master/main.py
