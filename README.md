# Rust Github

Rust based library for interacting with the Github API. This is just a practice library while I learn how to write Rust libraries / applications.

## Examples

### Get a user

This request will return a single `github::users::User` struct.

```rust
extern crate rust_github as github;

use github::Github;


fn main() {
    let github = Github::new();
    let user = github.users.get("octocat");
    println!("Name: {:?}", user.name);
    println!("Email: {:?}", user.email);
    println!("Location: {:?}", user.location);
}
```

### Get all repositories by user

Get a list of repositories by user, exposes a `Vec<github::repositories::Repository>`.

```rust
let github = Github::new();
let repositories = github.repositories.by_user("octocat");
for repo in repositories.iter() {
    println!("{:?}", repo.name);
}
```
