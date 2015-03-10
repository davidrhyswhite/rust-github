use client::Client;

pub struct UserService {
    client: Client,
}

impl UserService {

    pub fn new(c: Client) -> UserService {
        UserService { client: c }
    }

    pub fn get(self, name: &str) {
        let mut url = "https://api.github.com/users/".to_string();
        url.push_str(name);
        
        self.client.request(url.as_slice());
    }
}
