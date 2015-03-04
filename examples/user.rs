#![deny(warnings)]
extern crate "rust-github" as github;

use github::Github;

fn main() {
    let github = Github::new();
    github.user();
}
