use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use openssl::ssl::SslStream;
use std::fs::File;


#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[allow(dead_code)]
pub fn handle_connection(mut stream: SslStream<TcpStream>){
    let message = "this is a message from the client\r\n\r\n";

    stream.write_all(message.as_bytes()).unwrap();
    println!("Sent: {:#?}", message.lines().collect::<Vec<_>>());


    //let contents: Vec <_> = BufReader::new(&mut stream)
    //    .lines()
    //    .map(|result| result.unwrap())
    //    .take_while(|line| !line.is_empty())
    //    .collect();
    
    let mut contents: Vec<u8> = Vec::new();

    for line in BufReader::new(&mut stream).lines() {
        let string = match line {
            Ok(string) => string,
            Err(..) => "".to_owned(),
        };
        contents.append(&mut string.into_bytes());
    }

    let file_result = File::create("file_from_server"); 
    let mut file = match file_result {
        Ok(f) => f,
        Err(..) => panic!("Error, impossible to write to the file.")
    };
    //   print_type_of(&contents); //   contents is of type Vec<T>
    let result = file.write_all(&contents);
    match result {
        Ok(_) => println!("Writing Ok"),
        Err(..) => println!("Err with the writing."),
    };

    //file.expect("something").write_all(&contents);

}

