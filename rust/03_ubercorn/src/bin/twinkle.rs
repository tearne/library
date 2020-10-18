use ubercorn::pixel::*;
use ubercorn::error::*;
use ubercorn::display::*;
use ubercorn::input_device;

use rand::prelude::ThreadRng;
use rgb::*;
use std::{thread, time, collections::HashSet};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use rand::Rng;
use libc::input_event;

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

    fn grab_keyboard(tx: Sender<input_event>, input_filename: &str) -> Result<(), Error> {
        let mut input_device = input_device::InputDevice::open(input_filename).unwrap();
        input_device.grab()?;
        const KEY_DOWN: i32 = 1;

        loop {
            let event = input_device.read_event().unwrap();
            if event.value == KEY_DOWN {
                tx.send(event)?
            }
        }
    }

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let ret = grab_keyboard(tx, &input_filename);
        if let Err(e) = ret {
            println!("mapping for {} ended due to error: {}", input_filename, e);
        }
    });

    let mut display = Display::build();
    let mut rng = rand::thread_rng();

    //let mut leds: [RGB8; 256] =  [BLACK; 256];
    let mut pixels: Vec<Pixel> = Vec::new();
    for _ in 0..256 {
        pixels.push(Pixel::new(&mut rng));
    }

    let delay = time::Duration::from_millis(40);

    fn random_colour(rng: &mut ThreadRng) -> RGB8 {
        RGB8::new(
            rng.gen_range(0, 255),
            rng.gen_range(0, 255),
            rng.gen_range(0, 255)
        )
    }

    loop {
        rx.try_recv().into_iter().for_each(|event| {
            if keys_to_watch.contains(&event.code) {

                let base_colour = random_colour(&mut rng);
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
            pixels.iter_mut().map(|px| px.evolve_and_get()).collect();
        display.apply(&rendered);

        thread::sleep(delay);
    }
}