# Chain of Responsibility

**Chain of Responsibility** is behavioral design pattern that allows passing
request along the chain of potential handlers until one of them handles request.

## Conceptual Example

The chain of responsibility is constructed as follows:

```
Patient -> Reception -> Doctor -> Medical -> Cashier
```

How to execute the example:

```bash
cargo run --bin chain-of-responsibility
```

## Execution Result

```
Reception registering a patient John
Doctor checking a patient John
Medical giving medicine to a patient John
Cashier getting money from a patient John

The patient has been already handled:

Patient registration is already done
A doctor checkup is already done
Medicine is already given to a patient
Payment done
```

## Reference

[Chain of Responsibility in Go (Example)](https://refactoring.guru/design-patterns/chain-of-responsibility/go/example) is used as a reference for this Rust example.
