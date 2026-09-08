# FOREMAN_REPORT

Specified-only admission correction of the workflows fragment.
This output is not runtime acceptance.
This output is not a semantic freeze.
This output is not full SP01 approval.

## Identity

- Worker is Grok 4.6 high.
- Immutable candidate-01 SHA-256 is `2b7f65d8eef9822d9923ba5044b7d0906fb4a1e87c481767465df74ccfa14a86`.
- GPT-6 review SHA-256 is `0247c48e97e0893df46059e835c0d4cd7076e3fef94995d7a2b658d72b010fb6`.
- Owned `workflows.json` is 442746 bytes, SHA-256 `10af14f28c1710954003ad076b56a93331ae628dd7243c2fc67b17b0a3ca623d`.
- Owned byte limit is 524288.
- Seven rows, seven traces, 25 mutation ids, nine source pins, and 88 verification items are preserved.
- Candidate-01 files were not written.

## Independent arithmetic

Python3 stdlib integer arithmetic replayed every positive step.
Fees that also appear in `transfers` were applied once.
Post-state balances matched the running map.
Asset totals were conserved.
Ordinary and reserve work matched the post-state charges.

| Trace | Steps | Transfers | Ordinary | Reserve | Conserved |
|---|---:|---:|---:|---:|---|
| `intent-cases:partial-fill-batch:trace` | 4 | 8 | 40 | 0 | WETH 8000, USDC 15200 |
| `intent-cases:recurring-payments:trace` | 6 | 5 | 46 | 0 | USDC 2000 |
| `intent-cases:delegated-rebalancing:trace` | 2 | 3 | 20 | 0 | WETH 52000, USDC 140000 |
| `intent-cases:contingent-claim:trace` | 3 | 1 | 22 | 0 | USDC 30000 |
| `intent-cases:cross-domain-recovery:trace` | 3 | 3 | 14 | 8 | Midnight.USDC 2000 |
| `DeFi-regressions:asynchronous-settlement:trace` | 3 | 2 | 24 | 0 | USDC 1000 |
| `DeFi-regressions:conditional-payoff-insurance:trace` | 3 | 2 | 28 | 0 | USDC 16000 |

Fee-aware minReceive is unchanged.

- Fill1: `2*1510=3020 >= 3000+15=3015`. Ceiling 1508. Pass.
- Fill2: `2*1010=2020 >= 2000+15=2015`. Ceiling 1008. Pass.
- Counterexample: `2*1501=3002 < 3015`. Maker1 `5000-1501=3499`. Reject.

Alice signed net USDC is unchanged: `9970-15000=-5030` and `-10000+4970=-5030`.

Lock authority equation uses unit USDC scale 0 at all four states: `1000=1000+0`, then `1000=0+1000`.

C1 alternate starting totals: `9500+100+20400=30000`. Payout 10000 against 100 fails.
C9 alternate starting totals: `800+0+15200=16000`. Payout 8000 against 0 fails.
Alternate ordinary work is 0. Original funded traces still charge 22 and 16.

## Commands

All commands used Python3 stdlib. Exit 0.

1. Inspect packet, review, binding, owned fragment.
2. Apply WF-R01 through WF-R05.
3. Shrink duplicated conjunction copies to shared refs.
4. Replay 24 steps and 24 transfers. Check units, refs, footprints, rollback.

## Fixes

### WF-R01

Mutations 10 and 18 admit the entire alternate as a distinct toy starting boundary.
Genesis ids are `H-gen-cc-uf` and `H-gen-in-uf`.
Admin ids are `H-admin-cc-uf-bounds` and `H-admin-in-uf-bounds`.
No `preStateRef` points at the original funded preState.
History consumes only those genesis and admin ids.
Intent, plan, controller authorization, current head, and work belong to that boundary.
Payout authority is explicit distinct enrollment.
Type, currentness, time, budget, and head checks pass.
Custody is the sole failure.
Rollback equals that admitted whole alternate.
Original C1 and C9 positive histories are unchanged.

### WF-R02

`admittedToyContext` holds the current semantics/key/spec registry, named assumption registry, and pre-verification budgets.
Budgets are checked before expensive verification.
The four claims bind that finite conjunction and each trace `updateRelation`.
Unknown, revoked, stale, and budget controls are specified-only.
No runtime or proof claim is added.

### WF-R03

Affected negatives use `legalStepRef` plus exact overrides, or a complete typed input boundary.
Each lists currentTime, signer, controller, current head, budget, and stage order.
T5 uses now `1714435200` at or after dueTime.
M2 replay rejects consume-once before custody. Escrow `0<400` is unreachable.
Two-successor candidates are complete records, not labels.
CreateDebt and destination re-credit payloads are complete.
Fee-aware 1501 inherits duties, allowance, history, and work from legal fill1.

### WF-R04

Mechanical diffs of all 24 complete states were mapped to canonical resources.
Added writes: recurring `allowances:aggregate` 5, contingent premiumPaid 1, claimNominal 1, custody 1, cross-domain fee 1 and status 3, async status 3, insurance coverPayout 1 and custody 2.
Associated reads are present.
`totals:remaining_aggregate` is an alias of remaining only, with formula and dependencies.
Delegated singular `allowance:*` duplicates were removed.
Derived conservation and lock equations are listed.

### WF-R05

`remainingLockAuthority.unit` is USDC scale 0 at all four cross-domain states.
Mutation 22 destination credit uses unit USDC with asset Assumed.ForeignUSDC.
Every amount record was checked against `assetUnitScaleRegistry`. Zero mismatches remain.

## Gaps

- No Moriarty evaluator run.
- No native proof.
- No Preview, wallet, or chain operation.
- Independent GPT-6 review of these corrected bytes is not in this worker.
- Toy registries, calendars, custody, and bounds stay disclosed and unproved.
