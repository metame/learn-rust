// networking primitives are in std::net
use std::io::{Read, Result, Write};
use std::net::{TcpStream, UdpSocket};

fn write_read_tcp(port: u16) -> Result<()> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{port}"))?;

    stream.write_all(&[0, 1, 2, 3])?;

    let mut buffer = [0u8; 4];
    stream.read_exact(&mut buffer)?;
    println!("Received {buffer:?}");

    Ok(())
}

fn tcp_client() {
    match write_read_tcp(12345) {
        Ok(()) => println!("TCP client ran!"),
        Err { .. } => println!("TCP client failed :( - Is the tcp_server running?"),
    }
}

fn write_read_udp(port: u16) -> Result<()> {
    let host = format!("127.0.0.1:{port}");
    let socket = UdpSocket::bind(&host);

    socket.sent_to(&[0; 10], "172.0.0.1:")

    Ok(())
}

fn udp_client() {
    match write_read_udp(34254) {
        Ok(()) => println!("UDP client ran!"),
        Err { .. } => println!("UDP client failed :( - Is the udp_socket running?"),
    }
}

// before running this, run tcp_server
fn main() {
    tcp_client();
}
