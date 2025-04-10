use std::{fmt::Debug, pin::Pin};

fn main() {
    let config_pet = "cat";
    let config_os = "linux";
    let config_topping = "pepperoni";
    let config_ice_cream = "chocolate";

    trait Pet {
        fn new() -> Self;
    }
    #[derive(Debug)]
    struct Cat();
    #[derive(Debug)]
    struct Dog();
    #[derive(Debug)]
    struct Rat();
    impl Pet for Cat { fn new() -> Self { Cat() } }
    impl Pet for Dog { fn new() -> Self { Dog() } }
    impl Pet for Rat { fn new() -> Self { Rat() } }

    trait OS {
        fn new() -> Self;
    }
    #[derive(Debug)]
    struct Linux();
    #[derive(Debug)]
    struct Mac();
    #[derive(Debug)]
    struct Windows();
    impl OS for Linux { fn new() -> Self { Linux() } }
    impl OS for Mac { fn new() -> Self { Mac() } }
    impl OS for Windows { fn new() -> Self { Windows() } }

    trait Topping {
        fn new() -> Self;
    }
    #[derive(Debug)]
    struct Pepperoni();
    #[derive(Debug)]
    struct Pineapple();
    #[derive(Debug)]
    struct Cheese();
    impl Topping for Pepperoni { fn new() -> Self { Pepperoni() } }
    impl Topping for Pineapple { fn new() -> Self { Pineapple() } }
    impl Topping for Cheese { fn new() -> Self { Cheese() } }

    trait IceCream {
        fn new() -> Self;
    }
    #[derive(Debug)]
    struct Chocolate();
    #[derive(Debug)]
    struct Vanilla();
    impl IceCream for Chocolate { fn new() -> Self { Chocolate() } }
    impl IceCream for Vanilla { fn new() -> Self { Vanilla() } }

    #[derive(Debug)]
    struct World<P, O, T, I>
    where
        P: Pet,
        O: OS,
        T: Topping,
        I: IceCream,
    {
        pet: P,
        os: O,
        top: T,
        ice: I,
    };

    impl<P, O, T, I> World<P, O, T, I>
    where
        P: Pet + Debug,
        O: OS + Debug,
        T: Topping + Debug,
        I: IceCream + Debug,
    {
        pub fn new() -> Self {
            Self {
                pet: P::new(),
                os: O::new(),
                top: T::new(),
                ice: I::new()
            }
        }
        pub fn run(&self) {
            println!("{:?}", &self)
        }
    }

    let my_config = ["cat", "linux", "pepperoni", "chocolate"];

    match my_config {
        ["cat", "linux", "pepperoni", "chocolate"] => 
            World::<Cat, Linux, Pepperoni, Chocolate>::new().run(),
        ["dog", "linux", "cheese", "chocolate"] => 
            World::<Dog, Linux, Cheese, Chocolate>::new().run(),
        [.., "pineapple", _] => unimplemented!("Pineapple is illegal"),
        [..] => unimplemented!("You have to guess the acceptable combinations"),
    };
}
