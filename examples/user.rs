#![deny(warnings)]
extern crate github;

use github::Github;

fn main() {
    let github = Github::new();
    github.user("octocat");
}
