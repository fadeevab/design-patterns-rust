# Flyweight

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