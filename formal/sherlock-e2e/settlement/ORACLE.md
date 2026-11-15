# Settlement oracle: interface (Formal-Lean-AC)

Lean package `formal/sherlock-e2e/settlement/lean` (namespace `Settlement`), shared codec
`formal/sherlock-e2e/codec` (Lake `sherlock_codec`). Built on odin with `lake build --wfail`
(Lean 4.35.0): 0 sorry, 87 theorems with pinned axioms (`Settlement/Audit.lean`, allowlist
propext / Classical.choice / Quot.sound), `#audit_ns Settlement` = `[]`. Binary on odin:
`/home/charl/sherlock/formal-e2e/settlement/lean/.lake/build/bin/oracle`.
Specs: `sherlock-e2e-ledger-contract` (SLC, STL, LDG, LW-009, ANC-012, ZK, CLM, FREG, CRR-007,
ELD-005, ERT, GOV-006) and `sherlock-e2e-batched-settlement` (BAT). The LDG rows are modeled
from the t20-batched text (LEAD PICK, see FAL-SETTLEMENT-1).

Quint models must be written from the spec, not from this file; this file only fixes the wire
names that a replay projection maps to.

## Modes

- default: `{"protocol":"kink-implementation/1","inputs":{"collBudget":INT,"inclTime":INT,"regRoot":INT,"ops":[INT,...]}}`
  -> `{"end":"final","cells":{...}}` or `{"end":"could-not-apply","reason":...}`.
  Params: `collBudget` = collection proof time (s), `inclTime` = inclusion time (s),
  `regRoot` = current registry root (LDG-008).
- `--step`: JSON Lines `{"state":{<all cells>},"op":INT}` -> `{"state":{...}}`, or
  `{"refused":[CODE]}` (step domain only), or `{"error":TEXT}`.
- `--fields`: the cell names and kinds, in order.

Every spec response (a refused run, a blocked or refused collection, a refused plan) is a
state outcome, never a `--step` refusal.

## Cells (all required, exact keys)

| cell | kind | meaning |
|---|---|---|
| deployed | bool | a contract instance exists |
| anchor | int | anchor value written by the constructor (deploy arg cp) |
| anchorSlot | int | slot of the anchor (15, SLC-002) |
| anchorSealed | bool | the anchor slot is written only by the constructor |
| entries | int | exported entries (4: acceptRun, acceptCollection, readRoot, readRun; SLC-001) |
| keyRun, keyColl | int | key kind on the write entries (1 = universal wrap key, SLC-003) |
| keyRoot, keyRunRead | int | key kind on the read entries (2 = native key, SLC-004) |
| deploys | int | instances deployed |
| r1, r2, r3 | int | `runs[claim_pub]` outcome for claims 1..3 (0 = absent) |
| acceptRunTx | int | accepted acceptRun transactions |
| argFiles | int | argument files written; argsReady: a file is ready for one call |
| argsReady | bool | see argFiles |
| mismatches | int | settled runs whose outcome word differs from the expectation (STL-005) |
| refusedNative | int | calls refused before submit (native transcript or unbound leaf) |
| routedJobs | int | operator jobs routed to the collection path (BAT-002) |
| lastRefusal | int | bit set of the last refusal (table below), 0 none |
| collForm | bool | the L collection form is in the key set (BAT-009) |
| epochOpen, epochStart, epochJobs | bool/int/int | open epoch, its start time, its claims |
| now | int | clock (s) |
| eraEnd | int | announced `end_eff` of the accepting era (0 = none) |
| collBudget, inclTime | int | params |
| epochsClosed, jobsClosed | int | closed epochs, claims in them |
| collTx | int | acceptCollection transactions (one per closed epoch at most) |
| blockedColl, refusedColl | int | closes without the form, refused collection writes |
| pathsGiven, leafBytes | int | membership paths given (BAT-007), leaf bytes kept (32 per claim, BAT-008) |
| cr1, cr2, cr3 | bool | `claim_roots` holds root 1..3 (LDG-003) |
| regRoot | int | param |
| intentTtl | int | intent TTL (s) |
| planned, planRefused, lastK | bool/bool/int | last plan: refused (BAT-005), chunk count |
| runsStarted, runsRefusedEra | int | runs started, runs refused for the era end (BAT-011) |
| freezeWritten, openListed | bool | freeze record written; collections listed as open (BAT-012) |

Refusal bits in `lastRefusal`: 1 native transcript (SLC-010), 2 unbound leaf (SLC-012), 4 other
program, node error 115 (SLC-005), 8 replay of a claim (LDG-002), 16 argument mismatch
(SLC-009), 32 no collection form (BAT-009), 64 registry root differs (LDG-008), 128 duplicate
claims root (LDG-003), 256 run cannot land before end_eff (BAT-011).

## Operations: `op = code * 1000000000 + arg`

| code | operation | arg | enabled when | EARS |
|---|---|---|---|---|
| 1 | deploy | cp 1..99 (anchor) | always | SLC-001..004, STL-003 |
| 2 | buildArgs | 1 = a recomputed word differs, 0 = equal | deployed, no file ready | SLC-006, SLC-009, STL-001 |
| 3 | acceptRun | `c + 10 o + 1000 cp + 10^5 operator + 10^6 native + 10^7 unbound + 10^8 differs` (claim c 1..3, outcome o 1..99, cp 0..99 program anchor, flags 0/1) | deployed, file ready | SLC-005, SLC-010, SLC-012, LDG-002, STL-004, STL-005, BAT-002 |
| 4 | tick | dt >= 1 | no open epoch, or now + dt <= deadline | BAT-001 |
| 5 | job | 0 (the first job opens an epoch) | no open epoch, or claims < 2^20 and now < deadline | BAT-001 |
| 6 | closeEpoch | `root + 10 reg` (root 1..3) | open epoch and (now >= deadline or claims = 2^20) | BAT-001, BAT-007..010, LDG-003, LDG-006, LDG-008 |
| 7 | announceEra | end_eff >= 1 | always | BAT-010, BAT-011, GOV-006 |
| 8 | startRun | proving time p >= 0 | always | BAT-011 |
| 9 | intent | TTL (0 = none given -> 3,600) | always | BAT-006 |
| 10 | plan | entry length in words >= 1 | always | BAT-005, ANC-012 |
| 11 | addCollForm | 0 | always | BAT-009 |
| 12 | replayStart | 1 = state equals deploy-block state, 0 = differs (new instance, same anchor) | deployed | SLC-011 |
| 13 | freeze | 0 | always | BAT-012 |

`deadline = epochStart + 3,600` without an era end, else
`min(epochStart + 3,600, eraEnd - collBudget - inclTime)`. A plan needs `chunks(le) = 1` for
`le <= 1,015`, else `ceil(le / 990)`, and is refused when `chunks > K_MAX(ttl)`.

Step-domain refusal codes: `not-an-operation`, `op-not-enabled` (state unchanged),
`outside-state-domain` (`validState` false: negative counters or clock, epochJobs > 2^20,
lastRefusal outside 0..511).

## Constants

epoch 3,600 s, epoch cap 2^20 claims (claims tree depth 20), default TTL 3,600 s, call words
1,015 (= 1,024 - 9), chunk words 990, 1,755 s per chunk, K_MAX cap 2, anchor slot 15; largest
measured calls 474 and 699 words (t20 RESULTS.md:116).

## Replay

```
python3 ~/Projects/software-factory/plugins/formal/scripts/quint_replay.py \
  --oracle-json '["/home/charl/sherlock/formal-e2e/settlement/lean/.lake/build/bin/oracle"]' \
  --op-var lastOp --traces <traces dir> --compare <cells>
```

`--op-var` works when the model's op variable holds the integer above; otherwise the Quint lead
supplies a projection adapter (`--projection-json`).

## LEAD PICKs in this oracle

- LDG-001..008, LW-009: the t20-batched text (5dc1be2ee, four entries, `[reg_pub, claims_root]`)
  is modeled; the in-tree pre-T20 text contradicts SLC-001 (FAL-SETTLEMENT-1).
- BAT-005 / ANC-012: `K_MAX = min(min(floor(TTL / 1,755), 18), 2)`; BAT-005's status says it
  supersedes the ANC-012 formula (FAL-SETTLEMENT-2).
- BAT-001: the clock cannot pass an open epoch's deadline; the close is enabled at the deadline
  or at 2^20 claims.
- BAT-010 / BAT-011: modeled as written (close at `end_eff - proof - inclusion`, a run refused
  only when it ends after `end_eff`); both admit landing exactly at `end_eff`, where ERT-004
  accepts nothing (FAL-SETTLEMENT-3).
- BAT-002: the contract cannot tell an operator; the oracle routes an operator job to the
  collection path without an acceptRun.
