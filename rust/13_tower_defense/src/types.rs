use rgb::RGBA8;
use std::convert::TryInto;

use crate::{InteractionMode, Standby, grid::Grid16Square};

pub trait Renderable {
    fn render_layer(&self) -> Grid16Square {
        let mut g = Grid16Square::new();
        self.render_onto(&mut g);
        g
    }
    fn render_onto(&self, grid: &mut Grid16Square);
}

impl<T: Renderable> Renderable for Option<T> {
    fn render_onto(&self, grid: &mut Grid16Square) {
        self.iter().for_each(|s|s.render_onto(grid));
    }
}

pub struct Game {
    interaction_mode: InteractionMode,
    world: World,
}
impl Game {
    pub fn new(world: World) -> Self {
        Self {
            world,
            interaction_mode: InteractionMode::S(Standby{}),
        }
    }

    pub fn handle_key(&mut self, key: u16) {
        if let Some(replacement) = self.interaction_mode.handle_key(key, &mut self.world) {
            self.interaction_mode = replacement;
        }
    }
}
impl Renderable for Game {
    fn render_onto(&self, grid: &mut Grid16Square) {
        let mut background = self.world.render_layer();
        let mut selector_layer = self.interaction_mode.render_layer();

        //The dot of the tower in the middle of the selector
        if let InteractionMode::P(placing) = &self.interaction_mode {
            let tower_id = placing.tower_id;
            let tower_colour = self.world.towers[tower_id].colour;
            let position = &placing.position;
            selector_layer.data[position.x][position.y] = tower_colour;
        }

        background.add_layer_on_top(&selector_layer);
        *grid = background;
    }
}

pub struct World {
    pub path: Path,
    pub towers: [Tower; 8]
}
impl Renderable for World {
    fn render_onto(&self, grid: &mut Grid16Square) {
        self.path.render_onto(grid);

        self.towers.iter().for_each(|t|{
            grid.data[t.position.x][t.position.y] = t.colour;
        });
    }
}

pub struct Path {
    pub points: Vec<Position>
}
impl Path {
    pub fn build(corners: Vec<Position>) -> Path {
        Path {
            points : corners
                .windows(2)
                .flat_map(|pslice|{
                    let a = pslice[0];
                    let b = pslice[1];
                    a.interpolate(&b)
                })
                .collect()
        }
    }
}
impl Renderable for Path {
    fn render_onto(&self, grid: &mut Grid16Square) {
        self.points.iter().for_each(|p| {
            grid.data[p.x][p.y] = RGBA8::new(20,10,10, 255);
        });
    }
}


pub struct Tower {
    pub position: Position,
    charge_status: u8,
    charge_speed: u8,
    hit_points: u8,
    colour: RGBA8,
}
impl Tower {
    const COLOURS: [RGBA8; 8] = [
        RGBA8::new(100,0,0,255),
        RGBA8::new(0,100,0,255),
        RGBA8::new(0,0,100,255),
        RGBA8::new(100,100,0,255),
        RGBA8::new(100,0,100,255),
        RGBA8::new(0,100,100,255),
        RGBA8::new(100,100,100,255),
        RGBA8::new(50,150,50,255),
    ];

    pub fn new(idx: usize) -> Self {
        Self {
            position: Position::new(idx, 15),
            charge_status: 0,
            charge_speed: 0,
            hit_points: 0,
            colour: Self::COLOURS[idx]
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        ((self.x as f32- other.x as f32).powf(2.0) + (self.y as f32- other.y as f32).powf(2.0)).sqrt()
    }

    pub fn interpolate(&self, other: &Self) -> Vec<Position> {
        fn abs_diff(a: usize, b: usize) -> isize {
            let a = TryInto::<isize>::try_into(a).unwrap();
            let b = TryInto::<isize>::try_into(b).unwrap();
            (a - b).abs()
        }
        
        let x_diff = abs_diff(other.x, self.x);
        let y_diff = abs_diff(other.y, self.y);
        
        assert!(x_diff != 0 || y_diff != 0, "Diagonal interpolation not supported yet");

        let res: Vec<Position> = if self.x > other.x {
            (0..x_diff).map(|p|self.x_plus(-p)).collect()
        } else if self.x < other.x {
            (0..x_diff).map(|p|self.x_plus(p)).collect()
        } else if self.y > other.y {
            (0..y_diff).map(|p|self.y_plus(-p)).collect()
        } else { //y_diff < 0
            (0..y_diff).map(|p|self.y_plus(p)).collect()
        };

        res
    }

    fn add(a: usize, b: isize) -> usize {
        let a_isize: isize = a.try_into().unwrap();
        (a_isize + b).try_into().unwrap()
    }

    pub fn x_shift(&mut self, shift: isize) {
        self.x = Self::add(self.x, shift);
    }

    pub fn y_shift(&mut self, shift: isize) {
        self.y = Self::add(self.y, shift);
    }

    pub fn x_plus(&self, shift: isize) -> Position {
        let mut clone = self.clone();
        clone.x_shift(shift);
        clone
    }

    pub fn y_plus(&self, shift: isize) -> Position {
        let mut clone = self.clone();
        clone.y_shift(shift);
        clone
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub position: Position,
    pub colour: RGBA8,
}
impl Pixel {
    pub fn new(position: Position, colour: RGBA8) -> Self {
        Pixel{position, colour}
    }

    pub fn x(&self) -> usize {
        self.position.x
    }

    pub fn y(&self) -> usize {
        self.position.y
    }
}
