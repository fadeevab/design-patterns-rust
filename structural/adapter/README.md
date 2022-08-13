# Adapter

_**Adapter** is a structural design pattern that allows objects with
incompatible interfaces to collaborate._

In this example, the `trait SpecificTarget` is incompatible with a `call`
function which accepts `trait Target` only.

```rust
fn call(target: impl Target);
```

The adapter helps to pass the incompatible interface to the `call` function.

```rust
let target = TargetAdapter::new(specific_target);
call(target);
```

## How to Run

```bash
cargo run --bin adapter
```

## Execution Result

```
A compatible target can be directly called: 'Ordinary request.'
Adaptee is incompatible with client: '.tseuqer cificepS'
But with adapter client can call its method: 'Specific request.'
```
