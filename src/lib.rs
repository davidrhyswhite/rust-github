#![crate_name = "rust-github"]
#![crate_type = "rlib"]

extern crate hyper;


pub use github::Github;
mod github;

