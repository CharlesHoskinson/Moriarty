# FOREMAN_REPORT

Specified-only financial correction of the workflows fragment.
This output is not runtime acceptance.
This output is not a semantic freeze.
This output is not full SP01 approval.

## Identity

- Worker is Grok 4.6 high.
- Frozen input SHA-256 is `2f4f45e6d693eeec72fb621a5853c4f4c7a6bc9e1acfc9cee577f796f48e79c1`.
- Review SHA-256 is `acf40e0edcd72ec780e8b8748ce812c35270a12e569e81f55ea5cfe101d4f852`.
- Owned `workflows.json` is 349331 bytes, SHA-256 `2b7f65d8eef9822d9923ba5044b7d0906fb4a1e87c481767465df74ccfa14a86`.
- Owned byte limit is 524288. Compact JSON used. Financial fields were not truncated.
- Seven rows, seven traces, and the original 19 mutation ids are preserved.
- Current mutation count is 25 after split controls and the 1501 and controller cases.

## Independent arithmetic

Python3 stdlib integer arithmetic replayed every positive step.
Fees that also appear in `transfers` were applied once.
Every `from` and `to` resolved to an account.
Every quantity field `amount`, `asset`, `unit`, `scale` was present.
Post-state balances matched the running map.
Asset totals were conserved at every step.
Ordinary and reserve work matched the post-state charges.

| Trace | Steps | Ordinary | Reserve | Conserved |
|---|---:|---:|---:|---|
| `intent-cases:partial-fill-batch:trace` | 4 | 40 | 0 | {'WETH': '8000', 'USDC': '15200'} |
| `intent-cases:recurring-payments:trace` | 6 | 46 | 0 | {'WETH': '0', 'USDC': '2000'} |
| `intent-cases:delegated-rebalancing:trace` | 2 | 20 | 0 | {'WETH': '52000', 'USDC': '140000'} |
| `intent-cases:contingent-claim:trace` | 3 | 22 | 0 | {'WETH': '0', 'USDC': '30000'} |
| `intent-cases:cross-domain-recovery:trace` | 3 | 14 | 8 | {'Midnight.USDC': '2000'} |
| `DeFi-regressions:asynchronous-settlement:trace` | 3 | 24 | 0 | {'WETH': '0', 'USDC': '1000'} |
| `DeFi-regressions:conditional-payoff-insurance:trace` | 3 | 28 | 0 | {'WETH': '0', 'USDC': '16000'} |

Fee-aware minReceive:

- Fill1: `2*1510=3020 >= 3000+15=3015`. Ceiling `1508`. Pass.
- Fill2: `2*1010=2020 >= 2000+15=2015`. Ceiling `1008`. Pass.
- Counterexample: `2*1501=3002 < 3015`. Maker1 `5000-1501=3499`. Conserved USDC 15200 and milliWETH 8000. Reject.

Alice signed net USDC:

- `9970 - 15000 = -5030`.
- `-10000 + 4970 = -5030`.
- Fill-plus-fee gross debit `3015+2015=5030`.
- Double-counting the refund would give `-5030+4970=-60`. That result is illegal.

Custody conservation:

- C1 alternate: alice 9500 + insurer 100 + insurerTreasury 20400 = 30000. Payout 10000 against 100 fails.
- C9 alternate: insured 800 + insurer 0 + insurerTreasury 15200 = 16000. Payout 8000 against 0 fails.
- Original C1 forbidden payout remains `mut:claim-as-cash` on the funded no-payout trace.

## Fixes

### WF-F01

Replace the two custody rewrites with complete conserved alternate admitted contexts.
Premium and adjudication already pass.
Payout is authorized in the alternate.
Actual insurer balance is below payout.
Rejected complete state equals the alternate base.
Rejected financial effects are empty.

### WF-F02

Specify minReceive as `2*received >= debit + inputFee`.
Update intent, step checks, acceptance, and verification.
Add independently funded maker 1501 output with maker balance 3499.

### WF-F03

Set cross-domain `cumulativeLocked` to 0 before lock and 1000 through refund.
Set async send authority to 0/400 before send and 400/0 after send.
Track unsettled escrow 0,400,400,0 and cumulative settled 0,0,0,400.
Do not restore send authority at settlement.

### WF-F04

Replace the unfinished `-5030+4970-wait` string with typed `-5030`.
State the escrow custody bound and the fill-plus-fee gross bound.
Enforce both. Refund does not reduce cumulative fill-plus-fee debit 5030.

### WF-F05

Bind all four mandatory predicates to the complete intent, plan, pre-state,
step transfers, nominal changes, observations, post-state, duties,
controllers, authority, history, successors, and genesis/admin boundary.
Frame conditions cover untouched fields and controller lineage.
Each negative uses `expectedStateRef` complete equality.
Rejected financial effects are empty.
Network and proof fees stay a separate scope.
Remove `financialStateUnchanged: true`.
Add delegated unauthorized-controller mutation.
Split disjunctive proposed inputs into distinct controls.

### WF-F06

Define footprint counts as per-trace accesses, not unique resources.
RR-N7 write bound is at least 3.
D-cover-C9 write bound is at least 3.
DC-M1 write bound is at least 2.
Include recurring duty/status/totals, delegated effect grant, stored observations, and cover writes.
Traces 2, 4 and 5 now have `expected.duties` equal to complete post duties, including explicit empty lists.
All mutation and representation refs resolved.

### WF-F07

Persist `C1-cover` with principal, insurer, premium, conditions, controller, and lifecycle.
Carry it through premium, observation, and admission.
Claim C1 references `C1-cover`.
Cover, nominal claim, and paid cash stay distinct.

## Gaps

- No Moriarty evaluator run.
- No native proof.
- No Preview, wallet, or chain operation.
- Independent GPT-6 review of these corrected bytes is not in this worker.
- Toy prices, calendars, registries, custody assumptions, and bounds stay disclosed and unproved.

