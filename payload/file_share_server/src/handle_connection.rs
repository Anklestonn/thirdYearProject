use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use openssl::ssl::SslStream;
use std::fs::read;

pub fn handle_connection(mut stream: SslStream<TcpStream>, data: Vec<u8>){
    let buf_reader = BufReader::new(&mut stream);
    let contents: Vec <_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Recieved data from client: {:#?}", contents);

    stream.write_all(&data).unwrap();
}

pub fn data_from_server() -> Vec<u8>{

    let response = read("file_from_server");
    response.expect("Error reading file")
}
