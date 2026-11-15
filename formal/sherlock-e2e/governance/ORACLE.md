# Governance oracle: interface

Bounded context Governance (slice C): spec `sherlock-e2e-governance` (EGV-001 to EGV-010), the reused T22 specs of branch `t22-governance` (`sherlock-era-table` ERT, `sherlock-era-ledger` ELD, `sherlock-governance-verbs` GOV, `sherlock-consumer-read` CRR) and LPC-012. Lean package `formal/sherlock-e2e/governance/lean` (namespace `Governance`), executable `.lake/build/bin/oracle`, shared codec `formal/sherlock-e2e/codec` (`SherlockCodec.Wire`).

Quint models must be written from the spec, not from this file. This file only fixes the wire names, the operation codes and the outcome codes, so a replay can map the model's variables to the oracle's cells (`--map`, `--fill`, `--drop`).

## Build and run (odin)

```
cd ~/sherlock/formal-e2e/governance/lean && PATH=$HOME/.elan/bin:$PATH lake build --wfail
LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --fields     # cell names and kinds
LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step       # JSON Lines {"state", "op"}
```

Default mode: `{"protocol": "kink-implementation/1", "inputs": {"tl": 10, "rtl": 5, "q": 2, "n": 3, "grid": 10, "ops": [op, ...]}}` gives `{"end": "final", "cells": {...}}` for the run from `init`. The five parameters are required. `paramsOk`: `1 <= tl`, `0 <= rtl`, `1 <= q <= n <= 9`, `1 <= grid`. The pins use `TL = 10`, `RTL = 5`, `Q = 2`, `n = 3`, `GRID = 10` (test values; EGV-008 keeps the real values open, recommended `TL` = 7 days, `RTL = TL`, `Q` = 12 of `n` = 16, `GRID` = 3,600 s). The wire carries the signature count in one digit, so a replay uses `n <= 9`; the quorum rule is the same at any `n`.

## Cells (all 54 are required in every `--step` state)

| Cell | Kind | Meaning |
|---|---|---|
| `now` | int | block time |
| `tl`, `rtl`, `q`, `n`, `grid` | int | parameters `TL`, `RTL`, `Q`, `n`, `GRID` |
| `current` | int | the current era id (0 none) |
| `pending` | int | the pending (announced) era id (0 none) |
| `nextId` | int | the next era id; ids below it are used and never reused (GOV-007) |
| `e<i>St` (i = 1..3) | int | era `i` status: 0 unused, 1 announced (pending), 2 current, 3 superseded, 4 cancelled, 5 soundness ended |
| `e<i>Start`, `e<i>Fin` | int | `start(i)`, `end(i)` (0 = not set) |
| `e<i>Keyset`, `e<i>Ledger` | int | the era's universal key set digest (an id) and era ledger (1..3) |
| `e<i>AnnAt` | int | block time of the announcement |
| `roster` | int | roster digest in force |
| `rosterPending`, `rosterAt` | int | a pending rotation and the time it takes effect (`now + RTL`) |
| `rosterGen` | int | roster generation, +1 when a rotation takes effect |
| `nonce` | int | signed-verb nonce (GOV-010) |
| `attestRoot`, `attestAt`, `attestUses` | int | the attested root, its record time, accepted uses (GOV-008) |
| `tableDeployed`, `tableSealed` | bool | the era table is deployed; it passed its readback gate (ERT-002) |
| `l1`, `l2`, `l3` | int | era ledger state: 0 not deployed, 1 usable (gate passed), 2 unusable |
| `sherlockUsable` | int | SherlockLedger deploys that passed the gate |
| `unusable` | int | deploys marked unusable (EGV-002) |
| `pathEvidence`, `sealedReports` | int | report labels for unsealed / sealed ledgers (EGV-003) |
| `eraRow` | bool | the T25 rekey list holds the era-check row (EGV-005) |
| `prem1`, `prem2`, `prem3` | int | premise test results A1..A3: 0 untested, 1 pass, 2 fail (EGV-009) |
| `rekeyStarted` | bool | the rekey runner started (EGV-005) |
| `c1`, `c2`, `c3` | int | claims accepted by the era ledger of era 1..3 |
| `uses` | int | claim uses written by the consumer (its own nullifiers, CRR-007) |
| `lastRefusal` | int | 0, or the refusal code of the last refused request (table below) |

## Operations (`op = code * 1000000000 + arg`)

| code | name | arg | enabled when | IDs |
|---|---|---|---|---|
| 1 | tick | `dt` | `0 <= dt` | GOV-005 (rotation takes effect) |
| 2 | announce | `sigs + 10*keyset + 1000*ledger + 100000*T` (sigs 0..9, keyset 0..99, ledger 0..99) | `nextId <= 3`, `1 <= keyset`, ledger 1..3 | GOV-001, GOV-002, GOV-004, ERT-002, ERT-003, ERT-009, ELD-002, EGV-004 |
| 3 | activate | `m` | `m` in 1..3 | GOV-003 |
| 4 | cancel | `sigs + 10*m` | `m` in 1..3 | GOV-001, GOV-007 |
| 5 | soundnessEnd | `sigs` | always | GOV-001, GOV-006 |
| 6 | rotateRoster | `sigs + 10*r` | `1 <= r` | GOV-001, GOV-005 |
| 7 | cancelRotation | `sigs` | always | GOV-011 |
| 8 | attest | `sigs + 10*root` | `1 <= root` | GOV-008 |
| 9 | deploy | `target + 10*committee + 100*threshold + 1000*counter + 10000*vkEq + 100000*ledger` (digits; vkEq 0/1; target 0 table, 1 era ledger, 2 SherlockLedger) | table: not yet deployed, ledger 0; era ledger: ledger 1..3 not yet deployed; SherlockLedger: ledger 0 | EGV-001, EGV-002, ERT-001, ERT-002, ELD-001, ELD-002 |
| 10 | vkChange | 0 | always | EGV-007, GOV-009 |
| 11 | reportLedger | `sealed` 0/1 | always | EGV-003 |
| 12 | premise | `r + 10*i` (r 1 pass, 2 fail; i 1..3) | premise `i` untested | EGV-009 |
| 13 | addEraRow | 0 | no row yet | EGV-005 |
| 14 | rekeyStart | 0 | not started | EGV-005 |
| 15 | acceptCall | `e` | era `e` in 1..3 used (any status) | ELD-003, ELD-004, ELD-007 |
| 16 | useClaim | `e` | era `e` in 1..3 has a claim | CRR-004, CRR-006, EGV-006, CRR-007 |
| 17 | useAttested | 0 | always | GOV-008 |

The deploy op's arguments are the readback; the runner always deploys `{[], 1, 0}` (EGV-001). Spec responses are state outcomes, never `--step` refusals:

| `lastRefusal` | meaning |
|---|---|
| 1 | fewer than `Q` signatures (GOV-001) |
| 2 | announce with a pending era (GOV-002) |
| 3 | `T < now + TL` (GOV-002, GOV-004) |
| 4 | `T` off the grid (ERT-009) |
| 5 | key set held by an earlier era (ERT-003, EGV-004) |
| 6 | ledger held by an earlier era (ERT-003) |
| 7 | ledger has not passed its readback gate (ELD-002, EGV-002) |
| 8 | activate of a non-pending era or before `start(M)` (GOV-003) |
| 9 | cancel of a non-pending era or at/after `start(M)` (GOV-007) |
| 10 | soundness end with no active current era (GOV-006) |
| 11 | verifier-key change (EGV-007, GOV-009) |
| 12 | rekey start with no era row and no failed A2/A3 (EGV-005) |
| 13 | claim of a soundness-ended era (CRR-004) |
| 14 | era not active: at inclusion (ELD-004) or, under the fallback, at use (CRR-006, EGV-006) |
| 15 | attested root not yet usable (GOV-008) |
| 16 | cancel_rotation with no pending rotation (GOV-011) |
| 17 | announce before the era table passed its readback gate (ERT-002) |

The announce checks run in the order 1, 17, 2, 3, 4, 5, 6, 7 (LP-5). `--step` refusals (domain only): `not-an-operation` (unknown code or arg), `op-not-enabled` (operation not enabled), `outside-state-domain` (`validState` fails).

## Reads (ERT-004, ERT-008)

`active(e)`: `e` in 1..3, status announced, current or superseded, `start(e) <= now`, and `now < end_eff(e)` (or `end_eff(e)` unbounded). `end_eff(e) = min(end(e), start(P))` for the pending era `P != e`; an unset `end` or no pending era is unbounded. Reads are pure functions and write no cell.

## Model bounds

At most 3 eras (a fourth `announce` is `op-not-enabled`); one table deploy and one deploy per era ledger; one result per premise; one era row; one rekey start; `n <= 9` (one wire digit for the signature count). Signatures, roots, key set digests and message binding (table address, verb, `roster_gen`) are opaque integers.

## LEAD PICKs

- LP-1: in `end_eff(e)` the pending era `P` is another era than `e` (the ERT-004 scenario "Pending era reached without activate" needs `active(P)` to succeed). FAL-GOVERNANCE-2.
- LP-2: a readback passes only with committee `[]`, threshold exactly 1, counter 0 and equal keys (EGV-002 `{[], 1, 0}`; ERT-002 says `threshold >= 1`). FAL-GOVERNANCE-1.
- LP-3: a cancelled era keeps its key set and ledger for the reuse check of ERT-003.
- LP-4: `soundness_end` clears `current`; the next era needs a new announcement after `TL`. A due activation is applied first (GOV-006), then the era that accepts now is ended and a pending era is cancelled.
- LP-5: when several refusals apply, the first in the order above is recorded.
- LP-6: a premise result is written once (EGV-009); a failed A2 or A3 stays.

## Proved, and open obligations

`inv_run` proves `Inv` after every run from `init`: the scalar bounds, for each era slot its status range, `status != 0` iff the id is below `nextId` (GOV-007), status 1 iff it is `pending`, status 2 iff it is `current`, `end = 0` while pending or current (ERT-007), key set >= 1; and the boolean part: an era announced only after the table gate (ERT-002), the table sealed only when deployed (ERT-001), the rekey runner started only with an era row or a failed A2/A3 (EGV-005). `Steps.lean` holds the one-step requirement theorems; `Tests.lean` the pins.

- OB-1 (ERT-004, ERT-005): the order layer `start(i) < start(j)` and `end(i) <= start(j)` (or `i` current with `j` pending) for `i < j`, which gives at most one active era at every block time. Not proved in Lean: a first per-slot proof did not close within the heartbeat limit and is not in the repository. Checked by `pin_handover`, `pin_before_handover`, `pin_end_once` and the Quint replay.
- OB-2 (ERT-006): an era record keeps its ledger, key set and start after announce. Checked by pins and the replay; no general theorem.

## Not rows of this context

The oracle also models GOV-006 (soundness end), ERT-004 (accepting interval), ERT-009 (grid, `ERT_009_grid`) and CRR-007 (the consumer writes only its own nullifier, `CRR_007_use_writes_own`), whose rows belong to the settlement context; they are not duplicated in `COVERAGE.part.md`. ELD-005 (settlement) is not modelled here.

The Quint model `research/sherlock-t22-governance-20261114/quint/eraTable.qnt` on `t22-governance@fe7207819` (13 invariants, 11 of 11 mutants killed) has no Lean side; this oracle is that side. GOV-008, GOV-010, GOV-011, ERT-009 and CRR-006 were not modelled there (`adopted-C.md`).

## Replay

```
python3 ~/Projects/software-factory/plugins/formal/scripts/quint_replay.py \
  --oracle-json '["/home/charl/sherlock/formal-e2e/governance/lean/.lake/build/bin/oracle"]' \
  --traces <traces dir> --op-var lastOp --compare current,pending,nextId,e1St,e2St,e3St,lastRefusal ...
```

The model's `lastOp` must be mapped to the integer code above (`--action-map` for MBT traces, or a projection adapter); fill the oracle cells the model does not have with `--fill`.
