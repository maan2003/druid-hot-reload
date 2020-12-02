# Druid hot reloading

This is an experminent to hot reload druid apps. Code in `hot_reload` is adapted from https://github.com/irh/rust-hot-reloading.

## How to Run
```bash
cargo run &
cargo watch -s 'cargo build -p view`
```

now you can change the `view/src/lib.rs` have them live reload. Also remember to use lld linker otherwise hot reloads would be slow.

