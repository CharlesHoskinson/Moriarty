# CompactComp oracle: interface (Formal-Lean-AC)

Lean package `formal/sherlock-e2e/compact-compilation/lean` (namespace `CompactComp`), shared codec
`formal/sherlock-e2e/codec` (Lake `sherlock_codec`). Built on odin with `lake build --wfail`
(Lean 4.35.0): 0 sorry, 55 theorems with pinned axioms (`CompactComp/Audit.lean`, allowlist propext /
Classical.choice / Quot.sound). Binary on odin: `~/sherlock/formal-e2e/compact-compilation/lean/.lake/build/bin/oracle`.

Quint models must be written from the spec, not from this file; this file only fixes the wire
names that a replay projection maps to.

## Modes

- default: `{"protocol":"kink-implementation/1","inputs":{"deploy":INT,"ops":[INT,...]}}` ->
  `{"end":"final","cells":{...}}` or `{"end":"could-not-apply","reason":"not-an-operation|outside-state-domain"}`.
  Param `deploy`: 0 = T18 (c) spike profile (anchor cell 0), 1 = deploy profile (anchor slot 15).
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
| phase | int | 0 open (circuits being read), 1 ended with exit 0, 2 ended with a non-zero exit |
| deployProfile | bool | the deploy profile is selected (ECC-009) |
| anchorSlot | int | 15 (deploy) or 0 (spike) (ECC-009, MYC-SEAL-005) |
| requested | int | circuits read in this request |
| accepted | int | circuits admitted (image + ABI at finish) |
| refused | int | circuits refused in this request |
| lastRefusal | int | refusal code of the last refused circuit (0 none, table below) |
| images, abis | int | files written at finish (ECC-003, ECC-012) |
| exitCode | int | 0 not ended, 1 = exit code 0, 2 = non-zero exit |
| keccakChip | int | keccak256 chip instructions emitted (ECC-007) |
| tier3 | int | keccak256 calls sent to a tier 3 kit (always 0) |
| jubPoints, jubScalars | int | Jubjub point / scalar inputs admitted (ECC-014) |
| tapeWords | int | tape words for Jubjub inputs (2 per point, 1 per scalar) |
| curveChecks | int | on-curve and subgroup checks (1 per point) |
| secpKit | int | secp256k1 inputs lowered to the RSL-005 kit (CF-013) |
| img1Src, img1Hash, img2Src, img2Hash | int | the last two serializations: source and bytes (ECC-004) |
| gateBits | int | bit mask of passed gates 0..5 (63 = all) |
| gateFailed | bool | a gate failed (sticky) |
| failedGate | int | 1 + index of the failed gate, 0 none |
| settleUses | int | settled calls that used the compiler |
| useBits | int | gateBits at the last settled use |
| settleBlocked | int | settled uses refused because a gate failed (ECC-002) |

Refusal codes in `lastRefusal`: 1 carrier (not stock compactc 0.36.0 output; ECC-001,
MYC-REF-032), 2 IR version other than 3.1 (MYC-FE-002, MYC-REF-032), 3 input type with no
R_SL kit (CF-014), 4 shape outside the 13 `SInstr` shapes (ECC-005), 5 layout uses
`anchor_slot` (MYC-SEAL-021).

## Operations: `op = code * 1000000000 + arg`

| code | op | arg | enabled | EARS |
|---|---|---|---|---|
| 1 | admit (read one circuit) | stock + 10 irMinor + 100 kind + 1000 shapeOk + 10000 slotClash + 100000 keccak; booleans 0/1, irMinor 0..9, kind 0 field, 1 Jubjub point, 2 Jubjub scalar, 3 secp256k1, 4 no kit | phase = 0 | ECC-001, 005, 007, 010, 014, CF-013, CF-014, MYC-FE-001/002, MYC-REF-032, MYC-SEAL-021, MYC-VAL-014, MYC-BLT-007 |
| 2 | finish | 0 | phase = 0 and requested >= 1 | ECC-003, 012, 013, MYC-FE-011 |
| 3 | serialize | src * 1000 + tape (tape < 1000) | always | ECC-004, MYC-FE-009, MYC-FE-010 |
| 4 | gate | 2 i + ok, i 0..5 (KRS emit, oracle-equal, golden, coverage, mutations, compare) | always | ECC-002, ECC-011, LPC-013 |
| 5 | useForSettle | 0 | always | ECC-002 |
| 6 | newRequest | 0 | phase != 0 | (request reset) |

## Constants

deploy slot 15, spike cell 0, 13 instruction shapes, 6 gates (mask 63). Measured
(`measured_*`, by `decide`): 106 = 105 + 1 circuits; 208 = 139 + 62 + 1 + 6 + 0 calls
(mycroft/factory/REPORT.md:8-9; see FAL-COMPACT-COMPILATION-2 for the newer 142/3 split); gates 58 + 4 = 62
mutations, 104 + 103 = 207 compare; oracle-equal 104/104, Quint 1,226/1,226.

## LEAD PICKs in this oracle

- ECC-010: a circuit with several causes gets exactly one code; the specs give no precedence.
  The oracle takes the pipeline order read (1), version (2), types (3), lowering (4), seal (5).
  FAL-COMPACT-COMPILATION-4.
- ECC-013: a refused request writes no image for any of its circuits (no partial output),
  as ECC-013 says "write no image file for the request". Marked for T15.

## Replay

```
python3 ~/Projects/software-factory/plugins/formal/scripts/quint_replay.py \
  --oracle-json '["/home/charl/sherlock/formal-e2e/compact-compilation/lean/.lake/build/bin/oracle"]' \
  --traces <traces dir> --op-var lastOp --compare <cells>
```

`--op-var lastOp` works when the model's `lastOp` holds the integer op above; otherwise use a
projection adapter (`--projection-json '<adapter argv>'`, model state + action -> oracle state +
integer op), owned by the Quint lead.
