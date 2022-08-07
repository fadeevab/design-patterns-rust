# Visitor

A real-world example of Visitor pattern is [serde.rs](https://serde.rs) deserialization
model, see [Serde Data Model](https://serde.rs/data-model.html):

1. There is a `Visitor` implemented for a deserializable type.
2. A `Visitor` is passed to a `Deserializer` (an "Element" in terms of the Visitor Pattern), which accepts and drives the `Visitor` in order to construct a desired type.

_Visitor lets you add “external” operations to a whole class hierarchy without changing the existing code of these classes._

Let's implement a such synthetic deserializing example.

## Execution Result

```
Ok(TwoValuesStruct { a: 123, b: 456 })
Ok(TwoValuesStruct { a: 123, b: 456 })
Ok(TwoValuesArray { ab: [123, 456] })
Error: parse_str unimplemented
```