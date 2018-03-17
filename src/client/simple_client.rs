#![deny(missing_docs)]

use tokio;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::net::{ConnectFuture, TcpStream};
use tokio::io;
use std::net::ToSocketAddrs;
use std::{thread, time};
use std::cmp;
use std::io::BufRead;
use std::sync::{Arc, Mutex};

use url::{self, Host, Url};

use std::error;
use std::fmt;
use std::convert;
use std::io as stdio;

#[derive(Debug)]
struct HttpBody {
    text: String,
}

#[derive(Debug, Clone)]
struct HttpHeader {
    name: String,
    content: String,
}

#[derive(Debug)]
struct HttpHeaders {
    inner: Vec<HttpHeader>,
}

impl Iterator for HttpHeaders {
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.clone().into_iter().next()
    }

    type Item = HttpHeader;
}
#[derive(Debug)]
struct HttpResponse {
    head: HttpHeaders,
    body: HttpBody,
}

impl HttpResponse {
    fn new<S: Into<String>>(head: HttpHeaders, body_text: S) -> Self {
        HttpResponse {
            head,
            body: HttpBody {
                text: body_text.into(),
            },
        }
    }
}

#[derive(Debug)]
enum HttpResponseError {
    NotHttpScheme,
    ParseURL(url::ParseError),
    Io(stdio::Error),
    InvalidSocketAddress,
}

impl fmt::Display for HttpResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpResponseError::NotHttpScheme => {
                write!(f, "Not HTTP Scheme: input string hasn't http scheme")
            }
            HttpResponseError::ParseURL(ref err) => write!(f, "Parse URL Error: {}", err),
            HttpResponseError::Io(ref err) => write!(f, "IO Error: {}", err),
            HttpResponseError::InvalidSocketAddress => write!(
                f,
                "Invalid socket address: socket address is invalid or nothing"
            ),
        }
    }
}

impl error::Error for HttpResponseError {
    fn description(&self) -> &str {
        match *self {
            HttpResponseError::NotHttpScheme => "This hasn't http scheme",
            HttpResponseError::ParseURL(ref err) => err.description(),
            HttpResponseError::Io(ref err) => err.description(),
            HttpResponseError::InvalidSocketAddress => "Invalid socket address",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            HttpResponseError::NotHttpScheme => Some(&HttpResponseError::NotHttpScheme),
            HttpResponseError::ParseURL(ref err) => Some(err),
            HttpResponseError::Io(ref err) => Some(err),
            HttpResponseError::InvalidSocketAddress => {
                Some(&HttpResponseError::InvalidSocketAddress)
            }
        }
    }
}

impl convert::From<url::ParseError> for HttpResponseError {
    fn from(err: url::ParseError) -> HttpResponseError {
        HttpResponseError::ParseURL(err)
    }
}

impl convert::From<stdio::Error> for HttpResponseError {
    fn from(err: stdio::Error) -> HttpResponseError {
        HttpResponseError::Io(err)
    }
}

const DEFAULT_HTTP_BUF_SIZE: usize = 8 * 1024;

struct HttpStream {
    inner: TcpStream,
    buffer: Box<[u8]>,
    position: usize,
    capacity: usize,
}
impl HttpStream {
    fn new(inner: TcpStream) -> Self {
        HttpStream::with_capacity(DEFAULT_HTTP_BUF_SIZE, inner)
    }

    fn with_capacity(capacity: usize, inner: TcpStream) -> Self {
        unsafe {
            let mut buffer = Vec::with_capacity(capacity);
            buffer.set_len(capacity);
            HttpStream {
                inner,
                buffer: buffer.into_boxed_slice(),
                position: 0,
                capacity: 0,
            }
        }
    }
}

impl stdio::Read for HttpStream {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, stdio::Error> {
        if self.position == self.capacity && buffer.len() >= self.buffer.len() {
            return self.inner.read(buffer);
        }

        let nread = {
            let mut remain = self.fill_buf()?;
            remain.read(buffer)?
        };
        self.consume(nread);
        Ok(nread)
    }
}

impl stdio::BufRead for HttpStream {
    fn fill_buf(&mut self) -> Result<&[u8], stdio::Error> {
        if self.position >= self.capacity {
            self.capacity = self.inner.read(&mut self.buffer)?;
            self.position = 0;
        }
        Ok(&self.buffer[self.position..self.capacity])
    }

    fn consume(&mut self, amt: usize) {
        self.position = cmp::min(self.position + amt, self.capacity);
    }
}

impl io::AsyncRead for HttpStream {}

struct SimpleClient {}

impl SimpleClient {
    fn new() -> Self {
        SimpleClient {}
    }

    fn get<S: Into<String>>(&self, url: S) -> Result<HttpResponse, HttpResponseError> {
        let issue_list_url = Url::parse(&url.into())?;
        if issue_list_url.scheme() != "http" {
            return Err(HttpResponseError::NotHttpScheme);
        }
        if let Ok(mut socket_addrs) = issue_list_url.to_socket_addrs() {
            let socket_addr = socket_addrs.next().unwrap();
            let connect_future = TcpStream::connect(&socket_addr);
            let headers = Arc::new(Mutex::new(Vec::new()));
            let content = Arc::new(Mutex::new(String::new()));
            {
                let content = content.clone();
                let headers = headers.clone();
                let task = connect_future
                    .and_then(move |mut socket| {
                        let buffer = "GET / HTTP/2.0\nHost: localhost\nConnection: keep-alive\n\n"
                            .as_bytes();
                        loop {
                            match socket.poll_write(buffer) {
                                Ok(Async::Ready(_)) => break,
                                Err(err) => eprintln!("Error: {:?}", err),
                                _ => {}
                            }

                            let milli = time::Duration::from_millis(1);
                            let now = time::Instant::now();
                            thread::sleep(milli);
                        }

                        let content = content.clone();
                        let headers = headers.clone();
                        let mut in_http_header = false;
                        let mut http_content_remain: i64 = 0;
                        let http_stream = HttpStream::new(socket);
                        let read_to_end_task = io::lines(http_stream)
                            .map_err(|err| eprintln!("Error: {:?}", err))
                            .for_each(move |input| {
                                eprintln!("Read :{}", input);
                                if !in_http_header && http_content_remain > 0 {
                                    http_content_remain -= input.len() as i64 + 1;
                                    let mut content = content.lock().unwrap();
                                    *content = format!("{}{}\n", *content, input);
                                    if http_content_remain <= 0 {
                                        (*content).pop().unwrap();
                                        return Err(());
                                    }
                                    return Ok(());
                                }
                                if let Some(_) = input.find("HTTP") {
                                    in_http_header = true;
                                    return Ok(());
                                }
                                match input {
                                    ref x if x.trim().is_empty() => {
                                        in_http_header = false;
                                        Ok(())
                                    }
                                    header_content => {
                                        let mut header_content = header_content.splitn(2, ':');
                                        let (name, content) = (
                                            header_content.next().unwrap(),
                                            header_content.next().unwrap(),
                                        );
                                        let mut headers = headers.lock().unwrap();
                                        (*headers).push(HttpHeader {
                                            name: name.trim().to_string(),
                                            content: content.trim().to_string(),
                                        });
                                        if let Some(num) = name.trim().find("Content-Length") {
                                            if num == 0 {
                                                http_content_remain =
                                                    content.trim().parse::<_>().unwrap();
                                                eprintln!(
                                                    "Content remain: {:?}",
                                                    &http_content_remain
                                                );
                                            }
                                        }
                                        Ok(())
                                    }
                                }
                            })
                            .map_err(|err| eprintln!("Error: {:?}", err));
                        let mut http_runtime = Runtime::new().unwrap();
                        http_runtime.spawn(read_to_end_task);
                        http_runtime.shutdown_now().wait().unwrap();
                        Ok(())
                    })
                    .map_err(|err| eprintln!("Error: {:?}", err));
                let mut rt = Runtime::new().unwrap();
                rt.spawn(task);
                rt.shutdown_on_idle().wait().unwrap();
            }
            let content = content.lock().unwrap();
            eprintln!("Content:\n{:}", content);
            let headers = headers.lock().unwrap();
            Ok(HttpResponse::new(
                HttpHeaders {
                    inner: (*headers).clone(),
                },
                (*content).clone(),
            ))
        } else {
            Err(HttpResponseError::InvalidSocketAddress)
        }
    }

    fn head<S: Into<String>>(&self, url: S) -> Result<HttpHeaders, HttpResponseError> {
        let issue_list_url = Url::parse(&url.into())?;
        if issue_list_url.scheme() != "http" {
            return Err(HttpResponseError::NotHttpScheme);
        }
        if let Ok(mut socket_addrs) = issue_list_url.to_socket_addrs() {
            let socket_addr = socket_addrs.next().unwrap();
            let inner = Arc::new(Mutex::new(Vec::new()));
            let connect_future = TcpStream::connect(&socket_addr);
            {
                let inner = inner.clone();
                let task = connect_future
                    .and_then(move |mut socket| {
                        let buffer = "HEAD / HTTP/2.0\nHost: localhost\nConnection: keep-alive\n\n"
                            .as_bytes();
                        loop {
                            match socket.poll_write(buffer) {
                                Ok(Async::Ready(_)) => break,
                                Err(err) => eprintln!("Error: {:?}", err),
                                _ => {}
                            }

                            let milli = time::Duration::from_millis(1);
                            let now = time::Instant::now();
                            thread::sleep(milli)
                        }

                        let inner = inner.clone();
                        let mut in_http_header = false;
                        let mut http_content_remain: i64 = 0;
                        let http_stream = HttpStream::new(socket);
                        let lines_task = io::lines(http_stream)
                            .map_err(|err| eprintln!("Error: {:?}", err))
                            .for_each(move |input| {
                                eprintln!("Read : {}", input);
                                if let Some(_) = input.find("HTTP") {
                                    in_http_header = true;
                                    return Ok(());
                                }
                                match input {
                                    ref x if x.trim().is_empty() => Err(()),
                                    header_content => {
                                        let mut header_content = header_content.splitn(2, ':');
                                        let (name, content) = (
                                            header_content.next().unwrap(),
                                            header_content.next().unwrap(),
                                        );
                                        let mut inner = inner.lock().unwrap();
                                        (*inner).push(HttpHeader {
                                            name: name.trim().to_string(),
                                            content: content.trim().to_string(),
                                        });
                                        Ok(())
                                    }
                                }
                            })
                            .map_err(|err| eprintln!("Error: {:?}", err));
                        let mut http_runtime = Runtime::new().unwrap();
                        http_runtime.spawn(lines_task);
                        http_runtime.shutdown_now().wait().unwrap();
                        Ok(())
                    })
                    .map_err(|err| eprintln!("Error: {:?}", err));
                let mut rt = Runtime::new().unwrap();
                rt.spawn(task);
                rt.shutdown_on_idle().wait().unwrap();
            }
            let inner = inner.lock().unwrap();
            let inner = (*inner).clone();
            Ok(HttpHeaders { inner })
        } else {
            Err(HttpResponseError::InvalidSocketAddress)
        }
    }
}

#[test]
fn simple_get_http() {
    let client = SimpleClient::new();
    let response = client.get("http://127.0.0.1/").unwrap();
    let body_text = response.body.text;
    assert_eq!("Hello World!", body_text);

    let response = client.get("http://127.0.0.1:81/").unwrap();
    let body_text = response.body.text;
    assert_eq!("Hello World?", body_text);
}

#[test]
fn get_headers() {
    let client = SimpleClient::new();
    let mut headers = client.head("http://127.0.0.1/").unwrap();
    let server_name = headers.find(|x| x.name == "Server").unwrap().content;
    assert_eq!("nginx/1.10.3 (Ubuntu)", server_name);

    let mut headers = client.head("http://127.0.0.1:10080/").unwrap();
    let server_name = headers.find(|x| x.name == "Server").unwrap().content;
    assert_eq!("glass-fi server", server_name);
}

#[test]
fn get_headers_by_get_request() {
    let client = SimpleClient::new();
    let mut headers = client.get("http://127.0.0.1/").unwrap().head;
    let server_name = headers.find(|x| x.name == "Server").unwrap().content;
    assert_eq!("nginx/1.10.3 (Ubuntu)", server_name);
}
