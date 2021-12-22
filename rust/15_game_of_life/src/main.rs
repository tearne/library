use std::time::Duration;

use rgb::{RGB, RGB8};
use unicorn::pimoroni::unicorn::Unicorn;

const OFFSETS: [(u8,u8); 8] = [
    (0,14),
    (0,15),
    (15,14),
    (15,0),
    (15,15),
    (14,14),
    (14,0),
    (14,15)
];

const BLACK: RGB8 = RGB8::new(0,0,0);
const RED: RGB8 = RGB8::new(100,0,0);

struct Grid(Vec<RGB8>);
impl Grid {
    pub fn new() -> Self {
        Grid(vec![RGB::new(255, 0, 0); 256])
    }

    pub fn num_live_neighbours_at(&self, i:u8, j:u8) -> u8 {
        let prev_colour_at = |i:u8, j:u8| -> RGB8 {
            self.0[(i + j * 16) as usize] 
        };

        OFFSETS.iter().map(|(a,b)|{
            if prev_colour_at((i+a)%16, (j+b)%16) != BLACK { Some(true) }
            else { None }
        }).count() as u8
    }

    pub fn evolve(&self) -> Self {
        let new_grid: Vec<RGB8> = (0..self.0.len())
            .map(|i: usize|{
                let x = (i % 16) as u8;
                let y = (i / 16) as u8; 

                let px = self.0[i];
                let nbr_count = self.num_live_neighbours_at(x,y);
                if px == BLACK {
                    if nbr_count == 3 { RED }
                    else { BLACK }
                } else {
                    if nbr_count < 2 { BLACK }
                    else if nbr_count > 3 { BLACK }
                    else { RED }
                }
            })
            .collect();
        
        Grid(new_grid)
    }
}

struct World {
    pub grid: [RGB8; 256],
    pub previous: [RGB8; 256],
}
impl World {
    pub fn new() -> Self {
        World{
            grid: [RGB::new(255, 0, 0); 256],
            previous: [RGB::new(255, 0, 0); 256],
        }
    }

    pub fn evolve(&mut self) -> Self {
        World{
            grid: self.grid.evolve(),
            previous: g,
        }
    }
}


fn main() {
    println!("Hello, world!");

    let mut display = Unicorn::new();
    let r = RGB::new(255, 0, 0);
    let g = RGB::new(0, 255, 0);
    let b = RGB::new(0, 0, 255);

    display.set_xy(0, 0, &r);
    display.set_xy(1, 0, &r);
    display.set_xy(2, 0, &r);

    display.set_xy(3, 0, &r);
    display.set_xy(4, 0, &g);
    display.set_xy(4, 1, &b);

    display.set_xy(4, 2, &b);
    display.set_xy(4, 3, &b);
    display.set_xy(4, 4, &b);
    display.flush();
    std::thread::sleep(Duration::from_millis(10000));
}
