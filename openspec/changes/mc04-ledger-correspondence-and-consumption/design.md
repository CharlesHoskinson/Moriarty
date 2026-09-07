# Design: Compiler correspondence and durable ledger consumption

## Inputs and dependencies

Dependencies: MC01, MC02, MC03.

- `docs/research/2026-09-06-midnight-native-recursion.md`
- `experiments/moriarty-language/src/lower-compact.ts`
- `experiments/moriarty-native-ivc-r3/revision-01/relation-spec.md`
- `experiments/moriarty-midnight-financial/src/compare-effects.ts`

## Interfaces

LedgerAdapter consumes a versioned compiled program, typed proof artifact, full authority envelope, and predecessor identifiers. It returns Rejected, Unavailable, or FinalizedReceipt. Consumption identity is domain/principal/nonce plus ledger-consumed predecessor/output identities.

## Outputs and ownership

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

## Native relation extension: adapter-profile-01

Implement compiled loan and swap transition relations, complete effects, and durable authorization bindings.
Establish the retained MC03 verifier interface before extending the relation. Requalify correspondence using the new loan/swap proofs.
Freeze at most 6 positive transitions and meaningful invalid proof/context controls.
The charter allocates one campaign with 20 cumulative proving/verification minutes.
Require both audits before launch and independent retained-byte verification after proving.
The original fixed-loan MC03 proof cannot discharge this extended predicate.

Owned outputs:

- `experiments/moriarty-ledger-adapter/proof/relation-spec.md`
- `experiments/moriarty-ledger-adapter/proof/harness.rs`
- `experiments/moriarty-ledger-adapter/proof/verify-retained.rs`
- `experiments/moriarty-ledger-adapter/proof/encoding-controls.rs`
- `experiments/moriarty-ledger-adapter/proof/campaign-cases.json`
- `experiments/moriarty-ledger-adapter/proof/resource-contract.json`
- `experiments/moriarty-ledger-adapter/proof/run-reviewed.py`

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
