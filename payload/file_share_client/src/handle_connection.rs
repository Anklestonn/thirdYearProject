use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use openssl::ssl::SslStream;


#[allow(dead_code)]
pub fn handle_connection(mut stream: SslStream<TcpStream>){
    let message = "this is a message from the client\r\n\r\n";

    stream.write_all(message.as_bytes()).unwrap();
    println!("Sent: {:#?}", message.lines().collect::<Vec<_>>());

    let contents: Vec <_> = BufReader::new(&mut stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Recieved: {:#?}", contents);
    }
