//! HTTP incoming stream
use net::http::HttpListener;
use net::http::HttpStream;
use info::http1::request::{HttpRequest, HttpRequestBuilder};

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
    type Item = (HttpStream, HttpRequest);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let (_socket, _) = try_ready!(self.inner.tcp.poll_accept());
        Ok(Async::Ready(Some((HttpStream{}, HttpRequestBuilder::new()
                              .version(2.0)
                              .host("A")
                              .method("GET")
                              .path("/")
                              .build()))))
    }
}
