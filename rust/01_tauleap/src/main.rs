mod leap;

fn main() {
    use rand_distr::{Poisson, Distribution};

    let v: u64 = Poisson::new(5.0).unwrap().sample(&mut rand::thread_rng());

    let init = leap::SIR{ s:0, i:0, r:0 };

    println!("Hello, world! {}", v);
}
