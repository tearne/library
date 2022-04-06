use std::{path::Path, collections::HashMap};

use rgb::RGB8;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Colour{
    label: String,
    red: u8,
    green: u8,
    blue: u8
}
impl Colour {
    pub fn rgb8(&self) -> RGB8 {
        RGB8::new(self.red, self.green, self.blue)
    }
}

pub struct Image{
    pub px: Vec<RGB8>,
    pub num_rows: usize, 
}
impl Image {
    pub fn load<P: AsRef<Path>>(pixels: P, colours: P) -> Self {
        let colour_map: HashMap<String, RGB8> = csv::Reader::from_path(colours)
            .unwrap()
            .deserialize()
            .map(|r|{
                let c: Colour = r.unwrap();
                (c.label.clone(), c.rgb8())
            })
            .collect();

        let px: Vec<RGB8> = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(pixels)
            .unwrap()
            .records()
            .flat_map(|r|{ r.unwrap()
                .iter()
                .map(|s|*colour_map.get(s).unwrap())
                .collect::<Vec<RGB8>>()
            })
            .collect();

        assert!(px.len() % 16 == 0, "pixel array is ragged");

        let num_rows = px.len() / 16;

        Self {
            px,
            num_rows
        }
    }

    pub fn slice_of(&self, proportion: f64) -> &[RGB8] {
        assert!(proportion <= 1.0 && proportion >= 0.0);

        let start_row = (self.num_rows as f64 * proportion) as usize;
        let start = start_row * 16;

        &self.px[start..start+256]
    }
}