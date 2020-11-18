use std::os::unix::net::UnixStream;
use std::io::prelude::*;

fn main() {

    let mut stream = UnixStream::connect("/tmp/socket.sock").unwrap();
    // let mut length = 5u16.to_be_bytes();

    let numbers: Vec<u16> = (0..10).into_iter().map(|n| n as u16).collect();
    let mut bytes = vec![0u8; numbers.len() * 2];
    for (idx, value) in numbers.iter().enumerate() {
        let bytes_arr = u16::to_be_bytes(*value);
        println!("Encoding {} -> {:?}", value, bytes_arr);
        bytes[2 * idx] = bytes_arr[0];
        bytes[2 * idx + 1] = bytes_arr[1];
    }
    // let mut buf = [1u8, 2u8, 3u8, 4u8, 5u8];
    let length = (numbers.len() as u16).to_be_bytes();
    println!("Transmit length {} as {:?}", numbers.len(), length);
    stream.write_all(&length).unwrap();
    println!("Bytes out: {:?}", bytes);
    stream.write_all(&bytes).unwrap();


    let mut buff = vec![0u8; numbers.len() * 2];
    let mut h = stream.take((numbers.len() * 2) as u64);
    h.read(&mut buff);

    let res: Vec<u16> = buff
        .chunks(2)
        .into_iter()
        .map(|c| {
            println!("Chunk {:?}", c);
            u16::from_be_bytes([c[0], c[1]])
        })
        .collect();
    println!("response : {:?}", res);
}