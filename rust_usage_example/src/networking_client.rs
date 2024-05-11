use std::io::{Read, Result, Write};
use std::thread;
use std::time::Duration;
pub fn main() -> Result<()> {
    let mut stream = std::net::TcpStream::connect("127.0.0.1:12345")?;

    stream.write_all(&[0, 1, 2, 3])?;
    let mut buffer = [0u8; 4];
    let count=stream.read(&mut buffer)?;
    println!("Received {:?}",&buffer[0..count]);
    let mut stream = std::net::TcpStream::connect("127.0.0.1:12345")?;
    thread::sleep(Duration::from_millis(100));
    stream.write_all("abcd".as_bytes())?;
    let mut buffer = [0u8; 4];
    let count = stream.read(&mut buffer)?;
    println!("Received {:?}",&buffer[0..count]);

    Ok(())
}
