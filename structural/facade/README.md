# Facade

_**Facade** is a structural design pattern that provides a simplified
(but limited) interface to a complex system of classes, library or framework._

## Conceptual Example

`pub struct WalletFacade` hides a complex logic behind its API. A single method
`add_money_to_wallet` interacts with the account, code, wallet, notification
and ledger behind the scenes.

## How to Run

```bash
cargo run --bin facade
```

## Execution Result

```
Starting create account
Account created

Starting add money to wallet
Account verified
Security code verified
Sending wallet credit notification
Make ledger entry for accountId abc with transaction type credit for amount 10

Starting debit money from wallet
Account verified
Security code verified
Sending wallet debit notification
Make ledger entry for accountId abc with transaction type debit for amount 5
```

## Reference

This examples reproduces the [Facade Example in Go](https://refactoring.guru/design-patterns/facade/go/example).
