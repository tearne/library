use rgb::RGBA8;
use unicorn::pimoroni::unicorn::Unicorn;

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

    pub fn add_layer_on_top(&mut self, other: &Grid<W,H>) {
        for x in 0..W {
            for y in 0..H {
                let s = self.data[x][y];
                let o = other.data[x][y];

                let a_1: f32 = o.a as f32 + s.a as f32 * (255.0 - o.a as f32);

                let comp = |o:u8, oa: u8, s:u8, sa: u8| (o as f32 * oa as f32 / 255.0) + s as f32 * sa as f32 * (255.0 - oa as f32) / a_1;

                let r_1: f32 = comp(o.r, o.a, s.r, s.a);
                let g_1: f32 = comp(o.g, o.a, s.g, s.a);
                let b_1: f32 = comp(o.b, o.a, s.b, s.a);
                
                let res = RGBA8::new(r_1 as u8, g_1 as u8, b_1 as u8, a_1 as u8);

                self.data[x][y] = res;
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