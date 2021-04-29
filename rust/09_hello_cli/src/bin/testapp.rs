use std::path::PathBuf;

use simple_log::LogConfigBuilder;
use structopt::StructOpt;
use serde::Deserialize;

/// Some funky help text
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Files to process
    #[structopt(name = "CONFIG FILE", parse(from_os_str))]
    config_file: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    num_reps: u64,
    log_file_name: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    println!("{:?}", opt);

    let string = std::fs::read_to_string(opt.config_file).unwrap();
    let config = toml::from_str::<Config>(&string).unwrap();

    let log_config = LogConfigBuilder::builder()
        .path(config.log_file_name.as_os_str().to_string_lossy())
        .size(1 * 100)
        .roll_count(10)
        .level("debug")
        .output_file()
        .output_console()
        .build();

    simple_log::new(log_config).unwrap();

    log::info!("You've asked for {} reps", config.num_reps);
    log::info!("About to say hello...");
    hello::say_hello();
    log::info!("...done");
}