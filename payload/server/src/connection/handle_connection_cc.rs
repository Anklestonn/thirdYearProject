

use openssl::ssl::SslStream;
use std::net::TcpStream;
use std::io::{BufReader,BufRead,Write};

pub fn hc_cc(mut stream: SslStream<TcpStream>, order: Vec<u8>) {
    let buf_reader = BufReader::new(&mut stream);
    let contents: Vec <_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    println!("Received (CC): {:#?}", contents);
    
    stream.write_all(&order).unwrap();

}
