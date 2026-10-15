All four commands are **APPROVE_ORDINARY**, subject to seven premises (P1–P7). I didn't run anything. Native authority stays at 0, and this is not acceptance of the feature. The report is at `R/OPUS-CORE-CHECK-CLASSIFICATION.md`.

**Model identity:** Claude Opus 5.5 (`claude-opus-5-5`), high effort, completed.

**Pins:** all 25 entries match the worktree, including the oracle (`76075b63…9640`). Two files compiled into the tests through `include_str!` are not pinned. Add these before running:
- `examples/core/continuations/escrow.mori`: `2eb952dd267922c968764154e9d29fbc17f40f5e45fe8b5dac74d229f3d5d60a`
- `examples/core/continuations/reserve.mori`: `260f50443d31e58f62d83c8f7f00f3c110e3c116d216feb93f680f0b92224530`

Also check that `git diff fe01bc8b` is clean for mori-lexer, mori-diagnostics and mori-span. The parser calls the lexer, and those crates are unpinned.

**Two corrections to the request:**
1. `stage_static_cost` sets 64 proptest cases per property, not 32 (3 properties × 64). `stage_hostile` does use 32.
2. A failing proptest writes a seed file inside the worktree, not outside it. Proptest finds no `lib.rs` above `tests/`, so it falls back to `crates/mori-core/tests/<target>.proptest-regressions`. Neither file exists today. I classed it as an ordinary public artifact that appears only on failure. It should be reported, kept untracked, and neither committed nor deleted silently. If one exists before the run, it changes the test inputs and needs a recheck.

**Build effects:**
- The two Midnight SDK dev-dependencies (`midnight-base-crypto`, `midnight-serialize`) get compiled. That pulls in reqwest/rustls, aws-lc-sys and ring (C builds), plus their build scripts and proc macros. These are ordinary local, offline compile effects.
- None of the four test files references the SDK crates, so they are never linked or executed. No parameter fetching, data provider or file-writing SDK code runs.
- k256 is linked as part of mori-core, but only the intent signature functions reach it, and no selected test calls them.
- The only filesystem call in mori-core is `decode::read_bounded`, and none of these tests reaches it.

**Call chains:**
- **continuations, stage_static_cost, stage_hostile:** `stage::load` (parse, validate, plan, cost, then SHA-256 over canonical JSON), then `start`/`resume`, which run checks R0–R7 and the single shared evaluator, `runtime::run_segment`.
- **compat_v1:** only the v1 wire deserializers, the strict `json::parse`, the legacy `simulate`, and `validate`. No stage code.

The full chains are in the report.

**Premises:**
- **P1:** all pins, including the two new `.mori` pins and the clean lexer diff, still match when the commands run.
- **P2:** exactly these four commands run, in order, stopping at the first failure.
- **P3:** the child has no `PROPTEST_*` or extra `CARGO_*`/`RUSTC_WRAPPER` variables. I recommend unsetting `PROPTEST_*` by name, because `PROPTEST_FORK` or `PROPTEST_TIMEOUT` would re-run the test binary as a child process.
- **P4:** the SDK crates stay unreferenced in all four test files.
- **P5:** no Cargo config in `/home` or `/` (none in the worktree or home directory), and the installed 1.99.0 toolchain is used.
- **P6:** no seed file exists before the run.
- **P7:** the shared target directory holds only public compiled objects.

**Not inspected:** I didn't read the bodies of the evaluator, plan, cost and validate modules line by line. Their effect classification rests on a pattern search of the whole source, not a correctness audit. I also didn't inspect the third-party build scripts, the C sources, or the rest of the SDK source; P4 makes those compile-only. The Muse and Opus reviews of the candidate and actual results are still owed.
