# internal-error

Standalone workspace for the shared Rust internal error crates.

## Key Features

- `internal-error` provides the runtime types for tracked internal failures.
- `internal-error-derive` provides the `ErrorContract` proc macro that binds a
  crate-local `Error` enum to the shared internal-failure contract.
- `cargo xtask check` is the local equivalent of PR CI.
- Release-plz manages GitHub-only version PRs, tags, changelogs, and releases.

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

## Release Automation

Release-plz is configured for GitHub-only releases for now. Do not enable
crates.io publishing until package ownership and token policy are confirmed.

Required GitHub workflow permissions:

- release workflow: `contents: write` for tags/releases and
  `pull-requests: write` for release PRs
- CI workflow: `contents: read`

No release secret is required for the current GitHub-only setup beyond the
standard `GITHUB_TOKEN` provided by GitHub Actions.

## Plans

Implementation planning lives in [plans/README.md](plans/README.md).

## Related Docs

- [Error Contracts](docs/protocol/error-contracts.md)
- [Plans](plans/README.md)
