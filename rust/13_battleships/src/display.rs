use std::{cmp::min, io::Write};

use spidev::{SpiModeFlags, Spidev, SpidevOptions};

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }
}

const BLACK: RGB = RGB { r: 0, g: 0, b: 0 };
const ALL_BLACK: &[RGB] = &[BLACK; 256];

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
        
        let mut display = Display {
            spi,
        };
        display.reset();
        display
    }

    pub fn apply_single_layer(&mut self, led_layers: &[RGB]) {
        self.apply_multiple_layers(vec![&led_layers]);
    }

    pub fn apply_multiple_layers<'a>(&'a mut self, led_layers: Vec<&'a [RGB]>) {
        let flattened = Self::flatten(led_layers);

        self.spi.write(&[0x72]).expect("SPI write error");
        let data = Self::squash(&flattened);
        self.spi.write(&data).expect("SPI write error");
    }

    fn flatten(layers: Vec<&[RGB]>) -> Vec<RGB> {
        fn add_u8(a: u8, b: u8) -> u8 {
            min(u8::MAX as u16, a as u16 + b as u16) as u8
        }

        fn add_rgb8(a: RGB, b: RGB) -> RGB {
            RGB::new(add_u8(a.r, b.r), add_u8(a.g, b.g), add_u8(a.b, b.b))
        }

        let mut result = vec![BLACK; 256];
        for i in 0..256 {
            let mut dot_acc = BLACK;
            for layer in layers.iter() {
                dot_acc = add_rgb8(dot_acc, layer[i]);
            }
            result[i] = dot_acc;
        }

        result
    }

    fn squash(leds: &[RGB]) -> Vec<u8> {
        leds.iter()
            .flat_map(|pix| vec![pix.r, pix.g, pix.b])
            .collect()
    }

    pub fn reset(&mut self) {
        self.apply_single_layer(ALL_BLACK);
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        self.reset();
    }
}