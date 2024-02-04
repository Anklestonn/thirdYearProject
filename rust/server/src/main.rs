use std::net::{TcpListener,TcpStream};
use std::io::{BufReader,BufRead,Write};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("A new connection!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let contents: Vec <_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    println!("Request: {:#?}'", contents);
    
    // To make a new line, enter \r\n, to signal the End of String, \r\n\r\n
    let response = "Okay, do nothing for now o/\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();

    
}


// reference https://doc.rust-lang.org/book/ch20-01-single-threaded.html
