
mod handle_connection_cc;
mod handle_connection_fs;
mod write_order;
mod parse_order;

use std::net::TcpStream;
use openssl::ssl::SslStream;

pub fn flow(stream_cc: SslStream<TcpStream>, mut stream_fs: SslStream<TcpStream>) {
    println!("Could connect to server!");

    let contents = handle_connection_cc::handle_connection_cc(stream_cc);
    write_order::write_order(contents.clone());

    let list_contents_download = parse_order::get_to_download(contents); // TODO
    for string in list_contents_download {
    
        stream_fs = handle_connection_fs::handle_connection_fs(stream_fs, &string);

    }


    println!("End of connection \n");

}
