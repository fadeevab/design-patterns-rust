# Factory Method

_**Factory Method** is a creational design pattern that provides an interface
for creating objects in a supertrait, but allows subtraits to alter the type
of objects that will be created._

## Maze Game

Implementing the Factory Method pattern using **static dispatch** (generics).

This example reproduces one from the GoF Design Patterns book:
https://en.wikipedia.org/wiki/Factory_method_pattern

### How to Run

```bash
cargo run --bin maze-game
```

### Output

```
Loading resources...
Starting the game...
Magic Room: Infinite Room
Magic Room: Red Room
Loading resources...
Starting the game...
Ordinary Room: #2
Ordinary Room: #1
```

## Render Dialog

Implementing the Factory Method via **dynamic dispatch**.

See [Factory Method Java Example](https://refactoring.guru/design-patterns/factory-method/java/example)
as a reference.

### How to Run

```bash
cargo run --bin render-dialog
```

### Output

```
<button>Test Button</button>
Click! Button says - 'Hello World!'
Dialog - Refresh
```
