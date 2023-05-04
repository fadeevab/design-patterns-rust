# Mediator with `Rc<RefCell<..>>`

## How To Run

```bash
cargo run --bin mediator-rc-refcell
```

## Mimicking a Typical OOP

`Rc<RefCell<..>>` hides objects from compiler eyes inside of an opaque smart pointer.
In this case, borrow checks move into the runtime that means panicking in case of
borrow rules violation.

There is an example of a [Station Manager example in Go][4]. Trying to make it
with Rust leads to mimicking a typical OOP through reference counting and
borrow checking with mutability in runtime (which has quite unpredictable
behavior in runtime with panics here and there).

Key points:

1. All methods are read-only: immutable `self` and parameters.
2. `Rc`, `RefCell` are extensively used under the hood to take responsibility for the mutable borrowing from compilation time to runtime. Invalid implementation will lead to panic in runtime.

See the full article: [README.md](../README.md).

[4]: https://refactoring.guru/design-patterns/mediator/go/example