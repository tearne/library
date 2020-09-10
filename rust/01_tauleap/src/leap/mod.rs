pub mod params;

use params::Params;
use crate::epi::state::State;
use crate::epi::environment::Environment;

pub trait AutonomousTauLeaper<T: Copy> {
    fn step(y0: T, env: &Environment, params: &Params) -> T;

    fn leap(y0: T, env: &Environment, params: &Params) -> T {
        let mut now = 0.0;
        let mut position = y0;
        let step_size = params.tau_sub_step;

        while now < params.time_step {
            position = Self::step(position, env, params);
            now = now + step_size;
        }

        position
    }
}

pub struct Leaper;

impl AutonomousTauLeaper<State> for Leaper {
    fn step(y0: State, env: &Environment, p: &Params) -> State {
        fn next_poisson(rate: f64) -> u64 {
            use rand_distr::Distribution;
            println!("Rate = {}", rate);
            if rate != 0.0 {
                rand_distr::Poisson::new(rate)
                    .unwrap()
                    .sample(&mut rand::thread_rng())
            } else {
                0
            }
        }

        use std::cmp::min;

        let step_size = p.tau_sub_step;

        let beta_int = p.epi_params.beta_internal;
        let beta_ext = p.epi_params.beta_external;
        let sigma = p.epi_params.sigma;
        let gamma = p.epi_params.gamma;

        let s_to_e: u64 = {
            let within_group_rate =
                if y0.size() != 0 {
                    beta_int * y0.s() * y0.i() * step_size / y0.size() as f64
                } else {
                    0 as f64
                };
            let external_rate =
                if y0.size() != 0 {
                    let external_infectious = env.size() - y0.i();
                    beta_ext * y0.s() * external_infectious * step_size / env.size()
                } else {
                    0 as f64
                };

            min(
                next_poisson(within_group_rate + external_rate),
                y0.s,
            )
        };

        let e_to_i: u64 = min(
            next_poisson(sigma * y0.s() * step_size),
            y0.e,
        );

        let i_to_r: u64 = min(
            next_poisson(gamma * y0.i() * step_size),
            y0.i,
        );

        State {
            s: y0.s - s_to_e,
            e: y0.e + s_to_e - e_to_i,
            i: y0.i + e_to_i - i_to_r,
            r: y0.r + i_to_r,
        }
    }
}