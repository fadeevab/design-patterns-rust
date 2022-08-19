# Chain of Responsibility

_**Chain of Responsibility** is behavioral design pattern that allows passing
request along the chain of potential handlers until one of them handles request._

## Conceptual Example

The example demonstrates processing a patient through a chain of departments.
The chain of responsibility is constructed as follows:

```
Patient -> Reception -> Doctor -> Medical -> Cashier
```

💡 The chain is constructed using `Box` pointers, which means dynamic dispatch
in runtime. **Why?** It seems quite difficult to narrow down implementation
to a strict compile-time typing using generics: in order to construct a type
of a full chain Rust needs full knowledge of the "next of the next" link in the
chain. Thus, it ***would*** look like this:

```rust
let mut reception = Reception::<Doctor::<Medical::<Cashier>>>::new(doctor); // 😱
```

Instead, `Box` allows chaining in any combination:

```rust
let mut reception = Reception::new(doctor); // 👍

let mut reception = Reception::new(cashier); // 🕵️‍♀️
```

## How to Execute

```bash
cargo run --bin chain-of-responsibility
```

## Execution Result

```
Reception registering a patient John
Doctor checking a patient John
Medical giving medicine to a patient John
Cashier getting money from a patient John

The patient has been already handled:

Patient registration is already done
A doctor checkup is already done
Medicine is already given to a patient
Payment done
```

## Reference

[Chain of Responsibility in Go (Example)](https://refactoring.guru/design-patterns/chain-of-responsibility/go/example) is used as a reference for this Rust example.
