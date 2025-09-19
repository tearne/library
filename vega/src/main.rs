use std::{env, fs, path::{self, absolute, Path, PathBuf}, process::{Command, ExitStatus, Stdio}, str::FromStr};
use dialoguer::Confirm;
use eyre::{bail, Context, Result};
use serde_json::json;

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_ok() {
        println!("RUST_LOG not set, setting to 'info'");
        unsafe {
            env::set_var("RUST_LOG", "info");
        }
    }
    simple_logger::init_with_env().unwrap();

    let my_json = json!(
{
  "$schema": "https://vega.github.io/schema/vega/v6.json",
  "description": "A basic bar chart example, with value labels shown upon pointer hover.",
  "width": 400,
  "height": 200,
  "padding": 5,

  "data": [
    {
      "name": "table",
      "values": [
        {"category": "A", "amount": 28},
        {"category": "B", "amount": 55},
        {"category": "C", "amount": 43},
        {"category": "D", "amount": 91},
        {"category": "E", "amount": 81},
        {"category": "F", "amount": 53},
        {"category": "G", "amount": 19},
        {"category": "H", "amount": 87}
      ]
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

    vega_plot(
        &serde_json::to_string_pretty(&my_json).unwrap(), 
        Path::new("plot.jpeg")
    )?;

    Ok(())

}

fn install() -> Result<ExitStatus> {
    let here = fs::canonicalize(PathBuf::from("resources"))?;
    println!("here path: {:?}", here);
    let installer_path = here.join("installer.sh");
    println!("Installer path: {:?}", installer_path);

    Command::new(installer_path)
        .status()
        .wrap_err_with(||"failed to install")
}

enum VegaRender{
    PNG, PDF, JPEG, HTML, SVG
}
impl VegaRender {
    fn command(&self) -> &str {
        match self {
            VegaRender::PNG => "vg2png",
            VegaRender::PDF => "vg2pdf",
            VegaRender::JPEG => "vg2jpeg",
            VegaRender::HTML => "vg2html",
            VegaRender::SVG => "vg2svg",
        }
    }
}
impl FromStr for VegaRender {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "png" => Ok(VegaRender::PNG),
            "pdf" => Ok(VegaRender::PDF),
            "jpg" | "jpeg" => Ok(VegaRender::JPEG),
            "html" => Ok(VegaRender::HTML),
            "svg" => Ok(VegaRender::SVG),
            _ => Err(()),
        }
    }
}


fn vega_plot<P: AsRef<Path>>(json: &str, path: P) -> Result<()>{
    log::info!("Path {:?}", path.as_ref());
    let json_path = path::absolute(path.as_ref())?;
    let extension = json_path.extension().expect("Couldn't determine file extension for plot").to_string_lossy().into_owned();
    let render_path = json_path.with_extension(&extension);
    let renderer: VegaRender = extension.parse().expect("Failed to parse file extention to a valid Vega renderer.");

    log::info!("Saving Vega JSON to {:?}", &json_path);
    std::fs::write(&json_path, json).unwrap();

    match Command::new("vl-convert").stdout(Stdio::null()).stderr(Stdio::null()).status() {
        Err(e) if e.to_string().contains("No such file or directory") => {
            if Confirm::new()
                .with_prompt("Can't find vl-convert, install it in `~/.local`?")
                .default(false)
                .interact()
                .expect("Interaction error")
            {
                install()?;
            } else {
                bail!("Can't render, but chart data json has been saved.")
            }
        },
        _ => (),
    }

    log::info!("Rendering {:?} to {:?}", &json_path, render_path);
    let vega_result = Command::new("vl-convert")
        .args(&[
            renderer.command(), 
            "-i", json_path.to_str().unwrap(), 
            "-o", render_path.to_str().unwrap()
        ])
        .status();

    match vega_result {
        Err(e) => 
            log::warn!("Failed to execute plotting command, but json chart data saved.  Error was: {}", e),
        Ok(exit_status) if !exit_status.success() => 
            log::warn!("Plotting command failed to complete [{}], but json chart data saved.", exit_status),
        Ok(_) => ()
    };


    Ok(())
}
