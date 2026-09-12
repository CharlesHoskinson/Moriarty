# Financial agreement source /3

This profile declares one bounded repayment contract with multiple named
actions and six generic financial reads of a validated kernel pre-state. It is
a distinct `moriarty-financial-agreement-source/3` entry. The Core contract is
`moriarty-financial-expression-contract/2` because the previous constructor set
stays closed. Profiles /1 and /2 keep their exact APIs, reserved names and
Core /1 constructors.

```javascript
import { createFinancialAgreementSourceV3 } from '../../src/successor/financial-agreement-source-v3.ts';
const language = createFinancialAgreementSourceV3();
language.check(source);
language.elaborate(source);
language.evaluate(source, actionName, snapshotCanonicalJSON, repaymentStateJSON);
```

From the repository root:

```sh
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/3 experiments/moriarty-language/spec/successor/examples/financial-state-payment.mori
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-agreement-source/3 experiments/moriarty-language/spec/successor/examples/financial-state-payment.mori
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-agreement-source/3 --action repay --snapshots experiments/moriarty-language/spec/successor/examples/financial-state-payment.snapshots.json --repayment-state experiments/moriarty-language/spec/successor/examples/financial-state-payment.state.json experiments/moriarty-language/spec/successor/examples/financial-state-payment.mori
npm --prefix experiments/moriarty-language run financial-state-demo
```

The new profile rejects `--schema`. Check and format reject `--action`.
Simulate requires `--action` once. Selection is mandatory even when the
source contains one action. There is no implicit first-action fallback.
Snapshots supply runtime `Pre`, `Args`, `Obs` and `workInitial` for the
selected action only. State fields have no initializer. The repayment-state
file is a local validated projection, not authenticated ledger state.

## Declarations

The [grammar](financial-agreement-source-v3-grammar.ebnf) keeps the /2
declaration spellings, including uninitialized state, record and operation
forms, and multiple `action` declarations. It adds six generic reads. Zero
actions reject `SOURCE_ACTION_COUNT`. Duplicate action names, or an action
name that collides with another top-level declaration, reject at the offending
declaration. Unit and asset keep separate namespaces.

All existing /2 declarations, actions and ordinary expressions work in /3.
The six read names are reserved only in this profile. They are not reserved
in /1 or /2, and they are unavailable through those older entries.

## Financial reads

Each read takes type arguments in the same `typeArgs` form as other
generics. Exactly one simple generic symbol is a static rule
(`SOURCE_ARITY` / `SOURCE_TYPE_SHAPE`), not a parse rejection. The
identity argument has type `Text` and is evaluated once:

| Source | Result | Lookup |
| --- | --- | --- |
| `outstanding<Cash>(id)` | `Quantity<Units<Cash,1>,0>` | obligation.outstanding |
| `principal<Cash>(id)` | `Quantity<Units<Cash,1>,0>` | obligation.principal |
| `accrued<Cash>(id)` | `Quantity<Units<Cash,1>,0>` | obligation.accrued |
| `balance<Cash>(id)` | `Amount<Cash>` | exact `(party, Cash)` amount |
| `allowance_remaining<Cash>(id)` | `Amount<Cash>` | exact `(party, Cash)` remaining |
| `allowance_spent<Cash>(id)` | `Amount<Cash>` | exact `(party, Cash)` spent |

Cash is a declared unit for obligation reads and a declared asset for
balance and allowance reads. A wrong argument type rejects `TYPE_MISMATCH`.
An unknown unit or asset rejects `TYPE_NAME`. Spans are the original UTF-8
call.

Core /2 constructors are `ReadOutstanding`, `ReadPrincipal`, `ReadAccrued`,
`ReadBalance`, `ReadAllowanceRemaining` and `ReadAllowanceSpent`. Each has
an identifier operand `unit` or `asset` and an `identity` expression. They
are not `ReadPre` or `ReadObs`. Ordinary snapshots cannot supply these
values. A Core /2 evaluate without bound financial context rejects
`FINANCIAL_CONTEXT_REQUIRED`. Static `check` does not require live state.
Core /2 state-admission failures keep the kernel code and use the
expression rejection envelope: synthetic span `[0,0)`, empty `nodePath`,
`workUsed` `0`, and no `actionIndex`. Source `/3` evaluate still returns
the kernel envelope with `actionIndex` null for those faults. Core /1
rejects the new constructors, including in unselected branches.

At reduction the identity must match the kernel ASCII identifier rule.
Invalid identities reject `INVALID_IDENTIFIER`. Missing entries reject
`MISSING_OBLIGATION`, `MISSING_BALANCE` or `MISSING_ALLOWANCE` and never
default to zero. An obligation whose denomination differs from the generic
unit rejects `NOMINAL_UNIT`. A Quantity value above signed128 maximum
rejects `ARITH_RANGE` without truncation. Full UInt128 Amount values remain
representable. Settled obligations are readable and have zero outstanding.

All reads in an action, including ensures, see the same financial PRE
projection. Emitted descriptors are staged and do not mutate it. Ordinary
`post` ensures continue to see ordinary post-state. There are no financial
post-state reads.

## Validation, ownership and work

Evaluation order:

1. Primitive bounded source, parse, shared declarations, protected Transfer
   and Repay binding, and static checking of every action.
2. Mandatory primitive exact action selector. Missing or non-identifier
   selectors reject `SOURCE_ACTION_NAME`. Unknown names reject
   `SOURCE_ACTION_UNKNOWN`. Those use a synthetic span, empty `nodePath`
   and `workUsed` `0`.
3. Bounded primitive repayment-state JSON and full kernel state admission,
   including unrelated entries, closed keys, uniqueness, arithmetic
   relationships, conversion/status and work invariants.
4. Work equality with `workInitial`, selected ordinary snapshot admission,
   then selected expression evaluation with an immutable owned financial
   pre-state.
5. Debit expression work, map descriptors, run the existing repayment
   kernel once, and publish combined ordinary/financial result only on
   success.

`parseOwnedState` remains the shallow /1 and /2 helper. /3 uses the kernel
`parseState` rules before any guard or lookup. Malformed state beats runtime
false guards and missing lookups. A malformed unselected action still beats
all runtime inputs. State admission failures retain kernel codes such as
`DUPLICATE`, `CAPACITY`, `INVARIANT` and `INVALID_AMOUNT`, with
`actionIndex` null and no effects or posts.

Each read costs one expression reduction plus its identity child's actual
reductions. Short-circuit And/Or/Select executes only the selected runtime
branch; static checking covers every branch and action. Unselected actions
add no runtime debit. State admission and index building are bounded
validation work outside runtime E; each collection remains capped at 128.
Successful total debit is E + N, where N is the emitted kernel action
count. `closureReserve` stays separate. Failed guard, read, ensure, funding,
work or identifier reuse exposes no tentative output.

The public API accepts primitive source, action-name, snapshot and
repayment-state strings only. Returned schema and Core are inspectable
output. Later evaluate compiles from original source and never consumes
those artifacts.

## Results

Successful evaluate returns `FundedExpressionPrepared` with ordinary
`post`, complete `financialPost`, kernel `effects` and `workRemaining`.
CLI success exits 0 with `judgmentResult` `SourceSimulated`,
`sourceProfile`, `action`, ordinary `pre`, `financialPre`, `initialWork`
and `result`. Semantic rejection exits 1 with JSON on stderr, empty stdout
and no tentative result.

The [example](examples/financial-state-payment.mori) declares `repay`,
`repay_installment` and `repay_remaining`. Starting remaining work is 256,
spent 0 and `closureReserve` 16. Default `repay` of 30 has E 43 and N 2,
leaving remaining 211 and outstanding 70. Continuation `repay_installment`
of 20 has E 45 and N 2, leaving remaining 164 and outstanding 50.
`repay_remaining` reads the remaining 50, has E 47 and N 2, and emits
Transfer then Repay for that amount, leaving remaining 115. Final
principal, accrued and outstanding are 0, status Settled, payer/lender
balances 0/100, allowance remaining/spent 0/100, and ordinary `paid` 100.
A fourth `repay_remaining` fails the positive-payment guard with no
effects. Unrelated projection rows are preserved. Static work bounds equal
those E values because the example has no skipped branches. This slice does
not execute K, construct a proof, or submit a public transaction. The
bounded K kernel does not implement the new read constructors.
