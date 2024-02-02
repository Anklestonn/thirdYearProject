
use std::io::prelude::*;
use std::net::TcpStream;



fn main() -> std::io::Result<()> {
    
    let stream = TcpStream::connect("127.0.0.1:7878");

    // To sent a packet
    let _ = stream?.write(&[1]);
    println!("Packet sent!");

    // To receive a packet
    //stream.read(&mut [0; 128]);
    //println!("Packet received");

    Ok(())
}

// Reference https://doc.rust-lang.org/std/net/struct.TcpStream.html
