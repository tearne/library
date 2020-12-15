use std::cell::RefCell;
use libc::input_event;
use std::{collections::HashMap, sync::mpsc::Receiver};
use ubercorn::{filesystem::*, display::*, error::Error, keyboard::*, display::pixel::*};
use rand::prelude::ThreadRng;
use std::{thread, time, path::Path};
use rand::Rng;

#[derive(PartialEq, Eq, Hash)]
enum Mode {
    TWINKLE, 
    ZOMBIE, 
    SHEEP,
}
impl Mode {
    pub fn trigger_seq<'a>(&'a self, deps: &'a Dependencies) -> &Vec<&str> {
        deps.mode_keys.get(self).unwrap()
    }
}

struct Dependencies {
    mode_keys: HashMap<Mode, Vec<&'static str>>,
    random_ref_cell: RefCell<ThreadRng>,
    art: HashMap<Mode, Vec<RGB>>
}
impl Dependencies {
    pub fn init() -> Self {
        let mut mode_keys = HashMap::new();
        mode_keys.insert(Mode::ZOMBIE, vec!("R"));
        mode_keys.insert(Mode::SHEEP, vec!("A"));

        let mut art = HashMap::new();
        art.insert(Mode::ZOMBIE, Graphic::load(Path::new("resources/zombie.json")).as_pixels());
        art.insert(Mode::SHEEP, Graphic::load(Path::new("resources/sheep.json")).as_pixels());
        
        Dependencies{
            mode_keys,
            random_ref_cell: RefCell::new(rand::thread_rng()),
            art,
        }
    }
}

pub fn main() -> Result<(), Error>{
    let deps = Dependencies::init();

    let mut display = Display::build(FSStatus::init_polling());
    let delay = time::Duration::from_millis(3000);
    thread::sleep(time::Duration::from_millis(100));

    //Test sequence
    let mut leds: Vec<RGB> = vec![RGB::new(0,0,0); 256];
    for i in 0..256 {
        leds[i] = RGB::new(50,0,0);
    }
    display.apply_single_layer(&leds);
    thread::sleep(delay);

    for i in 0..256 {
        leds[i] = RGB::new(0,25,0);
    }
    display.apply_single_layer(&leds);
    thread::sleep(delay);
    
    for i in 0..256 {
        leds[i] = RGB::new(0,0,25);
    }
    display.apply_single_layer(&leds);
    thread::sleep(delay);

    let mut key_buffer = KeyBuffer::new(6);

    let key_rx = monitor::start();

    let mut mode= Mode::TWINKLE;
    loop {
        match mode {
            Mode::TWINKLE => mode = do_twinkle(&mut display ,&key_rx ,&mut key_buffer, &deps),
            Mode::ZOMBIE => mode = do_zombie(&mut display, &key_rx, &mut key_buffer, &deps),
            Mode::SHEEP => mode = do_sheep(&mut display, &key_rx, &mut key_buffer, &deps),
        }
    }
}

fn random_colour(rng: &RefCell<ThreadRng>) -> RGB {
    let mut rng = rng.borrow_mut();
    
    RGB::new(
        rng.gen_range(0, 255),
        rng.gen_range(0, 255),
        rng.gen_range(0, 255)
    )
}

fn do_zombie(display: &mut Display, rx: &Receiver<input_event>, key_buffer: &mut KeyBuffer, deps: &Dependencies) -> Mode {
    let z = deps.art.get(&Mode::ZOMBIE).unwrap();
    display.apply_single_layer(&z);
    let delay = time::Duration::from_millis(60000);
    let response: Result<input_event, Error> = rx.recv_timeout(delay).map_err(|e|e.into());
    key_buffer.log_event(&response);
    Mode::TWINKLE
}

fn do_sheep(display: &mut Display, rx: &Receiver<input_event>, key_buffer: &mut KeyBuffer, deps: &Dependencies) -> Mode {
    let s = deps.art.get(&Mode::SHEEP).unwrap();
    display.apply_single_layer(&s);
    let delay = time::Duration::from_millis(60000);
    let response: Result<input_event, Error> = rx.recv_timeout(delay).map_err(|e|e.into());
    key_buffer.log_event(&response);
    Mode::TWINKLE
}

fn do_twinkle(display: &mut Display,rx: &Receiver<input_event>, key_buffer: &mut KeyBuffer, deps: &Dependencies) -> Mode {
    let delay = time::Duration::from_millis(40);

    let mut pixels: Vec<Pixel> = Vec::new();
    for _ in 0..256 {
        pixels.push(Pixel::new(&deps.random_ref_cell));
    }

    loop {
        let response: Result<input_event, Error> = rx.try_recv().map_err(|e|e.into());
              
        key_buffer.log_event(&response);
        if response.is_ok() {
            let base_colour = random_colour(&deps.random_ref_cell);
            let rnd = |low, high| deps.random_ref_cell.borrow_mut().gen_range(low, high);
            for i in 0..256 {
                let variant_colour = RGB::new(
                    (base_colour.r as i16 + rnd(-50, 50)).max(0).min(255) as u8,
                    (base_colour.g as i16 + rnd(-50, 50)).max(0).min(255) as u8,
                    (base_colour.b as i16 + rnd(-50, 50)).max(0).min(255) as u8,
                ); 
                pixels[i].randomise(&deps.random_ref_cell, variant_colour);
            }
            if key_buffer.contains(Mode::ZOMBIE.trigger_seq(deps)) {
                break Mode::ZOMBIE
            }
            if key_buffer.contains(Mode::SHEEP.trigger_seq(deps)) {
                break Mode::SHEEP
            }
        }

        let rendered: Vec<RGB> = pixels.iter_mut()
            .map(|px| px.evolve_and_get())
            .collect();
        
        display.apply_single_layer(&rendered);

        thread::sleep(delay);
    }
}
