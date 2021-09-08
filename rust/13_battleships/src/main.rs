use types::{Boat, Orientation, Position};
use unicorn::{keyboard, pimoroni::{RGB, unicorn::Unicorn}};

const KEY_UP: u16 = 103;
const KEY_DOWN: u16 = 108;
const KEY_LEFT: u16 = 105;
const KEY_RIGHT: u16 = 106;

mod types;

#[tokio::main]
async fn main() {
    let mut rx = keyboard::grab_all_keyboards();
    let mut display = Unicorn::new();

    let mut boat = Boat {
        orientation: Orientation::LEFT,
        position: Position { x: 10, y: 5 },
        size: 6,
    };

    let c1 = RGB::new(250,250,0);
    let c2 = RGB::new(250,0,250);
    let c3 = RGB::new(0,250,250);

    while let Some(e) = rx.recv().await {
        if e.value == 1 {
            println!("--> {:?}", e);
            display.reset();

            match e.code {
                2 => display.set_idx(20, &c1),
                3 => display.set_idx(20, &c2),
                4 => display.set_idx(20, &c3),
                5 => display.set_idx(20, &RGB::new(0,250,0)),
                6 => display.set_idx(20, &RGB::new(0,250,0)),
                7 => display.set_idx(20, &RGB::new(250,0,0)),
                KEY_UP => boat.set_orientation(Orientation::UP),
                KEY_DOWN => boat.set_orientation(Orientation::DOWN),
                KEY_LEFT => boat.set_orientation(Orientation::LEFT),
                KEY_RIGHT => boat.set_orientation(Orientation::RIGHT),
                _ => (),
            }

            
            boat.render_pixels().iter().for_each(|p|{
                display.set_xy(p.x(), p.y(), &p.colour);
            });
            display.flush();
        }
    }
}
