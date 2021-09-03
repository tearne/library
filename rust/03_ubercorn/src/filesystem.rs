use crate::error::Error;
use std::{
    process::{Command, Stdio},
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum FSStatus {
    ReadOnly,
    ReadWrite,
}

impl FSStatus {
    pub fn init_polling() -> Arc<Mutex<FSStatus>> {
        let result = Arc::new(Mutex::new(FSStatus::get().unwrap_or(FSStatus::ReadWrite)));

        let data = result.clone();
        std::thread::spawn(move || {
            let delay = std::time::Duration::from_secs(5);
            loop {
                std::thread::sleep(delay);
                let mut data = data.lock().unwrap();
                *data = FSStatus::get().unwrap_or(FSStatus::ReadWrite);
            }
        });

        result
    }

    pub fn get() -> Result<FSStatus, Error> {
        let mut mount = Command::new("mount").stdout(Stdio::piped()).spawn()?;

        if let Some(mount_out) = mount.stdout.take() {
            let sed = Command::new("sed")
                .arg("-n")
                .arg("-e")
                .arg(r#"s/^\/dev\/.* on \/ .*(\(r[w|o]\).*/\1/p"#)
                .stdin(mount_out)
                .stdout(Stdio::piped())
                .spawn()?;

            let out = sed.wait_with_output()?;
            let _ = mount.wait()?;

            let str = String::from_utf8(out.stdout)?;
            FSStatus::from_str(&str)
        } else {
            Result::Err(Error::Internal(
                "Failed to get file system status".to_string(),
            ))
        }
    }
}

impl FromStr for FSStatus {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.contains("ro") {
            Result::Ok(FSStatus::ReadOnly)
        } else if str.contains("rw") {
            Result::Ok(FSStatus::ReadWrite)
        } else {
            Result::Err(Error::Internal(format!("can't parse: {}", str)))
        }
    }
}
