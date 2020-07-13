mod leap;

fn main() {
    use rand_distr::{Poisson, Distribution};

    let v: u64 = Poisson::new(5.0).unwrap().sample(&mut rand::thread_rng());

    let _init = leap::SIC{ s:0, i:0, c:0 };

    println!("Hello, world! {}", v);
}