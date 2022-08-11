# Strategy

_**Strategy** turns a set of behaviors into objects and makes them
interchangeable inside original context object._

## `conceptual.rs`

A conceptual Strategy example via traits.

```bash
cargo run --bin strategy
```

Output:

```
Walking route from Home to Club: 4 km, 30 min
Walking route from Club to Work: 4 km, 30 min
Public transport route from Home to Club: 3 km, 5 min
Public transport route from Club to Work: 3 km, 5 min
```

## 🦀 `functional.rs`

Functions and closures simplify Strategy implementation as you can
inject behavior right into the object without complex interface definition.

```bash
cargo run --bin strategy-func
```

It seems that Strategy is often implicitly and widely used in the modern
development with Rust, e.g. it's just like iterators work:

```rust
let a = [0i32, 1, 2];

let mut iter = a.iter().filter(|x| x.is_positive());
```

Output:

```
Walking route from Home to Club: 4 km, 30 min
Walking route from Club to Work: 4 km, 30 min
Public transport route from Home to Club: 3 km, 5 min
Public transport route from Club to Work: 3 km, 5 min
Specific route from Home to Club
Specific route from Club to Work
```
