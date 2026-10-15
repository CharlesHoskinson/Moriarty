# Sherlock universal interpreter: design and evidence

**Final design Candidate B — proposed, specified-only.** This integrates the research into a reviewable engineering design. Independent full-candidate audits and the recorded decision rule are required before adoption. A design approval does not establish implementation, correspondence, native acceptance or release readiness.

- [Proposal](proposal.md), [architecture](design.md) and [shared interfaces](interfaces.md).
- [35 nonsecurity EARS requirements and scenarios](specs/universal-interpreter/spec.md), [index](requirements.md) and [traceability](traceability.md).
- [36 independent normal demonstration contracts](demonstrators.md); these are proposed expectations, not executed results.
- [Required Lean/Clean stage](clean-stage.md) and [public API inventory](public-clean-api-inventory.json).
- Opus-owned [original security draft](security-requirements.md) and [Candidate B delta](security-candidate-b.md); the Opus-owned A1–A4 amendments govern the proposed profile and codec dispositions.
- [Implementation plan](../../../docs/SHERLOCK-IMPLEMENTATION-PLAN.md), [concept adoption matrix](../../../docs/research/sherlock/CONCEPT-ADOPTION.md) and [unchecked implementation tasks](tasks.md).
- [Ten online academic/project studies](../../../docs/research/sherlock/papers/README.md), following [ten optimization studies](../../../docs/research/sherlock/optimization/README.md).
- [Model coverage](model-coverage.md), [actual observations](../../../models/sherlock/observations.json) and [model source entrypoint](../../../models/sherlock/README.md).

## Design boundary

The reference remains a strict first-order machine with explicit environments, frames and recursive immutable values. Candidate B specifies indexed ProgramImage data, bounded MachineImage representations, explicit helper microsteps, checked UInt128 component interfaces, separate ConstraintPlan/WitnessPlan exports, an uncached witness DAG baseline, prefix-preserving slices, static residualization and declaration-driven ordered matching.

The proposed `sherlock-helper-microstep/1` schedule is distinct from frozen `sherlock-k-a/1` and `sherlock-quint-a/1`. Existing model checks do not validate B. Exact helper rule/allocation inventories, admission and representation bounds, cross-layer mappings, backend realization and financial opcodes remain implementation or reviewed decision obligations. Per-program, universal and fixed-family realizations remain alternatives; no measured winner is selected.

Core/2 successful-prefix await, complete ordered effects, fees, gross debit and residual duties remain controlling. Technical proof cuts do not create financial commits. Stable source profiles remain unchanged. Existing Midnight ledger-9 recursion is the intended integration route; VerifyProof is not a prerequisite.

## Observed evidence and limits

| Artifact | Actual evidence | Remaining gap |
|---|---|---|
| Candidate A K source | Frontend check emitted syntax Kore and compiled text; eight fixtures supplied | No runtime definition/rule execution, determinism proof or B behavior |
| Candidate A Quint companion | Eight tests; 32 sampled seed-1 traces, max 32 transitions; no sampled allInv violation and all seven witness kinds observed | Three finite algorithms, not general code interpretation or exhaustive verification |
| Fenrir Lean foundation | Generic pure reference and normal examples merged into the existing private fork after separate exact-source Muse/Opus review | No B helper/ADT reconciliation, Clean step circuit or cross-model correspondence |
| Candidate B documents | Proposed schemas, requirements, contracts and staged implementation | No B model/host/circuit/native execution supplied |
| Clean/native/target | Public API inventory and existing integration premise | No Sherlock exporter, constrained-step acceptance or qualified target result |

The K/Quint source and observation bytes remain unchanged by Candidate B. Private Fenrir pins, paths and source are retained privately. Security content and judgments belong exclusively to Opus 5.5; nonsecurity authoring uses GPT-6.1 Sol high and requires a fresh non-author Muse Wonder audit.

## Immutable provenance

[inputs.json](inputs.json), [original freeze](research-candidate-a-freeze.json) and [original document checks](research-original-doc-checks.json) describe the historical research snapshot. They are not current-B hashes or acceptance records. [publication-binding.json](publication-binding.json) and SHA256SUMS bind the current package separately. The academic studies retain source receipts, access failures, version differences and acquisition limits. Concepts may be independently rebuilt; repository license observations apply to any separately chosen code reuse.

No U0–U7, M1–M17, MC/SP/G, ZR or MNR parent exit is closed. Full ACTUS/DeFi and composition denominators remain unchanged. Source preparation and design review create no public developer approval requirement or new execution authority.
