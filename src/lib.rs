#![crate_name = "rust-github"]
#![crate_type = "rlib"]
#![feature(core)]
extern crate curl;
extern crate "rustc-serialize" as rustc_serialize;

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
