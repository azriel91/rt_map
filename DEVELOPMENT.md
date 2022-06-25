# Development

## Dependencies

```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov

# Optional: Use `nextest` to run tests
cargo install cargo-nextest
```


## Running Tests

```bash
cargo test --features "rt_vec"
cargo nextest run --features "rt_vec"
```


## Coverage

Collect coverage and output as `html`.

```bash
cargo llvm-cov --features rt_vec --open --output-dir ./target/coverage

# With `nextest`:
# https://github.com/taiki-e/cargo-llvm-cov/issues/151
cargo coverage
# This is an alias defined in `.cargo/config.toml` to:
cargo llvm-cov nextest --features rt_vec --open --output-dir ./target/coverage
```

Collect coverage and output as `lcov`.

```bash
cargo llvm-cov --features rt_vec --lcov --output-path ./target/coverage/lcov.info

# With `nextest`:
# https://github.com/taiki-e/cargo-llvm-cov/issues/151
cargo llvm-cov nextest --features rt_vec --lcov --output-path ./target/coverage/lcov.info
```


## Releasing

Update crate versions, then push a tag to the repository. The [`publish`] GitHub workflow will automatically publish the crates to [`crates.io`].

[`publish`]: https://github.com/azriel91/rt_map/actions/workflows/publish.yml
[`crates.io`]:https://crates.io/
