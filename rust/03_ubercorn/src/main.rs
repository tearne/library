use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use rgb::*;
use std::io::prelude::*;
use std::{thread, time};
use rand::Rng;

const BLACK: RGB8 = RGB8 { r: 0, g: 0, b: 0 };

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
        spi.configure(&options).expect("SPI config error");

        Display{spi}
    }

    pub fn apply(&mut self, leds: &[rgb::RGB8]) {
        self.spi.write(&[0x72]).expect("SPI write error");
        let data = Self::as_array(&leds);
        self.spi.write(&data).expect("SPI write error");
    }

    fn as_array(leds: &[rgb::RGB8]) -> Vec<u8> {
        let mut arr: Vec<u8> = vec![];
        // use ComponentSlice;
        arr.extend_from_slice(leds.as_slice());
        arr
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        let leds: [RGB8; 256] =  [BLACK; 256];
        self.apply(&leds);
    }
}

pub fn main() {
    let mut display = Display::build();
    let mut rng = rand::thread_rng();

    let mut leds: [RGB8; 256] =  [BLACK; 256];

    let y = 3;
    let x = 7;
    leds[(y * 16) + x] = RGB8::new(0, 255, 0);
    
    display.apply(&leds);
    let delay = time::Duration::from_millis(1);

    for _ in 1..100000 {
        // for i in leds.iter_mut() {
        //     // i = RGB8::new(
        //     //     i.r * 0.9 as u8, 
        //     //     i.g * 0.9 as u8, 
        //     //     i.b * 0.9 as u8, 
        //     // );
        // }
        let id = rng.gen_range(0, 256);
        let bright = rng.gen::<f64>();
        let bright = bright * bright * bright;
        let r = (rng.gen_range(127.0, 256.0) * bright) as u8;
        let g = (rng.gen_range(0.0, 256.0) * bright) as u8;
        let b = (rng.gen_range(0.0, 256.0) * bright) as u8;
        leds[id] = rgb::RGB8::new(r, g, b);
        display.apply(&leds);
        thread::sleep(delay);
    }
}
