#![deny(warnings)]
extern crate github;

use github::Github;


fn main() {
    let g = Github::new();
    g.users.get("octocat");


}
