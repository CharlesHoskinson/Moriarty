# Funded financial expression source

Local adapter. It evaluates `moriarty-financial-expression-source/1` and, when
every step succeeds, runs the retained funded-repayment kernel once. It produces
a prepared local candidate. It does not sign, prove, authenticate, or settle on
Preview. Full SP02/SP03, K correspondence, and ledger acceptance remain open.

```javascript
import { createFundedFinancialExpressionSourceV1 } from '../../src/successor/funded-expression-source-v1.ts';
const language = createFundedFinancialExpressionSourceV1(schemaCanonicalJSON);
language.evaluate(source, snapshotCanonicalJSON, repaymentStateJSON);
```

From the repository root:

```sh
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-expression-source/1 --schema experiments/moriarty-language/spec/successor/examples/expression-funded-payment.schema.json --snapshots experiments/moriarty-language/spec/successor/examples/expression-funded-payment.snapshots.json --repayment-state experiments/moriarty-language/spec/successor/examples/expression-funded-payment.state.json experiments/moriarty-language/spec/successor/examples/expression-funded-payment.mori
```

`--repayment-state` is legal only for this profile and `simulate`. The flag sits
immediately before SOURCE. Other commands and the original expression profile
reject the option. Pure simulate, check, and format keep their previous forms.

## Schema and operations

The trusted schema is the current financial expression schema. This adapter
rejects any field with `writeClass` `financial`. Pre and post are ordinary
application fields. Financial state is the separate complete repayment
projection. Source `ensures` clauses see ordinary staged post, not final
financial postconditions.

The schema must bind exactly two operations, `Transfer` and `Repay`:

| Operation | Fields |
| --- | --- |
| Transfer | `id`, `from`, `to`, `settlementAsset`: Text; `transferAmount`: Amount\<asset\> |
| Repay | `allocationId`, `transferId`, `obligationId`, `payer`: Text; `nominalAmount`: Quantity\<Units\<denomination,1\>,0\> |

The financial source reserves the identifier `amount` as a generic constructor,
so the Transfer Amount field cannot be named `amount`. The adapter maps
`transferAmount` to the kernel Transfer `amount`. `settlementAsset` maps to the
kernel `asset`. Each emitted `settlementAsset` value must equal the Amount type
index. Each Repay quantity unit must equal the selected obligation denomination.

Nominal Quantity values must be nonnegative and inside the signed-128 range.
Text identifiers still face kernel identifier and relationship checks. Unknown
operations, missing or malformed bindings, and empty descriptor batches reject
before the kernel runs.

## API result

`evaluate` accepts source text and owned parsed JSON only. It reuses
`createFinancialExpressionSourceV1` and `prepareRepayment`. It does not add a
second transition engine.

Success:

```json
{"status":"FundedExpressionPrepared","post":{},"financialPost":{},"effects":[],"workRemaining":"0"}
```

`post` is the ordinary staged result. `financialPost` is the complete kernel
projection. `effects` are the ordered kernel effects. `workRemaining` equals
`financialPost.work.remaining`.

Rejection contains diagnostics only. It never contains candidate `post`,
`financialPost`, `descriptors`, or `effects`. Expression rejections keep their
source records. Kernel rejections keep `code` and `actionIndex`. Adapter
rejections are `{ "status": "Rejected", "code": "..." }`.

CLI success exits 0 and writes one JSON line with `judgmentResult`
`SourceSimulated`, `sourceProfile`, initial ordinary `pre`, initial
`financialPre`, `initialWork`, and the combined `result`. Semantic rejection
exits 1 with JSON on stderr and empty stdout. CLI misuse or I/O failure exits 2.
The repayment-state file is a bounded UTF-8 regular-file JSON object, the bare
`RepaymentState`. The read cap is 65,536 UTF-8 bytes.

## Work

Snapshot `workInitial` must equal financial `work.remaining`. The expression
evaluator uses that remaining value and keeps its 65,536 ceiling. The kernel
`closureReserve` is separate from remaining; this adapter does not subtract or
spend it.

For actual expression reductions `E` and `N` emitted kernel actions:

```
remainingAfter = remainingBefore - E - N
spentAfter = spentBefore + E + N
closureReserveAfter = closureReserveBefore
```

The adapter passes a fresh state with expression work already debited into one
complete kernel call. The kernel then charges `N` and validates the full
projection before publication. Empty batches do not call the kernel. The static
expression bound is not charged. Neither stage is debited twice. Carried spent
and residual duties stay in the supplied projection.

## Bounds and scope

Source, schema, and repayment-state each stay within 65,536 UTF-8 bytes.
Snapshots stay within 2,000,000 bytes. Descriptor count and kernel collections
keep the existing 128 cap. Quantity does not broaden the kernel UInt128 type.

The [example](examples/expression-funded-payment.mori) adds two Quantity arguments,
rejects a negative sum, takes `magnitude` as cash, updates ordinary `paid`, and
emits that same nominal to Repay. Default arguments 10+20 pay 30 against due 100:
payer cash 70, lender cash 30, principal 70, allowance remaining 70 / spent 30.
A second CLI call with the same source, arguments 10+10, and the first
`financialPost` as state pays another 20 without editing the `.mori` file.

This slice does not add a profile, compiler, or financial schema language. It
does not execute K, construct a proof, or submit a public transaction. The
later [source-defined agreement profile](financial-agreement-source.md) declares
the same schema in source and keeps this adapter as the funded baseline.
