use client::Client;

pub struct UserService {
    pub client: Client,
}

impl UserService {

    pub fn get(self, name: &str) {
        let mut url = "https://api.github.com/users/".to_string();
        url.push_str(name);
        
        println!("{:?}", url.as_slice());
    }
}
