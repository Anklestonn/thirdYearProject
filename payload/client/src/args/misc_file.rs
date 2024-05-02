

pub fn parse_ipv4_file(ip_vector: &Vec<&str>) -> (u8, u8, u8, u8) {

    // Crash if there is a problem with the parsing, it is an unrecoverable error.

    let ip_addr_1: u8 = ip_vector[0].parse().expect("Please use a valid u8");
    let ip_addr_2: u8 = ip_vector[1].parse().expect("Please use a valid u8");
    let ip_addr_3: u8 = ip_vector[2].parse().expect("Please use a valid u8");
    let ip_addr_4: u8 = ip_vector[3].parse().expect("Please use a valid u8");

    (ip_addr_1, ip_addr_2, ip_addr_3, ip_addr_4)

}

pub fn parse_fs_file(args: &Vec<&str>) -> u16 {
    if args.len() < 2 {
        return 7870;
    }

    let fs_port: Result<u16, _> = args[1].parse();
    match fs_port {
        Ok(port) => port,
        _ => 7870,
    }
}

pub fn parse_cc_file(args: &Vec<&str>) -> u16 {
    if args.len() < 3 {
        return 7878
    }

    let cc_port: Result<u16, _> = args[2].parse();
    match cc_port {
        Ok(port) => port,
        _ => 7878,
    }
}
