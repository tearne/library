use rgb::{RGBA8};
use unicorn::pimoroni::unicorn::Unicorn;
use std::convert::TryInto;

pub struct Grid([[RGBA8; 16]; 16]);
impl Grid {
    pub fn new() -> Self {
        Self([[RGBA8::default(); 16]; 16])
    }
    pub fn layer_on_top(&mut self, other: &Grid) {
        todo!();
    }

    pub fn send_to_display(&self, display: &mut Unicorn) {
        for x in 0..16usize {
            for y in 0..16usize {
                display.set_xy(x, y, &self.0[x][y].rgb());
            }
        }
    }
}

pub trait Renderable {
    fn render_layer(&self) -> Grid {
        let mut g = Grid::new();
        self.render_onto(&mut g);
        g
    }
    fn render_onto(&self, grid: &mut Grid);
}

impl<T: Renderable> Renderable for Option<T> {
    fn render_onto(&self, grid: &mut Grid) {
        self.iter().for_each(|s|s.render_onto(grid));
    }
}

pub struct Game {
    pub world: World,
    pub selector: Option<Selector>
}
impl Game {
    pub fn new(world: World) -> Self {
        Self {
            world,
            selector: None,
        }
    }
}
impl Renderable for Game {
    fn render_onto(&self, grid: &mut Grid) {
        let mut background = self.world.render_layer();
        let selector_layer = self.selector.render_layer();

        background.layer_on_top(&selector_layer);
        *grid = background;
    }
}

pub struct Selector {
    pub holding: Option<Tower>,
    pub position: Position,
}
impl Renderable for Selector {
    fn render_onto(&self, grid: &mut Grid) {
        let colour = ;
        
        fn alpha(dist: usize) -> u8 {
            
        }

        for x in 0..16 {
            if x != self.position.x {
                grid.0[x][self.position.y] = RGBA8::new(0,0,10, alpha)
            }
        }
        for y in 0..16 {
            if y != self.position.y {
                grid.0[self.position.x][y] = colour
            }
        }
    }
}

pub struct World {
    pub path: Path,
    pub towers: Vec<Tower>
}
impl Renderable for World {
    fn render_onto(&self, grid: &mut Grid) {
        self.path.render_onto(grid);

        self.towers.iter().for_each(|t|{
            grid.0[t.position.x][t.position.y] = RGBA8::new(200,20,50, 255);
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
    fn render_onto(&self, grid: &mut Grid) {
        self.points.iter().for_each(|p| {
            grid.0[p.x][p.y] = RGBA8::new(20,10,10, 255);
        });
    }
}


pub struct Tower {
    pub position: Position,
    charge_status: u8,
    charge_speed: u8,
    hit_points: u8,
}
impl Tower {
    pub fn new(start_pos: usize) -> Self {
        Self {
            position: Position::new(start_pos, 15),
            charge_status: 0,
            charge_speed: 0,
            hit_points: 0,
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
        println!("Between {:?} and {:?}", self, other);

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

        println!(" ---> {:?}", res);

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
