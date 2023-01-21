# Singleton

_**Singleton** lets you ensure that only one object of its kind exists,
while providing a global access point to this instance._

Singleton is a _global mutable object_, and in terms of **Rust**
it is a _`static mut` item_ which in turn
[requires an **`unsafe`** block](https://doc.rust-lang.org/reference/items/static-items.html#mutable-statics)
for either reading or writing a mutable static variable.

For this reason, the Singleton pattern can be perceived as unsafe. However,
the pattern is still widely used in practice. A good read-world example of
Singleton is a `log` crate that introduces `log!`, `debug!` and other logging
macros, which you can use throughout your code after setting up a concrete
logger instance, such as [env_logger](https://crates.io/crates/env_logger).
As we can see, `env_logger` uses
[log::set_boxed_logger](https://docs.rs/log/latest/log/fn.set_boxed_logger.html)
under the hood, which has an unsafe block to set up a global logger object.

- In order to provide safe and usable access to a singleton object,
  introduce an API hiding unsafe blocks under the hood.
- See the thread about a mutable Singleton on Stackoverflow for more information.

There is a plenty of useful containers that allows to avoid an `unsafe` block
in your code:

1. [once_cell::sync::OnceCell](https://docs.rs/once_cell/latest/once_cell/sync/struct.OnceCell.html)
2. [lazy_static::lazy_static](https://docs.rs/lazy_static/latest/lazy_static)
3. [std::sync::Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)

In a general case, you can start with `OnceCell` like in the `once.rs` example
(see below).

## `local.rs`

A safe way to implement Singleton in Rust is using NO global variables
at all and passing everything around through function arguments.
The oldest living variable is an object created at the start of the `main()`.

### How to Run

```bash
cargo run --bin singleton-local
```

### Output

```
Final state: 1
```

## `lazy.rs`

This is a singleton implementation via `lazy_static!`.

`lazy-static` allows declaring a static variable with lazy initialization
at first access. A drawback of `lazy_static!` is that it doesn't allow
initialization at the arbitrary code place, only in the static block
with predefined instructions.

### How to Run

```bash
cargo run --bin singleton-lazy
```

### Output

```
Called 3
```

## `once.rs`

`OnceCell` allows having a custom initialization of a singleton at an
**arbitrary place**, unlike `lazy_static!`, where the initialization must be
placed in a static block. `Mutex` is still needed there to make an actual object
modifiable without an `unsafe` block.

A [`logger`](../logger/), a practical example of Singleton, is
implemented via `OnceCell`.

### How to Run

```bash
cargo run --bin singleton-once
```

### Output

```
[42, 1, 1, 1]
```

## `mutex.rs`

⚠ Starting with `rustc 1.63`.

> Starting with `Rust 1.63`, it can be easier to work with global mutable
> singletons, although it's still preferable to avoid global variables in most
> cases.
>
> Now that `Mutex::new` is `const`, you can use global static `Mutex` locks
> without needing lazy initialization.

```rust
use std::sync::Mutex;

static GLOBAL_DATA: Mutex<Vec<i32>> = Mutex::new(Vec::new());
```

### How to Run

```bash
cargo run --bin singleton-mutex
```

### Output

```
Called 3 times: [1, 1, 1]
New singleton object: [3, 4, 5]
```
