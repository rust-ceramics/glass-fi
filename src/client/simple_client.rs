#![deny(missing_docs)]

#[derive(Debug)]
struct HttpBody {
    text: String
}

#[derive(Debug)]
struct HttpResponse {
    body: HttpBody,
}

impl HttpResponse {
    fn new() -> Self {
        HttpResponse {
            body: HttpBody {
                text: "Hello World!".to_string()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum HttpResponseError {}

struct SimpleClient {}

impl SimpleClient {
    fn new() -> Self {
        SimpleClient{}
    }

    fn get<S: Into<String>>(&self, url: S) -> Result<HttpResponse, HttpResponseError> {
        Ok(HttpResponse::new())
    }
}

#[test]
fn simple_get_http() {
    let client = SimpleClient::new();
    let response = client.get("http://127.0.0.1/").unwrap();
    let body_text = response.body.text;
    assert_eq!("Hello World!", body_text)
}
