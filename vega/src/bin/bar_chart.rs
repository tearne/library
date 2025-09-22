use std::{env, path::Path};
use eyre::Result;
use serde::Serialize;
use serde_json::json;

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_ok() {
        println!("RUST_LOG not set, setting to 'info'");
        unsafe {
            env::set_var("RUST_LOG", "info");
        }
    }
    simple_logger::init_with_env().unwrap();

    #[derive(Serialize)]
    struct Row{
        category: String,
        amount: usize,
    }
    impl Row{
        pub fn new(category: &str, amount: usize) -> Self {
            Row{category: category.into(), amount}
        }
    }
    let my_data = vec![
        Row::new("A",28),
        Row::new("B",55),
        Row::new("C",43),
        Row::new("D",91),
        Row::new("E",81),
        Row::new("F",53),
        Row::new("G",19),
        Row::new("H",78),
        Row::new("I",13),
        Row::new("J",64),
        Row::new("K",29),
    ];

    let vega_json = json!(
{
  "$schema": "https://vega.github.io/schema/vega/v6.json",
  "background": "#ffffff",
  "description": "A basic bar chart example, with value labels shown upon pointer hover.",
  "width": 400,
  "height": 200,
  "padding": 5,

  "data": [
    {
      "name": "table",
      "values": my_data
    }
  ],

  "signals": [
    {
      "name": "tooltip",
      "value": {},
      "on": [
        {"events": "rect:pointerover", "update": "datum"},
        {"events": "rect:pointerout",  "update": "{}"}
      ]
    }
  ],

  "scales": [
    {
      "name": "xscale",
      "type": "band",
      "domain": {"data": "table", "field": "category"},
      "range": "width",
      "padding": 0.05,
      "round": true
    },
    {
      "name": "yscale",
      "domain": {"data": "table", "field": "amount"},
      "nice": true,
      "range": "height"
    }
  ],

  "axes": [
    { "orient": "bottom", "scale": "xscale" },
    { "orient": "left", "scale": "yscale" }
  ],

  "marks": [
    {
      "type": "rect",
      "from": {"data":"table"},
      "encode": {
        "enter": {
          "x": {"scale": "xscale", "field": "category"},
          "width": {"scale": "xscale", "band": 1},
          "y": {"scale": "yscale", "field": "amount"},
          "y2": {"scale": "yscale", "value": 0}
        },
        "update": {
          "fill": {"value": "steelblue"}
        },
        "hover": {
          "fill": {"value": "red"}
        }
      }
    },
    {
      "type": "text",
      "encode": {
        "enter": {
          "align": {"value": "center"},
          "baseline": {"value": "bottom"},
          "fill": {"value": "#333"}
        },
        "update": {
          "x": {"scale": "xscale", "signal": "tooltip.category", "band": 0.5},
          "y": {"scale": "yscale", "signal": "tooltip.amount", "offset": -2},
          "text": {"signal": "tooltip.amount"},
          "fillOpacity": [
            {"test": "datum === tooltip", "value": 0},
            {"value": 1}
          ]
        }
      }
    }
  ]
}
    );

    vega::plot_from_value(
        vega_json, 
        Path::new("bar_chart.jpg")
    )?;

    Ok(())
}