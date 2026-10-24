# Sherlock implementation loop

Superseded 2026-10-24: the process is the factory loop in [AGENTS.md](../../AGENTS.md).
Run `scripts/sherlock-next` and work the first failing predicate it names; heavy runs go
through `scripts/doctorwatson heavy`; `scripts/doctorwatson check <ID>` records results.
Interfaces are in [the factory contract](../../models/doctorwatson/factory/CONTRACT.md).
Review routing, adoption votes, consumed-attempt counting, no-retry rules, permanent
lanes and campaign queues that this file used to carry are retired. Earlier revisions
are in git history.

## Objective

The Sherlock objective is genuine recursion, independent verification, finalized full
effects on the Midnight ledger-9 stagenet, and an exact-intent synthetic Fenrir rollup
benchmark. Selection order: current native build, two linked child proofs with parent
and terminal decider, independent verification, stagenet recursive validation with
complete effects readback, then the Fenrir scale and performance matrix. Reuse the
existing contract, wallet and keys.

See [current status](stagenet-rescue-current-2026-10-24.md), the
[machine-readable observations](status-2026-10-24.json), the
[experiment](stagenet-recursive-rollup-experiment.md) and the
[stagenet OpenSpec](../../openspec/changes/sherlock-stagenet-recursive-rollup/proposal.md).

The public shell interfaces for the legacy DoctorWatson loop remain:

```sh
scripts/doctorwatson.sh --inspect --plan P --results H --reconciliations R --next
scripts/doctorwatson.sh --test-selected MODULE [MODULE...]
scripts/doctorwatson.sh --execute TASK --plan P --selection S --attempt DIR --history H --reconciliations R
```

`P/H/R/S/TASK/DIR/MODULE` are notation, not runnable selections. See the
[shell](../../scripts/doctorwatson.sh),
[readiness consumer](../../models/doctorwatson/reconciliation.py) and
[execution bridge](../../models/doctorwatson/run_selected.py).

## Post-deployment benchmark success criterion

Select the explicit stages in the [benchmark reference](benchmark-reference.md)
and run the existing Python DoctorWatson checker with all four benchmark inputs
in the [how-to](benchmark-howto.md). The initial deployed baseline requires at
least one actual current linked two-transition child/parent/decider chain, actual
per-node proof/statement/VK artifacts and positive native verification, complete
independent state/effects, current source/compiler/environment/toolchain/ledger/
network/circuit/workload/target pins and every fixed stage metric/repetition
predicate. A source merge, host/model fixture, scalar oracle, unavailable root,
missing required metric or cached report cannot satisfy that stage.

Host MapQueue/MirrorArbor compiler coverage qualifies through actual host receipts
without fictitious ledger evidence. The larger 1/2/4/6 Fenrir rollup series and
full-suite stage remain separate conjunctive obligations; initial baseline success
does not retire them or the full Sherlock goal. Report baseline observations
without inventing an SLA. Historical comparisons validate their own frozen source
cut and use the shared explicitly declared comparable-profile tolerance rules.
Actual per-layer resources and local verification remain separate from whole-run
and RPC/ledger timings; unexposed rows/columns remain unavailable.

The [additive benchmark EARS](../../openspec/changes/sherlock-moriarty-benchmarks/specs/benchmarks/spec.md)
retain fixed known stage/case/metric minimums and checked cross-lane source pins.
Pin the recomputed report, policy/current/historical contexts and complete
source/receipt closure in existing evidence/currentness records; keep actual
native verification as its own result. Preserve the original
35 clauses, 25 refinements/68 scenarios and every broader formal/financial/native/
release obligation. Acceptance follows AGENTS.md (factory predicates and holdouts).
