
use std::io::{Write, Read};
use std::net::TcpStream;
use openssl::ssl::SslStream;

use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;


pub fn handle_connection_fs(mut stream: SslStream<TcpStream>, file_requests: &str) {
    let message = file_requests.to_owned() + "\r\n\r\n";

    stream.write_all(message.as_bytes()).unwrap();
    //println!("Sent: (fs) {:#?}", message.lines().collect::<Vec<_>>());

    
    let mut contents: Vec<u8> = Vec::new();

    stream.read_to_end(&mut contents).unwrap();

    
    let file_result = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(0o755)
        .open("downloaded/".to_owned() + file_requests);

    match file_result {
        Ok(mut file) => match file.write_all(&contents) {
                Ok(_) => (), //println!("Ok: {}: Writing file finished.", file_requests),
                Err(..) => (), //println!("Fail: {}: Err the file can't be writen.", file_requests),
        },
        Err(..) => (), //println!("Warning: {}: No file received.", file_requests)
    };

}
