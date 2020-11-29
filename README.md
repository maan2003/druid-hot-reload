# Druid hot reloading

This is an experminent to hot reload druid apps. Code in `hot_reload_lib` is mostly taken from https://github.com/irh/rust-hot-reloading with minor changes (update notify to 5.0-pre).

This is only is tested on linux. Although porting should only require small changes in `hot_reload_lib`.

## How to Run
```bash
cargo run &
cargo watch -s 'cargo build -p view`
```

now you can change the `view/src/lib.rs` have them live reload. Also remember to use lld linker otherwise hot reloads would be slow.

