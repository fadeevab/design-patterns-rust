# Mediator Pattern

There is a research and discussion of the Mediator pattern in Rust: https://github.com/fadeevab/mediator-pattern-rust.

"Top-Down Ownership" approach allows to apply Mediator in Rust as it is a suitable for Rust's ownership model with strict borrow checker rules. It's not the only way to implement Mediator, but it's a fundamental one.

## Top-Down Ownership

The key point is thinking in terms of OWNERSHIP.

1. A mediator takes ownership of all components.
2. A component doesn't preserve a reference to a mediator. Instead, it gets the reference via a method call.
3. Control flow starts from the `fn main()` where the mediator receives external events/commands.
4. Mediator trait for the interaction between components is not the same as its external API for receiving external events (commands from the main loop).
