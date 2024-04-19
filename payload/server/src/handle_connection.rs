
use openssl::ssl::SslStream;
use std::io::{BufReader,BufRead,Write};
use std::net::TcpStream;
use std::fs::read;

pub fn handle_connection_cc(mut stream: SslStream<TcpStream>, order: Vec<u8>) {
    let buf_reader = BufReader::new(&mut stream);
    let contents: Vec <_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    println!("Received: {:#?}'", contents);
    

    //stream.write_all(&response_hello_world()).unwrap();
    stream.write_all(&order).unwrap();


    //println!("Send: {:#?}", &order );
    
}

#[allow(dead_code)]
fn response_hello_world() -> Vec<u8> {
    // To make a new line, enter \r\n, to signal the End of String, \r\n\r\n
    let response = "Hello world\r\n\r\n";
    return response.as_bytes().to_vec();
}


pub fn handle_connection_fs(mut stream: SslStream<TcpStream>){
    let buf_reader = BufReader::new(&mut stream);
    let contents: Vec <_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();


    println!("Recieved data from client: {:#?}", contents);

    let data = data_from_server_fs(parse_contents(contents));

    stream.write_all(&data).unwrap();
}

fn data_from_server_fs(file: String) -> Vec<u8>{

    match read("../www/".to_owned() + &file) {
        Ok(f) => return f,
        Err(..) => {
            println!("Error reading file");
            return "404".as_bytes().to_vec();
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
