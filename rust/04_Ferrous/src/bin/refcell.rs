use std::cell::{RefCell, RefMut};

fn main() {
    // Exploring RefCell with some examples.
    // First we need a helper method.
    trait Doubler {
        fn double_me(&mut self);
    }
    impl Doubler for i32 {
        fn double_me(&mut self) {
            *self = *self * 2;
        }
    }

    // Make a RefCell to play with
    let my_ref_cell = RefCell::new(2);

    // We can call `.double_me()` suggesting that `.borrow_mut()` behaves like `&mut`.
    my_ref_cell.borrow_mut().double_me();
    println!("1: {:?}", my_ref_cell); // RefCell { value: 4 }, as expected

    // But by contrast to `&mut`, if I bind RefMut to a variable I must make it mutable
    {
        let mut my_ref = my_ref_cell.borrow_mut();
        my_ref.double_me(); // Compile error here unless `my_ref` is mutable
    }
    println!("2: {:?}", my_ref_cell); // RefCell { value: 8 }, as expected

    // Verify that this isn't the case for `&mut`.
    let mut int = 10;
    let immutable_binding: &mut i32 = &mut int;
    //  ^----- immutable binding to `&mut`
    immutable_binding.double_me();
    println!("3: {:?}", int); // 20, as expected

    // Questions so far
    // 1. What exactly is RefMut? Sometimes it seems like runtime checked version of `&mut`, but
    //    other times not.
    // 2. How do the docs explain this to me?  Right now it just seems like magic

    // Lets Do some more experiments using this helper fn
    fn double_me(num: &mut i32){
        *num = *num * 2;
    }

    // This won't compile, it's expecting an &mut i32 but got a RefMut<i32> ?!
    // double_me(number_ref_cell.borrow_mut());

    // This will compile,seems a little odd though
    double_me(&mut *my_ref_cell.borrow_mut());
    println!("{:?}", my_ref_cell); // RefCell { value: 16 }, as expected
}

