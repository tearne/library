fn main() {
    use rand_distr::{Poisson, Distribution};

    let v: u64 = Poisson::new(5.0).unwrap().sample(&mut rand::thread_rng());

    println!("Hello, world! {}", v);
}
