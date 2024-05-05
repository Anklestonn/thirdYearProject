
mod misc;

use rustls::server::ServerConnection;
use rustls::Stream;
use std::net::TcpStream;
use std::io::{BufReader,BufRead,Write};

//use std::io::Read;

pub fn hc_cc(mut stream: Stream<'_, ServerConnection, TcpStream>) {

    let buf_reader = BufReader::new(&mut stream);
    let contents: Vec <_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    //println!("Received (CC): {:#?}", contents.first());

    let order_raw = match contents.first() {
        Some(string) => string,
        None => "order1",
    };
    let order;
    if order_raw == "order2" {
        order = misc::get_param("order2");
    } else {
        order = misc::get_param("order1");
    }
    
    stream.write_all(&order).unwrap();

}
