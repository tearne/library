pub mod pixel;

use std::cmp::min;
use rgb::*;
use spidev::*;
use std::{io::Write, sync::{Arc, Mutex}};

use crate::filesystem::FSStatus;

const BLACK: RGB8 = RGB8 { r: 0, g: 0, b: 0 };
const ALL_BLACK: &[RGB8] = &[BLACK; 256];

pub struct Display {
    spi: Spidev,
    file_sys_status: Arc<Mutex<FSStatus>>,  // Allows overlay if FS in RW mode
    red_overlay: [RGB8; 256],
}

impl Display {
    fn get_fs_status(&self) -> FSStatus {
        *self.file_sys_status.lock().unwrap()
    }

    pub fn build(file_sys_status: Arc<Mutex<FSStatus>>) -> Self {
        let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(9_000_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options).expect("SPI config error");

        let mut red_overlay = [BLACK; 256];
        let red = RGB8::new(200,0,0);
        red_overlay[0] = red;
        red_overlay[1] = red;
        red_overlay[14] = red;
        red_overlay[15] = red;
        red_overlay[240] = red;
        red_overlay[241] = red;
        red_overlay[254] = red;
        red_overlay[255] = red;
        let mut display = Display{
            spi,
            file_sys_status,
            red_overlay,
        };
        display.reset();
        display
    }

    pub fn apply_single_layer(&mut self, led_layers: &[RGB8]) {
        self.apply_layers(vec![&led_layers]);
    }

    // TODO understand explicit lifetime annotation
    pub fn apply_layers<'a>(&'a mut self, mut led_layers: Vec<&'a [RGB8]>) {
        if self.get_fs_status() == FSStatus::ReadWrite {
            led_layers.push(&self.red_overlay);
        };

        let flattened = Self::flatten(led_layers);

        self.spi.write(&[0x72]).expect("SPI write error");
        let data = Self::as_u8(&flattened);
        self.spi.write(&data).expect("SPI write error");
    }

    fn flatten(layers: Vec<&[RGB8]>) -> Vec<RGB8> {
        fn add_u8(a: u8, b: u8) -> u8 {
            min(u8::MAX as u16, a as u16 + b as u16) as u8
        }
    
        fn add_rgb8(a: RGB8, b: RGB8) -> RGB8 {
            RGB8::new(
                add_u8(a.r, b.r),
                add_u8(a.g, b.g),
                add_u8(a.b, b.b)
            )
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

    fn as_u8(leds: &[rgb::RGB8]) -> Vec<u8> {
        let mut arr: Vec<u8> = vec![];
        arr.extend_from_slice(leds.as_slice());
        arr
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