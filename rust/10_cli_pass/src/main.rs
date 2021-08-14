// https://stackoverflow.com/questions/34611742/how-do-i-read-the-output-of-a-child-process-without-blocking-in-rust

// [dependencies]
// futures = "0.3.15"
// tokio = { version = "1", features = ["full"] }

use futures::stream::StreamExt; // <-- claimed to be unused
use futures::prelude::*;
use std::process::Stdio; 
// use tokio::prelude::*;  <-- doesn't exit

use tokio::{io::AsyncBufRead, io::AsyncBufReadExt, io::BufReader, process::Command};
// use tokio_stream::Stream;
// use tokio_stream::StreamExt;
// use tokio::StreamExt;

#[tokio::main]
async fn main() {
    // use futures::future;
    // use futures::stream::{self, StreamExt};
    
    // // let mut x:u16 = 0;
    
    // // {
    // //     let fut = stream::repeat(1).take(3).for_each(|item| {
    // //         x += item;
    // //         future::ready(())
    // //     });
    // //     fut.await;
    // // }
    
    // // assert_eq!(x, 3);


    let mut child = Command::new("sudo")
        .arg("ls")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Command failed");

    let mut stdout = child.stdout.take().unwrap();

    BufReader::new(stdout)
        .lines()
        .for_each(|s| async move { println!("> {:?}", s) })
        .await;

    // let mut buf = String::new();
    // stdout.take(50).read_to_string(&mut buf).unwrap();

    // println!("123");

    // let mut buf = [0; 3];
    // stdout.read_exact(&mut buf).unwrap();

    // println!("?>> buf >> {:?}", buf);
    // println!(">>> {:?}", child.wait_with_output().unwrap().stdout.as_slice());
}
