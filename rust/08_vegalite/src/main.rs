use std::path::PathBuf;

use serde_json::json;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    let out_path = PathBuf::from("");

    let my_json = json!({
        "$schema": "https://vega.github.io/schema/vega-lite/v4.json",
        "description": "A simple bar chart with embedded data.",
        "data": {
            "values": [
            {"a": "A", "b": 28}, {"a": "B", "b": 55}, {"a": "C", "b": 43},
            {"a": "D", "b": 91}, {"a": "E", "b": 81}, {"a": "F", "b": 53},
            {"a": "G", "b": 19}, {"a": "H", "b": 87}, {"a": "I", "b": 52}
            ]
        },
        "mark": "bar",
        "encoding": {
            "x": {"field": "a", "type": "nominal", "axis": {"labelAngle": 0}},
            "y": {"field": "b", "type": "quantitative"}
        }
    });
    
    let json_file = {
        let mut p = out_path.clone();
        p.push("plot.json");
        p
    };
    std::fs::write(json_file.as_path(), serde_json::to_string_pretty(&my_json).unwrap()).unwrap();
    
    
    let json_path_str = json_file.to_str().unwrap();
    let png_path_str = {
        let mut p = json_file.clone();
        p.pop();
        p.push("plot.png");
        p.to_str().unwrap().to_owned()
    };
    let vega_result = std::process::Command::new("vl2png")
        .args(&[json_path_str, &png_path_str])
        .status();

    match vega_result {
        Err(e) => 
            log::warn!("Failed to run Vega-Lite (vl2png), but json chart data saved.  Error was: {}", e),
        Ok(exit_status) if !exit_status.success() => 
            log::warn!("Failed to run Vega-Lite (vl2png) [{}], but json chart data saved.", exit_status),
        Ok(_) => ()
    };
}
