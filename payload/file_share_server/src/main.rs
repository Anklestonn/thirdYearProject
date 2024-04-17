mod handle_connection;
mod set_ssl;

use std::net::TcpListener;
use std::thread;

fn main(){

    let acceptor = set_ssl::set_ssl();

    let listener = TcpListener::bind("127.0.0.1:7870").unwrap();
    let data = handle_connection::data_from_server();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        let acceptor = acceptor.clone();
        let data = data.clone();


        println!("Connection from client.");
         thread::spawn(move || {
             let stream = acceptor.accept(stream).unwrap();
             handle_connection::handle_connection(stream, data);
         });
    }
}
