# Design: Mandatory contract, intent, transition, and history claims

## Inputs and dependencies

Dependencies: MC03, MC04.

- `experiments/moriarty-developer-mock/src/language/claims.ts`
- `experiments/moriarty-developer-mock/src/language/policy.ts`
- `docs/research/2026-09-06-pcd-report-integration.md`
- `experiments/moriarty-ledger-adapter/correspondence-spec.md`

## Interfaces

ClaimSpec -> MandatoryManifest -> signed Intent -> BoundClaim -> ProofArtifact. VerifiedClaim records trusted verifier identity, domain, program/spec versions, dependencies, assumptions, and applicability. accept checks ContractInvariant, IntentRefinement, TransitionValidity, and HistoryCompliance.

## Outputs and ownership

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

## Native relation extension: mandatory-claims-01

Implement contract invariants, dynamic signed authority, intent refinement, valid transitions, and authenticated history.
Require actual signature and mandatory-claim checks in the proof/acceptance boundary. Requalify the affected MC04 theorem and adapter.
Freeze at most 8 positive transitions and meaningful invalid proof/context controls.
The charter allocates one campaign with 20 cumulative proving/verification minutes.
Require both audits before launch and independent retained-byte verification after proving.
The original fixed-loan MC03 proof cannot discharge this extended predicate.

Owned outputs:

- `experiments/moriarty-acceptance/proof/relation-spec.md`
- `experiments/moriarty-acceptance/proof/harness.rs`
- `experiments/moriarty-acceptance/proof/verify-retained.rs`
- `experiments/moriarty-acceptance/proof/encoding-controls.rs`
- `experiments/moriarty-acceptance/proof/campaign-cases.json`
- `experiments/moriarty-acceptance/proof/resource-contract.json`
- `experiments/moriarty-acceptance/proof/run-reviewed.py`

## Explicit upstream extension ownership

This package also owns the exact upstream extension paths listed in the program charter's ownership section.
Extend language lowering, correspondence theorems, and acceptance predicates when its supported semantic domain changes.
Use the single versioned `moriarty-ledger-adapter/contracts/acceptance.compact` lineage.
Integrate mandatory-claim code as a library; do not deploy it as a bypass validator.
Requalification includes changed-domain proofs and both audits, not just rerunning older loan/swap tests.
Freeze the exact shared-path subset before dispatch and serialize writers.

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
