# Visitor

_**Visitor** lets you add “external” operations to a whole class hierarchy
without changing the existing code of these classes._

A real-world example of the Visitor pattern is [serde.rs](https://serde.rs) deserialization
model, see [Serde Data Model](https://serde.rs/data-model.html):

1. `Visitor` should be implemented for a deserializable type.
2. `Visitor` is passed to a `Deserializer` (an "Element" in terms of the Visitor Pattern), which accepts and drives the `Visitor` in order to construct a desired type.

Let's reproduce this deserializing model in our example.

## How to Run

```bash
cargo run --bin visitor
```

## Execution Result

```
Ok(TwoValuesStruct { a: 123, b: 456 })
Ok(TwoValuesStruct { a: 123, b: 456 })
Ok(TwoValuesArray { ab: [123, 456] })
Error: parse_str unimplemented
```
