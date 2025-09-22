use std::{error::Error, ffi::OsString, fmt::Display, fs, path::{self, Path, PathBuf}, process::{Command, ExitStatus, Stdio}, str::FromStr, string::FromUtf8Error};
use dialoguer::Confirm;
use json_pretty_compact::PrettyCompactFormatter;
use serde_json::{Serializer, Value};
use serde::Serialize;
use strum::{EnumIter, IntoEnumIterator};

pub fn install_vega() -> VegaResult<ExitStatus> {
    let here = fs::canonicalize(PathBuf::from("resources"))?;
    println!("here path: {:?}", here);
    let installer_path = here.join("installer.sh");
    println!("Installer path: {:?}", installer_path);

    let install_script = r#"
        #!/bin/env bash
        DOWNLOAD=https://github.com/vega/vl-convert/releases/download/v1.8.0/vl-convert-linux-x86.zip

        FILE=${DOWNLOAD##*/}
        TEMP=/tmp/vl-convert
        SHARE=~/.local/share/${FILE%.*}

        rm -rf $TEMP \
        && wget -P $TEMP/ $DOWNLOAD \
        && unzip -d $TEMP/inner $TEMP/$FILE \
        && unzip -d $SHARE/ $TEMP/inner/*.zip \
        && ln -s $SHARE/bin/vl-convert ~/.local/bin/vl-convert \
        && rm -rf $TEMP

        type vl-convert
    "#;

    Command::new("bash")
        .arg("-c")
        .arg(&install_script)
        .status()
        .map_err(|x|x.into())
}

#[derive(EnumIter, Debug)]
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "png" => Ok(VegaRenderer::PNG),
            "pdf" => Ok(VegaRenderer::PDF),
            "jpg" | "jpeg" => Ok(VegaRenderer::JPEG),
            "html" => Ok(VegaRenderer::HTML),
            "svg" => Ok(VegaRenderer::SVG),
            _ => {
                let options = VegaRenderer::iter()
                    .map(|x|format!("{:?}", x).to_lowercase())
                    .collect::<Vec<_>>()
                    .join(", ");
                Err(format!(
                    "Requested output format not recognised ({}).  Available options are: {}", 
                    s, 
                    options
                ))
            },
        }
    }
}


pub fn plot_from_value<P: AsRef<Path>>(value: serde_json::Value, render_path: P) -> VegaResult<()>{
    let json_str = {
        let mut pretty_buf = Vec::new();
        let formatter = PrettyCompactFormatter::new();
        let mut ser = Serializer::with_formatter(&mut pretty_buf, formatter);
        value.serialize(&mut ser)?;

        &String::from_utf8(pretty_buf).map_err(|e|VegaError::ValueToUtf8Error(e, value))?
    };
    
    plot_from_str(&json_str, render_path)
}

pub fn plot_from_str<P: AsRef<Path>>(json: &str, render_path: P) -> VegaResult<()>{
    log::debug!("Path {:?}", render_path.as_ref());
    let render_path = path::absolute(render_path.as_ref())?;
    let extension = render_path.extension().expect("Couldn't determine file extension for plot").to_string_lossy().into_owned();
    let json_path = render_path.with_extension("vg.json");
    let renderer: VegaRenderer = extension.parse().expect("Failed to parse file extention to a valid Vega renderer.");

    log::debug!("Saving Vega JSON to {:?}", &json_path);
    std::fs::write(&json_path, json).unwrap();

    match Command::new("vl-convert").stdout(Stdio::null()).stderr(Stdio::null()).status() {
        Err(e) if e.to_string().contains("No such file or directory") => {
            if Confirm::new()
                .with_prompt("vl-convert not found, install it in `~/.local`?")
                .default(false)
                .interact()
                .expect("Interaction error")
            {
                install_vega()?;
            } else {
                log::warn!("Option to install Vega was declined, but chart data json has been saved.");
                Err(VegaError::DeclinedInstall)?
            }
        },
        _ => (),
    }

    log::debug!("Rendering {:?} to {:?}", &json_path, render_path);
    let vega_result = Command::new("vl-convert")
        .args(&[
            renderer.command(), 
            "-i", &json_path.into_os_string().into_string()? , 
            "-o", &render_path.into_os_string().into_string()?
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


type VegaResult<T> = Result<T, VegaError>;

#[derive(Debug)]
pub enum VegaError {
    IOError(std::io::Error),
    DeclinedInstall,
    JsonSerialisationError(String),
    ValueToUtf8Error(FromUtf8Error, Value),
    OsStringError(OsString),
}
impl Display for VegaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "{}", err),
            Self::DeclinedInstall => write!(f, "Installation was declined, plotting cancelled."),
            Self::JsonSerialisationError(msg) => write!(f, "Error serialising JSON: {}", msg),
            Self::ValueToUtf8Error(err, value) => write!(f, "Error converting to Utf8 ({}) for Value : {}", err, value),
            Self::OsStringError(_) => write!(f, "Error converting OsString to String"),
        }
    }
}

impl Error for VegaError {}

impl From<std::io::Error> for VegaError {
    fn from(value: std::io::Error) -> Self {
        VegaError::IOError(value)
    }
}

impl From<serde_json::Error> for VegaError {
    fn from(value: serde_json::Error) -> Self {
        VegaError::JsonSerialisationError(format!("serde_json error: {}", value))
    }
}
impl From<OsString> for VegaError {
    fn from(value: OsString) -> Self {
        VegaError::OsStringError(value)
    }
}