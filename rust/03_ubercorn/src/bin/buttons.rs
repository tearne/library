use std::time::{Duration, SystemTime};

use rppal::gpio::{Gpio, InputPin, Trigger};
use tokio::{sync::watch::{self, Receiver}, task::JoinHandle};

#[derive(Debug)]
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
    rx: Receiver<Option<Button>>,
    #[allow(dead_code)]
    button_join_handle: JoinHandle<()>,
}
impl UnicornMini {
    pub fn new() -> Self {
        let mut gpio = Gpio::new().unwrap();

        fn get_pin(gpio: &mut Gpio, id: u8)  -> InputPin {
            println!("-- {}", id);
            let mut pin = gpio.get(id).unwrap().into_input();
            pin.set_interrupt(Trigger::FallingEdge).unwrap();
            pin
        }

        let pins = [
            get_pin(&mut gpio, Button::A.pin()),
            get_pin(&mut gpio, Button::B.pin()),
            get_pin(&mut gpio, Button::X.pin()),
            get_pin(&mut gpio, Button::Y.pin()),
        ];

        let (tx, rx) = watch::channel(Option::<Button>::None);

        let button_join_handle: JoinHandle<()> = tokio::task::spawn_blocking(move || {
            let p: [&InputPin; 4] = [
                &pins[0],
                &pins[1],
                &pins[2],
                &pins[3],
            ];
            
            let mut prev_time = SystemTime::now();

            loop {
                let result = gpio.poll_interrupts(
                    &p,
                    true,
                    None
                );

                let elapsed = prev_time.elapsed().unwrap_or_default();
                
                if elapsed > Duration::new(0, 100000000) {
                    prev_time = SystemTime::now();
                    let result = result.unwrap();
                    let (pressed_pin,_) = result.as_ref().unwrap();

                    if *pressed_pin == p[0]      { tx.send(Button::A.into()).unwrap(); } 
                    else if *pressed_pin == p[1] { tx.send(Button::B.into()).unwrap(); } 
                    else if *pressed_pin == p[2] { tx.send(Button::X.into()).unwrap(); } 
                    else if *pressed_pin == p[3] { tx.send(Button::Y.into()).unwrap(); }
                }
            };
        });

        UnicornMini{
            button_join_handle,
            rx,
        }
    }

    pub fn subscribe(&self) -> Receiver<Option<Button>> {
        self.rx.clone()
    }
}

#[tokio::main]
async fn main() {
    let um = UnicornMini::new();

    let mut h = um.subscribe();
    loop {
        h.changed().await.unwrap();
        let b_opt = h.borrow_and_update();
        let t = b_opt.as_ref().unwrap();
        println!("==> {:?}", t);
    }
}