use std::{path::Path, os::unix::net::{UnixStream, UnixListener}, process::Command};
use anyhow::Result;
use std::io::prelude::*;

// https://github.com/BartMassey/unix-stream
// https://stackoverflow.com/questions/57646423/unixstream-write-read-full-string
// https://medium.com/swlh/getting-started-with-unix-domain-sockets-4472c0db4eb1
// https://www.reddit.com/r/rust/comments/em6rzh/anyone_knows_how_to_write_bytes_with_eof_when/
// https://www.reddit.com/r/learnrust/comments/mi21jy/reading_data_from_a_unix_socket/

fn main() -> Result<()>{
    let socket = "/tmp/rst.sock";

    std::fs::remove_file(Path::new(socket)).unwrap_or_else(|e| match e.kind() {
        std::io::ErrorKind::NotFound => (),
        _ => panic!("{}", e),
    });

    println!("Waiting for startup on {:?}", socket);
    println!("current wd {:?}", &std::env::current_dir());

    let listener = UnixListener::bind(socket)?;
    Command::new("target/debug/sub").spawn()?;
    match listener.accept() {
        Ok((mut stream, addr)) => {
            println!("Got a client: {:?} - {:?}", stream, addr);

            let mut buff = vec![0u8; 5];
            let mut h = stream.take(5u64);
            h.read(&mut buff);
        
            println!("Got the signal from the client {:?}, shutting down now", &buff);
        },
        Err(e) => println!("accept function failed: {:?}", e),
    }

    Ok(())
}
