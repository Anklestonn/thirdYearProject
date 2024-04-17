use openssl::ssl::{SslMethod, SslConnector, SslVerifyMode};

pub fn ssl_builder() -> SslConnector{
    let builder = SslConnector::builder(SslMethod::tls());
    let mut builder = builder.expect("dk");
    builder.set_verify(SslVerifyMode::empty());
    return builder.build();
}
