# File-length lint skips uncommitted edits

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Category:** Repository rule.
- **Added:** 2026-11-04, from the branch `conductor/agents-md-internal-error`.
- **Location:** `xtask/src/rust_file_length_lint.rs:58`, the function
  `changed_rust_files`. The file-length line under `## Repository rules` in
  `AGENTS.md`.
- **Reason:** The code is in `xtask`, and the branch that found the gap
  changed only agent rules and docs.

- **Terms:**
  - **File-length lint:** the command `cargo xtask rust-file-length-lint`,
    which fails when a Rust file under `crates/` or `xtask/` has more than
    300 lines.
  - **Merge base:** the newest commit that both the branch and `origin/main`
    contain.
- **Background:**
  - Without `--all`, the file-length lint checks only changed Rust files.
    The `rust-audit` phase of `cargo xtask check` runs it in this form.
  - The function `changed_rust_files` lists the changed files with
    `git diff --name-only --diff-filter=ACMRT origin/main... --`. It adds
    the untracked files from `git ls-files --others --exclude-standard`.
  - `git diff origin/main...` compares the merge base with `HEAD`. So it
    lists only committed changes.
  - The lint then reads each listed file from the working tree. So it does
    check an uncommitted edit to a file that a commit of the branch changed.
    A scratch clone on 2026-11-04 showed this: after a branch commit changed
    `crates/internal-error/src/context_frame.rs`, an uncommitted edit of that
    file to 301 lines failed the lint. In the same clone, a 316-line edit to
    a tracked file that no commit of the branch changed passed the lint.
  - A test in a scratch repository on 2026-11-04 showed the gap. The tracked
    file had no commit on the branch. The current form listed neither the
    modified tracked file nor a staged new file. This form listed both:
    `git diff --name-only --diff-filter=ACMRT --merge-base origin/main --`.
  - Git 2.30 added the `--merge-base` option.
  - CI checks out a clean commit, so CI still catches such a file after the
    push.
  - The file-length line in `AGENTS.md` documents the gap.
- **Problem:** An uncommitted edit that takes a tracked Rust file past 300
  lines passes the local lint when no commit of the branch changed that
  file. A new Rust file that is staged but not committed also passes,
  because it is no longer untracked.
- **Impact if no action:** The local early check passes, and CI then fails
  after the push.
- **Root question:** Which files does the local file-length lint check?
- **Options:**
  - **A.** List the changed files against the merge base and the working
    tree, with
    `git diff --name-only --diff-filter=ACMRT --merge-base origin/main --`,
    and keep the untracked files. Put this definition of the changes of the
    branch in one xtask helper. Add a test. Update the `AGENTS.md` line.
    Effort: Small.
  - **B.** Make `cargo xtask check` run the `--all` form. It checks unchanged
    files too, so the gate gets stricter. Effort: Small.
  - **C.** Keep the code. `AGENTS.md` already documents the gap.
    Effort: Small.
- **Left out:** A diff against the tip of `origin/main` instead of the merge
  base. It also lists the files that only `main` changed, so A is better on
  every point.
- **Wider fix:** A. One xtask helper then defines the changes of the branch
  as the merge base against the working tree, plus untracked files. Every
  changed-file check uses it, such as this lint and the whitespace check of
  `cargo xtask check`.
- **Previous examples:** `futex-ai/ai` recorded
  `plans/follow-ups/file-length-lint-all-flag.md`, for a file-length lint
  that ignored its `--all` flag.
- **Related:** `plans/follow-ups/whitespace-check-sees-only-unstaged-edits.md`.
  Shared wider fix: the whitespace check also decides which changes of the
  branch it sees, and the same helper can define them for both checks.
- **Recommendation:** A. The local lint then checks the files that the
  developer changed.
- **Action:** ask the user, because option A makes the local lint fail on
  uncommitted edits that pass today, and a stricter check needs the user.
