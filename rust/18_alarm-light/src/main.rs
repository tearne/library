use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct Colour{
    label: String,
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let mut rdr = csv::Reader::from_path("resources/colours.csv").unwrap();
    for r in rdr.deserialize() {
        let r: Colour = r.unwrap();
        println!("{:?}", &r);
    }

    let mut rdr = csv::Reader::from_path("resources/pixels.csv").unwrap();
    for r in rdr.records() {
        let r = r.unwrap();
        let r: Vec<String> = r.iter()
            .map(|c|c.to_string())
            .collect();
        println!("{:?}", &r);
    }
}
