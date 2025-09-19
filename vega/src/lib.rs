use std::{fs, path::{self, Path, PathBuf}, process::{Command, ExitStatus, Stdio}, str::FromStr};
use dialoguer::Confirm;
use eyre::{bail, Context, Result};

pub fn install_vega() -> Result<ExitStatus> {
    let here = fs::canonicalize(PathBuf::from("resources"))?;
    println!("here path: {:?}", here);
    let installer_path = here.join("installer.sh");
    println!("Installer path: {:?}", installer_path);

    Command::new(installer_path)
        .status()
        .wrap_err_with(||"failed to install")
}

pub enum VegaRenderer{
    PNG, PDF, JPEG, HTML, SVG
}
impl VegaRenderer {
    fn command(&self) -> &str {
        match self {
            VegaRenderer::PNG => "vg2png",
            VegaRenderer::PDF => "vg2pdf",
            VegaRenderer::JPEG => "vg2jpeg",
            VegaRenderer::HTML => "vg2html",
            VegaRenderer::SVG => "vg2svg",
        }
    }
}
impl FromStr for VegaRenderer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "png" => Ok(VegaRenderer::PNG),
            "pdf" => Ok(VegaRenderer::PDF),
            "jpg" | "jpeg" => Ok(VegaRenderer::JPEG),
            "html" => Ok(VegaRenderer::HTML),
            "svg" => Ok(VegaRenderer::SVG),
            _ => Err(()),
        }
    }
}


pub fn vega_plot<P: AsRef<Path>>(json: &str, render_path: P) -> Result<()>{
    // log::info!("Path {:?}", render_path.as_ref());
    let render_path = path::absolute(render_path.as_ref())?;
    let extension = render_path.extension().expect("Couldn't determine file extension for plot").to_string_lossy().into_owned();
    let json_path = render_path.with_extension("vg.json");
    let renderer: VegaRenderer = extension.parse().expect("Failed to parse file extention to a valid Vega renderer.");

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
                install_vega()?;
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
