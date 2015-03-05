#![crate_name = "rust-github"]
#![crate_type = "rlib"]

extern crate curl;


pub use github::Github;
mod github;

