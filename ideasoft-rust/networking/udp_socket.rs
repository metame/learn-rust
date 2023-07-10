// can be tested by running then sending packet via cli like
// `$ echo "foo" | nc -w1 -u 127.0.0.1 34254`

use std::io::{Result};
use std::net::{UdpSocket};

fn udp(port: u16) -> Result<()> {
    let host = format!("127.0.0.1:{port}");
    let socket = UdpSocket::bind(&host)?;
    println!("Upd bound to {host}");

    // Receives a single datagram message on the socket. If `buf` is too small, it will be cut off
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf)?;

    // Redeclare `buf` as slice of the received data and send reverse data back to origin
    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, &src)?;
    Ok(())
}

fn udp_socket() {
    match udp(34254) {
        Ok(()) => println!("Udp ran!"),
        Err { .. } => println!("Udp failed :("),
    }
}

fn main() {
    udp_socket();
}
