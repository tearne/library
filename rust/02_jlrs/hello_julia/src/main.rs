// fn main() {
//     println!("Hello, world!");
// }

use jlrs::prelude::*;

fn main() {
    let mut julia = unsafe { Julia::init(16).unwrap() };
    let result =julia.dynamic_frame(|global, frame| {
        // Create the two arguments
        let i = Value::new(frame, 2u64)?;
        let j = Value::new(frame, 1u32)?;

        // We can find the addition-function in the base module
        let func = Module::base(global).function("+")?;

        // Call the function and unbox the result
        let output = func.call2(frame, i, j)?.unwrap();
        output.cast::<u64>()
    }).unwrap();

    println!("Result is: {}", result);
}