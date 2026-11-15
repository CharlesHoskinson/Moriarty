# Wrap oracle: interface (Formal-Lean-AC)

Lean package `formal/sherlock-e2e/wrap/lean` (namespace `Wrap`), shared codec
`formal/sherlock-e2e/codec` (Lake `sherlock_codec`, NOT `codec-lean`). Built on odin with
`lake build --wfail` (Lean 4.35.0): 0 sorry, 69 theorems with pinned axioms
(`Wrap/Audit.lean`, allowlist propext / Classical.choice / Quot.sound), `#audit_ns Wrap` = `[]`.
Binary on odin: `~/sherlock/formal-e2e/wrap/lean/.lake/build/bin/oracle`.

Quint models are written from the spec, not from this file. This file only fixes the wire
names that a replay projection maps to.

## Modes

- default: `{"protocol":"kink-implementation/1","inputs":{"testSeed":0|1,"ops":[INT,...]}}` ->
  `{"end":"final","cells":{...}}` or `{"end":"could-not-apply","reason":"not-an-operation|outside-state-domain"}`.
- `--step`: JSON Lines `{"state":{<all cells>},"op":INT}` -> `{"state":{...}}`, or
  `{"refused":[CODE]}` (step domain only), or `{"error":TEXT}` (decode failure).
- `--fields`: the cell names and kinds, in order.

Every spec response (a stop, a refused deploy) is a state outcome in `stop` / `lastRefusal`,
never a `--step` refusal.

## Cells (all required, exact keys)

| cell | kind | meaning |
|---|---|---|
| phase | int | 0 idle, 1 sized (k <= 21), 2 costed (rows <= gate), 3 keyed (VK <= 50,000 B), 4 checked (key = E6_EXPECT_VK), 5 decoded, 6 written (one proof, three files), 7 stopped |
| k | int | requested circuit size |
| rows | int | cost-model rows at k |
| vkBytes | int | serialized wrap VK size |
| vkId | int | identity of the generated key (>= 1; 0 = none) |
| keygenRan | bool | keygen ran in this call |
| vkFileWritten | bool | `wrap.vk` written |
| files | int | output files written (0 or 3) |
| proofs | int | wrap proofs of this call |
| stop | int | 0 none, 1 `never k = 22` (WRP-002), 2 rows (WRP-003), 3 VK size (WRP-004), 4 key differs (WRP-008), 5 decode (WRP-007, ANC-013, ANC-016), 6 verifier refused (WRP-009) |
| lastRefusal | int | 0 none, 8 qualification incomplete (WRP-012), 9 second wrap key in the era (WRP-010), 10 GPU key differs (WRP-011) |
| anchorRead, chunkWrite | int | key constants in lanes 6 and 15 of the decoded call vector |
| mode | int | 0 ordinary (ORD), other = anchor modes |
| template | bool | the call carries the anchor-write template (lanes 11..19, 21) |
| testSeed | bool | build with the `test-seed` feature (ZK-003, ZK-014); fixed by init |
| draws | int | CSPRNG draws so far (era-wide) |
| lastBlinder, prevBlinder | int | blinder of the last two proofs (= draw number; 0 under test-seed) |
| qual | int | qualification record bits: 1 LW-003, 2 LW-004, 4 LW-005, 8 LW-006, 16 LW-014 |
| eraVk | int | the one wrap key of the era (0 = none deployed) |
| keysDeployed | int | distinct wrap keys deployed in the era |
| deployed | int | deployments in the era |
| gpuVk | int | key of the accepted GPU wrap run (0 = none) |

## Operations: `op = code * 1000000000 + arg`

| code | operation | arg | enabled when | EARS |
|---|---|---|---|---|
| 1 | askK | k | phase 0 | WRP-002, DG-001 |
| 2 | cost | rows | phase 1 | WRP-003, DG-002, RKY-003 |
| 3 | keygen | `vkId * 100000 + bytes` (vkId >= 1, bytes < 100000) | phase 2 | WRP-004, DG-003, RKY-004 |
| 4 | expect | 1 = key equals E6_EXPECT_VK, 0 = differs | phase 3 | WRP-008 |
| 5 | decode | `read + 100*write + 10000*mode + 100000*template` (read, write < 100; mode < 10; template 0/1) | phase 4 | WRP-007, ANC-013, ANC-016 |
| 6 | prove | 1 = library verifier accepts, 0 = refuses | phase 5 | WRP-001, WRP-006, WRP-009, LW-012, ZK-003 |
| 7 | qualify | check index 0..4 (LW-003, -004, -005, -006, -014) | always (arg in 0..4) | WRP-012 |
| 8 | deploy | 0 | phase 6 | WRP-010, WRP-012 |
| 9 | gpuProve | vkId the GPU run uses | phase 6 | WRP-011, GPW-001, GPW-003 |
| 10 | newCall | 0 | phase 6 or 7 | resets the per-call cells; era and CSPRNG cells stay |

Step-domain refusal codes: `not-an-operation` (code unknown or arg outside its encoding),
`op-not-enabled` (operation not enabled in the phase; the state is unchanged),
`outside-state-domain` (`validState` false: phase outside 0..7, negative counts, stop > 6,
lastRefusal > 10, qual > 31).

## Constants

kMax 21, rowGate 1,782,573 (= 85 % of 2^21 - 7, theorem `rowGate_derived`), vkMax 50,000,
anchorSlot 15, L_TX 1,024, public inputs 1,026, qualAll 31.

## Replay

```
python3 ~/Projects/software-factory/plugins/formal/scripts/quint_replay.py \
  --oracle-json '["/home/charl/sherlock/formal-e2e/wrap/lean/.lake/build/bin/oracle"]' \
  --traces <traces dir> --projection-json '<adapter argv>' --compare <cells>
```

The Quint model of this context keeps `lastOp: str` and its own vocabulary, so the replay
needs a projection adapter (model state + action -> oracle state + integer op), owned by the
Quint lead. `--op-var` works only if the model's op variable already holds the integer above.

## LEAD PICKs in this oracle

- WRP-002 and WRP-003 both say "before keygen"; the oracle orders the checks k, then rows,
  then keygen (VK size), then the key file (WRP-008), then decode, then prove. The spec gives
  no order between WRP-008 and WRP-007 decode; the oracle checks the key file first because
  WRP-008 says "before writing an output file" and decode belongs to proving.
- WRP-010 is enforced at deploy (a second key in an era is refused, `lastRefusal = 9`).
- WRP-012 is enforced at deploy (record incomplete -> `lastRefusal = 8`, no deploy).
