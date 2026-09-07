# Design: Preview financial operation and differential settlement

## Inputs and dependencies

Dependencies: MC01.

- `experiments/moriarty-developer-mock/src/language/core.ts`
- `experiments/moriarty-midnight-network/preview-call.mjs`
- `experiments/moriarty-midnight-network/preview-verify.mjs`
- `evidence/midnight-preview-2026-09-07/README.md`

## Interfaces

FinancialRunInput binds program, initial state, bounded action, authority, observations, and explicit test-asset mapping. DifferentialReceipt binds local expected trace, complete observed ledger effects, transaction identifier, block hash, finality, and readback.

## Outputs and ownership

- `experiments/moriarty-midnight-financial/fixtures/loan-preview.json`
- `experiments/moriarty-midnight-financial/fixtures/swap-preview.json`
- `experiments/moriarty-midnight-financial/asset-mapping.json`
- `experiments/moriarty-midnight-financial/contracts/financial.compact`
- `experiments/moriarty-midnight-financial/src/run-preview.ts`
- `experiments/moriarty-midnight-financial/src/compare-effects.ts`
- `experiments/moriarty-midnight-financial/tests/differential.test.mjs`
- `experiments/moriarty-midnight-financial/package.json`

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
Each result audit requires exact Opus and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.
