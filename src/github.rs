extern crate curl;

use std::str;
use curl::http;

pub struct Github;

impl Github {
    // Create an instance of Github
    pub fn new() -> Github { Github }

    pub fn user(self) {
        println!("Getting user octocat");

        let res = http::handle()
            .get("https://api.github.com/users/octocat")
            .header("User-Agent", "Rust-Github-Client")
            .exec().unwrap();
         
        let body = match str::from_utf8(res.get_body()) {
            Ok(b) => b,
            Err(..) => "Errors parsing body"
        };

        println!("Body: {:?}", body);
    }
}
