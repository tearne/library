use std::process::{Command, Stdio};
use unicorn::error::Error;

fn main() -> Result<(), Error> {
    let mut du_out = Command::new("du").stdout(Stdio::piped()).spawn()?;

    if let Some(out) = du_out.stdout.take() {
        let wc_out = Command::new("wc")
            .arg("-l")
            .stdin(out)
            .stdout(Stdio::piped())
            .spawn()?;

        let output = wc_out.wait_with_output()?;
        du_out.wait()?;

        println!("{}", String::from_utf8(output.stdout).unwrap());
    }

    Ok(())
}
