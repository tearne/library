use rgb::RGBA8;
use std::convert::TryInto;

use crate::{InteractionMode, Standby, grid::Grid16Square};

pub trait Renderable {
    fn render_layer(&self, time: f32) -> Grid16Square {
        let mut g = Grid16Square::new();
        self.render_onto(time, &mut g);
        g
    }
    fn render_onto(&self, time: f32, grid: &mut Grid16Square);
}

impl<T: Renderable> Renderable for Option<T> {
    fn render_onto(&self, time: f32, grid: &mut Grid16Square) {
        self.iter().for_each(|s|s.render_onto(time, grid));
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

    pub fn tick(&mut self) {
        self.world.tick();
    }
}
impl Renderable for Game {
    fn render_onto(&self, time: f32, grid: &mut Grid16Square) {
        let mut background = self.world.render_layer(time);
        let mut selector_layer = self.interaction_mode.render_layer(time);

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
    pub towers: [Tower; 9],
    pub zombies: Vec<Zombie>,
    pub shots: Vec<Shot>,
}
impl World {
    pub fn tick(&mut self, time: f32) {
        // Get rid of any zombies that walked off the end
        self.zombies = self.zombies
            .into_iter()
            .filter(|z|z.has_exited(time, &self.path))
            .collect();

        // Get rid of any shots that have landed
        self.shots = self.shots
            .into_iter()
            .filter(|s| s.expired(time))
            .collect();

        // Create new shots for zombies that are next to charged towers
        for t in self.towers.iter_mut() {
            if let Some(ref t_pos) = t.position {
                if t.ready_to_shoot(time) {
                    if let Some(zombie) = self.zombies.iter()
                        .min_by_key(|z|z.distance_from(t_pos, time, &self.path) as u64) {
                            self.shots.push(t.shoot_at(zombie));
                        }
                }
            }
        }
        

        //TODO Damage any zombies that were hit by shots


    }
}

impl Renderable for World {
    fn render_onto(&self, time: f32, grid: &mut Grid16Square) {
        self.path.render_onto(time, grid);

        let legend_y = grid.height()-1;
        self.towers.iter().enumerate().for_each(|(idx, t)|{
            t.position.iter().for_each(|pos|
                grid.data[pos.x][pos.y] = t.colour
            );
            grid.data[Tower::POSITIONS[idx]][legend_y] = t.colour;
        });

        let mut z_grid = Grid16Square::new();
        self.zombies.iter().for_each(|z|{
            if let Some(pixels) = z.pixels(time, self.path) {
                for p in pixels {
                    z_grid.data[p.position.x][p.position.y] = p.colour
                }
            }
        });

        grid.add_layer_on_top(&z_grid);
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
    fn render_onto(&self, time: f32, grid: &mut Grid16Square) {
        self.points.iter().for_each(|p| {
            grid.data[p.x][p.y] = RGBA8::new(20,10,10, 255);
        });
    }
}


pub struct Tower {
    pub position: Option<Position>,
    pub last_shot_time: f32,
    colour: RGBA8,
}
impl Tower {
    const COLOURS: [RGBA8; 9] = [
        RGBA8::new(100,0,0,255),
        RGBA8::new(0,100,0,255),
        RGBA8::new(0,0,100,255),
        RGBA8::new(100,100,0,255),
        RGBA8::new(100,0,100,255),
        RGBA8::new(0,100,100,255),
        RGBA8::new(100,100,100,255),
        RGBA8::new(50,150,50,255),
        RGBA8::new(50,50,150,255),
    ];

    const POSITIONS: [usize; 9] = [
        2,3,4,6,7,8,10,11,12
    ];

    pub fn new(idx: usize) -> Self {
        Self {
            position: None,
            last_shot_time: 0,
            colour: Self::COLOURS[idx]
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub fn new(colour: RGBA8, position: Position) -> Self {
        Pixel{ colour, position }
    }
}

pub struct Zombie {
    start_time: f32
}
impl Zombie {
    pub fn new(start_time: f32) -> Self {
        Zombie{ start_time }
    }

    // pub fn colour(&self) -> RGBA8 {
    //     RGBA8::new(200,100,50, 255)
    // }

    pub fn has_exited(&self, time: f32, path: &Path) -> bool {
        todo!();
    }

    pub fn distance_from(&self, position: &Position, time: f32, path: &Path) -> f32 {
        todo!();
    }

    pub fn pixels(&self, time: f32, path: Path) -> Option<Vec<Pixel>> {
        let diff = time - self.start_time;

        if diff < 0.0 || diff > path.points.len() as f32 { None }
        else {
            let idx = diff as usize;
            let frac = diff - idx as f32;
            let frac = frac.powf(4.0);

            let result = Vec::<Pixel>::with_capacity(2);
            let p = path.points[idx];
            let colour: RGBA8 = RGBA8::new(200,100,50, (255.0 * (1.0 - frac)).try_into().unwrap());
            result.push(Pixel::new(colour, p));

            if idx + 1 < path.points.len() {
                let p = path.points[idx+1];
                let colour: RGBA8 = RGBA8::new(200,100,50, (255.0 * frac).try_into().unwrap());
                result.push(Pixel::new(colour, p));
            }

            Some(result)
        }
    }
}

pub struct Shot {
    pub start_time: f32,
    pub start_position: Position,
    pub colour: RGBA8,
    pub target: Zombie,
}
impl Shot {
    pub fn expired(&self, time: f32) -> bool {
        self.start_time + 1.0 < time
    }

    pub fn render_onto(&self, target_pos: Position, time: f32, grid: &mut Grid16Square) {
        let elapsed = time - self.start_time;
        if elapsed < 0.5 {
            grid.place(
                Pixel::new(self.colour, self.start_position),  
                (elapsed * 500.0).try_into().unwrap()
            )
        } else if elapsed < 1.0 {
            grid.place(
                Pixel::new(self.colour, target_pos),  
                255.0.try_into().unwrap()
            )
        }
    }
}