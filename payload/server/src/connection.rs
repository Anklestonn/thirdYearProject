
mod handle_connection_fs;
mod handle_connection_cc;


use std::net::TcpListener;
use std::thread;
use rustls::server::{ServerConfig, ServerConnection};
use rustls::Stream;
use std::sync::Arc;

pub fn file_share_server(acceptor: Arc<ServerConfig>, my_ip: String) {
    
    let listener_file_share = TcpListener::bind(my_ip + ":7870").unwrap();

    for stream in listener_file_share.incoming(){
        let mut stream = stream.unwrap();
        let conn_raw = ServerConnection::new(acceptor.clone());
        let mut conn = match conn_raw {
            Ok(con) => con,
            Err(..) => panic!("problem with network"),
        };

        //println!("Connection from client. (file_sharing)");

        thread::spawn(move || {
            let stream = Stream::new(&mut conn, &mut stream);
            handle_connection_fs::hc_fs(stream);
        });
    }
}

pub fn command_control_server(acceptor: Arc<ServerConfig>, my_ip: String) {
    
    let listener_command_control = TcpListener::bind(my_ip + ":7878").unwrap();

    for stream in listener_command_control.incoming() {
        let mut stream = stream.unwrap();
        let conn_raw = ServerConnection::new(acceptor.clone());
        let mut conn = match conn_raw {
            Ok(con) => con,
            Err(..) => panic!("problem with network"),
        };

        //println!("Connection from client. (Command_Control)");

        thread::spawn(move || {
            let stream = Stream::new(&mut conn, &mut stream);
            handle_connection_cc::hc_cc(stream);
        });
    }

}
