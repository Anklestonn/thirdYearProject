
mod handle_connection_cc;
mod handle_connection_fs;
mod write_order;
mod parse_order;

use std::net::TcpStream;
use openssl::ssl::{SslStream, SslConnector};
use std::net::SocketAddr;

pub fn flow(stream_cc: SslStream<TcpStream>, ip_addr: String, fs_sock: SocketAddr, connector: SslConnector, number_while: u64) {
    println!("Could connect to server!");

    let order;
    if number_while == 0 {
        order = "order1";
    } else {
        order = "order2";
    }

    let contents = handle_connection_cc::handle_connection_cc(stream_cc, order);
    write_order::write_order(order, contents.clone());

    let list_contents_download = parse_order::get_to_download(contents.clone());
    let list_contents_exec = parse_order::get_to_exec(contents);

    for string in list_contents_download {
        if let Ok(stream_fs) = TcpStream::connect(fs_sock) {
            let stream_fs = connector.connect(ip_addr.to_string().as_str(),stream_fs).unwrap();
    
            handle_connection_fs::handle_connection_fs(stream_fs, &string);
        } else {
            println!("problem downloading file {} from server.", string);
        }

    }

    for mut prog in list_contents_exec {
        let return_code = prog.exec();
        if return_code == 0 {
            println!("Ok: {}: Execution complete.", prog.get_binscript());
        } else {
            println!("Fail: {}: Error while executing the program.", prog.get_binscript());
        }
    }

    println!("End of connection \n");

}
