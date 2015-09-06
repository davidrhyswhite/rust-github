#![crate_name = "rust_github"]
#![crate_type = "rlib"]
//#![feature(core)]
extern crate curl;
extern crate rustc_serialize;

use client::Client;
use users::UserService;
use repositories::RepositoryService;
use std::collections::hash_map::HashMap;

pub struct Github {
    client: Client,
    pub users: UserService,
    pub repositories: RepositoryService,
}

impl Github {
    pub fn new() -> Github {
        let client = Client { headers: HashMap::new() };
        Github {
            client: client,
            users: UserService::new(client),
            repositories: RepositoryService::new(client),
        }
    }

    pub fn header(self, key: &str, value: &str) -> Github {
       self.client.add_header(key, value);
       return self;
    }
}

pub mod client;
pub mod users;
pub mod repositories;
