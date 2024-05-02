
pub fn parse_ipv4_argv(args: &Vec<String>) -> (u8, u8, u8, u8) {
    let ip_addr: Vec<&str> = args[1].split('.').collect();

    // Crash if there is a problem with the parsing, it is an unrecoverable error.

    let ip_addr_1: u8 = ip_addr[0].parse().expect("Invalid input. Please us the format: a.b.c.d fs_port_number cc_port_number");
    let ip_addr_2: u8 = ip_addr[1].parse().expect("Invalid input. Please us the format: a.b.c.d fs_port_number cc_port_number");
    let ip_addr_3: u8 = ip_addr[2].parse().expect("Invalid input. Please us the format: a.b.c.d fs_port_number cc_port_number");
    let ip_addr_4: u8 = ip_addr[3].parse().expect("Invalid input. Please us the format: a.b.c.d fs_port_number cc_port_number");

    (ip_addr_1, ip_addr_2, ip_addr_3, ip_addr_4)
}

pub fn parse_fs_argv(args: &Vec<String>) -> u16 {

    if args.len() < 3 {
        return 7870;
    }

    let fs_port: Result<u16, _> = args[2].parse();
    match fs_port {
        Ok(port) => port,
        _ => 7870,
    }
}

pub fn parse_cc_argv(args: &Vec<String>) -> u16 {

    if args.len() < 4 {
        return 7878
    }

    let cc_port: Result<u16, _> = args[3].parse();
    match cc_port {
        Ok(port) => port,
        _ => 7878,
    }
}
