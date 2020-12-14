// use std::{collections::HashMap, fs::File, io::BufReader};
// use serde::Deserialize;
// use serde_json;
// use ubercorn;

// #[derive(Deserialize, Debug, Clone)]
// struct RGB {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
// }

// #[derive(Deserialize, Debug)]
// struct Layer {
//     colour: String,
//     points: Vec<(usize, usize)>,
// }

// #[derive(Deserialize, Debug)]
// struct Graphic {
//     name: String,
//     colours: HashMap<String, RGB>,
//     groups: Vec<Layer>,
// }
// impl Graphic {
//     pub fn as_pixels(&self) -> Vec<RGB> {
//         let black = RGB { r: 0, g: 0, b: 0 };
//         let mut pixels = vec![black; 256];

//         self.groups.iter().for_each(|layer|{
//             let colour = self.colours.get(&(layer.colour)).unwrap();
//             for (x,y) in layer.points.iter() {
//                 pixels[ubercorn::to_idx(*x,*y)] = colour.clone();
//             }
//         });

//         pixels
//     }
// }

// // fn print_type_of<T>(_: &T) {
// //     println!("{}", std::any::type_name::<T>())
// // }

// fn main() {
//     let file = File::open("resources/zombie.json").unwrap();
//     let reader = BufReader::new(file);

//     let g: Graphic = serde_json::from_reader(reader).unwrap();

//     println!("{:?}", g.as_pixels());
// }