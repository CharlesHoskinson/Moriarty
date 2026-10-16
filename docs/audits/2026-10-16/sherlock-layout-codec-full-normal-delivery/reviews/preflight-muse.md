APPROVE_CODE_CONTRACT_PREFLIGHT_WITH_LIMITS

Requested identity: fresh independent MuseWonder1.0, high effort, nonsecurity code/contract auditor.
Returned identity: Muse Code powered by muse-wonder-1.0, high effort, nonsecurity review only.

I used no tools and executed nothing.
I recomputed no hashes and claim no independent hash authenticity.
All security work and judgments remain Opus-owned; this review supplies no substitute security judgment.
Native authority remains 0.

## Substantive basis

No static blocker was found by reading the complete supplied candidate.

- Extension is selection-only. [the full8 leaf](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/full-prefix-01/candidate/models/sherlock/reference/layout_observations.rs) extends `INPUTS` to N01–N08 with completion fuels 1, 7, 9, 17, 27, 19, 38, 82 and stack limit 8. The fuel ranges `0..=completion+1` give 3+9+11+19+29+21+40+84 = 216 runs, matching [CONFIG.json](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/full-prefix-01/CONFIG.json).
- Example capacities remain exact and unchanged: `Capacities { tables: [64; 15], data: 64 }` and `StateCapacities { data: 64, data_fields: 64, env_cells: 32, value_slots: 32, frame_slots: 16, control_rows: 16, stack_limit: 32 }`. These remain examples, not defaults, admission bounds, or F3 envelope certification.
- No all8-fit assumption is made. N03–N08 may still refuse during preparation, canonical encoding, or Step-mode materialization. Any such refusal is the terminal result for the single attempt, even after earlier runs have written partial output. No retry, widening, truncation, sampling, or substitution is approved.
- The internal gate remains the real typed path: `Machine` traces, `prepare_a`, `encode_observed_a`, Step-mode `materialize_trace_a`, and the existing `first_difference` over the complete materialized and Machine traces. A materialization difference returns `CaptureFailure::MaterializationDifference`.
- [the external comparator](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/full-prefix-01/candidate/compare_layout.py) retains both frozen families. It runs the existing `check.run`, calls `validate_pair` for `normal` and `extra-normal`, derives the complete expectation through the unchanged `expected_observations`, and calls `compare_observations` with `first_difference` over both complete documents. It does not select a subset after expansion and supplies no expected row from the producer, evaluator, or codec.
- [EXPECTED-FREEZE.json](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/full-prefix-01/EXPECTED-FREEZE.json) consistently records the pre-extension freeze: 8 programs, 216 runs, 5,355 ordered records, compact JSON SHA-256 `e3711195…eea84`, and 8,003,726 bytes. That digest was treated only as a freeze record, not as semantic inspection of the derived 8 MB projection.
- [run_prefix.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/full-prefix-01/run_prefix.py) is limited to one fresh `attempt-01`, the fixed rustc path, the existing `Attempt` methods, direct argv, minimal environment, process groups, and 30 seconds per child. Its sequence is preflight, version, compile-producer, run-producer, plain-child compare, candidate postflight, and finish/postflight. No new security or resource guard is introduced by this nonsecurity review.
- The declared 27-member compile closure and 44-member snapshot are internally consistent with the supplied module graph, including the `#[cfg(test)]` producer registration and the leaf path. Compilation, producer execution, and comparison remain UNRUN. The prior baseline 35 tests and prior N01/N02 49-record result do not transfer to these new full8 bytes.

## Conditions and limits

1. Only the planned single invocation may proceed: `python3 -I -B run_prefix.py` from the stated fresh runtime directory. No second attempt is approved by this preflight.
2. The S1 plain-child artifact-binding gap remains. After the run, Root must recompute the artifact SHA-256 and match it against the producer receipt and `compare.stdout`, together with snapshot/transcription pins, all8/216/5,355 counts, and the exact 27-member closure. Any reading or rehash limit must be reported honestly.
3. The S2 limit remains. The internal typed Step-mode roundtrip is the only prefix-codec evidence. Machine-corpus agreement supplies no independent physical-codec evidence, and retained `Program`/expression `Arc`s are not row reconstruction.
4. Artifact raw-coverage limits must be disclosed in later reviews. Full retained artifact/source hashes and mechanical all-record comparison are required; no subset result may be presented as the full8 result.
5. No claim is made for canonical uniqueness, byte expansion, parser or wire format, general transition or continuity, rule validity, hostile or refusal coverage, B-profile work, financial behavior, proof, native/target acceptance, or parent acceptance.
6. H1, CandidateA, F3, F5, C4, P3, full39, L4, G4, P4, P5, F4, MD2, StepInv, Constant typing, D-NAMES, O-ARITH, L04, L05, prior dissent, old caps/counters/failures, and the full U/M/MC/SP/G roadmap remain unchanged.
7. If any security-guard change is needed, Opus alone must author it. Root alone owns canonical registration and Git.

This preflight does not predict a pass.
