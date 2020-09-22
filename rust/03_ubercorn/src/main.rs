use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use rgb;
use std::io::prelude::*;
use std::{thread, time};

const BLACK: rgb::RGB8 = rgb::RGB8 { r: 0, g: 0, b: 0 };

struct Display {
    spi: Spidev,
}

impl Display {
    pub fn build() -> Self {
        let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(9_000_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options);

        Display{spi}
    }

    pub fn apply(&mut self, leds: &[rgb::RGB8]){
        self.spi.write(&[0x72]);
        let data = Self::as_array(&leds);
        self.spi.write(&data);
    }

    fn as_array(leds: &[rgb::RGB8]) -> Vec<u8> {
        let mut arr: Vec<u8> = vec![];
        use rgb::ComponentSlice;
        arr.extend_from_slice(leds.as_slice());
        arr
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        let leds: [rgb::RGB8; 256] =  [BLACK; 256];
        self.apply(&leds);
    }
}

pub fn main() {
    let mut display = Display::build();

    let mut leds: [rgb::RGB8; 256] =  [BLACK; 256];

    let y = 3;
    let x = 7;
    leds[(y * 16) + x] = rgb::RGB8::new(0, 255, 0);
    
    display.apply(&leds);

    let mut counter = 0;
    let delay = time::Duration::from_millis(50);
    loop {
        leds[counter] = BLACK;
	counter = (counter + 1) % 255;
	leds[counter] = rgb::RGB8::new(0, 255, 0);
        display.apply(&leds);
        thread::sleep(delay);
    }
}
