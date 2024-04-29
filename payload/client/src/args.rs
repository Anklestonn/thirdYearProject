use std::env::args;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::string::String;

pub fn get_args()-> (IpAddr, SocketAddr, SocketAddr){
    let args: Vec<String> = args().collect();
    let ip_addr: Vec<&str> = args[1].split('.').collect();
    

    let ip_addr_1: u8 = ip_addr[0].parse().expect("Args must be a valid u8");
    let ip_addr_2: u8 = ip_addr[1].parse().expect("Args must be a valid u8");
    let ip_addr_3: u8 = ip_addr[2].parse().expect("Args must be a valid u8");
    let ip_addr_4: u8 = ip_addr[3].parse().expect("Args must be a valid u8");
    let fs_port: u16 = args[2].parse().expect("Args must be a valid u16");
    let cc_port: u16 = args[3].parse().expect("Args must be a valid u16");

    let ip_addr = IpAddr::V4(Ipv4Addr::new(ip_addr_1,ip_addr_2, ip_addr_3, ip_addr_4));
    let fs_sock_addr = SocketAddr::new(ip_addr,fs_port);
    let cc_sock_addr = SocketAddr::new(ip_addr,cc_port);


    (ip_addr, fs_sock_addr, cc_sock_addr)
}
