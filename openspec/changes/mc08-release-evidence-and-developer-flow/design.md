# Design: Completion evidence and developer acceptance

## Inputs and dependencies

Dependencies: MC01, MC02, MC03, MC04, MC05, MC06, MC07.

- `openspec/moriarty-completion-program.json`
- `deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml`
- `docs/FOOTGUNS.md`

## Interfaces

CompletionRecord maps each requested checklist item and legacy G01-G24 gate to exact evidence and an explicit scoped status. A release record cannot inherit approval from a different source digest or a planning audit.

## Outputs and ownership

- `experiments/moriarty-release-check/src/verify-release.ts`
- `experiments/moriarty-release-check/tests/release.test.mjs`
- `experiments/moriarty-release-check/release-gate-crosswalk.json`
- `experiments/moriarty-release-check/developer-scenarios.json`
- `experiments/moriarty-release-check/package.json`

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

The [September 9 refinement](../../ROADMAP-REFINEMENT-2026-09-09.md) applies through [SP01](../../sprints/sp01-financial-contract-and-execution-admission.md#report-informed-acceptance-refinement), [SP02](../../sprints/sp02-complete-mori-authoring-frontend.md#report-informed-acceptance-refinement), [SP08](../../sprints/sp08-defi-actions-and-outcome-intents.md#report-informed-acceptance-refinement), [SP12](../../sprints/sp12-developer-release-and-reproducible-evidence.md#report-informed-acceptance-refinement). Retain every original numbered task and requirement; the [task crosswalk](../../sprints/package-task-map.json) names its closing sprint task. Research-derived changes require the scoped positive/negative evidence and requalification owned there. Planning status does not promote this package or replace its existing admission gates.
