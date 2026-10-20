Top finding: No mandatory defect was found in the bounded five-mode production relation or in the corrected 22-test source. Approval is strictly bounded to uncompiled, UNRUN source. It is not a complete-leaf, runtime, synthesis, proof, native, degree, or whole-goal approval.

I am Muse Wonder 1.0 HIGH, a fresh independent non-author reviewer under the stated Opus-outage fallback. I did not use hashes as approval evidence. The author is not a vote. The two supplied C2 opinions approve only emitter plus consumer production source; they do not approve these tests.

## Verdicts

- **DESIGN: AGREE / APPROVE**, bounded to `(First, Not)` and `(Second, Add|Sub|Mul|Lt)`.
- **SECURITY_RELATION: AGREE / APPROVE**, bounded to the selected-step relation under the retained premises below.
- **SOURCE_API: AGREE / APPROVE**, bounded to static source/API review of the unchanged production files and parent registration. This is not a Rust typecheck.
- **TEST_SOURCE: AGREE / APPROVE**, bounded to the delivered 10 positive plus 12 raw test sources as a genuine-owner/raw-preprocessing test deliverable. All 22 remain UNRUN and uncompiled.
- **COVERAGE_SCOPE: CHANGES_REQUIRED for a complete-coverage claim**. The delivered coverage is substantial but expressly incomplete in the required-open items below.
- **DELIVERY_SCOPE: CHANGES_REQUIRED for any complete ScalarArithmetic-leaf claim**. Separate eligible compilation and execution, pairing isolation, remaining coverage, and all full-goal gates remain open.
- **OVERALL: AGREE / APPROVE for bounded source adoption only**.

## Bounded read coverage

The decisive source reviewed comprised the complete emitter, consumer, positive tests, raw tests, and parent registration, plus the called Core `plan_scalar`/`pure_leaves`, selector, owner runtime, SI21/SI22, observation, physical codecs, checked reader, canonical META/PROG/CTX/DATA/helper blocks, Push family parameters, SDK IR/`IrSource::load`/`preprocess`, `ProofPreimage::check`, `transient_commit`, field representation, and the called Add/Mul/Neg/Not/TestEq/LessThan/CondSelect/DivMod bodies.

No actually called production definition needed for the bounded judgments was replaced by a digest. The disclosed uncalled `cfg(test)`, native-key/prove/example, and unrelated backend-lowering exclusions do not discharge synthesis or native behavior.

Genuinely missing or unestablished premises are:

- SI08/SI21 entry and top-frame coherence, including actual ScalarEntry installation.
- Full SI17 marker placement, SI21, SI22, and StepInv.
- Program membership, P1 native issuance, and SourceSpec authentication.
- Compilation, SDK preprocessing execution, synthesis, MockProver, proof, degree/`k`, performance, native, ledger, and stagenet results.
- SDK/native C13 remains UNSELECTED. Native23 remains `c=null`, `S NOT_ESTABLISHED`, stopped at Am10, with caps untouched.

## Design findings

The emitter selects exactly `(First, Not)` and `(Second, Add|Sub|Mul|Lt)`. Mode bits are derived from the helper mode lane, `is_num` is the sum of Add/Sub/Mul/Lt bits, and the gated predicate is effectively:

```text
enable ⇒
  ((phase == First && mode == Not) ||
   (phase == Second && mode ∈ {Add, Sub, Mul, Lt}))
```

Applied post phase is `Done`; the corresponding Core cursors are First/0, Second/1, and Done/2. P3 preserves the actual First/Second phase instead of advancing the helper.

`Not` reads operand A. Bool stages `BoolValue{!b}`; every other kind stages `PrimError{11}`. Numeric modes read operand B. A non-UInt operand B stages error 11. `Lt` stages `BoolValue{reg_a < n}`. Add/Sub/Mul stage the checked `u128` result or error 10. This matches the supplied Core arithmetic fragment and the 19 author-visible branch examples. Those examples are not an independent admitted-state oracle.

For Second numeric modes, the circuit additionally enforces:

```text
reader_a.is_uint == 1
pre.reg_a == reader_a.uint_value
```

under `enable && numeric`. For First/`Not`, it enforces `reg_a == 0`. This is the valid SI22 numeric domain and is stronger than bare `plan_scalar`, which trusts an installed `reg_a`. It does not prove predecessor Tag behavior or full SI22.

The 64-bit multiplication retains the complete product structure. With limbs below `2^64`, all partial products and sums remain far below the BLS12-381 scalar modulus. Overflow is exactly `x1*y1 + cross_hi != 0`, before projecting the low 128 bits. The supplied cases `2^64 * 2^64` and `MAX * MAX` therefore stage error 10, while `MAX * 1` and `MAX * 0` succeed. Addition uses the exact sum below `2^129`; subtraction reconstructs `x - y + borrow*2^128` before the 128-bit split.

P3 is constrained against the separately retained admission bound:

```text
is_p3 = (pre_image[L_HELPER_WORK] == source_params.admission_bounds().max_helper_work)
enable ⇒ disposition_is_refused_helper_work == is_p3
```

P3 changes only outcome lanes to `Exhausted{HelperWork}` and preserves counters, visits, helper, and reference cursor. The applied path charges remaining/consumed/helper-work/helper-visits exactly once, preserves the reference cursor, and guards charged counters against `u128::MAX`. There is correctly no P6 allocation refusal because this rule has no allocation, frame, task, environment, or value-slot delta.

The exported `post_phase`, `post_reg_a`, and `post_staged_*` operands are ungated derived receipts. In particular, `post_phase` remains `Done` for P3 and disabled statements. The consumer correctly does not bind LedgerCall output or the constrained post-state to those receipts. It outputs the actual supplied post image, DATA, and helper operands. The exported `overflow` is exact `0` only when `enable == 0`; the consumer also does not misuse that receipt as post-state evidence.

## Security-relation findings

The host-pair route does not synthesize or overwrite an expected post. Its refusals are prefix, retained-config, geometry, and encoder refusals. An encodable semantically wrong post is placed into the witness and must be rejected by `ProofPreimage::check` preprocessing. That preprocessing result must not be relabelled as native constraint synthesis or proof soundness.

The owner-record route validates detailed internal consistency among the event and both snapshots: rule, disposition, deltas, source occurrence and expression, site, origin cursor, phase/cursor before and after, preservation sections, checked counter arithmetic, and a complete applied post helper recomputed from the actual operand rows using checked host arithmetic.

That validation is not authenticated provenance. `CompleteStepRecord` is publicly constructible, so a forged but internally consistent record can pass `witness_from_owner_record`. Likewise, a helper whose site lane says `RetRight` can satisfy the arithmetic relation without proof that ScalarEntry installed it from an actual top `RightF` frame. Those are retained SI08/SI21 premises, not defects concealed by the consumer.

Exactly one public `record_step` call occurs in `record_scalar_arithmetic_step`. Its non-rollback behavior is explicit: after a wrong-rule or terminal owner step, an `Err` does not imply that the image is unchanged.

LedgerCall commitment is recomputed on every consumer check from the current statement and current post image/DATA/helper lanes. `KeyLocation("sherlock_c6_scalar_arithmetic_step_check")` is diagnostic only and is not native verifier issuance.

The namespace test source covers all eight composed reserved segments across 16 operand groups and checks retained-serial disjointness across repeated emitter calls. That is host emitter-API evidence, not an executed serialization or synthesis result.

## Latest test-only hunks

The raw-search correction is valid. Genesis `helper=None` and non-Scalar helper intermediates now return `false` from raw `is_selected` before destructure, while the subsequent mode, phase, operand, staged, expected, and `reg_a` checks remain intact. It introduces no synthetic admitted state.

The stale-commitment correction is valid as isolated source logic. It mutates only preserved initial-fuel image lane 26, pre and post identically, to `u128::MAX`; first requires the same mutated statement to pass with its recomputed commitment, then requires the old commitment to fail with the exact substring `Communications commitment mismatch`. In the supplied SDK `preprocess`, the instruction and transcript checks precede the final communications-commitment comparison. Therefore a successful paired positive would show that the stale case reaches that comparison. Because the test is UNRUN, actual reachability is not claimed.

The phase/mode correction is honest. Mutating pre and post helper mode/phase identically still also violates the separately constrained applied post phase and, for cases such as `(Second, Not)`, operand-B presence and `reg_a == 0`. The retained assertions prove only the coupled rejection union. The same applies to the changed-`reg_a` case, whose retained staged payload also differs from the payload derived from the mutated `reg_a`. Pairing-predicate isolation remains required-open; no host precheck may substitute for it.

No production guard, configuration, or admission-policy change was introduced by those hunks.

## Test-source findings

The positive source genuinely uses `host_owner_engine`, `prepare`/`admit`/`admit_run`, search through actual `record_step`, capture comparison, a wrapper step on the original image, and an independently replayed raw `record_step`. Stated staged expectations are supplied independently as checked mathematical results rather than copied from emitter receipts.

Positive coverage includes ordinary, zero, maximum, overflow, underflow, high-half multiplication, Lt less/equal/greater, Not true/false, Const and DATA representatives, wrong-kind error 11, genuine P3 for Not and Add, owner-record cross-field negatives, an independent wrong-post host-pair rejection in both modes, and wrapper non-rollback.

Raw coverage includes genuine five-mode positives in both modes, wrong applied post fields, preserved-section representatives, raw `Fr >= 2^128` at gate 1 and gate 0, disabled raw-128/derived-129 slice cases, Ref-space and index representatives, arithmetic wrong-staged companions, P3/disabled/counter cases, live env/value-slot and full-Ref identity, the deliberately unadmitted Marker codec fixture, namespace composition, and the isolated stale-commitment case.

The held-Marker fixture is only a whole-program codec-domain fixture. It is not genuine owner-admitted SI17 evidence and must not be described as such.

## Mandatory fixes

For bounded adoption of the current production and delivered test source: **none found**.

Before any complete test, leaf, runtime, or full-goal claim, all of the following remain mandatory:

1. Add pairing-predicate isolation with coherent post phase, staged value, operand-B optionality, and `reg_a` companions for each rejected pairing, so only the selected phase/mode predicate can reject.
2. Add host geometry, length, and `usize::MAX` cap producer-refusal tests; disabled exported-`overflow == 0` receipt evaluation; systematic all-mode Const/Data operand crosses; true P3 with retained admission `max_helper_work == u128::MAX`; and broader per-lane preserved-section mutations.
3. Run the 22 tests only through a separately eligible Root-owned compilation and SDK execution action. Do not describe source presence as a test pass.
4. Keep the remaining 39 selectors, refinement/witness/constraints/exports, groups 1/2/4/8, residual, finance, persistence, recursion, SI08/SI17/SI21/SI22/StepInv, program membership, P1 issuer/authentication, native, and stagenet work open.

**Overall judgment: AGREE / APPROVE for bounded ScalarArithmetic emitter, consumer, and delivered test-source adoption only. DELIVERY_SCOPE remains CHANGES_REQUIRED.**