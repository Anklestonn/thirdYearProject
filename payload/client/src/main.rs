
use openssl::ssl::{SslMethod, SslConnector, SslStream, SslVerifyMode};
use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use std::process::Command;



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
            .args(["2"]).output();
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

    println!("End of connection \n");
}


// Reference https://doc.rust-lang.org/std/net/struct.TcpStream.html
