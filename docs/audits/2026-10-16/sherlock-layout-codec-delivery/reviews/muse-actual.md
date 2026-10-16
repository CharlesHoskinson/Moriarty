**Terminal verdict: APPROVE_ACTUAL_RESULT_AND_REPORT_WITH_LIMITS**

Requested identity: fresh independent `muse-wonder-1.0`, high effort, nonsecurity code/contract/result auditor.  
Returned identity: Muse Code powered by `muse-wonder-1.0`, high effort, nonsecurity review only. Native authority remains 0. No security judgment is supplied; security work and judgments remain Opus-owned.

I used no tools and executed nothing. I independently recomputed no SHA-256, ran no Rust or Python, and claim no independent hash authenticity. The findings below are by reading the complete supplied bodies and comparing recorded hash strings, names, counts, steps, and outputs textually.

## Baseline attempt

The baseline actual result is internally consistent:

- The command was one `python3 -I -B run_layout.py` invocation, with receipt SHA-256 record `441060b670ccf2d5b58c3f61f08df907ae52140c58a2fe8d06a239318a057b22`, manifest `a164cdaec0ca4931cc1bc9bb34757afc4f35307ba1f74af482f84cfbe9d57d2f`, and `result.json` reporting `terminal_state: success`.
- The receipt records exactly six steps, all `ok`: `preflight`, `rustc-version`, `compile-tests`, `run-tests`, `candidate-postflight`, and `postflight`.
- The `compile_inputs` list contains the exact 24-member closure declared in `CONFIG.json`.
- `run-tests.stdout` states `running 35 tests` and `35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`. The 35 listed names match the fixed names in `CONFIG.json` and in `ACTUAL-RESULT-VERIFICATION.json`.
- The receipt records matching before, snapshot-after, and checkout-after hash strings for the snapshot members, and matching reviewed-manifest and reviewed-candidate after-values. This supports the claim that baseline source, test, and capacity bytes were unchanged during that attempt.
- Compilation emitted 22 retained warnings. They are warnings only; the public report does not convert them into a clean-warning claim.

## Prefix attempt

The prefix actual result is also internally consistent:

- The command was one `python3 -I -B run_prefix.py` invocation, with manifest `6975756204ac5d2c62dac73853d4319c554cc911a4f0e3dde2b55ef4c097df36`, receipt `d7ea086a820070ee0af802983a5770a357416512c4c2efc576f7d86e1f8bdda9`, and `terminal_state: success`.
- Executed steps were `ok`; `compile-tests` and `run-tests` were explicitly `not-selected`, not passes.
- The compile-producer `compile_inputs` list contains the exact 27 members declared in prefix `CONFIG.json`.
- The observation artifact hash string `a71422bcc58dd574c4fb433198a2e631fdb0682184102db20edb622d56ac5d25` appears consistently in the run-producer receipt record, `compare.stdout` as `observed_file.sha256`, and prefix `ACTUAL-RESULT-VERIFICATION.json`.
- `compare.stdout` reports `status: pass`, `programs: 2`, `prefix_runs: 12`, `first_difference: null`, and scope `complete-corpus-projection-match-only; not independent physical codec evidence`.
- The retained observation artifact and frozen `EXPECTED.json` both represent 2 programs, 12 N01/N02 fuel-prefix runs, and 49 ordered records. The Root verification record states parsed equality with the frozen expectation; I did not independently parse or rerun that comparison.
- By reading the leaf source, it uses actual `Machine` traces, `prepare_a`, `encode_observed_a`, Step-mode `materialize_trace_a`, and the existing `first_difference`. The external comparator derives expectations from the existing frozen corpus projection and selects N01/N02 after literal expansion. Expected states are not supplied by the producer.
- The leaf dimensions are the exact prior state-encode fixture values: program tables `[64; 15]`, data `64`, and state capacities `64/64/32/32/16/16/32`. They are examples only, not defaults or admission bounds.
- Prefix compilation emitted 89 retained warnings in `compile-producer.stderr`; the result does not depend on treating those as absent.

## Corrected G2 binding

The original Opus prefix G2 text was erroneous: it claimed 23 baseline-shared compile-closure members and wrongly treated `normal_oracle_inputs.rs` and `extra_oracle_programs.rs` as baseline-shared.

The separate owner addendum corrects that history without erasing it. The binding partition is:

- 21 byte-identical baseline-shared members;
- 3 same-path changed members: `layout_harness.rs`, `layout_harness_shim.rs`, and `layout_a/mod.rs`;
- 3 prefix-only members: `normal_oracle_inputs.rs`, `extra_oracle_programs.rs`, and `layout_observations.rs`.

That sums to the exact 27-member prefix closure. The corrected G2 supersedes the original G2 for gating only; the original error and corrective history remain part of the record.

## Public report assessment

The proposed public report accurately separates the three distinct facts:

1. the baseline 35 normal tests, including limited literal byte and decoder tests;
2. internal complete typed Step-mode encode/decode/materialize preservation for the 12 Machine traces;
3. independent corpus-projection agreement for those same Machine traces.

It correctly states that the corpus-projection agreement is not independent physical or canonical codec validation. It also keeps the required limits explicit: H1, CandidateA/F3/F5, C4, P3/full39/L4 conditions, Muse R2-4 dissent, the width-only zero-dimension auxiliary limitation, missing production integration, unhashed runtime components, retained warnings, and the unchanged roadmap obligations. It states that `PreparedA` retains Program and expression source Arcs and that no ProgramBlock reconstruction, full canonicality, general continuity/rule validity, or hostile/refusal evidence follows.

Header `UNRUN` labels are source-time history. The separate receipt and result records describe the later single invocations; the report does not treat the headers as post-run facts.

## Blockers, dissent, and limits

No nonsecurity blocker or report inaccuracy requiring changes was found by reading.

No dissent from this reviewer. Prior retained dissents and limits, including Muse R2-4, remain preserved and are not overridden.

Mandatory-condition limitation stated explicitly: prefix C4 and the baseline Opus success conditions include hash-binding checks. The supplied Root transport records state that Root recomputed the observation SHA-256, matched it to the producer receipt and `compare.stdout`, checked the frozen parsed `EXPECTED.json`, checked frozen/source/transcription pins, and checked the exact closures. I inspected those retained records and their textual cross-bindings, but because this review had tools disabled, I did not personally rehash files or rerun any step. Therefore this approval relies on Root’s recorded recomputation for cryptographic hash authenticity; it does not silently waive that condition and does not claim independent reviewer rehashing.

Production module registration is not integrated or certified by either attempt. No future run is admitted by this review, and no code, source, guard, capacity, or result modification is authorized.
