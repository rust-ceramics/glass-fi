#![deny(missing_docs)]
//! glass-fi
extern crate tokio;
extern crate url;
#[macro_use]
extern crate futures;

mod client;
pub mod net;

/// HTTP header struct
#[derive(Debug, Clone)]
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
pub struct HttpResponse {
    version: f32,
    status_code: usize,
    status_string: String,
    head: HttpHeaders,
    body: Option<Vec<u8>>,
}

/// Builder struct for HTTP response.
#[derive(Debug)]
pub struct HttpResponseBuilder<VersionType, HostType, StatusCodeType, StatusStringType> {
    version: VersionType,
    host: HostType,
    status_code: StatusCodeType,
    status_string: StatusStringType,
    head: Vec<HttpHeader>,
    server: String,
    body: Option<Vec<u8>>,
}

impl HttpResponseBuilder<(), (), (), ()> {
    /// new HTTP Response Builder
    pub fn new() -> Self {
        HttpResponseBuilder {
            version: (),
            host: (),
            status_code: (),
            status_string: (),
            head: Vec::new(),
            server: "glass-fi".to_string(),
            body: None,
        }
    }
}

impl HttpResponseBuilder<f32, String, usize, String> {
    /// build HTTP response
    pub fn build(self) -> HttpResponse {
        let mut head = self.head.clone();
        head.push(HttpHeader {
            name: "Host".to_string(),
            content: self.host,
        });
        let mut head = self.head.clone();
        head.push(HttpHeader {
            name: "Server".to_string(),
            content: self.server,
        });
        HttpResponse {
            version: self.version,
            status_code: self.status_code,
            status_string: self.status_string,
            head: HttpHeaders::new(self.head),
            body: self.body,
        }
    }
}

impl<VersionType, HostType, StatusCodeType, StatusStringType>
    HttpResponseBuilder<VersionType, HostType, StatusCodeType, StatusStringType>
{
    /// HTTP version, e.g., 1.1, 2.0
    pub fn version(
        self,
        version: f32,
    ) -> HttpResponseBuilder<f32, HostType, StatusCodeType, StatusStringType> {
        HttpResponseBuilder {
            version,
            host: self.host,
            status_code: self.status_code,
            status_string: self.status_string,
            head: self.head,
            server: self.server,
            body: self.body,
        }
    }

    /// response side Host
    pub fn host<S: Into<String>>(
        self,
        host: S,
    ) -> HttpResponseBuilder<VersionType, String, StatusCodeType, StatusStringType> {
        HttpResponseBuilder {
            version: self.version,
            host: host.into(),
            status_code: self.status_code,
            status_string: self.status_string,
            head: self.head,
            server: self.server,
            body: self.body,
        }
    }

    /// Status Code, e.g., 200, 404
    pub fn status_code(
        self,
        status_code: usize,
    ) -> HttpResponseBuilder<VersionType, HostType, usize, StatusStringType> {
        HttpResponseBuilder {
            version: self.version,
            host: self.host,
            status_code,
            status_string: self.status_string,
            head: self.head,
            server: self.server,
            body: self.body,
        }
    }

    /// Status String, e.g., OK, Not Found
    pub fn status_string<S: Into<String>>(
        self,
        status_string: S,
    ) -> HttpResponseBuilder<VersionType, HostType, StatusCodeType, String> {
        HttpResponseBuilder {
            version: self.version,
            host: self.host,
            status_code: self.status_code,
            status_string: status_string.into(),
            head: self.head,
            server: self.server,
            body: self.body,
        }
    }

    /// HTTP-response header
    pub fn header(mut self, header: HttpHeader) -> Self {
        self.head.push(header);
        HttpResponseBuilder {
            version: self.version,
            host: self.host,
            status_code: self.status_code,
            status_string: self.status_string,
            head: self.head,
            server: self.server,
            body: self.body,
        }
    }
}

/// HTTP request which includes also body.
#[derive(Debug)]
pub struct HttpRequest {
    version: f32,
    method: String,
    path: String,
    head: HttpHeaders,
    body: Option<Vec<u8>>,
}

/// Builder struct for HTTP request.
#[derive(Debug)]
pub struct HttpRequestBuilder<VersionType, HostType, MethodType, PathType> {
    version: VersionType,
    host: HostType,
    method: MethodType,
    path: PathType,
    head: Vec<HttpHeader>,
    body: Option<Vec<u8>>,
}

impl HttpRequestBuilder<(), (), (), ()> {
    /// new HTTP Request Builder
    pub fn new() -> Self {
        HttpRequestBuilder {
            version: (),
            host: (),
            method: (),
            path: (),
            head: Vec::new(),
            body: None,
        }
    }
}

impl HttpRequestBuilder<f32, String, String, String> {
    /// build HTTP request
    pub fn build(self) -> HttpRequest {
        let mut head = self.head.clone();
        head.push(HttpHeader {
            name: "Host".to_string(),
            content: self.host,
        });
        HttpRequest {
            version: self.version,
            method: self.method,
            path: self.path,
            head: HttpHeaders::new(self.head),
            body: self.body,
        }
    }
}

impl<VersionType, HostType, MethodType, PathType>
    HttpRequestBuilder<VersionType, HostType, MethodType, PathType>
{
    /// HTTP version, e.g., 1.1, 2.0
    pub fn version(self, version: f32) -> HttpRequestBuilder<f32, HostType, MethodType, PathType> {
        HttpRequestBuilder {
            version,
            host: self.host,
            method: self.method,
            path: self.path,
            head: self.head,
            body: self.body,
        }
    }

    /// request side host
    pub fn host<S: Into<String>>(
        self,
        host: S,
    ) -> HttpRequestBuilder<VersionType, String, MethodType, PathType> {
        HttpRequestBuilder {
            version: self.version,
            host: host.into(),
            method: self.method,
            path: self.path,
            head: self.head,
            body: self.body,
        }
    }

    /// request method
    pub fn method<S: Into<String>>(
        self,
        method: S,
    ) -> HttpRequestBuilder<VersionType, HostType, String, PathType> {
        HttpRequestBuilder {
            version: self.version,
            host: self.host,
            method: method.into(),
            path: self.path,
            head: self.head,
            body: self.body,
        }
    }

    /// request http path
    pub fn path<S: Into<String>>(
        self,
        path: S,
    ) -> HttpRequestBuilder<VersionType, HostType, MethodType, String> {
        HttpRequestBuilder {
            version: self.version,
            host: self.host,
            method: self.method,
            path: path.into(),
            head: self.head,
            body: self.body,
        }
    }

    /// HTTP-request contents
    pub fn body<S: Into<String>>(self, body: S) -> Self {
        let body = body.into();
        let body = body.into_bytes();
        HttpRequestBuilder {
            version: self.version,
            host: self.host,
            method: self.method,
            path: self.path,
            head: self.head,
            body: Some(body),
        }
    }

    /// HTTP-request header
    pub fn header(self, http_header: HttpHeader) -> Self {
        let mut head = self.head.clone();
        head.push(http_header);
        HttpRequestBuilder {
            version: self.version,
            host: self.host,
            method: self.method,
            path: self.path,
            head,
            body: self.body,
        }
    }
}

#[cfg(test)]
mod test_lib {
    use {HttpHeader, HttpHeaders, HttpRequest, HttpRequestBuilder, HttpResponse,
         HttpResponseBuilder};

    #[test]
    fn new_http_response() {
        let _: HttpResponse = HttpResponseBuilder::new()
            .version(2.0) // required
            .host("Host") // required
            .status_code(200)
            .status_string("OK")
            .header(HttpHeader {
                name: "Dog".to_string(),
                content: "Bow-wow".to_string(),
            })
            .build();
    }

    #[test]
    fn new_http_headers() {
        HttpHeaders::new(Vec::new());
    }

    #[test]
    fn new_http_request() {
        let _: HttpRequest = HttpRequestBuilder::new()
            .version(2.0) // required
            .host("Host") // required
            .method("GET") // required
            .path("/") // required
            .header(HttpHeader { // option
                name: "Neko".to_string(),
                content: "Meow".to_string(),
            })
            .build();
    }

}
