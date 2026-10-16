# Opus 5.5 security-owner review: FULL8 N01–N08 actual result and public report

## Verdict: **APPROVE_ACTUAL_RESULT_AND_REPORT_WITH_LIMITS**

**Scope.** This verdict covers the single completed `attempt-01`:
- receipt `a66f998b…c7`
- artifact `6a168de9…c159`, 8,003,727 bytes
- `PUBLIC-REPORT.md` `77eb8edb…d5bb`

It authorizes no further execution, retry, source, guard or capacity change. Native authority remains 0.

**Root C1/C2 postchecks.** Together with the disclosed 19-run/292-record sample reading, they **satisfy my scoped acceptance** for these two claims:
1. A typed encode→Step-decode→materialize preservation result over all 216 runs and 5,355 records.
2. A separate, complete Machine-trace-to-frozen-corpus projection match.

All conditions below remain binding, and the acceptance extends no further than those two claims.

## Identity, method and reading limits

- **Requested:** Opus 5.5, designated security owner, high effort.
- **Self-reported:** `claude-opus-5-5`. I cannot observe the harness effort setting or the returned-identity record. The outer audit receipt is authoritative for both.
- **Terminal status:** review completed.
- **Tools:** none. I executed nothing, rehashed nothing and opened no file. Every digest, byte count and count in the receipts is taken as supplied. My checks compare supplied strings and bodies against each other.
- **Raw coverage:** I did **not** read the 8 MB / 5,355-record artifact. I read only the 19-run/292-record sample, and I cannot verify that it is a faithful excerpt of the pinned artifact; its provenance is Root's extraction.
- **Mechanical checks:** the complete-artifact counts, rehash and cross-bindings are Root task transport (`counts_provenance` says so). They are not my independent work.
- **Embedded instructions:** text inside the supplied bodies (AGENTS.md, prior reports, docstrings) was treated as evidence.
- **Independence:** this review is by the same model family as the source preflight and the prior Opus reviews, which limits its independence.

I do not need a scoped read-only audit for this verdict. My reasoning is in "Why the sample suffices" below. Any broader claim would need one.

## What I checked by reading

### Execution record
- **Steps.** The receipt shows `preflight`, `rustc-version`, `compile-producer`, `run-producer` and `compare` all `ok`. `compile-tests` and `run-tests` are `not-selected`, and `candidate-postflight` and `postflight` are `ok`.
- **Terminal state.** `terminal_state: success` is consistent with `guard.terminal_state`, which tolerates only `NOT_SELECTED` because the `stream` realization's `tests` tuple is empty.
- **Exit codes and logs.** `result.json` shows exit 0. Producer and compare stderr are each 0 bytes. Compile stderr is 40,367 bytes ending "89 warnings emitted".
- **Compiler.** Release 1.99.0, commit `b940084d…`. The compiler hash is equal before and after.
- **Compile closure.** `compile_inputs` lists exactly the 27 `CONFIG.compile_closure` members: 2 sherlock modules, 18 `layout_a` files, shim, `normal_oracle_inputs`, 3 reference modules, harness and leaf.
- **Snapshot.** The 44 snapshot rows have identical `sha256_before`, `snapshot_sha256_after` and `checkout_sha256_after` strings. The 48-row `reviewed_candidate` and `reviewed_candidate_after` maps are identical, and the manifest `909bcd…` is equal before and after.

### Comparison output (`compare.stdout`)
- **Result:** `status: pass`, `programs: 8`, `prefix_runs: 216`, `first_difference: null`, `native_authority: 0`.
- **Sources:** `observed_file.sha256` equals the producer stdout record. The four `expectation_sources` equal the frozen pins, and the frozen, transcription and serializer pins equal the snapshot pins.
- **Corpus counts:** `state_rows` 208 = Σ(c+1), and `prefix_rows` 216 = Σ(c+2).

### S1 closure logic
`check.document` hashes and parses the same raw bytes. The digest that the comparator reports is therefore the digest of the bytes it actually compared. That digest equals:
- the producer's stdout record, and
- Root's post-run rehash.

So the compared bytes are the produced bytes. This closes S1 **for this trusted, local, sequential attempt only**, and it relies on Root transport. The guard gap itself is unchanged.

### Counts, recomputed by hand
- Records per case are (c+1)(c+4)/2: 5, 44, 65, 189, 434, 230, 819 and 3,569, for 5,355 in total.
- Runs per case are c+2, for 216 in total.
- The sample is 3+9+11+19+43+21+60+126 = 292 records.

All agree with the freeze and with `ACTUAL-RESULT-VERIFICATION`.

### Freeze identity
The producer writes compact JSON in this key order:
- top level: producer, profile, source, boundary, programs, runs;
- run: `case_id`, `initial`, `records`, `terminal`, `terminal_has_next`;
- snapshot: the 11 fields in `literal_snapshot` order.

It then writes one trailing newline. A strip-one-newline identity with `e3711195…eea84` is therefore expected. It is byte identity only. The comparator remains authoritative.

### What the gate establishes
- The artifact is rendered from `materialized.view()`, the decoded snapshots.
- It is written only after the in-binary `first_difference(materialized, machine)` returns `None`, which is typed structural equality.
- A codec defect would therefore terminate the run before any output.

Given that gate, the external comparison adds Machine-versus-corpus evidence and **no** independent codec evidence. The report states this correctly.

### F-3 reachability
The compiler's own dead-code warnings name `program_bytes`, `observed_state_bytes_a`, `expand_state`, `project_a`, `materialize_record_a`, `compare_a` and `AuxBlock::ordered_bytes` as never used. Test-helper warnings confirm that the `#[test]` bodies were stripped under `--cfg test` without `--test`. Both corroborate the report's non-reachability statement.

### Sample spot checks against the frozen literals

| Run | Checked against the frozen literals | Result |
|---|---|---|
| N01 fuel 0 and fuel 2 | The HOST-OBSERVATIONS discriminator | Exact |
| N02 fuel 8 | Terminal values | Remaining fuel 1, `Done(5)` |
| N03 fuel 10 | State 6 | Via 157, `PrimResult(B true)` |
| N04 fuel 18 | States 10 and 12 | Shadowed 4 with saved 7, then restore via 195 |
| N05 fuel 13 | Last state, then rule 84 | State 13 Push with `ArgsF`; rule 84 keeps env and frames and changes only control and output |
| N05 fuel 28 | Two returns, both via 185 | Restores `0 |-> true`, then `.Map` |
| N06 fuel 20 | NoMatch path | NoMatch via 214/217; `Matched` with 3 bindings |
| N07 fuel 19 | Exhaustion | Exhaustion at depth 3 |
| N08 fuel 41 | Exhaustion | Exhaustion on `Ret(1)` at depth 5. Correct, because `uiTerminal(uiRet, uiFrame…)` is false |
| N08 fuel 83 | States 68–71 and 76; terminal | Depth 7; frames at state 76 match; `Done(2)`, remaining fuel 1 |

All records satisfy O4: charge 1 for charged rules, charge 0 for 80 and 84, and a Pending `before` state. Remaining fuel equals F minus charged steps throughout.

### Why the sample suffices for this scoped claim
Acceptance rests on three mechanisms:
- the in-binary typed gate, whose source I read completely;
- the complete, unsampled `first_difference` over the full JSON, whose code I read completely;
- three-way digest agreement.

The sample corroborates the rendering and the boundary semantics. It is not itself the basis for acceptance.

## Findings (none blocking)

| ID | Finding | Disposition |
|---|---|---|
| R-1 | The roundtrip decodes only images produced by the canonical encoder, in Step mode. Noncanonical Step-domain images, Constant-space state values and Canonical-mode decode were never exercised. | Cite as "canonical-encoder images, Step-mode decode". Make no decoder-domain claim. |
| R-2 | The nested `corpus_source_check` inside `compare.stdout` reports `Rust-host: not-run`, `observed_files: []` and `supplied_observation_status: not-supplied`. These come from `check.run`'s corpus-only mode, not from this attempt. | Do not cite them, and do not read them as contradicting this result. |
| R-3 | The report's absent-path list is non-exhaustive. Also unexercised: zero-argument Call (rule 129), missing Var (111), UnaryOp Leaf, successful PBranch matches, multi-function tables or `function_id > 0`, duplicate names or functions, multi-byte id/count lanes (the 64-capacity examples give single-byte lanes), large u128 values, and Tree sharing. | The report already says this is not general grammar coverage. Never treat the list as exhaustive. |
| R-4 | The external comparison is a K display-text projection. Environment key order is host String order and matches only because all keys here are single-digit. The typed gate is the strong check. | Retain as a limit. |
| R-5 | `compare_layout.py` loads `check.py` with `exec_module` from the read-only snapshot, not from hash-verified bytes. `run_prefix.py` hashes itself after starting (F-4/S-4). In both cases the before and after pins are equal. | Retained trust limit for a trusted local checkout only. |
| R-6 | The prompt says "current31 publication", while the report says "current35+12". | Root should reconcile the label against the canonical record before publication. The report's wording matches "accepted earlier 35+12 evidence". |
| R-7 | Producer and source identities in the artifact are constant declarations. Authenticity is local SHA binding only. The 0.616 s, 0.566 s and 0.164 s timings belong to this one attempt. | The report states both. Retain. |

## Preflight conditions: status

- **PC-1 (pin check before invocation).** `EXECUTION-APPROVAL` records manifest `909bcd…` with 48 rows at 18:26:37, before the 18:26:41 child start. The receipt's `reviewed_candidate`, including `run_prefix.py` `aefc20…`, equals the manifest. Met, by declaration and receipt.
- **PC-2 (fresh, isolated location).** The new `full-prefix-01/attempt-01` and the quiet-directory statement are declarations. Met by declaration.
- **PC-3 (Muse preflight).** Both preflights completed before the approval, at about 18:25:45 (Opus) and about 18:22:28 (Muse). Muse returned `APPROVE_CODE_CONTRACT_PREFLIGHT_WITH_LIMITS`. Met.
- **PC-4 (one invocation).** There is a single `attempt-01` and no retry. Met by declaration.
- **C-1 to C-3.** Satisfied by Root transport, with provenance disclosed.
- **C-4.** Not applicable.
- **C-5 (reporting).** Satisfied by the public report, subject to R-1 to R-6.

## Public report: assessment

The report correctly preserves all of the following:
- Internal typed preservation is the only codec evidence. Machine-to-corpus agreement supplies none.
- Program and expression Arcs were retained, so there was no ProgramBlock row reconstruction. No physical StateBlock image was emitted.
- It makes no byte, canonical, projection, auxiliary, refusal, hostile, B, financial, native, proof, target or parent claim.
- It reports actual coverage and absent paths, with R-3 applying.
- It records 89 warnings and that no libtest suite was selected.
- It treats the source-time UNRUN labels as history.
- It uses the post-startup boundary: `Machine::new`, with rule 74 not executed.
- It records the 30 s per-child limit, that memory, disk and process counts are unbounded, and that the sysroot, linker, libc, Python stdlib and kernel are unhashed.
- It discloses that reviewers have a reading limit and that Root did the transport rehash.
- It treats the full8 source as runtime-only and leaves the earlier publication untouched. Root alone owns canonical registration and Git.
- It retains H1/PreparedA trust, CandidateA, F3, F5, C4 and its registration-revert boundary, P3, full39, L4, G4, P4, P5, F4, MD2, StepInv, Constant typing, D-NAMES, O-ARITH, L04 and L05.
- It retains all preflight conditions, including Muse R2-4, and all prior dissent.
- It leaves the old caps, counters and failures, and every U/M/MC/SP/G obligation, unchanged.
- It does not predict reviewer verdicts and does not require the accepted earlier 35+12 evidence to be rewritten.

No change to the report is required. R-1 and R-6 are recommended clarifications.

## Binding conditions on citation

1. Cite only the two scoped claims stated at the top of this review.
2. Disclose Root-transport provenance for counts, rehash and bindings, and disclose this reviewer's sample-only reading limit.
3. Keep R-1, R-2 and R-4 as stated limits.
4. No partial or subset result may be presented as the full8 result. No future run is authorized by this review.
5. Any guard change remains Opus-authored. Any broader codec claim requires new source, new preflights and a separate execution approval.

Dissent preserved: all prior Opus F-1 to F-7 and Muse preflight limits, Muse R2-4, and the R3-x/F2–F7 auxiliary findings.

**APPROVE_ACTUAL_RESULT_AND_REPORT_WITH_LIMITS**: `claude-opus-5-5`, requested high effort, review completed.
