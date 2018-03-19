//! HTTP incoming stream
use net::http::HttpListener;
use net::http::HttpStream;

use std::io;
use futures::stream::Stream;
use futures::{Async, Poll};

/// Stream of sockets received from a listener
#[derive(Debug)]
pub struct Incoming {
    inner: HttpListener,
}

impl Incoming {
    pub(crate) fn new(listener: HttpListener) -> Self {
        Incoming { inner: listener }
    }
}

impl Stream for Incoming {
    type Item = HttpStream;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let (socket, _) = try_ready!(self.inner.tcp.poll_accept());
        Ok(Async::Ready(Some(HttpStream {})))
    }
}
