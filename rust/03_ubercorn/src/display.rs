use rgb::*;
use spidev::*;
use std::io::Write;

const BLACK: RGB8 = RGB8 { r: 0, g: 0, b: 0 };

pub struct Display {
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

        let mut display = Display{spi};
        // display.reset();
        display
    }

    pub fn apply(&mut self, leds: &[RGB8]) {
        self.spi.write(&[0x72]).expect("SPI write error");
        let data = Self::as_u8(&leds);
        self.spi.write(&data).expect("SPI write error");
    }

    fn as_u8(leds: &[rgb::RGB8]) -> Vec<u8> {
        let mut arr: Vec<u8> = vec![];
        // use ComponentSlice;
        arr.extend_from_slice(leds.as_slice());
        arr
    }

    fn reset(&mut self) {
        let leds: [RGB8; 256] =  [BLACK; 256];
        self.apply(&leds);
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        self.reset();
    }
}