# Financial agreement source /1

This profile declares the bounded repayment contract in one `.mori` file and
runs it without an external schema file. It is a distinct
`moriarty-financial-agreement-source/1` entry. Older expression and financial
expression profiles keep their exact contracts and still require `--schema`.

```javascript
import { createFinancialAgreementSourceV1 } from '../../src/successor/financial-agreement-source-v1.ts';
const language = createFinancialAgreementSourceV1();
language.check(source);
language.elaborate(source);
language.evaluate(source, snapshotCanonicalJSON, repaymentStateJSON);
```

From the repository root:

```sh
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/1 experiments/moriarty-language/spec/successor/examples/source-defined-payment.mori
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-agreement-source/1 experiments/moriarty-language/spec/successor/examples/source-defined-payment.mori
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-agreement-source/1 --snapshots experiments/moriarty-language/spec/successor/examples/expression-funded-payment.snapshots.json --repayment-state experiments/moriarty-language/spec/successor/examples/expression-funded-payment.state.json experiments/moriarty-language/spec/successor/examples/source-defined-payment.mori
```

The new profile rejects `--schema`. Simulate requires both snapshots and
repayment state. Check and format require neither. Snapshots supply runtime
`Pre`, `Args`, `Obs` and `workInitial`. State fields have no initializer.

## Declarations

The [grammar](financial-agreement-source-grammar.ebnf) adds profile-gated
`record` and `operation` declaration spellings and uninitialized `state`
fields. Those spellings are not global keywords. Unit and asset use separate
namespaces, so `Cash` may occur once in each. Same-kind duplicates reject.
Every other top-level name must not collide with another top-level name,
including unit and asset names. Record fields are record-local.

Enabled forms are unit, party, asset, record, ordinary state, operation and
exactly one action. Enum, variant, observation, vault, const and initialized
state reject. Collect declarations before resolving so record types may refer
forward; cycles and unknown references reject.

Elaboration compiles those declarations into the existing checked financial
schema. Every state field has `writeClass` `ordinary`. Source field names that
look like financial projection fields remain ordinary bookkeeping. The public
API accepts primitive source, snapshot and repayment-state strings only.
Returned schema and Core are inspectable output, never evaluation input.

## Protected operations

Check, elaborate and evaluate require exactly `Transfer` and `Repay`. Record
type names may be chosen, but their fields and value types must satisfy the
existing funded operation binding:

| Operation | Fields |
| --- | --- |
| Transfer | `id`, `from`, `to`, `settlementAsset`: Text; `transferAmount`: Amount\<declared asset\> |
| Repay | `allocationId`, `transferId`, `obligationId`, `payer`: Text; `nominalAmount`: Quantity\<Units\<declared denomination,1\>,0\> |

Missing or extra operations, changed fields, scalar substitutions, quantity
powers or scales, unknown record references, mixed Transfer/Repay units and
reclassification reject. One binding validator is reused. Runtime retains
settlement-unit and nominal-unit witnesses, nonnegative signed128 range,
identifier/relationship checks and funding rules. One ordered kernel
invocation follows a successful expression evaluation.

## Results and work

Successful evaluate returns the same `FundedExpressionPrepared` envelope as
the schema-based funded adapter: ordinary `post`, complete `financialPost`,
kernel `effects` and `workRemaining`. Source declarations add no expression
work. Work remains actual expression reductions E plus action count N.
`closureReserve` stays separate.

CLI success exits 0 with `judgmentResult` `SourceSimulated`, `sourceProfile`,
ordinary `pre`, `financialPre`, `initialWork` and `result`. Semantic rejection
exits 1 with JSON on stderr, empty stdout and no tentative result. Source
failures keep original UTF-8 byte spans. Check and format do not authenticate
snapshots or establish financial authority.

The [example](examples/source-defined-payment.mori) is the funded payment
action with its schema declared in source. It reuses the existing snapshot
and repayment-state fixtures. Default 10+20 pays 30 against due 100. This
slice does not execute K, construct a proof, or submit a public transaction.
