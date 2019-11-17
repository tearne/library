## Setup dev env
* `pip3 install uflash` 
  * https://uflash.readthedocs.io
* `pip3 install microfs`
  * https://microfs.readthedocs.io

## Load onto microbit
* Flash a fresh micropython using a blank source file 
  * `uflash`
* Copy on libs 
  * `ufs put somelib.py`
* Copy on a main 
  * `ufs put main`
* REPL
  * `screen /dev/cu.usbmodem14202 115200`

## Moar linkage

* https://microbit-micropython.readthedocs.io/en/latest/tutorials/storage.html
* https://github.com/magopian/microbit-fun/blob/master/main.py
