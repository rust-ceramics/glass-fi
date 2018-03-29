//! HTTP/1.x Request
use info::http1::header::*;
/// HTTP/1.x request which includes also body.
#[derive(Debug)]
pub struct HttpRequest {
    version: f32,
    method: String,
    path: String,
    head: HttpHeaders,
    body: Option<Vec<u8>>,
}

/// Builder struct for HTTP/1.x request.
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
    /// new HTTP/1.x Request Builder
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
    /// HTTP/1.x version, e.g., 1.1, 2.0
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

    /// HTTP/1.x-request contents
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

    /// HTTP/1.x-request header
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
mod test_request {
    use info::http1::request::*;

    #[test]
    fn new_request() {
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
