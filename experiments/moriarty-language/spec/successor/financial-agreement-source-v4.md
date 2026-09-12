# Financial agreement source /4

This profile declares one bounded repayment contract with multiple named
actions, six generic financial PRE reads, and six Ensure-only financial
POST reads. It is a distinct `moriarty-financial-agreement-source/4`
entry. The Core contract is `moriarty-financial-expression-contract/3`
because the previous constructor set stays closed. Profiles /1, /2 and /3
keep their exact APIs, reserved names and evaluation order.

```javascript
import { createFinancialAgreementSourceV4 } from '../../src/successor/financial-agreement-source-v4.ts';
const language = createFinancialAgreementSourceV4();
language.check(source);
language.elaborate(source);
language.evaluate(source, actionName, snapshotCanonicalJSON, repaymentStateJSON);
```

From the repository root:

```sh
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/4 experiments/moriarty-language/spec/successor/examples/financial-postconditions-payment.mori
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-agreement-source/4 experiments/moriarty-language/spec/successor/examples/financial-postconditions-payment.mori
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-agreement-source/4 --action repay --snapshots experiments/moriarty-language/spec/successor/examples/financial-state-payment.snapshots.json --repayment-state experiments/moriarty-language/spec/successor/examples/financial-state-payment.state.json experiments/moriarty-language/spec/successor/examples/financial-postconditions-payment.mori
npm --prefix experiments/moriarty-language run financial-postconditions-demo
```

The new profile rejects `--schema`. Check and format reject `--action`.
Simulate requires `--action` once. Selection is mandatory even when the
source contains one action. There is no implicit first-action fallback.
Snapshots supply runtime `Pre`, `Args`, `Obs` and `workInitial` for the
selected action only. State fields have no initializer. The repayment-state
file is a local validated projection, not authenticated ledger state.

## Declarations

The [grammar](financial-agreement-source-v4-grammar.ebnf) keeps the /3
declaration spellings and adds six generic POST reads. Zero actions reject
`SOURCE_ACTION_COUNT`. The six PRE names stay reserved in /3 and /4.
The six new POST names are reserved only in /4 and remain legal identifiers
in /1–/3.

## Financial reads

Unprefixed reads keep immutable PRE meaning throughout the action,
including delayed `ensures`. POST reads are legal only in `ensures`,
including nested and short-circuited expressions. Static placement still
checks dead branches. A POST read in `requires`, `let`, `next` or `emit`
rejects `TYPE_POST_SCOPE` at the original source location.

| Source | Result | Lookup |
| --- | --- | --- |
| `post_outstanding<Cash>(id)` | `Quantity<Units<Cash,1>,0>` | prepared obligation.outstanding |
| `post_principal<Cash>(id)` | same Quantity | prepared obligation.principal |
| `post_accrued<Cash>(id)` | same Quantity | prepared obligation.accrued |
| `post_balance<Cash>(id)` | `Amount<Cash>` | prepared `(party, Cash)` amount |
| `post_allowance_remaining<Cash>(id)` | same Amount | prepared remaining |
| `post_allowance_spent<Cash>(id)` | same Amount | prepared spent |

Core `/3` constructors are `ReadPostOutstanding`, `ReadPostPrincipal`,
`ReadPostAccrued`, `ReadPostBalance`, `ReadPostAllowanceRemaining` and
`ReadPostAllowanceSpent`. Each has an identifier operand `unit` or `asset`
and an `identity` expression. Old Core `/1` and `/2` reject those
constructors, including in dead branches. Missing entries reject
`MISSING_OBLIGATION`, `MISSING_BALANCE` or `MISSING_ALLOWANCE` and never
default to zero. A Quantity value above signed128 maximum rejects
`ARITH_RANGE`. Full UInt128 Amount values remain representable.

## Evaluation and publication

`/4` type-checks every action, selects one action, admits owned financial
input, validates snapshots, executes the prefix once, prepares the existing
kernel once, then executes the entire `ensures` suffix once. All `ensures`
run after the kernel, including ordinary-only ones. Earlier profiles retain
their original ensure-before-kernel order.

Work is prefix reductions plus kernel actions plus suffix reductions.
Short-circuited nodes consume no reductions. Closure reserve stays
separate. Postcondition rejection after successful kernel preparation
reports total attempted work through the failing suffix node. Kernel
rejection envelopes retain their defined action index. No failed
preparation publishes tentative state, effects or a reusable continuation.

Successful `repay_remaining` on the [example](examples/financial-postconditions-payment.mori)
at debt 100 costs 41 + 2 + 6 + 33 = 82. Spendable 82 succeeds with remaining
0 and reserve 16. Spendable 81 is a late suffix `WORK_EXHAUSTED`. Spendable
43 fails the first ensure. Spendable 42 is kernel `INSUFFICIENT_WORK`.

Core `/3` `check` needs no financial state. Integrated `evaluate` is
action-only funded evaluation. A well-typed standalone expression rejects
`TYPE_ACTION_REQUIRED`. A valid action without state rejects
`FINANCIAL_CONTEXT_REQUIRED`. Extra request fields, including a supplied
financial post-state, reject `INPUT_SCHEMA`.

This slice does not execute K, construct a proof, or submit a public
transaction. The bounded K kernel does not implement the post-read
constructors.
