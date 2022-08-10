# Memento

**Memento** allows making snapshots of an object’s state and restoring it in future.

## `conceptual.rs`

A conceptual example of Memento pattern.

However, in Rust, a storing and restoring objects can be done via `serde`
framework (see an example below).

```bash
cargo run --bin memento
```

Output:

```
Originator backup: '1'
Originator backup: '2'
Restored to state: 2
Restored to state: 1
```

## `serde.rs`

A common way to make a structure serializable is to derive `Serialize` and
`Deserialize` traits from `serde` crate. Then the type instance can be converted
to many different formats (e.g. JSON with `serde_json` crate).

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Originator {
    state: u32,
}
```

Test it:

```bash
cargo run --bin memento-serde
```

Output:

```
{"state":1}
{"state":2}
Restored to state: 2
Restored to state: 1
```
