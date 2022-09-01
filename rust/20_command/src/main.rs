use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use color_eyre::Result;

fn main() -> Result<()>{
    run_sudo("ls")
}


fn run_sudo(command: &str) -> Result<()>{
    let mut password = rpassword::prompt_password("Your password: ").unwrap();
    password.push('\n');

    let password_prompt = "[sudo] password for";

    let mut child = Command::new("bash")
        .arg("-c")
        .arg("sudo -S ls")
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("ls command failed to start");


    let stderr = child.stderr.take().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    std::thread::spawn(move || {
        use read_char::ReadIter;
        let mut acc: String = "".to_string();

        for char_res in ReadIter::new(stderr) {
            match char_res {
                Ok(ch) => {
                    acc.push(ch);
                    println!("buffer: {}", &acc);
                    if acc.contains(password_prompt) {
                        println!("sending password");
                        stdin.write_all(password.as_bytes());
                        acc.clear();
                    }
                },
                Err(read_char::Error::EOF) => break,
                Err(e) => panic!("{:?}", e),
            };
        }
    });

    let stdout = child.stdout.take().unwrap();
    for line in BufReader::new(stdout).lines() {
        let line = line.unwrap();
        println!("--stdout--> {}", line);
    }

    Ok(())


}
