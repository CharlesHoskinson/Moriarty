# C-group dependency propositions

`RequirementRollups.lean` records the exact C1--C6 dependency lists in
`qa/sherlock/predicates.yaml` and `scripts/qa/check-c.sh`. Each external
proposition `U n` denotes the evidence obligation for UIREQ-n. A rollup is
the conjunction of its supplied obligations; a missing component blocks it.

| Group | UIREQ numbers |
| --- | --- |
| C1 | 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 |
| C2 | 13, 14, 15, 16, 32 |
| C3 | 22, 27, 31, 34, 35 |
| C4 | 23, 24, 28, 29, 30, 33 |
| C5 | 17, 18, 19, 20 |
| C6 | 21, 25, 26 |

The iff theorems unpack the conjunctions. They do not prove any external
U obligation, execute holdouts, synthesize a common implementation relation,
or promote a finite model agreement to refinement. In particular, the C5
conjunction does not turn local host policy arithmetic into target evidence,
and the C6 conjunction does not turn a task manifest into completed proofs.

The module imports only `Init` and declares no `sorry` or custom axioms.
Sherlock-fable's required heavy-runner build 19 passed all 21 jobs under
`memcap 48G`, recorded as `20261026T035951Z-sherlock-lean-build` in ledger
commit `8853c8f8c`. The incremental run reported 6.229 seconds and
890,322,944 bytes peak RSS; Lake reported 464 ms for this module and 322 ms
for its query. These are run observations, not benchmarks. The coordinator
attested an outer real `heavy.lock` flock and a private `DW_LOCK` runner
proxy. Preflight passed without being skipped; its `lockHeld=false` probe
concerns the proxy and does not attest the outer lock. This agent inspected
the full trace, ledger and exact current/run-head bytes for the sources
committed in `9a5865203`; it did not execute the build or inspect the host
lock. Earlier builds 1--18 remain capped but unlocked kernel observations.

The actual eight-name query reports no axioms for `component_required` and
`missing_component_blocks`; each of `c1_iff` through `c6_iff` depends on
`propext` and `Quot.sound` only. The JSON records the exact per-name sets and
log digest. The sandbox's own capped invocation remains blocked before Lean
by the user scope bus connection failure.
