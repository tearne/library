use rgb::RGBA8;
use unicorn::pimoroni::unicorn::Unicorn;

use crate::types::Pixel;

pub type Grid16Square = Grid<16, 16>;

pub struct Grid<const W: usize, const H: usize>{
    pub data: [[RGBA8; H]; W]
}
impl<const W: usize, const H: usize> Grid<W, H> {
    pub fn new() -> Self {
        Grid{ data: [[RGBA8::default(); H]; W] }
    }

    pub fn width(&self) -> usize { W }
    pub fn height(&self) -> usize { H }

    pub fn place(&mut self, px: Pixel, opacity: f32) {
        let current = self.data[px.position.x][px.position.y];
        let Pixel{ position, colour } = px;
        let with_opacity = colour.map_alpha(|a| (a as f32 * opacity) as u8);
        // println!("opacity: {}, pos:  {:?}, op: {:?}", opacity, position, with_opacity);
        let blended = Self::mix(current, with_opacity);
        self.data[position.x][position.y] = blended;
    }

    fn mix(a: RGBA8, b: RGBA8) -> RGBA8 {
        let c_alpha: f32 = b.a as f32 + a.a as f32 * (255.0 - b.a as f32);

        let comp = |b:u8, b_alpha: u8, a:u8, a_alpha: u8| (b as f32 * b_alpha as f32 / 255.0) + a as f32 * a_alpha as f32 * (255.0 - b_alpha as f32) / c_alpha;

        let red: f32 = comp(b.r, b.a, a.r, a.a);
        let green: f32 = comp(b.g, b.a, a.g, a.a);
        let blue: f32 = comp(b.b, b.a, a.b, a.a);
        
        RGBA8::new(red as u8, green as u8, blue as u8, c_alpha as u8)
    }

    pub fn add_layer_on_top(&mut self, other: &Grid<W,H>) {
        for x in 0..W {
            for y in 0..H {
                self.data[x][y] = Self::mix(
                    self.data[x][y],
                    other.data[x][y]
                );
            }
        }
    }

    pub fn send_to_display(&self, display: &mut Unicorn) {
        for x in 0..W {
            for y in 0..H {
                display.set_xy(x, y, &self.data[x][y].rgb());
            }
        }
    }
}