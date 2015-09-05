extern crate rust_github as github;

use github::Github;

fn main() {
    let github = Github::new();
    let user = github.users.get("octocat");
    println!("Name: {:?}", user.name);
    println!("Email: {:?}", user.email);
    println!("Location: {:?}", user.location);
}
