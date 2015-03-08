# Rust Github

Rust based library for interacting with the Github API. This is just a practice library while I learn how to write Rust libraries / applications.

## Examples

### Get a user

```rust
extern crate github;

use github::Github;

fn main() {
    let github = Github::new();
    github.user("octocat");
}
```
