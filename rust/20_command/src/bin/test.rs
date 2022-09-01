#![allow(unused)]
fn main() {
use std::process::{Command, Stdio};
use std::io::{self, Write};

let output = Command::new("rev")
    .stdin(Stdio::inherit())
    .stdout(Stdio::piped())
    .output()
    .expect("Failed to execute command");

print!("You piped in the reverse of: ");
io::stdout().write_all(&output.stdout).unwrap();
}