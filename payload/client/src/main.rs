
use openssl::ssl::{SslMethod, SslConnector, SslStream, SslVerifyMode};
use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use std::process::Command;

use std::fs;
// use std::io::prelude::*;



fn main() {

    loop {
    
        let builder = SslConnector::builder(SslMethod::tls());
        let mut builder = builder.expect("dk");
        builder.set_verify(SslVerifyMode::empty());
        let connector = builder.build();

        if let Ok(stream) = TcpStream::connect("127.0.0.1:7878") {
            
            let stream = connector.connect("127.0.0.1",stream).unwrap();

            println!("Could connect to server!");
            
            handle_connection(stream);


        } else {
            println!("Couldn't connect to socket. Is the server is up ?");
        }

        let _ = Command::new("sleep")
            .args(["5"]).output();
    }

}


fn handle_connection(mut stream: SslStream<TcpStream>) {
    // To make a new line, enter \r\n, to signal the End of String, \r\n\r\n
    let message = "I am a client ;)\r\nalllala\r\n\r\n";


    stream.write_all(message.as_bytes()).unwrap();
    println!("Send: {:#?}", message.lines().collect::<Vec<_>>());

    let contents: Vec <_> = BufReader::new(&mut stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Received: {:#?}", contents);

    let content_as_string = make_one_string(contents);

    write_order(content_as_string);

    println!("End of connection \n");
}

fn make_one_string(vec_string: Vec<String>) -> String {
    let mut ret = String::new();
    for chaine in vec_string.iter() {
        if chaine != "" {
            ret.push_str(&chaine);
            ret.push('\n');
        }

    }

    return ret
}

fn write_order(contents: String) -> u32{
    // return an error code of 1 in case file couldn't be write.
    // if everything good, return 0.
    
    let order_option = fs::OpenOptions::new().write(true).create(true).open("order");
    let mut order = match order_option {
        Ok(file) => file,
        Err(error) => {
            dbg!(error);
            println!("impossible to write contents");
            return 1;
        },
    };

    let contents_bytes = contents.as_bytes();
    let _ = order.write_all(&contents_bytes);

    return 0;

    
    
}


// Reference https://doc.rust-lang.org/std/net/struct.TcpStream.html
