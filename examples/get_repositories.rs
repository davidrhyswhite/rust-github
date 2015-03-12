extern crate github;

use github::Github;

fn main() {
    let github = Github::new();
    let repositories = github.repositories.by_user("octocat");
    for repo in repositories.iter() {
        println!("{:?}", repo.name);
    }
}
