use std::ffi::CStr;
use std::os::raw::c_int;

// https://github.com/pi-pi3/julia-rs
// https://github.com/Taaitaaiger/jlrs

fn main() {
    unsafe {
        jffi::jl_init__threading();
        assert!(jffi::jl_is_initialized() != 0);

        // let mut num = 5;
        // let r1 = &num as *const i32;
        // let r2 = &mut num as *mut i32;

        // https://doc.rust-lang.org/reference/tokens.html#raw-byte-string-literals
        // https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/reference.html#raw-byte-string-literals
        // let bytes = b"print(\"Hello, world!\")\n\0";
        // let string = CStr::from_bytes_with_nul(bytes).unwrap();
        // jffi::jl_eval_string(string.as_ptr());

        // let bytes = b"1 + 7\0";
        // let string = CStr::from_bytes_with_nul(bytes).unwrap();
        // // jffi::jl_eval_string(string.as_ptr());
        // let jvalue: *mut jffi::jl_value_t = jffi::jl_eval_string(string.as_ptr());
        // println!("--> {:?}", *(jvalue as *mut c_int) as u64);

        // let bytes = b"""print(1 + 2)\0""";
        // let string = CStr::from_bytes_with_nul(bytes).unwrap();
        // jffi::jl_eval_string(string.as_ptr());

        let bytes = b"include(\"function.jl\")\0";
        let string = CStr::from_bytes_with_nul(bytes).unwrap();
        jffi::jl_eval_string(string.as_ptr());
        let bytes = b"add_one_hundred(3)\0";
        let string = CStr::from_bytes_with_nul(bytes).unwrap();
        let jvalue: *mut jffi::jl_value_t = jffi::jl_eval_string(string.as_ptr());
        println!("--> {:?}", *(jvalue as *mut c_int) as u64);
    };
}
