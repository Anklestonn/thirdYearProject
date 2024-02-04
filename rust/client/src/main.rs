
use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;



fn main() {
    
    if let Ok(stream) = TcpStream::connect("127.0.0.1:7878") {
        println!("Could connect to server!");
        handle_connection(stream)


    } else {
        println!("Couldn't connect to socket. Is the server is up ?");
    }

}


fn handle_connection(mut stream: TcpStream) {
    // To make a new line, enter \r\n, to signal the End of String, \r\n\r\n
    let message = "I am a client ;)\r\nalllala\r\n\r\n";

    stream.write_all(message.as_bytes()).unwrap();
    println!("Packet sent!");

    let contents: Vec <_> = BufReader::new(&mut stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Answer: {:#?}", contents);
}


// Reference https://doc.rust-lang.org/std/net/struct.TcpStream.html
