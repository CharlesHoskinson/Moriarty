# Profile design-check results

Date: 2026-10-26. Quint 0.33.0, default Rust backend.
All commands below ran from `/home/charl/code/Moriarty-perf/models/recursion`.

## Final required checks

```sh
quint typecheck profile.qnt
quint typecheck profile_test.qnt
quint test profile_test.qnt --main profile_test
```

Both typechecks exited 0 with no diagnostics. Tests exited 0: **25 passing (5238ms)**,
including all seven seeded-mutant detection tests. Five tests involving RSS sampling
each passed 10,000 runs; the other twenty each passed one deterministic run.

```sh
quint run profile.qnt --main profile --invariant exclusiveOwnership --max-steps 30 --max-samples 1000
```

Exit 0. [ok] No violation found (33ms at 30303 traces/second). Trace length statistics: max=9, min=2, average=3.24
Reproduction seed: `0x229e4a91a4a45748`.

```sh
quint run profile.qnt --main profile --invariant wallReconciliation --max-steps 30 --max-samples 1000
```

Exit 0. [ok] No violation found (49ms at 20408 traces/second). Trace length statistics: max=8, min=2, average=3.42
Reproduction seed: `0xc0ecc7c76eaf6950`.

```sh
quint run profile.qnt --main profile --invariant coverageQualification --max-steps 30 --max-samples 1000
```

Exit 0. [ok] No violation found (42ms at 23810 traces/second). Trace length statistics: max=8, min=2, average=3.30
Reproduction seed: `0x27b9b1178a8268a7`.

```sh
quint run profile.qnt --main profile --invariant rssHonesty --max-steps 30 --max-samples 1000
```

Exit 0. [ok] No violation found (45ms at 22222 traces/second). Trace length statistics: max=11, min=2, average=3.50
Reproduction seed: `0x3ac89ce947870084`.

```sh
quint run profile.qnt --main profile --invariant unsupportedCounters --max-steps 30 --max-samples 1000
```

Exit 0. [ok] No violation found (68ms at 14706 traces/second). Trace length statistics: max=10, min=2, average=3.34
Reproduction seed: `0xe1111d9b308273a8`.

```sh
quint run profile.qnt --main profile --invariant overheadQualification --max-steps 30 --max-samples 1000
```

Exit 0. [ok] No violation found (39ms at 25641 traces/second). Trace length statistics: max=9, min=2, average=3.22
Reproduction seed: `0x5a3e3c87528417cb`.

```sh
quint run profile.qnt --main profile --invariant bufferIsolation --max-steps 30 --max-samples 1000
```

Exit 0. [ok] No violation found (40ms at 25000 traces/second). Trace length statistics: max=11, min=2, average=3.56
Reproduction seed: `0x72c55c34790ae09e`.

## Reachability check

```sh
quint run profile.qnt --main profile --invariant coverageQualification --witnesses wAdded wSampled wDense wClosed wCovered wDropped --max-steps 30 --max-samples 1000
```

Exit 0; no invariant violation. Out of 1,000 traces: phases added 486,
RSS sampled 250, two-or-more interior samples 109, profile closed 1,000,
coverage-valid closure 56, dropped events 46.
Seed: `0x97a50f89cf8ee2fa`.

## Scope and interpretation

- RPP-003: `exclusiveOwnership`, `wallReconciliation`,
  `coverageQualification`. Tests distinguish overlapping diagnostics from
  exclusive intervals and exercise both discrepancy and unexplained thresholds.
- RPP-004: `rssHonesty`, `unsupportedCounters`. RSS includes boundary readings;
  only strictly interior samples count toward quality. Sampled peaks never
  certify an instantaneous peak. CPU is optional all-worker core-ms.
- RPP-005: `overheadQualification`, `bufferIsolation`. The supplied one-sided
  95% overhead bound is in basis points (100 bp = 1%); its statistical estimation
  is outside this model. Each qualification record represents one workload/cache
  stratum. Full capacity conservatively invalidates immediately; subsequent
  attempted events increment the dropped count.
- The bounded machine has a 100 ms wall interval, at most three phase identities,
  four candidate RSS timestamps, and five retained event slots. Logical
  observations remain available to the design check after drops; invalid
  profiles cannot qualify for coverage. Accepted and rejected proof outcomes
  are both exercised and remain unchanged.
- Closure is terminal and always enabled while open, so traces may finish before
  30 steps. All sampled witness traces reached closure. These are sampled design
  checks, not exhaustive verification or performance measurements.
- Command accounting, concurrent proof CPU attribution, statistical estimation,
  and actual prover execution are outside this single-proof model. Update it
  when RPP-003/004/005 accounting or qualification rules change.

## Development commands and resolved failures

```sh
quint typecheck profile.qnt
quint run profile.qnt --main profile --invariant wallReconciliation --witnesses wAdded --max-steps 5 --max-samples 20
```

Initial add-phase-only model: both exited 0; no invariant violation; `wAdded`
reached in 20/20 traces (seed `0xd14d5d781c690d41`).

```sh
quint test profile_test.qnt --main profile_test
```

First attempt exited 0 but discovered no tests. Hypothesis: this Quint version
discovers names ending in `Test`. Confirmed in installed CLI source; renamed
the 25 test entry points, leaving two reusable trace helpers without that suffix.

Second attempt exited 1: 22 passing, three QNT502 failures (`Variable s not set`)
in chained guard-rejection tests. Hypothesis: `.fail()` provides no next-state
assignment for subsequent actions. Replaced those chains with pure guard
assertions and kept a terminal `.fail()` as the last expression only.
The final rerun above passed all 25 tests.

```sh
quint run profile.qnt --main profile --invariant exclusiveOwnership --max-steps 30 --max-samples 1000
```

An earlier full-model run also exited 0 with no violation
(seed `0x2ca5ee57611424eb`).

```sh
quint run profile.qnt --main profile --invariant coverageQualification --witnesses wAdded,wSampled,wDense,wClosed,wCovered,wDropped --max-steps 30 --max-samples 1000
```

Exited 1, QNT501: comma-separated witnesses were parsed as one expression.
Hypothesis: `--witnesses` expects space-separated values, confirmed by
`quint run --help`. The corrected reachability command above passed.

Diagnostic commands: `quint --version` exited 0 and reported 0.33.0;
`quint test --help` and `quint run --help` exited 0.
No installation, commit, or push was performed.


## Independent re-run by the lead (2026-10-26 05:30)

The lead re-ran everything under the capped agents slice, with seed `0x7`. `quint typecheck` passed for both files, and `quint test profile_test.qnt --main profile_test` gave 25 passing. `quint run --max-steps 30 --max-samples 1000` found no violation for exclusiveOwnership, wallReconciliation, coverageQualification, rssHonesty, overheadQualification, unsupportedCounters or bufferIsolation. The reachability witnesses `wCovered` and `wDropped` are violated within 2,000 samples, as intended: a covered profile and a dropped-event profile are both reachable, so the invariants are not vacuous.

**What this establishes:** the accounting rules of RPP-003, RPP-004 and RPP-005 are mutually consistent, and each seeded mutant is caught. **What it does not establish:** that the Rust observer implements these rules (tasks 3-4), or any measurement.
