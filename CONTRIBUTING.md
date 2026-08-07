# Contributing

Thanks for taking the time to improve RookTAB.

## Getting started

```
rustup target add wasm32-wasip2
cargo build --release
```

`.cargo/config.toml` pins `wasm32-wasip2` as the default target, the plugin crate only links as a
WebAssembly component.

## Before opening a pull request

- `cargo fmt --all`
- `cargo clippy --workspace --target wasm32-wasip2 -- -D warnings`
- `cargo build --release`
- Update `README.md` when you change commands, permissions or the stored data format.

## Style

- Follow the surrounding code. Naming, module layout and error handling are already consistent.
- Prefer composition over inheritance style trait stacking, and keep types focused.
- Comments are only for things the code cannot say. Write them in English.
- Use `-` instead of long dashes in documentation.

## Commits

Conventional commit titles, one purpose per commit:

```
feat: add temporary permission nodes
fix: keep default group when a parent is removed
docs: document the duration format
```

Avoid unrelated formatting changes so blame history stays useful.

## Reporting bugs

Open an issue with the server version, the plugin version, the command you ran and the relevant
log output. If the problem involves stored data, include the matching JSON file with any private
information removed.
