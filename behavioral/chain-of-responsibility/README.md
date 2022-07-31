# Chain of Responsibility

## Conceptual Example

The chain of responsibility is constructed as the following:

```
Patient -> Reception -> Doctor -> Medical -> Cashier
```

How to execute the example:

```bash
cargo run
```

## Execution Result

```
Reception registering a patient John
Doctor checking a patient John
Medical giving medicine to a patient John
Cashier getting money from a patient John

The patient has been already handled:

Patient registration already done
Doctor checkup has been already done
Medicine has been already given to a patient
Payment done
```