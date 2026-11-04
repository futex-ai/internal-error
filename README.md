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

`cargo xtask check` runs the complete verification, as PR CI does. It runs
the Markdown and workflow checks, the Rust audits, the format check, Clippy,
and the workspace tests. The Commands section of
[`AGENTS.md`](AGENTS.md#commands) lists the other commands.

`cargo xtask review` runs an optional, read-only AI review of the local diff
against `origin/main`. It needs the `codex` CLI.

## Key Code

- `crates/internal-error` contains the runtime `InternalError`, context frames,
  source-chain storage, and logging helper.
- `crates/internal-error-derive` contains the `ErrorContract` derive macro.
- `xtask` contains local automation used by PR CI.

## Version References

This repository does not currently run release automation. Link consumers to a
specific commit or branch until a registry release path is needed.

The PR CI workflow only needs `contents: read`.

## Related Docs

- [Error Contracts](docs/protocol/error-contracts.md)
