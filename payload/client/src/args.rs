use std::env::args;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::string::String;
use std::fs::File;
use std::io::Read;

pub fn get_args()-> Result<(IpAddr, SocketAddr, SocketAddr),&'static str>{
    let args: Vec<String> = args().collect();

    if args.len() == 0{

        let mut file = File::open("../conf/default_args")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let vector: Vec<&str> = contents.split(' ').collect::<Vec<_>>();
        let ip_vector: Vec<&str> = vector[1].split('.').collect();

        let ip_addr_1: u8 = ip_vector[0].parse().expect("Conf file error");
        let ip_addr_2: u8 = ip_vector[1].parse().expect("Conf file error");
        let ip_addr_3: u8 = ip_vector[2].parse().expect("Conf file error");
        let ip_addr_4: u8 = ip_vector[3].parse().expect("Conf file error");

        let fs_port: u16 = vector[2].parse().expect("Conf file error");
        let cc_port: u16 = vector[3].parse().expect("Conf file error");



        let ip_addr = IpAddr::V4(Ipv4Addr::new(ip_addr_1,ip_addr_2, ip_addr_3, ip_addr_4));
        let fs_sock_addr = SocketAddr::new(ip_addr, fs_port);
        let cc_sock_addr = SocketAddr::new(ip_addr, cc_port);


        Ok((ip_addr, fs_sock_addr, cc_sock_addr))
    } else{
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

        Ok((ip_addr, fs_sock_addr, cc_sock_addr))
    }


}
