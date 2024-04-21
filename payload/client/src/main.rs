
mod ssl_builder;
mod connection;
mod make_dir;

use std::net::TcpStream;
use std::process::Command;

fn main() {

    let code = make_dir::make_dir();
    match code {
        0 => {},
        _other =>{
            println!("Error, couldn't make the nessessary directory.");
            println!("The proram should be quit here.");
        },
    };

    let mut number_of_order: u64 = 0;
    loop {
    
        let connector = ssl_builder::ssl_builder();
        
        if let Ok(stream_cc) = TcpStream::connect("127.0.0.1:7878") {
            if let Ok(stream_fs) = TcpStream::connect("127.0.0.1:7870") {
                let stream_cc = connector.connect("127.0.0.1",stream_cc).unwrap();
                let stream_fs = connector.connect("127.0.0.1",stream_fs).unwrap();
                connection::flow(stream_cc, stream_fs, number_of_order);

                number_of_order += 1;
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
