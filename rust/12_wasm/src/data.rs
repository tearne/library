use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DataPoint {
    t: u32,
    x: u32,
    y: u32,
    v: u32,
}

impl DataPoint {
    pub fn parse_vec(s: String) -> Vec<DataPoint> {
        serde_json::from_str(&s).unwrap()
    }

    pub fn data_str() -> String {
        String::from(r#"
        {
            "meta" : {
                "name": "me",
                "date": "today"
            },
            "time_series" : [
                {
                    "t": 1,
                    "x": 10,
                    "y": 50,
                    "v": 1
                }
            ]
        }
        "#)
    }
}