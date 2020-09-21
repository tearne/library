use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use rgb;
use std::io::prelude::*;

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        eprintln!("Doing some final cleanup");
    }
}

pub fn main() {
    let _cleanup = Cleanup;

    const BLACK: rgb::RGB8 = rgb::RGB8 { r: 0, g: 0, b: 0 };

    let mut leds: [rgb::RGB8; 256] =  [BLACK; 256];

    let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(9_000_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options);

    let y = 3;
    let x = 7;
    leds[(y * 16) + x] = rgb::RGB8::new(0, 255, 0);
    
    plot_stuff(&mut spi, &leds);
}

fn plot_stuff(spi: &mut Spidev, leds: &[rgb::RGB8]){
    spi.write(&[0x72]);
    let data = as_array(&leds);
    spi.write(&data);
}

fn as_array(leds: &[rgb::RGB8]) -> Vec<u8> {
    let mut arr: Vec<u8> = vec![];
    use rgb::ComponentSlice;
    arr.extend_from_slice(leds.as_slice());
    arr
}
