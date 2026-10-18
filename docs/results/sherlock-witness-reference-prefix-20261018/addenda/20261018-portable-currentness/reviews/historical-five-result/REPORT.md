### Actual-Result Review — Saved-Record Reference Projection (`witness-reference-projection-actual-result-review`)

**Reviewer Identity:** Same independent non-author Gemini 4 Argon (High) ordinary source reviewer (`CID: fe7486d5-c674-4f04-9f52-ef41e1eff966`; standing Opus outage fallback; continuing scoped review context, not a new source/security vote)  
**Mode:** Inline data-only audit of the supplied execution receipts, factored result tables, and raw streams; zero tools, commands, file searches/reads/writes, provider calls, or subject/native/model executions performed.

---

### 1. Verdict Summary

- **ACTUAL 5-OPERATION EXECUTION VERDICT:** **`PASS / VERIFIED` (Scoped Ordinary Runtime QA Execution)**
- **PUBLICATION / ARCHIVAL ELIGIBILITY:** **ELIGIBLE** to record and publish the scoped runtime QA execution receipts under [`actual-five-01/RESULT.json`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/actual-five-01/RESULT.json) (`bytes=66419`, `sha256=0d974b2b53517b15678cc1498f4ff74736fa319c457829e8404aa7052adc4980`), subject to the exact scope boundaries in Sections 3 and 4 (including that canonical worktree portable promotion of the 4 `include_str!` paths is a separate unrun source patch requiring its own fresh independent source audit).

---

### 2. Verification of the 5 Executed Operations, Streams, Dep-Info, and Pin Invariants

#### A. Command & Toolchain Descriptor Binding (`actual_commands` vs. [`PROPOSED.json`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/PROPOSED.json))
- All 5 executed commands in [`COMPLETE-ACTUAL-DATA-FACTORED.json`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/review/COMPLETE-ACTUAL-DATA-FACTORED.json) (`bytes=80345`, `sha256=09f39961f9a39e611a8612d1eda888edaf48bb36aeddaa76391b6731d339a8cc`) match the exact 5 selected operations in [`PROPOSED.json`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/PROPOSED.json) (`bytes=14411`, `sha256=efb5bd930153604d6925aa4c1010428f07ebacedffd9542da9888fe4d4c909bc`), bound by [`ROOT-SELECTION.json`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/actual-five-01/ROOT-SELECTION.json) (`bytes=1422`, `sha256=3945b1d8943c0d94d637583ee9ebaae808e1a1e4f619d00bfa632b56a3c1c19a`) and captured by the unchanged R435 revision02 callback [`run_selected_13.py`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-execution-preparation/revision02/run_selected_13.py) (`bytes=15371`, `sha256=ce0482fdc12e4146500fa4421f7e4333b14321bdaa207c123a09ee6cb34c9932`) with `serialFailStopNoRetry: true` (`noRetry: true`) and no rerun prefix.
- Per [`ROOT-SELECTION.json`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/actual-five-01/ROOT-SELECTION.json), the frozen descriptor toolchain path `/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu` (`@TC@`, pinned in `inventory[0..=45]`) is authoritative over the prior report's prose paraphrase (`stable-x86_64-unknown-linux-gnu`).

#### B. Phase-by-Phase Execution, Stream, and Test-Membership Audit

| Phase ID | Exit | Controller Failure | Elapsed (s) | `stdout.bin` (B / SHA-256) | `stderr.bin` (B / SHA-256) | Full Retained | Static Pins Unchanged | Generated Pins | Test Membership (`selected / pass / fail / ign / meas / filt`) |
|---|---:|---|---:|---|---|---|---:|---:|---|
| `01-compileNormalCoreRlib` | `0` | `null` | `4.310214` | `0` (`e3b0c442...`) | `5553` (`7e7e7d21...`) | `true` / `true` | `486` | `2` (`486..=487`) | `null` (compile) |
| `02-compileWitnessReferencePrefixConsumer` | `0` | `null` | `0.487589` | `0` (`e3b0c442...`) | `0` (`e3b0c442...`) | `true` / `true` | `486` | `4` (`486..=489`) | `null` (compile) |
| `03-factorial_three_complete_reference_prefix_and_pending_caller` | `0` | `null` | `1.480253` | `185` (`7b8493d0...`) | `0` (`e3b0c442...`) | `true` / `true` | `486` | `4` (`486..=489`) | `1 / 1 / 0 / 0 / 0 / 2` |
| `04-list_two_five_complete_reference_prefix` | `0` | `null` | `1.343955` | `164` (`93251036...`) | `0` (`e3b0c442...`) | `true` / `true` | `486` | `4` (`486..=489`) | `1 / 1 / 0 / 0 / 0 / 2` |
| `05-tree_four_one_three_complete_reference_prefix` | `0` | `null` | `2.595384` | `170` (`b7deacd1...`) | `0` (`e3b0c442...`) | `true` / `true` | `486` | `4` (`486..=489`) | `1 / 1 / 0 / 0 / 0 / 2` |

- **All 10 Raw Streams Inspected Byte-Exact:**
  - `01-compileNormalCoreRlib/stderr.bin` (`5,553`B, `sha256=7e7e7d218c3889e8069d3bbe869116a751c06d6468f5b18760456006bd880fdc`) contains the exact same **11 preexisting warnings** in unchanged modules (`owner_ext_t3_uses.rs` [2], `owner_si21_bind_match.rs` [1], `owner_runtime.rs` [5], `recursive_source.rs` [1], `draft_source.rs` [1], `draft_typecheck.rs` [1]; `5,520 + 11 * 3 = 5,553`B due to the 3-character longer QA directory name). It emits **0 warnings** in [`mod.rs`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/snapshot/crates/mori-core/src/sherlock/helper_machine/mod.rs), [`reference_projection.rs`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/snapshot/crates/mori-core/src/sherlock/helper_machine/reference_projection.rs), or `witness_plan*`.
  - `02-compileWitnessReferencePrefixConsumer/stderr.bin` is `0` bytes (`sha256=e3b0c442...`), confirming **0 compiler warnings** in [`sherlock_witness_reference_prefix_positive.rs`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/snapshot/crates/mori-core/tests/support/sherlock_witness_reference_prefix_positive.rs).
  - `03`, `04`, and `05` `stdout.bin` streams (`185`B, `164`B, `170`B) each confirm `running 1 test`, exact test name `... ok`, and `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out`, with `0`-byte `stderr.bin` streams.

#### C. Dep-Info Source Membership (`91` Core + `5` Consumer Inputs)
- **Phase `01` (`build/core-rlib.d`, `50,509`B, `sha256=cae5d50f33ceda8f9039bd4bcc34594cdcb56b7acaab97c021ce767be99ebc2a`):** `depInfoInputs` records **91** active normal `mori_core` source files (the `90` prior normal files plus index `480` = [`reference_projection.rs`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/snapshot/crates/mori-core/src/sherlock/helper_machine/reference_projection.rs) and index `479` = [`mod.rs`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/snapshot/crates/mori-core/src/sherlock/helper_machine/mod.rs), alongside indices `394..=398` for `witness_plan*` and lexical `include!` aliases `374`/`375` for `fragments/{legacy,scalar}.proposed.rs`, with zero `*_tests.rs` files).
- **Phase `02` (`build/witness-reference-prefix-positive.d`, `2,964`B, `sha256=231fd023bd02d05e888e1c962f6c60da41fb2d3a199591514da6408056183dbf`):** `depInfoInputs` records **5** inputs (`[485, 482, 483, 484, 481]`): [`sherlock_witness_reference_prefix_positive.rs`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/snapshot/crates/mori-core/tests/support/sherlock_witness_reference_prefix_positive.rs) (`481`) plus all **4** frozen oracle JSON fixtures (`482..=485`) embedded via `include_str!`.
- **Qualification:** Compiler `.d` dep-info establishes `rustc` source/include membership only, not exclusive OS-level I/O attestation or SDK-free runtime proof.

#### D. Static & Generated Pin Accounting (`486` Static + `4` Generated = `490` Total)
- Original capture in [`actual-five-01/RESULT.json`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/actual-five-01/RESULT.json) stores the per-phase aggregate count `"staticPinsUnchanged": 486` and `generatedPins` array on each phase (`2` generated files after `01`: `libmori_core.rlib` [`43,779,842`B, `sha256=4afb6609...`] and `core-rlib.d` [`50,509`B, `sha256=cae5d50f...`]; `4` generated files after `02`–`05`, adding `witness-reference-prefix-positive` [`18,756,760`B, `sha256=d121abb4...`] and `witness-reference-prefix-positive.d` [`2,964`B, `sha256=231fd023...`]), rather than per-file pre/post arrays.
- [`DATA-READBACK.json`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/DATA-READBACK.json) (`bytes=2454`, `sha256=e9408a52e58fee23ca22784d8d29026ce81ef5f1eeae1cd30b618ca4f5957e6f`) independently rehashes all `490` inventory entries (`486` static + `4` generated) and confirms `"all490currentpins_match": true`.

---

### 3. Exact Observed Meaning vs. Open Scope Boundaries

1. **What Is Now Observed:**
   - Across the 3 positive consumer fixtures (`factorial(3)`, `list [2, 5]`, `tree 4,(1, 3)`), single-run `host_owner_engine()` streaming (`wp::stream_owner_steps`) and [`reference_projection.rs`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/snapshot/crates/mori-core/src/sherlock/helper_machine/reference_projection.rs) produced:
     - Exact 5-way decoded witness equality (`evaluate_witness` uncached baseline, `ScratchReuse::LastUse` uncached, `evaluate_witness_cached` cold, `evaluate_witness_cached` warm, and streamed `s.evaluation.witness.decoded`) at every step boundary and terminal state;
     - Exact match of all 8 comparison fields (`complete_program`, `snapshots`, `records`, `prefix_work_A`, `charged_A_work`, `A_event_count`, `terminal_outcome`, `logical_depth_peak_A`) against the 3 independently frozen full-prefix A39 oracles (`115` snapshots / `114` records / work `113` / depth `9` / `Done(6)` for `factorial(3)`; `70` snapshots / `69` records / work `68` / depth `8` / `Done(7)` for `list [2, 5]`; `104` snapshots / `103` records / work `102` / depth `8` / `Done(8)` for `tree 4,(1, 3)`);
     - Exact match of the `factorial(3)` pending-caller slice (`A_snapshot_indices = [22, 32, 35, 36]`) against [`factorial-pending-caller3.json`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ035-independent-reference-prefix-oracle-20261018/expectations/factorial-pending-caller3.json) and exact `source_occurrence` join at every projected step.
2. **What Remains Open (`whole035Closed: false`, `nativeSelected: false`):**
   - **B Absolute Clock & Resource Bounds:** Projected A work (`charged_A_work`) uses the fixed 39-rule `{0, 1}` table in [`reference_rule`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/snapshot/crates/mori-core/src/sherlock/helper_machine/reference_projection.rs#L487-L531); an independent upper bound or equivalence theorem for B's raw `work_ChargedTotal` / `fuel_Remaining` across multi-step H1 intervals remains **OPEN**.
   - **Unexercised Variants & Negative / Counter-Cases:** The 3 positive fixtures exercise 7 of the 9 `a::Frame` variants (`Frame::Let` and `Frame::Unary` are unexercised, as are `Pattern::Var`, `Control::Error`, `Outcome::Rejected`, `Outcome::Exhausted`, and negative `ProjectionGap` rejection cases).
   - **Physical Scope, `ProjectRel`/`Link`, Native/Proof, and Retained Requirements:** Elapsed wall times (`4.31s`, `0.49s`, `1.48s`, `1.34s`, `2.60s`) are ordinary execution completion measurements, not speedup or memory benchmarks. General H1 physical scope, canonical `ProjectRel`/`Link` projection, native/proof/stagenet targets, and whole `UIREQ-035`, `SCON-013`, and `SCON-024` remain **OPEN**.

---

### 4. Portable Promotion & Canonical Integration Boundary

- Cache/stream PR #251 is already merged on canonical branch `sherlock`; no replay or retargeting of that prior slice is needed.
- Promoting [`sherlock_witness_reference_prefix_positive.rs`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-ordinary-QA-20261018/snapshot/crates/mori-core/tests/support/sherlock_witness_reference_prefix_positive.rs#L36-L43) into the canonical repository by replacing its 4 absolute `include_str!("/home/charl/Moriarty/.moriarty-dev/runtime/...")` literals with portable relative paths (alongside the 4 byte-identical opaque JSON oracle files) creates **new modified source bytes** in an **unrun proposal**.
- This historical runtime QA `PASS` applies strictly to the pinned snapshot bytes (`abf69577b7538c9ddd04558dba3b5ce68dc50971d5bbdef252dcad343f3b7980`) and **does not pre-approve new portable bytes**; the 4-literal relative-path patch requires a fresh independent source audit before canonical adoption by Root.

