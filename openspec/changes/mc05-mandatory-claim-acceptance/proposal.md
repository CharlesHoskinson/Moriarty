# Change: Mandatory contract, intent, transition, and history claims

## Why

Enforce all four required claims in the real transaction acceptance path.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Freeze canonical claim and policy bindings.
- Prove bounded contract properties and refinement.
- Wire required evidence into ledger acceptance.
- Replace unavailable demo claims honestly.

- Implement the separately allocated native relation and retained-proof campaign.

## Impact

- `experiments/moriarty-acceptance/claim-policy.json`
- `experiments/moriarty-acceptance/src/claim-codec.ts`
- `experiments/moriarty-acceptance/src/certificates.ts`
- `experiments/moriarty-acceptance/src/verify-intent.ts`
- `experiments/moriarty-acceptance/src/accept.ts`
- `experiments/moriarty-acceptance/formal/ContractProperties.lean`
- `experiments/moriarty-acceptance/formal/IntentRefinement.lean`
- `experiments/moriarty-acceptance/formal/lakefile.toml`
- `experiments/moriarty-acceptance/contracts/mandatory-claims.compact`
- `experiments/moriarty-acceptance/tests/acceptance.test.mjs`
- `experiments/moriarty-acceptance/package.json`
- `evidence/moriarty-completion-program-2026-09-07/MC05/`

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
