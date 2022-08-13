# Bridge

_**Bridge** divides business logic or huge type into separate type hierarchies
that can be developed independently._

This example reproduces a [Bridge Example in Java](https://refactoring.guru/design-patterns/bridge/java/example).
It shows separation between the types of **remotes** and **devices**
that they control.

Remotes act as abstractions, and devices are their implementations.
Thanks to the common interfaces, the same remotes can work with different
devices and vice versa.

The Bridge pattern allows changing or even creating new classes without
touching the code of the opposite hierarchy.

## How to Run

```bash
cargo run --bin bridge
```

## Output

```
Tests with basic remote.
Remote: power toggle
------------------------------------
| I'm TV set.
| I'm enabled
| Current volume is 30%
| Current channel is 1
------------------------------------

Tests with advanced remote.
Remote: power toggle
Remote: mute
------------------------------------
| I'm TV set.
| I'm enabled
| Current volume is 0%
| Current channel is 1
------------------------------------

Tests with basic remote.
Remote: power toggle
------------------------------------
| I'm radio.
| I'm enabled
| Current volume is 30%
| Current channel is 1
------------------------------------

Tests with advanced remote.
Remote: power toggle
Remote: mute
------------------------------------
| I'm radio.
| I'm enabled
| Current volume is 0%
| Current channel is 1
------------------------------------
```
