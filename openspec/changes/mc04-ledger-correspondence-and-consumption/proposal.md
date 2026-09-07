# Change: Compiler correspondence and durable ledger consumption

## Why

Connect compiled semantics and native proof verification to durable Midnight effects and consumption.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Establish the verifier adapter boundary.
- Implement a complete effect projection.
- Mechanize correspondence.
- Implement durable consumption and recovery.

- Implement the separately allocated native relation and retained-proof campaign.

## Impact

- `experiments/moriarty-ledger-adapter/verifier-compatibility.json`
- `experiments/moriarty-ledger-adapter/correspondence-spec.md`
- `experiments/moriarty-ledger-adapter/formal/Correspondence.lean`
- `experiments/moriarty-ledger-adapter/contracts/acceptance.compact`
- `experiments/moriarty-ledger-adapter/src/adapter.ts`
- `experiments/moriarty-ledger-adapter/src/consumption.ts`
- `experiments/moriarty-ledger-adapter/src/effect-projection.ts`
- `experiments/moriarty-ledger-adapter/tests/adapter.test.mjs`
- `experiments/moriarty-ledger-adapter/tests/recovery.test.mjs`
- `experiments/moriarty-ledger-adapter/package.json`
- `experiments/moriarty-ledger-adapter/formal/lakefile.toml`
- `evidence/moriarty-completion-program-2026-09-07/MC04/`

## Non-goals

No A4/A5 continuation, mainnet funds, new consensus, or Foreman repair belongs to this package.
No downstream requirement becomes complete from this package's narrower result.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.
