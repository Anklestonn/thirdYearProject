
mod set_ssl;
mod connection;
mod save_ip;

use std::thread;


fn main() {

    let acceptor = set_ssl::set_ssl(); // Get Arc<SslAcceptor>

    let my_ip = save_ip::save_ip();
    
    // Launch in a new thread the file sharing server
    
    let my_ip_fs = my_ip.clone();
    let acceptor_fs = acceptor.clone();
    thread::spawn(move || { // lauch file_share_server
        connection::file_share_server(acceptor_fs, my_ip_fs);
    });

    // launch the Command an Control server.
    connection::command_control_server(acceptor, my_ip);

}


// reference https://doc.rust-lang.org/book/ch20-01-single-threaded.html
