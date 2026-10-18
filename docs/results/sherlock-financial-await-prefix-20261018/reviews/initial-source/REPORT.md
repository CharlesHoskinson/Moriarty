# Fresh Non-Author Security / Normal Source + Ordinary-Operation Review: Core/2 `AwaitPrefixObservation` & Publication Increment

## 0. Reviewer Identity, Independence & Mandatory Read Coverage Declaration

- **Reviewer Identity & Role**: `Gemini 4 Argon (High)` (`actualGemini4ArgonHIGH` under standing observed Opus quota-outage fallback), acting as **one genuinely fresh designated non-author security/normal source + ordinary-operation reviewer** (1 context = 1 opinion, never 2 votes).
- **Separation of Duties**: Not the production author (`financial-await-projection-implementation-20261018`), not the independent expectation author (`financial-await-projection-independent-expectations-20261018`), and not the positive consumer/QA preparer (`financial-await-projection-positive-consumer-20261018` / `financial-await-projection-ordinary-QA-20261018`). No reused author conversation ID or prior vote. Existing canonical `TypeId` review `82741`, native, and SDK lanes remain untouched; stopped `Am10` terminal state is preserved.
- **Tool & Read Discipline**: Every listed page (`86/86` pages: `01-page-01.txt` through `51-page-01.txt`, covering all `51` delivery bodies and all `75` complete original files `ORIGINAL01`..`ORIGINAL75`) was read in full using **only** `view_file` with exact `AbsolutePath` and whole-file/default parameters (`0` shell/compiler/test/git/write calls, `0` unlisted paths).
- **Clipping / Truncation / Gaps**:
  - **Clipped or truncated page reads**: `0` (`86/86` pages read completely; each page $\le 16{,}000\text{ B}$ / $\le 401\text{ lines}$).
  - **Missing mandatory critical source files**: `0` (`GAP = NONE`; no `ABSTAIN` triggered).
  - **Factored Build Closure**: Full critical Core/2 production, financial projection (afterimage `b2d5765d...`, preimage `54ebd690...`, and diff `aed23f40...`), stage (`mod`, `admit`, `plan`, `cost`, `decode`, `digest`, `profile`, `validate`), evaluator (`runtime`, `parser`, `ir`, `logical_cost`, `policy`, `json`), and retained `R191` lexer (`diagnostics`, `lib`, `number`, `scan`, `string`, `token`, `lexer_harness`) sources were read in full (`ORIGINAL01`–`ORIGINAL23`, `ORIGINAL57`–`ORIGINAL59`, `ORIGINAL64`–`ORIGINAL65`). The remaining `141` unrelated `mori-core` source files (out of `157` total in `SOURCE-COMPOSITION.json`, `SHA0a158fcc224e49cc03b69a5e43cccaa7405328c0caad21873537939c4f9ca68c`) and `323` normal tool/dependency inputs (`490` total pins in `FLAT-INPUT-PINS.json`, `SHAdee8d4e752c8330a1a8da9e5e10d162fad34fea956987a4d1aff8ec149115ce6`) are factored as byte-pinned compilation inputs (`ORIGINAL74`), not claimed as a fresh semantic re-audit.

---

## 1. Source & Security Financial Correctness Verdict

### Verdict: `APPROVE_SCOPED_SOURCE_AND_SECURITY_SEMANTICS`
*(0 mandatory source code fixes required for the scoped additive Core/2 `AwaitPrefixObservation` and `publication()` projection; mandatory semantic/security scope limits below apply.)*

### Exact Code & Security Assessment

1. **Exact Production Diff (`ORIGINAL64` $\to$ `ORIGINAL02`, `ORIGINAL65` `SOURCE.diff` `SHAaed23f402fe608df163d7af4546f4f73c107c63b446386d4c59828f611622815`)**:
   - `crates/mori-core/src/sherlock/financial_projection.rs` changes from `21,297B` (`SHA54ebd690ce5748e970b579ab6eb43fe7a9353f190017675abec4924df6368a6f`) to `24,351B` (`SHAb2d5765d5a6bfcbd2139f4df7130bbddc85268e891ea63ccf60a2974040f65df`).
   - Adds `pub enum AwaitPrefixObservation<'a>` (`ORIGINAL02` lines `79–93`):
     ```rust
     pub enum AwaitPrefixObservation<'a> {
         Waiting {
             advanced: bool,
             action: &'a str,
             site: u16,
             captures: &'a [NamedValue],
             logical: &'a StageLogical,
             committed_boundary: &'a Snapshot,
             residual: &'a [Liability],
             published_segment: Option<SegmentObservation<'a>>,
             attempted: UInt,
         },
         Complete,
         RejectedNoReturnedCheckpoint,
     }
     ```
   - Private helper `fn project_await_prefix(result: &Result<StageOutcome, StageRejection>) -> AwaitPrefixObservation<'_>` (`ORIGINAL02` lines `97–127`) exhaustively destructures `Ok(StageOutcome::Waiting { advanced, segment, continuation, attempted, residual })` and `let Continuation { format: _, program_hash: _, plan_hash: _, action, site, captures, logical, boundary } = continuation;`, borrowing `action`, `captures`, `logical`, `committed_boundary: boundary`, `residual`, and `published_segment: segment.as_ref().map(project_segment)`, and copying `*advanced`, `*site`, and `*attempted`. `Ok(StageOutcome::Complete { segment: _, committed: _, attempted: _, residual: _ })` maps to `AwaitPrefixObservation::Complete`, and `Err(StageRejection { error: _, attempted: _, committed: _ })` maps to `AwaitPrefixObservation::RejectedNoReturnedCheckpoint`.
   - Adds `impl<'a> InvocationObservation<'a> { pub fn await_prefix(&self) -> AwaitPrefixObservation<'a> { project_await_prefix(self.result) } }` (`ORIGINAL02` lines `335–342`).
   - Updates `ProducedInvocation::publication(&self)` (`ORIGINAL02` lines `543–574`):
     ```rust
     let invocation = self.observation();
     let await_prefix = serde_json::to_value(invocation.await_prefix())?;
     let observation = serde_json::to_value(invocation)?;
     let object = report.as_object_mut().expect("stage reports are JSON objects");
     object.insert("financial_observation".into(), observation);
     object.insert("await_prefix".into(), await_prefix);
     ```
     All existing top-level keys (`"status"`, `"advanced"`, `"segment"`, `"continuation"`, `"attempted"`, `"committed"`, `"residual"`, `"error"`, `"financial_observation"`) and all `crate::stage::STAGE_LABELS` (`"qualification": "local-reference-stage"`, `"authentication": "none"`, `"consumption": "untracked"`) are preserved unchanged.

2. **Same-Call Ownership vs. Caller-Supplied Diagnostic Relationship Limits**:
   - `ProducedInvocation<'p>` (`ORIGINAL02` lines `494–499`) has private fields (`program: &'p StagedProgram`, `input: CapturedInvocationInput`, `result: Result<StageOutcome, StageRejection>`) and can only be constructed by `start_observed` (`lines 503–513`, calling `crate::stage::start(program, &action, scenario.clone(), cap)` once) or `resume_observed` (`lines 517–531`, calling `crate::stage::resume(program, continuation.clone(), event.clone(), cli_action.as_deref(), cli_cap)` once).
   - `ProducedInvocation::observation(&self)` (`lines 539–550`) calls `project_invocation(self.program, input, &self.result)` on that exact single-call record, and `invocation.await_prefix()` projects `self.result` from the same `ProducedInvocation` without re-running the evaluator.
   - Lower-level diagnostic functions (`project_invocation`, `project_stage_result`, `project_resume_result`) borrow caller-supplied references and cannot construct `ProducedInvocation`; they do not establish that a caller-supplied `input` or `program` produced `result`. `ResumeFinancialObservation::input_boundary(&self)` (`lines 250–252`) returns `&self.input.boundary` as unvalidated caller input, never a returned rollback continuation.

3. **Waiting `advanced: false` with `None` Segment vs. Real Empty Segment**:
   - In `crates/mori-core/src/stage/admit.rs` (`ORIGINAL04` lines `169–196`), when `run_segment` returns `SegmentRun::NotAdvanced { attempted }` (guard evaluated to `Value::Bool(false)` at `Statement::Await`), `resume` returns `Ok(StageOutcome::Waiting { advanced: false, segment: None, continuation: c, attempted, residual: owed })` where `owed = residual(&c.boundary)`.
   - `project_await_prefix` preserves `advanced: false` and `published_segment: None`, while `project_invocation` (`ORIGINAL02` lines `421–432`) maps `Ok(StageOutcome::Waiting { segment: None, .. })` to `(AccountingAvailability::NoPublishedSegment, TentativePublication::NoAdvancedSegment)`. A segment that advances with zero financial effects remains `Some(SegmentObservation)` with `PublishedSegment` and `PublishedSuccessfulSegment`.

4. **Complete / Rejection Handling, Per-Segment Financial Accounting, Residuals & Stage/Lexer Security**:
   - **No Synthetic Checkpoint or Prior-Effect Reconstruction**: `Complete` (`AwaitPrefixObservation::Complete`) and `Err(StageRejection)` (`AwaitPrefixObservation::RejectedNoReturnedCheckpoint`, `AccountingAvailability::RejectedNoPublishedDelta`, `TentativePublication::UnavailableFromStageRejection`) never fabricate a checkpoint, tentative snapshot, or rollback continuation.
   - **Per-Segment Accounting (`account_effects`, `ORIGINAL02` lines `374–408`)**: Borrows the single segment's ordered `effects: &[Effect]`, ignores `Effect::State | Effect::Liability | Effect::Emit` for monetary flow rows while keeping them in `AccountingReport.effects`, groups `Effect::Transfer` and `Effect::Fee` by exact `(AssetId, account)` in first-encounter order (visiting `from` debit before `to` credit), and uses checked `u128` addition (`value.0.checked_add(amount.0)` $\to$ `AmountAggregate::Exact(UInt(sum))` or `AmountAggregate::AboveU128`). Fees add to `fee_debit`/`fee_receipts` and `gross_debit`/`gross_receipts` once; transfers add to `transfer_debit`/`transfer_receipts` and `gross_debit`/`gross_receipts` once; reverse transfers never reduce `gross_debit`; `net_flow` is `Inflow`, `Outflow`, `Balanced`, or `Unavailable` if either gross aggregate is `AboveU128`. Accounts with no monetary effect (`observer`) get no `AccountFlow` row while remaining in `Snapshot.balances`.
   - **Liabilities & Residual Duties (`ORIGINAL11` `runtime.rs` lines `536–564`, `622–630`; `ORIGINAL04` `stage/admit.rs` lines `369–374`)**: `Statement::Discharge` reduces the liability in place and retains zeroed rows (`amount: "0"`) in `Snapshot.liabilities`, while `residual(boundary: &Snapshot)` filters `.filter(|row| row.amount.0 > 0)` so `residual` lists every positive remaining duty (`auditDuty` amount `"5"` after `pay-beneficiary` is discharged from `"40"` to `"10"` and then `"0"`).
   - **Stage Admission, Cost, Decode & R191 Lexer (`ORIGINAL03`–`ORIGINAL23`)**: `stage::load` enforces `LANGUAGE_2 = "moriarty/2"`, `validate::validate_program` (`MAX_AWAITS_PER_ACTION = 16`, `MAX_EVENT_BINDERS = 16`, `MAX_CAPTURES = 256`, no binder shadowing, `Bool`/`UInt` event binders), and `cost::plan_program` (`PROFILE_2 = "moriarty-reference-logical-cost/2"`). `stage::decode` enforces `MAX_CONTINUATION_BYTES = 262_144`, `MAX_EVENT_BYTES = 16_384`, and `json::parse` (`MAX_DEPTH = 64`, duplicate-key rejection). `stage::admit::resume` checks `FORMAT_1`, `program_hash`, `plan_hash`, `cli_action`, `site` lookup, `logical.profile == PROFILE_2`, `logical.committed == segment.committed_before`, `cli_cap <= c.logical.cap`, `committed.checked_add(segment.cost)` against effective cap, canonical state/balance/liability ordering, capture bindings, and event bindings before running the segment. Retained `R191` lexer (`ORIGINAL18` `lexer/src/lib.rs`) enforces `MAX_SOURCE_BYTES = 65_536` and `MAX_TOKENS = 16_384`.

5. **Mandatory Security & Financial Scope Limits**:
   - `continuation.logical.cap` is outside `program_hash` and `plan_hash` (`ORIGINAL04` lines `88–112`, `ORIGINAL60` lines `132–140`) and remains an unauthenticated local advisory ceiling.
   - Stage labels `{"qualification": "local-reference-stage", "authentication": "none", "consumption": "untracked"}` (`ORIGINAL03` lines `22–26`) remain mandatory on every publication.
   - This increment establishes **no** continuation authentication, replay/consumption tracking, persistent checkpoint codec, universal machine financial lowering (`UIREQ-018`), local segment failure delta reporting (`UIREQ-019`), native/Halo2/ZKIRv3 proof correspondence, DUST/ledger fee accounting, or Midnight Preview/Fenrir settlement.

---

## 2. Normal Code / Consumer / Full Oracle / Independence Verdict

### Verdict: `APPROVE_SCOPED_POSITIVE_CONSUMER_AND_ORACLE_WITH_DISCLOSED_COORDINATION`

1. **External Positive Consumer (`ORIGINAL24` `crates/mori-core/tests/support/sherlock_financial_await_prefix_positive.rs`, `9,140B`, `SHAdef37ae71f22ea16b1614f228b476dfefe486edf897b8c7bcf0c6ccf24a08e18`)**:
   - Placed under `crates/mori-core/tests/support/` for explicit external compilation (`--test`), avoiding automatic Cargo test discovery (`ORIGINAL68`, `ORIGINAL69`).
   - Single test `fn real_core2_three_calls_preserve_complete_financial_await_publications()` (`ORIGINAL24` lines `161–215`) calls `stage::load(SOURCE)` once on `retained-payment.mori` (`821B`, `SHA14df54dae79da68a20d37f285b7de91628cf6d999db197f423f6689e490c13f0`) and chains **3 actual owning producer invocations** using each prior call's returned `Continuation`:
     1. `let start = projection::start_observed(&program, action.clone(), scenario.clone(), cap);`
     2. `let first_returned = waiting(&start).clone(); let suspended = projection::resume_observed(&program, first_returned.clone(), events[0].clone(), cli_action.clone(), cli_cap);`
     3. `let second_returned = waiting(&suspended).clone(); let complete = projection::resume_observed(&program, second_returned.clone(), events[1].clone(), cli_action.clone(), cli_cap);`
   - Also asserts `assert_eq!(waiting(&suspended), &first_returned, "false guard retains the exact prior continuation");` (`line 201`) and `assert_eq!(complete.observation().await_prefix(), AwaitPrefixObservation::Complete);` (`line 214`).
   - **Typed Borrow & Pointer Identity Joins (`compare_call`, `ORIGINAL24` lines `82–159`)**: Verifies `std::ptr::eq(view.program, program)`, `std::ptr::eq(view.result, produced.raw_result())`, and for `AwaitPrefixObservation::Waiting` checks `std::ptr::eq(action, raw_continuation.action.as_str())`, `std::ptr::eq(captures, raw_continuation.captures.as_slice())`, `std::ptr::eq(logical, &raw_continuation.logical)`, `std::ptr::eq(committed_boundary, &raw_continuation.boundary)`, `std::ptr::eq(residual, raw_residual.as_slice())`, and `same_segment(segment, raw_segment)` (`frame_pre`, `frame_post`, `execution.pre`, `execution.post`, `execution.steps`, `execution.effects`).
   - **Whole-Value JSON Oracle Comparisons**: `whole(&publication, &expected_publication, ...)` and `whole(&serde_json::to_value(prefix).expect(...), &expected_prefix, ...)` compare the entire `produced.publication()` and `view.await_prefix()` JSON values against the 6 standalone fixtures (`ORIGINAL28`–`ORIGINAL33`), plus `produced.raw_result()` against `expected_obs["result"]` and `view` against `expected_publication["financial_observation"]`. No keys are stripped, sorted, or self-generated.

2. **Full Independent Oracle Verification (`ORIGINAL28`–`ORIGINAL56`)**:
   - All 9 consumer fixture files (`ORIGINAL25`–`ORIGINAL33`) byte-match the sealed `EXPECTATION-FREEZE.json` (`ORIGINAL43`, `8,664B`, `SHA17256af0dd387b21180baefed55bab626668e2d0c303de3afd54e71f23aa75b7`, frozen at `2026-10-18T14:21:58Z` before any candidate execution) and reuse the byte-identical `R175`/`017` derivations (`ORIGINAL35` `RETENTION.json`, `ORIGINAL44`–`ORIGINAL56`).
   - Verified expected values across the 3 calls (`program_hash: "a3fc591f7da1b94d86094e6507c0d1c2e3af3c195d7a937eb8de181d6a63b723"`, `plan_hash: "b8d0c5e4deaded2e29295d0ba4248df1a7d5f5100418b7e1bcecaaa53e3d8a2d"`):
     - **`START` (`ORIGINAL28` `111,331B`, `ORIGINAL29` `22,406B`)**: `status: "waiting"`, `advanced: true`, `site: 1`, `captures: []`, `attempted: "7"`, `committed: "7"`; `reserve` `UNIT` `100` $\to$ `70` (Transfer `30` to `beneficiary`) $\to$ `68` (Fee `2` to `fee_recipient`); `pay-beneficiary` discharged by `30` (`40` $\to$ `10`); `residual`: `auditDuty` (`5` `AUX`) and `pay-beneficiary` (`10` `UNIT`); `accounting`: `reserve` `transfer_debit: {"Exact": "30"}`, `fee_debit: {"Exact": "2"}`, `gross_debit: {"Exact": "32"}`, `net_flow: {"Outflow": "32"}`; `await_prefix` matches `Waiting` with full `committed_boundary`, `residual`, and `published_segment`.
     - **`FALSE_AWAIT` (`ORIGINAL30` `26,231B`, `ORIGINAL31` `3,338B`)**: `status: "waiting"`, `advanced: false`, `segment: null`, `published_segment: null`, `attempted: "2"`, `committed: "7"`, unchanged `committed_boundary` (`reserve` `68`, `pay-beneficiary` `10`) and `residual`, `accounting: "NoPublishedSegment"`, `tentative_publication: "NoAdvancedSegment"`.
     - **`TRUE_RESUME` (`ORIGINAL32` `89,230B`, `ORIGINAL33` `11B` `"Complete"`)**: `status: "complete"`, `await_prefix: "Complete"`, `attempted: "8"`, `committed: "15"`; steps `3..6` (`Await` `true`, `Transfer` `10` `UNIT` `reserve` `68` $\to$ `58` / `beneficiary` `30` $\to$ `40`, `Fee` `1` `UNIT` `reserve` `58` $\to$ `57` / `fee_recipient` `2` $\to$ `3`, `Discharge` `10` `pay-beneficiary` `10` $\to$ `0`); `post.liabilities` retains `pay-beneficiary` at `"amount": "0"` while `residual` has only `auditDuty` (`"amount": "5"`); per-segment `accounting` reports `reserve` `transfer_debit: {"Exact": "10"}`, `fee_debit: {"Exact": "1"}`, `gross_debit: {"Exact": "11"}`, `net_flow: {"Outflow": "11"}`.

3. **Disclosed Coordination & Coverage Limits (`ORIGINAL34`, `ORIGINAL41`, `ORIGINAL75`)**:
   - The expectation author read the declaration-only `PUBLIC-API.md` (`ORIGINAL66`, `SHA1ef7871426b3b82326954207e9345a9ddea55589c29b17b27fbdc2c288ddaaaa`), which disclosed the `AwaitPrefixObservation` declaration and the known empty-captures fact (`captures: []` for `pay`). No candidate output or populated expectations were shared with the production author.
   - This single positive 3-call sequence does **not** exercise `RejectedNoReturnedCheckpoint`, nonempty `captures`, non-`None` `cli_action`/`cli_cap`, `AmountAggregate::AboveU128`, or different programs.

---

## 3. Closed `UIREQ-017` & All Affected Currentness Disposition

1. **Direct `UIREQ-017` Currentness & Source Compatibility (`ORIGINAL73` `CURRENTNESS-PROJECTION.json`)**:
   - In the baseline ledger (`current-evidence.preimage.json`, `810,901B`, `SHA77fc4050c622ed5335e17c24ccaf7ef7b83cf6e94064bc36b3ce8ba22025a585`), `crates/mori-core/src/sherlock/financial_projection.rs` appears at exactly two JSON pointers, both in closed row `UIREQ-017`:
     - `/requirements/16/implementation/sourceRefs/1` (`sha256: "54ebd690ce5748e970b579ab6eb43fe7a9353f190017675abec4924df6368a6f"`)
     - `/requirements/16/currentness/sourcePins/1` (`sha256: "54ebd690ce5748e970b579ab6eb43fe7a9353f190017675abec4924df6368a6f"`)
   - **Predicate & Semantic Disposition**: `UIREQ-017` (`uireq-017_original_clause_and_scenarios` / `First paid prefix`) remains **validly closed**. Every `UIREQ-017` struct/enum layout, `start_observed`/`resume_observed` ownership invariant, `project_invocation`/`account_effects` calculation, and the serialized `"financial_observation"` object and `STAGE_LABELS` in `ProducedInvocation::publication()` are preserved in `b2d5765d5a6bfcbd2139f4df7130bbddc85268e891ea63ccf60a2974040f65df`.
   - **Exact Compatibility Disposition of Existing `UIREQ-017` Consumers**:
     - `crates/mori-core/tests/sherlock_financial_observation_positive.rs` (`ORIGINAL57`, `12,064B`, `SHA03a8d211cd4e982039f51bbe418bb434746f1ff0017f8558283b048ec33689c6`): destructures `InvocationObservation` (`lines 129–130`) and does not call `ProducedInvocation::publication()`. Because `InvocationObservation`'s fields are unchanged and `await_prefix(&self)` is a method, `ORIGINAL57` is **source- and value-compatible** with `b2d5765d...`.
     - `crates/mori-core/tests/sherlock_financial_producer_positive.rs` (`ORIGINAL58`, `7,846B`, `SHAcc746c09c6f627a66e2c70c1951ede71b88b7681754392dead9c9bd7c0b456f2`): compiles against `b2d5765d...` and its `serde_json::to_value(&view)` check (`line 72`) remains value-compatible, **but** its strict whole-object assertion `assert_eq!(produced.publication().unwrap(), report, ...)` (`lines 76–96`) constructs `report` without the new top-level `"await_prefix"` key. Thus `ORIGINAL58` is **historical `54ebd690...` wire-schema evidence** and would fail at `line 95` if rerun unmodified against `b2d5765d...`.
     - `crates/mori/src/commands/simulate.rs` (`ORIGINAL59`, `9,702B`, `SHAf02804f8496dd06d6c689583de4e24859e680973d4e98627bc417d7b1915c27b`): `print_produced_stage_result` (`lines 155–161`) calls `produced.publication()` and `print_stage_report(report)`, remaining **source-compatible** and emitting the additive `"await_prefix"` field on `/2` CLI start/resume runs (remains source-audited, not subprocess-executed).
     - `docs/tutorials/sherlock/core-continuations.md` (`ORIGINAL60`, `10,562B`) and `docs/reference/sherlock/financial-observation.md` (`ORIGINAL61`, `10,302B`): remain accurate for their documented `54ebd690...` cut; if Root adopts `b2d5765d...`, `financial-observation.md` requires an additive documentation update describing `AwaitPrefixObservation`, `InvocationObservation::await_prefix`, and the `"await_prefix"` key in `publication()`.
   - **Rule Against Blind Hash Refresh**: Historical `UIREQ-017` runs (`R176`, `R189`, and `sherlock-owning-financial-producer-20261018/actual/ordinary-four`) were executed on `54ebd690...` and **not** rerun on `b2d5765d...`. Root must **not** blindly replace `54ebd690...` with `b2d5765d...` in `UIREQ-017` without: (1) actual passing execution of the 3 new operations below, (2) independent post-result review, and (3) explicit currentness qualification recording that `ORIGINAL58` (`cc746c09...`) is preserved historical evidence at preimage `54ebd690...` while `ORIGINAL24` (`def37ae7...`) is the whole-`publication()` consumer for `b2d5765d...`.

2. **Other 13 Closed Rows & Open `UIREQ-018` / `UIREQ-019` (`ORIGINAL63`, `ORIGINAL67`, `ORIGINAL73`)**:
   - The other 13 closed rows (`UIREQ-001`, `UIREQ-002`, `UIREQ-003`, `UIREQ-005`, `UIREQ-006`, `UIREQ-008`, `UIREQ-009`, `UIREQ-010`, `UIREQ-011`, `UIREQ-021`, `UIREQ-022`, `UIREQ-025`, `UIREQ-027`, `UIREQ-030`) have zero source references to `financial_projection.rs` and are **semantically unaffected** (touching `mori_core` only if their shared crate `libmori_core.rlib` is recompiled).
   - `UIREQ-018` (`ORIGINAL67` `UIREQ018-GAP-MAP.json`) and `UIREQ-019` remain **OPEN**; this Core/2 projection does not implement universal machine financial lowering, Commit/Yield/Await opcodes, persistent checkpoints, or A/B/Core2 correspondence.

---

## 4. Operation Decision for the Exact 3 Ordinary Operations

### Verdict: `ELIGIBLE`
*(Authorized for Root-only execution of the exact 3 serial operations in `ORIGINAL70` `PROPOSED.json`, `11,424B`, `SHA8f279df5fadbd43b1eb45a3d83d5c313aaa0b3f4581fc8bd804d6b062cdfc2d8`, subject to the mandatory conditions below.)*

### Approved 3-Operation Sequence (`ORIGINAL70`)

1. **`01-compileNormalCoreRlib`**:
   - Compiles `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/financial-await-projection-ordinary-QA-20261018/snapshot/crates/mori-core/src/lib.rs` with `/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc` (`--edition=2024`, `--crate-name mori_core`, `--crate-type=rlib`, no `--test` / `cfg(test)`, no Cargo/build scripts) under `timeout 30s` and `prlimit --cpu=25:25 --as=2147483648:2147483648 --fsize=67108864:67108864 --core=0:0`, emitting `build/libmori_core.rlib` and `build/core-rlib.d`.
2. **`02-compileFinancialAwaitConsumer`**:
   - Compiles `snapshot/crates/mori-core/tests/support/sherlock_financial_await_prefix_positive.rs` with `--crate-name financial_await_prefix_positive --test` linked against the freshly produced `build/libmori_core.rlib` and pinned `libserde_json-76dc87088753b61e.rlib` under `timeout 30s` and `prlimit --cpu=25:25 --as=2147483648:2147483648 --fsize=67108864:67108864 --core=0:0`, emitting `build/financial-await-prefix-positive` and `build/financial-await-prefix-positive.d`.
3. **`03-runExactFinancialAwaitPrefixPositive`**:
   - Runs `build/financial-await-prefix-positive --exact real_core2_three_calls_preserve_complete_financial_await_publications --test-threads=1 --nocapture` under `timeout 10s` and `prlimit --cpu=8:8 --as=1073741824:1073741824 --fsize=16777216:16777216 --core=0:0`.
   - *(Note on `fsize` & pin-count text in `PROPOSED.json`: `03-runExactFinancialAwaitPrefixPositive` specifies `--fsize=16777216:16777216` [16 MiB] matching `finite_classes` `"external normal10s CPU8 AS1GiB file16MiB core0; ... Exact argv is authoritative"` in `ORIGINAL70` lines `143` and `197`, while writing zero runtime output files; and `FLAT-INPUT-PINS.json` `SHAdee8d4e752c8330a1a8da9e5e10d162fad34fea956987a4d1aff8ec149115ce6` contains `490` static pins [`157` Core + `10` consumer/fixtures + `323` tools/deps] per `ORIGINAL71`, `ORIGINAL72`, and `ORIGINAL74`, whereas `PROPOSED.json` line `194` prose says `all484static flat inputs`. The exact `argv` and all `490` pins in `FLAT-INPUT-PINS.json` govern.)*

### Mandatory Execution & Capture Conditions

1. **Root Sole Executor & Serial Fail-Stop**: Root executes the 3 commands serially in `cwd: "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/financial-await-projection-ordinary-QA-20261018"`, with exact 3 environment variables (`PATH="/usr/bin:/bin"`, `LANG="C.UTF-8"`, `TMPDIR=".../tmp"`, no inherited environment), `keptBytesPerStream: 131072`, 1 attempt only, fail-stop on first non-zero exit, no auto-retry, and no source mutation.
2. **Pre/Post Pin Verification (`490` Static Pins) & Generated Output Chaining**:
   - Verify all `490` static input pins in `FLAT-INPUT-PINS.json` (`SHAdee8d4e752c8330a1a8da9e5e10d162fad34fea956987a4d1aff8ec149115ce6`) before `01-compileNormalCoreRlib` and after `03-runExactFinancialAwaitPrefixPositive`.
   - Record the actual SHA-256 and byte size of `build/libmori_core.rlib` and `build/core-rlib.d` immediately after `01-compileNormalCoreRlib` succeeds and bind that exact `libmori_core.rlib` hash as the input to `02-compileFinancialAwaitConsumer`.
   - Record the actual SHA-256 and byte size of `build/financial-await-prefix-positive` and `build/financial-await-prefix-positive.d` immediately after `02-compileFinancialAwaitConsumer` succeeds and execute that exact binary hash in `03-runExactFinancialAwaitPrefixPositive`.
   - Inspect `.d` dep-info files for source-membership consistency only (never as exclusive host I/O or linker/SDK attestation).
3. **Strict Test Pass Criterion**: `03-runExactFinancialAwaitPrefixPositive` must exit `0` with `selected: 1`, `passed: 1`, `failed: 0`, `ignored: 0` for `real_core2_three_calls_preserve_complete_financial_await_publications` (`0` tests selected with exit `0` is a failure; `0` old `UIREQ-017` test cohorts are run or claimed rerun).

