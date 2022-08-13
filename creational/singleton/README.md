# Singleton

_**Singleton** lets you ensure that only one object of its kind exists,
while providing a global access point to this instance._

💡 Now, Singleton is a _global mutable object_, and in terms of **Rust**
it is a _`static mut` item_ which in turn means
[it requires an **`unsafe`** block](https://doc.rust-lang.org/reference/items/static-items.html#mutable-statics)
either reading or writing a mutable static variable.

On one side, it can be treated as an unsafe pattern, however,
on the other side, Singleton is used in Rust on practice. A good **read-world**
example of Singleton is `log` crate that introduces `log!`, `debug!` and other
logging macro which you can use throughout your code **after** setting up a concrete
logger instance, e.g. [env_logger](https://crates.io/crates/env_logger).
(`env_logger` uses
[log::set_boxed_logger](https://docs.rs/log/latest/log/fn.set_boxed_logger.html)
under the hood, which has an `unsafe` block to set up a global logger object).

## `safe.rs`

A pure safe way to implement Singleton in Rust is using no global variables
at all and passing everything around through function arguments.
The oldest living variable is an object created at the start of the `main()`.

### How to Run

```bash
cargo run --bin singleton
```

### Output

```
Final state: 1
```

## `lazy.rs`

This is a singleton implementation via `lazy_static!`.

`lazy-static` allows declaring a static variable with lazy initialization
at first access. It is actually implemented via `unsafe` with `static mut`
manipulation, however, it keeps your code clear of `unsafe` blocks.

### How to Run

```bash
cargo run --bin singleton-lazy
```

### Output

```
Called 3
```

## Notes

In order to provide safe and usable access to a singleton object,
introduce an API hiding `unsafe` blocks under the hood.
