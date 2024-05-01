
mod certs_manips;

use rustls::server::ServerConfig;

use std::sync::Arc;
use std::io::BufReader;
use std::fs::File;



pub fn conf_tls() -> Arc<ServerConfig> {
    let config = ServerConfig::builder()
        .with_no_client_auth();

    let cert_file_raw = File::open("conf/certs.pem");
    let cert_file = match cert_file_raw {
        Ok(file) => file,
        Err(..) => panic!("Error while reading the file. certs"),
    };
    let key_file_raw = File::open("conf/privkey.pem");
    let key_file = match key_file_raw {
        Ok(file) => file,
        Err(..) => panic!("Error while reading the file. privkey"),
    };

    let cert_reader = BufReader::new(cert_file);
    let key_reader = BufReader::new(key_file);

    let cert_chain = certs_manips::get_cert(cert_reader);
    let keys = certs_manips::get_key(key_reader);

    let config = config.with_single_cert(vec!(cert_chain), keys);
    match config {
        Ok(config) => return Arc::new(config),
        Err(..) => panic!("Error setting the connection."),
    };
}
