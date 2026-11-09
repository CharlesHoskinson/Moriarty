# C0 first profile: GP node chain

> Sources: task 2.9 of `openspec/changes/sherlock-vm-consensus-20261107/tasks.md`; `research/sherlock-speed-replan-20261108/tree.md` T1 and §7; runs `c0-gp-r4-t4`, `c0-gp-r4-t10`, `c0-gp-r4-t20` (on and off), `c0-gp-r5-t20` (on), `C0-t1b-solo-t6`, `C0-t1b-concurrent-3`
> Raw: `qa/sherlock/profiles/c0-gp-r4-t*/` in the main checkout (raw event traces, not committed); `qa/sherlock/traces/*c0-gp-r4*`; `scripts/qa/profile-recursion.py decide`
> Updated: 2026-11-09

**Label: overhead unqualified, est +0.7% (task 2.8).** The task 2.8 run `c0-a2-qual-20261108-n20` gives a one-sided 95% bound of 1.017. The limit is 1.01. Read every share and every "on" wall on this page with this label.

## Evidence class

- The numbers come from raw event traces (`recursion.phase-events/1`). They are not qualified `recursion.phase-profile/1` sidecars. `decide` marks every row `arithmetic_only` (or `unresolved`) and `profile_valid: false`.
- The observer is valid for the t4 and t10 "on" runs.
- The observer is NOT valid for the t20 "on" run `c0-gp-r4-t20`: the RSS sampler recorded 1 failure (`sampler_valid: false`). Its rows below are marked diagnostic. The rerun `c0-gp-r5-t20` (2026-11-09) has a valid observer, and the C11-C14 rows use it.
- Each "on" run has 5,973 returned-error intervals: 5,972 transcript reads (phase 19) and 1 verify interval (phase 5) that carries the expected rejection of the negative control. The in-circuit node verifier parses proof bytes that do not exist during node keygen, the genesis prove and decider synthesis; its caller handles each read error. `reduce` and `decide` count these per phase and per context (command 3,104; key_load 438; setup_pk 438; prove 179; verify 1; outside the command 1,813) and do not let them make the observer invalid. Status 0/3, status 2 in other phases, drops, timer failures, reentrancy and sampler failures stay invalidating.
- Each proof partition reconciles. The unexplained part of a node proof is 0.2% to 0.5% of its wall.
- The command partition does not reconcile: 18% (t4) to 26% (t20) of the command wall is outside every span. The GP harness has no caller span around `cost_model` and relation validation. This does not change the proof rows. It blocks a command-level claim.
- Workload: 3 stand-in leaves (k=7), 3 node steps, 1 run decider, resident node PK, official k20 SRS. Every proof verifies, the node negative control rejects, and the ledger verifier accepts the decider.

## Node rows and k

| Item | Value |
|---|---|
| Node rows | 652,364 |
| Node k | 20 (rows limit 1,048,576) |
| Fix in force | `1edd79146` SHKIVC05 (genesis accumulator) |
| Table rows | 1,032,211 |
| Sections | prepare_self 297,401; collapse_and_publish 269,460; accumulate 59,339; prepare_leaf 26,149; own_key_and_state 15 |
| Decider k | 20 |

## Wall per thread count

Node step wall is the caller `prove` span of each node step ("on" runs). Command wall is the whole GP command (off / on).

| RAYON_NUM_THREADS | Node steps 0, 1, 2 (s) | Decider prove (s) | Command off / on (s) | Peak RSS | Run |
|---|---|---|---|---|---|
| 4 | 102.96, 101.37, 101.63 | 105.16 | 628 / 635 | 16.2 GB in node prove spans | `c0-gp-r4-t4` |
| 10 | 72.73, 76.27, 69.51 | 80.57 | 498 / 505 | 16.4 GB in node prove spans | `c0-gp-r4-t10` |
| 20 (diagnostic) | 61.22, 62.13, 64.89 | 67.12 | 450 / 458 | 17.2 GB process lifetime | `c0-gp-r4-t20` |
| 20 | 64.95 (step 0) | 77.88 | - / 473 | 17.3 GB process lifetime | `c0-gp-r5-t20` |

`node-threads-4` and `node-threads-10` in `qa/sherlock/budgets.json` hold the t4 and t10 node step walls. `node-threads-20` holds the `c0-gp-r5-t20` node step 0 wall (64.95 s).

## Shares of one node proof (d, m, q, U)

Definitions (`scripts/qa/profile-recursion.py`, `prover.md` table 3): d = domain-proportional share; m = commitment MSM share; q = quotient and NTT share (phases 13, 14, 22, 24); m or q = the union of the m and q intervals (the two overlap); U = average cores = process CPU of the proof root / its wall.

| Threads | Step | d | m | q | m or q | U (cores) |
|---|---|---|---|---|---|---|
| 4 | 0 / 1 / 2 | 0.890 / 0.903 / 0.900 | 0.162 / 0.147 / 0.159 | 0.709 / 0.706 / 0.705 | 0.829 / 0.827 / 0.838 | 4.26 / 4.29 / 4.29 |
| 10 | 0 / 1 / 2 | 0.877 / 0.866 / 0.868 | 0.228 / 0.197 / 0.217 | 0.618 / 0.655 / 0.622 | 0.809 / 0.817 / 0.800 | 6.58 / 6.25 / 6.89 |
| 20 (diagnostic) | 0 / 1 / 2 | 0.843 / 0.844 / 0.850 | 0.245 / 0.241 / 0.244 | 0.566 / 0.573 / 0.562 | 0.768 / 0.771 / 0.761 | 8.81 / 8.61 / 9.12 |

At 20 threads, valid rerun `c0-gp-r5-t20`, step 0: **d = 0.884, m = 0.256, U = 9.77 cores** (decide C12, C14, C11). The diagnostic r4 values were d = 0.84, m = 0.24, q = 0.57, U = 8.8. m + q is above 0.4, so the GPU-track condition of G-A holds on these numbers. U is below 9 of 20 logical CPUs: one node proof does not fill the host.

## T1(b) concurrency probe (f3)

**f3 = 1.69 / 1.73 / 1.74** (slots 0 / 1 / 2). Solo genesis node proof at 6 threads: 75.77 s (`C0-t1b-solo-t6`, peak 16.2 GB). Three together: 128.09 / 130.68 / 131.50 s (`C0-t1b-concurrent-3`, 2026-11-09, each 16.3 GB, aggregate peak 47.7 GB, cap 60 GiB). Three concurrent proofs finish in 1.73x the solo wall, so throughput is 3 / 1.73 = 1.73x one proof at 6 threads.

The probe is three genesis node proofs at `RAYON_NUM_THREADS=6` (`qa/sherlock/t1b-node-concurrent-3.commands`, budget `node-concurrent-3`, 20 GiB per process, 60 GiB aggregate) and the same proof solo at 6 threads. `profile-gp` has a node-only mode (`MORIARTY_GP_NODE_ONLY=1`, one stand-in leaf); the measured wall is `prove_wall_ns` of the genesis node proof in each sidecar. f3 = wall(one of three) / wall(solo).

## C1-C14 rows

From `decide` over `c0-gp-r4-t4`, `c0-gp-r4-t10` and `c0-gp-r5-t20` with `--overhead c0-a2-qual-20261108-n20/overhead.json` (`qa/sherlock/profiles/c0-decide-20261109/decide.json`). Shares C3-C9 use the first node proof of the first run (t4, step 0, wall 102.96 s). C11-C14 use the r5-t20 node step 0 (wall 64.95 s).

| Row | Status | Metric | Value |
|---|---|---|---|
| C1 | unresolved | - | No fused-terminal candidate row count and same-workload comparison |
| C2 | unresolved | - | No mono16 or 2x8 candidate with a matched 0.8.3 baseline |
| C3 | arithmetic_only | commitment MSM share (t4) | 0.162; ideal bound 1.19x |
| C4 | arithmetic_only | witness share (t4) | 0.083; ideal bound 1.09x |
| C5 | unresolved | - | No scheduler, GPU service or batch throughput candidate |
| C6 | arithmetic_only | lookup share (t4) | 0.062; ideal bound 1.07x |
| C7 | arithmetic_only | quotient share (t4) | 0.610; ideal bound 2.56x |
| C8 | inapplicable | - | 0.8.3 always uses BLST |
| C9 | arithmetic_only | multiopen non-MSM share (t4) | 0.030; ideal bound 1.03x |
| C10 | unresolved | - | Needs matched cold/resident-PK proofs with cache-hit counters |
| C11 | arithmetic_only | node step wall by threads | 4: 102.96 s; 10: 72.73 s; 20: 64.95 s; decision keep 20 threads; U at 20 = 9.77 |
| C12 | arithmetic_only | domain-proportional share (t20) | 0.884; predicted k19 factor 1.79x |
| C13 | unresolved | steps per leaf | node-only time for 320 steps: S=1 20,783 s, S=2 10,392 s, S=4 5,196 s, S=8 2,598 s, S=16 1,299 s (target 5,760 s); needs t_leaf(S) of the universal leaf (M4) |
| C14 | arithmetic_only | MSM share (t20) | 0.256; Amdahl bound 1.34x |

## Open items

- C1, C2, C5, C10 and C13 stay unresolved until their candidates or the universal leaf (M4) exist.
- The overhead label stays until task 2.8 qualifies the observer at or below 1.01.
