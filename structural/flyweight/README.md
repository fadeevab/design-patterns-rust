# Flyweight

_**Flyweight** is a structural design pattern that allows programs to support
ast quantities of objects by keeping their memory consumption low._

The pattern achieves it by sharing parts of object state between multiple
objects.

## Rendering a Forest

```bash
cargo run --release
```

## Screenshot

_res/forest.svg_ (10,000 trees):

![Rendered Forest](res/forest.svg)

## RAM usage stats

For 100,000 trees:

```
100000 trees drawn
---------------------
Memory usage:
Tree size (16 bytes) * 100000
+ TreeKind size (~30 bytes) * 2
---------------------
Total: 1MB (instead of 4MB)
```

Flyweight is implemented via a `HashMap` that holds only one copy
of a `TreeKind` for each `Tree` inside of the `Forest` structure.

```rust
#[derive(Default)]
pub struct Forest {
    pub tree_kinds: HashMap<String, Rc<TreeKind>>,
    pub trees: Vec<Tree>,
}
```

## Reference

The example reproduces a [Flyweight Example in Java (Rendering a Forest)](https://refactoring.guru/design-patterns/flyweight/java/example).
