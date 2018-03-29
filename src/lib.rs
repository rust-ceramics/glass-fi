#![deny(missing_docs)]
//! glass-fi
extern crate tokio;
extern crate url;
#[macro_use]
extern crate futures;

pub mod net;
pub mod info;

use info::http1::header::{HttpHeader, HttpHeaders};
#[cfg(test)]
mod test_lib {
    use info::http1::header::{HttpHeader, HttpHeaders};

    #[test]
    fn new_http_headers() {
        HttpHeaders::new(Vec::new());
    }

}
