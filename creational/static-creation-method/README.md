# Static Creation Method

_**Static Creation Method** is an
[associated function](https://doc.rust-lang.org/rust-by-example/fn/methods.html)
that returns a new object which is usually an instance of that particular type._

There is a notion of "constructor" in a typical OOP language which is a
default class method to create an object. Not in **Rust**: constructors are
_thrown away_ because there is nothing that couldn't be achieved with a static
creation method.

💡 See **[The Easiest Patterns in Rust](https://fadeevab.com/the-easiest-patterns-in-rust/)**.
See also [Factory Comparison](https://refactoring.guru/design-patterns/factory-comparison).

There are a few ways to define a static creation method.

1. A `default()` method from Default trait for construction with no parameters. Use either default `#[derive(Default)]`, or a manual trait implementation.

   ```rust
   #[derive(Default)]
   struct Circle;

   let circle = Circle::default();
   ```

2. A handwritten `new()` method for a custom object creation with parameters:

   ```rust
   impl Rectangle {
       pub fn new(width: u32, length: u32) -> Rectangle {
           Self { width, length }
       }
   }

   let rectangle = Rectangle::new(10, 20);
   ```

### How to Run

```bash
cargo run --bin static-creation-method
```

### Output

```
Alice Fisher
John Smith
```
