[![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]

# dflake

A simple Discord snowflake parsing library. Comes with optional chrono support using the `chrono_support` feature.

## Sample Usage
```rust
use dflake::parse;

fn main() {
    let dflake = parse(3971046231244935168);
    println!("My internal process ID is: {}", dflake.process_id);
}
```

[ci]: https://github.com/Elinvynia/dflake/actions?query=workflow%3ARust
[ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/dflake/Rust/master?style=flat-square
[docs]: https://docs.rs/dflake
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
[crate-link]: https://crates.io/crates/dflake
[crate-version]: https://img.shields.io/crates/v/dflake.svg?style=flat-square
