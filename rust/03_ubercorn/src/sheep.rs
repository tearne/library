use rgb::RGB8;
use std::collections::HashSet;

pub fn get() -> Vec<RGB8> {
    const BLACK: RGB8 = RGB8 { r: 0, g: 0, b: 0 };
    let mut pixels = vec![BLACK; 256];

    fn add_pixels_with_colour(colour: RGB8, to_add: HashSet<(usize, usize)>, pixels: &mut Vec<RGB8>) {
        to_add.iter().for_each(|(x,y)| {
            pixels[crate::to_idx(*x,*y)] = colour;
        });
    }

    let blue = RGB8::new(100,150,255);
    add_pixels_with_colour(blue, sky_px(), &mut pixels);

    let pink = RGB8::new(250,130,130);
    add_pixels_with_colour(pink, big_pink_arch_px(), &mut pixels);
    add_pixels_with_colour(pink, small_pink_arch_px(), &mut pixels);

    let orange = RGB8::new(255,160,0);
    add_pixels_with_colour(orange, small_orange_arch_px(), &mut pixels);
    add_pixels_with_colour(orange, sheep_legs_px(), &mut pixels);

    let red = RGB8::new(255,70,70);
    add_pixels_with_colour(red, heart_px(), &mut pixels);

    let brown = RGB8::new(150,75,0);
    add_pixels_with_colour(brown, poo_px(), &mut pixels);

    let grey = RGB8::new(180,180,180);
    add_pixels_with_colour(grey, sheep_body_px(), &mut pixels);

    pixels
}

fn sky_px() -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    set.insert((1,16));
    set.insert((2,16));
    set.insert((3,16));
    set.insert((5,16));
    set.insert((4,16));
    set.insert((6,16));
    set.insert((7,16));
    set.insert((8,16));
    set.insert((9,16));
    set.insert((10,16));
    set.insert((11,16));
    set.insert((12,16));
    set.insert((13,16));
    set.insert((14,16));
    set.insert((15,16));
    set.insert((16,16));
    set
}

fn big_pink_arch_px() -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    set.insert((1,1));
    set.insert((1,2));
    set.insert((1,3));
    set.insert((1,4));
    set.insert((1,5));
    set.insert((1,6));
    set.insert((1,7));
    set.insert((1,8));
    set.insert((1,9));
    set.insert((1,10));
    set.insert((1,11));
    set.insert((1,12));
    set.insert((1,13));
    set.insert((1,14));
    set.insert((1,15));
    set.insert((1,15));
    set.insert((2,15));
    set.insert((3,15));
    set.insert((4,15));
    set.insert((5,15));
    set.insert((6,15));
    set.insert((7,15));
    set.insert((8,15));
    set.insert((9,15));
    set.insert((10,15));
    set.insert((11,15));
    set.insert((12,15));
    set.insert((13,15));
    set.insert((14,15));
    set.insert((15,15));
    set.insert((16,15));
    set.insert((16,1));
    set.insert((16,2));
    set.insert((16,3));
    set.insert((16,4));
    set.insert((16,5));
    set.insert((16,6));
    set.insert((16,7));
    set.insert((16,8));
    set.insert((16,9));
    set.insert((16,10));
    set.insert((16,11));
    set.insert((16,12));
    set.insert((16,13));
    set.insert((16,14));
    set.insert((16,15));
    set
}

fn small_orange_arch_px() -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    set.insert((2,1));
    set.insert((2,2));
    set.insert((2,3));
    set.insert((2,4));
    set.insert((2,5));
    set.insert((2,6));
    set.insert((2,7));
    set.insert((2,8));
    set.insert((2,9));
    set.insert((2,10));
    set.insert((2,11));
    set.insert((2,12));
    set.insert((2,13));
    set.insert((2,14));
    set.insert((3,14));
    set.insert((4,14));
    set.insert((5,14));
    set.insert((6,14));
    set.insert((7,14));
    set.insert((8,14));
    set.insert((9,14));
    set.insert((10,14));
    set.insert((11,14));
    set.insert((12,14));
    set.insert((13,14));
    set.insert((14,14));
    set.insert((15,14));
    set.insert((15,13));
    set.insert((15,12));
    set.insert((15,11));
    set.insert((15,10));
    set.insert((15,9));
    set.insert((15,8));
    set.insert((15,7));
    set.insert((15,6));
    set.insert((15,5));
    set.insert((15,4));
    set.insert((15,3));
    set.insert((15,2));
    set.insert((15,1));
    
    set
}

fn small_pink_arch_px() -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    set.insert((3,13));
    set.insert((3,12));
    set.insert((3,11));
    set.insert((3,10));
    set.insert((3,9));
    set.insert((3,8));
    set.insert((3,7));
    set.insert((3,6));
    set.insert((3,5));
    set.insert((3,4));
    set.insert((3,3));
    set.insert((3,2));
    set.insert((3,1));
    set.insert((4,13));
    set.insert((5,13));
    set.insert((6,13));
    set.insert((7,13));
    set.insert((8,13));
    set.insert((9,13));
    set.insert((10,13));
    set.insert((11,13));
    set.insert((12,13));
    set.insert((13,13));
    set.insert((14,13));
    set.insert((14,12));
    set.insert((14,11));
    set.insert((14,10));
    set.insert((14,9));
    set.insert((14,8));
    set.insert((14,7));
    set.insert((14,6));
    set.insert((14,5));
    set.insert((14,4));
    set.insert((14,3));
    set.insert((14,2));
    set.insert((14,1));
    set
}

fn heart_px() -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    set.insert((11,12));
    set.insert((8,12));

    set.insert((12,11));
    set.insert((11,11));
    set.insert((10,11));
    set.insert((9,11));
    set.insert((7,11));
    set.insert((8,11));

    set.insert((13,10));
    set.insert((12,10));
    set.insert((11,10));
    set.insert((10,10));
    set.insert((9,10));
    set.insert((8,10));
    set.insert((7,10));
    set.insert((6,10));

    set.insert((11,9));
    set.insert((10,9));
    set.insert((9,9));
    set.insert((8,9));
    set.insert((7,9));

    set.insert((10,8));
    set.insert((9,8));
    set.insert((8,8));

    set.insert((9,7));

    set.insert((13,7)); //Sheep eye
    set
}

fn poo_px() -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    set.insert((4,1));
    set.insert((5,1));
    set.insert((6,1));
    set.insert((6,4));
    set.insert((5,2));
    
    set
}

fn sheep_legs_px() -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    set.insert((7,1));
    set.insert((11,1));
    set.insert((7,2));
    set.insert((11,2));
    
    set
}

fn sheep_body_px() -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    set.insert((7,3));
    set.insert((7,4));
    set.insert((7,5));
    set.insert((8,3));
    set.insert((8,4));
    set.insert((8,5));
    set.insert((9,3));
    set.insert((9,4));
    set.insert((9,5));
    set.insert((10,3));
    set.insert((10,4));
    set.insert((10,5));
    set.insert((11,3));
    set.insert((11,4));
    set.insert((11,5));
    set.insert((11,7));
    set.insert((12,5));
    set.insert((12,6));
    set.insert((12,7));
    set.insert((13,6));

    set.insert((6,7));
    set.insert((6,6));
    set.insert((6,5));
    
    set
}