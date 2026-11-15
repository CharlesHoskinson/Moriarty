# Key Management oracle: `--step` interface

Context: Key Management (slice B, `research/sherlock-e2e-spec-20261115/bus/ddd-B.md`). Specs: `sherlock-e2e-universal-keys` (UKS-001 to UKS-010); reused RKY-001, RKY-002, RKY-005, CPR-003, SPL-004, SPL-005. Lean package: `formal/sherlock-e2e/key-management/lean/` (namespace `KeyManagement`). Coverage rows: `coverage.tsv`.

## Build and run (odin)

```
formal/sherlock-e2e/scripts/odin-build.sh key-management      # rsync to odin:~/sherlock/formal-e2e, lake build --wfail
ssh odin 'cd ~/sherlock/formal-e2e/key-management/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step'
```

Replay command (from `~/Projects/software-factory/plugins/formal/examples/token-bucket/scripts/quint_replay.py`):

```
quint_replay.py --oracle ssh odin 'cd ~/sherlock/formal-e2e/key-management/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step' \
  --traces <dir> --op-var lastOp \
  --compare manifestD,manifestListD,keygens,phase,gatesPass,qualified,box0,box1,box2,proofs,keyRoot,programs
```

## Protocol

- Request: `{"state": St, "op": N}`. Answer: `{"state": St}`, `{"refused": [CODE, ...]}` (state unchanged) or `{"error": TEXT}` (decode failure).
- `op = code * 16 + arg`, with `arg < 16`. The Quint model keeps the last operation in a state variable (`lastOp`) in this encoding.
- Integers are JSON integers >= 0 (or canonical decimal strings). Booleans are JSON booleans. The state object has exactly the keys below.
- A refused operation is a disagreement when the model took it: write the model so that an action fires only when its guard holds, and check the refusals with the default mode or the pins.
- Default mode (`kink-implementation/1`): `inputs` = every state field plus `ops` (a list of op codes). The oracle applies each op in order; a refused op leaves the state unchanged (the same as `--step`). Answer `{"end":"final","cells":St}`, or `could-not-apply` with `outside-capability` when the initial state is outside the domain.

## State fields

| Field | Type | Meaning (requirement) |
|---|---|---|
| `listRows` | nat, bits 0..5 | rows of `LIST.md` among the six of UKS-010: bit 0 FP-3, 1 anchor slot 15, 2 range checks, 3 R_SL statement bindings, 4 R_SL key leaf index, 5 era-check words; all = 63 |
| `rowsCommitted` | bool | every row has a commit on its branch (RKY-002) |
| `treeHoldsAll` | bool | keygen tree descends from every listed commit (UKS-009) |
| `rslKeyIdx`, `keydagIdx` | nat | R_SL key leaf index, path C DAG leaf index `KEYDAG_IDX` (UKS-003) |
| `listD` | nat | sha256 of `LIST.md` now (abstract) |
| `manifestD` | nat | id of the universal key manifest; 0 = none (UKS-004) |
| `manifestListD` | nat | `LIST.md` sha256 recorded in the manifest (UKS-004) |
| `keygens` | nat | keygens so far (RKY-001) |
| `phase` | nat 0..2 | 0 no keys, 1 keys made and gates pending, 2 rekey DONE (UKS-006) |
| `gatesPass` | nat, bits 0..11 | gates of UKS-006 passed on the current keys, in the spec's order (check-cases, V.9 omit, V.10 check-chain, pathc, pathc-d3, gp2, x7, d3work, iface-fixture, lfr-e3, i7-decode, sg6); all = 4095 |
| `d3MinBits`, `d3MinGrindBits` | nat | weakest D3 term and weakest grindable term in bits (RKY-005) |
| `qualified` | bool | manifest status: false = `prototype`, true = `qualified` (UKS-007) |
| `box0`, `box1`, `box2` | nat | manifest id held by muninn, huginn, odin; 0 = none (UKS-005) |
| `proofs` | nat | proof jobs started (UKS-005) |
| `setupRuns` | nat | setup runs (UKS-008) |
| `keyRoot` | nat | `key_root` of the per-program keys (SPL-004) |
| `programs` | nat | programs planned (CPR-003) |

State domain (`Valid`, else `outside-capability`): `phase <= 2`; `manifestD <= keygens`; `phase = 0` iff `manifestD = 0`; each box id `<= manifestD`; `qualified` implies `phase = 2`; `phase = 2` implies `gatesPass = 4095`.

## Operations

| code | op | arg | guard (refusal codes) | effect |
|---|---|---|---|---|
| 0 | keygen | 0 | `listRows = 63` (`list-row-missing`, UKS-010); `rowsCommitted` (`row-uncommitted`, RKY-002); `treeHoldsAll` (`tree-not-descendant`, UKS-009); `rslKeyIdx != keydagIdx` (`key-leaf-collision`, UKS-003); not (`manifestD != 0` and `manifestListD = listD`) (`keys-current`, RKY-001). All failing codes, in this order | `keygens += 1`, `manifestD := keygens + 1` (old value + 1), `manifestListD := listD`, `phase := 1`, `gatesPass := 0`, `qualified := false` |
| 1 | gatePass g | g < 12 | `phase = 1` (`wrong-phase`) | `gatesPass := gatesPass OR 2^g` |
| 2 | complete | 0 | `phase = 1` (`wrong-phase`); `gatesPass = 4095` (`gates-incomplete`); `d3MinBits >= 100` and `d3MinGrindBits >= 101` (`d3-below-bound`) | `phase := 2` |
| 3 | distribute b | b < 3 | `phase = 2` (`wrong-phase`) | box b := `manifestD` |
| 4 | startProof b | b < 3 | `phase = 2` (`wrong-phase`); box b = `manifestD` (`manifest-mismatch`) | `proofs += 1` |
| 5 | setup b | b < 3 | none | `setupRuns += 1` |
| 6 | addProgram p | p < 16 | none | `keyRoot := keyRoot * 16 + p + 1`, `programs += 1` |
| 7 | securityGate | 0 | `phase = 2` (`wrong-phase`) | `qualified := true` |
| 8 | changeList d | d < 16 | none | `listD := listD + d + 1` |
| other | | | `outside-capability` | none |

## Suggested constants and fill

`--const rslKeyIdx=4 --const keydagIdx=3 --const d3MinBits=101 --const d3MinGrindBits=101` for a passing instance; `listRows`, `rowsCommitted`, `treeHoldsAll` may be model variables or constants. Fields the model leaves out: `--fill setupRuns=0,proofs=0`.

## Open interpretation points (ledger)

| ID | Point | Status |
|---|---|---|
| KM-1 | UKS-003 says the generator SHALL give a distinct index. The oracle refuses the keygen when the indexes are equal (the generator is not modelled as choosing). | held |
| KM-2 | RKY-001 "once": a second keygen is allowed only after `LIST.md` changes (UKS-004 scenario "a second keygen is needed"). | held |
| KM-3 | UKS-007: the security gate is one event (op 7) on DONE keys; G1 to G5 and FP-8 are in the Assurance oracle. | held |
