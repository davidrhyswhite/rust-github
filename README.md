# Rust Github

Rust based library for interacting with the Github API. This is just a practice library while I learn how to write Rust libraries / applications.

## Examples

### Get User Information & Repositories

This request will return a single `github::users::User` struct.

```rust
extern crate rust_github;

use rust_github::Github;

fn main() {
    let github = Github::new();
    let user = github.users.get("sriharshakappala");
    let repositories = github.repositories.by_user("sriharshakappala");
    println!("Name: {:?}", user.name);
    println!("Email: {:?}", user.email);
    println!("Location: {:?}", user.location);
    for repo in repositories.iter() {
        println!("{:?}", repo.name);
    }
}
```
