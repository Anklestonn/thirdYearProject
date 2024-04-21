
use local_ip_address::local_ip;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::io::Write;


pub fn save_ip() {

    let my_local_ip_raw = local_ip().unwrap();

    let my_local_ip = my_local_ip_raw.to_string();

    let file_result = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(0o600)
        .open("../conf/".to_owned() + "ip_addr");

    match file_result {
        Ok(mut file) => match file.write_all(my_local_ip.as_bytes()) {
                Ok(_) => println!("Ok: {}: Writing finished.", my_local_ip),
                Err(..) => println!("Fail: {}: Err the file can't be writen.", my_local_ip),
        },
        Err(..) => println!("Warning: {}: No file received.", my_local_ip)
    };

}
