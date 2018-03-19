//! HTTP Listener
use tokio::net::TcpListener;

use std::net::SocketAddr;

/// HTTP connection listener
#[derive(Debug)]
pub struct HttpListener {
    tcp: TcpListener,
}

#[cfg(test)]
mod listner_test {
    use tokio;
    use net::HttpListener;

    #[test]
    fn new_listen() {
        let address = "127.0.0.1:10080".parse().unwrap();
        let listener = HttpListener::bind(&address).expect("unable to bind HTTP Listener");
        let server = listener
            .incoming()
            .map_err(|error| eprintln!("Error: {:?}", error))
            .for_each(|socket| Ok(()));
        tokio::run(server);
    }
}
