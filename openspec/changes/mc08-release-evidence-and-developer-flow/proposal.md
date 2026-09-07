# Change: Completion evidence and developer acceptance

## Why

Close the seven requested workstreams only after reproducible evidence, independent audits, and a usable developer workflow.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Implement the final evidence gate.
- Exercise the developer workflow.
- Run both independent final audits.
- Publish the local completion dossier.

## Impact

- `experiments/moriarty-release-check/src/verify-release.ts`
- `experiments/moriarty-release-check/tests/release.test.mjs`
- `experiments/moriarty-release-check/release-gate-crosswalk.json`
- `experiments/moriarty-release-check/developer-scenarios.json`
- `experiments/moriarty-release-check/package.json`
- `evidence/moriarty-completion-program-2026-09-07/MC08/`

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
