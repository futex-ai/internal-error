# Markdown check lints wrong files from subfolder

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Category:** Repository rule.
- **Added:** 2026-11-04, from the branch `conductor/agents-md-internal-error`.
- **Location:** `xtask/src/check.rs:79` and `:106`, the functions
  `run_script_checks` and `markdown_files`. `xtask/src/command.rs:49`, where
  `RealCommandRunner::run` sets no current directory.
- **Reason:** The code is in `xtask`, and the branch that found the bug
  changed only agent rules and docs.

- **Terms:**
  - **Workspace root:** the folder with the root `Cargo.toml` of this
    repository, which xtask treats as the tree to check.
  - **Markdown check:** the `scripts` phase of `cargo xtask check`, which
    runs `npx --yes markdownlint-cli2` on a list of Markdown files and then
    `git diff --check`.
- **Background:**
  - `markdown_files` lists every Markdown file except `AGENTS.md` and
    `CLAUDE.md`, as paths relative to the workspace root.
  - `run_script_checks` passes that list to `npx --yes markdownlint-cli2`.
  - `RealCommandRunner::run` starts each command in the current directory of
    the caller. `cargo run`, which `cargo xtask` uses, keeps that directory.
  - So from a subfolder, `npx markdownlint-cli2` resolves the paths from the
    subfolder. It skips each path that does not exist there, and it does not
    fail.
  - On 2026-11-04, `cargo xtask check --include scripts` ran in
    `crates/internal-error/`. It printed "Linting: 1 file" and "Summary: 0
    issues in 0 files", and it passed. From that folder, only `README.md`
    exists, and there it names the crate README.
  - From the root, the same command linted all 11 Markdown files of that
    time.
  - The other commands of the gate also run in the current directory. Each
    still finds the right files from a subfolder:
    - `cargo fmt --all -- --check`, `cargo clippy --workspace`, and
      `cargo test --workspace` find the workspace from the current directory
      upward, and they check all of it.
    - `git diff --check` covers the whole repository from a subfolder. The
      Git docs say that `--relative` "can be told to exclude changes outside
      the directory". A user who sets the `diff.relative` config option gets
      only the subfolder.
    - `actionlint -shellcheck=` gets no file argument. Its usage doc says
      that it "finds all workflow files in the current repository". Its
      `findProject` function walks up to the first folder that has both
      `.github/workflows` and `.git`.
    - The `workflow` phase decides whether to skip from the workspace root,
      not from the current directory.
- **Problem:** From a subfolder, the Markdown check lints a different set of
  files than `AGENTS.md` describes, and it still passes.
- **Impact if no action:** A developer who runs the gate from a subfolder
  gets a Markdown check that skips most files. CI runs from the root, so it
  still lints every file.
- **Root question:** How does each gate command check the same files from
  any folder of the repository?
- **Options:**
  - **A.** Pass absolute paths to `npx markdownlint-cli2`. Every other
    command still depends on the folder of the caller. Effort: Small.
  - **B.** Run every command that xtask starts in the workspace root. Set
    the directory once, in `RealCommandRunner::run`. Land it with the
    run-time root of `plans/follow-ups/xtask-root-fixed-at-compile-time.md`.
    Alone, it moves every command to the compile-time root. Effort: Small.
  - **C.** Document that `cargo xtask` runs from the workspace root, and
    change nothing. Effort: Small.
- **Left out:** Stop with an error when the current directory is not the
  root. B is better on every point: it costs the same, and the gate then
  works from any folder.
- **Wider fix:** B. One place then sets the folder of every command, so no
  check depends on the folder of the caller.
- **Previous examples:** None found.
- **Related:** `plans/follow-ups/xtask-root-fixed-at-compile-time.md`. Shared
  wider fix: one root found at run time, and every command runs in it. B
  waits for the run-time root of that item, or lands with it.
- **Recommendation:** B. It fixes the Markdown check, and no command can
  depend on the folder of the caller again, at about the cost of A.
- **Action:** fix after merge, apply B, because the fix makes the Markdown
  check lint the files that `AGENTS.md` names, and it touches only xtask.
