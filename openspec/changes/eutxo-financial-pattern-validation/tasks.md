# Draft financial validation tasks

All tasks are unchecked and prospective. No implementation or native command is authorized by this draft.

## Common proposed verification interface

Create fixture and test inputs before backend behavior changes. Use immutable external source inputs. Keep one instruction per sentence.

Proposed runner: `experiments/moriarty-eutxo/tests/financial-patterns.test.mjs`. This runner does not exist as a result of this draft. It accepts a test-name pattern for each EU-FIN identifier. Each case writes `artifacts/financial/<ID>.receipt.json` under a future isolated evidence directory.

Expected runner status: exit 0 only when positive and negative predicates match. Exit 1 identifies a mismatch. Exit 2 identifies an unbound required compiler, ledger, or native dependency. An unbound case cannot count as passed.

## FIN-001: Complete independent result oracle

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/oracle-schema.json` for `EU-FIN-001`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-001' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: status, all ordinary fields, complete financial state, ordered effects, and workRemaining equal the frozen expected envelope.

Negative predicate: the case fails with the first differing JSON path and both values.

Receipt: `artifacts/financial/EU-FIN-001.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-002: Profile and action identity

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/profile-action-cases.json` for `EU-FIN-002`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-002' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: the compiled relation evaluates release and preserves the revision03 ABI.

Negative predicate: the source cross-action case is GUARD_FAILED and T1 profile mismatch is PROFILE_MISMATCH.

Receipt: `artifacts/financial/EU-FIN-002.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-003: Ordered rows and durable state

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/state-retention-cases.json` for `EU-FIN-003`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-003' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: the post appends Lender Cash 100 and deep-equals the source envelope.

Negative predicate: complete-result comparison fails and the compiled transition rejects.

Receipt: `artifacts/financial/EU-FIN-003.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-004: Exact escrow branches

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/escrow-branch-cases.json` for `EU-FIN-004`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-004' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: Escrow Cash becomes 0, selected balance increases 100, allowance consumes 100, and locked becomes 0.

Negative predicate: the source returns its complete GUARD_FAILED envelope and no accepted post or effects.

Receipt: `artifacts/financial/EU-FIN-004.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-005: Residual duty and T1 fence

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/residual-duty-cases.json` for `EU-FIN-005`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-005' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: FundedExpressionPrepared retains outstanding 40 and T1 performs no spend.

Negative predicate: the conformance case fails and the residual-duty native positive remains future work.

Receipt: `artifacts/financial/EU-FIN-005.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-006: Whole interval branch semantics

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/interval-cases.json` for `EU-FIN-006`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-006' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: the boundary follows the selected source guard without a live inclusion claim.

Negative predicate: source GUARD_FAILED or ledger interval-binding rejection is recorded at its actual layer.

Receipt: `artifacts/financial/EU-FIN-006.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-007: Authenticated oracle projection

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/oracle-binding-cases.json` for `EU-FIN-007`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-007' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: feed, publisher, price, and timestamp equal authenticated datum fields and whole-window freshness holds.

Negative predicate: context authentication or freshness fails without publishing a financial transition.

Receipt: `artifacts/financial/EU-FIN-007.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-008: Joint role semantic authority

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/joint-role-cases.json` for `EU-FIN-008`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-008' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: the source role guards and actual ledger witness predicates hold together.

Negative predicate: native authority rejects even if source labels evaluate successfully.

Receipt: `artifacts/financial/EU-FIN-008.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-009: Bilateral atomic settlement

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/bilateral-cases.json` for `EU-FIN-009`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-009' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: both ordered settlement effects match the atomic source oracle and closed changes 0 to 1.

Negative predicate: the corresponding source or native case rejects without a partial business settlement.

Receipt: `artifacts/financial/EU-FIN-009.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-010: Constant product exact arithmetic

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/pool-arithmetic-cases.json` for `EU-FIN-010`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-010' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: output is 19743, remainder is 162290000, reserves become 1010000 and 1980257, and product is 2000059570000.

Negative predicate: arithmetic or full-envelope comparison fails without accepted effects.

Receipt: `artifacts/financial/EU-FIN-010.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-011: Slippage and serialized successor

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/pool-successor-cases.json` for `EU-FIN-011`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-011' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: first output is 19743 and second output is 19356 with reserves 1020000 and 1960901.

Negative predicate: source GUARD_FAILED or actual ledger conflicting-input rejection is recorded.

Receipt: `artifacts/financial/EU-FIN-011.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-012: Order execution and timeout cancellation

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/order-timeout-cases.json` for `EU-FIN-012`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-012' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: phase becomes 1 or 2, payout is 100, allowance becomes 0, and complete post matches the source.

Negative predicate: source GUARD_FAILED publishes no accepted post or effects.

Receipt: `artifacts/financial/EU-FIN-012.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-013: Injective batch satisfaction

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/batch-allocation-cases.json` for `EU-FIN-013`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-013' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: paid increases 100 and both ordered effects allocate once even when recipients share a credential.

Negative predicate: source guard or native allocation rejection prevents both orders claiming one payout.

Receipt: `artifacts/financial/EU-FIN-013.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-014: Funded loan origination

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/origination-cases.json` for `EU-FIN-014`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-014' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: balances become 0 and 110, Loan1 outstanding is 100, liabilityIncurred is 100, and nominal cap is 110.

Negative predicate: the actual first failure is recorded, including INSUFFICIENT_BALANCE for repeat-unfunded.

Receipt: `artifacts/financial/EU-FIN-014.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-015: Period accrual and cumulative cap

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/accrual-cases.json` for `EU-FIN-015`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-015' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: accrued becomes 10, outstanding and incurred become 110, last period becomes 1, and nextAccrualAt becomes 1120.

Negative predicate: PERIOD_NOT_ELIGIBLE, DUPLICATE, or LIABILITY_CAP_EXCEEDED matches the complete source rejection.

Receipt: `artifacts/financial/EU-FIN-015.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-016: Repayment allocation and final settlement

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/repayment-cases.json` for `EU-FIN-016`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-016' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: principal becomes 80, accrued becomes 0, outstanding becomes 80, and later repayment 80 settles outstanding to 0.

Negative predicate: full comparison fails or DUPLICATE rejects with no accepted financial publication.

Receipt: `artifacts/financial/EU-FIN-016.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-017: Allocation policy arithmetic

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/allocation-policy-cases.json` for `EU-FIN-017`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-017' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: AccrualFirst uses dA=min(n,A), PrincipalFirst uses dP=min(n,P), and ProRata uses dP=floor(n*P/(P+A)) with dA=n-dP.

Negative predicate: the actual kernel returns INVARIANT, OVERFLOW, or ALLOCATION_COMPONENT without accepted post.

Receipt: `artifacts/financial/EU-FIN-017.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-018: Protected fee effect semantics

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/fee-cases.json` for `EU-FIN-018`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-018' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: balances become 9 and 101, Borrower allowance becomes 109/spent 1, and debt remains unchanged.

Negative predicate: full comparison fails or source INTENT_NET_GOAL rejects.

Receipt: `artifacts/financial/EU-FIN-018.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-019: Gross inputs and net wallet outcomes

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/wallet-gross-net-cases.json` for `EU-FIN-019`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-019' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: gross cap 10000 rejects and cap 100000 remains subject to every other authority predicate.

Negative predicate: INTENT_DEBIT_CAP or native outcome-policy rejection is retained at its actual layer.

Receipt: `artifacts/financial/EU-FIN-019.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-020: Exact authority envelope identity

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/outcome-identity-cases.json` for `EU-FIN-020`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-020' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: source complete result has net Borrower credit 95 and native acceptance additionally verifies all identity bindings.

Negative predicate: source INTENT_NET_GOAL rejects underfill and native identity rejects substitution despite source shape acceptance.

Receipt: `artifacts/financial/EU-FIN-020.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-021: Durable per-instance nonce and identifiers

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/durable-nonce-cases.json` for `EU-FIN-021`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-021' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: nonce becomes 1 and the identifier remains in subsequent authenticated state.

Negative predicate: native replay rejects while pure old-snapshot source acceptance remains recorded as a source limitation.

Receipt: `artifacts/financial/EU-FIN-021.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-022: Lifetime duties across lifecycle operations

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/lifecycle-duty-cases.json` for `EU-FIN-022`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-022' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: sum of residual liabilities and remaining authority matches its admitted policy without resetting spent work.

Negative predicate: admission rejects unsupported operation or the conservation predicate rejects the proposed transition.

Receipt: `artifacts/financial/EU-FIN-022.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-023: Arithmetic and quantum refusal

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/numeric-quantum-cases.json` for `EU-FIN-023`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-023' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: nominal-to-ledger conversion is exact and no undeclared Cash-to-ADA conversion occurs.

Negative predicate: source INEXACT_CONVERSION or current-layer unsupported admission rejects without integer wrapping.

Receipt: `artifacts/financial/EU-FIN-023.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-024: Work counters and early failure

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/work-cases.json` for `EU-FIN-024`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-024' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: semantic work becomes 975/spent 49/reserve 16 while transport 0/1 becomes 1/0 and ordinary revision becomes 1.

Negative predicate: source WORK_EXHAUSTED at workUsed 48 or INVARIANT is retained instead of replacing it with full debit 49.

Receipt: `artifacts/financial/EU-FIN-024.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-025: Separate network fee and collateral policy

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/collateral-fee-cases.json` for `EU-FIN-025`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-025' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: Cash effects remain exact and final-body collateral exposure fits the authorized cap.

Negative predicate: wallet authorization rejects excess and any actual collateral charge is reported separately from unchanged business state.

Receipt: `artifacts/financial/EU-FIN-025.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## FIN-026: Evidence tier and private boundary

- [ ] Freeze `experiments/moriarty-eutxo/fixtures/financial/tier-reconciliation.json` for `EU-FIN-026`.
- [ ] Add positive and failure cases before implementation.
- [ ] Bind complete source envelopes and independent expected records.
- [ ] Verify this case after its required backend amendment.

Proposed verification command:

```bash
node --test --test-name-pattern='EU-FIN-026' experiments/moriarty-eutxo/tests/financial-patterns.test.mjs
```

Positive predicate: only its exact predicate is credited and all twenty financial family native gaps remain explicit.

Negative predicate: the promotion is refused and private, PCD, Complete, and mandatory Midnight release gates remain unverified.

Receipt: `artifacts/financial/EU-FIN-026.receipt.json` binds fixture, expected, actual, compiler, and context digests.

## Prospective implementation paths

- [ ] Define proposed `FinancialCaseOracle/1` in `experiments/moriarty-eutxo/fixtures/financial/oracle-schema.json`.
- [ ] Add independent expected fixtures in `experiments/moriarty-eutxo/fixtures/financial/expected/`.
- [ ] Define versioned finance admission in `experiments/moriarty-eutxo/src/admit-financial-v1.mjs`.
- [ ] Define full kernel lowering in `experiments/moriarty-eutxo/src/lower-financial-v1.mjs`.
- [ ] Consume ledger `EffectAllocationReceipt/1` in future compiled-context cases.
- [ ] Consume final-body `FinancialWalletProjection/1` in future authority cases.
- [ ] Record any compiler or ledger interface amendment before implementation.
- [ ] Preserve original /5 and /6 behavior in source regressions.

These are proposed files and interfaces. No exported API or backend functionality is claimed. Full finance changes require separate scope approval. Current T1 implementation remains Grok-owned.

## Prospective complete-envelope test snippet

The actual /5 factory exists. The fixture record and compiled runner below are proposed test inputs.

```javascript
import assert from 'node:assert/strict';
import { createFinancialAgreementSourceV5 } from '../../moriarty-language/src/successor/financial-agreement-source-v5.ts';
const api = createFinancialAgreementSourceV5();
const actual = api.evaluate(caseRecord.sourceBytes, caseRecord.actionSelector,
  caseRecord.ordinaryCanonicalJSON, caseRecord.financialCanonicalJSON);
assert.deepStrictEqual(actual, caseRecord.independentExpectedEnvelope);
// Compile/context checks additionally compare complete source-success envelopes.
// Native-negative and admission-negative statuses remain different fields.
```

The independent envelope is constructed before evaluator invocation. Tests must not read observed outputs as their expected oracle. Packet19 RESULTS.json may serve as the independent actual evaluator record for a separately authored backend. It does not replace requirements-derived independent economic expectations.

## Prospective packet and source reproduction

These commands are specified-only here. Some existing verifiers write receipts beside their inputs. Use isolated copies before execution.

```bash
node --experimental-strip-types /home/charl/moriarty-eutxo-validation-2026-10-12/t1-independent-oracles-19/generate-source-oracles.mjs verify
node --experimental-strip-types /home/charl/moriarty-eutxo-validation-2026-10-12/designers/02-escrow-time/verify.mjs
node --experimental-strip-types /home/charl/moriarty-eutxo-validation-2026-10-12/designers/03-swaps-batch/verify.mjs
node --experimental-strip-types /home/charl/moriarty-eutxo-validation-2026-10-12/designers/04-finance/verify.mjs
node --experimental-strip-types /home/charl/moriarty-eutxo-validation-2026-10-12/designers/05-authority-private/verify.mjs
```

Expected source exits: 0 for intact frozen oracles. Expected counterexample exit: 1 after an isolated expected payout 100→99 mutation. Packet write/bind overwrite refusal remains exit 3. Keep all original observed and failed records.

## Native and private dependencies

- [ ] Require compiler exact-source UPLC receipts before native promotion.
- [ ] Require ledger authenticated head, signer, reference, and allocation receipts.
- [ ] Require native POSIX parity before calibrated sibling generation.
- [ ] Require native final-body and independent ledger readback receipts.
- [ ] Keep concurrent and sequential conflicts as different test cases.
- [ ] Require conformance/private whole-relation evidence for private or PCD claims.
- [ ] Preserve mandatory Midnight release gates after public Cardano success.

Native commands belong to the native package. No node, wallet, key, or submission command is executed here.

## Draft structure verification

- [ ] Run only this package's strict OpenSpec validation.

```bash
openspec validate eutxo-financial-pattern-validation --strict --json --no-interactive
```

Expected status: exit 0 with no structural issues. Structural success is not semantic, source-reproduction, UPLC, native, or private approval.
