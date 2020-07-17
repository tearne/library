extern crate tauleap;
use tauleap::epi;
use tauleap::test;

pub fn main() {
    test();

    use rand_distr::{Distribution, Poisson};

    let v: u64 = Poisson::new(5.0).unwrap().sample(&mut rand::thread_rng());

    let _init = epi::state::State::start();

    println!("Hello, world! {}", v);
}
