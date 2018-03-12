struct SimpleClient {}

impl SimpleClient {
    fn new() -> Self {
        SimpleClient{}
    }

    fn get<S: Into<String>>(&self, url: S) {}
}

#[test]
fn simple_get_http() {
    let client = SimpleClient::new();
    client.get("http://127.0.0.1/");
}
