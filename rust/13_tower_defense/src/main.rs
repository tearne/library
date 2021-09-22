use grid::Grid16Square;
use rgb::RGBA8;
use types::Position;
use unicorn::{keyboard, pimoroni::unicorn::Unicorn};
use crate::types::{Game, Renderable};
use std::convert::TryInto;

use crate::types::{Path, Tower, World};

const KEY_ESC: u16 = 1;
const KEY_1: u16 = 2;
const KEY_2: u16 = 3;
const KEY_3: u16 = 4;
const KEY_4: u16 = 5;
const KEY_5: u16 = 6;
const KEY_6: u16 = 7;
const KEY_7: u16 = 8;
const KEY_8: u16 = 9;
const KEY_UP: u16 = 103;
const KEY_DOWN: u16 = 108;
const KEY_LEFT: u16 = 105;
const KEY_RIGHT: u16 = 106;
const KEY_W: u16 = 17;
const KEY_A: u16 = 30;
const KEY_S: u16 = 31;
const KEY_D: u16 = 32;
const KEY_ENTER: u16 = 28;

mod types;
mod grid;

pub enum InteractionMode {
    S(Standby),
    P(Placing),
}
impl InteractionMode {
    pub fn handle_key(&self, key: u16, world: &mut World) -> Option<Self> {
        match self {
            InteractionMode::S(ref s) => s.handle_key(key, world),
            InteractionMode::P(ref p) => p.handle_key(key, world),
        }
    }
}
impl Renderable for InteractionMode {
    fn render_onto(&self, grid: &mut Grid16Square) {
        match self {
            Self::S(_) => (),
            Self::P(p) => p.render_onto(grid),
        }
    }
}

pub struct Standby{}
impl Standby {
    fn handle_key(&self, key: u16, world: &mut World) -> Option<InteractionMode> {
        match key {
            KEY_1 => Some(InteractionMode::P(Placing::new(1))),
            KEY_2 => Some(InteractionMode::P(Placing::new(2))),
            KEY_3 => Some(InteractionMode::P(Placing::new(3))),
            KEY_4 => Some(InteractionMode::P(Placing::new(4))),
            KEY_5 => Some(InteractionMode::P(Placing::new(5))),
            KEY_6 => Some(InteractionMode::P(Placing::new(6))),
            KEY_7 => Some(InteractionMode::P(Placing::new(7))),
            KEY_8 => Some(InteractionMode::P(Placing::new(8))),
            _ => None
        }
    }
}

pub struct Placing{
    pub tower_id: usize,
    pub position: Position,
}
impl Placing {
    pub fn new(tower_id: usize) -> Self {
        Placing{
            tower_id,
            position: Position::new(3,5)
        }
    }

    fn move_x(&self, change: isize, width: usize) -> Option<InteractionMode> {
        if (change > 0 && self.position.x < width - 1) || (change < 0 && self.position.x > 0) {
            let position = self.position.x_plus(change);
            Some(InteractionMode::P(Placing{position, ..*self}))
        } else {
            None
        }
    }
    fn move_y(&self, change: isize, height: usize) -> Option<InteractionMode> {
        if (change > 0 && self.position.y < height - 2) || (change < 0 && self.position.y > 0) {
            let position = self.position.y_plus(change);
            Some(InteractionMode::P(Placing{position, ..*self}))
        } else {
            None
        }
    }

    fn handle_key(&self, key: u16, world: &mut World) -> Option<InteractionMode> {
        match key {
            KEY_UP => self.move_y(-1, 16),
            KEY_DOWN => self.move_y(1, 16),
            KEY_LEFT => self.move_x(-1, 16),
            KEY_RIGHT => self.move_x(1, 16),
            _ => None
        }
    }
}
impl Renderable for Placing {
    fn render_onto(&self, grid: &mut Grid16Square) {
        fn alpha(dist: isize) -> u8 {
            let d: usize = dist.try_into().unwrap();
            (255 - 255.min(d.min(5) * 50)).try_into().unwrap()
        }

        for x in 0..grid.width() {
            if x != self.position.x {
                let dist = (x as isize - self.position.x as isize).abs();
                grid.data[x][self.position.y] = RGBA8::new(0,0,150, alpha(dist));
            }
        }
        for y in 0..grid.height() {
            if y != self.position.y {
                let dist = (y as isize - self.position.y as isize).abs();
                grid.data[self.position.x][y] = RGBA8::new(0,0,150, alpha(dist));
            }
        }
    }
}


#[tokio::main]
async fn main() {
    let mut rx = keyboard::grab_all_keyboards();
    let mut display = Unicorn::new();
    
    let mut world = World {
        path: Path::build(
            vec![
                Position::new(0, 2),
                Position::new(13, 2),
                Position::new(13, 5),
                Position::new(2, 5),
                Position::new(2, 8),
                Position::new(13, 8),
                Position::new(13, 11),
                Position::new(2, 11)
            ]),
        towers: [
            Tower::new(0), 
            Tower::new(1),
            Tower::new(2), 
            Tower::new(3),
            Tower::new(4), 
            Tower::new(5),
            Tower::new(6), 
            Tower::new(7)
        ],
    };
    let mut game = Game::new(world);

    
    game.render_layer().send_to_display(&mut display);
    display.flush();


    while let Some(e) = rx.recv().await {
        if e.value == 1 {
            println!("--> {:?}", e);
            game.handle_key(e.code);

            // match e.code {
            //     // 2 => grid[1][6] = c1,
            //     // 3 => grid[2][5] = c2,
            //     // 4 => grid[3][4] = c2,
            //     // 5 => grid[4][3] = c1,
            //     // 6 => grid[5][2] = c2,
            //     // 7 => grid[6][1] = c1,
            //     KEY_UP => 
            //         game.
            //         game.selector.as_mut().unwrap().position.y_shift(-1),
            //     KEY_DOWN => 
            //         game.selector.as_mut().unwrap().position.y_shift(1),
            //     KEY_LEFT => 
            //         game.selector.as_mut().unwrap().position.x_shift(-1),
            //     KEY_RIGHT => 
            //         game.selector.as_mut().unwrap().position.x_shift(1),
            //     _ => (),
            // }

            game.render_layer().send_to_display(&mut display);
            display.flush();
        }
    }
}
