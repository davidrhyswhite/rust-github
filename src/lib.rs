#![crate_name = "github"]
#![crate_type = "rlib"]
#![feature(rustc_private, core)]
extern crate curl;
extern crate serialize;

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
