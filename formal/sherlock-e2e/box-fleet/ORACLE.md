# Box Fleet oracle: `--step` interface

Context: Box Fleet (slice B, `research/sherlock-e2e-spec-20261115/bus/ddd-B.md`). Specs: `sherlock-e2e-box-fleet` (BXF-001 to BXF-014), `sherlock-e2e-gpu-proving` (GPV-001 to GPV-007), `sherlock-e2e-prover-daemon` (PDM-001 to PDM-009); reused PRD-001 to PRD-005, GPW-007, LPC-005, LPC-007. Lean package: `formal/sherlock-e2e/box-fleet/lean/` (namespace `BoxFleet`). Coverage rows: `coverage.tsv`. Findings: `FINDINGS.md`.

## Build and run (odin)

```
formal/sherlock-e2e/scripts/odin-build.sh box-fleet
ssh odin 'cd ~/sherlock/formal-e2e/box-fleet/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step'
```

Replay:

```
quint_replay.py --oracle ssh odin 'cd ~/sherlock/formal-e2e/box-fleet/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step' \
  --traces <dir> --op-var lastOp --compare <fields the model has>
```

## Protocol

Same as `key-management/ORACLE.md`: request `{"state": St, "op": N}`, answer `{"state": St}`, `{"refused": [CODE, ...]}` (state unchanged) or `{"error": TEXT}`. `op = code * 16 + arg` (`arg < 16`). Naturals are JSON integers >= 0; booleans are JSON booleans; the state object has exactly the 43 keys below. Default mode: `inputs` = every state field plus `ops`; a refused op leaves the state unchanged.

## State fields

| Field | Type | Meaning (requirement) |
|---|---|---|
| `roster` | nat, bits 0..3 | owned-box roster: bit 0 muninn, 1 huginn, 2 odin, 3 a fourth box (BXF-001) |
| `slotsUsed` | nat | shared heavy-slot jobs on muninn (BXF-002, BXF-003) |
| `exclRunning` | bool | an exclusive heavy-slot job runs (BXF-003) |
| `remoteJobs` | nat | jobs started on boxes other than muninn |
| `gpuHolder` | nat | `gpu.lock` holder: 0 none, 1 prover, 2 `wrapd`, 3 eligible orphan (sampler or monitor of an ended job whose parent is gone), 4 or more other (BXF-004 to BXF-006) |
| `idleMin` | nat | minutes the holder made no progress (BXF-005) |
| `termSent`, `termAgeS` | bool, nat | SIGTERM sent; seconds since (BXF-013) |
| `busLog` | nat | lines of `~/.cache/box-bus/all.log` (BXF-005, BXF-008, BXF-013) |
| `shaOk` | bool | input: the moving file's sha256 equals the bus message (BXF-007) |
| `transfers` | nat | accepted transfers |
| `ownerAway` | bool | input (BXF-010) |
| `q1`, `q2` | nat | queued jobs of huginn and odin (BXF-010) |
| `daemonUp`, `socketMode`, `keysLoaded` | bool, nat, bool | `wrapd serve` runs; socket mode (384 = 0o600); keys, SRS, tables, CUDA context loaded (PDM-001, PDM-002, PDM-004) |
| `daemonManifest`, `reqManifest` | nat | key manifest loaded; manifest the next request names (PDM-006) |
| `reqPeerOk` | bool | input: the next request is from a peer of the same user and machine (PRD-001) |
| `reqLeaves` | nat | input: leaf count of the next `call` (PDM-007) |
| `served`, `errors`, `statsReplies`, `statsWitness` | nat | proofs served; error answers (PDM-003); stats answers and witness words in them (PDM-008) |
| `poolFree`, `slotsMade`, `slotsSpent` | nat | blinding slots free, made, consumed (PRD-002, PDM-005, PDM-009) |
| `lastPath` | nat | 0 none, 1 full tree, 3 path C (PDM-007, PRD-004) |
| `cuda`, `nodeEnv`, `lEnv` | bool, nat, nat | build feature `cuda`; `T7_NODE_GPU` 0 unset, 1 `0`, 2 `off`, 3 other; `T14_L_GPU` 0 unset, 1 `off`, 2 other |
| `shardsProved`, `runStopped`, `stopShard`, `stopDevice` | nat, bool, nat, nat | GPV-006 |
| `lastAppDev`, `lastNodeDev`, `lastLDev` | nat | device of the last shard's app and leaf layer, nodes, `L`: 0 CPU, 1 GPU |
| `runsStarted`, `unitRuns` | nat | runs started, as systemd units (BXF-009) |
| `claims`, `gpuQueries` | nat | speed claims (BXF-011), GPU data reads (BXF-012) |

State domain (`Valid`, else `outside-capability`): `slotsUsed <= 3`; `exclRunning` implies `slotsUsed = 0`; `termSent` implies `gpuHolder = 3`; `daemonUp` implies `socketMode = 384` and `keysLoaded`; `poolFree + slotsSpent = slotsMade`; `served = slotsSpent`; `statsWitness = 0`; `runStopped` implies `stopDevice = 1`; `unitRuns <= runsStarted`.

## Operations

| code | op | arg | refusal codes (in this order) | effect |
|---|---|---|---|---|
| 0 | submitJob h | h < 4 | `host-not-in-roster` (bit h of roster clear, BXF-001); for h = 0: `no-slot` (exclusive running or 3 slots used, BXF-002) | h = 0: `slotsUsed += 1`; else `remoteJobs += 1` |
| 1 | sharedJob m | m (8 m GB) | `over-budget` (8 m > 64, BXF-003), `no-slot` | `slotsUsed += 1` |
| 2 | exclusiveJob | 0 | `no-slot` (exclusive running or any slot used) | `exclRunning := true` |
| 3 | endShared | 0 | `no-job` | `slotsUsed -= 1` |
| 4 | endExclusive | 0 | `no-job` | `exclRunning := false` |
| 5 | addBox h | h < 4 | none | set bit h of `roster` |
| 6 | gpuAcquire k | 1..4 | `gpu-locked` (holder != 0, BXF-004) | holder := k; `idleMin`, `termSent`, `termAgeS` reset |
| 7 | gpuRelease | 0 | `no-holder` | holder := 0, reset |
| 8 | idle m | m | `no-holder` | `idleMin += m` |
| 9 | sigterm | 0 | `holder-protected` (holder 1 or 2, BXF-006); `not-eligible` (holder != 3 or idleMin <= 10, BXF-005); `already-signalled` | `busLog += 1`, `termSent := true`, `termAgeS := 0` |
| 10 | wait t | t | none | `termAgeS += 10 t` |
| 11 | sigkill | 0 | `holder-protected`; `not-eligible` (holder != 3, no SIGTERM, or termAgeS < 30, BXF-013) | holder := 0, reset, `busLog += 1` |
| 12 | transfer a | src = a / 4 != dst = a % 4 | `not-via-muninn` (src and dst both != 0, BXF-014), `sha256-mismatch` (not shaOk, BXF-007) | `transfers += 1` |
| 13 | outbox b | b < 4 | none | `busLog += 1` (BXF-008) |
| 14 | linkDrop b | 1 or 2 | none | if `ownerAway`: queue of b moves to the other box (BXF-010) |
| 15 | queueJob b | 1 or 2 | none | `q_b += 1` |
| 16 | daemonStart m | m | `daemon-running` | up, `socketMode := 384`, keys loaded, `daemonManifest := m` |
| 17 | request k | 0 prove, 1 call, 2 stats, 3 quit, 4..15 other | `daemon-down`; `peer-refused` (PRD-001); for k <= 1: `manifest-mismatch` (PDM-006), `no-blinding-slot` (PRD-002) | k <= 1: `served += 1`, `poolFree -= 1`, `slotsSpent += 1`; k = 1 also `lastPath := 3 if reqLeaves = 1 else 1`; k = 2: `statsReplies += 1`; k = 3: down, keys unloaded; else `errors += 1` (PDM-003) |
| 18 | makeSlot o | 0 daemon, 1 helper | `daemon-down`, `helper-slot` (o = 1, PRD-002) | `poolFree += 1`, `slotsMade += 1` |
| 19 | exportSlot | 0 | always `slot-off-machine` (PRD-002, PRD-003) | none |
| 20 | proveShard f | 0 ok, 1 device error | `run-stopped` (GPV-006); f = 1 without cuda: `outside-capability` | f = 1: `runStopped`, `stopShard := shardsProved`, `stopDevice := 1`; else `shardsProved += 1` and the three device fields from `cuda`, `nodeEnv`, `lEnv` |
| 21 | startRun a | minutes 10 (a / 2), unit a % 2 | `needs-unit` (minutes > 30 and not a unit, BXF-009) | `runsStarted += 1`, `unitRuns += unit` |
| 22 | speedClaim a | box a / 4 vs box a % 4 | `cross-box-claim` (BXF-011) | `claims += 1` |
| 23 | gpuQuery q | 0 query-gpu, 1 CUDA memory, 2 conf-compute | `conf-compute-forbidden` (q = 2, BXF-012) | `gpuQueries += 1` |
| other | | | `outside-capability` | none |

Every refusal is preceded by `outside-capability` when the state is outside `Valid`.

## Suggested constants and fill

`--const roster=7 --const cuda=true --const nodeEnv=0 --const lEnv=0`; inputs the model may vary: `shaOk`, `ownerAway`, `reqManifest`, `reqPeerOk`, `reqLeaves`. Fill the fields a sub-model does not have with the `init` values (all 0 or false, `shaOk`/`reqPeerOk` true, `reqLeaves` 1).

## Open interpretation points (ledger)

| ID | Point | Status |
|---|---|---|
| BF-1 | BXF-002/003: heavy-slot WAITS when no slot is free; the oracle returns `no-slot` (not admitted now). | held |
| BF-2 | BXF-014 names huginn and odin; the oracle refuses any transfer between two non-muninn boxes, including a fourth roster box (the factory-fleet rule "files move only through muninn"). | held |
| BF-3 | PDM-005 (draw from the OS CSPRNG per request) and PRD-002 (pool of one-use slots) are modelled together: a request consumes one daemon-made slot. D-040 keeps the pool TO BUILD; the code today draws per proof (serve.rs:117), which is the pool of size one per request. | held |
| BF-4 | PDM-009 distribution equality is a zero-knowledge statement; the oracle checks only that no slot serves two proofs. | held |
| BF-5 | One `gpu.lock` modelled; holder kind 3 encodes the BXF-005 eligibility (sampler or monitor, ended job, parent gone) as one input. | held |
