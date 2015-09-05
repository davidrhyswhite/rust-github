#[allow(unused_attributes)]

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

        let req = self.client.request(&url);

        let repos: Vec<Repository> = json::decode(&req).unwrap();

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
