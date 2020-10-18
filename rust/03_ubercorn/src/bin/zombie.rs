use rgb::RGB8;
use std::{collections::HashSet, thread, time::Duration};

use ubercorn::{display::Display};

fn main() {
    const BLACK: RGB8 = RGB8 { r: 0, g: 0, b: 0 };
    let mut pixels = vec![BLACK; 256];

    fn add_pixels_with_colour(colour: RGB8, to_add: HashSet<(usize, usize)>, pixels: &mut Vec<RGB8>) {
        to_add.iter().for_each(|(x,y)| {
            pixels[to_idx(*x,*y)] = colour;
        });
    }

    let dark_green = RGB8::new(50,90,10);
    add_pixels_with_colour(dark_green, dark_green_px(), &mut pixels);

    let light_green = RGB8::new(10,230,30);
    add_pixels_with_colour(light_green, light_green_px(), &mut pixels);
    
    let blue = RGB8::new(60,120,160);
    add_pixels_with_colour(blue, blue_shirt_px(), &mut pixels);

    let white = RGB8::new(255,255,255);
    add_pixels_with_colour(white, eyes_px(), &mut pixels);

    let mut display = Display::build();
    display.apply(&pixels);


    thread::sleep(Duration::from_millis(100000));
}

fn to_idx(x: usize, y: usize) -> usize {
    (x - 1) * 16 + y - 1
}

fn dark_green_px() -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    set.insert((5,12));
    set.insert((6,12));
    set.insert((7,12));
    set.insert((8,12));
    set.insert((9,12));
    set.insert((10,12));
    set.insert((11,12));
    set.insert((12,12));
    set.insert((6,11));
    set.insert((7,11));
    set.insert((8,11));
    set.insert((12,11));
    set.insert((12,5));
    set.insert((12,6));
    set.insert((12,7));
    set.insert((10,5));
    set.insert((10,6));
    set.insert((7,6));
    set.insert((5,5));
    set.insert((6,5));
    set.insert((5,10));
    set.insert((7,6));
    set.insert((8,5));
    set.insert((9,7));
    set.insert((7,5));
    set.insert((8,7));
    set.insert((5,11));

    set.insert((7,4));
    set.insert((8,4));
    set.insert((9,4));
    set.insert((10,4));

    set.insert((8,3));
    set.insert((9,3));

    set
}

fn light_green_px() -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    set.insert((9,6));
    set.insert((9,5));
    set.insert((8,6));
    set.insert((8,9));
    set.insert((11,6));
    set.insert((11,5));
    set.insert((11,7));
    set.insert((10,7));
    set.insert((9,9));
    set.insert((10,9));
    set.insert((11,9));
    set.insert((12,9));
    set.insert((5,9));
    set.insert((12,9));
    set.insert((11,9));
    set.insert((10,9));
    set.insert((9,9));
    set.insert((8,9));
    set.insert((7,9));
    set.insert((6,9));
    set.insert((11,11));
    set.insert((10,11));
    set.insert((9,11));
    set.insert((8,8));
    set.insert((8,9));
    set.insert((5,6));
    set.insert((5,7));
    set.insert((5,8));
    set.insert((7,7));
    set.insert((6,7));
    set.insert((6,6));
    set.insert((12,8));
    set.insert((9,8));
    set.insert((6,10));
    set.insert((7,10));
    set.insert((8,10)); 
    set.insert((9,10));
    set.insert((10,10));
    set.insert((11,10));
    set.insert((12,10));
    set.insert((3,1));
    set.insert((3,2));
    set.insert((4,2));
    set.insert((4,1));
    set.insert((13,1));
    set.insert((13,2));
    set.insert((14,1));
    set.insert((14,2));

    set
}

fn eyes_px() -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    set.insert((11,8));
    set.insert((10,8));
    set.insert((7,8));
    set.insert((6,8));
    set
}

fn blue_shirt_px() -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    set.insert((3,4));
    set.insert((4,4));
    set.insert((5,4));
    set.insert((6,4));
    set.insert((11,4));
    set.insert((12,4));
    set.insert((13,4));
    set.insert((14,4));
    set.insert((3,4));
    set.insert((4,4));
    set.insert((5,4));
    set.insert((6,4));
    set.insert((11,4));
    set.insert((12,4));
    set.insert((13,4));
    set.insert((14,4));

    set.insert((3,3));
    set.insert((4,3));
    set.insert((5,3));
    set.insert((6,3));
    set.insert((7,3));
    set.insert((10,3));
    set.insert((11,3));
    set.insert((12,3));
    set.insert((13,3));
    set.insert((14,3));

    set.insert((5,2));
    set.insert((6,2));
    set.insert((7,2));
    set.insert((8,2));
    set.insert((9,2));
    set.insert((10,2));
    set.insert((11,2));
    set.insert((12,2));    
    
    set.insert((5,1));
    set.insert((6,1));
    set.insert((7,1));
    set.insert((8,1));
    set.insert((9,1));
    set.insert((10,1));
    set.insert((11,1));
    set.insert((12,1));

    set
}