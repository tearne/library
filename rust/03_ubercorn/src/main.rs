use libc::input_event;
use std::{collections::HashMap, sync::mpsc::Receiver};
use ubercorn::{filesystem::*, art::*, display::*, error::Error, keyboard::{device, monitor}, display::pixel::*};
use rand::prelude::ThreadRng;
use rgb::*;
use std::{thread, time};
use rand::Rng;

enum Mode {
    TWINKLE, ZOMBIE, SHEEP
}

struct KeyBuffer {
    key_buffer: Vec<&'static str>,
    max_length: usize,
    key_map: HashMap<u16, &'static str>,
}
impl KeyBuffer {
    pub fn new(length: usize) -> Self {
        KeyBuffer {
            key_buffer: Vec::<&str>::new(),
            max_length: length,
            key_map: device::key_map(),
        }
    }
    
    pub fn log_event(&mut self, e_res: &Result<input_event, Error>) {
        let t = e_res.iter();
        t.for_each(|e|{
            self.key_buffer.push(self.key_map.get(&e.code).unwrap_or(&""));
            if self.key_buffer.len() > self.max_length {
                self.key_buffer.remove(self.key_buffer.len() - self.max_length - 1);
            }
        });
    }

    pub fn contains(&self, str: &str) -> bool {
        let buf_length= self.key_buffer.len();
        let str_length = str.len();

        if buf_length < str_length {
            false
        } else {
            println!("{} - {}", buf_length, str_length);

            let section = self.key_buffer[(buf_length - str_length)..].join("");
            section == str
        }
    }
}


pub fn main() -> Result<(), Error>{
    let mut display = Display::build(FSStatus::init_polling());
    let delay = time::Duration::from_millis(3000);

    let mut leds: Vec<RGB8> = Vec::new();
    for _ in 0..256 {
        leds.push(RGB8::new(50,0,0));
    }
    display.apply(vec!(leds));
    thread::sleep(delay);

    leds = Vec::new();
    for _ in 0..256 {
        leds.push(RGB8::new(0,25,0));
    }
    display.apply(vec!(leds));
    thread::sleep(delay);

    leds = Vec::new();
    for _ in 0..256 {
        leds.push(RGB8::new(0,0,25));
    }
    display.apply(vec!(leds));
    thread::sleep(delay);

    let mut rng = rand::thread_rng();
    let mut key_buffer = KeyBuffer::new(6);

    let key_rx = monitor::start();

    let mut mode= Mode::TWINKLE;
    loop {
        match mode {
            Mode::TWINKLE => mode = do_twinkle(&mut display ,&key_rx , &mut rng, &mut key_buffer),
            Mode::ZOMBIE => mode = do_zombie(&mut display, &key_rx, &mut key_buffer),
            Mode::SHEEP => mode = do_sheep(&mut display, &key_rx, &mut key_buffer),
        }
    }
}

fn random_colour(rng: &mut ThreadRng) -> RGB8 {
    RGB8::new(
        rng.gen_range(0, 255),
        rng.gen_range(0, 255),
        rng.gen_range(0, 255)
    )
}

fn do_zombie(display: &mut Display, rx: &Receiver<input_event>, key_buffer: &mut KeyBuffer) -> Mode {
    display.apply(vec!(zombie::get()));
    let delay = time::Duration::from_millis(60000);
    let response: Result<input_event, Error> = rx.recv_timeout(delay).map_err(|e|e.into());
    key_buffer.log_event(&response);
    Mode::TWINKLE
}

fn do_sheep(display: &mut Display, rx: &Receiver<input_event>, key_buffer: &mut KeyBuffer) -> Mode {
    display.apply(vec!(sheep::get()));
    let delay = time::Duration::from_millis(60000);
    let response: Result<input_event, Error> = rx.recv_timeout(delay).map_err(|e|e.into());
    key_buffer.log_event(&response);
    Mode::TWINKLE
}

fn do_twinkle(display: &mut Display,rx: &Receiver<input_event>, rng: &mut ThreadRng, key_buffer: &mut KeyBuffer) -> Mode {
    let delay = time::Duration::from_millis(40);

    let mut pixels: Vec<Pixel> = Vec::new();
    for _ in 0..256 {
        pixels.push(Pixel::new(rng));
    }

    loop {
        let response: Result<input_event, Error> = rx.try_recv().map_err(|e|e.into());
              
        key_buffer.log_event(&response);
        if response.is_ok() {
            let base_colour = random_colour(rng);
            for i in 0..256 {
                let variant_colour = RGB8::new(
                    (base_colour.r as i16 + rng.gen_range(-50, 50)).max(0).min(255) as u8,
                    (base_colour.g as i16 + rng.gen_range(-50, 50)).max(0).min(255) as u8,
                    (base_colour.b as i16 + rng.gen_range(-50, 50)).max(0).min(255) as u8,
                ); 
                pixels[i].randomise(rng, variant_colour);
            }
            if key_buffer.contains("ROWAN") {
                break Mode::ZOMBIE
            }
            if key_buffer.contains("ANNA") {
                break Mode::SHEEP
            }
        }

        let rendered: Vec<RGB8> = pixels.iter_mut()
            .map(|px| px.evolve_and_get())
            .collect();
        
        display.apply(vec!(rendered));

        thread::sleep(delay);
    }
}
