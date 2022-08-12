# Builder

_**Builder** is a creational design pattern, which allows constructing of complex
objects step by step._

In this example, [`trait Builder`](builders/mod.rs) defines how to assemble
a car. However, depending on the builder implementation, a constructed object
can be either a car, or a car manual.

💡 The **Builder** design pattern is not the same as a **Fluent Interface** idiom,
although Rust developers sometimes use those terms interchangeably.

1. **Fluent Interface** idiom is a way to chain methods for constructing or
   modifying an object using the following approach:
   `let car = Car::default().places(5).gas(30)`.
   It's pretty useful for constructing an object. Still, it's not the Builder.
2. **Builder** is a pattern with a common building trait but with different
   building implementations. At the same time, Fluent Interface can be used
   together with the Builder pattern for a better code design.

## How to Run

```bash
cargo run --bin builder
```

## Output

```
Car built: SportsCar

Car manual built:
Type of car: CityCar
Count of seats: 2
Engine: volume - 1.2; mileage - 0
Transmission: Automatic
GPS Navigator: Functional
```

## Reference

This example is reproducing the [Builder Example in Java](https://refactoring.guru/design-patterns/builder/java/example).