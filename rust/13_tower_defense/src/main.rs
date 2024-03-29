use grid::Grid16Square;
use rgb::RGBA8;
use types::Position;
use unicorn::{keyboard, pimoroni::unicorn::Unicorn};
use crate::types::{Game, Renderable, Zombie};
use std::convert::TryInto;

use crate::types::{Path, Tower, World};

mod types;
mod grid;
mod keys;

pub enum InteractionMode {
    S(Standby),
    P(Placing),
}
impl InteractionMode {
    pub fn handle_key(&self, key: u16, world: &mut World) -> Option<Self> {
        match self {
            InteractionMode::S(ref s) => s.handle_key(key),
            InteractionMode::P(ref p) => p.handle_key(key, world),
        }
    }
}
impl Renderable for InteractionMode {
    fn render_onto(&self, time: f32, grid: &mut Grid16Square) {
        match self {
            Self::S(_) => (),
            Self::P(p) => p.render_onto(time, grid),
        }
    }
}

pub struct Standby{}
impl Standby {
    fn handle_key(&self, key: u16) -> Option<InteractionMode> {
        Self::number_keys(key)
    }

    pub fn number_keys(key:u16) -> Option<InteractionMode> {
        match key {
            keys::KEY_1 => Some(InteractionMode::P(Placing::new(0))),
            keys::KEY_2 => Some(InteractionMode::P(Placing::new(1))),
            keys::KEY_3 => Some(InteractionMode::P(Placing::new(2))),
            keys::KEY_4 => Some(InteractionMode::P(Placing::new(3))),
            keys::KEY_5 => Some(InteractionMode::P(Placing::new(4))),
            keys::KEY_6 => Some(InteractionMode::P(Placing::new(5))),
            keys::KEY_7 => Some(InteractionMode::P(Placing::new(6))),
            keys::KEY_8 => Some(InteractionMode::P(Placing::new(7))),
            keys::KEY_9 => Some(InteractionMode::P(Placing::new(8))),
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
        Standby::number_keys(key)
            .or_else(|| match key {
                keys::KEY_UP => self.move_y(-1, 16),
                keys::KEY_DOWN => self.move_y(1, 16),
                keys::KEY_LEFT => self.move_x(-1, 16),
                keys::KEY_RIGHT => self.move_x(1, 16),
                keys::KEY_ENTER => {
                    if !world.path.points.contains(&self.position) 
                            && !world.towers.iter().any(|t|t.position == Some(self.position)) {
                        world.towers[self.tower_id].position = Some(self.position);
                        Some(InteractionMode::S(Standby{}))
                    } else { None }
                }
                _ => None
            })
    }
}
impl Renderable for Placing {
    fn render_onto(&self, time: f32, grid: &mut Grid16Square) {
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
    
    let world = World {
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
            Tower::new(7),
            Tower::new(8)
        ],
        zombies: vec![
            Zombie::new(1.0),
            Zombie::new(3.4),
            Zombie::new(5.8),
            Zombie::new(10.0),
        ],
        shots: vec![],
    };
    let mut game = Game::new(world);
    let start = std::time::SystemTime::now();
    let time_now = || start.elapsed().unwrap().as_millis() as f32 / 1000.0;
    game.render_layer(time_now()).send_to_display(&mut display);
    display.flush();

    loop {
        // println!(" - {}", time_now());
        game.render_layer(time_now()).send_to_display(&mut display);
        display.flush();
    };

    // while let Some(e) = rx.recv().await {
    //     if e.value == 1 {
    //         println!("--> {:?}", e);
    //         game.handle_key(e.code);

    //         game.render_layer(time_now()).send_to_display(&mut display);
    //         display.flush();
    //     }
    // }
}
