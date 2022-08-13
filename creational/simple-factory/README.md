# Simple Factory

_The **Simple Factory** pattern is just a function with a large conditional
that based on parameters chooses which product to instantiate and then return._

The idea is encapsulating a creation logic inside of a method, while
object utilization is being kept "outside". There is no inheritance and
complex creational traits, thus it's the **Simple** factory.

Here, `create_button` creates random buttons:

```rust
fn create_button(random_number: f64) -> Box<dyn Button> {
    if random_number < 0.5 {
        Box::new(TitleButton::new("Button".to_string()))
    } else {
        Box::new(IdButton::new(123))
    }
}
```

while `render_dialog` manipulates with whatever it gets from `create_button`:

```rust
fn render_dialog(random_number: f64) {
    // ...
    let button = create_button(random_number);
    button.render();
    // ...
}
```

## How to Run

```bash
cargo run --bin simple-factory
```

## Output

```
-- Title ---
'Button'
-------------
--- Title ---
Button #123
-------------
```
