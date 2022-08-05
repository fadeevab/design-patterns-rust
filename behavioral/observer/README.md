# Observer

In Rust, the convenient way to define a listener is to use a function as a callable object with complex logic.

In this Observer example, Subscribers are either lambda functions or explicit functions subscribed to the event. Explicit function objects could be also unsubscribed (although, there could be limitations for some function types).

```bash
cargo run --bin observer
```

## Execution Result

```
Save log to /path/to/log/file.txt: Load file test1.txt
Save log to /path/to/log/file.txt: Load file test2.txt
Email to admin@example.com: Save file test2.txt
```
