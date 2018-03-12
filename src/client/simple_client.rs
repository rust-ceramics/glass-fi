#![deny(missing_docs)]

use url::{self, Url, Host};
use std::error;
use std::fmt;
use std::convert;

#[derive(Debug)]
struct HttpBody {
    text: String
}

#[derive(Debug)]
struct HttpResponse {
    body: HttpBody,
}

impl HttpResponse {
    fn new<S: Into<String>>(body_text: S) -> Self {
        HttpResponse {
            body: HttpBody {
                text: body_text.into()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum HttpResponseError {
    NotHttpScheme,
    ParseURLError(url::ParseError),
}

impl fmt::Display for HttpResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpResponseError::NotHttpScheme => write!(f, "Not HTTP Scheme: input string hasn't http scheme"),
            HttpResponseError::ParseURLError(ref err) => write!(f, "Parse URL Error: {}", err)
        }
    }

}

impl error::Error for HttpResponseError {
    fn description(&self) -> &str {
        match *self {
            HttpResponseError::NotHttpScheme => "This hasn't http scheme",
            HttpResponseError::ParseURLError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            HttpResponseError::NotHttpScheme => Some(&HttpResponseError::NotHttpScheme),
            HttpResponseError::ParseURLError(ref err) => Some(err),
        }
    }
}

impl convert::From<url::ParseError> for HttpResponseError {
    fn from(err: url::ParseError) -> HttpResponseError {
        HttpResponseError::ParseURLError(err)
    }
}

struct SimpleClient {}

impl SimpleClient {
    fn new() -> Self {
        SimpleClient{}
    }

    fn get<S: Into<String>>(&self, url: S) -> Result<HttpResponse, HttpResponseError> {
        let issue_list_url = Url::parse(&url.into())?;
        if issue_list_url.scheme() != "http" {
            return Err(HttpResponseError::NotHttpScheme)
        }
        if issue_list_url.port() == None {
            Ok(HttpResponse::new("Hello World!"))
        }
        else {
            Ok(HttpResponse::new("Hello World?"))
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
