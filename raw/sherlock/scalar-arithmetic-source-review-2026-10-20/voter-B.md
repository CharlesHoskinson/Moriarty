Top finding: the bounded five-mode production relation is sound as source, and the three test-only hunks are substantively correct; I find no mandatory production fix. The candidate is nevertheless not a complete leaf, not compiled, and not executed.

I am Muse Wonder 1.0 HIGH, acting as fresh independent non-author reviewer B under the stated Opus-outage fallback. This judgment uses only the inline source and evidence supplied. I did not approve anything by hash or by the author responses.

## Verdicts

- **DESIGN: AGREE / APPROVE** — bounded to `(First, Not)` and `(Second, Add|Sub|Mul|Lt)`.
- **SECURITY_RELATION: AGREE / APPROVE** — bounded to the constrained statement relation and the disclosed internal-consistency host routes, under the retained premises below.
- **SOURCE_API: AGREE / APPROVE** — bounded static source judgment only. This is not a Rust typecheck.
- **TEST_SOURCE: AGREE / APPROVE** — bounded to the delivered 10 positive + 12 raw test sources as UNRUN source, including the three corrected hunks.
- **COVERAGE_SCOPE: CHANGES_REQUIRED** — for any claim that the required test coverage is complete. The open items listed below remain mandatory for that stronger claim.
- **DELIVERY_SCOPE: CHANGES_REQUIRED** — for any complete-leaf, runtime, proof, native, degree, performance, or whole-goal claim.
- **OVERALL: AGREE / APPROVE** — for bounded source adoption of emitter + consumer + registration + these test sources only.

## Basis, missing premises, and read coverage

I reviewed the complete candidate emitter, consumer, both test files, and parent registration, plus the supplied Core Scalar fragment and owner selector/driver, SI21/SI22 state rules, checked-value reader, canonical META/PROG/CTX/DATA/helper blocks, retained Push family adapter, physical codecs, complete-observation recorder, owner prepare/admission units used by builtin fixtures, and the called SDK IR/VM, `Fr`, `ProofPreimage::check`, transient-commit, decomposition, comparison, and native instruction bodies.

No actually called production definition needed for the bounded judgments was missing. The disclosed exclusions are uncalled `cfg(test)` bodies, nominal Ext producers not used by these builtin fixtures, unused key/prove/example SDK material, and unrelated backend lowering. The whole backend crate root was not supplied, so a possible deny-warnings lint gate cannot be proved from the supplied header alone; that is a delivery/lint residual, not a demonstrated relation defect.

The two supplied C2 reviews approve only the earlier emitter+consumer production source. They do not approve these tests. The 19 branch examples are author-visible mathematical examples, not an independent admitted-state oracle.

## Design findings

The emitter implements exactly five selected shapes:

- `First + Not`, pre-cursor `0`, `regA == 0`, applied post `Done`/cursor `2`.
- `Second + Add|Sub|Mul|Lt`, pre-cursor `1`, applied post `Done`/cursor `2`.
- P3 preserves the actual pre phase and cursor: `First/0` for `Not`, `Second/1` for numeric modes.

`Not` reads operand A. Bool stages `BoolValue{!b}`; every other kind stages `PrimError{11}`. Numeric modes read operand B for the right operand and use pre `regA` as the left operand. Non-UInt operand B stages error `11`; Add/Sub/Mul stage the checked `u128` result or error `10`; Lt stages `BoolValue{regA < n}`.

The numeric `regA` rule is correctly stronger than bare Core arithmetic plus selector. Core `plan_scalar` trusts the already-installed `regA`; this circuit and the owner-record validator require operand A to be UInt and require `pre regA == reader_a.uint_value`. That is the valid SI22 Second-state denotation for this bounded relation. It does not prove the predecessor ScalarTag step or full SI22.

The checked arithmetic is correct as written:

- Add splits the exact sum below `2^129` at `2^128`.
- Sub uses the same `LessThan` result as both Lt and the 128-bit borrow.
- Mul decomposes both operands into 64-bit limbs, forms the full cross sum, and defines overflow exactly as `x1*y1 + c1 != 0`, retaining the complete high half before projecting the low 128 bits.
- The `2^64 * 2^64` and `max * max` wrong-low-only-success cases are therefore properly rejected when staged as `U128Value`.

P3 is computed against the original separately retained admission bound:

```text
is_p3 == (pre.helper_work == source_params.admission_bounds().max_helper_work)
```

P3 is not derived from layout bounds, post labels, or a host flag. Applied steps charge remaining `-1`, consumed `+1`, helper-work `+1`, and helper-visits `+1`, with charged-counter overflow guards. There is no allocation or value-slot delta for this rule, so there is correctly no P6 allocation refusal in this emitter.

## Security-relation findings

The consumer does not bind post-state or LedgerCall output to the emitter’s ungated receipts. `block.post_phase` is the derived value `Done` even for P3 or disabled execution. LedgerCall output instead uses the actual supplied `post_image`, `post_data`, and `post_helper` statement operands, which are the operands constrained by the emitted helper-post block.

The host-pair route is an independent-post route. `witness_from_host_pair` refuses prefix/config/encoder problems only; it does not synthesize an expected post and does not semantically repair a wrong post. An encodable wrong staged payload therefore reaches `ProofPreimage::check`, as required.

LedgerCall commitment handling is current-statement binding: the consumer recomputes `transient_commit` from the current statement plus the current post-flat lanes on every check. Ordinary raw negatives do the same. The stale-commitment case is the only labelled exception.

The owner-record route validates internal consistency only. `CompleteStepRecord` is publicly constructible, so a forged record with mutually consistent event and snapshots can pass `witness_from_owner_record`. That is not authenticated owner provenance, and the source correctly says so. `record_scalar_arithmetic_step` calls the real `record_step` exactly once and correctly documents that an `Err` return does not roll back that step.

Retained premises remain open and are not discharged by these sources: SI08/SI21 ScalarEntry installation from the actual control and top frame, frame/site agreement beyond record-internal site equality, full SI17 marker placement, full SI21/SI22/StepInv, program membership, P1 native issuance, SourceSpec authentication, and exactly-one whole-step dispatch.

Counterexample that remains outside the approved relation: a canonical Second/Add helper with valid operand rows and `regA`, but whose site lane was not installed from the actual top `RightF` frame, can satisfy the arithmetic relation. That does not contradict the bounded approval because ScalarEntry/top-frame coherence is an explicit retained premise.

`KeyLocation("sherlock_c6_scalar_arithmetic_step_check")` is diagnostic only. It is not a native verifier issuer or ledger deployment authority.

## Source/API findings

The parent registration exposes the consumer names actually defined by the consumer:

- `build_c6_scalar_arithmetic_step_circuit`
- `check_c6_scalar_arithmetic_step_circuit`
- `record_scalar_arithmetic_step`
- the two aliased witness constructors
- `C6ScalarArithmeticConsumerError`
- `C6ScalarArithmeticStepCircuit`
- `C6ScalarArithmeticStepWitness`

The called SDK shapes match the supplied bodies, including `PublicInput`, `Impact`, `Output`, `TestEq`, `LessThan`, `CondSelect`, `DivModPowerOfTwo`, `ConstrainBits`, `ConstrainEq`, `ConstrainToBoolean`, `IrSource::load`, `TypedIdentifier`, `transient_commit`, and the preprocessing order in `IrSource::preprocess`.

Error formatting does not impose a prohibited `Debug` bound on `CaptureStepError`, `Tooling`, or `ExtTypeBuildError`. The labels destructure `CaptureStepError` and format only fields supported by the supplied declarations. Field-value `{:?}` uses on supplied violation and snapshot fields are supported by their actual declarations.

Namespace composition is coherent. The parent scalar emitter reserves the eight composed segments used by the parent and `.arithread` readers, rejects caller operands under those prefixes, and retains serials across retained-emitter calls. The raw test exercises all eight segments against all 16 operand groups and checks repeated-emitter destination disjointness for the destination-producing instruction forms used by these emitters.

## Test-source findings

The actual current count is **10 positive + 12 raw = 22 tests**. The historical “11 + 9” owner prose is wrong and was not fitted to the source. All 22 remain UNRUN and uncompiled.

The positive file uses the genuine route required here: `host_owner_engine`, `prepare`/`admit` through the public owner path as actually wired, `admit_run`, search by real `record_step`, one selected step through `record_scalar_arithmetic_step`, and an independent replay through raw `record_step`. It compares complete captures and the event, checks both statement modes through the real circuit builder and `check_c6_scalar_arithmetic_step_circuit`, and states expected staged values from independent checked host arithmetic rather than copying emitter receipts.

The latest three hunks are correct:

1. **Raw `is_selected` fix — correct.** Genesis `helper=None` and non-Scalar helpers now return `false` before the Scalar destructure. The later mode, phase, staged, expected, operand-kind, and `regA` checks are unchanged.
2. **Stale-commitment fix — correct as source.** The test mutates only preserved image initial-fuel lane `26`, identically in pre and post, asserts the recomputed commitment differs, proves the same mutated statement passes with the current recomputed commitment, and then requires the old commitment to fail with the exact SDK string `Communications commitment mismatch`. From the supplied SDK source, instruction and transcript checks precede that final commitment comparison. Because the test is UNRUN, actual reachability is not claimed as observed.
3. **Phase/mode correction — honest.** The test now withdraws the false claim that pre/post-identical mode/phase mutations isolate only the pairing predicate. Several cases also violate the applied post-phase constraint, operand-B presence, or the staged payload derived from `regA`. Pairing-predicate isolation remains open.

The held-Marker raw fixture is also honestly scoped. It is built through the real program codec with roundtrip and exact-length checks, but it is a deliberately unadmitted schema-domain fixture. It proves the reader stages Marker `Not` as error `11`; it is not genuine owner-admitted SI17 evidence.

## Coverage assessment and mandatory coverage additions

Present as source: five genuine modes in both statement modes; Add/Sub/Mul/Lt/Not branch cases; overflow and underflow; high-half Mul; wrong-kind error `11`; genuine applied counters; genuine P3 for Not/Add in the positive file and Add in the raw file; raw `Fr >= 2^128` rejection under gate `1` and gate `0`; disabled raw-128 slice-end acceptance at `2^128` and `2^129 - 2`; Ref space value `2`; Const and DATA equal-to-count and capacity-padding indices; wrong post phase, `regA`, staged tag/payload, Ref indices, result record, task count, outcome, cursor, counters, and preserved META/PROG/CTX/image/DATA sections; live env/value-slot Ref identity; all eight namespace segments against 16 operand groups; and repeated retained-emitter serial checks.

The following remain **mandatory before a coverage-complete claim**. They are not blockers to the bounded source approval above, and they must not be relabelled as already covered:

1. **Pairing-predicate isolation.** Add coherent companions for invalid pairs such as `(Second, Not)` and `(First, Add)`: applied post phase `Done`, staged tag/payload recomputed for the mutated mode, `operandB` present exactly for numeric modes, and `regA == 0` for `Not` or `regA == operandA UInt payload` for numeric modes, with valid Ref rows. Only then attribute rejection to the pairing or SI22 predicate alone. The same isolation requirement applies to the current Second/`regA` mismatch case.
2. **Host producer refusals.** Add direct tests for META/PROG/CTX `HostPrefixMismatch`, geometry and length mismatch, cap/geometry producer refusal, and retained-config mismatch. Current tests mostly assert checker rejection, not these producer categories.
3. **Disabled exported overflow receipt.** Add a test that actually evaluates or otherwise binds `block.overflow == 0` under `enable == 0`. Disabled statement equality alone does not test that exported receipt, which the consumer discards.
4. **Operand-space crosses.** Add genuine fixtures for every numeric mode with DATA operand B and the remaining Const/DATA operand-A/B crosses not already represented. Current Data-B coverage is representative, not exhaustive across all modes and crosses.
5. **True retained-max P3 at `u128::MAX`.** Add a raw statement whose retained admission `max_helper_work` is itself `u128::MAX`, with pre helper-work `u128::MAX`, disposition `1`, preserved counters/helper, and Exhausted HelperWork outcome accepted. The existing charged-`MAX` rejection with retained max `100` is the complementary negative, not this case.
6. **Per-lane preserved-section coverage.** Extend representative preserved-section mutations to the remaining individual lanes or provide an explicit lane-equivalence argument. Current source covers representatives, not every lane.

## Delivery scope

This review grants no execution authority. Compilation, Cargo, SDK preprocessing execution, synthesis, MockProver, proof generation, native execution, degree/`k`, performance, ledger deployment, and stagenet qualification remain unestablished. SDK/native C13 remains UNSELECTED; native23 remains `c=null`, `S NOT_ESTABLISHED`, stopped at Am10, with caps untouched.

Final judgment: **approve the bounded production + test source; require changes before any coverage-complete, complete-leaf, runtime, proof, native, or whole-goal claim.**