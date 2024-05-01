
mod ssl_builder;
mod connection;
mod make_dir;
mod args;

use std::net::TcpStream;
use std::process::Command;
use std::process::exit;


fn main() {

    let (ip_addr, fs_sock_addr, cc_sock_addr) = args::get_args();




    let code = make_dir::make_dir();
    match code {
        0 => {},
        other =>{
            println!("Error, couldn't make the nessessary directory.");
            println!("The proram should be quit here.");
            exit(other);
        },
    };

    let mut number_of_order: u64 = 0;
    loop {
    
        let connector = ssl_builder::ssl_builder();


        if let Ok(stream_cc) = TcpStream::connect(fs_sock_addr) {
            if let Ok(stream_fs) = TcpStream::connect(cc_sock_addr) {
                let stream_cc = connector.connect(ip_addr.to_string().as_str(),stream_cc).unwrap();
                let stream_fs = connector.connect(ip_addr.to_string().as_str(),stream_fs).unwrap();
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
