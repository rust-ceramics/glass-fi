//! Server
use tokio::net::TcpListener;

/// HTTP connection listener
#[derive(Debug)]
pub struct HttpListener {
    tcp: TcpListener,
}

#[cfg(test)]
mod server_test {
    use tokio;
    use server::HttpListener;

    #[test]
    fn new_server() {
        let address = "127.0.0.1:10080".parse().unwrap();
        let listener = HttpListener::bind(&address).expect("unable to bind HTTP Listener");
        let server = listener
            .incoming()
            .map_err(|error| eprintln!("Error: {:?}", error))
            .for_each(|socket| Ok(()));
        tokio::run(server);
    }
}
