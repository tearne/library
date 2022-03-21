use rand::Rng;
use std::time::Duration;
use unicorn::pimoroni::unicornhatmini::{Button, UnicornHatMini};
// use unicorn::pimoroni::unicornhatmini::*;

#[tokio::main]
async fn main() {
    let mut um = UnicornHatMini::new();

    let mut rng = rand::thread_rng();

    let mut h = um.button_subscribe();
    loop {
        h.changed().await.unwrap();
        let b_opt = h.borrow_and_update();
        let t = b_opt.as_ref().unwrap();
        println!("==> {:?}", t);

        match *t {
            Button::A | Button::B => {
                for i in 0..17usize {
                    um.set_xy(i, 3, rng.gen(), rng.gen(), rng.gen());
                    um.flush();
                    std::thread::sleep(Duration::new(0, 10000000));
                }
            }
            Button::X | Button::Y => {
                for i in 0..7usize {
                    um.set_xy(7, i, rng.gen(), rng.gen(), rng.gen());
                    um.flush();
                    std::thread::sleep(Duration::new(0, 10000000));
                }
            }
        }
    }
}
