use unicorn::RGB;

pub struct World {
    players: Vec<Player>,
}

pub struct Player {
    boats: Vec<Boat>,
}

pub struct Boat {
    pub orientation: Orientation,
    pub position: Position,
    pub size: usize,
}
impl Boat {
    pub fn set_orientation(&mut self, o: Orientation) {
        self.orientation = o;
    }

    pub fn render_pixels(&self) -> Vec<Pixel> {
        let halfish = ((self.size as f32) / 2.0) as usize;
        let offsets: Vec<isize> = (0..self.size as isize)
            .into_iter()
            .map(|i| i - halfish as isize)
            .collect();

        let colour = RGB::new(100, 200, 250);

        let px_bldr = |plotter: fn(&Position, isize) -> Position| -> Vec<Pixel> {
            offsets
                .into_iter()
                .map(|offset| Pixel {
                    colour,
                    position: plotter(&self.position, offset),
                })
                .collect::<Vec<Pixel>>()
        };

        let vec = match self.orientation {
            Orientation::UP => px_bldr(|pos, offset| pos.y_plus(offset)),
            Orientation::DOWN => px_bldr(|pos, offset| pos.y_plus(-offset)),
            Orientation::LEFT => px_bldr(|pos, offset| pos.x_plus(offset)),
            Orientation::RIGHT => px_bldr(|pos, offset| pos.x_plus(-offset)),
        };

        vec
    }
}

pub enum Orientation {
    UP,
    DOWN,
    LEFT,
    RIGHT,
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

    fn add(a: usize, b: isize) -> usize {
        let updated_isize = a as isize + b;
        let updated_usize = updated_isize as usize;
        assert!(updated_usize as isize == updated_isize);
        updated_usize
    }

    pub fn x_plus(&self, other: isize) -> Position {
        Position {
            x: Self::add(self.x, other),
            y: self.y,
        }
    }

    pub fn y_plus(&self, other: isize) -> Position {
        Position {
            x: self.x,
            y: Self::add(self.y, other),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub position: Position,
    pub colour: RGB,
}
impl Pixel {
    pub fn x(&self) -> usize {
        self.position.x
    }

    pub fn y(&self) -> usize {
        self.position.y
    }
}
