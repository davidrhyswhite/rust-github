extern crate "rust-github" as github;

use github::Github;

fn main() {
    let github = Github::new();
    let repositories = github.repositories.by_user("davidrhyswhite");
    for repo in repositories.iter() {
        println!("{:?}", repo.name);
    }
}
