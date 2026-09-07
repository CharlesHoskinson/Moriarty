# Design: Complete ACTUS and DeFi conformance

## Inputs and dependencies

Dependencies: MC01, MC04, MC05, MC06.

- `evidence/moriarty-design-sprint-2026-09-06/actus-32-requirements.csv`
- `evidence/moriarty-design-sprint-2026-09-06/defi-72-requirements.csv`
- `evidence/moriarty-r2b-heldouts-2026-09-06/manifest.json`
- `docs/research/2026-09-06-actus-defi-design-study.md`
- `evidence/actus-public-source-acquisition-2026-09-03.json`

## Interfaces

CoverageRow binds stable source/fixture identity, package implementation, independent oracle, complete expected fields, positive feasibility, negative controls, certificate, compiler mapping, and actual acceptance evidence. Totals derive from immutable inventories.

## Outputs and ownership

- `experiments/moriarty-conformance/source-gap-dispositions.json`
- `experiments/moriarty-conformance/coverage.json`
- `experiments/moriarty-conformance/numeric-comparison-policy.json`
- `experiments/moriarty-conformance/src/actus-runner.ts`
- `experiments/moriarty-conformance/src/defi-runner.ts`
- `experiments/moriarty-conformance/src/coverage-gate.ts`
- `experiments/moriarty-conformance/tests/coverage.test.mjs`
- `experiments/moriarty-conformance/tests/heldouts.test.mjs`
- `experiments/moriarty-conformance/fixtures/nam19.json`
- `experiments/moriarty-conformance/fixtures/maple-refinance.json`
- `experiments/moriarty-conformance/fixtures/huma-redemption.json`
- `experiments/moriarty-conformance/package.json`

## Native relation extension: financial-coverage-01

Implement extended financial profiles, row-specific properties, intent refinement, and held-out pending obligations.
Keep all 277 ACTUS fixtures and 72 DeFi rows mandatory. Requalify affected earlier semantic, proof, compiler, and acceptance predicates.
Freeze at most 349 positive episodes with individually frozen transition bounds and meaningful invalid proof/context controls.
The charter allocates one campaign with 120 cumulative proving/verification minutes.
Require both audits before launch and independent retained-byte verification after proving.
The original fixed-loan MC03 proof cannot discharge this extended predicate.

Owned outputs:

- `experiments/moriarty-conformance/proof/relation-spec.md`
- `experiments/moriarty-conformance/proof/harness.rs`
- `experiments/moriarty-conformance/proof/verify-retained.rs`
- `experiments/moriarty-conformance/proof/encoding-controls.rs`
- `experiments/moriarty-conformance/proof/campaign-cases.json`
- `experiments/moriarty-conformance/proof/resource-contract.json`
- `experiments/moriarty-conformance/proof/run-reviewed.py`

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
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.
