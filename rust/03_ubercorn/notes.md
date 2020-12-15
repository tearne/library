Cross compiling

? Not sure if this is needed
# apt install gcc-arm-linux-gnueabihf libc6-armhf-cross libc6-dev-armhf-cross

rustup target add arm-unknown-linux-gnueabihf

Get the rpi tools:
git clone https://github.com/raspberrypi/tools $HOME/rpi_tools

Then in .cargo/config in project

[target.arm-unknown-linux-gnueabihf]
linker = "/home/user/rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"

cargo build --target=arm-unknown-linux-gnueabihf

scp target/arm-unknown-linux-gnueabihf/debug/ubercorn pi@piot:ubercorn-binary

## sources
* https://disconnected.systems/blog/rust-powered-rover/#setting-up-rust-for-cross-compiling
* https://github.com/japaric/rust-cross/issues/42



# Installation as a service on Pi Zero
Install cargo-deb

    cargo install cargo-deb

https://gill.net.in/posts/creating-web-server-deb-binary-with-rust/

    cargo deb --target=arm-unknown-linux-gnueabihf