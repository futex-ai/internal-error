# Rust audits read parent directory names

- **Severity:** Medium. **Likelihood:** Rare. **Effort:** Small.
  **Confidence:** High. **Harder later:** No.
- **Kind:** Outside the scope.
- **Category:** Product bug, because the gate tool gives wrong output.
- **Added:** 2026-11-04, from the branch `conductor/agents-md-internal-error`.
- **Location:** `xtask/src/rust_trait_audit.rs:48` and `:78`, the functions
  `rust_test_files` and `is_test_file`. `xtask/src/rust_source_audit.rs:75`,
  the function `production_rust_files`.
- **Reason:** The code is in `xtask`, and the branch that found the bug
  changed only agent rules and docs.

- **Terms:**
  - **Trait audit:** the check in `xtask/src/rust_trait_audit.rs` that fails
    on a handwritten trait implementation in a test file.
  - **Orphan-file audit:** the check in `xtask/src/rust_source_audit.rs` that
    fails on a production Rust file that no `mod` line reaches.
- **Background:**
  - The `rust-audit` phase of `cargo xtask check` runs both audits.
  - Both audits walk absolute paths from the workspace root. They test every
    component of a path, also the folders above the repository.
  - `is_test_file` treats a file as a test file when any component is
    `_tests_` or `tests`.
  - `production_rust_files` drops a file when any component is `_tests_`.
  - On 2026-11-04, a copy of this repository under a folder named `tests`
    failed `cargo xtask check --include rust-audit`. The trait audit printed
    "found 5 Rust trait audit violation(s)" for these production lines:
    - `crates/internal-error/src/defined_at.rs:51` and `:96`
    - `crates/internal-error/src/internal_error.rs:162`, which is
      `impl fmt::Debug for InternalError`
    - `crates/internal-error-derive/src/error_contract_derive/expansion/mod.rs:18`
    - `xtask/src/command.rs:48`
  - On the same day, a copy under a folder named `_tests_` had an
    undeclared file, `crates/internal-error/src/orphan_probe.rs`. The
    orphan-file audit passed there. It reported the file in a copy under a
    neutral folder. The trait audit failed in the `_tests_` copy too.
  - To reproduce, give each copy its own target directory. With a shared
    `CARGO_TARGET_DIR`, cargo reuses one xtask binary, which audits the copy
    that last compiled xtask.
- **Problem:** The result of both audits depends on the names of the folders
  above the repository. Under `tests`, the trait audit fails on correct
  code. Under `_tests_`, the orphan-file audit checks nothing.
- **Impact if no action:** A developer whose clone sits under such a folder
  gets a gate failure on correct code, or an orphan-file audit that checks
  nothing. The workaround is to move the clone.
- **Root question:** How do the audits classify each file, whatever folder
  holds the clone?
- **Options:**
  - **A.** Test only the components of the path relative to the workspace
    root, with `strip_prefix`. Add a test with a root under a `tests`
    folder. Effort: Small.
  - **B.** Make one walk, shared by both audits, yield paths relative to the
    workspace root. The changed-file mode of the file-length lint already
    works on relative paths from Git. Add the same test as A. Effort: Small.
  - **C.** Document the limit, and change nothing. Effort: Small.
- **Left out:** None. The three options cover a fix at each filter, a fix at
  the walk, and no fix.
- **Wider fix:** B. Relative paths from the walk give no check in either
  audit a path above the workspace root.
- **Previous examples:** None found.
- **Related:**
  - `plans/follow-ups/trait-audit-misses-impl-forms.md`. Same code: both
    items change `xtask/src/rust_trait_audit.rs`, so one branch can fix both.
  - `plans/follow-ups/xtask-root-fixed-at-compile-time.md`. Both bugs make
    the audits check the wrong set of files.
- **Recommendation:** B. It removes the cause for every check in both
  audits, at about the cost of A.
- **Action:** fix after merge, apply B, because the fix restores the
  behaviour that the audits describe and touches only xtask.
