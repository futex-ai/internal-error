# internal-error

`internal-error` is the shared internal-failure runtime for Rust error
contracts. Depend on it when a crate wants typed handled variants plus one
canonical `Internal(InternalError)` fallback with tracked call sites and stacked
context.

## Responsibilities

- Define the module-level `DefinedAt` anchor used by error-contract modules.
- Reexport the `ErrorContract` derive that binds one local error enum to the
  shared internal-failure contract.
- Capture precise call sites with `#[track_caller]` without requiring feature
  code to pass `Location::caller()` manually.
- Preserve one underlying source chain while stacking higher-level context
  frames.
- Keep normal `Display` output generic so internal failures can be redacted on
  user-facing surfaces.
- Provide one shared top-level logging helper for debug-rich internal error
  output.

## What This Crate Does

- `defined_at!()` creates a module-level `DefinedAt` constant.
- `DefinedAt` records the module/file anchor for an error contract.
- `SourceLocation` records a concrete caller file/line/column.
- `ContextFrame` records one context message plus the contract and caller that
  added it.
- `InternalError` stores the originating contract anchor, the creation call
  site, context frames, and the underlying source chain.
- `ErrorContract` binds one local error enum to the canonical
  `Internal(#[from] InternalError)` contract.
- `ErrorExt` converts one source error into `InternalError`.
- `ResultExt` converts `Result<T, E>` into `Result<T, InternalError>`.
- `log_top_level_error(...)` emits a consistent debug-rich top-level error log.

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
    let port = raw
        .parse::<u16>()
        .with_context(|| format!("parse configured port for agent {agent_id}"))?;

    if port == 0 {
        return Err(Error::InvalidPort);
    }

    Ok(port)
}

fn bootstrap(raw: &str, agent_id: &str) -> Result<u16> {
    match load_port(raw, agent_id) {
        Ok(port) => Ok(port),
        Err(err) => Err(err.context("bootstrap runtime")),
    }
}

fn workspace_id(maybe_workspace_id: Option<u64>) -> Result<u64> {
    maybe_workspace_id.ok_or_internal("session missing workspace id")
}

let error = bootstrap("abc", "root").unwrap_err();

match error {
    Error::Internal(internal) => {
        assert_eq!(
            internal.context_frames()[0].message(),
            "parse configured port for agent root"
        );
        assert_eq!(internal.context_frames()[1].message(), "bootstrap runtime");
        assert_eq!(
            internal.source_ref().to_string(),
            "invalid digit found in string"
        );
    }
    Error::InvalidPort => panic!("expected internal error"),
}
```

The derive also generates local `ErrorResultExt<T>` and `ErrorOptionExt<T>`
traits beside the enum. Those traits are automatically in scope inside the same
module, so feature code can call:

- `result.context("...")`
- `result.with_context(|| format!(...))`
- `err.context("...")`
- `err.with_context(|| format!(...))`
- `option.ok_or_internal("...")`
- `option.ok_or_internal_else(|| format!(...))`

Foreign `Result<T, E>` errors collapse into the canonical internal fallback.
Existing `Error::Internal(...)` values gain one context frame. Handled typed
variants stay unchanged. `Option` uses `ok_or_*` names because `None` is
creating a new internal failure, not annotating an existing source error.

If another module wants the same method syntax while returning this error type,
it can import the generated traits explicitly:

```rust
use crate::config::{ErrorOptionExt as _, ErrorResultExt as _};
```

## Development

Run:

```sh
cargo test -p internal-error -p internal-error-derive
cargo clippy -p internal-error --all-targets --all-features -- -D warnings
cargo clippy -p internal-error-derive --all-targets --all-features -- -D warnings
cargo xtask check
```

Keep this crate generic. It should not depend on application-specific domains
or service crates.

### Key Code

- [`src/internal_error.rs`](src/internal_error.rs) defines `InternalError`, the
  context-frame stack, and source-chain behavior.
- [`src/error_contract.rs`](src/error_contract.rs) defines the shared contract
  trait used by derived error enums.
- [`../internal-error-derive/src/error_contract_derive/`](../internal-error-derive/src/error_contract_derive/)
  contains the proc-macro expansion that generates local `Result` and `Option`
  helpers.

### Related Docs

- [`../../docs/protocol/error-contracts.md`](../../docs/protocol/error-contracts.md)
- [`../../AGENTS.md`](../../AGENTS.md)
- [`../../plans/internal-error-standalone-repo.md`](../../plans/internal-error-standalone-repo.md)
