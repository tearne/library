pub mod vega;

use std::env;

pub use crate::vega::{plot_from_str, plot_from_value};

pub fn init_logging() {
    if env::var("RUST_LOG").is_ok() {
        println!("RUST_LOG not set, setting to 'info'");
        unsafe {
            env::set_var("RUST_LOG", "info");
        }
    }
    simple_logger::init_with_env().unwrap();
}