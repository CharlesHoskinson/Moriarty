# Change: Preview financial operation and differential settlement

## Why

Settle bounded loan and swap examples on Preview and compare all effects with independent local expectations.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Freeze feasible test fixtures.
- Implement differential local tests.
- Run the bounded Preview campaign.
- Reconcile integration evidence.

## Impact

- `experiments/moriarty-midnight-financial/fixtures/loan-preview.json`
- `experiments/moriarty-midnight-financial/fixtures/swap-preview.json`
- `experiments/moriarty-midnight-financial/asset-mapping.json`
- `experiments/moriarty-midnight-financial/contracts/financial.compact`
- `experiments/moriarty-midnight-financial/src/run-preview.ts`
- `experiments/moriarty-midnight-financial/src/compare-effects.ts`
- `experiments/moriarty-midnight-financial/tests/differential.test.mjs`
- `experiments/moriarty-midnight-financial/package.json`
- `evidence/moriarty-completion-program-2026-09-07/MC02/`

## Non-goals

No A4/A5 continuation, mainnet funds, new consensus, or Foreman repair belongs to this package.
No downstream requirement becomes complete from this package's narrower result.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable 5.1 at medium effort and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.
