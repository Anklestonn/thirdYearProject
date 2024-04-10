

use openssl::ssl::{SslMethod, SslAcceptor, SslFiletype};
use std::sync::Arc;

pub fn set_ssl() -> Arc<SslAcceptor> {
    
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor.set_private_key_file("privkey.pem",SslFiletype::PEM).unwrap();
    acceptor.set_certificate_chain_file("certs.pem").unwrap();
    acceptor.check_private_key().unwrap();
    return Arc::new(acceptor.build());
}
