use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use openssl::ssl::SslStream;
use std::fs::{File, write};
use std::io::prelude::*;


#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

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
    let mut file = File::create("file_from_server"); 
    //   print_type_of(&contents); //   contents is of type Vec<T>
    file.expect("something").write_all(contents);

}

