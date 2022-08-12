# Abstract Factory

_**Abstract Factory** solves the problem of creating entire product families
without specifying their concrete classes._

## GUI Factory Example

There is a GUI Factory trait and two implementations: Windows and Mac OS factories. A factory can create buttons and checkboxes, however, depending on
thr factory subtype, it will create either Windows or Mac OS components.

There are also 2 approaches to implementing abstract factory in Rust:
using generics and using dynamic allocation.

## `main.rs`

Here, abstract factory is implemented via **generics** which allow the compiler
to create a code that doesn't require dynamic dispatch in runtime.

See `GuiFactory` in [gui/mod.rs](gui/mod.rs).

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

## `main_dyn.rs`

If a concrete type of abstract factory is not known at the compilation time,
then is should be implemented using `Box` pointers.

See See `GuiFactoryDynamic` in [gui/mod.rs](gui/mod.rs).

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
