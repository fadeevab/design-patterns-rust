# Command

_**Command** is behavioral design pattern that converts requests or
simple operations into objects._

In 🦀 Rust, a command instance should _NOT hold a permanent reference to global
context_, instead the latter should be passed _from top to down as a mutable
parameter_ of the "`execute`" method:

```rust
fn execute(&mut self, app: &mut cursive::Cursive) -> bool;
```

## Text Editor: Commands and Undo

How to launch:

```bash
cargo run --bin command
```

Key points:

- Each button runs a separate command.
- Because a command is represented as an object, it can be pushed into a
  `history` array in order to be undone later.
- TUI is created with `cursive` crate.

![Text Editor screenshot](res/editor.png)

## Reference

This example is inspired by [Command in Java (Example)](https://refactoring.guru/design-patterns/command/java/example).
