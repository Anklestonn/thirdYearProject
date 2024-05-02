
mod misc_file;
mod misc_args;

use std::env::args;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::string::String;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn get_args()-> Result<(IpAddr, SocketAddr, SocketAddr),std::io::Error>{
    let args: Vec<String> = args().collect();

    if args.len() == 1{

        // Recover File ip_serv
        let mut file = File::open("conf/ip_serv")?; // Here, we can keep the '?', because the
                                                       // client have to crash it there is no
                                                       // ip_serv precised, or if the file can't be
                                                       // read.
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let vector: Vec<&str> = contents.split(' ').collect::<Vec<_>>();
        let ip_vector: Vec<&str> = vector[0].split('.').collect();

        // Parse it's values
        let ip = misc_file::parse_ipv4_file(&ip_vector);

        let fs_port: u16 = misc_file::parse_fs_file(&vector);
        let cc_port: u16 = misc_file::parse_cc_file(&vector);

        // Make the result.
        let ip_addr = IpAddr::V4(Ipv4Addr::new(ip.0,ip.1, ip.2, ip.3));
        let fs_sock_addr = SocketAddr::new(ip_addr, fs_port);
        let cc_sock_addr = SocketAddr::new(ip_addr, cc_port);


        Ok((ip_addr, fs_sock_addr, cc_sock_addr))
    } else{

        //Parse argv values
        let ip = misc_args::parse_ipv4_argv(&args);

        let fs_port: u16 = misc_args::parse_fs_argv(&args);
        let cc_port: u16 = misc_args::parse_cc_argv(&args);

        // Make the result
        let ip_addr = IpAddr::V4(Ipv4Addr::new(ip.0, ip.1, ip.2, ip.3));
        let fs_sock_addr = SocketAddr::new(ip_addr,fs_port);
        let cc_sock_addr = SocketAddr::new(ip_addr,cc_port);

        Ok((ip_addr, fs_sock_addr, cc_sock_addr))
    }

}
