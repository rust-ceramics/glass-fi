//! HTTP/1.x Header
/// HTTP/1.x header struct
#[derive(Debug, Clone)]
pub struct HttpHeader {
    /// header name
    pub name: String,
    /// header content
    pub content: String,
}

/// multiple HTTP/1.x headers
#[derive(Debug)]
pub struct HttpHeaders {
    inner: Vec<HttpHeader>,
}

impl HttpHeaders {
    /// new HTTP/1.x headers
    pub fn new(inner: Vec<HttpHeader>) -> Self {
        HttpHeaders { inner }
    }
}

