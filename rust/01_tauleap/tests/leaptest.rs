extern crate tauleap;

use tauleap::epi::*;
use tauleap::leap::*;

#[test]
fn my_test() {
    let epi_params = EpiParams::default();
    let params = params::Params::new(1.0, 5, epi_params);

    println!("{:?}", params);

    let init = state::State::start();

    let dummy_env = environment::Environment{
        size: 1000,
        infectious: 5
    };
    let evolved = Leaper::leap(init, &dummy_env, &params);

    let serialized = serde_json::to_string(&evolved).unwrap();
    println!("serialized = {}", serialized);

    println!("{:?}", init);
    println!("{:?}", evolved);

    use std::{thread, time};
    let ten_millis = time::Duration::from_millis(100);
    thread::sleep(ten_millis);

    // assert!(false);
}
