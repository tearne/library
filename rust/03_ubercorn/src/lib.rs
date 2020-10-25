pub mod input_device;
pub mod error;
pub mod display;
pub mod pixel;
pub mod monitor;
pub mod zombie;
pub mod sheep;

fn to_idx(x: usize, y: usize) -> usize {
    (16 - x) * 16 + (16 - y)
}

