
use std::io::{BufReader, Write, Read};
use std::net::TcpStream;
use openssl::ssl::SslStream;
use std::fs::File;


pub fn handle_connection_fs(mut stream: SslStream<TcpStream>, file_requests: &str) -> SslStream<TcpStream> {
    let message = file_requests.to_owned() + "\r\n\r\n";

    stream.write_all(message.as_bytes()).unwrap();
    println!("Sent: {:#?}", message.lines().collect::<Vec<_>>());

    
    let mut contents: Vec<u8> = Vec::new();

    for byte in BufReader::new(&mut stream).bytes() {
        let mut vec_bytes = match byte {
            Ok(b) => vec![b],
            Err(e) => {
                dbg!(e);
                println!("Error when recovering bytes.");
                vec![0]
            },
        };
        contents.append(&mut vec_bytes);
    }

    let file_result = File::create("../downloaded/".to_owned() + file_requests); 
    match file_result {
        Ok(mut file) => match file.write_all(&contents) {
                Ok(_) => println!("Writing Ok"),
                Err(..) => println!("Err with the writing."),
        },
        Err(..) => println!("Error, impossible to write to the file.")
    };

    return stream;

}
