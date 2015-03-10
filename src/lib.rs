#![crate_name = "github"]
#![crate_type = "rlib"]

// extern crate curl;

use client::Client;
use user::UserService;


pub struct Github {
    pub users: UserService,
}

impl Github {
    pub fn new() -> Github {
        let client = Client;
        Github { users: UserService { client: client } }
    }
}

mod client;
mod user;
