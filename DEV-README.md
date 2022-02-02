In order to properly generate documentation and run tests, developers are meant to request all features because `cargo doc` passes `--cfg test` and `--cfg doc` only to the outermost/foremost/current crate, not to its dependencies.

For generating docs, it's advised to use

```text
cargo doc --features all
```

For running tests, it was sufficient to use

```
cargo test
```

at the time of writing. Yet in the future it's recommended to use

```
cargo test --features all
```