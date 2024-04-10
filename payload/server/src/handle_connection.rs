
use openssl::ssl::SslStream;
use std::io::{BufReader,BufRead,Write};
use std::net::TcpStream;

pub fn handle_connection(mut stream: SslStream<TcpStream>, order: Vec<u8>) {
    let buf_reader = BufReader::new(&mut stream);
    let contents: Vec <_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    println!("Received: {:#?}'", contents);
    

    //stream.write_all(&response_hello_world()).unwrap();
    stream.write_all(&order).unwrap();


    //println!("Send: {:#?}", &order );
    
}

#[allow(dead_code)]
fn response_hello_world() -> Vec<u8> {
    // To make a new line, enter \r\n, to signal the End of String, \r\n\r\n
    let response = "Hello world\r\n\r\n";
    return response.as_bytes().to_vec();
}
