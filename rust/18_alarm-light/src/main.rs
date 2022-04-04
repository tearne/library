mod image;
mod rotation;

use std::time::Duration;

use image::Image;
use rotation::{Matrix, Rotation};
use unicorn::pimoroni::unicorn::Unicorn;

fn main() {
    let colours_path = "resources/colours.csv";
    let pixels_path = "resources/pixels.csv";
    let image = Image::load(pixels_path, colours_path);

    let mut display = Unicorn::new();
    
    let rot = Matrix::new(Rotation::QUARTER_COUNTER); 

    for i in 0..100 {
        let grid = image.slice_of(i as f64 /100.0);
        let grid = rot.apply(grid);
        for (idx, colour) in grid.iter().enumerate() {
            if idx == 256 { break; }
            display.set_idx(idx, colour )
        }
        display.flush();

        std::thread::sleep(Duration::from_secs(1));
    }
}
