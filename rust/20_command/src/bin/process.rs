use std::{env, ffi::OsStr, io, ops::DerefMut, path::Path, process::Command, sync::{Arc, Mutex}};

use chrono::{DateTime, Local};
use clap::Parser;
use log::LevelFilter;
use sysinfo::{Pid, Process, ProcessRefreshKind, ProcessesToUpdate, System};
use color_eyre::eyre::Result;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Verbose mode (three levels: -v, -vv, -vvv)
    #[structopt(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// CPU polling interval (seconds)
    #[structopt(short, long, default_value = "1")]
    interval: u64,

    /// Command to run
    #[arg(last = true, required = true)]
    command: Vec<String>,

    /// Output file to save data to
    #[structopt(short, long, default_value = "out.csv")]
    out_file: String,
}

pub fn setup_logging(level: u8) {
    fn set_log_level(local_level: LevelFilter, dep_level:  LevelFilter) {
        let prog: String = std::env::current_exe()
            .unwrap()
            .file_name().unwrap()
            .to_str().unwrap()
            .to_owned();

        env_logger::builder()
            .filter_level(dep_level)
            .filter(Some(&prog), local_level)
            .init();
        log::info!("Program name detected: {}", &prog);
        log::info!("Local log level set to {}", local_level);
        log::info!("Default Log level set to {}", dep_level);
    }

    match level {
        0 => set_log_level(LevelFilter::Warn, LevelFilter::Warn),
        1 => set_log_level(LevelFilter::Info, LevelFilter::Warn),
        2 => set_log_level(LevelFilter::Debug, LevelFilter::Warn),
        3 => set_log_level(LevelFilter::Trace, LevelFilter::Info),
        _ => panic!("Too many levels of verbosity.  You can have up to 3."),
    };
}

// example command: stress --cpu 2 --timeout 10s
fn main() -> Result<()> {
    let cli = Cli::parse();
    setup_logging(cli.verbose);

    let out_file = Path::new(cli.out_file);


    let wtr = {
        Arc::new(Mutex::new(csv::Writer::from_path(Path::new("out.csv"))))
    };

    let wtr_0 = wtr.clone();

    let mut child = Command::new(&cli.command[0])
        .args(&cli.command[1..])
        .spawn()
        .expect("command failed to start");

    let pid = child.id();
    let pause = std::time::Duration::from_secs(cli.interval);
    let start_time = Local::now();

    let thread = std::thread::spawn(move ||{
        let mut sys = System::new_all();
        let pid = Pid::from_u32(pid);

        let mut t = wtr_0.lock().unwrap();
        
        // let t = t.;

        loop{
            sys.refresh_processes_specifics(
                ProcessesToUpdate::All,
                true,
                ProcessRefreshKind::nothing().with_cpu()
            );
            if let Some(process) = sys.process(pid) {
                let cpu_percent = get_children_usage(process, &sys);

                let record = UsageRecord::new(start_time, cpu_percent);
                log::info!("{:#?}", &record);
                let t = t.as_mut().unwrap();
                t.serialize(record);
            } else {
                log::info!("He's dead, Jim");
                break;
            }

            sys.refresh_processes_specifics(
                ProcessesToUpdate::All,
                true,
                ProcessRefreshKind::nothing().with_cpu()
            );
            std::thread::sleep(pause);
        }
    });

    log::info!("Waiting for command to complete...");
    child.wait();
    log::info!("Waiting for monitoring thread...");
    thread.join();
    log::info!("Flushin report");
    wtr.lock().unwrap().as_mut().unwrap().flush();
}

fn get_children_usage(parent: &Process, sys: &System) -> f32 {
    let parent_pid = parent.pid();
    let children_load: f32 = sys.processes()
        .iter()
        .filter_map(|(_, child)|
            child.parent()
                .and_then(|p|
                    if p == parent_pid { 
                        Some(get_children_usage(child, sys)) 
                    }
                    else { None }
                )
        )
        .sum();

    parent.cpu_usage() + children_load
}

#[derive(Debug, serde::Serialize)]
struct UsageRecord {
    timestamp: String,
    elapsed_seconds: usize,
    cpu_percent: u16
}

impl UsageRecord {
    fn new(start_time: DateTime<Local>, cpu_percent: f32) -> Self {
        let now = Local::now();
        let elapsed_seconds = (now - start_time).as_seconds_f32();
        Self {
            timestamp: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            elapsed_seconds: elapsed_seconds.round() as usize,
            cpu_percent: cpu_percent.round() as u16
        }
    }   
}