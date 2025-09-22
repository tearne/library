use std::path::Path;
use eyre::Result;
use rand_distr::{Distribution, Normal};
use serde_json::json;
use vega::init_logging;

fn main() -> Result<()> {
    init_logging();

    let mut rng = rand::rng();
    let normal = Normal::new(5.0, 3.0).unwrap();
    let samples: Vec<f64> = (1..=100).map(|_|normal.sample(&mut rng)).collect();

    let vega_json = json!(
{
  "$schema": "https://vega.github.io/schema/vega/v6.json",
  "background": "#ffffff",
  "width": 500,
  "height": 200,
  "padding": 5,

  "data": [
    {
      "name": "source",
      "values": samples,
    },
    {
      "name": "density",
      "source": "source",
      "transform": [
        {
          "type": "kde",
          "field": "data",
          "extent": [-5, 15],
          "bandwidth": 0.8,
          "steps": 200,
          "as": ["value", "density"]
        }
      ]
    }
  ],

  "scales": [
    {
      "name": "xscale",
      "type": "linear",
      "range": "width",
      "domain": {"data": "density", "field": "value"}
    },
    {
      "name": "yscale",
      "type": "linear",
      "range": "height",
      "domain": {"data": "density", "field": "density"},
      "nice": true,
      "zero": true
    }
  ],

  "axes": [
    {"orient": "bottom", "scale": "xscale", "title": "Value"},
    {"orient": "left", "scale": "yscale", "title": "Density"}
  ],

  "marks": [
    {
      "type": "line",
      "from": {"data": "density"},
      "encode": {
        "enter": {
          "x": {"scale": "xscale", "field": "value"},
          "y": {"scale": "yscale", "field": "density"},
          "stroke": {"value": "steelblue"},
          "strokeWidth": {"value": 2}
        }
      }
    }
  ]
}
    );

    vega::plot_from_value(
        vega_json, 
        Path::new("kde.png")
    )?;

    Ok(())
}