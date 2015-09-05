#[allow(unused_attributes)]

use rustc_serialize::json;
use client::Client;

pub struct UserService {
    client: Client,
}

impl UserService {

    pub fn new(c: Client) -> UserService {
        UserService { client: c }
    }

    pub fn get(self, name: &str) -> User {
        let mut url = "https://api.github.com/users/".to_string();
        url.push_str(name);

        let req = self.client.request(&url);

        let user: User = json::decode(&req).unwrap();

        return user;
    }
}

#[allow(dead_code)]
#[derive(RustcDecodable)]
pub struct User {
    pub login: String,
    pub id: i32,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub site_admin: bool,
    pub name: String,
    pub company: String,
    pub blog: String,
    pub location: String,
    pub email: String,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub public_repos: i32,
    pub public_gists: i32,
    pub followers: i32,
    pub following: i32,
    pub created_at: String,
    pub updated_at: String,
}
