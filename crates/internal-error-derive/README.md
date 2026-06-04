# internal-error-derive

`internal-error-derive` is the proc-macro crate behind
[`internal-error`](../internal-error/README.md). Most crates should not depend
on it directly; use `internal-error` and derive
`internal_error::ErrorContract` from there.

## Responsibilities

- Validate that one error enum exposes the canonical
  `Internal(#[from] InternalError)` variant.
- Generate the hidden module-level `DEFINED_AT` binding for that enum.
- Generate the tracked helper surface for local `Error`, `Result`, and
  `Option` ergonomics.

## What This Crate Does

The derive expects an explicit canonical internal variant:

```rust
#[derive(Debug, thiserror::Error, internal_error::ErrorContract)]
pub enum Error {
    #[error("[demo/config] invalid port")]
    InvalidPort,

    #[error("[demo/config] internal error")]
    Internal(#[from] internal_error::InternalError),
}
```

From that explicit enum shape, the derive generates:

- hidden `DEFINED_AT` binding for the module contract
- public helpers such as `Error::context(...)`,
  `Error::with_context(...)`, and `Error::internal_message(...)`
- a local `ErrorResultExt<T>` trait so same-module code can call
  `.context("...")` and `.with_context(|| ...)` on `Result` values
- a local `ErrorOptionExt<T>` trait so same-module code can call
  `.ok_or_internal("...")` and `.ok_or_internal_else(|| ...)` on `Option`
  values

## Quick Start

```rust
use internal_error::InternalError;
use thiserror::Error;

#[derive(Debug, Error, internal_error::ErrorContract)]
pub enum Error {
    #[error("[demo/config] invalid port")]
    InvalidPort,

    #[error("[demo/config] internal error")]
    Internal(#[from] InternalError),
}

pub type Result<T> = std::result::Result<T, Error>;

fn load_port(raw: &str, agent_id: &str) -> Result<u16> {
    raw.parse::<u16>()
        .with_context(|| format!("parse configured port for agent {agent_id}"))
}

fn workspace_id(maybe_workspace_id: Option<u64>) -> Result<u64> {
    maybe_workspace_id.ok_or_internal("session missing workspace id")
}
```

## Development

Run:

```sh
cargo test -p internal-error -p internal-error-derive
cargo clippy -p internal-error --all-targets --all-features -- -D warnings
cargo clippy -p internal-error-derive --all-targets --all-features -- -D warnings
cargo xtask check
```

### Key Code

- [`src/error_contract_derive/mod.rs`](src/error_contract_derive/mod.rs)
  coordinates validation and expansion.
- [`src/error_contract_derive/validation.rs`](src/error_contract_derive/validation.rs)
  enforces the canonical enum shape.
- [`src/error_contract_derive/expansion/`](src/error_contract_derive/expansion/)
  generates the local helper surface.

### Related Docs

- [`../internal-error/README.md`](../internal-error/README.md)
- [`../../docs/protocol/error-contracts.md`](../../docs/protocol/error-contracts.md)
- [`../../plans/internal-error-standalone-repo.md`](../../plans/internal-error-standalone-repo.md)
