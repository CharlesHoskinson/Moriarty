# Bridge oracle: `--step` interface

Context: Bridge (slice B, `research/sherlock-e2e-spec-20261115/bus/ddd-B.md`). Specs: `sherlock-e2e-l-transcript` (LTR-001 to LTR-008), `sherlock-e2e-call-binding` (CLB-001 to CLB-008); reused LW-001 to LW-022, FLG-001 to FLG-009, HDM-001 to HDM-010, ANC-001 to ANC-011 and ANC-013 to ANC-019. Lean package: `formal/sherlock-e2e/bridge/lean/` (namespace `Bridge`). Coverage rows: `coverage.tsv`. Findings: `FINDINGS.md`.

## Build and run (odin)

```
formal/sherlock-e2e/scripts/odin-build.sh bridge
ssh odin 'cd ~/sherlock/formal-e2e/bridge/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step'
```

Replay:

```
quint_replay.py --oracle ssh odin 'cd ~/sherlock/formal-e2e/bridge/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step' \
  --traces <dir> --op-var lastOp \
  --compare lKeyId,planned,phase,keyPathC,keyFull,verifierRefused,records,wraps,recordSha,movedSha,proofSha
```

## Protocol

Same as `../key-management/ORACLE.md`: request `{"state": St, "op": N}`, answer `{"state"}`, `{"refused": [CODE..]}` (state unchanged) or `{"error"}`; `op = code * 16 + arg`; integers >= 0; exact keys. Default mode: `inputs` = every state field plus `ops`; a refused op is a no-op.

## State fields

| Field | Type | Meaning (requirement) |
|---|---|---|
| `lCols`, `lInter` | nat | columns of the `L` key, largest interaction count of one AIR (LTR-003) |
| `slotIface`, `slotWrap` | nat | anchor slot constant of the `L_Fr` interface and of the wrap decoder (CLB-004, CLB-008) |
| `lKeyId` | nat | id of the `L` key, 0 = none (LTR-007) |
| `keygens` | nat | keygens so far |
| `mode` | nat 0..1 | 0 ordinary call (ORD), 1 chunked run (ANC-003) |
| `k`, `slices` | nat | stated chunk count, slice count of the run (CLB-003, CLB-007) |
| `laneOk` | bool | every effect lane holds the guest's word (CLB-002, ANC-018) |
| `tplWrite` | bool | the call carries the anchor-write template in lanes 11 to 19 and 21 (CLB-006, ANC-016) |
| `lockOk` | bool | every lock operand `x_j` is the run's (CLB-005, ANC-017) |
| `tapeExposed` | bool | public words through `Tape::expose` (LTR-004) |
| `tapeTyped` | bool | every tape operand well typed (LW-021) |
| `transcriptOk` | bool | the tape replays the stock `L` transcript (LTR-005) |
| `stockAccepts` | bool | the stock verifier accepts the `L_Fr` proof (LTR-008) |
| `planned` | bool | the chunk planner admitted the call (CLB-007) |
| `phase` | nat 0..4 | 0 not proved, 1 `L` proved, 2 `L_Fr` proved, 3 verified, 4 record published |
| `keyPathC`, `keyFull` | nat | `L` key id used on path C, on the full tree; 0 = not yet (LTR-007) |
| `verifierRefused` | bool | the stock verifier refused the proof (LTR-008) |
| `proofSha`, `recordSha`, `movedSha` | nat | sha256 of the proof bytes, the record's `proof_sha256`, the bytes on muninn (LTR-006) |
| `records`, `wraps` | nat | `LfrProofReady` records, wrap starts |

Witness fields (`lCols` to `stockAccepts` except the ids) never change; choose them per trace (`--const`).

State domain (`Valid`, else `outside-capability`): `phase <= 4`; `mode <= 1`; `lKeyId <= keygens`; a key exists only within bounds (`lCols <= 2000`, `lInter <= 100`) and with both slots 15; `planned` implies a key and (`mode = 1` implies `k <= 2`); `phase >= 1` implies `planned`, `laneOk`, (`mode = 1` implies `k = slices`), (`mode = 0` implies not `tplWrite`); `phase >= 2` implies `tapeExposed`, `tapeTyped`, `transcriptOk`, (`mode = 1` implies `lockOk`); `phase >= 3` implies `stockAccepts`; `records != 0` implies `phase = 4`, `recordSha = proofSha`, `stockAccepts`; `phase = 4` implies `records = 1`; `wraps != 0` implies `records != 0` and `movedSha = recordSha`; a nonzero `keyPathC` or `keyFull` equals `lKeyId`.

## Operations

| code | op | arg | guard (refusal codes, in this order) | effect |
|---|---|---|---|---|
| 0 | keygenL | 0 | `lCols <= 2000` and `lInter <= 100` (`l-key-bounds`, LTR-003); `slotIface = 15` (`anchor-slot`, CLB-004); `slotIface = slotWrap` (`slot-mismatch`, CLB-008); `lKeyId = 0` (`keys-exist`, LTR-007) | `keygens += 1`, `lKeyId := keygens + 1` |
| 1 | plan | 0 | `lKeyId != 0` (`no-key`); not `planned` (`wrong-phase`); not (`mode = 1` and `k > 2`) (`k-max`, CLB-007) | `planned := true` |
| 2 | proveL f | f < 2 (0 path C, 1 full tree) | `planned` and `phase = 0` (`wrong-phase`); `laneOk` (`lane-unbound`, CLB-002); not (`mode = 1` and `k != slices`) (`k-mismatch`, CLB-003); not (`mode = 0` and `tplWrite`) (`ordinary-anchor-write`, CLB-006) | `phase := 1`; `keyPathC := lKeyId` (f = 0) or `keyFull := lKeyId` (f = 1) |
| 3 | proveLfr | 0 | `phase = 1` (`wrong-phase`); `tapeExposed` (`tape-not-exposed`, LTR-004); `tapeTyped` (`ill-typed-tape`, LW-021); `transcriptOk` (`transcript-mismatch`, LTR-005); not (`mode = 1` and not `lockOk`) (`lock-mismatch`, CLB-005) | `phase := 2`, `proofSha += 1` |
| 4 | stockVerify | 0 | `phase = 2` and not `verifierRefused` (`wrong-phase`) | `stockAccepts`: `phase := 3`; else `verifierRefused := true` (LTR-008) |
| 5 | publish | 0 | `phase = 3` (`not-verified`, LTR-008) | `phase := 4`, `records += 1`, `recordSha := proofSha` (LTR-006) |
| 6 | transfer c | c < 2 | `records != 0` and `wraps = 0` (`wrong-phase`) | `movedSha := recordSha + c` (c = 1: one changed byte) |
| 7 | wrapStart | 0 | `records != 0` (`wrong-phase`); `movedSha = recordSha` (`digest-mismatch`, LTR-006) | `wraps += 1` |
| other | | | `outside-capability` | none |

## Suggested constants and fill

Honest chunked instance: `--const lCols=1995 --const lInter=99 --const slotIface=15 --const slotWrap=15 --const mode=1 --const k=2 --const slices=2` and every witness boolean true except `tplWrite=false`. Mutant instances flip one witness field. Fields the model leaves out: `--fill keygens=0,verifierRefused=false`.

## Open interpretation points (ledger)

| ID | Point | Status |
|---|---|---|
| BR-1 | CLB-004 and CLB-008 are checked at the `L` keygen (the rekey runner start); the spec places CLB-008 at the rekey runner and CLB-004 in the interface. | held |
| BR-2 | LTR-003 bound is the spec's 2,000 columns, not the code's 2,200 (F-BR-1). The oracle follows the spec text. | held, see F-BR-1 |
| BR-3 | CLB-005: the oracle models the lock check as one boolean `lockOk`; the operand list (6 or 7 operands) is F-BR-2. | held, see F-BR-2 |
| BR-4 | CLB-006 applies to mode 0 only; a chunk call with the write template is accepted (ANC-003). | held |
| BR-5 | LTR-006 transfer is allowed only before the wrap starts (a moved file after the wrap start is a new hand-off). | held |
