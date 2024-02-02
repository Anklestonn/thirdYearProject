use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Packet received!");
    }
}


// reference https://doc.rust-lang.org/book/ch20-01-single-threaded.html
