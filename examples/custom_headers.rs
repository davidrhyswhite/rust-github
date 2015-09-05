extern crate rust_github as github;

use github::Github;

fn main(){
    let github = Github::new()
        .header("X-Powered-By", "rust-curl");

    github.users.get("octocat");
}
