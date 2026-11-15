# Scheduling oracle: `--step` interface

Context: Scheduling (slice B, `research/sherlock-e2e-spec-20261115/bus/ddd-B.md`). Specs: `sherlock-e2e-setup-mode` (STM-001 to STM-019), `sherlock-e2e-elastic-proving` (ELA-001 to ELA-007); reused SPL-001 to SPL-003, SPL-007 to SPL-012 (processor-final), LPC-006 (completion). Lean package: `formal/sherlock-e2e/scheduling/lean/` (namespace `Scheduling`). Coverage rows: `coverage.tsv`. Findings: `FINDINGS.md` (none).

## Build and run (odin)

```
formal/sherlock-e2e/scripts/odin-build.sh scheduling      # rsync to odin:~/sherlock/formal-e2e, lake build --wfail
ssh odin 'cd ~/sherlock/formal-e2e/scheduling/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step'
```

Replay command (`~/Projects/software-factory/plugins/formal/examples/token-bucket/scripts/quint_replay.py`):

```
quint_replay.py --oracle ssh odin 'cd ~/sherlock/formal-e2e/scheduling/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step' \
  --traces <dir> --op-var lastOp \
  --compare profWritten,profKeyD,memBudget,unified,gpuCount,calMask,setupStops,limMemOn,limMem,gpusOff,limGpuMemOn,limGpuMem,salt,box,scheduled,calibrated,schShape,schDevice,schConc,schEst,cut,forced,policy,earlyWraps,runs,recBox,recShape,recDevice,recLeaves,recCalibrated,recKeys,recClaim,nodes
```

## Protocol

- Request: `{"state": St, "op": N}`. Answer: `{"state": St}`, `{"refused": [CODE, ...]}` (state unchanged) or `{"error": TEXT}` (decode failure).
- `op = code * 16 + arg`, with `arg < 16`. The Quint model keeps the last operation in `lastOp` in this encoding.
- Integers are JSON integers >= 0 (or canonical decimal strings), booleans JSON booleans, lists JSON arrays of integers. The state object has exactly the 61 keys below.
- A refused operation is a disagreement when the model took it: an action fires only when its guard holds.
- Default mode (`kink-implementation/1`): `inputs` = every state field plus `ops`. Ops apply in order; a refused op leaves the state unchanged. `could-not-apply` with `outside-capability` when the initial state is outside the domain.

## Units and fixed shapes

Memory in any byte unit (the pins use MB), time in ms, digests natural numbers. Two leaf shapes (index 0, 1) and two devices (0 = GPU, 1 = CPU). A calibration entry is `e = shape * 2 + device`; `calT`, `calHost`, `calDev` have 4 entries, `caps` 2. One GPU per box.

## State fields

| Field | Type | Meaning (requirement) |
|---|---|---|
| `memAvail`, `cgroupHead` | nat | `MemAvailable`, cgroup headroom of the prover (0 = no cgroup limit) (STM-003) |
| `hasGpu`, `gpuReportsMem` | bool | the box has a usable GPU; the GPU reports own memory (STM-001, STM-002) |
| `hwGpuTotal`, `hwGpuFree` | nat | VRAM total and free as `nvidia-smi` reports them |
| `cores` | nat | usable cores |
| `proverKeyD` | nat | the prover's universal key set (`key_root`, `keys_d`) (STM-008, ELA-002) |
| `caps` | [nat; 2] | steps per leaf `S` of each shape of the universal key set (STM-013) |
| `calT`, `calHost`, `calDev` | [nat; 4] | per entry: leaf time, host bytes, device bytes of one full leaf (STM-004) |
| `profWritten`, `profFormatOk`, `profFieldsOk` | bool | the profile exists; `format` is `sherlock-profile/1`; all 11 fields (STM-006, STM-007, STM-016) |
| `profKeyD` | nat | key set the profile was made with (STM-008) |
| `memBudget` | nat | profile memory budget (STM-003) |
| `unified`, `gpuCount`, `gpuTotal`, `gpuFree`, `profCores` | bool, nat | profile GPU and CPU fields (`gpuTotal`, `gpuFree` = 0 for a unified GPU) |
| `calMask` | nat 0..15 | bit e set: the profile has calibration entry e |
| `setupStops` | nat | setup runs stopped by a refused calibration proof (STM-005) |
| `memUnit` | nat | byte unit of the limit operations (constant) |
| `limMemOn`, `limMem` | bool, nat | `SHERLOCK_MEM_BUDGET` (STM-010) |
| `gpusOff` | bool | `SHERLOCK_GPUS = 0` (STM-010) |
| `limGpuMemOn`, `limGpuMem` | bool, nat | `SHERLOCK_GPU_MEM` (STM-010) |
| `n` | nat | run length in steps |
| `prog`, `input`, `rP`, `salt` | nat | the prepared run (ELA-001) |
| `box` | nat 0..2 | 0 muninn, 1 huginn, 2 odin |
| `scheduled`, `calibrated` | bool | a schedule exists; its profile is of this key set (STM-008) |
| `schShape`, `schDevice`, `schConc`, `schEst` | nat | the pick: shape, device, leaf concurrency, estimated leaf-stage time (STM-009, STM-017) |
| `cut` | [nat] | leaf sizes of the run |
| `forced` | bool | the cut is the `T16_CUT` override |
| `policy` | nat 0..2 | wrap pipeline: 0 full overlap, 1 evict and overlap after the quotient, 2 no overlap (STM-015) |
| `peakStark`, `peakWrap`, `peakCoset` | nat | device peaks of one STARK call, one wrap, the cosets (STM-015) |
| `wrapFree`, `starkExtra`, `margin` | nat | free device bytes at the wrap start, STARK extra bytes, margin (STM-019) |
| `earlyWraps` | nat | admitted early wrap starts |
| `runs`, `recBox`, `recShape`, `recDevice`, `recLeaves`, `recCalibrated`, `recKeys`, `recClaim` | nat, bool | run record of the last proof (ELA-007); `recClaim` = `claim_pub` |
| `nodes` | nat | N1 joins started (ELA-006) |

State domain (`Valid`, else `outside-capability`): `caps` has 2 entries and `calT`, `calHost`, `calDev` 4; `gpuCount <= 1`; `box < 3`; `calMask < 16`; `scheduled` implies `profWritten`.

## Derived values (the same definitions the model needs)

- `ceilDiv(a, c) = (a + c - 1) / c`; `div1(a, d) = a / max(d, 1)`.
- `hostBudget = limMemOn ? min(limMem, memBudget) : memBudget`; `gpusUsable = gpusOff ? 0 : gpuCount`.
- `concOf(e, host)`: CPU (e odd): `min(div1(host, calHost[e]), max(1, profCores / 4))`; GPU with `gpusUsable = 0`: 0; unified: `div1(limGpuMemOn ? min(limGpuMem, host) : host, calDev[e] + calHost[e])`; discrete: `min(div1(limGpuMemOn ? min(limGpuMem, gpuFree) : gpuFree, calDev[e]), div1(host, calHost[e]))`.
- Candidate of entry e at `host`: exists when bit e of `calMask` is set, `calT[e] > 0`, `caps[e/2] > 0`, `n > 0` and `c = min(concOf(e, host), m) > 0`, where `m = ceilDiv(n, caps[e/2])`; then `{shape: e/2, dev: e%2, m, conc: c, est: ceilDiv(m, c) * calT[e]}`.
- Pick: the least candidate in the order (est, m, shape, dev) (STM-017).
- Greedy cut of `k` leaves with `left` steps and cap `S`: `take = min(S, left - (k - 1))`, repeated. Forced cut `balCut(n, k)`: leaf i is `n / k + (i < n % k ? 1 : 0)`.
- `policyOf(mem)`: `usable = mem - mem / 16`; 0 if `peakStark + peakWrap <= usable`, else 1 if `peakStark - peakCoset + peakWrap <= usable`, else 2. The memory is `gpuTotal` (or `hostBudget` for a unified GPU), lowered by `SHERLOCK_GPU_MEM`; no usable GPU gives 2.
- `wrapTail = policy == 0 ? peakWrap - peakCoset : peakWrap`.
- `claimOf(prog, input, rP, salt) = ((prog * 1000003 + input) * 1000003 + rP) * 1000003 + salt`.
- `budgetOf(memAvail, cgroupHead) = cgroupHead == 0 ? memAvail : min(memAvail, cgroupHead)`.

## Operations

| code | op | arg | guard (refusal codes) | effect |
|---|---|---|---|---|
| 0 | setup | 0 = every calibration proof verifies, 1 = one refused | arg 1: `setup-stopped` (STM-005) | `profWritten, profFormatOk, profFieldsOk := true`; `profKeyD := proverKeyD`; `memBudget := budgetOf(...)`; `unified := hasGpu && !gpuReportsMem`; `gpuCount := hasGpu ? 1 : 0`; `gpuTotal, gpuFree := unified ? 0 : hw values`; `profCores := cores`; `calMask := hasGpu ? 15 : 10`; `scheduled := false` |
| 1 | startRun | 0 | `profWritten` (`no-profile`, STM-016); profile well formed: format, fields, every present entry has `calT > 0` (`profile-refused`, STM-007); a pick exists at `hostBudget` (`no-shape-fits`, STM-012); first failing code only | `scheduled := true`; `calibrated := profKeyD == proverKeyD`; `schShape, schDevice, schConc, schEst` := the pick; `cut := greedy cut` of `m` leaves, `forced := false`; `policy := policy` |
| 2 | memLimit a | a < 16 | none | `limMemOn := true`, `limMem := a * memUnit`, `scheduled := false` |
| 3 | gpus g | g <= 1 | none | `gpusOff := (g == 0)`, `scheduled := false` |
| 4 | gpuMemLimit a | a < 16 | none | `limGpuMemOn := true`, `limGpuMem := a * memUnit`, `scheduled := false` |
| 5 | forceCut k | k < 16 | `scheduled` (`no-schedule`) | `cut := balCut(n, k)`, `forced := true` |
| 6 | prove | 0 | `scheduled` (`no-schedule`); else `check_free` (STM-013), all failing codes in this order: shape < 2 and `caps[shape] > 0` (`shape-no-key`); cut not empty (`empty-cut`); every leaf in 1..S (`leaf-out-of-range`); sum = n (`cut-not-covering`) | `runs += 1`; `recBox := box`, `recShape`, `recDevice`, `recLeaves := len(cut)`, `recCalibrated := calibrated`, `recKeys := proverKeyD`, `recClaim := claimOf(prog, input, rP, salt)` |
| 7 | newRun | 0 | none | `salt += 1`, `scheduled := false` |
| 8 | moveTo b | b < 3 | none | `box := b`, `profWritten := false`, `scheduled := false` |
| 9 | join v | v <= 1 | v = 1: `leaf-not-verified` (ELA-006) | `nodes += 1` |
| 10 | earlyWrap | 0 | `scheduled` (`no-schedule`); `wrapFree >= starkExtra + wrapTail + margin` (`early-wrap-refused`, STM-019) | `earlyWraps += 1` |
| other | | | `outside-capability` | none |

## Suggested constants and fill

`--const caps=[2,4] --const calT=[2135,5604,2094,5355] --const calHost=[1001,1747,997,1747] --const calDev=[1026,0,1026,0] --const memUnit=1026 --const peakStark=8800 --const peakWrap=13300 --const peakCoset=8050` (huginn's MEASURED calibration in MB and ms, `Tests.lean` `hug0`). Small abstract values work as well; keep `n` and the limits small. Fields the model leaves out: `--fill earlyWraps=0,setupStops=0,nodes=0`.

## Open interpretation points (ledger)

| ID | Point | Status |
|---|---|---|
| SC-1 | STM-004 "each GPU": the oracle has one GPU per box (the code has one GPU engine; the spec status says OPEN). | held |
| SC-2 | STM-008 "record the run as not calibrated": the run proceeds and the record carries `recCalibrated = false`; no refusal. | held |
| SC-3 | STM-016: the profile path is modelled as "the box has a profile" (`profWritten`); a move to another box clears it. | held |
| SC-4 | STM-007 "finite positive time": with natural-number times this is `calT[e] > 0` for every present entry. | held |
| SC-5 | `startRun` returns the first failing code only (no-profile, then profile-refused, then no-shape-fits): each later check needs the earlier one to hold. `prove` returns every failing code of `check_free`. | held |
| SC-6 | ELA-001 "prepared run": `claim_pub` is a function of (prog, input, r_P, salt); `newRun` draws a fresh salt (ZK-013). | held |
