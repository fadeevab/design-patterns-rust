# Iterator

_**Iterator** is a behavioral design pattern that allows sequential traversal
through a complex data structure without exposing its internal details._

## Standard Iterator

Iterators are heavily used in idiomatic 🦀 Rust code. The first example is
how to use iterators over a standard array collection.

```rust
let array = &[1, 2, 3];
let iterator = array.iter();

// Traversal over each element of the vector.
iterator.for_each(|e| print!("{}, ", e));
```

## Custom Iterator

In Rust, the recommended way to define your _custom_ iterator is to use a
standard `Iterator` trait. The example doesn't contain a synthetic iterator
interface, because it is really recommended to use the idiomatic Rust way.

```rust
let users = UserCollection::new();
let mut iterator = users.iter();

iterator.next();
```

A `next` method is the only `Iterator` trait method which is mandatory to be
implemented. It makes accessible a huge range of standard methods,
e.g. `fold`, `map`, `for_each`.

```rust
impl Iterator for UserIterator<'_> {
    fn next(&mut self) -> Option<Self::Item>;
}
```

## How to Run

```bash
cargo run --bin iterator
```

## Execution Result

```
Iterators are widely used in the standard library: 1 2 3

Let's test our own iterator.

1nd element: Some("Alice")
2nd element: Some("Bob")
3rd element: Some("Carl")
4th element: None


All elements in user collection: Alice Bob Carl
```
