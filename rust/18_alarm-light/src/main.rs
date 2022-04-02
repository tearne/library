use std::{collections::HashMap, time::Duration};

use rgb::RGB8;
use serde::Deserialize;
use unicorn::pimoroni::unicorn::Unicorn;


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

fn main() {
    let mut rdr = csv::Reader::from_path("resources/colours.csv").unwrap();
    let colour_map: HashMap<String, RGB8> = rdr.deserialize()
        .map(|r|{
            let c: Colour = r.unwrap();
            (c.label.clone(), c.rgb8())
        })
        .collect();
    
    for r in rdr.deserialize() {
        let r: Colour = r.unwrap();
        println!("{:?}", &r);
    }

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("resources/pixels.csv").unwrap();
    let px: Vec<RGB8> = rdr.records()
        .flat_map(|r|{
            r.unwrap()
                .iter()
                .map(|s|*colour_map.get(s).unwrap())
                .collect::<Vec<RGB8>>()
        })
        .collect();
    
    println!("px = {:#?}", &px);

    let mut display = Unicorn::new();
    for (idx, colour) in px.iter().enumerate() {
        if idx == 256 { break; }
        display.set_idx(idx, colour )
    }
    display.flush();

    std::thread::sleep(Duration::from_secs(100));
}
