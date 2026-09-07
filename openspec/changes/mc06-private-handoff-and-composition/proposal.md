# Change: Private witness handoff and bounded split/join

## Why

Demonstrate independent private successor proving and bounded branch composition with residual authority and obligations.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Specify residual and handoff semantics.
- Implement independent private handoff.
- Implement split and join.
- Validate ledger conflicts and recovery.

- Implement the separately allocated native relation and retained-proof campaign.

## Impact

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
- `evidence/moriarty-completion-program-2026-09-07/MC06/`

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
