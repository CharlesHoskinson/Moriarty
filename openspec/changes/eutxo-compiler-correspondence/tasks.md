# Prospective compiler tasks

Status: S2. All tasks are unchecked and specified-only.

Run future commands from `experiments/moriarty-eutxo`. Commands below are proposed verification commands. This drafting session does not run them.

## C01: Add routing rejection fixtures

- [ ] C01.1 Add fixtures at `experiments/moriarty-eutxo/test/financial-profile-router.test.mjs`.
- [ ] C01.2 Add the controls listed below.

Controls: source profiles /5, /6, private, certificate; wrong contract and unknown lifecycle.
- [ ] C01.3 Confirm failure before behavior changes.
- [ ] C01.4 Implement the reviewed interface at `experiments/moriarty-eutxo/src/financial-profile-router.mjs`.
- [ ] C01.5 Run `node --test test/financial-profile-router.test.mjs`.
- [ ] C01.6 Check the pass predicate below.

Pass predicate: every mismatch refuses without a blueprint.
- [ ] C01.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-001, EUC-016.

## C02: Add exhaustive constructor fixtures

- [ ] C02.1 Add fixtures at `experiments/moriarty-eutxo/test/core-inventory.test.mjs`.
- [ ] C02.2 Add the controls listed below.

Controls: nested ConstructRecord, unknown constructor, Let scope, source spans, work mismatch.
- [ ] C02.3 Confirm failure before behavior changes.
- [ ] C02.4 Implement the reviewed interface at `experiments/moriarty-eutxo/src/core-inventory.mjs`.
- [ ] C02.5 Run `node --test test/core-inventory.test.mjs`.
- [ ] C02.6 Check the pass predicate below.

Pass predicate: all selected-table constructors have coverage; unknown nodes fail.
- [ ] C02.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-002, EUC-003, EUC-009.

## C03: Add real lifecycle ABI fixtures

- [ ] C03.1 Add fixtures at `experiments/moriarty-eutxo/test/lifecycle-codec.test.mjs`.
- [ ] C03.2 Add the controls listed below.

Controls: full HeadDatum, residual 40, wrong tag, extra fields, UTF-8, swapped fields.
- [ ] C03.3 Confirm failure before behavior changes.
- [ ] C03.4 Implement the reviewed interface at `experiments/moriarty-eutxo/src/lifecycle-codec.mjs`.
- [ ] C03.5 Run `node --test test/lifecycle-codec.test.mjs`.
- [ ] C03.6 Check the pass predicate below.

Pass predicate: Lucid definite decode equals admitted source; malformed inputs refuse.
- [ ] C03.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-005, EUC-006, EUC-007, EUC-014.

## C04: Add Aiken full decode controls

- [ ] C04.1 Add fixtures at `experiments/moriarty-eutxo/eutxo-financial/lib/moriarty/codec.ak`.
- [ ] C04.2 Add the controls listed below.

Controls: complete definite HeadDatum and 19-field obligation; constructor arity negatives.
- [ ] C04.3 Confirm failure before behavior changes.
- [ ] C04.4 Implement the reviewed interface at `experiments/moriarty-eutxo/eutxo-financial/lib/moriarty/lifecycle.ak`.
- [ ] C04.5 Run `/home/charl/.aiken/bin/aiken check eutxo-financial`.
- [ ] C04.6 Check the pass predicate below.

Pass predicate: named full-head and residual decode tests pass; swapped fields fail equality.
- [ ] C04.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-005, EUC-006, EUC-014.

## C05: Add digest and runtime pin tests

- [ ] C05.1 Add fixtures at `experiments/moriarty-eutxo/test/financial-pins.test.mjs`.
- [ ] C05.2 Add the controls listed below.

Controls: numeric bounds, atomic domain, zero/equal policies, changed program, self identity cycle.
- [ ] C05.3 Confirm failure before behavior changes.
- [ ] C05.4 Implement the reviewed interface at `experiments/moriarty-eutxo/src/financial-pins.mjs`.
- [ ] C05.5 Run `node --test test/financial-pins.test.mjs`.
- [ ] C05.6 Check the pass predicate below.

Pass predicate: all tampering fails; exact revision03 preimages recompute.
- [ ] C05.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-008, EUC-011.

## C06: Add generic expression tests

- [ ] C06.1 Add fixtures at `experiments/moriarty-eutxo/test/lower-financial-expression.test.mjs`.
- [ ] C06.2 Add the controls listed below.

Controls: each constructor family; zero divisor; short-circuit; financial post reads; type widths.
- [ ] C06.3 Confirm failure before behavior changes.
- [ ] C06.4 Implement the reviewed interface at `experiments/moriarty-eutxo/src/lower-financial.mjs`.
- [ ] C06.5 Run `node --test test/lower-financial-expression.test.mjs`.
- [ ] C06.6 Check the pass predicate below.

Pass predicate: source and generated results match; protected writes refuse.
- [ ] C06.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-002, EUC-003, EUC-007, EUC-010.

## C07: Add complete financial operation tests

- [ ] C07.1 Add fixtures at `experiments/moriarty-eutxo/test/lower-financial-kernel.test.mjs`.
- [ ] C07.2 Add the controls listed below.

Controls: Originate funding, Accrue 101/10, Repay allocation and unsupported standalone Allocate routing, Repay residual40, Fee gross/net.
- [ ] C07.3 Confirm failure before behavior changes.
- [ ] C07.4 Implement the reviewed interface at `experiments/moriarty-eutxo/eutxo-financial/lib/moriarty/kernel.ak`.
- [ ] C07.5 Run `node --test test/lower-financial-kernel.test.mjs`.
- [ ] C07.6 Check the pass predicate below.

Pass predicate: complete states and ordered effects match selected source kernel.
- [ ] C07.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-004, EUC-007, EUC-009, EUC-010.

## C08: Add work and distinct-counter tests

- [ ] C08.1 Add fixtures at `experiments/moriarty-eutxo/test/financial-work.test.mjs`.
- [ ] C08.2 Add the controls listed below.

Controls: packet additive work, exact debit, one-short, closureReserve, early guard paths.
- [ ] C08.3 Confirm failure before behavior changes.
- [ ] C08.4 Implement the reviewed interface at `experiments/moriarty-eutxo/eutxo-financial/lib/moriarty/work.ak`.
- [ ] C08.5 Run `node --test test/financial-work.test.mjs`.
- [ ] C08.6 Check the pass predicate below.

Pass predicate: actual work and source failure accounting match independent packet.
- [ ] C08.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-009.

## C09: Add authenticated binding tests

- [ ] C09.1 Add fixtures at `experiments/moriarty-eutxo/test/financial-bindings.test.mjs`.
- [ ] C09.2 Add the controls listed below.

Controls: joint signers, Args=TxInfo, missing observation trust, reference-only head.
- [ ] C09.3 Confirm failure before behavior changes.
- [ ] C09.4 Implement the reviewed interface at `experiments/moriarty-eutxo/eutxo-financial/lib/moriarty/bindings.ak`.
- [ ] C09.5 Run `node --test test/financial-bindings.test.mjs`.
- [ ] C09.6 Check the pass predicate below.

Pass predicate: missing authenticated context fails spending entrypoint.
- [ ] C09.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-010, EUC-011.

## C10: Add compiler failure publication tests

- [ ] C10.1 Add fixtures at `experiments/moriarty-eutxo/test/compile-financial.test.mjs`.
- [ ] C10.2 Add the controls listed below.

Controls: stale blueprint, failed check/build, missing named test, changed dependencies.
- [ ] C10.3 Confirm failure before behavior changes.
- [ ] C10.4 Implement the reviewed interface at `experiments/moriarty-eutxo/src/compile-financial.mjs`.
- [ ] C10.5 Run `node --test test/compile-financial.test.mjs`.
- [ ] C10.6 Check the pass predicate below.

Pass predicate: failure exits nonzero; no success blueprint; historical logs retained.
- [ ] C10.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-012.

## C11: Bind independent source envelopes

- [ ] C11.1 Add fixtures at `experiments/moriarty-eutxo/test/financial-correspondence.test.mjs`.
- [ ] C11.2 Add the controls listed below.

Controls: all frozen packet sourceBindings and compiledValidatorTemplates; differing balance/allowance/id/work states.
- [ ] C11.3 Confirm failure before behavior changes.
- [ ] C11.4 Implement the reviewed interface at `experiments/moriarty-eutxo/test/fixtures/financial-correspondence/input-manifest.json`.
- [ ] C11.5 Run `node --experimental-strip-types --test test/financial-correspondence.test.mjs`.
- [ ] C11.6 Check the pass predicate below.

Pass predicate: accepted complete outputs match; refused cases cannot spend.
- [ ] C11.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-013, EUC-014.

## C12: Execute actual generated spending validator

- [ ] C12.1 Add fixtures at `experiments/moriarty-eutxo/test/financial-uplc.test.mjs`.
- [ ] C12.2 Add the controls listed below.

Controls: decoded inputs and independently reconstructed ScriptContext; source failure controls.
- [ ] C12.3 Confirm failure before behavior changes.
- [ ] C12.4 Implement the reviewed interface at `experiments/moriarty-eutxo/test/fixtures/financial-correspondence/uplc-results.json`.
- [ ] C12.5 Run `node --experimental-strip-types --test test/financial-uplc.test.mjs`.
- [ ] C12.6 Check the pass predicate below.

Pass predicate: actual pinned compiled UPLC accepts positives and refuses negatives.
- [ ] C12.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-013, EUC-014.

## C13: Reconcile twenty-family evidence

- [ ] C13.1 Add fixtures at `experiments/moriarty-eutxo/test/financial-coverage.test.mjs`.
- [ ] C13.2 Add the controls listed below.

Controls: PATTERN-INVENTORY ids and T1–T18; five evidence tiers; unrun native/private rows.
- [ ] C13.3 Confirm failure before behavior changes.
- [ ] C13.4 Implement the reviewed interface at `experiments/moriarty-eutxo/artifacts/financial-coverage.json`.
- [ ] C13.5 Run `node --test test/financial-coverage.test.mjs`.
- [ ] C13.6 Check the pass predicate below.

Pass predicate: 20 unique ids remain mapped; unrun tiers cannot advance.
- [ ] C13.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-015, EUC-016.

## C14: Draft bounded lifecycle-operation amendment

- [ ] C14.1 Add fixtures at `experiments/moriarty-eutxo/docs/financial-lifecycle-extensions.md`.
- [ ] C14.2 Add the controls listed below.

Controls: S03 unsupported split; S04 outstanding80 termination refusal; conserved duties.
- [ ] C14.3 Confirm failure before behavior changes.
- [ ] C14.4 Implement the reviewed interface at `experiments/moriarty-eutxo/test/lifecycle-extension-contract.test.mjs`.
- [ ] C14.5 Run `node --test test/lifecycle-extension-contract.test.mjs`.
- [ ] C14.6 Check the pass predicate below.

Pass predicate: independent duty oracle exists before new operations are admitted.
- [ ] C14.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-004, EUC-015.

## C15: Freeze source and compiled dependency manifests

- [ ] C15.1 Add fixtures at `experiments/moriarty-eutxo/test/financial-artifact-drift.test.mjs`.
- [ ] C15.2 Add the controls listed below.

Controls: moving custody source changes, dependency edits, stale runtime pins.
- [ ] C15.3 Confirm failure before behavior changes.
- [ ] C15.4 Implement the reviewed interface at `experiments/moriarty-eutxo/artifacts/financial-compiler-manifest.json`.
- [ ] C15.5 Run `node --test test/financial-artifact-drift.test.mjs`.
- [ ] C15.6 Check the pass predicate below.

Pass predicate: drift refuses exact candidate consumption; no broad approval inferred.
- [ ] C15.7 Retain input hashes, logs, result layers, and refusal codes.

Requirements: EUC-008, EUC-011, EUC-012, EUC-013.

## Required dependency order

Complete C01–C05 before broad lowerer admission. Add C06–C09 tests before their behavior changes. Complete C10 before publishing compiler artifacts. Complete C11 before C12. Complete C12 before upgrading UPLC evidence. Native-runtime tasks remain independently owned. C14 requires separate language and ledger design review. C15 freezes the exact candidate before independent result review.

## Exact fixture snippets

Add these adversarial fixture assertions to C11. Load the immutable packet by its pinned manifest.

```js
assert.equal(post('release-lender-balance-7').balance('Lender','Cash'), '107');
assert.deepEqual(post('release-prior-used-id').usedTransferIds, ['PriorSpend','Release1']);
assert.equal(post('release-revision-4').ordinary.revision, '5');
assert.equal(post('release-work-additive').work.remaining, '151');
assert.equal(post('release-work-additive').work.spent, '59');
assert.equal(result('e12-cancel-residual').admissionCode, 'NONEMPTY_OBLIGATIONS');
assert.equal(result('e12-cancel-residual').sourceStatus, 'FundedExpressionPrepared');
```
These adapters are proposed fixture accessors. Their values must come from actual generated-spending execution, not expected fixture copies.

## C16: Draft structural verification

- [ ] C16.1 Run `openspec validate eutxo-compiler-correspondence --strict --json --no-interactive`.
- [ ] C16.2 Save output in the compiler evidence directory.
- [ ] C16.3 Report structural validation separately from semantic or capability approval.

C16 checks OpenSpec structure only. It does not satisfy EUC-015 or EUC-016 semantic acceptance.
