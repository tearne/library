mod input_device;
mod error;

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

    let mut leds: [RGB8; 256] =  [BLACK; 256];

    let y = 3;
    let x = 7;
    leds[(y * 16) + x] = RGB8::new(0, 255, 0);
    
    display.apply(&leds);
    let delay = time::Duration::from_millis(100);

    let mut ranges: [f64; 6] = [
        127.0, 256.0,
        0.0, 127.0,
        0.0, 127.0
    ];

    fn randomise_range(arr: &mut [f64; 6], rng: &mut ThreadRng){
        arr[0] = rng.gen_range(0.0, 250.0);
        arr[1] = rng.gen_range(arr[0], 256.0);
        arr[2] = rng.gen_range(0.0, 250.0);
        arr[3] = rng.gen_range(arr[2], 256.0);
        arr[4] = rng.gen_range(0.0, 250.0);
        arr[5] = rng.gen_range(arr[4], 256.0);
    }

    //for event in rx {
    loop {
        let received = rx.try_recv();
        
        received.map(|event| {
            if keys_to_watch.contains(&event.code) {
                println!("===> {:?}", event);
                randomise_range(&mut ranges, &mut rng);
            }
        });

        let id = rng.gen_range(0, 256);
        let bright = rng.gen::<f64>();
        let bright = bright * bright * bright;
        let r = (rng.gen_range(ranges[0], ranges[1]) * bright) as u8;
        let g = (rng.gen_range(ranges[2], ranges[3]) * bright) as u8;
        let b = (rng.gen_range(ranges[4], ranges[5]) * bright) as u8;
        leds[id] = rgb::RGB8::new(r, g, b);
        display.apply(&leds);
        thread::sleep(delay);
    }
}
