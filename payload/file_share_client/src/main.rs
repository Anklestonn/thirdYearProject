mod handle_connection;
mod ssl_builder;

use std::net::TcpStream;
use std::process::Command;

fn main(){

    loop{
        let connector = ssl_builder::ssl_builder();

        if let Ok(stream) = TcpStream::connect("127.0.0.1:7870") {
            let stream = connector.connect("127.0.0.1",stream).unwrap();

            println!("Connected to server.");
            let contents = handle_connection::handle_connection(stream);

            println!("{:#?}", contents);
            
            println!("EOC \n");
        }else {
            println!("Couldn't connect to socket.");
        }
        let _ = Command::new("sleep")
            .args(["5"]).output();
    }

}
