
mod ssl_builder;
mod connection;

use std::net::TcpStream;
use std::process::Command;

fn main() {

    loop {
    
        let connector = ssl_builder::ssl_builder();
        
        if let Ok(stream_cc) = TcpStream::connect("127.0.0.1:7878") {
            if let Ok(stream_fs) = TcpStream::connect("127.0.0.1:7870") {
                let stream_cc = connector.connect("127.0.0.1",stream_cc).unwrap();
                let stream_fs = connector.connect("127.0.0.1",stream_fs).unwrap();
                connection::flow(stream_cc, stream_fs);
            } else {
                println!("Counldn't connect to the file_sharing socket. Is the server is up ?")
            }
        } else {
            println!("Couldn't connect to the cc socket. Is the server is up ?");
        }

        let _ = Command::new("sleep")
            .args(["5"]).output();
    }

}





// Reference https://doc.rust-lang.org/std/net/struct.TcpStream.html
