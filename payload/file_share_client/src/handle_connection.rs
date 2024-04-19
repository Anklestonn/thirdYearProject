use std::io::{BufReader, Write, Read};
use std::net::TcpStream;
use openssl::ssl::SslStream;
use std::fs::File;


#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn handle_connection(mut stream: SslStream<TcpStream>, file_requests: &str){
    let message = file_requests.to_owned() + "\r\n\r\n";

    stream.write_all(message.as_bytes()).unwrap();
    println!("Sent: {:#?}", message.lines().collect::<Vec<_>>());


    //let contents: Vec <_> = BufReader::new(&mut stream)
    //    .lines()
    //    .map(|result| result.unwrap())
    //    .take_while(|line| !line.is_empty())
    //    .collect();
    
    let mut contents: Vec<u8> = Vec::new();

    for byte in BufReader::new(&mut stream).bytes() {
        let mut vec_bytes = match byte {
            Ok(b) => vec![b],
            Err(e) => {
                dbg!(e);
                println!("Error when recovering bytes.");
                vec![0]
            },
        };
        contents.append(&mut vec_bytes);
    }

    let file_result = File::create("../downloaded/".to_owned() + file_requests); 
    match file_result {
        Ok(mut file) => match file.write_all(&contents) {
                Ok(_) => println!("Writing Ok"),
                Err(..) => println!("Err with the writing."),
        },
        Err(..) => println!("Error, impossible to write to the file.")
    };
    //   print_type_of(&contents); //   contents is of type Vec<T>

    //file.expect("something").write_all(&contents);

}

