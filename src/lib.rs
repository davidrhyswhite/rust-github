#![crate_name = "rust_github"]
#![crate_type = "rlib"]
extern crate curl;
extern crate rustc_serialize;

use client::Client;
use users::UserService;
use repositories::RepositoryService;

pub struct Github {
    pub users: UserService,
    pub repositories: RepositoryService,
}

impl Github {
    pub fn new() -> Github {
        let client = Client;
        Github {
            users: UserService::new(client),
            repositories: RepositoryService::new(client),
        }
    }
}

pub mod client;
pub mod users;
pub mod repositories;
