## Verdict: not approved. The review is incomplete, and I found no source defect in the files I read.

- **Source security: not approved.** I read about a quarter of the security-relevant source. That covered the Core admission, decoding, digest and runtime-segment code plus parts of the cost and validation code, and none of it had a defect. I'm not marking it NEEDS_CHANGES, because I found nothing concrete to fix, and I'm not approving it on missing evidence.
- **Actual results: not approved, not verified.** I never checked that the 58 captured tests ran against the candidate commit `e9a2b80b`. The permission layer refused the read-only pin-to-blob hash comparison. The Core summary also records base `fe01bc8b` and says "candidate and actual result audits remain owed".

The full report is at `R/OPUS-FULL-CANDIDATE-REVIEW.md`. It is the only file I wrote. I ran only the guarded `status` call, and native authority stays at 0.

**Confirmed in source** (behaviour as designed):
- **Input limits:** continuation and event files are size-capped before parsing (`decode.rs:19-33`), and unknown fields are refused.
- **Resume checks:** the eight resume checks (R0 to R7) run in the designed order, and every refusal reports zero attempted work with the input's committed value unchanged (`admit.rs:279-373`).
- **Guards and cost:**
  - A false guard returns the input continuation unchanged, with no new segment and no committed cost.
  - Measured work must exactly equal the statically planned cost, with each segment's exit site and committed work checked against the plan.
  - Resume records the await step at its original statement position, and positive remaining duties are reported explicitly.
- **Untrusted values:** capture values and the stored cap are untrusted and caller-editable, with no replay or consumption protection. This matches the design, and the `none` / `untracked` / `local-reference-stage` labels are present (`mod.rs:37-41`).
- **Backend:** staged `/2` relations refuse a signed target (`context.rs:35`).

**Open items to check, not findings yet:**
1. `start` only checks that each input is present (`admit.rs:239-249`). Rejecting a wrong-typed input depends on `scenario_snapshot`, which I didn't read.
2. On resume, captures and event values are added on top of state values (`runtime.rs:956`). I haven't confirmed that `let` names, and so capture names, can't reuse a state name. If they could, a capture could replace a state value during evaluation.

**Not read:**
- **Backend:** `frame.rs`, `artifact.rs`, `lower.rs`, and most of `context.rs`. This is the `/4` context-29 and pending null/zero layout.
- **Rest of the source:** the CLI commands, the ledger changes, and the `ir.rs`, `parser.rs` and `json.rs` diffs.
- **Tests:** all eight test files.
- **Results:** every original stdout/stderr capture and the comparison against the frozen oracle.
- **Baselines:** the `/1` and `/3` byte baselines.
- **Documents:** the frozen design and precedence documents, the security addenda, the product contract and the U3 EARS requirements.

An Opus approval for this candidate needs a completed pass over those files, the pin-to-candidate hash check, and inspection of the 58 results, including the earlier failures that were kept on record.
