# Conformance oracle: interface (Formal-Lean-AC)

Lean package `formal/sherlock-e2e/conformance/lean` (namespace `Conformance`), shared codec
`formal/sherlock-e2e/codec` (Lake `sherlock_codec`). Built on odin with `lake build --wfail`
(Lean 4.35.0): 0 sorry, 54 theorems with pinned axioms (`Conformance/Audit.lean`, allowlist propext /
Classical.choice / Quot.sound). Binary on odin: `~/sherlock/formal-e2e/conformance/lean/.lake/build/bin/oracle`.

Quint models must be written from the spec, not from this file; this file only fixes the wire
names that a replay projection maps to.

## Modes

- default: `{"protocol":"kink-implementation/1","inputs":{"config":INT,"ops":[INT,...]}}` ->
  `{"end":"final","cells":{...}}` or `{"end":"could-not-apply","reason":"not-an-operation|outside-state-domain"}`.
  Param `config`: dispatch + 2 preview + 4 kits, 0..31; dispatch = `T23_ALLOW_DISPATCH=1`, preview = `T23_ALLOW_PREVIEW=1`, kits bit 1 jubjub, 2 secp256k1, 4 keccak256 (the kits of the machine version).
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
| stage | int | 0 idle, 1 started, 2 native ref stored, 3 compiled (W equal), 4 proved, 5 settled, 6 done (classed) |
| callId, maxId | int | identity of the open call; highest identity taken (ECS-013) |
| starts, dupRefused | int | calls started; starts refused for a repeated identity |
| nativeKind | int | 0 none, 1 ok, 2 rejected, 3 pure (ECS-002) |
| transcriptWords | int | 1 when the reference holds transcript words (ok only) |
| refs | int | reference records stored |
| cls | int | class: 0 none, 1 pass, 2 refused-as-native, 3 mismatch, 4 blocked, 5 prove-failed, 6 n/a |
| reason | int | blocked reason: 1 R_SL chip set, 2 kit jubjub, 3 kit secp256k1, 4 kit keccak256, 5 dispatch off, 6 no native Preview transcript |
| derived | bool | refused-as-native row labelled DERIVED (ECS-009) |
| execRejected | bool | the executor ran the R_SL rejection |
| compileRan, proveRan, settleRan, compareRan | bool | stage ran for the open call (ECS-003) |
| verified | bool | stock and ledger verification ran |
| txIncluded | bool | the Preview tx was included |
| dispatch, preview | bool | the run switches (ECS-007, ECS-008) |
| kits | int | kit bits of the machine version (ECS-006) |
| boxMsgs | int | calls sent to a prover box |
| previewTx | int | Preview transactions submitted |
| evidence | int | evidence items of the open call, 0..7 (ECS-014) |
| nPass, nRefused, nMismatch, nBlocked, nProveFailed, nNa | int | table counts per class (ECS-004) |
| calls | int | calls classed |
| compared, mismatchCompared | int | calls that reached compare after a proof; mismatches among them (ECS-012) |
| eligibleIncomplete | int | eligible calls classed with fewer than 7 evidence items (ECS-014) |
| complete | bool | live run reported complete |
| completeBlocked | int | completion reports refused |

## Operations: `op = code * 1000000000 + arg`

| code | op | arg | enabled | EARS |
|---|---|---|---|---|
| 1 | start | identity >= 1 | stage 0 or 6 | ECS-013 |
| 2 | nativeRef | 1 ok, 2 rejected, 3 pure | stage 1 | ECS-002 |
| 3 | compile | wEq + 2 execRej + 4 kitReason (kitReason 0 none, 2, 3, 4) | stage 2 | ECS-003, 004, 006, 009 |
| 4 | prove | 0 ok, 1 failed, 2 no R_SL chip set | stage 3 | ECS-005, 007 |
| 5 | verify | stock + 2 ledger | stage 4, not yet verified | ECS-014 |
| 6 | settle | included (0/1) | stage 4 | ECS-008, 014 |
| 7 | compare | native + 2 eq | stage 5 | ECS-005, 010, 012 |
| 8 | reportComplete | 0 | stage 0 or 6 | ECS-010, 014 |

Evidence (ECS-014): prove ok +2 (proof, key-set identity), verify +stock +ledger, settle
+included (preview on), compare +included +1 (read-back and native comparison); 7 = all.

## Constants

Measured (`measured_*`, by `decide`): 36 = 33 + 3 programs (MANIFEST.json); 208 = 146 + 62
native refs (RESULTS.md:5); dry-run classes 0+62+0+145+0+1 = 208, blocked reasons
139+3+2+1 = 145, kit-blocked 3+2+1 = 6, 0 compared, 0 Preview tx (t23-suite@16836929b
T23-DRYRUN.md:3, 13-19, 21, 27-30); 106 = 105 + 1 circuits.

## LEAD PICKs in this oracle

- ECS-005: a call with no R_SL chip set is classed blocked (reason 1) before the dispatch
  check (reason 5): the first blocking stage names the reason.
- ECS-014: a class does not need all evidence; a call classed with fewer than 7 items counts
  in `eligibleIncomplete` and blocks the completion report. FAL-CONFORMANCE-1.

## Replay

```
python3 ~/Projects/software-factory/plugins/formal/scripts/quint_replay.py \
  --oracle-json '["/home/charl/sherlock/formal-e2e/conformance/lean/.lake/build/bin/oracle"]' \
  --traces <traces dir> --op-var lastOp --compare <cells>
```

`--op-var lastOp` works when the model's `lastOp` holds the integer op above; otherwise use a
projection adapter (`--projection-json '<adapter argv>'`, model state + action -> oracle state +
integer op), owned by the Quint lead.
