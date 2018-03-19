//! HTTP Listener
use net::http::Incoming;

use tokio::net::TcpListener;
use std::io;
use std::net::SocketAddr;

/// HTTP connection listener
#[derive(Debug)]
pub struct HttpListener {
    pub(crate) tcp: TcpListener,
}

impl HttpListener {
    /// binding socket address(ex. 127.0.0.1:10080) for HttpListener
    pub fn bind(socket_address: &SocketAddr) -> io::Result<Self> {
        let tcp = TcpListener::bind(socket_address)?;
        Ok(HttpListener { tcp })
    }

    /// get incoming HTTP streams
    pub fn incoming(self) -> Incoming {
        Incoming::new(self)
    }
}

#[cfg(test)]
mod listner_test {
    use tokio;
    use net::HttpListener;
    use futures::Stream;

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
