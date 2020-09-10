
#![allow(dead_code)]

// https://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-a-struct-in-rust
use std::cell::RefCell;

pub struct Cached
{
    calculation: Box<dyn Fn() -> f32>,
    value: RefCell<Option<f32>>
}
impl Cached {
    pub fn new(calc: impl Fn() -> f32 + 'static) -> Self {
        Self {
            calculation: Box::new(calc), 
            value: RefCell::new(None)
        }
    }

    pub fn get(&mut self) -> f32 {
        match *(self.value.borrow()) {
            Some(v) => v,
            None => {
                let v = (self.calculation)();
                self.value.replace(Some(v));
                v
            }
        }
    }
}