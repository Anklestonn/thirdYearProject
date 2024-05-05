use local_ip_address::local_ip;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::io::Write;


pub fn save_ip() -> String{

    let my_local_ip_raw = local_ip().unwrap();

    let my_local_ip = my_local_ip_raw.to_string();

    let file_result = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(0o600)
        .open("www/".to_owned() + "ip_addr");

    match file_result {
        Ok(mut file) => match file.write_all(my_local_ip.as_bytes()) {
                Ok(_) => (),
                Err(..) => println!("Fail: {}: Err the file can't be writen.", my_local_ip),
        },
        Err(..) => println!("Warning: {}: No file received.", my_local_ip)
    };


    return my_local_ip.clone();

}

/*fn make_ip_addr(ip_addr: &str) -> IpAddr{

        let ip_vector: Vec<&str> = ip_addr.split('.').collect();


        let ip_addr_1: u8 = ip_vector[0].parse().expect("Please use a valid u8");
        let ip_addr_2: u8 = ip_vector[1].parse().expect("Please use a valid u8");
        let ip_addr_3: u8 = ip_vector[2].parse().expect("Please use a valid u8");
        let ip_addr_4: u8 = ip_vector[3].parse().expect("Please use a valid u8");


        let ip_addr = IpAddr::V4(Ipv4Addr::new(ip_addr_1,ip_addr_2, ip_addr_3, ip_addr_4));
        
        ip_addr
}*/

/*pub fn ip_targeting(path: &str) -> Result<Vec<IpAddr>,std::io::Error>{

    let mut victims_ip: Vec<IpAddr> = vec![];

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines(){

        let line = make_ip_addr(&line.unwrap());
        victims_ip.push(line);
    }

    Ok(victims_ip)

}*/

