# Author notes: kernel-backed financial reads

Author: Grok 4.6 high, authorized implementation in
`/home/charl/Moriarty/.worktrees/financial-state-reads` on
`feat/financial-state-reads` at base `96860344978e176236a6f152cd694433263c8d5f`.
This is not an independent audit and does not close the goal.

## What changed

Source profile `moriarty-financial-agreement-source/3` and Core contract
`moriarty-financial-expression-contract/2` share the existing compiler and
evaluator behind explicit gates. Six generic reads consume an owned,
fully admitted kernel pre-state. Ordinary `Pre`/`Obs` cannot supply those
values. `/1` and `/2` keep parseOwnedState-after-expression order, reserved
names, Core `/1` constructors and public arities.

Edits are limited to `experiments/moriarty-language/**` and root `README.md`.
Copied AGENTS/ROADMAP/plugin files were not modified in this slice.

## Semantics and compatibility

- Evaluation order for `/3`: compile and type every action; select the
  action; admit full kernel state; check `workInitial`; admit ordinary
  snapshots; reduce with immutable financial pre-state; debit E; map
  descriptors; run `prepareRepayment` once; publish only on success.
- `admitRepaymentStateJSON` exposes `parseState`. It does not run a dummy
  Transfer or empty batch. `/1` and `/2` still use `parseOwnedState`.
- Core `/2` constructors: `ReadOutstanding`, `ReadPrincipal`,
  `ReadAccrued`, `ReadBalance`, `ReadAllowanceRemaining`,
  `ReadAllowanceSpent`. Each has `unit` or `asset` plus `identity`.
- Core `/2` `check` does not require state. `evaluate` without a primitive
  state string rejects `FINANCIAL_CONTEXT_REQUIRED` after typing.
  Callers cannot inject context through the request record.
- Core `/1` rejects the new constructors, including unselected branches.
- New names are reserved only in `/3`. `/2` still accepts `outstanding` as
  an action name. `/2` source cannot parse the new generic calls as reads.
- Missing lookups never default to zero. Settled obligations with zero
  outstanding remain readable. Quantity `2^127` rejects `ARITH_RANGE`.
  Amount permits the full kernel UInt128 domain.
- Reads and ordinary `ensures` see financial pre-state and ordinary
  post-state respectively. Emits do not mutate the read view.
- Successful debit is E + N. Closure reserve is unchanged.

## Exact example work

Frozen `financial-state-payment.mori` bodies, start remaining 256,
spent 0, reserve 16:

| Action | static bound | E | N | remaining | spent |
| --- | --- | --- | --- | --- | --- |
| repay 30 | 43 | 43 | 2 | 211 | 45 |
| repay_installment 20 | 45 | 45 | 2 | 164 | 92 |
| repay_remaining 50 | 47 | 47 | 2 | 115 | 141 |

Fourth `repay_remaining` rejects `GUARD_FAILED` with no posts. Historical
100-work fixture is unchanged.

## Checks actually run

- Red: `npm --prefix experiments/moriarty-language test` before production
  modules existed: 748 pass, 3 fail `ERR_MODULE_NOT_FOUND`. Receipt
  `author-red-tests.txt`.
- Green: same command, 792 pass, 0 fail. `npm --prefix experiments/moriarty-language run typecheck` exit 0.
- `npm --prefix experiments/moriarty-language run financial-state-demo` exit 0.
- README CLI forms from the repository root: check 0, format 0, simulate
  repay 0 (`paid` 30, remaining 211), hostile `payment == 1` exit 1,
  empty stdout, `GUARD_FAILED` on stderr.
- README EBNF block equals
  `financial-agreement-source-v3-grammar.ebnf`.
- Guarded develop status had no pending transactions. Live accounting
  blocks were not used for these local edits.

## Repair 01

Root reproduced Core /2 returning `{status,code,actionIndex}` for invalid
state, grammar `financialRead` spelling a single `typeArgument` while the
parser accepts extra generics, and README omitting family result types.

- Core /2 admission failures now use the expression rejection envelope:
  original kernel code, `SYNTHETIC_SPAN`, `nodePath` `[]`, `workUsed` `0`,
  no `actionIndex`. The `as ExpressionResult` cast on kernel objects is
  gone. Source `/3` still returns `{status,code,actionIndex:null}` for
  kernel validation faults.
- Canonical `/3` EBNF and the README copy spell `financialRead` with
  `typeArgs`. Extra generics still parse and format; static checking still
  rejects `SOURCE_ARITY`.
- README now tables obligation `Quantity<Units<U,1>,0>` and
  balance/allowance `Amount<A>`, with typing judgments and an unchanged
  pre-state lookup costing 1 plus identity. The Dynamic semantics row
  names TypeScript plus the bounded K Transfer/Repay subset and does not
  claim K reads.

Repair checks: 794 pass, typecheck 0, README CLI/demo unchanged in
observable results. Receipts `author-repair-*`.

## Remaining defects and limits

- Root independent probes and the fresh Astra medium audit have not run.
- Changed bytes after those probes would need a current audit.
- K does not implement the new constructors. No native/proof/Preview work
  was done. Local state is a supplied projection, not authenticated ledger
  state.
- The example `repay` / `repay_installment` / `repay_remaining` path uses
  `outstanding`, `balance` and `allowance_remaining`. Tests, not the demo
  bodies, cover `principal`, `accrued` and `allowance_spent`.
- Kernel admission failures keep the kernel envelope (`actionIndex` null)
  rather than a source span. Runtime read failures keep the read-node span
  and actual `workUsed`.
