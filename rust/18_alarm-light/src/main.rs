mod image;
mod rotation;

use std::time::Duration;

use image::Image;
use rotation::{Matrix, Rotation};
use unicorn::pimoroni::unicorn::Unicorn;

fn main() {
    let who = "Anna";
    let colours_path = format!("resources/{}/colours.csv", &who);
    let pixels_path = format!("resources/{}/pixels.csv", &who);
    let image = Image::load(pixels_path, colours_path);

    let mut display = Unicorn::new();
    
    let rot = Matrix::new(Rotation::QUARTER_COUNTER); 

    for i in 0..1000 {
        let grid = image.slice_of(i as f64 /100.0);
        let grid = rot.apply(grid);
        for (idx, colour) in grid.iter().enumerate() {
            // if idx == 256 { break; }
            display.set_idx(idx, colour )
        }
        display.flush();

        std::thread::sleep(Duration::from_millis(200));
    }
}
