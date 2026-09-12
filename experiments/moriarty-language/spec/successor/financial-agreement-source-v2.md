# Financial agreement source /2

This profile declares one bounded repayment contract with multiple named
actions in a single `.mori` file. It is a distinct
`moriarty-financial-agreement-source/2` entry. The /1 profile keeps its
exactly-one-action contract. Older expression and financial expression
profiles keep their exact contracts and still require `--schema`.

```javascript
import { createFinancialAgreementSourceV2 } from '../../src/successor/financial-agreement-source-v2.ts';
const language = createFinancialAgreementSourceV2();
language.check(source);
language.elaborate(source);
language.evaluate(source, actionName, snapshotCanonicalJSON, repaymentStateJSON);
```

From the repository root:

```sh
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/2 experiments/moriarty-language/spec/successor/examples/multiple-action-payment.mori
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-agreement-source/2 experiments/moriarty-language/spec/successor/examples/multiple-action-payment.mori
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-agreement-source/2 --action repay --snapshots experiments/moriarty-language/spec/successor/examples/multiple-action-payment.snapshots.json --repayment-state experiments/moriarty-language/spec/successor/examples/expression-funded-payment.state.json experiments/moriarty-language/spec/successor/examples/multiple-action-payment.mori
```

The new profile rejects `--schema`. Check and format reject `--action`.
Simulate requires `--action` once. Selection is mandatory even when the
source contains one action. There is no implicit first-action fallback.
Snapshots supply runtime `Pre`, `Args`, `Obs` and `workInitial` for the
selected action only. State fields have no initializer.

## Declarations

The [grammar](financial-agreement-source-v2-grammar.ebnf) keeps the /1
declaration spellings and admits more than one `action`. Zero actions
reject `SOURCE_ACTION_COUNT`. Duplicate action names, or an action name
that collides with another top-level declaration, reject at the offending
declaration. Unit and asset keep separate namespaces. Collect shared
declarations before resolving so record types may refer forward; cycles
and unknown references reject.

Elaboration compiles shared units, assets, records, ordinary state and
protected operations once, then derives an independent `schema.args` and
checked Core for every action in source declaration order. Each action
has its own parameters and locals. The same parameter or local name may
occur in another action, including with a different type. References to
another action's bindings reject. Returned schema and Core are
inspectable output. Later evaluate compiles from original source and never
consumes those artifacts.

The public API accepts primitive source, action-name, snapshot and
repayment-state strings only. A missing, non-string, empty or non-identifier
selector rejects `SOURCE_ACTION_NAME`. A well-formed identifier that is
absent from the action set rejects `SOURCE_ACTION_UNKNOWN`. Those selector
errors use a synthetic span, empty `nodePath` and `workUsed` `0`. Static
errors in any action reject before selector or runtime-input handling.

## Protected operations

Check, elaborate and evaluate require exactly `Transfer` and `Repay` with
the same funded field and unit rules as /1. Runtime retains settlement-unit
and nominal-unit witnesses, nonnegative signed128 range, identifier and
relationship checks and funding rules. One ordered kernel invocation
follows a successful selected expression evaluation.

## Results and work

Successful evaluate returns the same `FundedExpressionPrepared` envelope as
/1: ordinary `post`, complete `financialPost`, kernel `effects` and
`workRemaining`. Only the selected action runs. Work is that action's
actual expression reductions E plus its kernel action count N. Extra
unselected actions add no runtime debit. `closureReserve` stays separate.

CLI success exits 0 with `judgmentResult` `SourceSimulated`, `sourceProfile`,
`action`, ordinary `pre`, `financialPre`, `initialWork` and `result`. Only
/2 adds `action`. Semantic rejection exits 1 with JSON on stderr, empty
stdout and no tentative result. Source failures keep original UTF-8 byte
spans.

The [example](examples/multiple-action-payment.mori) declares `repay` and
`repay_installment`. Default `repay` of 30 against due 100 leaves principal
and outstanding 70. A continuation `repay_installment` of 20, using the
first `post`, complete `financialPost`, remaining work and fresh identifiers,
leaves 50. This slice does not execute K, construct a proof, or submit a
public transaction.
