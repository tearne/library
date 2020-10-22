use inotify::{EventMask, Inotify, WatchMask};
use libc::input_event;
// use nix::sys::inotify::Inotify;
use std::{sync::mpsc, thread};
use crate::input_device;
use crate::error::*;

const KEYBOARD_DEVICE_PATH: &str = "/dev/input/event0";

pub fn go() -> mpsc::Receiver<input_event> {
    
    fn grab_keyboard(tx: mpsc::Sender<input_event>, input_filename: &str) -> Result<(), Error> {
        const KEY_DOWN: i32 = 1;
        let mut input_device = input_device::InputDevice::open(input_filename)?;
        input_device.grab()?;

        let mut do_loop = || -> Result<(), Error>{
            loop {
                let event = input_device.read_event()?;
                if event.value == KEY_DOWN {
                    tx.send(event)?
                }
            }
        };

        if let Err(error) = do_loop() {
            let _thing = input_device.release();
            // Result::Err(err)
            Result::Err(error)
        } else {
            Result::Ok(())
        }
    }

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let ret = grab_keyboard(tx.clone(), &KEYBOARD_DEVICE_PATH);
        if let Err(e) = ret {
            println!("Lost keyboard: {}", e);
            
            let mut inotify = Inotify::init()
                .expect("Failed to initialize inotify");
            inotify.add_watch("/dev/input/", WatchMask::CREATE)
                .expect("Failed to add inotify watch on /dev/input");

            let mut buffer = [0u8; 4096];
            loop {
                let events = inotify.read_events_blocking(&mut buffer);
                if let Ok(events) = events {
                    events.into_iter()
                        .filter(|event| !event.mask.contains(EventMask::ISDIR))
                        .map(|event| event.name.and_then(|name| name.to_str()))
                        .filter(|name| name.map(|name|name == "event0").unwrap_or(false))
                        .for_each(|_| {
                            let delay = std::time::Duration::from_millis(1000);
                            std::thread::sleep(delay);
                            let ret = grab_keyboard(tx.clone(), KEYBOARD_DEVICE_PATH);
                            if let Err(e) = ret {
                                println!("Lost keyboard: {}", e);
                            }
                        })
                }
            }

        }
    });

    rx
}