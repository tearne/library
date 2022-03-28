use rand::Rng;
use std::time::Duration;
use unicorn::pimoroni::unicornhatmini::{Button, UnicornHatMini};
// use unicorn::pimoroni::unicornhatmini::*;

#[tokio::main]
async fn main() {
    let mut um = UnicornHatMini::new();

    let mut rng = rand::thread_rng();

    let mut h = um.button_subscribe();

    fn fill_with_random_colour(um: &mut UnicornHatMini, rng: &mut impl Rng){
        let r = rng.gen();
        let g = rng.gen();
        let b = rng.gen();
        for i in 0..17usize {
            for j in 0..7usize {
               um.set_xy(i, j, r, g, b);
            }
        }
        um.flush();
    }


    loop {
        h.changed().await.unwrap();
        let b_opt = h.borrow_and_update();
        let t = b_opt.as_ref().unwrap();
        println!("==> {:?}", t);

        match *t {
            Button::A | Button::B | Button::X | Button::Y => {
                fill_with_random_colour(&mut um, &mut rng);
            }
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}
