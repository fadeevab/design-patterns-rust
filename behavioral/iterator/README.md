# Iterator

Iterator is a behavioral design pattern that allows sequential traversal through a complex data structure without exposing its internal details.

## Standard Iterator

```rust
let array = &[1, 2, 3];
let iterator = array.iter();

// Traversal over each element of the vector.
iterator.for_each(|e| print!("{}, ", e));
```

## Custom Iterator

```rust
let users = UserCollection::new();
let mut iterator = users.iter();

iterator.next();
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
