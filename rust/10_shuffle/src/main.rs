use std::collections::VecDeque;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let items:usize = 10;
    
    let mut times_at_front = vec![0usize; items];
    let trials = 100000;
    for _ in 0..trials {
        let value = trial(items)[0];
        times_at_front[value] = times_at_front[value] + 1;
    }

    println!("{:?}", times_at_front.iter().map(|v|(*v as f64) /(trials as f64)).collect::<Vec<f64>>());
}

fn trial(items: usize) -> VecDeque<usize> {
    let mut buf = VecDeque::new();
    for i in 0..items{
        buf.push_back(i);
    }

    // println!("{:?}", buf);
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..buf.len());

    for _ in 0..buf.len(){
        let idx = die.sample(&mut rng);
        let moving = buf.remove(idx).unwrap();
        buf.push_back(moving);
        // println!("move idx {}", idx);
        // println!("{:?}", buf);
    }

    buf
}