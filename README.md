# Rust

Adventures in learning the [Rust](https://www.rust-lang.org) programming
language.

**Note**: having installed [rustup](https://github.com/rust-lang-nursery/rustup.rs),
remember to run ``export PATH=$HOME/.cargo/bin:$PATH`` in a shell session to add the rust
binaries to the path.

## Cargo

```
# Generate a Rust library template:
cargo new <NAME>
# Generate a binary instead of a library template:
cargo new <NAME> --bin
# Build an executable:
cargo build
cargo build --release
# Try building and running an executable:
cargo run
# Run library tests:
cargo test
```
