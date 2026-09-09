# Funded repayment projection kernel

This module is a bounded executable financial **projection** kernel for SP03.1 and RP01 CM04/CM09.

It is not a full successor Core/K implementation, not an admission checker, not a signing encoding, and not a ledger.
`prepareRepayment` produces **Prepared candidates only**. It never produces acceptance, signatures, proofs, or network messages.

This kernel implements the funded Transfer/Repay transition in an isolated local projection. It is not a full successor semantic freeze, not a full SP03 implementation, and does not replace the retained composition model. Surface integration, the K toolchain, and native campaign work do not belong here.

Product module: `experiments/moriarty-language/src/successor/repayment.ts`
Tests: `experiments/moriarty-language/tests/successor-repayment.test.mjs`
Example: `experiments/moriarty-language/spec/successor/examples/funded-repayment.json`

## Why this kernel exists

The retained composition model `f6f5ead61868a4b693bc4bf710123120fa00fda9857b37ca3911a5a6bc6e43fa` reduces duty 50 to 30 after every Transfer is removed, and leaves Payer cash 50 / Lender cash 0. That is the wrong financial transition: debt moves without the settlement-asset movement that funded it.

The retained model remains failing. This projection implements a funded transition only inside this kernel: a `Repay` action may discharge nominal debt only from a `Transfer` that already executed **in this same candidate**. Removing the Transfer from the action list removes the funding; the Repay then rejects. Fixture labels and checker flags must not paper over that rule.

## Later obligations (out of scope)

These remain later work. This kernel does not discharge them:

- SP02 elaboration of richer debt / wire records
- SP03 K executable semantics and a full successor freeze
- full RP01
- signing, signatures, and authorization proofs
- authenticated ledger state and admission
- adapters from atomic / source / signing schemas (those adapters must preserve richer fields separately)
- host/network computational fees
- surface integration, native campaign, and K toolchain wiring

Input financial state is a **supplied local projection**. It is not an authenticated ledger state, not an accepted successor record, and not the full proposed Debt wire record. There is no implicit adapter from atomic/source/signing schemas. The kernel preserves the complete projection it is given.

## Citations (source paths, no new external claims)

- `openspec/sprints/sp03-executable-bounded-semantics-in-k.md` — due100 / pay30
- `openspec/sprints/report-lessons.json` — TX02
- retained round-08 composition `gpt6-mechanism-result-review.md` — CM04 / CM09
- retained successor `semantic-contract.md` section F.2 — AccrualFirst, PrincipalFirst, ProRata, and conversion rounding names. This kernel is a provisional local projection and makes no full F.2 correspondence claim

## Public API

```ts
prepareRepayment(source: string): RepaymentResult
REPAYMENT_VERSION = 'moriarty-funded-repayment/0'
REPAYMENT_BOUNDS = {
  sourceUtf8Bytes: 65536,
  collectionCapacity: 128,
  identifierCharacters: 64,
  maxScale: 18,
  uint128Max: '340282366920938463463374607431768211455',
}
```

`REPAYMENT_BOUNDS` is immutable.

Exported closed types:

- `RepaymentInput` — `{ schemaVersion, state, actions }`
- `RepaymentState` — `{ balances, allowances, obligations, usedTransferIds, usedAllocationIds, work }`
- `Balance` — `{ party, asset, amount }`
- `Allowance` — `{ party, asset, remaining, spent }`
- `Work` — `{ remaining, spent, closureReserve }`
- `Obligation` — `{ id, debtor, creditor, denomination, settlementAsset, principal, accrued, outstanding, allocationRule, conversion, status }`
- `Conversion` — `{ mantissa, scale, rounding }`
- `TransferAction` — `{ kind: 'Transfer', id, from, to, asset, amount }`
- `RepayAction` — `{ kind: 'Repay', allocationId, transferId, obligationId, payer, nominalAmount }`
- `RepaymentEffect` — `{ kind: 'Repayment', allocationId, transferId, obligationId, payer, creditor, denomination, settlementAsset, nominalAmount, settlementAmount, principalDischarged, accruedDischarged, remainingOutstanding }`
- `PreparedRepayment` — `{ status: 'Prepared', schemaVersion, post, effects }`
- `RejectedRepayment` — `{ status: 'Rejected', code, actionIndex }`
- `RepaymentResult` — `PreparedRepayment | RejectedRepayment`

The public function accepts a primitive string only. There is no object/accessor overload that bypasses source admission. No field, flag, constructor, or variant is silently ignored.

## Numeric representation

All amounts, counters, and scales are canonical unsigned **decimal strings**, not JavaScript numbers. Arithmetic uses `BigInt`. Every sum, product, and intermediate must fit UInt128 **before** division, including pro-rata `N*P` and conversion `nominal*mantissa`. There is no automatic wide arithmetic. Exposed output contains decimal strings, never `BigInt`.

Canonical unsigned decimal text:

- digits only
- no sign, decimal point, exponent, or leading zero except the value `"0"`
- numeric value in `0 ..= uint128Max`

Identifiers match `/^[A-Za-z][A-Za-z0-9_]{0,63}$/` with a genuine full-string end check (a valid prefix plus junk is not an identifier).

## Source format (not a signing encoding)

Admission requires:

1. `typeof source === 'string'` before any coercion. Hostile `toString` / `valueOf` / `toPrimitive` traps must not run.
2. UTF-16 length `<= 65536` before a bounded UTF-8 encode. Reject raw lone surrogates.
3. UTF-8 byte length `<= 65536`.
4. JSON parse with structured rejection.
5. `JSON.stringify(parsed) === source`. Duplicate keys, whitespace, and nonminimal escape or number forms reject.

Do **not** call this roundtrip a canonical signing encoding. Object key order is not sorted. Exact JSON output ordering of a Prepared result is not an acceptance hash. Catch `JSON.stringify` depth failures as `INPUT_ENCODING`.

Validation does not normalize records or approximate quantities. Return a `RejectedRepayment`. Do not throw and do not crash.

## Input closed shape

```
RepaymentInput {
  schemaVersion: REPAYMENT_VERSION,
  state: RepaymentState,
  actions: Action[]          // nonempty, length <= 128
}
```

Collections in `state` and `actions` each have capacity 128. Post-collection sizes after tombstone or balance appends must also stay `<= 128`.

Uniqueness (namespaces are separate; no cross-kind uniqueness):

- balances unique by `(party, asset)`
- allowances unique by `(party, asset)`
- obligations unique by `id`
- `usedTransferIds` unique
- `usedAllocationIds` unique

Invariants, checked before any action:

- `Work.remaining + spent + closureReserve` fits UInt128
- `Allowance.remaining + spent` fits UInt128
- `Obligation.principal + accrued == outstanding` and the sum fits UInt128
- `Outstanding` iff `outstanding > 0`; `Settled` iff `outstanding == 0`
- conversion `mantissa > 0`; `scale` in `0..=18`
- `allocationRule` is exactly `AccrualFirst` | `PrincipalFirst` | `ProRata`
- conversion `rounding` is exactly `none` | `floor` | `ceil`
- `status` is exactly `Outstanding` | `Settled`

Unknown keys, including `__proto__`, unknown action kinds, unknown nested fields, unknown profiles, JSON numeric amounts, and non-canonical decimals reject. Validate each entire closed record before executing anything.

## Transition semantics

Run the ordered actions on a fresh tentative copy. Preserve array order and every unchanged field. There are no externally visible intermediate states.

On the first failure return **only** `{ status: 'Rejected', code, actionIndex }`.

- Invalid input / schema / bounds: `actionIndex` is `null`.
- Action rule failure: `actionIndex` is the 0-based index of that action.
- No `post`, no `effects`, no mutation of the source, no success fallback.

### Work

Before actions, require `work.remaining >= actions.length` and `work.spent + actions.length` fits UInt128. Cost is exactly 1 per action, derived from the actual action list, not from a supplied certificate. `closureReserve` is not ordinary work: remaining 0 with a large reserve still rejects a one-action candidate.

On success, decrement `remaining` and increment `spent` by that count. Leave `closureReserve` unchanged. Host/network computational fees are outside this local rule.

### Transfer

- `amount > 0`
- `from != to`
- `id` absent from `usedTransferIds` and from earlier transfers in this step
- sender balance must exist and cover `amount` (no implicit zero sender row)
- an exact `(sender, asset)` allowance must exist and cover `amount`
- debit sender; credit receiver (append a zero-based receiver balance if absent, within capacity)
- decrement `allowance.remaining` and add `allowance.spent` by the gross amount
- a refund transfer never restores an earlier sender's allowance. Creditor refunds are separate Transfer actions. This module does not assert net-goal satisfaction, fees, or ledger acceptance
- check every addition for UInt128 overflow
- append the transfer id tombstone
- emit a Transfer effect that copies the complete `TransferAction`
- store an **internal** allocation remainder for this step's executed transfer (not present in output)

No prior, future, or missing transfer can fund a Repay. A Transfer-only candidate is legal: cash moves, debt does not.

### Repay

- `nominalAmount > 0` and `nominalAmount <= obligation.outstanding`
- target obligation exists and is `Outstanding`
- `allocationId` unused across complete `usedAllocationIds` and earlier Repays in this step
- referenced transfer already executed **in this step**
- that transfer has `from == payer`, `to == obligation.creditor`, `asset == obligation.settlementAsset`
- payer may be a third party with its own exact allowance; never require `debtor == payer`. Retained F.2 text is debtor-funded; this local rule is not that text
- no token debt is created from a transfer
- settlement asset may differ from nominal denomination and is never coerced

`settlementAmount = round(nominalAmount * conversion.mantissa / 10^scale)` using the registered mode, per allocation. Split repayments can differ from aggregate rounding of the summed nominal:

- compute `product = nominalAmount * mantissa` (must fit UInt128)
- `Q = 10^scale`
- `quotient = product / Q`, `remainder = product % Q`
- `none`: require `remainder == 0`; settlement is `quotient`
- `floor`: settlement is `quotient`
- `ceil`: settlement is `quotient` if `remainder == 0`, else `quotient + 1` (the increment must fit UInt128)

Zero settlement for a positive nominal discharge rejects. Dust needs a separate explicit future resolution; this kernel does not erase debt for free.

Require `settlementAmount <=` this transfer's remaining unallocated amount, then subtract it. Across all obligations/allocations, aggregate allocated `<=` actual transfer amount. Allocation ids persist after settlement and cannot be reused for a different obligation.

Allocate nominal `N` against principal `P` and accrued `A`:

- AccrualFirst: `dA = min(N, A)`, `dP = N - dA`
- PrincipalFirst: `dP = min(N, P)`, `dA = N - dP`
- ProRata: `dP = floor(N * P / (P + A))` with `N * P` and `P + A` fitting UInt128 before division; `dA = N - dP`

Require each component `<=` the existing component. Update `P`, `A`, and `outstanding` exactly. Settled iff both are zero. Keep identity and all metadata/tombstone; never remove the obligation.

Emit:

```
{
  kind: 'Repayment',
  allocationId, transferId, obligationId, payer, creditor,
  denomination, settlementAsset,
  nominalAmount, settlementAmount,
  principalDischarged, accruedDischarged, remainingOutstanding
}
```

Every quantity is decimal text. Append the allocation id tombstone. No effect-dependent label determines financial success.

### Success

Return **only**:

```
{ status: 'Prepared', schemaVersion: REPAYMENT_VERSION, post: RepaymentState, effects: Effect[] }
```

Effects are one per action, in action order. Preserve untouched debt, balance, allowance, and history records. No signature, proof, or true-authorized flags.

## Stable emitted error codes

`status` is `Rejected` or `Prepared`. `code` is exactly one of the names below. The same source produces the same code. Do not use fixture names or message text as codes.

| code | actionIndex | when |
| --- | --- | --- |
| `INPUT_NOT_STRING` | `null` | source is not a primitive string |
| `INPUT_UTF16_LENGTH` | `null` | UTF-16 length exceeds `sourceUtf8Bytes` |
| `INPUT_LONE_SURROGATE` | `null` | raw lone surrogate |
| `INPUT_UTF8_LENGTH` | `null` | UTF-8 byte length exceeds `sourceUtf8Bytes` |
| `INPUT_JSON` | `null` | `JSON.parse` failure |
| `INPUT_ENCODING` | `null` | `JSON.stringify` depth or type failure |
| `INPUT_COMPACT` | `null` | `JSON.stringify(parsed) !== source` |
| `SCHEMA` | `null` on admission; action index if an action is missing at apply | closed-shape, missing field, unknown variant, empty actions, wrong `schemaVersion`, non-record |
| `UNKNOWN_FIELD` | `null` | extra key, including `__proto__` |
| `UNKNOWN_ACTION` | `null` on admission; action index at apply | unknown action `kind` |
| `INVALID_IDENTIFIER` | `null` | identifier syntax or length |
| `INVALID_AMOUNT` | `null` | non-canonical decimal or out of UInt128 text range |
| `CAPACITY` | `null` on admission; action index at apply | collection length > 128, or receiver/tombstone append exceeds 128 |
| `DUPLICATE` | `null` on admission; action index at apply | duplicate pair, obligation id, used-id, transfer id, or allocation id |
| `INVARIANT` | `null` on admission; action index at apply | mantissa 0, scale > 18, remaining+spent(+reserve) overflow, principal+accrued ≠ outstanding, status mismatch, or ProRata `P+A == 0` |
| `INSUFFICIENT_WORK` | `null` | `work.remaining < actions.length` (closure reserve is not ordinary work) |
| `OVERFLOW` | `null` if work spent overflows before actions; else action index | UInt128 overflow on work spent, credit, allowance spent, conversion product, ceil increment, or pro-rata `N*P` / `P+A` |
| `ZERO_AMOUNT` | action index | Transfer or Repay amount is 0 |
| `SELF_TRANSFER` | action index | `from == to` |
| `MISSING_BALANCE` | action index | sender balance row absent |
| `INSUFFICIENT_BALANCE` | action index | sender cannot cover |
| `MISSING_ALLOWANCE` | action index | no exact `(from, asset)` allowance |
| `INSUFFICIENT_ALLOWANCE` | action index | allowance remaining cannot cover |
| `MISSING_OBLIGATION` | action index | obligation id not found |
| `NOT_OUTSTANDING` | action index | obligation not `Outstanding` or outstanding is 0 |
| `EXCEEDS_OUTSTANDING` | action index | `nominalAmount > outstanding` |
| `TRANSFER_NOT_IN_STEP` | action index | referenced transfer did not execute in this candidate |
| `TRANSFER_MISMATCH` | action index | transfer `from`/`to`/`asset` does not match payer/creditor/settlementAsset |
| `INEXACT_CONVERSION` | action index | `rounding` is `none` and remainder ≠ 0 |
| `DUST` | action index | positive nominal converted to settlement 0 |
| `INSUFFICIENT_UNALLOCATED` | action index | settlement exceeds this transfer's remaining unallocated amount |
| `ALLOCATION_COMPONENT` | action index | discharged principal or accrued exceeds the existing component |

## Example

The file `spec/successor/examples/funded-repayment.json` is exact compact JSON with no trailing newline. It is the due100 / pay30 projection: Payer cash 100, cap 100, work remaining 100 / spent 0 / closureReserve 16, Transfer 30 then Repay 30.

Prepared post: Payer 70 / Lender 30, principal 70 / outstanding 70, cap remaining 70 / spent 30, work remaining 98 / spent 2 / closureReserve 16 unchanged.

```js
import { readFileSync } from 'node:fs';
import { prepareRepayment } from '../src/successor/repayment.ts';

const source = readFileSync(
  'experiments/moriarty-language/spec/successor/examples/funded-repayment.json',
  'utf8',
);
const result = prepareRepayment(source);
```

Independent reference values used by tests (not computed by a copied kernel):

- AccrualFirst P100/A10 N7 → P100/A3 outstanding 103
- PrincipalFirst P100/A10 N7 → P93/A10 outstanding 103
- ProRata P100/A10 N7 → `floor(7*100/110) = 6` so P94/A9 outstanding 103
- conversion mantissa 2 scale 0 N30 → settlement 60
- conversion mantissa 3 scale 1 N4 → product 12, quotient 1 remainder 2 → floor 1, ceil 2, none rejects
- conversion mantissa 3 scale 1 N10 → product 30, remainder 0 → settlement 3 in all three modes
- conversion mantissa 1 scale 1 N1 → floor settlement 0 (dust reject); none inexact reject; ceil settlement 1
