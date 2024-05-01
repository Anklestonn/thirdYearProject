

use std::io::BufReader;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pemfile::{Item, read_one};
use std::iter;
use std::fs::File;

pub fn get_cert(mut reader: BufReader<File>) -> CertificateDer<'static> {
    for item in iter::from_fn(|| read_one(&mut reader).transpose()) {
        match item.unwrap() {
            Item::X509Certificate(cert) => return cert,
            _ => panic!("unhandled item"),
        };
    }
    panic!("shouldnt be here");
}
pub fn get_key(mut reader: BufReader<File>) -> PrivateKeyDer<'static> {
    for item in iter::from_fn(|| read_one(&mut reader).transpose()) {
        match item.unwrap() {
            Item::Pkcs8Key(key) => return PrivateKeyDer::Pkcs8(key),
            _ => panic!("unhandled item"),
        };
    }
    panic!("shouldnt be here");
}
