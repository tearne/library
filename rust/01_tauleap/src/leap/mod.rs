mod params;
mod cached;

use params::Params;

trait AutonomousTauLeaper<T: Copy> {
    fn step(&self, y: T, step_size: f32, params: &Params) -> T;

    //TODO inner mutability on Params
    fn leap(&self, y0: T, params: &mut Params) -> T {
        let now = 0.0;
        let mut position = y0;
        let step_size = params.tau_sub_step;

        while now < params.time_step {
            position = self.step(position, step_size, &params);
        }

        position
    }
}

#[derive(Copy, Clone)]
pub struct SIC {
    pub s: u16,
    pub i: u16,
    pub c: u16
}

impl AutonomousTauLeaper<SIC> for SIC{
    fn step(&self, y0: SIC, step_size: f32, p: &Params) -> Self {
        let enviro: f32 = 0.01;

        fn next_poisson(rate: f32) -> u64 {
            use rand_distr::Distribution;
            rand_distr::Poisson::new(5.0).unwrap().sample(&mut rand::thread_rng())
        }

        use std::cmp::min;

        let s_to_i = min(next_poisson(p.beta * enviro * (self.s as f32) * step_size) as u16, self.s);
        let i_to_c = min(next_poisson(p.gamma * (self.i as f32) * step_size) as u16, self.i);
        let c_to_i = min(next_poisson(p.a * (self.c as f32) * step_size) as u16, self.c);

        SIC {
            s: self.s - s_to_i,
            i: self.i + s_to_i - i_to_c,
            c: self.c + i_to_c - c_to_i
        }
    }
}

// struct Environment_Leaper<T> {
//     rng: SmallRng,
// }

// impl AutonomousTauLeaper for Environment_Leaper<T> {
//     fn step(y: T, step_size: f32) -> T {

//     }
// }

