# Ledger-verification oracle: interface (Formal-Lean-AC)

Lean package `formal/sherlock-e2e/ledger-verification/lean` (namespace `LedgerVerification`),
shared codec `formal/sherlock-e2e/codec` (Lake `sherlock_codec`). Built on odin with
`lake build --wfail` (Lean 4.35.0): 0 sorry, 47 theorems with pinned axioms
(`LedgerVerification/Audit.lean`, allowlist propext / Classical.choice / Quot.sound),
`#audit_ns LedgerVerification` = `[]`. Binary on odin:
`/home/charl/sherlock/formal-e2e/ledger-verification/lean/.lake/build/bin/oracle`.
Spec: `sherlock-e2e-ledger-verifier` (LVR-001..008, MID-001..004, MID-006, MID-007).

Quint models must be written from the spec, not from this file; this file only fixes the wire
names that a replay projection maps to.

## Modes

- default: `{"protocol":"kink-implementation/1","inputs":{"ops":[INT,...]}}` (no params) ->
  `{"end":"final","cells":{...}}` or `{"end":"could-not-apply","reason":...}`.
- `--step`: JSON Lines `{"state":{<all cells>},"op":INT}` -> `{"state":{...}}`, or
  `{"refused":[CODE]}` (step domain only), or `{"error":TEXT}` (decode failure).
- `--fields`: the cell names and kinds, in order.

Every spec response (a nonzero exit, a refused key, a refused call, a chain stop) is a state
outcome, never a `--step` refusal.

## Cells (all required, exact keys)

| cell | kind | meaning |
|---|---|---|
| cr0..cr3 | bool | manifest crate `i` resolves to the pinned version (LVR-002) |
| gateOk | bool | the manifest gate passed on the current manifest; any manifest change resets it |
| keyRead | bool | a non-G4 verifying key was read (LVR-003, MID-001) |
| lastG4 | bool | the last key offered was a G4 key |
| checks | int | checks run |
| controls | int | controls run (10 per check: 7 lanes + 3 flips, LVR-004) |
| checkDone | bool | at least one check finished |
| exitNonzero | bool | exit status of the last check (LVR-005) |
| lastHonest, lastLanes, lastFlips, lastGate | bool/int/int/bool | inputs of the last check |
| callPhase | int | 0 none, 1 built, 2 native accepted, 3 submitted, 4 refused (LVR-006, LVR-007) |
| nativeChecks, nativeOk | int | native-validator runs, accepted runs |
| refusedCalls | int | calls the native validator refused (never submitted) |
| submits | int | submitted calls |
| genesis | int | genesis hash of the deploy chain (0 = none) |
| nodeVersion | int | node version recorded at the first connect |
| connects | int | connects |
| chainStop | bool | a connect saw another genesis (LVR-008) |
| lastRefusal | int | 0 none, 1 G4 key (LVR-003), 2 native refused (LVR-007), 3 other chain (LVR-008) |

## Operations: `op = code * 1000000000 + arg`

| code | operation | arg | enabled when | EARS |
|---|---|---|---|---|
| 1 | manifest | `10 i + b` (i crate 0..3, b 1 = pinned version) | always | LVR-002 |
| 2 | gate | 0 | always (gateOk := all four pinned) | LVR-002 |
| 3 | loadKey | 1 = G4 key, 0 = universal wrap key | always | LVR-003, MID-001 |
| 4 | check | `h + 10 lanes + 100 flips` (h 1 = honest accepted; lanes 0..7 refused; flips 0..3 refused) | keyRead | LVR-001, LVR-004, LVR-005 |
| 5 | buildCall | 0 | callPhase 0, 3 or 4 | LVR-006 |
| 6 | native | 1 = native validator accepts, 0 = refuses | callPhase 1 | LVR-006, LVR-007 |
| 7 | submit | 0 | callPhase 2 | LVR-006 |
| 8 | connect | `1000 g + v` (g genesis >= 1, v node version < 1000) | not chainStop | LVR-008 |

A check exits 0 only when the honest proof is accepted, all 7 lanes and all 3 flips are
refused, and the manifest gate passed (`gateOk`).

Step-domain refusal codes: `not-an-operation` (unknown code or arg outside its encoding),
`op-not-enabled` (state unchanged), `outside-state-domain` (`validState` false: negative counts,
lastLanes > 7, lastFlips > 3, callPhase > 4, lastRefusal > 3).

## Constants

VK bound 50,000 B, measured wrap VK 1,325 B (t17-preview RESULTS.md:17), acceptRun 6,780 B
(Preview) and 6,783 B (devnet), transaction limit 1,048,576 B (t20 RESULTS.md:11,15); 7 control
lanes 0, 10, 20, 34, 39, 513, 1025 and 3 flips (LVR-004).

## Replay

```
python3 ~/Projects/software-factory/plugins/formal/scripts/quint_replay.py \
  --oracle-json '["/home/charl/sherlock/formal-e2e/ledger-verification/lean/.lake/build/bin/oracle"]' \
  --op-var lastOp --traces <traces dir> --compare <cells>
```

`--op-var` works when the model's op variable holds the integer above; otherwise the Quint lead
supplies a projection adapter (`--projection-json`).

## LEAD PICKs in this oracle

- LVR-001: the stock verifier counts only with pinned crates; a check whose manifest gate did
  not pass exits nonzero (the spec status names the gate as MISSING in code).
- LVR-008: a reconnect at the same genesis keeps the first recorded node version (recording a
  later version is STL-006's task per the spec status); another genesis stops (chainStop,
  lastRefusal 3) and later connects are disabled.
- MID-002 is not modeled: it contradicts SLC-004 (FAL-LEDGER-VERIFICATION-1); the settlement
  oracle models the SLC-004 key placement.
