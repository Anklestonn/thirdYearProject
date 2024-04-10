
mod set_ssl;
mod get_order;
mod handle_connection;

use std::net::TcpListener;
use std::thread;


fn main() {

    let acceptor = set_ssl::set_ssl(); // Get Arc<SslAcceptor
    
    let order = get_order::get_order(); // Get Vec<u8> corresponding at order.

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let acceptor = acceptor.clone();

        println!("A new connection!");
        let ordeal = order.clone();

        thread::spawn(move || {
            let stream = acceptor.accept(stream).unwrap();
            handle_connection::handle_connection(stream, ordeal);
        });
    }
}


// reference https://doc.rust-lang.org/book/ch20-01-single-threaded.html
