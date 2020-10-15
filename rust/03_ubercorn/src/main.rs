mod input_device;
mod error;

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

const KEY_DOWN: i32 = 1;

pub fn main() {
    let input_filename = "/dev/input/event0".to_string();
    let mut keys_to_watch = HashSet::new();
    let keys = input_device::key_map();

    keys_to_watch.insert(keys.get("Q").unwrap());
    keys_to_watch.insert(keys.get("W").unwrap());
    keys_to_watch.insert(keys.get("E").unwrap());
    keys_to_watch.insert(keys.get("R").unwrap());
    keys_to_watch.insert(keys.get("T").unwrap());
    keys_to_watch.insert(keys.get("Y").unwrap());

    fn go_loop(tx: Sender<input_event>, input_filename: &str) -> Result<(), Error> {
        let mut input_device = input_device::InputDevice::open(input_filename).unwrap();
        input_device.grab()?;
    
        loop {
            let event = input_device.read_event().unwrap();
            // println!("{:?}", event);
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

    for event in rx {
        if keys_to_watch.contains(&event.code) {
            println!("I'm interested in {:?}", event);
        }
    }

    
    
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
