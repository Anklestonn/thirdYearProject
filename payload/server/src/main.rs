use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};
use std::net::{TcpListener,TcpStream};
use std::sync::Arc;
use std::thread;
use std::io::{BufReader,BufRead,Write};




fn main() {
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor.set_private_key_file("privkey.pem",SslFiletype::PEM).unwrap();
    acceptor.set_certificate_chain_file("certs.pem").unwrap();
    acceptor.check_private_key().unwrap();
    let acceptor = Arc::new(acceptor.build());
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let acceptor = acceptor.clone();

        println!("A new connection!");

        thread::spawn(move || {
            let stream = acceptor.accept(stream).unwrap();
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: SslStream<TcpStream>) {
    let buf_reader = BufReader::new(&mut stream);
    let contents: Vec <_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    println!("Received: {:#?}'", contents);
    
    // To make a new line, enter \r\n, to signal the End of String, \r\n\r\n
    let response = "Okay, do nothing for now o/\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
    println!("Send: {:#?}", response.lines().collect::<Vec<_>>());


    
}


// reference https://doc.rust-lang.org/book/ch20-01-single-threaded.html
