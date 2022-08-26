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
cargo run --bin factory-method-maze-game
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

This example shows a GUI framework can organize its classes into independent libraries:
 
1. The `gui` library defines interfaces for all the components. It has no external dependencies.
2. The `html_gui` library provides HTML implementation of the base GUI. Depends on `gui`.
3. The `windows_gui` library provides Windows implementation of the base GUI. Depends on `gui`.

The `app` is a client application that can use several implementations of the GUI framework, depending on the current environment or configuration. However, most of the `app` code doesn't depend on specific types of GUI elements. All the client code works with GUI elements through abstract interfaces defined by the `gui` lib.

### How to Run

```bash
cargo run --bin factory-method-render-dialog
```

### Output

```
<button>Test Button</button>
Click! Button says - 'Hello World!'
Dialog - Refresh
```
