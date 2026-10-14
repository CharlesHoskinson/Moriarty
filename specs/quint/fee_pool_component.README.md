# FeePool component model

`fee_pool_component.qnt` models the accepted finite monetary component of
`FeePoolU64Component` in `language "moriarty/1"`. Its `State` record has exactly
32 numeric fields and 22 flags; its 23 component actions use nine fixed duties.
`World` adds abstract custody, ordered financial history and QA bookkeeping.
`init` and `step` are model drivers, outside the 23 component actions.
This is a component toward M11; full M11 and Capacity Exchange remain open.

## Behavioral reference and provenance

The authoritative behavioral reference is Project Fenrir's
[`fee_pool.rs`](https://github.com/CharlesHoskinson/projectfenrir/blob/ec8da3be532f29e5e92ff7b1804b7dff0bc89b15/research/fenrir-v2/fenrir-contracts/src/fee_pool.rs),
pinned at commit `ec8da3be532f29e5e92ff7b1804b7dff0bc89b15`.
The corresponding
[`fee_pool.qnt`](https://github.com/CharlesHoskinson/projectfenrir/blob/ec8da3be532f29e5e92ff7b1804b7dff0bc89b15/research/fenrir-v2/quint-spec/fee_pool.qnt)
is a comparison model. The frozen reference bytes have these SHA-256 digests:

| Reference | Bytes | SHA-256 |
| --- | ---: | --- |
| `fee_pool.rs` | 34143 | `193863e898a79cf411b8b8cd78cd11b656920200b03a8af5cbdeb74aaf14f7ae` |
| `fee_pool.qnt` | 32067 | `9dc8e8b16113ca8c583fea3996a4e8498cd6921198fb96b8be174d67ad2b080a` |

The integration preserves the accepted model's executable body. Its original
SHA-256 was `fb9b41a0c6cbfae37e4a81753aee8075625f28086632f5b7aa11714dcb0381d3`.
The 13-test input had digest
`16de81f90768be6b2fd72781efc67fe7fee066f419f0d8a43e9fe71917d77b04`;
the supplemental positive DA/L2 claim test had digest
`5a9465da40410c44d3663011b9f7197e3c9c2fd7ad2b0798ede004c85670e4ae`.
The supplemental case now belongs to `fee_pool_component_test.qnt`, making
14 deterministic tests. These input digests identify provenance, not the
current files. The accepted interface digest is
`67e55497523eb6321dbc4ebfb84460897848f386339c0b89f982aef792f0abc2`;
the requirements below make the tracked model readable independently.

## Finite interface and financial requirements

The single immutable epoch has two operator seats, one DA seat and one watcher
seat. Receipt slots permit one ordinary reimbursement, one watcher stipend and
one watcher reserve spend. Fixed slots abstract identity and membership.
The exact asset is `FEES = (local, fee-pool-source, SERVICE, scale 0)`;
these are local example identities, without deployment implications.
The funder is `funding-source`, and custody belongs to `fee-pool`.
All numeric source fields start at zero and all flags start false. All named
duties start absent. Numeric inputs and predecessor fields must lie in
`0..18446744073709551615` (full U64); operator raw weight is additionally
bounded by 86400. DA and watcher raw weights use U64. Attested effective
weight equals raw weight; unattested effective weight equals zero.

Every action uses a pure `canX` guard and a pure `applyX` transition.
Invalid actions are disabled; there is no rejected-call stutter fallback.
Call `applyX` only after its guard. Pure negative guard tests establish model
refusal conditions; they do not execute Moriarty runtime refusal.

| Actions | Required behavior |
| --- | --- |
| `intake` | Clean genesis and absent duties; compute checked `(epoch+1)*86400`, require observed close at or after it, transfer gross from funder to pool, record gross as carried custody. Gross zero is allowed. |
| `registerOperatorA`, `registerOperatorB`, `registerDa`, `registerWatcher` | Once per slot before settlement; add effective weights. Every operator/DA registration counts toward allocation, including zero weight. Watchers receive no reward seat. |
| `submitOrdinary`, `submitStipend` | Positive gross-funded costs before settlement; total metered cannot exceed gross. Ordinary kind is fixed at 0. Stipend requires positive watcher weight, its limit and a shared once-per-epoch/recipient flag. |
| `settle` | Split surplus `gross-metered` into independent floors 55% operator, 10% DA, 5% held watcher reserve and 15% L2; Midnight receives the exact residual. Route L2 through target-gap refill. Create immutable treasury duties. |
| `allocateOperatorA`, `allocateOperatorB`, `allocateDa` | Each registered reward seat allocates once before finish, against original category total and original aggregate effective weight. Zero aggregate uses denominator 1. Create a present duty even for zero payout. |
| `finish` | Once every registered reward seat is allocated, route operator+DA residue through refill and create L2 dust duty. Clear carried/remainders; preserve older duties and entitlements. |
| `spendWatcher` | One positive spend, kind 7 or 8, within held reserve; create watcher duty and reduce reserve. No new metered cost, stipend kind 3 or watcher day-30 payout. |
| `setTarget` | Replace target only. Lowering it does not withdraw existing reserve or erase duties. |
| Nine `claim*` actions | Positive immutable entitlement, exact present duty and beneficiary, sufficient custody, maturity and one-shot flag. Append Transfer then Discharge, retaining the duty present with zero amount. Claims remain available after finish. |

Untrusted quotient/remainder inputs must satisfy `q*d+r=n*w`, `d>0`,
`0<=r<d`, with U64 operands. This uniquely constrains floor division;
Quint's exact integers permit the U128-sized intermediate product.
Source consumers must range-check before checked arithmetic or effects.
Refill is `min(amount, max(target-balance, 0))`; source implementations must
express the above-target branch without eager unsigned subtraction.
Operator/DA rewards unlock at epoch close plus 2592000 seconds; all other
entitlements unlock at epoch close. Reward unlock overflow disables allocation.
The watcher reserve has no reward vesting clock.

| Duty | Fixed recipient | Immutable amount | Claim action |
| --- | --- | --- | --- |
| `cost_ordinary` | `cost-provider` | `ordinaryAmount` | `claimOrdinary` |
| `cost_stipend` | `watcher` | `stipendAmount` | `claimStipend` |
| `reward_operator_a` | `operator-a` | `opAAmount` | `claimOperatorA` |
| `reward_operator_b` | `operator-b` | `opBAmount` | `claimOperatorB` |
| `reward_da` | `da-provider` | `daAmount` | `claimDa` |
| `treasury_l2` | `l2-treasury` | `l2Amount` | `claimL2` |
| `treasury_midnight` | `midnight-treasury` | `midnightAmount` | `claimMidnight` |
| `treasury_dust` | `l2-treasury` | `dustAmount` | `claimDust` |
| `watcher_spend` | `watcher` | `spendAmount` | `claimWatcherSpend` |

Entitlement creation records liability without paying it or debiting gross
again. The core accounting predicate is
`gross = claimedTotal + outstanding duties + backstop + watcherHeld + carried`.
Custody separately satisfies `pool cash + claimedTotal = gross`.
`allInvariants` also checks exact splits, costs, immutable entitlements,
original-weight allocation, counters, one-shot claims, conditional maturity,
watcher reserve conservation, final residual sweep, exact asset/recipient
history and no refund that resets gross. Backstop is bounded by the maximum
target seen, since an existing balance can exceed a later reduced target.
The unrelated-asset sentinel is QA bookkeeping, not an actual second-asset
ledger execution.

## Premises and correspondence limits

The closed epoch's canonical origin, gross normalization and exclusion of
capacity proceeds, attestation, importer/governance/beneficiary authority,
authenticated predecessor roots and ledger time are external premises.
`observedAt` and `closeObservedAt` are public inputs that enforce numeric
thresholds only. The model does not authenticate them or implement the
reference recipient-secret hash gadget. Fixed names do not establish
cryptographic receipt uniqueness, cross-instance isolation or multi-epoch replay
protection. The initial model funder has U64-max cash and the other actors zero;
actual execution must supply and authenticate its own starting balances.

The abstract ordered history includes only Transfer, Liability and Discharge.
It is not an exported Core execution and omits Core state effects, statement
snapshots, proof transcripts and native ledger handling. Correspondence requires
independent comparison of every source action's complete effects and pre/post
state, exact assets, recipients, amounts and order, including actual atomic
refusals with no committed effects. A successful model guard does not supply
that evidence. The whole-program signed-v2 sole-payer/sole-debtor policy cannot
admit both funded intake and pool claims under one owner; unsigned static
compilation does not close that authentication gap.

Other excluded reference behavior includes general registries and hashes,
multiple epochs and global reserve sharing across epochs, governance step-down
signals and additional cost kinds. This model differs from the old reference
Quint model's conservative signed-63-bit bound and rejected-call stutters:
it preserves full U64 monetary semantics and disables invalid actions.
It does not establish native proof, withdrawal, financial settlement or full
M11 acceptance. Update the model and its checks when the monetary contract or
finite interface changes, and obtain fresh correspondence evidence separately.

## Reproducing bounded QA

Use Quint 0.32.0 with the TypeScript backend. From the repository root:

```sh
flock /tmp/moriarty-language-wave1-build.lock quint typecheck specs/quint/fee_pool_component.qnt
flock /tmp/moriarty-language-wave1-build.lock quint typecheck specs/quint/fee_pool_component_test.qnt
flock /tmp/moriarty-language-wave1-build.lock quint test specs/quint/fee_pool_component_test.qnt --main fee_pool_component_test --backend typescript --seed 20261014 --max-samples 1 --match '.*Test$'
flock /tmp/moriarty-language-wave1-build.lock quint run specs/quint/fee_pool_component.qnt --main fee_pool_component --backend typescript --seed 20261014 --max-samples 20 --max-steps 24 --invariant allInvariants --verbosity 1
```

The 14 deterministic cases include gross111/cost10, surplus99's Midnight
floor15%+4 residual, reverse original-weight allocation with dust1, target
reduction, present-zero duties, disabled guards, late claims, watcher reserve
independence, and positive DA/L2 claims with boundary/replay checks. Exact
U64-max arithmetic and settlement are deterministic cases; the sampled driver
uses only epoch0, gross99/100/111, weights0/1/2, targets0/17/500, fixed cost5,
stipend5, reserve spend1 and mature claim time2678400. It tries target change
once. The driver bounds do not reduce the component's admitted U64 domain.
Each component action has a positive `w*` hit predicate; `wLateClaim` is an
additional target. Reaching an action in a deterministic test does not supply
its sampled witness count. Unobserved targets remain unresolved coverage.

The earlier full-model 1000-trace/40-step run, seed20261014, timed out after
50 seconds with no output. Its invariant verdict and witness counts remain
unknown. A smaller completed run cannot resolve that timed-out run, and no
sample establishes exhaustive safety or formal source correspondence.

Integration checks on 2026-10-14 with Quint 0.32.0: both modules typechecked,
and all 14 deterministic TypeScript tests passed (one sample each,
seed20261014). The bounded TypeScript sample completed 20 traces with a
24-step maximum and seed20261014, checking `allInvariants` and all 24 `w*`
predicates. No counterexample was observed; each of the 23 action targets
was reached in at least 3 of the 20 traces, and `wLateClaim` in 17.
These observations apply to these exact tests and sampled driver bounds.
