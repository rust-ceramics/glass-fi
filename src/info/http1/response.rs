//! HTTP/1.x Response
use info::http1::header::{HttpHeader, HttpHeaders};

/// HTTP/1.x response which includes also body.
#[derive(Debug)]
pub struct HttpResponse {
    version: f32,
    status_code: usize,
    status_string: String,
    head: HttpHeaders,
    body: Option<Vec<u8>>,
}

/// Builder struct for HTTP/1.x response.
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
    /// new HTTP/1.x Response Builder
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
    /// build HTTP/1.x response
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
    /// HTTP/1.x version, e.g., 1.0, 1.1
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

    /// HTTP/1.x-response header
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

#[cfg(test)]
mod test_response {
    use info::http1::response::*;

    #[test]
    fn new_response() {
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

}
