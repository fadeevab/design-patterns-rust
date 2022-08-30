# Factory Method

_**Factory Method** is a creational design pattern that provides an interface
for creating objects in a supertrait, but allows subtraits to alter the type
of objects that will be created._

## Maze Game

This example reproduces one from the GoF Design Patterns book:
https://en.wikipedia.org/wiki/Factory_method_pattern, implementing
the Factory Method pattern using generics (_static dispatch_).

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

This example shows a GUI framework can organize its types into
independent modules:

1. The `gui` module defines interfaces for all the components.
   It has no external dependencies.
2. The `html_gui` module provides HTML implementation of the base GUI.
   Depends on `gui`.
3. The `windows_gui` module provides Windows implementation of the base GUI.
   Depends on `gui`.

The app is a client application that can use several implementations
of the GUI framework, depending on the current environment or configuration.
However, most of the app code doesn't depend on specific types of GUI elements.
All the client code works with GUI elements through abstract interfaces
defined by the `gui` module.

💡 The [Abstract Factory example](../abstract-factory/) demonstrates even
greater separation of Factory interface and its implementations.

### How to Run

```bash
cargo run --bin factory-method-render-dialog
```

### Output

```
-- No OS detected, creating the HTML GUI --
<button>Test Button</button>
Click! Button says - 'Hello World!'
Dialog - Refresh
```

### Reference

This example reproduces [Factory Method Java Example](https://refactoring.guru/design-patterns/factory-method/java/example).
