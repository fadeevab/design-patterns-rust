# State

State Pattern reminds a finite-state machine, but each state is represented by
a separate trait (with a common supertrait) and there are transitions between these
state traits.

The State Pattern is described in detail in The Rust Book:
https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

## Run

```bash
cargo run --bin state
```

Press buttons, ESC for exit, enjoy!

## Screenshots

|                                |                                |
| ------------------------------ | ------------------------------ |
| ![Stopped](images/stopped.png) | ![Playing](images/playing.png) |
