use rand::rngs::SmallRng;

struct Params {
    tau_sub_steps: f32,
    time_step: f32,
}

trait AutonomousTauLeaper<T: Copy> {
    fn step(y: T, step_size: f32) -> T;

    fn leap(y0: T, params: Params) -> T {
        let now = 0.0;
        let position = y0;
        let step_size = params.time_step / params.tau_sub_steps;
        
        while now < params.time_step {
            position = step(position, step_size)
        }

        position
    }
}

pub struct SIR {
    s: u16,
    i: u16,
    r: u16
}

impl SIR {
    fn step(self: Self, environ: f32, p: Params, &mut rnd: SmallRNG) -> Self {
        fn nextPoisson(rate: f32, &mut rnd: rand::rngs::SmallRng) -> f32 {
            rand_distr::Poisson::new(5.0).unwrap().sample(&mut rand::thread_rng());
        }

        // val StoI = min(nextPoisson(beta * envPressure * susceptible * timeStep), susceptible)
        // val ItoC = min(nextPoisson(gamma * infectious * timeStep), infectious)
        // val CtoI = min(nextPoisson(a * carrier * timeStep), carrier)
  
        // StateGroup(
        //   penState.susceptible - StoI,
        //   penState.infectious + StoI - ItoC + CtoI,
        //   penState.carrier + ItoC - CtoI
        // )

        let 
    }
}

struct Environment_Leaper<T> {
    rng: SmallRng,
}

impl AutonomousTauLeaper for Environment_Leaper<T> {
    fn step(y: T, step_size: f32) -> T {

    }
}

