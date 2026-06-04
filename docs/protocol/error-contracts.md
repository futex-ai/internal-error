# Error Contracts

This page defines the canonical error-contract model for Rust crates that use
the `internal-error` workspace.

## Scope

The contract applies to:

- crate-local `Error` and `Result<T>` types
- trait/interface crate error boundaries
- implementation-crate propagation rules
- user-facing CLI, REST, and gRPC redaction rules
- model-visible tool failure payloads

Implementation helpers for this contract live in `crates/internal-error` and
its derive companion, `crates/internal-error-derive`.

## Core Model

Every crate or module that exposes a meaningful boundary must define its own
typed `Error` and `Result<T>`.

Public error enums must keep only the variants that callers explicitly branch
on. All other failures must collapse into `Internal(InternalError)`.

Typical typed handled variants include:

- authentication or authorization failures
- invalid user or tool input
- explicit not-found or conflict cases
- protocol validation failures
- documented business-contract cases that change caller behavior

Typical internal failures include:

- storage, query, filesystem, or object-store failures
- provider, HTTP client, or serialization failures
- crypto, template, parsing, or configuration failures
- child-crate errors that the parent does not explicitly handle

For reusable public library crates, downstream library consumers count as the
caller set. That means the stable typed surface may be broader than in a
service-internal crate when external consumers are expected to branch on those
variants as part of the public API contract.

Illustrative shape:

```rust
#[derive(Debug, thiserror::Error, internal_error::ErrorContract)]
pub enum Error {
    #[error("[demo/workspace] unknown agent `{agent_id}`")]
    UnknownAgent { agent_id: String },

    #[error("[demo/workspace] invalid login")]
    InvalidLogin,

    #[error("[demo/workspace] internal error")]
    Internal(#[from] internal_error::InternalError),
}
```

## Internal Error Metadata

`InternalError` is the shared internal-failure carrier. It must keep rich debug
metadata without forcing every module to define redundant wrapper variants.

An `InternalError` must carry:

- `defined_at`: a module-level contract anchor defined once beside the owning
  error enum
- `caller_at`: the call site that created or recontextualized this internal
  failure
- a context-frame stack that records higher-level operation breadcrumbs
- one underlying `source()` chain for the real causal error

Rules:

- error-contract modules should normally derive
  `internal_error::ErrorContract`, which binds one hidden module-level
  `DEFINED_AT` anchor with `defined_at!()` and generates local helper methods
  such as `Error::context(...)`, `Error::with_context(...)`,
  `Error::internal_message(...)`, and local `*ResultExt` / `*OptionExt` traits
  for same-module `.context("...")`, `.with_context(|| ...)`,
  `.ok_or_internal("...")`, and `.ok_or_internal_else(|| ...)` call sites
- handwritten `DEFINED_AT` adapters are only for unusual cases where the derive
  cannot be used cleanly
- feature code must not pass `Location::caller()` manually
- call-site capture must happen through `#[track_caller]` helper methods or
  constructors
- recontextualizing an existing `InternalError` appends a context frame instead
  of wrapping one `InternalError` inside another
- normal `Display` for `Internal(InternalError)` must stay stable and generic
- rich `defined_at`, `caller_at`, context, and source data belongs in `Debug`
  and structured logs

## Ownership

Trait interface crates own the error contract for their methods.

Rules:

- interface crates such as `*-interface` or `*-store-interface` define the
  typed handled variants that downstream callers may branch on
- concrete implementation crates return those interface errors directly for
  trait methods
- implementation crates must not maintain mirror enums for the same trait
  boundary when they add no distinct handled behavior
- parent crates may keep their own error enums, but only for handled variants
  they actually branch on at that layer
- reusable public library crates may define a larger stable typed surface than
  service-internal crates when those variants are part of the documented public
  API contract

## Propagation And Collapse

When a parent crate receives a child error:

- explicitly map the child variants the parent handles
- collapse every other child variant into parent `Internal`
- if the child error is already `Internal`, append parent context instead of
  nesting another internal wrapper

This keeps contracts small and avoids chains such as
`ParentInternal -> ChildInternal -> serde_json::Error` when only one internal
carrier is needed.

## Boundary Redaction

Public and model-visible boundaries must not leak internal implementation
details.

Rules:

- REST and gRPC responses map `Internal(InternalError)` to stable generic codes
  and messages such as `internal` / `internal server error`
- normal CLI output maps `Internal(InternalError)` to stable generic messages
  and exit behavior without file paths, line numbers, SQL text, provider
  failures, or source-chain dumps
- model-visible tool failure payloads may surface handled user-actionable
  messages, but internal failures must use a stable generic message such as
  `internal tool error`
- operator/debug surfaces may show full internal metadata intentionally
- redaction does not suppress logging; top-level binaries must still emit the
  full internal failure in logs

## Implementation Rules

Repository-wide implementation rules that follow from this contract:

- use `thiserror` enums for boundary errors
- keep only handled variants typed; everything else is internal
- prefer the shared `internal_error::ErrorContract` derive for the ergonomic
  binding step instead of hand-writing module-local `DEFINED_AT` result
  adapters
- prefer `Internal(#[from] InternalError)` for the canonical internal fallback
  path, and do not use `#[from]` for public wrapper variants or cross-crate
  error translation
- do not stringify internal failures into ad hoc buckets such as
  `Json { message: String }`
- do not use direct feature-level `map_err(...)` conversions that lose precise
  `#[track_caller]` call-site capture
- do not match on error strings to recover behavior

The coding-level enforcement for these rules lives in `AGENTS.md`; this page is
the protocol contract those rules are enforcing.
