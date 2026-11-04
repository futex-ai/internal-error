# xtask Git errors name the wrong cause

- **Severity:** Low. **Likelihood:** Rare. **Effort:** Small.
  **Confidence:** High. **Harder later:** No.
- **Kind:** Outside the scope.
- **Category:** Product bug, because the gate tool reports a wrong cause.
- **Added:** 2026-11-04, from the branch `conductor/agents-md-internal-error`.
- **Location:** `xtask/src/rust_file_length_lint.rs:125`, the function
  `git_lines`. `xtask/src/review.rs:142`, the function `git_capture`.
  `xtask/src/error.rs:29`, the variant `GitBaseUnavailable`.
- **Reason:** The code is in `xtask`, and the branch that found the bug
  changed only agent rules and docs.

- **Terms:**
  - **Error output:** the text that Git writes to its standard error stream,
    which names the cause of a failure.
- **Background:**
  - `git_lines` runs two Git commands for the changed-file mode of the
    file-length lint:
    - `git diff --name-only --diff-filter=ACMRT origin/main... --`
    - `git ls-files --others --exclude-standard`
  - `git_capture` runs eight Git commands for the prompt of
    `cargo xtask review`:
    - `git status --short`
    - `git diff --stat origin/main...` and
      `git diff --name-status origin/main...`
    - `git diff --staged --stat --` and `git diff --staged --name-status --`
    - `git diff --stat --` and `git diff --name-status --`
    - `git ls-files --others --exclude-standard`
  - Only three of these ten commands name `origin/main`.
  - Both helpers run Git with `.output()` in the workspace root. When Git
    exits without success, both return
    `Error::GitBaseUnavailable { base_ref: "origin/main" }`. Neither reads
    the error output of Git.
  - The message of that variant is "git base `origin/main` is not
    available", after the prefix `[xtask/error]`.
  - The existing variant `Error::CommandFailed` holds the command and the
    status, but not the error output. `RealCommandRunner` uses it.
  - On 2026-11-04, a check ran xtask with a root that had no Git folder. It
    printed the message about `origin/main`.
- **Problem:** Every failed Git command reports a missing `origin/main`,
  also when the command does not use `origin/main`. Examples are a failed
  `git ls-files --others --exclude-standard`, a failed `git status --short`,
  and a root with no Git folder. The error output of Git is lost.
- **Impact if no action:** A developer looks for a missing `origin/main`
  while the real cause is elsewhere. The error output of Git, which names
  the cause, never reaches the developer.
- **Root question:** How does xtask report the cause of a failed Git command?
- **Options:**
  - **A.** Put the error output of Git into the existing message of
    `GitBaseUnavailable`. The message still names `origin/main` for every
    failure. Effort: Small.
  - **B.** Add one Git helper that both modules share. It returns a typed
    variant for a failed Git command, with the command, the status, and the
    error output of Git. It returns `GitBaseUnavailable` only when
    `origin/main` does not resolve. Add tests. Effort: Small.
  - **C.** Keep the code. Effort: Small.
- **Left out:** Pick the variant by matching the error output of Git. The
  `rust` skill of `futex-ai/skills` forbids matching errors on their text.
- **Wider fix:** B. One shared helper gives every Git call of xtask the same
  typed errors.
- **Previous examples:** None found.
- **Related:** `plans/follow-ups/file-length-lint-skips-uncommitted-edits.md`.
  Same code: its fix changes the Git calls of `changed_rust_files`, which
  use `git_lines`, so one branch can fix both.
- **Recommendation:** B. It names the real cause of each failed Git call, at
  about the cost of A.
- **Action:** fix after merge, apply B, because the fix makes the gate
  report the real cause, and it touches only xtask.
