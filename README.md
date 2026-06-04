# internal-error

Standalone workspace for the shared Rust internal error crates.

## Key Features

- `internal-error` provides the runtime types for tracked internal failures.
- `internal-error-derive` provides the `ErrorContract` proc macro that binds a
  crate-local `Error` enum to the shared internal-failure contract.
- `cargo xtask check` is the local equivalent of PR CI.
- Consumers can reference a Git commit or branch until a registry release path
  is needed.

## Developer Get Started

```sh
cargo test -p internal-error -p internal-error-derive
cargo xtask check
```

Run the full verification before completing implementation work:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo xtask check
cargo xtask review
```

## Key Code

- `crates/internal-error` contains the runtime `InternalError`, context frames,
  source-chain storage, and logging helper.
- `crates/internal-error-derive` contains the `ErrorContract` derive macro.
- `xtask` contains local automation used by PR CI.

## Version References

This repository does not currently run release automation. Link consumers to a
specific commit or branch until a registry release path is needed.

The PR CI workflow only needs `contents: read`.

## Plans

Implementation planning lives in [plans/README.md](plans/README.md).

## Related Docs

- [Error Contracts](docs/protocol/error-contracts.md)
- [Plans](plans/README.md)
