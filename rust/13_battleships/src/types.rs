use unicorn::pimoroni::RGB;

pub struct World {
    players: Vec<Player>,
}

pub struct Player {
    boats: Vec<Boat>,
}

pub struct Boat {
    pub orientation: Orientation,
    pub position: Position,
    pub size: u8,
}
impl Boat {
    pub fn set_orientation(&mut self, o: Orientation){
        self.orientation = o;
    }

    pub fn render_pixels(&self) -> Vec<Pixel> {
        let halfish = ((self.size as f32) / 2.0) as u8;
        let offsets: Vec<i16> = (0..self.size as i16).into_iter().map(|i| i - halfish as i16).collect();

        let colour = RGB::new(100, 200, 250);

        let px_bldr = |plotter: fn(&Position, i16) -> Position| -> Vec<Pixel> {            
            offsets.into_iter().map(|offset| Pixel{
                colour,
                position: plotter(&self.position, offset)
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

pub enum Orientation{
    UP, 
    DOWN, 
    LEFT, 
    RIGHT,
}

pub struct Position {
    pub x: u8,
    pub y: u8,
}
impl Position{
    pub fn new(x: u8, y: u8) -> Self {
        Position{x,y}
    }

    fn add(a: u8, b:i16) -> u8 {
        let updated_i16 = a as i16 + b;
        let updated_u8 = updated_i16 as u8;
        assert!(updated_u8 as i16 == updated_i16);
        updated_u8
    }

    pub fn x_plus(&self, other: i16) -> Position {
        Position{
            x: Self::add(self.x, other),
            y: self.y
        }
    }

    pub fn y_plus(&self, other: i16) -> Position {
        Position{
            x: self.x,
            y: Self::add(self.y, other)
        }
    }
}

pub struct Pixel {
    pub position: Position, 
    pub colour: RGB,
}
impl Pixel {
    pub fn x(&self) -> u8 {
        self.position.x
    }

    pub fn y(&self) -> u8 {
        self.position.y
    }
}