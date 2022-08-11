# State

The **State** pattern reminds a finite-state machine, but each state
is represented by a separate trait (with a common supertrait) and there are
transitions between these state traits.

The State Pattern is described in detail in _The Rust Book_:
https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

A state trait methods should be defined via a special notation `self: Box<Self>`:

```rust
pub trait State {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State>;
}
```

1. First, `self` is not a reference, it means that the method is a "one shot",
   it consumes `self` and exchanges onto another state returning `Box<dyn State>`.
2. Second, the method consumes the boxed object like `Box<dyn State>` and
   not an object of a concrete type like `PlayingState`, because the concrete
   state is unknown at compile time.

See trait description in the [`state.rs`](state.rs) for more details.

## Run

```bash
cargo run --bin state
```

Press buttons, ESC for exit, enjoy!

## Screenshots

|                                |                                |
| ------------------------------ | ------------------------------ |
| ![Stopped](images/stopped.png) | ![Playing](images/playing.png) |

## Reference

[State Pattern in Java (Example)](https://refactoring.guru/design-patterns/state/java/example)
