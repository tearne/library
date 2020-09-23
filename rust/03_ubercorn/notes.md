Cross compiling
https://disconnected.systems/blog/rust-powered-rover/#setting-up-rust-for-cross-compiling

apt install gcc-arm-linux-gnueabihf libc6-armhf-cross libc6-dev-armhf-cross

rustup target add arm-unknown-linux-gnueabihf





in project root .cargo/config:

[target.arm-unknown-linux-gnueabihf]
linker = "arm-unknown-linux-gnueabihf"

cargo build --target=arm-unknown-linux-gnueabihf

scp target/arm-unknown-linux-gnueabihf/debug/ubercorn pi@piot:test

didn't work, segfault


https://github.com/japaric/rust-cross/issues/42
git clone https://github.com/raspberrypi/tools $HOME/rpi_tools

then in .cargo/config in project

# default target
target = "arm-unknown-linux-gnueabihf"

[target.arm-unknown-linux-gnueabihf]
linker = "/home/user/rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"
