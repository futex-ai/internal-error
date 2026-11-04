# Protocol doc holds coding rules and code names

- **Severity:** Low. **Effort:** Medium. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Fix after merge.
- **Category:** Docs or spec.
- **Added:** 2026-11-04, from the branch `conductor/agents-md-internal-error`.
- **Location:** `docs/protocol/error-contracts.md:16`, `:79`, `:147`, and
  `:165`. `crates/internal-error/README.md:145`.
- **Reason:** On 2026-11-04, the user chose a separate branch for this doc,
  because it names crates and a protocol doc change needs its own review.

- **Terms:**
  - **Protocol doc:** a spec file under `docs/protocol/` that holds the
    requirements and the decisions in force for one area.
  - **Documentation test:** the check of the `documentation` skill that keeps
    a protocol doc sentence only when a different implementation could
    satisfy it.
  - **Pointer line:** the line under `## Repository rules` in `AGENTS.md`
    that names the `rust` skill as the home of the error-handling rules.
- **Background:**
  - `docs/protocol/error-contracts.md` defines the error contract that the
    crates `internal-error` and `internal-error-derive` implement.
  - Its last sentence, at line 165, says that the coding-level enforcement
    of its rules lives in `AGENTS.md`.
  - On 2026-11-04, the branch `conductor/agents-md-internal-error` removed
    the error rules from `AGENTS.md`. It added the pointer line instead.
  - The pointer line names the `rust` skill of `futex-ai/skills`. Its
    error-handling reference, `references/errors.md`, holds the same rules.
  - The doc names the two crates at line 16.
  - From line 79, the doc describes the generated helpers and the mechanics
    of the `defined_at!()` macro. That text already differs from the code.
    The derive does not call `defined_at!()`. It calls `DefinedAt::new`
    with the same arguments, at
    `crates/internal-error-derive/src/error_contract_derive/expansion/inherent.rs:9`.
  - The doc section "Implementation Rules", from line 147, lists coding
    rules for consumer code.
  - Some sentences are requirements of the library and stay. Examples are
    what an `InternalError` must carry and that `Display` stays generic.
  - `crates/internal-error/README.md` links to `AGENTS.md` under
    `### Related Docs`, at line 145.
  - The `documentation` skill of `futex-ai/skills` keeps code names,
    mechanics, and coding rules out of protocol docs. It also says that a
    change that touches a protocol doc moves such content to its home in the
    same change.
- **Problem:** The doc mixes requirements with code names, mechanics, and
  coding rules. Its last sentence points to `AGENTS.md` for rules that
  `AGENTS.md` no longer holds.
- **Impact if no action:** A reader who follows the last sentence finds only
  the pointer line in `AGENTS.md`. A reviewer can take a coding rule or a
  mechanic as a requirement of the library. The mechanics drift further
  from the code.
- **Root question:** What does the error-contract protocol doc hold, and how
  does a reader find the coding rules?
- **Options:**
  - **A.** Apply the documentation test to each sentence. Keep the
    requirements. Move code names and mechanics to
    `crates/internal-error/README.md` and
    `crates/internal-error-derive/README.md`, which already describe most of
    them. Remove the coding rules that the `rust` skill holds, and the stale
    last sentence. Then remove the pointer line from `AGENTS.md`, or point
    it at the new text. Remove the `AGENTS.md` link from
    `crates/internal-error/README.md` when the pointer line goes.
    Effort: Medium.
- **Left out:** Change only the last sentence. The `documentation` skill
  requires the full move when a change touches a protocol doc. Keep the doc
  as it is. That fails the goal.
- **Wider fix:** None. The `documentation` rule is already in force.
- **Previous examples:** `futex-ai/ai` recorded
  `plans/follow-ups/move-ai-mcp-conventions-checklist.md` for agent rules
  inside a protocol doc.
- **Recommendation:** A. The protocol doc then holds only requirements, and
  each rule that it drops already has a home.
- **Action:** fix after merge, apply A, because the user chose on 2026-11-04
  to give the protocol doc its own branch.
