struct Params {
    tau_sub_steps: f32,
    time_step: f32,
}

trait Autonomous_Tau_Leaper<T,P: Params> {
    fn step(y: T, step_size: f32) -> T;

    fn leap(y0: T, params: P) -> T {
        let now = 0.0;
        let position = y0.copy();
        let step_size = params.time_step / params.tau_sub_steps;
        
        while now < params.time_step {
            position = step(position, step_size)
        }
    }
}

struct SIR {
    s: u16,
    i: u16,
    r: u16
}
impl State {
    fn step(self: Self, environ: f32, p: Params, &mut rnd: SmallRNG) -> Self {
        fn nextPoisson(rate: f32, &mut rnd: SmallRNG) -> f32 {
            use rand_distr::{Poisson, Distribution};

            let poi = Poisson::new(2.0).unwrap();
            let v: u64 = poi.sample(&mut rand::thread_rng());

            use rand::distributions::Poisson;
            let poi = Poisson::new(rate);
            poi.sample()
        }
    }
}

struct Environment_Leaper<T> {
    rng: SmallRng,
}

impl Autonomous_Tau_Leaper for Environment_Leaper<T> {
    fn step(y: T, step_size: f32) -> T {

    }
}

