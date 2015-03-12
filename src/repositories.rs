#[allow(unused_attributes)]
//extern crate "rustc-serialize" as rustc_serialize;

use rustc_serialize::json;
use client::Client;

pub struct RepositoryService {
    client: Client,
}

impl RepositoryService {

    pub fn new(c: Client) -> RepositoryService {
        RepositoryService { client: c }
    }

    pub fn by_user(self, username: &str) -> Vec<Repository> {
        let mut url = "https://api.github.com/users/".to_string();
        url.push_str(username);
        url.push_str("/repos");
        
        let req = self.client.request(url.as_slice());

        let repos: Vec<Repository> = json::decode(req.as_slice()).unwrap();

        return repos;
    }
}

#[allow(dead_code)]
#[derive(RustcDecodable)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
}

