
mod get_order;
mod handle_connection_fs;
mod handle_connection_cc;

use std::net::TcpListener;
use std::thread;
use openssl::ssl::SslAcceptor;
use std::sync::Arc;

pub fn file_share_server(acceptor: Arc<SslAcceptor>) {
    
    let listener_file_share = TcpListener::bind("127.0.0.1:7870").unwrap();

    for stream in listener_file_share.incoming(){
        let stream = stream.unwrap();
        let acceptor = acceptor.clone();

        println!("Connection from client. (file_sharing)");

        thread::spawn(move || {
             let stream = acceptor.accept(stream).unwrap();
             handle_connection_fs::hc_fs(stream);
        });
    }
}

pub fn command_control_server(acceptor: Arc<SslAcceptor>) {
    
    let listener_command_control = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener_command_control.incoming() {
        let stream = stream.unwrap();
        let acceptor = acceptor.clone();

        println!("Connection from client. (Command_Control)");
        let order1 = get_order::get_order();

        thread::spawn(move || {
            let stream = acceptor.accept(stream).unwrap();
            handle_connection_cc::hc_cc(stream, order1);
        });
    }

}
