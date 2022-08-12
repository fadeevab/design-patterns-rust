# Prototype

_**Prototype** allows cloning objects, even complex ones, without coupling to their specific classes._

**_Rust_** has standard `Clone` implementation (via `#[derive(Clone)]`) for many
types which makes Prototype easy and seamless to use.

```rust
let mut circle2 = circle1.clone();
```

See **[The Easiest Patterns in Rust](https://fadeevab.com/the-easiest-patterns-in-rust/)**.

## How to Execute

```bash
cargo run --bin prototype
```

## Output

```
Circle 1: 10, 15, 10
Circle 2: 10, 15, 77
```
