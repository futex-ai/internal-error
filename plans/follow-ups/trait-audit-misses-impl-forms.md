# Trait audit misses some impl forms

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Category:** Repository rule.
- **Added:** 2026-11-04, from the branch `conductor/agents-md-internal-error`.
- **Location:** `xtask/src/rust_trait_audit.rs:43`, the function
  `is_trait_impl_line`.
- **Reason:** The code is in `xtask`, and the branch that found the gap
  changed only agent rules and docs.

- **Terms:**
  - **Trait audit:** the check in `xtask/src/rust_trait_audit.rs` that the
    `rust-audit` phase of `cargo xtask check` runs on each Rust file under a
    `_tests_/` or `tests/` directory.
  - **Test double:** a stand-in implementation of a trait that a test uses
    instead of the real implementation.
- **Background:**
  - The trait audit reads each line of a test file. The function
    `is_trait_impl_line` flags a line when, after trimming, it starts with
    `"impl "`, contains `" for "`, and does not contain `"From<"`.
  - The audit reports each flagged line as a handwritten trait double, and
    the gate fails.
  - The `rust` skill of `futex-ai/skills` asks for `unimock` test doubles at
    trait boundaries. `AGENTS.md` describes the audit under
    `## Repository rules`.
  - `rustfmt` puts `for` on its own line when an impl header is too long. A
    check on 2026-11-04 confirmed this.
  - `syn` 2.0.117 is already in `Cargo.lock`. `clap_derive` builds it with
    the `full` feature, which the `syn::parse_file` function needs.
- **Problem:** The audit misses three forms of a trait implementation:
  `impl<T> Trait for Type`, `unsafe impl Trait for Type`, and a header that
  puts `for` on its own line. It also lets `impl TryFrom<A> for B` pass,
  because that line contains `"From<"`.
- **Impact if no action:** A handwritten test double in one of these forms
  passes the gate. Only a review can then catch it.
- **Root question:** How does the gate catch every handwritten trait double
  in test code?
- **Options:**
  - **A.** Match the whole impl header, also when it has generics or
    `unsafe`, or spans lines. Exempt only an impl of the `From` trait. Add
    tests. A `TryFrom` impl or an `unsafe impl Send` in test code then fails
    too. Effort: Small.
  - **B.** Keep the line pattern, and leave the other forms to review.
    Effort: Small.
  - **C.** Parse each test file with `syn`, and flag each `impl` item that
    names a trait, except `From`. xtask gets a new direct dependency.
    Effort: Medium.
  - **D.** Flag a trait impl only when the workspace declares the trait.
    Collect the trait names from the production files, and match the impl
    headers as A does. Impls of standard traits, such as `Display` and
    `TryFrom`, then pass. A trait name that another crate also uses can
    match by mistake. Effort: Medium.
- **Left out:** None. The four options cover a wider pattern, a parser, a
  narrower target, and no change.
- **Wider fix:** C. A parser sees each impl item, whatever its layout.
- **Previous examples:** None found.
- **Related:** `plans/follow-ups/audits-read-parent-directory-names.md`.
  Same code: both items change `xtask/src/rust_trait_audit.rs`, so one
  branch can fix both.
- **Recommendation:** A. It closes each known gap with a small change to one
  function. C best protects the codebase, because a parser does not depend
  on the layout of the code.
- **Action:** ask the user, because each fix option changes what the gate
  rejects, and the user decides each stricter or weaker check.
