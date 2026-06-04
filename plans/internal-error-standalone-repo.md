# Internal Error Standalone Repo

## Summary

Build this repository as the standalone home for the `internal-error` and
`internal-error-derive` Rust crates copied from:

- `/Users/calummoore/projects/futex/juno`
- `/Users/calummoore/projects/futex/accounting`

The current workspace only has `AGENTS.md`, a root `README.md`, and the Git
remote `git@github.com:futex-ai/internal-error.git`, so the implementation
should create a complete Rust workspace, local verification tooling, and PR CI.

## Source Comparison

The two source repos have the same crate file layout for:

- `crates/internal-error`
- `crates/internal-error-derive`

The Rust implementation files match. The differences found so far are:

- `internal-error` README names Juno in one repo and Bookfolio in the other.
- `internal-error-derive` README has the same repo-specific plan link
  difference.
- Juno uses workspace version `0.2.0`; Accounting uses `0.1.0`.
- Juno authors are `Polybase`; Accounting authors are `Accounting`.
- Juno has a `release-plz.toml`; Accounting does not.
- Juno PR CI is larger and pinned around its monorepo needs; Accounting PR CI
  is smaller but still includes TypeScript, Go, Terraform, and Helm setup.
- Juno has a fuller `docs/protocol/error-contracts.md`; Accounting has a
  shorter summary.

Recommended defaults unless the user chooses otherwise:

- Copy the crate source from either repo because the implementation matches.
- Neutralize docs so they describe the standalone crate, not Juno or Bookfolio.
- Take Juno's fuller protocol doc and remove Juno-specific wording.
- Use a minimal standalone CI workflow instead of copying either monorepo CI
  workflow verbatim.
- Start at version `0.1.0`.
- Do not configure release automation for now; consumers should link to a
  commit or branch.
- Use Stable Rust in CI.

## Decisions

1. Release destination:
   No release automation for now. Consumers should link to a commit or branch.
   Do not enable crates.io publishing yet.

2. Initial crate version:
   Use `0.1.0`, treating this as a new standalone repo.

3. Rust toolchain policy:
   Use Stable Rust in CI, matching Accounting's simpler approach.

## Milestone 1: Workspace Bootstrap

Create a functioning standalone Rust workspace containing the two crates and
their protocol documentation.

- [x] Add root `Cargo.toml` with workspace members for `crates/internal-error`,
  `crates/internal-error-derive`, and `xtask`.
- [x] Copy `crates/internal-error` and `crates/internal-error-derive`, excluding
  source-repo artifacts such as `.DS_Store`.
- [x] Normalize crate READMEs and module docs to refer to the standalone
  `internal-error` workspace.
- [x] Add `docs/protocol/error-contracts.md` based on Juno's fuller protocol
  doc with project-specific wording removed.
- [x] Generate and commit `Cargo.lock`.
- [x] Update the root `README.md` with key features, developer getting started,
  code entry points, and links to protocol docs and plans.
- [x] Run `cargo fmt --all -- --check`.
- [x] Run `cargo test -p internal-error -p internal-error-derive`.

## Milestone 2: Local Automation

Add a small `xtask` crate that can run the same local checks developers and CI
will run.

- [x] Create `xtask` with a CLI that supports `cargo xtask check`.
- [x] Add the Cargo alias that makes `cargo xtask ...` invoke the `xtask`
  package.
- [x] Include Markdown/diff checks, GitHub workflow linting when workflows
  exist, Rust source layout audit, Rust file length lint, Rust formatting,
  clippy, and workspace tests.
- [x] Add `cargo xtask rust-file-length-lint --all`.
- [x] Add `cargo xtask review` as a read-only diff review against
  `origin/main`.
- [x] Keep xtask modules under the repo line limits.
- [x] Add focused tests for check selection and audit behavior.
- [x] Run `cargo test --workspace`.

## Milestone 3: PR CI

Create a lean GitHub Actions PR workflow that mirrors `cargo xtask check`
without inheriting unrelated monorepo dependencies.

- [x] Add `.github/workflows/ci.yml` for pull requests.
- [x] Use checkout with full history so `origin/main...HEAD` comparisons work.
- [x] Install Rust with the selected toolchain policy.
- [x] Install actionlint for workflow validation.
- [x] Cache Rust dependencies with `Swatinem/rust-cache`.
- [x] Run `cargo xtask check`.
- [x] Confirm `.github/actionlint.yaml` is not needed for the current workflow.

## Milestone 4: Version Reference Policy

Document the current version reference policy without adding release
automation.

- [x] Do not add a release-plz workflow.
- [x] Do not add release-plz configuration.
- [x] Document that consumers should link to a commit or branch.
- [x] Document that PR CI only needs `contents: read`.

## Milestone 5: Final Verification

Validate the complete standalone repository before completion.

- [x] Run `cargo fmt --all -- --check`.
- [x] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- [x] Run `cargo test --workspace`.
- [x] Run `cargo xtask check`.
- [x] Run `cargo xtask review` after tests and `cargo xtask check`.
- [x] Review all local diffs against `origin/main`.
- [x] Report any `cargo xtask review` findings without automatically fixing
  them.
