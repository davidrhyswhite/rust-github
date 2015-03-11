#![crate_name = "github"]
#![crate_type = "rlib"]
#![feature(rustc_private, core)]
extern crate curl;
extern crate serialize;

use client::Client;
use users::UserService;

pub struct Github {
    pub users: UserService,
}

impl Github {
    pub fn new() -> Github {
        let client = Client;
        Github { 
            users: UserService::new(client),
        }
    }
}

pub mod client;
pub mod users;
