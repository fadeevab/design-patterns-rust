# Logger

A practical example of how the Rust logging subsystem is essentially implemented using the Singleton pattern.

- [`log.rs`](./log.rs) - a lightweight logging facade encapsulating a static
  logger object (singleton); the module is a representation of
  the [`log`](https://docs.rs/log/latest/log/) crate,
- [`simple_logger.rs`](./simple_logger.rs) - a logger implementation printing
  into _stdout_, it represents the [`simplelog`](https://docs.rs/simplelog/latest/simplelog/)
  crate,
- [`app.rs`](./app.rs) - an arbitrary code using a logging facade,
- [`main.rs`](./main.rs) - application initialization

## How to Run

```bash
cargo run --bin singleton-logger
```

## Output

```bash
[I] Application starts
[W] Something non-critical happened
[E] Execution failed
```
