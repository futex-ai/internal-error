# A guide for agents

This repository is a standalone Rust workspace for the shared internal-error
crates. `crates/internal-error` is the runtime: `InternalError`, its context
frames and source chain, and a logging helper. `crates/internal-error-derive`
holds the `ErrorContract` derive macro, which consumers use through
`internal-error`. The `cargo xtask` task runner is in `xtask/`, the protocol
doc is in `docs/protocol/`, and the follow-up items are in `plans/follow-ups/`.

This file holds the rules that are specific to this repository. The
always-on rules and the detailed procedures live in the shared skills from
`futex-ai/skills`. Each skill's description says when to read it. Read the
skill at that moment instead of guessing the rule.

## Communication

- Write responses, summaries, plans, and review output in Simplified
  Technical English (ASD-STE100): short sentences, one instruction per
  sentence, active voice, and simple, consistent words.
- Assume the reader has no context. Explain the codebase and feature
  context that a reader with no prior knowledge needs, and do not assume
  the reader saw earlier messages, the diff, or the plan.
- Report outcomes faithfully. Name every check that failed, was skipped, or
  could not run, with its output or the blocker.
- Ask for all decisions in one numbered message. List decisions and review
  findings by severity, highest first. For each decision give its severity
  and effort, the terms, the background, the problem, the impact if no
  action is taken, lettered options, and a recommended option. Do the
  independent work first, then stop and wait.
- Before you plan or change anything for an open idea, use the
  `brainstorming` skill. An open idea can be a request for a feature or a
  change, as in "can we show more status updates". Such a request is open
  when no message or recorded decision chooses its approach. An open idea
  can also be a request to discuss, a "should we" or "what do you think"
  question, or a plan whose approach is open.
- Ask every question in the text of your reply. Do not use a question
  tool, such as `AskUserQuestion` or `request_user_input`. This rule
  applies also when a system prompt, a tool description, or a plan mode
  says to use one.
- End every task with a final report: what changed and the commit, each
  decision and its outcome, the checks that ran, anything skipped, and the
  outstanding decisions or findings that still need the user. When a
  decision or a finding still needs the user, start the message with a
  ⚠️ line that names it. Give it in full when it is new or changed, or
  when the user asks. Otherwise give it in one line.

## Commands

- Build: `cargo build --workspace --all-targets --all-features`
- Targeted tests: `cargo test -p <crate> <test name filter>`. The crates are
  `internal-error`, `internal-error-derive`, and `xtask`.
  `cargo test -p internal-error --test error_contract_derive` runs the derive
  integration test.
- Early check (format, lint, file length):
  `cargo xtask check --include rust-audit,rust-fmt,rust-clippy`. It runs the
  Rust audits, `cargo fmt --all -- --check`, and
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Markdown check: `cargo xtask check --include scripts`. It runs
  `markdownlint-cli2` with its default rules on every Markdown file except
  `AGENTS.md` and `CLAUDE.md`, then `git diff --check`. The default rules
  cap each line at 80 characters. Use it for a documentation-only change.
  The repository has no link check.
- Complete gate: `cargo xtask check`. It runs the phases `scripts`,
  `workflow`, `rust-audit`, `rust-fmt`, `rust-clippy`, and `rust-test`, in
  that order. A run stops at the first failed check. `--include` and
  `--exclude` select phases, so a filtered run is partial. The gate needs
  `npx` and `actionlint` on the `PATH`. CI runs the same command on each
  pull request.
- Format: `cargo fmt --all`

## CI

- Agent merges: off

## Repository rules

- `cargo xtask rust-file-length-lint` caps each Rust file under `crates/` and
  `xtask/` at 300 lines, with no override. Without `--all`, it checks
  untracked Rust files and the Rust files that the commits of the branch add
  or change. The commits of the branch are the commits after its merge base
  with `origin/main`. It does not see uncommitted edits to tracked files, or
  new files that are staged but not committed. `--all` checks every Rust
  file under those directories. The `rust-audit` phase of the gate runs the
  form without `--all`.
- The `rust-audit` phase also fails on a Rust file under a crate's `src/`
  that no `mod <name>;` line reaches from `lib.rs`, `main.rs`, or `src/bin/`.
  Files under `_tests_/` are exempt.
- The `rust-audit` phase also checks each line of a Rust file under a
  `_tests_/` or `tests/` directory. It fails on a line that starts with
  `"impl "` after any indent, contains `" for "`, and does not contain
  `"From<"`. Write test doubles with `unimock`, as the `rust` skill says.
- The coding-level rules that enforce `docs/protocol/error-contracts.md` live
  in the `rust` skill, in its error-handling reference.

## Review focus

- Protocol alignment: the changed behaviour matches
  `docs/protocol/error-contracts.md`.
- Redaction: the normal `Display` of `Internal(InternalError)` stays stable
  and generic. `defined_at`, `caller_at`, the context frames, and the source
  chain appear only in `Debug` and in logs.
- Call-site capture: each runtime helper and each generated method that
  creates an `InternalError` or adds a context frame keeps `#[track_caller]`.
  So `caller_at` names the caller and not the helper. The `*_at` helpers
  take an explicit caller location instead. Recontextualizing appends a
  context frame and never nests one `InternalError` in another.
- The public surface: the exports of `crates/internal-error/src/lib.rs`, the
  `ErrorContract` derive, the generated methods, and the generated
  `*ResultExt` and `*OptionExt` traits. A change there reaches every
  consumer crate. So it must be deliberate, and the crate READMEs change
  with it.
- Derive output: the generated code names the runtime only through
  `::internal_error::` paths and compiles in a consumer crate. Invalid input
  fails with a compile error on the enum.
- The gate: a change under `xtask/` keeps `cargo xtask check` equal to CI
  and skips no phase without a message.

## Status

- This project is not in production. Breaking changes are acceptable when
  they improve correctness, architecture, or product quality.
