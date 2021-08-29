Cross compiling

? Not sure if this is needed
`apt install gcc-arm-linux-gnueabihf libc6-armhf-cross libc6-dev-armhf-cross`
or
`apt install gcc-arm-linux-gnueabi`

`rustup target add arm-unknown-linux-gnueabihf`
or
`rustup target add arm-unknown-linux-gnueabi`

On OSX, get the rpi tools:
`git clone https://github.com/raspberrypi/tools $HOME/rpi_tools`

Then in .cargo/config in project
```
[target.arm-unknown-linux-gnueabi]
linker = "arm-linux-gnueabi-gcc"
```
or
```
[target.arm-unknown-linux-gnueabihf]
linker = "~/rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"
```

`cargo build --release --target=arm-unknown-linux-gnueabihf`
or
`cargo build --release --target=arm-unknown-linux-gnueabi`

scp target/arm-unknown-linux-gnueabihf/debug/ubercorn pi@piot:ubercorn-binary

## sources
* https://disconnected.systems/blog/rust-powered-rover/#setting-up-rust-for-cross-compiling
* https://github.com/japaric/rust-cross/issues/42



# Installation as a service on Pi Zero
Install cargo-deb

    cargo install cargo-deb

https://gill.net.in/posts/creating-web-server-deb-binary-with-rust/

    cargo deb --target=arm-unknown-linux-gnueabihf