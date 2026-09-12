# Financial agreement source /5

This profile declares one bounded lifecycle contract with multiple named
actions, four protected operations Transfer, Repay, Originate and Accrue,
six generic financial PRE reads, and six Ensure-only financial POST reads.
It is a distinct `moriarty-financial-agreement-source/5` entry. The Core
contract is `moriarty-financial-expression-contract/4` because the previous
constructor set stays closed. Profiles /1–/4 keep their exact APIs,
reserved names and evaluation order. The kernel is
`moriarty-financial-lifecycle/1` with explicit state
`moriarty-financial-lifecycle-state/1`.

```javascript
import { createFinancialAgreementSourceV5 } from '../../src/successor/financial-agreement-source-v5.ts';
const language = createFinancialAgreementSourceV5();
language.check(source);
language.elaborate(source);
language.evaluate(source, actionName, snapshotCanonicalJSON, financialPreStateJSON);
```

From the repository root:

```sh
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/5 experiments/moriarty-language/spec/successor/examples/financial-lifecycle-payment.mori
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-agreement-source/5 experiments/moriarty-language/spec/successor/examples/financial-lifecycle-payment.mori
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-agreement-source/5 --action originate --snapshots experiments/moriarty-language/spec/successor/examples/financial-lifecycle-payment.snapshots.json --repayment-state experiments/moriarty-language/spec/successor/examples/financial-lifecycle-payment.state.json experiments/moriarty-language/spec/successor/examples/financial-lifecycle-payment.mori
npm --prefix experiments/moriarty-language run financial-lifecycle-demo
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/5 experiments/moriarty-language/spec/successor/examples/loan-lifecycle.mori
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-agreement-source/5 experiments/moriarty-language/spec/successor/examples/loan-lifecycle.mori
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-agreement-source/5 --action originate --snapshots experiments/moriarty-language/spec/successor/examples/loan-lifecycle.snapshots.json --repayment-state experiments/moriarty-language/spec/successor/examples/loan-lifecycle.state.json experiments/moriarty-language/spec/successor/examples/loan-lifecycle.mori
npm --prefix experiments/moriarty-language run loan-lifecycle-demo
```

The new profile rejects `--schema`. Check and format reject `--action`.
Simulate requires `--action` once. Selection is mandatory even when the
source contains one action. There is no implicit first-action fallback.
Snapshots supply runtime `Pre`, `Args`, `Obs` and `workInitial` for the
selected action only. State fields have no initializer. The repayment-state
file is a local validated lifecycle projection, not authenticated ledger state.

## Declarations

The [grammar](financial-agreement-source-v5-grammar.ebnf) keeps the /4
declaration spellings. Protected Originate and Accrue use nested
`record<Row>{...}` constructors and `u64(...)` time/index literals.
Zero actions reject `SOURCE_ACTION_COUNT`. Quantity and Amount remain
distinct. Source U and settlement A are the same name.

## Evaluation and publication

`/5` type-checks every action, selects one action, admits owned lifecycle
input, validates snapshots, executes the prefix once, prepares the lifecycle
kernel once, then executes the entire `ensures` suffix once. Financial PRE
never changes within one source action. Same-call Originate followed by
Accrue is permitted as explicit ordered descriptors. Accrue target
denomination is checked against the agreement binding even though Accrue
has no nominal operand.

Work is prefix reductions plus kernel actions plus suffix reductions.
Failed suffix after a tentative financial event publishes no state, effects
or IDs. Canonical compact financial post must be <=65536 bytes and fully
re-admissible after kernel debit and after suffix debit.

This slice does not execute K, construct a proof, or submit a public
transaction.
