pub mod pixel;

use serde::Deserialize;
use spidev::*;
use std::{cmp::min, collections::HashMap, fs::File, io::BufReader, path::Path};
use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use crate::filesystem::FSStatus;

const BLACK: RGB = RGB { r: 0, g: 0, b: 0 };
const ALL_BLACK: &[RGB] = &[BLACK; 256];

// Convert x & y to pixel number
pub fn to_idx(x: usize, y: usize) -> usize {
    (16 - x) * 16 + (16 - y)
}
#[derive(Deserialize, Debug, Clone, Copy)]
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

#[derive(Deserialize, Debug)]
struct Layer {
    colour: String,
    points: Vec<(usize, usize)>,
}

#[derive(Deserialize, Debug)]
pub struct Graphic {
    name: String,
    colours: HashMap<String, RGB>,
    groups: Vec<Layer>,
}
impl Graphic {
    pub fn load(path: &Path) -> Self {
        let reader = BufReader::new(File::open(path).unwrap());
        serde_json::from_reader(reader).unwrap()
    }

    pub fn as_pixels(&self) -> Vec<RGB> {
        let black = RGB { r: 0, g: 0, b: 0 };
        let mut pixels = vec![black; 256];

        self.groups.iter().for_each(|layer| {
            let colour = self.colours.get(&(layer.colour)).unwrap();
            for (x, y) in layer.points.iter() {
                pixels[to_idx(*x, *y)] = colour.clone();
            }
        });

        pixels
    }
}

pub struct Display {
    spi: Spidev,
    file_sys_status: Arc<Mutex<FSStatus>>, // Allows overlay if FS in RW mode
    red_overlay: [RGB; 256],
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
        let red = RGB::new(200, 0, 0);
        red_overlay[0] = red;
        red_overlay[1] = red;
        red_overlay[14] = red;
        red_overlay[15] = red;
        red_overlay[16] = red;
        red_overlay[31] = red;

        red_overlay[224] = red;
        red_overlay[239] = red;
        red_overlay[240] = red;
        red_overlay[241] = red;
        red_overlay[254] = red;
        red_overlay[255] = red;
        let mut display = Display {
            spi,
            file_sys_status,
            red_overlay,
        };
        display.reset();
        display
    }

    pub fn apply_single_layer(&mut self, led_layers: &[RGB]) {
        self.apply_layers(vec![&led_layers]);
    }

    pub fn apply_layers<'a>(&'a mut self, mut led_layers: Vec<&'a [RGB]>) {
        if self.get_fs_status() == FSStatus::ReadWrite {
            led_layers.push(&self.red_overlay);
        };

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
