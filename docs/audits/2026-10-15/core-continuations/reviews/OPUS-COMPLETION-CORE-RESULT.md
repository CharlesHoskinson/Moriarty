**Verdict: NEEDS_CHANGES, for missing tests only.** I found no bug in the Core source, but eight hostile tests the frozen design requires were never written or run. The report is at `R/OPUS-COMPLETION-CORE.md`, and it was my only write.

**What I read:**
- **Source:** all eight Core stage modules in full, the complete ir/parser/json/lib/logical_cost/runtime diffs, and the runtime functions you named (`scenario_snapshot`, `execute_statement`, `run_segment`, the evaluator and the shared meter). The `sha256sum --check` of the candidate checksums passed.
- **Tests:** the four Core test files in full.
- **Results:** stdout, stderr, result and start files for all four Core checks. They show 32 passes and 0 failures, every stdout hash matches, and every start file pins the current sources, the tests and the oracle (`76075b63…`). The oracle fixture is byte-identical to the independent oracle. I recounted the escrow costs (5, 6, and 2 for a false guard) and they match. I did not read the 4,488-line oracle fixture line by line; I relied on the hash match.
- **Unread within scope:** nothing. Backend, CLI and ledger code were out of scope, and I make no security claim about them.

**The two open items from the first pass are resolved; neither is a bug:**
- **Wrong-typed start inputs:** `runtime.rs:572-585` enforces the exact input count and types, and `admit.rs:236` runs that check before any work, so a bad input is refused with zero work done.
- **Let/state shadowing:** the static checks at `validate.rs:154/189/206/397` plus the exact capture and event matching at resume mean the runtime overlay can never shadow a state.

**Also confirmed in source:** a single shared evaluator with no second semantics, frozen `/1` serde with Await refused, unchanged strict-JSON limits, the resume checks running in design order, exact static cost enforcement, the cap never being raised, and strict boundary ordering. The ordering matters: liability reads use a binary search, so they are only correct because unsorted boundaries are refused.

**Findings:**
- **F1 (Medium, required):** these design hostile cases have no test in any Core suite, and grep finds none in the CLI tests either:
  - H6: duplicate JSON keys
  - H7: non-canonical number `"010"`
  - H9: unsorted or duplicate boundary balances
  - H10: tampered liability identity
  - H11: emptied escrow balance
  - H12: removed liability
  - H14: tampered capture accepted (characterization)
  - H24: oversized continuation (4 MiB+1) and event (65,537 bytes)

  From the code, I expect all of them to pass. Fixing this needs only new tests, an executed and classified run, and a fresh Opus pass on that change.
- **F2 (optional):** no tests for bad start inputs through `stage::start`, missing or extra states in a resumed boundary, or liabilities out of order.
- **F3 (optional):** the unit tests inside `cost.rs` are not among the four captured test runs.
- **F4 (info):** a boundary with several defects can report a different first error than the legacy check would. No security impact.

**README change:** acceptable. It matches the 32 Core passes I verified; I did not verify the "58 checks" total or the CLI runs. Its word "exercised" does not cover the hostile cases in F1.

This verdict covers subscope A only. The final integrated review of the whole feature is still required.
