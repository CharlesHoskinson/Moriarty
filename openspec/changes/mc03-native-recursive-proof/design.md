# Design: Reviewed native encoding and recursive proof

## Inputs and dependencies

Dependencies: MC01.

- `experiments/moriarty-native-ivc-r3/README.md`
- `experiments/moriarty-native-ivc-r3/harness/moriarty_loan_r3.rs`
- `experiments/moriarty-native-ivc-r3/run-native.py`
- `evidence/moriarty-native-ivc-r3-2026-09-07/README.md`

## Interfaces

NativeEpisode binds the original R2 financial episode, all public contexts, fixed authority scope, and the remaining lifecycle bound. ProofArtifact binds proof bytes, VK, SRS, backend pin, statement encoding, and final accumulator verification.

## Outputs and ownership

- `experiments/moriarty-native-ivc-r3/revision-01/encoding-decision.md`
- `experiments/moriarty-native-ivc-r3/revision-01/relation-spec.md`
- `experiments/moriarty-native-ivc-r3/revision-01/encoding-fixtures.json`
- `experiments/moriarty-native-ivc-r3/revision-01/resource-contract.json`
- `experiments/moriarty-native-ivc-r3/revision-01/harness.rs`
- `experiments/moriarty-native-ivc-r3/revision-01/verify-retained.rs`
- `experiments/moriarty-native-ivc-r3/revision-01/tests/encoding-controls.rs`
- `experiments/moriarty-native-ivc-r3/revision-01/run-reviewed.py`

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
