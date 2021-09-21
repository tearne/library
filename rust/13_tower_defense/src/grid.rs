use rgb::RGBA8;
use unicorn::pimoroni::unicorn::Unicorn;

pub struct Grid(pub [[RGBA8; 16]; 16]);
impl Grid {
    pub fn new() -> Self {
        Self([[RGBA8::default(); 16]; 16])
    }
    pub fn add_layer_on_top(&mut self, other: &Grid) {
        for x in 0..16 {
            for y in 0..16 {
                let s = self.0[x][y];
                let o = other.0[x][y];

                let a_1: f32 = o.a as f32 + s.a as f32 * (255.0 - o.a as f32);

                let comp = |o:u8, oa: u8, s:u8, sa: u8| (o as f32 * oa as f32 / 255.0) + s as f32 * sa as f32 * (255.0 - oa as f32) / a_1;

                let r_1: f32 = comp(o.r, o.a, s.r, s.a);
                let g_1: f32 = comp(o.g, o.a, s.g, s.a);
                let b_1: f32 = comp(o.b, o.a, s.b, s.a);
                
                let res = RGBA8::new(r_1 as u8, g_1 as u8, b_1 as u8, a_1 as u8);

                self.0[x][y] = res;
            }
        }
    }

    pub fn send_to_display(&self, display: &mut Unicorn) {
        for x in 0..16usize {
            for y in 0..16usize {
                display.set_xy(x, y, &self.0[x][y].rgb());
            }
        }
    }
}