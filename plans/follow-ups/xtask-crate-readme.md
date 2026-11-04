# The xtask crate has no README

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Category:** Docs or spec.
- **Added:** 2026-11-04, from the branch `conductor/agents-md-internal-error`.
- **Location:** `xtask/`, which has no `README.md`.
- **Reason:** The README is a new crate doc, and the branch that found the
  gap changed only agent rules and docs.

- **Terms:**
  - **xtask:** the task runner crate of the workspace, in `xtask/`, which
    developers and CI run as `cargo xtask`.
- **Background:**
  - `.cargo/config.toml` maps `cargo xtask` to
    `cargo run --package xtask --`.
  - `xtask/Cargo.toml` sets `publish = false`.
  - The crate has three subcommands:
    - `check` runs the verification phases `scripts`, `workflow`,
      `rust-audit`, `rust-fmt`, `rust-clippy`, and `rust-test`. Its
      `--include` and `--exclude` filters select phases.
    - `review` runs a read-only AI review of the local diff against
      `origin/main` with the `codex` CLI.
    - `rust-file-length-lint` checks the 300-line cap on changed Rust files.
      With `--all`, it checks every Rust file under `crates/` and `xtask/`.
  - The `rust` skill of `futex-ai/skills` says that every crate has its own
    `README.md`. The other two crates of the workspace have one.
  - The `documentation` skill of `futex-ai/skills` gives the sections of a
    crate README, in this order: `## Responsibilities`,
    `## What This Crate Does`, `## Quick Start`, `## Development`,
    `### Key Code`, and `### Related Docs`.
  - The `documentation` skill also asks binary crates to document their
    command-line interface.
- **Problem:** The `xtask` crate has no `README.md`. It is the only crate of
  the workspace without one.
- **Impact if no action:** The crate breaks the README rule. Its commands
  are documented only in `--help` and in `AGENTS.md`.
- **Root question:** Where does the `xtask` crate document its commands?
- **Options:**
  - **A.** Add `xtask/README.md` with the six sections, each subcommand, and
    the phases of `check`. Effort: Small.
- **Left out:** Exempt `xtask` because it is not published. The `rust` skill
  has no such exemption. Move the command docs into the root `README.md`.
  The `documentation` skill puts crate docs in the crate README.
- **Wider fix:** None. A gate check for crate READMEs is a new lint, which
  needs the user.
- **Previous examples:** None found.
- **Recommendation:** A. It is the only option that meets the README rule.
- **Action:** fix after merge, apply A, because the README is outside the
  scope of the branch that found the gap.
