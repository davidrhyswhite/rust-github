extern crate hyper;

use hyper::Client;

pub struct Github;

impl Github {
    // Create an instance of Github
    pub fn new() -> Github { Github }

    pub fn user(self) {
        println!("Getting user octocat");
        let mut client = Client::new();

        let mut res = match client.get("https://api.github.com/users/octocat").send() {
            Ok(res) => res,
            Err(err) => panic!("Failed to connect: {:?}", err)
        };        
        
        
        println!("Response: {:?}", res.status_raw());
    }
}
