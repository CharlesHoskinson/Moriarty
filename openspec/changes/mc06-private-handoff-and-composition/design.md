# Design: Private witness handoff and bounded split/join

## Inputs and dependencies

Dependencies: MC05.

- `docs/research/2026-09-06-pcd-report-integration.md`
- `docs/research/2026-09-06-intents-report-integration.md`
- `experiments/moriarty-acceptance/src/accept.ts`

## Interfaces

HandoffPackage lists each successor artifact, recipient, confidentiality, availability, and recovery owner. Split/Join bind distinct predecessor and output identities, compatible policies, residual authority, outstanding obligations, and a conserved global work budget.

## Outputs and ownership

- `experiments/moriarty-composition/handoff-schema.json`
- `experiments/moriarty-composition/leakage-and-recovery.md`
- `experiments/moriarty-composition/src/handoff.ts`
- `experiments/moriarty-composition/src/split.ts`
- `experiments/moriarty-composition/src/join.ts`
- `experiments/moriarty-composition/src/residuals.ts`
- `experiments/moriarty-composition/tests/alice-bob.test.mjs`
- `experiments/moriarty-composition/tests/split-join.test.mjs`
- `experiments/moriarty-composition/formal/Composition.lean`
- `experiments/moriarty-composition/formal/lakefile.toml`
- `experiments/moriarty-composition/package.json`

## Native relation extension: composition-01

Implement private successor handoff, split, independent branches, join, residual authority, and conserved global bounds.
Require private successor and real branch/join proofs. Requalify affected MC04 correspondence and MC05 acceptance.
Freeze at most 8 positive transitions and meaningful invalid proof/context controls.
The charter allocates one campaign with 20 cumulative proving/verification minutes.
Require both audits before launch and independent retained-byte verification after proving.
The original fixed-loan MC03 proof cannot discharge this extended predicate.

Owned outputs:

- `experiments/moriarty-composition/proof/relation-spec.md`
- `experiments/moriarty-composition/proof/harness.rs`
- `experiments/moriarty-composition/proof/verify-retained.rs`
- `experiments/moriarty-composition/proof/encoding-controls.rs`
- `experiments/moriarty-composition/proof/campaign-cases.json`
- `experiments/moriarty-composition/proof/resource-contract.json`
- `experiments/moriarty-composition/proof/run-reviewed.py`

## Explicit upstream extension ownership

This package also owns the exact upstream extension paths listed in the program charter's ownership section.
Extend language lowering, correspondence theorems, and acceptance predicates when its supported semantic domain changes.
Use the single versioned `moriarty-ledger-adapter/contracts/acceptance.compact` lineage.
Integrate mandatory-claim code as a library; do not deploy it as a bypass validator.
Requalification includes changed-domain proofs and both audits, not just rerunning older loan/swap tests.
Freeze the exact shared-path subset before dispatch and serialize writers.

## Trust boundaries

Treat source text, solvers, indexers, remote provers, generated code, and supplied receipts as untrusted inputs.
Keep oracle truth, ledger uniqueness, semantic validity, and confidentiality claims separate.
Bind all outputs to the semantic profile and the exact implementation candidate.

## Decisions and failures

Use the shared bounded Core rather than financial family names as primitive proof claims.
Preserve complete effects and positive feasibility when refining an adapter.
Record a concrete changed hypothesis before any correction.
Stop a dependent package when its prerequisite fails.
Do not reset exhausted contracts or replace a missing proof with a mock.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable 5.1 at medium effort and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.

## Report-informed acceptance refinement

The [September 9 refinement](../../ROADMAP-REFINEMENT-2026-09-09.md) applies through [SP01](../../sprints/sp01-financial-contract-and-execution-admission.md#report-informed-acceptance-refinement), [SP10](../../sprints/sp10-private-handoff-and-bounded-composition.md#report-informed-acceptance-refinement), [SP11](../../sprints/sp11-full-financial-and-formal-conformance.md#report-informed-acceptance-refinement). Retain every original numbered task and requirement; the [task crosswalk](../../sprints/package-task-map.json) names its closing sprint task. Research-derived changes require the scoped positive/negative evidence and requalification owned there. Planning status does not promote this package or replace its existing admission gates.
