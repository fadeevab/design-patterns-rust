# Design Patterns in Rust

This repository contains **Rust** 🦀 examples for **all 23 classic GoF design
patterns**, and even a little more.

All examples are designed to introduce _practical applicability_ in the
**Rust** language. There are _conceptual_ and _real-world_ examples.
In both cases, Rust idiomatic ways of code development and all the specifics
are taken into account.

The repository is developed to be a part of the
[Refactoring.Guru](https://refactoring.guru/design-patterns) project.

<img align="left" width="200" height="200" src="https://avatars.githubusercontent.com/u/8557932?s=200&v=4" />
<img width="200" height="200" src="https://www.rust-lang.org/logos/rust-logo-512x512.png" style="padding-left:20px" />

## 🔧 Requirements

These examples have been tested with a _stable_ `rustc 1.62` (2021 edition).

All examples can be launched via the command line, using `cargo` as follows:

```bash
cargo run --bin adapter
```

You can find a proper target in `Cargo.toml` of each example:

```toml
[[bin]]
name = "adapter"
path = "main.rs"
```

Each example contains a **README.md** with instructions and additional
explanations.

## 💡 Overview

Interestingly, in Rust:

1. Almost all **structural** and **creational** patterns can be implemented
   using generics, hence, _static dispatch_.
2. Most **behavioral** patterns can NOT be implemented using static dispatch,
   instead, they can be implemented only via _dynamic dispatch_.

A well-thought pattern classification fits the Rust language design perfectly
as "behavior" is dynamic in nature and "structure" is static.

Some patterns are really easy to implement in Rust, mostly
_creational_ ones, e.g.
[Prototype](creational/prototype),
[Static Creation Method](creational/static-creation-method/).

The [Mediator](behavioral/mediator) _behavioral_ pattern
is the hardest to implement with Rust, considering Rust's specific ownership
model with strict borrow checker rules.

## License

This work is licensed under a Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License.

<a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-nd/4.0/80x15.png" /></a>

## Credits

Authors: Alexander Fadeev ([@fadeevab](https://github.com/fadeevab)).
