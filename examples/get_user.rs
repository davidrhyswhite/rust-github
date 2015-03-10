extern crate github;

use github::Github;


fn main() {
    let github = Github::new();
    github.users.get("octocat");
}
