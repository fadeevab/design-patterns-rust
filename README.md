# Design Patterns in Rust

This repository contains **Rust** 🦀 examples for **all 23 classic GoF design
patterns**, and even a little more. It is developed to be a part of the [Refactoring.Guru](https://refactoring.guru/design-patterns) project.

![Refactoring.Guru](https://avatars.githubusercontent.com/u/8557932?s=200&v=4)

Each example is designed to introduce _a practical applicability_ in **Rust**, either
it's a conceptual or a real world example. The Rust ideology and
all the specifics are taken into account.

1. For instance, some patterns are easy to implement with Rust, mostly
   creational ones, e.g.
   [Prototype](creational/prototype),
   [Static Creation Method](creational/static-creation-method/).
2. The [Mediator](behavioral/mediator) behavioral pattern is hardest to
   implement with Rust, considering Rust's specific ownership model with a
   strict borrow checker rules.

You can find README.md in each example with instructions and additional explanation.

## Requirements

These examples have been tested with a _stable_ `rustc 1.62` (2021 edition).

All examples can be launched via the command line, using `cargo` as follows:

```
cargo run --bin adapter
```

## License

This work is licensed under a Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License.

<a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-nd/4.0/80x15.png" /></a>

## Credits

Authors: Alexander Fadeev ([@fadeevab](https://github.com/fadeevab)).
