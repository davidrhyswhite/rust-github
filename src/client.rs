extern crate curl;

use std::str;
use curl::http;
use std::collections::hash_map::HashMap;

#[derive(Copy, Clone)]
struct Header {
    name: &'static  str,
    value: &'static  str,
}

#[derive(Copy, Clone)]
pub struct Client {
    headers: Header,
}

impl Client {

    pub fn add_header(self, key: &str, value: &str) {

    }

    pub fn request(self, url: &str) -> String {
        let res = http::handle()
            .get(url)
            .header("User-Agent", "Rust-Github-Client")
            .exec().unwrap();

        let body = match str::from_utf8(res.get_body()) {
            Ok(b) => b,
            Err(..) => "Unable to parse"
        };

        return body.to_string();
    }
}
