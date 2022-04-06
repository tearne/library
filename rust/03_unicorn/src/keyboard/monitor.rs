use crate::error::Error;
use inotify::{EventMask, Inotify, WatchMask};
use libc::input_event;
use std::{sync::mpsc, thread};

const KEYBOARD_DEVICE_PATH: &str = "/dev/input/event0";

pub fn start() -> mpsc::Receiver<input_event> {
    fn grab_keyboard(tx: mpsc::Sender<input_event>, input_filename: &str) -> Result<(), Error> {
        todo!()
    }

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let ret = grab_keyboard(tx.clone(), &KEYBOARD_DEVICE_PATH);
        if let Err(e) = ret {
            println!("No keyboard: {}", e);

            let mut inotify = Inotify::init().expect("Failed to initialize inotify");
            inotify
                .add_watch("/dev/input/", WatchMask::CREATE)
                .expect("Failed to add inotify watch on /dev/input");

            let mut buffer = [0u8; 4096];
            loop {
                let events = inotify.read_events_blocking(&mut buffer);
                if let Ok(events) = events {
                    events
                        .into_iter()
                        .filter(|event| !event.mask.contains(EventMask::ISDIR))
                        .map(|event| {
                            println!("Events: \n{:#?}", event);
                            event.name.and_then(|name| name.to_str())
                        })
                        .filter(|name| name.map(|name| name == "event0").unwrap_or(false))
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
