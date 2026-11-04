# xtask fixes its workspace root at compile time

- **Severity:** Medium. **Likelihood:** Rare. **Effort:** Small.
  **Confidence:** High. **Harder later:** No.
- **Kind:** Outside the scope.
- **Category:** Product bug, because the gate tool checks the wrong tree.
- **Added:** 2026-11-04, from the branch `conductor/agents-md-internal-error`.
- **Location:** `xtask/src/cli.rs:110`, the function `workspace_root`, which
  reads `env!("CARGO_MANIFEST_DIR")` at line 111.
- **Reason:** The code is in `xtask`, and the branch that found the bug
  changed only agent rules and docs.

- **Terms:**
  - **Workspace root:** the folder with the root `Cargo.toml` of a copy of
    this repository, which xtask treats as the tree to check.
  - **Shared target directory:** one Cargo build folder that two copies of
    the repository use, through `CARGO_TARGET_DIR` or `build.target-dir`.
- **Background:**
  - `.cargo/config.toml` maps `cargo xtask` to
    `cargo run --package xtask --`.
  - `workspace_root` takes the parent folder of
    `env!("CARGO_MANIFEST_DIR")`. The compiler writes that value into the
    binary when it builds xtask. It is the only `env!` lookup in the
    workspace.
  - These commands use the root:
    - the Rust audits, also through `cargo xtask rust-file-length-lint`
    - the Markdown file list of the `scripts` phase
    - the test of the `workflow` phase that decides whether to skip
    - `cargo xtask review`, for its Git commands and for the repository path
      in its prompt
  - The other commands run in the current directory: `cargo fmt`,
    `cargo clippy`, `cargo test`, `git diff --check`, `npx markdownlint-cli2`,
    and `actionlint`. So `npx markdownlint-cli2` gets its file list from the
    root, but it runs in the current directory.
  - On 2026-11-04, two copies of this repository shared one target
    directory. The first copy sat under a folder named `tests`. The second
    copy had an undeclared file, `crates/internal-error/src/orphan_probe.rs`.
    - `cargo xtask check --include rust-audit` in the first copy compiled
      xtask and failed with five trait audit findings.
    - The same command in the second copy did not compile xtask again. It
      printed the same five findings and "rust file length lint passed for
      0 file(s)". It did not report the orphan file.
    - With its own target directory, the second copy reported the orphan
      file and "rust file length lint passed for 1 file(s)".
  - On the same day, a scratch package in two copies with one shared target
    directory showed these values:
    - `cargo run` sets `CARGO_MANIFEST_DIR` at run time to the folder of the
      package that it runs. This is also true from a subfolder.
    - In the second copy, the compile-time value still named the first copy.
    - `cargo run` keeps the current directory of the caller.
    - A binary that runs without cargo has no run-time `CARGO_MANIFEST_DIR`.
  - CI is not affected, for two reasons:
    - `.github/workflows/ci.yml` checks out the repository with
      `actions/checkout@v4` and no `path`. The GitHub docs say that
      `GITHUB_WORKSPACE` is "the default location of your repository when
      using the checkout action". So each run uses the same folder.
    - The workflow caches `target/` with `Swatinem/rust-cache@v2`. Its README
      says that "the workspace crates themselves are not cached". The
      workflow does not set `cache-workspace-crates`, so each run compiles
      xtask again.
- **Problem:** With a shared target directory, `cargo xtask` checks the copy
  that last compiled xtask, not the copy where it runs. One gate run can
  then mix two copies: the audits read one copy, and the formatter and the
  tests read the other.
- **Impact if no action:** A developer gets gate results for another copy.
  The gate can pass on code that fails CI, or fail on correct code. Several
  worktrees of one repository that share a target directory meet this.
- **Root question:** How does xtask find the copy of the repository that it
  runs in?
- **Options:**
  - **A.** Read `CARGO_MANIFEST_DIR` at run time, which `cargo run` sets.
    Keep the compile-time value as a fallback for a binary that runs without
    cargo. Such a run keeps the bug. Effort: Small.
  - **B.** Find the root from the current directory: the nearest folder,
    upward, whose `Cargo.toml` has a `[workspace]` table. This works for
    every way to start xtask inside a copy. Outside a copy, xtask then stops
    with an error. Effort: Small.
  - **C.** Document that two copies must not share a target directory, and
    change nothing. Effort: Small.
- **Left out:** A build script that makes cargo compile xtask again for each
  copy. B is better on every point: it needs no new build, and it also works
  for a binary that runs without cargo.
- **Wider fix:** B. Together with the fix of
  `plans/follow-ups/markdown-check-lints-wrong-files-from-subfolder.md`, one
  root found at run time then serves every command.
- **Previous examples:** None found.
- **Related:**
  - `plans/follow-ups/audits-read-parent-directory-names.md`. Both bugs make
    the audits check the wrong set of files. Its reproduction note describes
    this bug.
  - `plans/follow-ups/markdown-check-lints-wrong-files-from-subfolder.md`.
    Shared wider fix: one root found at run time, and every command runs in
    it. Its fix moves every command into the root, so it waits for this fix
    or lands with it.
- **Recommendation:** B. It removes the compile-time root, so no way to start
  xtask inside a copy can name another copy.
- **Action:** fix after merge, apply B, because the fix makes the gate check
  the copy where it runs, and it touches only xtask.
