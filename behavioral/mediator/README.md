# Mediator Pattern

There is a research and discussion of the **Mediator Pattern in Rust**:
https://github.com/fadeevab/mediator-pattern-rust.

Top-Down Ownership approach allows to apply Mediator in Rust as it is
a suitable for Rust's ownership model with strict borrow checker rules. It's not
the only way to implement Mediator, but it's a fundamental one.

## How To Run

```bash
cargo run --bin mediator
```

## Execution Result

```
Passenger train Train 1: Arrived
Freight train Train 2: Arrival blocked, waiting
Passenger train Train 1: Leaving
Freight train Train 2: Arrived
Freight train Train 2: Leaving
'Train 3' is not on the station!
```

## Top-Down Ownership

The key point is thinking in terms of OWNERSHIP.

1. A mediator takes ownership of all components.
2. A component doesn't preserve a reference to a mediator. Instead, it gets the reference via a method call.

   ```rust
   // A train gets a mediator object by reference.
   pub trait Train {
       fn name(&self) -> &String;
       fn arrive(&mut self, mediator: &mut dyn Mediator);
       fn depart(&mut self, mediator: &mut dyn Mediator);
   }

   // Mediator has notification methods.
   pub trait Mediator {
       fn notify_about_arrival(&mut self, train_name: &str) -> bool;
       fn notify_about_departure(&mut self, train_name: &str);
   }
   ```

3. Control flow starts from `fn main()` where the mediator receives external events/commands.
4. `Mediator` trait for the interaction between components (`notify_about_arrival`, `notify_about_departure`) is not the same as its external API for receiving external events (`accept`, `depart` commands from the main loop).

   ```rust
   let train1 = PassengerTrain::new("Train 1");
   let train2 = FreightTrain::new("Train 2");

   // Station has `accept` and `depart` methods,
   // but it also implements `Mediator`.
   let mut station = TrainStation::default();

   // Station is taking ownership of the trains.
   station.accept(train1);
   station.accept(train2);

   // `train1` and `train2` have been moved inside,
   // but we can use train names to depart them.
   station.depart("Train 1");
   station.depart("Train 2");
   station.depart("Train 3");
   ```

![Top-Down Ownership](https://github.com/fadeevab/mediator-pattern-rust/raw/main/images/mediator-rust-approach.jpg)

## References

1. [Mediator Pattern in Rust](https://github.com/fadeevab/mediator-pattern-rust)
2. [Mediator in Go (Example)](https://refactoring.guru/design-patterns/mediator/go/example)
