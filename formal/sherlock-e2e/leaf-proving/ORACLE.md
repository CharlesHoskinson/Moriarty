# Leaf Proving oracle: `--step` interface

Context: Leaf Proving (slice B, `research/sherlock-e2e-spec-20261115/bus/ddd-B.md`). Spec: `sherlock-e2e-leaf-proving` (LFP-001 to LFP-012); reused RSL-001 to RSL-008, CPR-001, CPR-002, CPR-004, CPR-005, SPL-006, LPC-018. Lean package: `formal/sherlock-e2e/leaf-proving/lean/` (namespace `LeafProving`). Coverage rows: `coverage.tsv`. Findings: `FINDINGS.md`.

## Build and run (odin)

```
formal/sherlock-e2e/scripts/odin-build.sh leaf-proving
ssh odin 'cd ~/sherlock/formal-e2e/leaf-proving/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step'
```

Replay:

```
quint_replay.py --oracle ssh odin 'cd ~/sherlock/formal-e2e/leaf-proving/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step' \
  --traces <dir> --op-var lastOp \
  --compare admitted,shardsProved,appProofs,leafProofs,leafProgD,foreignLeaves,badLeaves,records,recordLeaves,merged,steps
```

## Protocol

Same as `../key-management/ORACLE.md`: request `{"state": St, "op": N}`, answer `{"state"}`, `{"refused": [CODE..]}` (state unchanged) or `{"error"}`; `op = code * 16 + arg` (`arg < 16`); integers >= 0, booleans as JSON booleans, exactly the keys below. Default mode: `inputs` = every field plus `ops`; refused ops leave the state unchanged; `could-not-apply` / `outside-capability` only for an initial state outside the domain.

## State fields

| Field | Type | Meaning (requirement) |
|---|---|---|
| `appAirs` | nat | app AIRs without FP-1 (LFP-003, RSL-002) |
| `hasSha`, `hasKeccak` | bool | the program uses the SHA-256 kit / the Keccak-256 kit (LFP-003) |
| `appCols` | nat | columns of the app verifying key (LFP-004) |
| `nonCanonical` | bool | an ALU or Poseidon input word is not canonical (RSL-008) |
| `shapeOk` | bool | every leaf trace has the ordered height vector of the admitted shape (LFP-005) |
| `progD` | nat | `prog_d` of the run's program (LFP-006, LFP-011) |
| `inputD`, `tapeD` | nat | `input_d` of the leaf statement; digest of the tape the app consumed (LFP-012) |
| `stmtD`, `fp1Outcome` | nat | digest of `comm` and ordered W; outcome commitment in the FP-1 words (LFP-010) |
| `steps`, `shardCap` | nat, `shardCap > 0` | run length and shard length; shards = ceil(steps / shardCap) (LFP-008, CPR-002) |
| `keyRoot`, `manifestD` | nat | `key_root` and the universal key manifest id; no operation changes them (LFP-008) |
| `admitted` | bool | the shape was admitted (op 0) |
| `shardsProved`, `appProofs`, `leafProofs` | nat | proved shards and proofs (LFP-002) |
| `leafProgD` | nat | `PROG_D` slot of the last proved leaf (LFP-006) |
| `foreignLeaves`, `badLeaves` | nat | leaves that joined from op 2; those with another `prog_d` (LFP-007) |
| `records`, `recordLeaves` | nat | `LeafProofsReady` records; leaves the record names (LFP-009) |
| `merged` | bool | N1 merged the leaves (LFP-007) |

State domain (`Valid`, else `outside-capability`): `shardCap > 0`; `shardsProved <= shards`; `appProofs = leafProofs = shardsProved`; `badLeaves <= foreignLeaves`; not admitted implies `shardsProved = 0`; admitted implies the shape facts of op 0; `shardsProved != 0` implies `leafProgD = progD`; `records != 0` implies `recordLeaves = shards`; `merged` implies `badLeaves = 0` and `records != 0`.

## Operations

| code | op | arg | guard (refusal codes, all failing ones in this order) | effect |
|---|---|---|---|---|
| 0 | admit | 0 | `appAirs <= 8` (`air-count-over-bound`, LFP-003); not both kits (`mixed-hash`, LFP-003); `appAirs + 1` in {6, 9} (`outside-bucket`, RSL-002); `appCols <= 30000` (`column-bound`, LFP-004) | `admitted := true` |
| 1 | proveShard | 0 | `admitted` (`not-admitted`); `shardsProved < shards` (`all-shards-proved`, LFP-002); `shapeOk` (`shape-mismatch`, LFP-005); `tapeD = inputD` (`input-mismatch`, LFP-012); `fp1Outcome = stmtD` (`statement-unbound`, LFP-010); not `nonCanonical` (`noncanonical-word`, RSL-008) | `shardsProved`, `appProofs`, `leafProofs` += 1; `leafProgD := progD` |
| 2 | foreignLeaf p | p < 16 | not `merged` (`already-merged`); `p = progD` (`wrong-prog-d`, LFP-007: N1 refuses the leaf, D-leaf-2) | `foreignLeaves += 1` (a leaf of another program never joins, so `badLeaves` stays 0) |
| 3 | publish | 0 | `shardsProved = shards` (`leaves-incomplete`, LFP-009) | `records += 1`; `recordLeaves := shards` |
| 4 | n1Merge | 0 | `records != 0` (`not-published`); `badLeaves = 0` (`wrong-prog-d`, LFP-007) | `merged := true` |
| 5 | extend d | d < 16 | `shardsProved = 0` and `records = 0` (`run-started`, LFP-008) | `steps += (d + 1) * shardCap` |
| other | | | `outside-capability` | none |

## Suggested constants and fill

`--const appAirs=5 --const hasSha=false --const hasKeccak=false --const appCols=9000 --const nonCanonical=false --const shapeOk=true --const progD=7 --const inputD=11 --const tapeD=11 --const stmtD=13 --const fp1Outcome=13 --const shardCap=4 --const keyRoot=21 --const manifestD=1`; vary one constant per instance for the refusal cases.

## Open interpretation points (ledger)

| ID | Point | Status |
|---|---|---|
| LP-1 | RSL-002 is a SHALL-give property of the chip set; the oracle checks it at admission with the code `outside-bucket` (F-B-LFP-1). | held |
| LP-2 | LFP-010 and LFP-012 are AIR constraints ("refuse the trace/shard"); the oracle refuses the shard proof. | held |
| LP-3 | LFP-007: a leaf of another program is an input to N1 (op 2), not a proof this run makes. | held |
| LP-4 | The run length is fixed once proving starts (op 5 refused after op 1): LFP-008 compares two runs of different length. | held |
