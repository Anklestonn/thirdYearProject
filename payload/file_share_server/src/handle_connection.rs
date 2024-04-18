use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use openssl::ssl::SslStream;
use std::fs::read;

pub fn handle_connection(mut stream: SslStream<TcpStream>){
    let buf_reader = BufReader::new(&mut stream);
    let contents: Vec <_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();


    println!("Recieved data from client: {:#?}", contents);

    let data = data_from_server(parse_contents(contents));

    stream.write_all(&data).unwrap();
}

fn data_from_server(file: String) -> Vec<u8>{

    match read("../www/".to_owned() + &file) {
        Ok(f) => return f,
        Err(..) => {
            println!("Error reading file");
            return Vec::new();
        },
    };
}

fn parse_contents(lines: Vec<String>) -> String {
    let first_line: &String = &lines[0];

    for c in first_line.chars() {
        match c {
            ' ' | '/' | '\\' | '`' | '|' | '\'' => return "error_file".to_owned(),
            _other => {}
        };
    }
    println!("{}",first_line);
    return first_line.clone()
}
