use futures::{FutureExt};
use gpio_cdev::{Chip, EventRequestFlags, Line, LineHandle, LineRequestFlags, Lines};
use rppal::gpio::{Gpio, InputPin, Trigger};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use tokio::{sync::{broadcast::{self, Receiver, channel}, watch}, task::JoinHandle};
use core::time;
use std::{borrow::{Borrow, BorrowMut, Cow}, io::Write, ops::Range};

enum Button {
    A,B,X,Y
}
impl Button {
    pub fn pin(&self) -> u8 {
        match self {
            Button::A => 5,
            Button::B => 6,
            Button::X => 16,
            Button::Y => 24
        }
    }
}

struct UnicornMini {
    join_handle: JoinHandle<()>,
}
impl UnicornMini {
    

    pub fn new(mut handler: Box<dyn FnMut(String) + Send>) -> Self {
        let mut gpio = Gpio::new().unwrap();

        fn get_pin(gpio: &mut Gpio, id: u8)  -> InputPin {
            println!("-- {}", id);
            let mut pin = gpio.get(id).unwrap().into_input();
            pin.set_interrupt(Trigger::FallingEdge);
            pin
        }

        let pins = [
            get_pin(&mut gpio, Button::A.pin()),
            get_pin(&mut gpio, Button::B.pin()),
            get_pin(&mut gpio, Button::X.pin()),
            get_pin(&mut gpio, Button::Y.pin()),
        ];

        let join_handle: JoinHandle<()> = tokio::task::spawn_blocking(move || {
            let p: [&InputPin; 4] = [
                &pins[0],
                &pins[1],
                &pins[2],
                &pins[3],
            ];
            
            loop {
                let result = gpio.poll_interrupts(
                    &p,
                    true,
                    None
                );
                
                let result = result.unwrap();
                let (pressed_pin,v) = result.as_ref().unwrap();

                if *pressed_pin == p[0] {
                    handler("A".into());
                } else if *pressed_pin == p[1] {
                    handler("B".into());
                } else if *pressed_pin == p[2] {
                    handler("X".into());
                } else if *pressed_pin == p[3] {
                    handler("Y".into());
                }
            };
        });

        UnicornMini{
            join_handle,
        }
    }
}

#[tokio::main]
async fn main() {
    let handler = |msg| println!("{}", msg);
    let um = UnicornMini::new(Box::new(handler));

    um.join_handle.await;
}