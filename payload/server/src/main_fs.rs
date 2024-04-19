
use crate::handle_connection;

use std::net::TcpListener;
use std::thread;
use openssl::ssl::SslAcceptor;
use std::sync::Arc;

pub fn file_share_server(acceptor: Arc<SslAcceptor>) {
    
    let listener_file_share = TcpListener::bind("127.0.0.1:7870").unwrap();

    for stream in listener_file_share.incoming(){
        let stream = stream.unwrap();
        let acceptor = acceptor.clone();
        //let data = data.clone();


        println!("Connection from client. (file_sharing)");
         thread::spawn(move || {
             let stream = acceptor.accept(stream).unwrap();
             handle_connection::handle_connection_fs(stream);
         });
    }
}
