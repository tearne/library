# Introduction

The two main parts are the **scroll:bit** (matrix of white LEDs) and the **micro:bit** (a microcontroller).  Plug them together and stick on the battery pack so it looks like this:

![assembled.png](assembled.png)

Note the micro:bit needs to be pushed (quite hard) all the way into the edge connector so none of the "teeth" are showing.  The power switch is sticking out the side on the battery box and represents the peak of design sophistication.

# Installation & Setup (Windows)

## Python 3

* [Download](https://www.python.org/downloads/windows/) the current v3.x installer: 
* Run the installer and tick "Add Python 3.x to PATH"
* Click *Install Now* to do a default installation which includes the `pip` program needed later.

## Visual Studio Code
This is the source code editor and will provide a terminal for running commands.

* Download and install from https://code.visualstudio.com/download
* Once installed open the VS Code application and from the menu bar click *Terminal* -> *New Terminal*.
* In the terminal type `python --version`, it should say Python 3.8, or similar.  

Note that at various points VS Code will offer to install plug-ins which it thinks will be helpful, based on what it can see you doing.  You don't have to install them, and sometimes they will be stupid, but they shouldn't  do any harm.  They do things like colour the code nicely or show syntax errors.

## Python tools for micro:bit
* In the terminal type `pip install uflash microfs` to install two important tools: [uflash](https://pypi.org/project/uflash/) & [microfs](https://pypi.org/project/microfs/).  You may get some warnings about pip not being the latest version - shouldn't matter.
* Plug the micro:bit into the computer using the micro USB port.  It will spring into life and also appear as a tiny flash drive, but we can ignore that (you don't need to eject it or power it down - you just yank it out, even when it's running a program)
* Type `ufs ls` in the termianl.  The command should connect to the micro:bit, connect to it, and then list the files currenty loaded into its noggin, which are *<span>main.</span>py* & *<span>scrollbit.</span>py*.

Congratulations.  You have installed stuff.  Celebrate with cake and a chocolatey beverage.

# Prepare the project

## Scource code folder
* Copy this folder from the USB stick to somewhere on your computer.
* In VSCode select *Menu* -> *Add Folder to Workspace...*, highlight this folder and press *Add*.  The folder will be added to file explorer on the left. It's possible to add lots of folder this way.
* Open `readMe.md` file to continue reading this text but now with glorious markdown syntax highlighting.  Once opened, you can also press an icon in the top right to open a preview window displaying the rendered view. 

## Setup terminal
* Now the project folder is set up, if you open a terminal (*Terminal* -> *New Terminal*) it should automagically set the terminal to be "in" the project folder. 
* Type `ls` and it should show you the files in the project folder.

Contratulations, you have set up your development environment.  Celebrate by cancelling a high profile project at work.

# Code!

At this point you may wish to go to the menu bar and click *View* -> *Appearance* and select *Full Screen* or *Zen Mode*.  This will help you pretend you're [l33t](https://en.wikipedia.org/wiki/Leet), or a hacker in a film.

## Modify Code
* In VS Code, open and comment out the line near the end of `main.py` which says `sleep(100)` - do this by putting the cursor on the line, then pressing `Ctrl /` or just typing a `#` at the start of the line.
* Save the file with `Ctrl s`.
* Go to the terminal and upload the modified file to the plugged in micro:bit with the command `ufs put main.py`.
* The yellow LED on the back of the micro:bit will flash during the transfer.  Once completed it might reboot on it's own, or you may need to press the reset button on the back to enjoy the  silly fast game.

If at any point there is an error in the code, the micro:bit will scroll a message on it's small red LED display, hopefully telling you which line it thinks the problem relates to.


## Break it and fix it
* In the terminal, wipe its memory with the command `uflash`.  This will connect to the micro:bit, erase everything on it, and load a fresh version of the MicroPython interpreter.  It's like installing an operating system without any pre-installed programs.
* Load the library which makes it easier to program the while LED matrix onto the micro:bit using the command `ufs put scrollbit.py`.  This file is a cut down version of the [official](https://github.com/pimoroni/micropython-scrollbit/blob/master/library/scrollbit.source.py) version, to save space so we can fit more of our own code on.  Official docs for the full library are [here](https://github.com/pimoroni/micropython-scrollbit).
* Load our program with `ufs put main.py`.  Note that the main program must always be called `main.py` in order to be detected and run.
* If the micro:bit hasn't automatically started running, reach behind its face to press reset.

# Onward

* Official [tutorials & API](https://microbit-micropython.readthedocs.io/en/latest/tutorials/hello.html) docs including the different commands you can use on the micro:bit.
* Go [shopping](https://coolcomponents.co.uk/collections/micro-bit)
* Discover [awesome](https://github.com/carlosperate/awesome-microbit)
* Build a bioinfomatics pipeline.
* Host a business critical application on it.

