mod input_device;
mod error;
mod pixel;

use crate::pixel::Pixel;
use rand::prelude::ThreadRng;
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use rgb::*;
use std::{collections::HashSet, io::prelude::*};
use std::{thread, time};
use rand::Rng;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use libc::input_event;
use crate::error::Error;

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
}

impl Drop for Display {
    fn drop(&mut self) {
        let leds: [RGB8; 256] =  [BLACK; 256];
        self.apply(&leds);
    }
}

const KEY_DOWN: i32 = 1;

pub fn main() {
    let input_filename = "/dev/input/event0".to_string();
    let keys = input_device::key_map();

    let keys_wanted = [
        "Q", "W", "E", "R", "T", "Y", "SPACE"
    ];
    let keys_to_watch: HashSet<&u16> = keys_wanted
        .iter()
        .cloned()
        .map(|k| keys.get(k).unwrap())
        .collect();

    fn go_loop(tx: Sender<input_event>, input_filename: &str) -> Result<(), Error> {
        let mut input_device = input_device::InputDevice::open(input_filename).unwrap();
        input_device.grab()?;
    
        loop {
            let event = input_device.read_event().unwrap();
            if event.value == KEY_DOWN {
                tx.send(event)?
            }
        }
    }

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let ret = go_loop(tx, &input_filename);
        if let Err(e) = ret {
            println!("mapping for {} ended due to error: {}", input_filename, e);
        }
    });



    let mut display = Display::build();
    let mut rng = rand::thread_rng();

    //let mut leds: [RGB8; 256] =  [BLACK; 256];
    let mut pixels: Vec<Pixel> = Vec::new();
    for i in 0..256 {
        pixels.push(Pixel::new(&mut rng));
    }

    let delay = time::Duration::from_millis(40);

    loop {
        let received = rx.try_recv();
        
        received.map(|event| {
            if keys_to_watch.contains(&event.code) {
                println!("===> {:?}", event);
                let base_colour = RGB8::new(
                    rng.gen_range(0, 255),
                    rng.gen_range(0, 255),
                    rng.gen_range(0, 255)
                );
                for i in 0..256 {
                    let variant_colour = RGB8::new(
                        (base_colour.r as i16 + rng.gen_range(-50, 50)).max(0).min(255) as u8,
                        (base_colour.g as i16 + rng.gen_range(-50, 50)).max(0).min(255) as u8,
                        (base_colour.b as i16 + rng.gen_range(-50, 50)).max(0).min(255) as u8,
                    );
                    pixels[i].randomise(&mut rng, variant_colour);
                }
            }
        });

        let rendered: Vec<RGB8> = 
            pixels.iter_mut().map(|pix| pix.evolve_and_get()).collect();
        display.apply(&rendered);
        thread::sleep(delay);
    }
}
