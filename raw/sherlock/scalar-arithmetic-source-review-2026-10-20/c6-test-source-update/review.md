Top finding: the C6 post-mode preservation fix removes the demonstrated C5 coupling, and the new pairing and producer tests are substantively sound as source. No mandatory defect was found in the bounded production or 32-test source. All 32 tests remain UNRUN and uncompiled.

I am Muse Wonder 1.0 HIGH, acting as a fresh independent non-author reviewer under the stated Opus-outage fallback. The author responses are not votes. I did not approve anything by hash. Earlier Opus labels are historical only.

## Verdicts

- **DESIGN: AGREE / APPROVE**, bounded to `(First, Not)` and `(Second, Add|Sub|Mul|Lt)`.
- **SECURITY_RELATION: AGREE / APPROVE**, bounded to the constrained selected-step relation and disclosed host routes, under the retained premises below.
- **SOURCE_API: AGREE / APPROVE**, bounded static source/API review only. This is not a Rust typecheck.
- **TEST_SOURCE: AGREE / APPROVE**, bounded to 10 positive plus 22 raw UNRUN test sources, including the 10 tests added by C5 and the one-test C6 correction.
- **PAIRING_ISOLATION: AGREE / APPROVE** as source-level isolation by construction. This is not observed runtime reachability.
- **HOST_PRODUCER_REFUSALS: AGREE / APPROVE**, with the CTX and `max_data_nodes` obstructions below expressly not counted as direct coverage of those two guards.
- **COVERAGE_SCOPE: CHANGES_REQUIRED** for any coverage-complete claim.
- **DELIVERY_SCOPE: CHANGES_REQUIRED** for any complete-leaf, runtime, proof, native, degree, performance, or whole-goal claim.
- **OVERALL: AGREE / APPROVE for bounded source adoption only**.

## Read coverage

The decisive reviewed source comprised:

- The five complete current Rust files, first and whole: ScalarArithmetic emitter, real consumer, 10 positive tests, 22 raw tests, and normal parent registration.
- Actual Core `plan_scalar` arithmetic, owner selector, SI21/SI22 Scalar state rules, checked-value reader, and complete-observation recorder.
- Called META, PROG/CTX, DATA, and helper canonical emitters.
- `C6PushFamilyParams::new`, `validate_geometry`, `C6PushHostPrefixLanes::from_host_snapshot`, `encode_context`, checked padded geometry, and Core physical codecs.
- Called SDK `IrSource::load`, `IrSource::preprocess`, `ProofPreimage::check`, `transient_commit`, `Fr`, and the called Add, Mul, Neg, Not, TestEq, LessThan, CondSelect, and DivMod instruction bodies.
- The complete historical C5 `coherent_first_numeric_pairings_reject_both_modes` function and the current C6 replacement.

No actually called definition needed for these bounded judgments was replaced by an ellipsis or digest in the packet I assessed. The disclosed exclusions remain uncalled Push-only circuit/record/projection/decoder implementations, SDK key/SRS/cache/native-verifier implementations, unrelated backend artifact/action lowering, redundant mechanical/history JSON, original unused SDK `cfg(test)` and key/prove examples, and nominal Ext producers not used by these fixtures.

The passive whole-root AST observation of zero global inner attributes is not a whole-crate compilation claim.

## Missing premises

The following are explicitly **not established**:

- Compilation or execution of any of the 32 tests.
- SDK preprocessing execution, synthesis, MockProver, proof generation, degree/`k`, or performance.
- SI08/SI21 entry coherence, including proof that ScalarEntry installed the helper from the actual control and top frame: `RetUnary` for `Not`, `RetRight` for numeric modes, with site/frame agreement.
- Full SI17 marker placement.
- Full SI21, SI22, StepInv, SI08, program membership, exactly-one whole-step dispatch, and the full 39-selector relation.
- P1 native issuance, native key issuance, SourceSpec authentication, ledger deployment, finance, persistence, recursion, and stagenet qualification.
- An independent private whole-state oracle for the genuine fixtures.
- SDK/native C13 remains **UNSELECTED**. Native23 remains `c=null`, `S NOT_ESTABLISHED`, stopped at Am10, with caps unchanged.

## Design

The emitter selects exactly:

```text
(First, Not)
(Second, Add | Sub | Mul | Lt)
```

Its effective gated pairing is:

```text
enable ⇒
  ((phase == First && mode == Not) ||
   (phase == Second && mode ∈ {Add, Sub, Mul, Lt}))
```

Mode bits are derived from the pre-helper mode lane. `is_num` is the sum of the Add/Sub/Mul/Lt bits. Reader A is gated by `enable`. Reader B is gated by `enable && is_num`, so the absent `operandB` lanes for `Not` are not interpreted as a reference.

For `Not`:

- Operand A Bool stages `BoolValue { !b }`.
- Every other operand-A kind stages `PrimError { 11 }`.
- Pre `regA == 0` is enforced.

For numeric modes:

- The actual operand-B row is read.
- Non-UInt operand B stages `PrimError { 11 }`.
- `Lt` stages `BoolValue { regA < n }`.
- Add/Sub/Mul stage the checked mathematical `u128` result, or `PrimError { 10 }` outside `[0, 2^128)`.
- Under `enable && numeric`, the circuit additionally enforces:

```text
reader_a.is_uint == 1
pre.regA == reader_a.uint_value
```

That numeric `regA` condition is the valid SI22 Second-state denotation for this bounded relation. It is stronger than bare Core `plan_scalar` plus selector, because Core trusts an already-installed `regA`. It does not prove the predecessor ScalarTag step or full SI22.

Checked multiplication is sound as written. With 64-bit limbs, every partial product is below `2^128`, every relevant sum is far below the BLS12-381 scalar modulus, and overflow is exactly:

```text
x1*y1 + c1 != 0
```

The complete high half is retained before projecting the low 128 bits. Addition splits the exact sum below `2^129` at `2^128`. Subtraction reconstructs `x - y + borrow*2^128` before the corresponding split.

P3 is:

```text
is_p3 =
  pre_image[L_HELPER_WORK]
  == source_params.admission_bounds().max_helper_work

enable ⇒ disposition_is_refused_helper_work == is_p3
```

The bound is the original separately retained admission bound. It is not derived from post labels, layout bounds, or a host flag. P3 changes only the outcome lanes to `Exhausted { HelperWork }` and preserves counters, visits, helper state, and reference cursor. The applied path charges remaining work by `-1` and consumed work, helper work, and helper visits by exactly `+1`. It guards charged counters against `u128::MAX`. There is correctly no P6 delta for this rule because it has no allocation, environment, frame, task, or value-slot delta.

## Security relation

The consumer does not bind post-state, LedgerCall output, or statement output to the emitter's ungated receipts. In particular:

- Exported `post_phase` is always the derived value `Done`, including P3 and disabled statements.
- Exported `post_reg_a`, `post_staged_tag`, and `post_staged_payload` are likewise derived receipts, not constrained post-state evidence.
- LedgerCall output uses the actual supplied post image, post DATA, and post helper operands.
- Exported `overflow` is exact `0` when `enable == 0`; the consumer does not misuse it as post-state evidence.

The independent host-pair route does not synthesize, regenerate, or semantically repair a post state. It refuses only prefix, retained-config, geometry, and encoder problems. A semantically wrong post that encodes therefore reaches `ProofPreimage::check`. That is a preprocessing result only, not synthesis or proof soundness.

The owner-record route validates internal consistency only. `CompleteStepRecord` is publicly constructible, so a forged but internally consistent record can pass. That limitation is correctly stated in source and is not authenticated provenance.

A concrete retained-premise counterexample remains outside the bounded relation: a canonical Second/Add helper with valid operand rows and valid `regA`, but whose site lane was not installed from the actual top `RightF` frame, can satisfy this relation. This does not defeat the bounded approval because ScalarEntry/top-frame coherence is an explicit retained SI08/SI21 premise.

`record_scalar_arithmetic_step` calls the real `record_step` exactly once. Its non-rollback contract is real: an `Err` after that one owner step does not mean the image is unchanged.

LedgerCall communications commitments are recomputed from the current statement and current post-flat lanes on every consumer check. The only stale-commitment case remains separately labelled in the raw tests and is not used as a relation-rejection proxy.

`KeyLocation("sherlock_c6_scalar_arithmetic_step_check")` is diagnostic only. It is not a native verifier issuer, circuit authentication, or ledger authority.

Error formatting does not impose a prohibited `Debug` expectation on `CaptureStepError`, `Tooling`, or `ExtTypeBuildError`. The labels destructure the supplied error forms and display only supported fields.

## Pairing isolation

### C5 defect and C6 correction

The historical C5 function mutated the pre mode to Sub, Mul, or Lt while leaving post helper relative lane 5 at Add (`0`). The emitter's applied helper loop allows post/pre differences only at:

- relative lane 13: phase,
- relative lane 15: staged tag,
- relative lane 16: staged payload.

Therefore Sub/Mul/Lt post mode `Add` independently violated helper mode preservation. The old rejection was not pairing-only.

The current C6 function writes the same mutated mode to both:

```rust
s[pre_helper_at(w, 5)] = Fr::from(mode);
s[post_helper_at(w, 5)] = Fr::from(mode);
```

For Add, that post write is value-identical to the genuine base. For Sub/Mul/Lt, it removes the demonstrated preservation violation.

I found no remaining coupling in these four cases by source enumeration:

- Pre phase is First; applied post phase remains Done.
- Pre staged remains `None`.
- Mode-correct post staged values are recomputed for `(regA, operandB) = (7, 3)`:
  - Add: tag `1`, payload `10`.
  - Sub: tag `1`, payload `4`.
  - Mul: tag `1`, payload `21`.
  - Lt: tag `2`, payload `0`.
- Operand B remains present and UInt `3`.
- Operand A remains the same UInt `7` Ref row, with the same kind/index identity and marker lanes.
- `regA` remains `7` in pre and post.
- Expected remains absent.
- Result remains absent and task count remains zero.
- Common site, origin cursor, and absent `parent_resume` remain unchanged.
- Applied P4/P7 counters, outcome, reference cursor, and all META/PROG/CTX/image/DATA preservation lanes remain coherent.
- LedgerCall uses the fresh current mutated statement and current post-flat lanes.

The generic SDK `Err` is not treated as a per-predicate oracle. Isolation follows from construction and the unchanged paired positive in both modes.

### `Second + Not`

Both new cases mutate only pre phase from First to Second:

- Bool operand A retains post `Done` with `BoolValue { false }` for `Not(true)`.
- UInt operand A retains post `Done` with `PrimError { 11 }`.
- Operand B remains absent.
- `regA` remains `0`.
- The same operand-A Ref identity remains in pre and post.
- Mode remains `Not` in both copies.

No other emitter predicate rejects solely from that phase change: reader B is gated off, operand-B absence matches `is_num == 0`, and the Not-specific `regA == 0` gate remains satisfied.

### `Done + Add` and `Done + Not`

These mutate only pre phase to Done. Applied post phase is already Done, staged companions remain the genuine applied companions, and all other selected-state companions remain coherent. The pairing predicate is therefore the source-level rejecting predicate.

### Numeric `regA` isolation

The new case changes pre and post `regA` from `7` to `8`, while the actual operand-A row remains UInt `7`. It recomputes the staged payload as:

```text
8 + 3 = 11
```

with tag `U128Value`. Post `regA` follows the mutated pre lane, so helper preservation is satisfied. The remaining source-level rejection is:

```text
reader_a.uint_value == 7
pre.regA == 8
```

under the numeric SI22 denotation gate.

## Host producer refusals

### META and PROG prefix mismatches

Consumer order is:

```text
C6PushFamilyParams::new
→ pre prefix from C6PushHostPrefixLanes::from_host_snapshot
→ post prefix from the same constructor
→ META comparison
→ PROG comparison
→ CTX comparison
→ pre/post state split
```

The META mutation changes only post `run_id.fuel_input` to `Nat(65)`. That is a META lane source. It remains encode-valid, does not alter PROG or CTX derivation, and must refuse as:

```rust
HostPrefixMismatch { section: "meta" }
```

The PROG mutation increments only post `admission.view.entry`. The resulting whole `ProgramSnapshot` still passes through the real codec inside `from_host_snapshot`. Program entry is not a META source, so this reaches:

```rust
HostPrefixMismatch { section: "prog" }
```

Each has the unmutated genuine host pair as a paired control. Neither uses `anyhow`-style untyped refusal, a fake category, or a private admission guard.

### CTX obstruction

The standalone CTX mismatch is genuinely unreachable through the current public API:

```text
witness_from_host_pair(enable, pre, post, cfg, limits)
```

takes one `cfg` and calls `from_host_snapshot` for both states with that same `cfg`. Each prefix derives:

```text
ctx = encode_context(cfg.ext, cfg.budgets)
```

It does not read a CTX source from either `HostSnapshot`. Hence `pp.ctx == qp.ctx` for every input pair, and `HostPrefixMismatch { section: "ctx" }` cannot be produced through this normal API.

The test correctly proves the derivation and a paired positive instead of constructing a fake CTX post. This is an obstruction report, not direct CTX-refusal coverage and not a coverage waiver.

### Retained-config mismatch

Changing retained layout bounds from `100` to `101` gives the exact order:

1. `C6PushFamilyParams::new` is constructed first.
2. The pre-prefix builder checks `host.admission.layout != cfg.layout`.
3. It refuses before any META/PROG/CTX comparison.

The asserted category is exact:

```rust
Push(C6PushStepError::ConfigMismatch { field: "layout" })
```

### Geometry before lengths

Emitter check order is:

```text
validate source_params.validate_geometry(supplied geometry)
→ usize::MAX cap loop
→ image/data/helper section-length loop
→ META host-prefix length/offset checks
```

Increasing `image_total_lanes` by one therefore reaches:

```rust
Push(C6PushStepError::InvalidGeometry { section: "geometry" })
```

before either caps or section lengths.

Removing one pre-image lane with otherwise valid geometry reaches:

```rust
LengthMismatch {
    section: "image",
    expected: geometry.image_total_lanes,
    actual: geometry.image_total_lanes - 1,
}
```

The test asserts both section and exact counts, with a valid paired control.

### State limit and checked geometry

For `max_state_lanes = state_total_lanes - 1`, normal params construction reaches:

```rust
C6PushStepError::StateLimitExceeded {
    needed: state_total_lanes,
    limit: state_total_lanes - 1,
}
```

For `frames = usize::MAX`, checked image geometry overflows before carrier allocation:

```rust
C6PushStepError::GeometryOverflow { section: "image" }
```

No huge allocation is sized from the mutated lane or cap.

### `usize::MAX` caps

`max_constants = usize::MAX` is reachable through the normal retained-params chain because it is not a padded-carrier lane multiplier. Geometry recomputes identically, and the emitter cap loop then reaches:

```rust
C6ScalarArithmeticStepError::GeometryOverflow { section: "caps" }
```

By contrast, `max_data_nodes = usize::MAX` is dominated earlier. Normal params construction computes its checked data geometry first and refuses:

```rust
C6PushStepError::GeometryOverflow { section: "data.data" }
```

Therefore the emitter's own `max_data_nodes` cap guard is not reachable through retained params. The test reports that domination instead of fabricating a direct guard test. This is accurate and must not be relabelled as direct emitter-cap coverage for `max_data_nodes`.

## Test source

The positive tests use the genuine owner route:

- `host_owner_engine`
- real public admission and genesis
- search through real `record_step`
- selected wrapper step on the original image
- independent replay through raw `record_step`
- complete capture and event comparison
- both `LocalPublic` and `LedgerCall` through the actual circuit builder and consumer checker

Expected staged values are independently stated checked mathematical results, not copied from emitter receipts.

The raw positives begin from real admitted owner bases. Mutations are then applied to the raw statement domain. Those mutated statements are not owner-admitted states and are correctly not described as such.

The raw suite retains the previously reviewed coverage, including:

- genuine five-mode positives in both modes,
- wrong applied post fields,
- preserved-section representatives,
- raw `Fr >= 2^128` under gate 1 and gate 0,
- Ref space/index boundaries,
- arithmetic wrong-staged companions,
- P3, disabled, and counter-max cases,
- live env/value-slot and full-Ref identity,
- the deliberately unadmitted Marker codec fixture,
- all eight namespace segments across 16 operand groups,
- repeated retained-emitter serial disjointness,
- the separately labelled stale-commitment case.

The Marker fixture remains codec-domain evidence only. It does not establish SI17.

Existing coupled phase/mode negatives remain honestly labelled as coupled rejection unions. The stale-CC paired positive remains unchanged.

## Mandatory findings

### Blocking bounded source adoption

**None found.**

### Mandatory before any stronger claim

1. Execute all 32 tests only through a separately eligible Root-owned compilation and SDK execution action. Source presence is not a test pass.
2. Keep these four historical coverage classes open:
   - disabled exported `overflow == 0` receipt evaluation;
   - systematic all-mode Const/DATA operand A/B crosses;
   - true P3 with retained admission `max_helper_work == u128::MAX`, using a helper-work-max helper rather than only the complementary charged-MAX rejection;
   - broader per-lane preservation/equivalence coverage.
3. Keep standalone CTX `HostPrefixMismatch` coverage open as an API obstruction. It is unreachable through the one-`cfg` host-pair constructor; do not fabricate it.
4. Keep the dominated `max_data_nodes == usize::MAX` emitter cap guard distinct from the reachable checked-geometry refusal.
5. Keep SI08, SI17, SI21, SI22, StepInv, program membership, P1 issuer/SourceSpec authentication, exactly-one whole-step dispatch, the other selectors, full39, witness/constraints/export work, groups 1/2/4/8, residual, finance, persistence, recursion, native, and stagenet items open.
6. Do not claim typechecking, execution, synthesis, proof, native issuance, degree/`k`, performance, or complete ScalarArithmetic leaf status from this review.

**Overall judgment: AGREE / APPROVE for bounded ScalarArithmetic production and 32-test source adoption only. COVERAGE_SCOPE and DELIVERY_SCOPE remain CHANGES_REQUIRED for stronger claims.**