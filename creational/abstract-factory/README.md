# Abstract Factory

_**Abstract Factory** solves the problem of creating entire product families
without specifying their concrete classes._

## GUI Factory Example

This example shows a GUI framework can organize its classes into independent
libraries:

1. The `gui` library defines interfaces for all the components.
   It has no external dependencies.
2. The `windows-gui` library provides Windows implementation of the base GUI.
   Depends on `gui`.
3. The `macos-gui` library provides Mac OS implementation of the base GUI.
   Depends on `gui`.

The `app` is a client application that can use several implementations of the
GUI framework, depending on the current environment or configuration.
However, most of the `app` code _**doesn't depend on specific types of GUI
elements**_. All the client code works with GUI elements through abstract
interfaces (traits) defined by the `gui` lib.

There are also 2 approaches to implementing abstract factory in Rust:
using generics (_static dispatch_) and using dynamic allocation
(_dynamic dispatch_).

## `app/main.rs`

Here, abstract factory is implemented via **generics** which allow the compiler
to create a code that does NOT require dynamic dispatch in runtime.

```rust
pub trait GuiFactory {
    type B: Button;
    type C: Checkbox;

    fn create_button(&self) -> Self::B;
    fn create_checkbox(&self) -> Self::C;
}
```

### How to Run

```bash
cargo run --bin abstract-factory
```

### Output

```
Windows button has pressed
Windows button has pressed
Windows checkbox has switched
Windows checkbox has switched
```

## `app-dyn/main.rs`

If a concrete type of abstract factory is not known at the compilation time,
then is should be implemented using `Box` pointers.

```rust
pub trait GuiFactoryDynamic {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}
```

### How to Run

```bash
cargo run --bin abstract-factory-dyn
```

### Output

```
MacOS button has pressed
MacOS button has pressed
MacOS button has pressed
MacOS checkbox has switched
MacOS checkbox has switched
```

## Reference

[Abstract Factory in Java (Example)](https://refactoring.guru/design-patterns/abstract-factory/java/example)
