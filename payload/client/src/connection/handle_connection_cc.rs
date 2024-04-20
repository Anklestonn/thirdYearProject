
use std::io::{BufReader, BufRead};
use std::io::{Write};
use std::net::TcpStream;
use openssl::ssl::SslStream;

pub fn handle_connection_cc(mut stream: SslStream<TcpStream>) -> Vec<String> {
    // To make a new line, enter \r\n, to signal the End of String, \r\n\r\n
    let message = "Order_1 Pls\r\n\r\n";


    stream.write_all(message.as_bytes()).unwrap();
    println!("Send: {:#?}", message.lines().collect::<Vec<_>>());

    let contents: Vec <_> = BufReader::new(&mut stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Received: {:#?}", contents);
    return contents;

}
