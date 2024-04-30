
mod misc;

use rustls::Stream;
use std::net::TcpStream;
use rustls::server::ServerConnection;
use std::io::{BufReader,BufRead,Write};


pub fn hc_fs(mut stream: Stream<'_, ServerConnection, TcpStream>){

    let buf_reader = BufReader::new(&mut stream);
    let contents: Vec <_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();


    println!("Recieved data from client (fs): {:#?}", contents);

    let data = misc::fs_read_file(misc::parse_file_name(contents));

    stream.write_all(&data).unwrap();
}


