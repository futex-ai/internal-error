# Whitespace check sees only unstaged edits

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Category:** Repository rule.
- **Added:** 2026-11-04, from the branch `conductor/agents-md-internal-error`.
- **Location:** `xtask/src/check.rs:86`, in the function `run_script_checks`,
  which runs `git diff --check`.
- **Reason:** The code is in `xtask`, and the branch that found the gap
  changed only agent rules and docs.

- **Terms:**
  - **Whitespace check:** the command `git diff --check`, which the
    `scripts` phase of `cargo xtask check` runs after the Markdown lint.
  - **Merge base:** the newest commit that both the branch and `origin/main`
    contain.
- **Background:**
  - `git diff --check` with no revision compares the working tree with the
    index. So it checks only unstaged edits to tracked files.
  - CI checks out a clean commit, so there the check sees nothing.
  - Locally, it skips staged edits, new files, and committed changes.
  - A test on 2026-11-04 used a scratch repository with four lines that end
    in a space. One line was committed, one staged, one unstaged, and one in
    an untracked file.
    - `git diff --check` reported only the unstaged line.
    - `git diff --check --merge-base origin/main` reported the committed,
      the staged, and the unstaged line.
    - `git diff --check origin/main...HEAD` reported only the committed
      line.
    - No form reported the untracked file.
    - Each form exited with status 2 when it reported a line.
    - On a clean tree, as in CI, `git diff --check` passed with status 0.
  - Git 2.30 added the `--merge-base` option.
  - `AGENTS.md` names `git diff --check` in its Markdown check command. The
    help text of the `scripts` phase says "Markdown and whitespace diff
    checks".
- **Problem:** The whitespace check misses committed and staged changes. In
  CI, it checks nothing.
- **Impact if no action:** Whitespace errors reach `main`, although
  `AGENTS.md` and the help text of the gate name a whitespace check.
- **Root question:** Which changes of the branch does the whitespace check
  see?
- **Options:**
  - **A.** Run `git diff --check --merge-base origin/main`. It covers the
    commits of the branch, staged edits, and unstaged edits. Untracked files
    stay unchecked until they are staged. Effort: Small.
  - **B.** Run `git diff --check origin/main...HEAD` for the commits of the
    branch, and keep the plain form for unstaged edits. Staged edits and
    untracked files stay unchecked. This form also works with Git older
    than 2.30. Effort: Small.
  - **C.** Keep the check as it is. Effort: Small.
  - **D.** Check the changes that one shared xtask helper lists: the merge
    base against the working tree, plus untracked files.
    `plans/follow-ups/file-length-lint-skips-uncommitted-edits.md` proposes
    the same helper. Effort: Medium.
- **Left out:** Drop the whitespace check, because markdownlint and rustfmt
  already catch most trailing spaces in Markdown and Rust. That fails the
  goal for other files, such as TOML and YAML.
- **Wider fix:** D. One definition of the changes of the branch then serves
  every changed-file check.
- **Previous examples:** None found.
- **Related:** `plans/follow-ups/file-length-lint-skips-uncommitted-edits.md`.
  Shared wider fix: both checks decide which changes of the branch they
  see, and the same helper can define them for both.
- **Recommendation:** A. One changed command closes the gap in CI, where
  every change of the branch is committed. D best protects the codebase,
  because one definition then serves every check.
- **Action:** ask the user, because the fix makes the gate fail on
  whitespace errors that pass today, and a stricter check needs the user.
