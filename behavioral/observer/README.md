# Observer

_**Observer** is a behavioral design pattern that allows some objects to notify other objects about changes in their state._

**In Rust**, a convenient way to define a subscriber is to have **a function**
as a callable object with complex logic passing it to a event publisher.

In this Observer example, Subscribers are either **a lambda function** or
**an explicit function** subscribed to the event. Explicit function objects could be also unsubscribed (although, there could be limitations for some function types).

## How to Run

```bash
cargo run --bin observer
```

## Execution Result

```
Save log to /path/to/log/file.txt: Load file test1.txt
Save log to /path/to/log/file.txt: Load file test2.txt
Email to admin@example.com: Save file test2.txt
```

## Reference

[Observer in Java (Example)](https://refactoring.guru/design-patterns/observer/java/example)
