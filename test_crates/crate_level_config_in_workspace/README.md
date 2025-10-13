# Crate-level config

There's a `.cargo/config.toml` file inside the `test-pkg` crate directory
setting `--cfg tokio_unstable` which is required to compile the test crate here.

- `cargo check` from inside `test-pkg` works.
- `cargo check` at workspace level *does not work*.
- `cargo check --manifest-path <path>` only works from inside the `test-pkg` directory,
  even if pointing to the workspace-level `Cargo.toml` file.
