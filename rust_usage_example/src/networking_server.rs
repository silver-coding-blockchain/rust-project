use std::io::{Read, Result, Write};

pub fn main() -> Result<()> {
    let listener = std::net::TcpListener::bind("0.0.0.0:12345")?;

    // This single threaded server can handle only one incoming connection at a
    // time.
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0u8; 4096];
        let count = stream.read(&mut buffer)?;
        stream.write_all(&buffer[0..count])?;
        println!("{:?}",&buffer[0..count]);
    }
    Ok(())
}
