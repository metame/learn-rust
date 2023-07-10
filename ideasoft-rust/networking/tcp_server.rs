use std::io::{Read, Result, Write};
use std::net::{TcpListener};

fn tcp_server(port: u16) -> Result<()> {
    let host = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&host)?;
    println!("Server started at {host}");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0u8; 4096];
        let count = stream.read(&mut buffer)?;
        stream.write_all(&buffer[0..count])?;
    }
    Ok(())
}

fn start_server() {
    match tcp_server(12345) {
        Ok(()) => println!("Server good!"),
        Err { .. } => println!("Server failed to start :("),
    }
}

fn main() {
    start_server();
}
