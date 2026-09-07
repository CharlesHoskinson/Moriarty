# Change: Reviewed native encoding and recursive proof

## Why

Produce and independently verify a real two-step financial recursive proof under a reviewed resource contract.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Review the encoding and resource decision.
- Implement encoding and arithmetic controls.
- Implement retained proof verification.
- Run one reviewed proof campaign.

## Impact

- `experiments/moriarty-native-ivc-r3/successor/encoding-decision.md`
- `experiments/moriarty-native-ivc-r3/successor/relation-spec.md`
- `experiments/moriarty-native-ivc-r3/successor/encoding-fixtures.json`
- `experiments/moriarty-native-ivc-r3/successor/resource-contract.json`
- `experiments/moriarty-native-ivc-r3/successor/harness.rs`
- `experiments/moriarty-native-ivc-r3/successor/verify-retained.rs`
- `experiments/moriarty-native-ivc-r3/successor/tests/encoding-controls.rs`
- `experiments/moriarty-native-ivc-r3/successor/run-reviewed.py`
- `evidence/moriarty-completion-program-2026-09-07/MC03/`

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
