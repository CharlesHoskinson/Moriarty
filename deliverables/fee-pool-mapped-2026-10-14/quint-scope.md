# M11 FeePool bounded Quint QA scope

Status: specified only, awaiting root design agreement. This is an implementation-ready plan for the actual Moriarty library, not a Fenrir application model or proof/ledger acceptance claim. No model or product source changed.

## Evidence and boundary

The pinned public reference is Project Fenrir commit `ec8da3be532f29e5e92ff7b1804b7dff0bc89b15`; exact bytes and SHA256 values are in `reference-manifest.json`. Rust is the authoritative behavioral reference. Its `fee_pool.rs` lines 203–215 and 324–350 define U64 checked state arithmetic, U128 products, checked quotient and target-gap routing; 359–442 genesis/import; 450–589 roster/meter receipts; 597–655 settlement; 662–747 fixed original-weight rewards and residual sweep; 757–808 reserve spending; 815–852 claim/target change. Lines 859–901 governance signal are excluded from this monetary profile.

Existing reference `fee_pool.qnt` lines 37–38 use conservative `2^63-1`, not full native U64. Lines 119–122 record rejected calls as ledger stutters; the new main model instead has disabled pure guards and separate actual-refusal tests. Lines 393–399 have unbounded time advance; the new QA environment uses finite monotone maturity phases. These are abstraction differences to disclose, not financial differences to introduce.

Moriarty base `dc31345850b5800687b16b26bfa20f1d66d94211`: `crates/mori-core/src/ir.rs` has UInt(u128)/Bool state, checked Add/Sub/Mul, literal-account Transfer/Fee/Liability/Discharge, and complete Effect records. `crates/mori-core/README.md` documents no branch/division/dynamic accounts, immediate `next`, eager booleans and atomic simulate refusal. A finite public-source profile can enforce U64 ranges and `q*d+r=n*w`, `d>0`, `r<d` using existing primitives. No new division, limbs or G1 change is required.

Startup loaded installed Moriarty and Quint modeling/lang skills; repository `AGENTS.md` was read and guarded `status --json` reports no pending transactions. Existing historical native admission remains blocked; source planning proceeds within parent authorization.

## Proposed finite profile and model shape

Conditional on design A: one exact asset, one epoch, two operator slots, one DA slot, one watcher slot; one ordinary meter, one watcher stipend and one watcher reserve-spend receipt. Nine possible immutable payout slots: two costs, three rewards, two initial treasuries, L2 residue and reserve spend. Accounts and exact AssetId are source literals. This cannot demonstrate unbounded registries, hash keys, cross-epoch import uniqueness or cross-instance isolation. A second nonzero asset in actual Core scenarios is required to exercise the exact-asset effect boundary.

Use plain shared-state Quint: a cohesive Pool record with fixed prepopulated slot maps and present flags, plus separate environment and QA ghost records. Each operation has `pure canX`, `pure applyX`, an independently computed expected-effect function and a thin guarded action. Main `step` has no synthetic rejection/no-op fallback. Every action has a positive action-hit witness, including individual registration/allocation/positive claim slots. The clock is a finite environment premise, never an authored authority Bool or source timestamp passed off as native enforcement.

Sample domains: gross {0,1,7,99,100,101,111,999}; meter amounts {0,1,10,50,100,111}; weights {0,1,2,3}; targets {0,1,15,17,500}; monetary QA aggregates <=1000. Actual source bounds all monetary state/input/q/r to `18446744073709551615`. Epoch close=86400, reward hold=2592000 seconds; maturity phases are before close, closed before vesting and vested. Larger operator uptime/DA values and full U64 boundaries come from independent source fixtures, not the small sampled state domain.

## Required predicates

The JSON artifact gives precise predicates and nonvacuity witnesses. Require monetary entitlement conservation separately from actual custody conservation; reimbursements equal metered costs and precede splitting; independent 55/10/5/15 floors and exact Midnight residual within floor15%+[0,4]; original-category original-weight rewards; no double allocation/claim; reserve credit minus reserve spending equals held watcher funds; no watcher seat rewards; target-gap refill then L2 overflow; full residual sweep only after all reward allocations; roster/meter freezing and global receipt reuse rejection. Compare every financial effect's exact asset, source, recipient and amount, with held-out unrelated balances/duties unchanged. Exact asset isolation cannot prove the imported gross amount excludes economic capacity proceeds or forbidden token origins; that is an explicit canonical-import premise.

Vesting, recipient authentication, genesis initialization, canonical imports and actual withdrawal are explicit conditional predicates. If the frozen source lacks authenticated time/authority/recipient binding, model-side early/foreign rejection cannot be reported as source enforcement. A caller supplied Bool or time is not a substitute. If actual source claims discharge duties and transfer funds, compare both complete effects; if it only emits an entitlement, report the missing withdrawal acceptance predicate.

## Independent expected outcomes and deliberately false properties

- Gross111, costs10 => surplus101, totals55/10/5/15/16. Operator weights1/2 =>18/36, zero DA weight=>0, residue11. Target17 from0: initial backstop15, finish refill2 and L2 residue9.
- Surplus99 =>54/9/4/14/18: Midnight18 is floor15%14 plus4. A +3 bound must fail.
- Op total55 weights1/2 =>18/36 in either allocation order, dust1. A cumulative-floor algorithm produces18/37 in one ordering and must fail expected vectors.
- Settle100 at target17 yields backstop15. Lowering target to0 retains balance15. `balance<=currentTarget` is false by reference design; history-aware bounds and per-refill gap are correct.
- Surplus100 holds watcher5; bounty3 leaves2. Stipends are gross-funded meter costs. Watcher seat allocations are forbidden.
- Global reference reserve may fund a receipt attributed to a different settled epoch. Per-epoch reserve debit<=that epoch's credit is false in the multi-epoch reference; the one-epoch M11 profile excludes this case explicitly.

Independent full-domain vectors are in `arithmetic-expected.md/json` (50 cases); consume them without deriving expectations from source execution output.

## Correspondence and validation sequence

Freeze actual Moriarty source, Program JSON, action inventory, complete effect schema and static compiled artifacts. Map named model transitions to those actual actions and every state/effect field. Compare independently expected complete pre/post snapshots, balances, liabilities, ordered State/Transfer/Fee/Liability/Emit effects and statement snapshots; Core `next` reads change immediately. Every negative guard case must reach actual Core refusal with no committed result. Compare an altered source share or recipient to demonstrate held-out behavioral divergence. Source static checks must show quotient/remainder constraints before financial effects.

After root approval, create separate `fee_pool_m11.qnt` / `fee_pool_m11_test.qnt`, concrete fully bound instance modules, then incrementally typecheck and run each action with a positive witness. Run deterministic scenario/negative tests, then sampled invariants+witnesses using max48 steps,10000 samples, seed20261014; cap each invocation at120 seconds and retain partial results on the task timebox. Exact command forms are in JSON. Inspect short traces as possible terminal/deadlocked states. Zero witness counts are unresolved coverage, not accepted progress.

No `quint verify`, environment installs/setup, native VM, SDK, queries, k/constraint/prove/verify jobs, network, keys, params, SRS or private cache access. No commands were executed against a new model; all results remain specified only. Quint executable presence was observed.

## Decisions required before implementation

Freeze exact library action names and registration interface; custody/intake and immutable entitlement representation; authority/time/recipient binding or named missing predicates; inclusion of target updates; finite receipt identity/replay representation; source/declaration/liability limits for nine payout slots. `quint-scope.json` is the detailed implementation checklist.
