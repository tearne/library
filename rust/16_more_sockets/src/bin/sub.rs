use std::{os::unix::net::UnixStream, io::Write};

use anyhow::Result;

fn main() -> Result<()> {
    

    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Connecting");
    let mut stream = UnixStream::connect("/tmp/rst.sock").unwrap();
    

    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("Sending hello message");
    let hello = [0u8,1,2,3,4];
    stream.write_all(&hello);

    println!("Sent hello message: {:?}", &hello);

    Ok(())
}