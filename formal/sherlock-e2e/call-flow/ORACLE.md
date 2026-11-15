# CallFlow oracle: interface (Formal-Lean-AC)

Lean package `formal/sherlock-e2e/call-flow/lean` (namespace `CallFlow`), shared codec
`formal/sherlock-e2e/codec` (Lake `sherlock_codec`). Built on odin with `lake build --wfail`
(Lean 4.35.0): 0 sorry, 50 theorems with pinned axioms (`CallFlow/Audit.lean`, allowlist propext /
Classical.choice / Quot.sound). Binary on odin: `~/sherlock/formal-e2e/call-flow/lean/.lake/build/bin/oracle`.

Quint models must be written from the spec, not from this file; this file only fixes the wire
names that a replay projection maps to.

## Modes

- default: `{"protocol":"kink-implementation/1","inputs":{"vk":INT,"ops":[INT,...]}}` ->
  `{"end":"final","cells":{...}}` or `{"end":"could-not-apply","reason":"not-an-operation|outside-state-domain"}`.
  Param `vk`: the one universal wrap key (>= 1), made with no program input (MYC-SEAL-015).
- `--step`: JSON Lines `{"state":{<all cells>},"op":INT}` -> `{"state":{...}}`, or
  `{"refused":[CODE]}`, or `{"error":TEXT}` (decode failure).
- `--fields`: the cell names and kinds, in order.

Step-domain refusal codes: `not-an-operation` (code or arg outside the table below),
`op-not-enabled` (the op's enabled condition is false; `apply` leaves the state unchanged),
`outside-state-domain` (the given state fails `validState`). Every spec response (a refusal,
a stop, a blocked class) is a state outcome in the cells, never a `--step` refusal.

## Cells

| cell | kind | meaning |
|---|---|---|
| stage | int | last stage done: 0 none, 1 compile, 2 setup, 3 plan, 4 deploy, 5 tape, 6 execute, 7 prove, 8 wrap, 9 ledger check, 10 submit, 11 read-back |
| stopped | bool | the call stopped |
| failedStage | int | stage that failed (EUF-005), 0 none |
| failedPin | int | pin named by the stop: 1 compactc, 2 ZKIR, 3 machine, 4 key set, 5 ledger (EUF-012) |
| nativeRejected | bool | execute rejected the call (EUF-006) |
| universalVk | int | the universal wrap key (param) |
| programs, deploys | int | programs deployed; deployments |
| lastDeployVk | int | key stored by the last deployment (EUF-009, MYC-KEY-002) |
| vkRefused | int | deployments refused for a different key |
| installedKeys, profileKeys | int | installed universal key set; key set of the box profile (EUF-007) |
| provedOk | bool | the prove stage ran on a valid profile |
| claim, claimA, claimB | int | claim_pub of the call; as given by box 1 and box 2 (EUF-008) |
| txs, txTotal | int | transactions of this call (0/1); all transactions |
| unknown | bool | the submit response was lost (EUF-014) |
| resendRefused | int | second submissions refused for the same claim_pub |
| outcome | int | 0 none, 1 read-back equal, 2 differs (EUF-010) |
| eligible, joined | int | calls past execute; calls joined at read-back (EUF-013) |
| firstKeys, mixedKeys | int, bool | key set of the first joined call; a joined call used another |
| complete, completeBlocked | bool, int | prototype reported complete; reports refused |

## Operations: `op = code * 1000000000 + arg`

| code | op | arg | enabled | EARS |
|---|---|---|---|---|
| 1 | step | n + 100 ok (n 1..11) | not stopped, n = stage + 1, status known | EUF-001, 002, 005, 006, 007, 010, 013 |
| 2 | setupBox | key set >= 1 | always | EUF-007 |
| 3 | rekey | key set >= 1 | always | EUF-007, 013 |
| 4 | newCall | identity (= claim) >= 1 | stage 11 or stopped | EUF-002, 014 |
| 5 | checkPins | differing-pin mask 0..31 (bit 0 compactc .. bit 4 ledger) | not stopped, stage < 10 | EUF-012 |
| 6 | proveOnBox | box 1 or 2 | not stopped, stage >= 7 | EUF-008 |
| 7 | deploySecond | key | programs >= 1 | EUF-009, MYC-KEY-002, MYC-SEAL-015 |
| 8 | lostResponse | 0 | not stopped, stage 10, status known | EUF-014 |
| 9 | reconcile | 0 not found, 1 equal, 2 differs | stage 10, status unknown | EUF-014 |
| 10 | reportComplete | 0 | always | EUF-013 |

Stage results: execute with ok = 0 is a native refusal (EUF-006); any other failed stage
before submit stops the call (EUF-005); prove with a profile of another key set stops with
pin 4 (EUF-007); submit with ok = 0 is a lost response; read-back with ok = 0 is a differing
outcome.

## LEAD PICKs in this oracle

- EUF-002: a further call of a deployed program starts after the deploy stage.
- EUF-007: a stale profile stops the call at prove and names the key-set pin (EUF-012 pin 4).
- EUF-012: with several differing pins the lowest-numbered pin is named.

## Replay

```
python3 ~/Projects/software-factory/plugins/formal/scripts/quint_replay.py \
  --oracle-json '["/home/charl/sherlock/formal-e2e/call-flow/lean/.lake/build/bin/oracle"]' \
  --traces <traces dir> --op-var lastOp --compare <cells>
```

`--op-var lastOp` works when the model's `lastOp` holds the integer op above; otherwise use a
projection adapter (`--projection-json '<adapter argv>'`, model state + action -> oracle state +
integer op), owned by the Quint lead.
