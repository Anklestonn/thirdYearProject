use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn get_target_line() -> String {
   /* 
    let file = File::open("www/ip_addr_to_hack");
    let reader = BufReader::new(file);

    if let Some(Ok(firstLine)) = reader.lines().next() {
        Ok(firstLine)
    }
   */     
    let addr_to_hack = File::open("conf/ip_addr_to_hack");

    let addr_to_hack = match addr_to_hack{
        Ok(file) => file,
        Err(..) => return "".to_string(),
    };

    let file = BufReader::new(addr_to_hack);

    let addrs: Vec <_> = file
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line |!line.is_empty())
        .collect();

    let addr: String = addrs[0].clone();

    addr
}

pub fn get_vec_targeted() -> Vec<String>{

    let used_ips = File::open("conf/used_ips");

    let used_ips = match used_ips{
        Ok(file) => file,
        Err(..) => return vec!(),
    };

    let file = BufReader::new(used_ips);

    let ips: Vec <_> = file
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line |!line.is_empty())
        .collect();
    ips


}

