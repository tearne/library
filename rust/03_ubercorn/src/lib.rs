pub mod error;
pub mod display;
pub mod keyboard;
pub mod art;
pub mod filesystem;

fn to_idx(x: usize, y: usize) -> usize {
    (16 - x) * 16 + (16 - y)
}

