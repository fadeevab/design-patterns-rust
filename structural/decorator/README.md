# Decorator

_**Decorator** is a structural pattern that allows adding new behaviors
to objects by placing them inside special wrapper objects, called decorators._

There is a **_practical example_** in Rust's standard library for input/output
operations.

A buffered reader decorates a vector reader adding buffered behavior.

```rust
let mut input = BufReader::new(Cursor::new("Input data"));
input.read(&mut buf).ok();
```

## How to Run

```bash
cargo run --bin decorator
```

## Output

```
Read from a buffered reader: Input data
```
