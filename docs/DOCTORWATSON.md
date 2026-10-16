# DoctorWatson: the Sherlock software factory

Current working plan, 2026-10-16. The user names the factory **DoctorWatson**;
Sherlock is the universal interpreter it builds. The user authorizes building and running this
factory and requires **local CI**, because hosted CI is too expensive. GitHub
stores source, PRs and result summaries. No Foreman or replacement scheduler is
used. The [research review](research/sherlock/factory-2026-10-16/README.md)
records the nine planning reviews and ten online research assignments.

The default executable lane checks the independent A-profile corpus and its
committed source transcription. The explicit `--host` lane builds and executes
the actual Sherlock host reference, exercises bounded stepping and compares its
complete normal observations with those expectations. These are distinct
developer QA jobs; neither establishes the remaining cross-layer correspondence.
See the [local commands](DOCTORWATSON-CI.md), the QA
[contract](../models/sherlock/qa/CONTRACT.md), and the
[factory EARS](../openspec/changes/doctorwatson/requirements.md).

The [parallel delivery results](audits/2026-10-16/sherlock-parallel-delivery/README.md)
now include actual Rust host, flat-ID image and Quint observations against the
same independently frozen eight-program corpus. Each matches all 216 prefixes
and 5,355 complete records. All 14 selected Rust host tests pass; image tests
were not selected. The new continuation tests cover all 204 N03–N08 prefixes,
every event cut and widths 1/2/4/8. A captured regression also fixes selection of
the correct fuel run in raw divergence diagnostics. K decoding and supplied
Lean comparison are implemented consumers; their presence does not establish
actual K or Lean execution. Source-Core and streaming execution are separate
increments, outside those recorded captures.

## One delivery cycle

1. Select an unchecked feature in the existing
   [Sherlock plan](SHERLOCK-IMPLEMENTATION-PLAN.md) or current
   [A-machine status](../openspec/changes/sherlock-machine-circuit-layout/IMPLEMENTATION-STATUS-2026-10-16.md).
   Name its existing requirement/rule, actual consumer and observable result.
2. Assign disjoint paths and an owner. Freeze independent expected observations
   before executing the implementation. Shared interfaces belong to one integrator.
3. Implement the smallest useful consumer increment. Run local ordinary checks
   and the affected separately admitted K, Quint and Lean/Clean checks. Record
   missing layers as gaps. Witness generation and constraint checks are separate.
4. Compare complete states, prefixes, charges, outcomes and relevant ordered
   financial effects. Preserve the first failing input and field difference.
   Repair from the semantic contract, rather than replacing the expected result.
5. Give a fresh non-author reviewer the full candidate and actual results.
   Integrate through a normal GitHub PR; repeat affected local checks after
   integration changes. Report source, observations and acceptance separately.

Local CI is an explicit command, not a daemon. Agents run it when their changed
closure requires it. It never dispatches model/proof/ledger work implicitly.
After two failures of the same defect class, reduce the reproducer and change
approach. After two process-only cycles or thirty administration minutes, return
to eligible implementation or name the concrete blocker and owner.

## Exact candidate and dependency freshness

Use Git's candidate commit/tree and integration-base identity in the PR, alongside
the applicable profile, bounds, oracle/projection and tool/command identities.
These identify the inputs actually checked; a task label is insufficient.
Keep original outputs and their digests with the result, including failure and
timeout. A passing checker is not a successful later test command.
Each attempt also has its own identity. A result must match both the candidate
fingerprint and the current attempt; retrying unchanged source cannot admit a
late result from the timed-out or cancelled attempt.

An edit to any consumed input invalidates the affected results and reviews.
Declared dependencies include shared schemas, profiles, expectations, adapters,
test/import membership, entrypoints and integration wiring. Follow those edges
transitively; a missing dependency declaration is a process limitation, not
evidence that nothing changed. A merge changes the integration base. Before the
next merge, recheck the candidate against that base, refresh the affected
comparisons and obtain review of the resulting tree and actual results.

Source-only integration can explicitly retain unrun higher tiers; it cannot mark
the feature behavior, cross-layer correspondence or parent exit complete. An
available tool/layer is not an observation. Cancelled or blocked tasks release
their active write ownership and retain historical failures and results.
Upstream edits invalidate their candidate evidence without reactivating those
terminal dispositions or clearing their recorded administration/failure history.
Reactivation needs an explicit owner decision outside this finite delivery cycle.

The [DoctorWatson Quint model](../models/doctorwatson/README.md) checks these
workflow transitions on a small concrete task/actor instance. It is a model of
the design, not a scheduler or proof that agents/Git execute the design faithfully.

## Parallel ownership

Use the available session slots for independent ready work. There is no fixed
three- or six-lane ceiling. More workers help only while write sets, review,
integration and physical resources remain available. Source concurrency changes
no execution authority or resource counter.

| Work | Owner | Useful Luna support |
|---|---|---|
| Shared profiles, IR, ABI, exports and consumers | Integrator | Source/field maps and call-site inventories |
| Nonsecurity implementation | GPT-6.1 Sol high | Precisely specified fixture transcription and narrow leaf changes under an expert contract |
| Independent normal expectations | Non-author oracle owner | Hand calculations from frozen semantics, checked independently before use |
| Security work and judgments | Opus 5.5, with the existing outage fallback | Gather exact source locators; do not decide constraints or security outcomes |
| Nonsecurity candidate/result audit | Fresh Muse Wonder 1.0 high | Link/coverage inventories; no substitute approval |
| Private Lean/Clean integration | Existing Fenrir fork owner | Public scope crosswalks; private sources and paths stay private |

Each assignment records the requirement, predicate, exact paths, consumer,
dependencies, oracle owner, applicable checks, first blocked boundary and expected
deliverable. A reviewer does not author the implementation or derive expectations
from its output. Luna tasks return bounded files and unresolved choices, rather
than repeatedly reading the whole repository. Track useful integrated behavior,
review delay, repeated defects and actual check duration; agent count is not
completion.

## Sprint sequence

These are execution slices of the existing roadmap, not replacement milestones.
Advance ready independent source tasks in parallel; acceptance retains dependencies.

| Slice | Concrete output and acceptance test | Parallel preparation | Retained blocker |
|---|---|---|---|
| Current: local QA foundation | One local command validates the frozen eight-case corpus, checks committed source readback, and reports incomplete or different supplied observations | Local command/docs; independent checker tests; source/task maps | No actual K/Quint/Lean/Clean/circuit execution follows from this check |
| Current: first real Sherlock build | Explicit local host build, new `Machine::run_steps`, normal cut/resume tests and all 216 complete prefix observations through the actual consumer | Reference API; fixed typed inputs/exporter; independent literal projection; real CLI regressions; Opus wrapper | Host post-startup common domain only; constructor startup and model/circuit/native layers stay separate |
| Current: factory design audit | Quint scenarios and sampled workflow traces check SF-01–15, candidate/attempt freshness, ownership, CI fail-stop and integration | Independent requirements checklist and source performance review | This models the factory's design; real agent/Git compliance and interpreter correctness remain separate |
| Existing A arithmetic/fuel QA | Finish adjudicating the already captured 18-test attempt; run the existing separately admitted sampled phase once only if its postgates pass | K source review and normal literal maps; G4 source analysis | No rerun of terminal attempts; K execution and full relation correspondence remain open |
| Reference and model adapters | Map one declared common domain to complete typed observations and earliest differing prefix | Independent fixtures; K field map; finite Quint map; existing Lean reconciliation | Profile clocks differ; absent layers cannot count as agreement |
| Bounded state and helpers | Complete the existing rule/representation decisions and source consumers for data, frames, arithmetic and immutable append | Disjoint host codecs, oracle derivations and operation inventories | Candidate B schemas require their existing decision; A P1–P6 is a separate scope |
| Handwritten and Clean components | Same admitted step projected through independently bound witness and constraint implementations | Component inventories and source preparation within existing authority | C4 requires admitted A102 compilation before dependent circuit edits; native authority is zero |
| Witness/export and resources | Actual consumer reports supported operations and separate witness/constraint export, logical work and physical costs | Witness DAG, exporter capability and resource adapters after their interfaces stabilize | Unknown physical costs are unknown, not zero; source inventory is not native support |
| Optimization | Batch/resume/cache/residual implementations preserve every internal prefix and earliest stop against baseline | Independent width 1/2/4/8 expectations and cut/resume cases | Stable semantics and complete observation adapters first |
| Financial compiler mapping | Two contrasting supported programs preserve full ordered effects, work, fees, liabilities and residual duties | Independent source programs and finance expectations | General compiler/continuation mapping and target phase semantics remain open |
| Native and Midnight stagenet | Sherlock itself compiles, proves, verifies, deploys, calls and reads back complete state/effects on the qualified target | Source integration and operator-supplied non-secret target facts | Existing execution admission, exact toolchain/target tuple and required security evidence |

For each slice update the existing correctness-loop evidence matrix: source and
profile, independent oracle, actual command/exit, raw output digest, per-layer
status, first divergence and next concrete action. Keep failed results. Do not
merge unrelated historical test cohorts into one completion denominator.

## Full V1 remains downstream

Sherlock supports Moriarty's [U0–U7 roadmap](../ROADMAP.md); it does not replace it.
Retain M1–M17, MC/SP/G exits, general source-to-ZKIRv3 correspondence, recursive
and private history, all five composition operators, lifecycle/recovery, fee and
federation policy, ACTUS/DeFi conformance and independent developer release.
The financial inventory distinguishes 276 ACTUS contract cases plus AD0 analysis
from the 277 aggregate, and retains the unresolved ANN file-count discrepancy.
The 160 canonical headings, 15 separate OSR clauses and 35 Sherlock EARS are
indices, not atomic completion percentages. Every full parent exit remains open.

K, Quint and Lean/Clean are required iterative developer QA alongside handwritten
circuits. They are not public compiler/runtime/prover dependencies. Midnight
stagenet acceptance is required for Sherlock; Preview or Fenrir observations
cannot replace it. Do not invent Fenrir identities or network versioning.
All native limits, C4, counters and terminal results remain unchanged. Project
review does not become permission for a developer to use Moriarty.
