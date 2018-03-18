#![deny(missing_docs)]
//! glass-fi
extern crate tokio;
extern crate url;
mod client;

/// HTTP header struct
#[derive(Debug)]
pub struct HttpHeader {
    /// header name
    pub name: String,
    /// header content
    pub content: String,
}

/// multiple HTTP headers
#[derive(Debug)]
pub struct HttpHeaders {
    inner: Vec<HttpHeader>,
}

impl HttpHeaders {
    /// new HTTP headers
    pub fn new(inner: Vec<HttpHeader>) -> Self {
        HttpHeaders { inner }
    }
}

/// HTTP response which includes also body.
#[derive(Debug)]
pub struct HttpResponse<'http> {
    /// HTTP version, ex. HTTP/1.1, HTTP/2.0
    pub version: f32,
    /// HTTP status code, ex. 200, 404
    pub status_code: usize,
    /// HTTP status string, ex. OK, Not Found
    pub status_string: String,
    /// HTTP headers
    pub head: HttpHeaders,
    /// HTTP body
    pub body: Box<&'http [u8]>,
}

#[cfg(test)]
mod test_lib {
    use {HttpHeaders, HttpResponse};

    #[test]
    fn new_http_response() {
        HttpResponse {
            version: 1.1,
            status_code: 200,
            status_string: "OK".to_string(),
            head: HttpHeaders { inner: Vec::new() },
            body: Box::new(&"".as_bytes()),
        };
    }

    #[test]
    fn new_http_headers() {
        HttpHeaders::new(Vec::new());
    }
}
