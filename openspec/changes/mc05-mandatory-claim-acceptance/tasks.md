# Tasks: Mandatory contract, intent, transition, and history claims

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Enforce all four required claims in the real transaction acceptance path.

**Dependencies:** MC03, MC04.

**Implementation root:** `experiments/moriarty-acceptance/`.

**Interfaces:** ClaimSpec -> MandatoryManifest -> signed Intent -> BoundClaim -> ProofArtifact. VerifiedClaim records trusted verifier identity, domain, program/spec versions, dependencies, assumptions, and applicability. accept checks ContractInvariant, IntentRefinement, TransitionValidity, and HistoryCompliance.

## 1. Freeze canonical claim and policy bindings

- [ ] 1.1 Write hash-order vectors and mutation controls before changing claim codecs. Specify allowed verifiers and certificate applicability.
- [ ] 1.2 Reject altered domain, program, policy, dependency, predecessor, and mandatory-root fields.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Prove bounded contract properties and refinement

- [ ] 2.1 Implement explicit loan/swap properties and their relation to signed authority. Keep oracle assumptions separate.
- [ ] 2.2 Mechanically verify properties and positive feasibility. Reject vacuous guards and the wrong-recipient falsifier.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Wire required evidence into ledger acceptance

- [ ] 3.1 Write absent-proof and valid-but-intent-invalid end-to-end tests first. Connect MC03 artifacts through the MC04 adapter.
- [ ] 3.2 Require target rejection for every mandatory-claim mutation, including stripped or downgraded evidence.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Replace unavailable demo claims honestly

- [ ] 4.1 Expose real verified evidence only for supported profiles. Keep unsupported certificates and profiles unavailable.
- [ ] 4.2 Run browser and SDK flows against actual acceptance results and preserve every limitation.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## P. Implement and review the native extension

These tasks precede any proof-dependent acceptance or correspondence claim in this package.
Require actual signature and mandatory-claim checks in the proof/acceptance boundary. Requalify the affected MC04 theorem and adapter.

- [ ] P.1 Write `experiments/moriarty-acceptance/proof/relation-spec.md` and `experiments/moriarty-acceptance/proof/campaign-cases.json` for mandatory-claims-01.
- [ ] P.2 Add independent expected-field and rejection controls in `experiments/moriarty-acceptance/proof/encoding-controls.rs` before changing the relation.
- [ ] P.3 Implement `experiments/moriarty-acceptance/proof/harness.rs` and the separate verifier `experiments/moriarty-acceptance/proof/verify-retained.rs`.
- [ ] P.4 Implement `experiments/moriarty-acceptance/proof/run-reviewed.py` and freeze `experiments/moriarty-acceptance/proof/resource-contract.json` under the charter allocation.
- [ ] P.5 Obtain both implemented-relation and resource-contract audits before executing the frozen campaign.
- [ ] P.6 Retain actual proofs and verifier controls. Stop on first failure without automatic retry.
- [ ] P.7 Recheck affected earlier package predicates and obtain updated candidate-bound audits.

## X. Extend and requalify shared implementation

- [ ] X.1 Freeze the exact upstream extension paths permitted by the charter.
- [ ] X.2 Extend required language lowering, theorem domains, and acceptance code with behavioral tests first.
- [ ] X.3 Reprove changed correspondence and claim predicates; rerun affected earlier package checks.
- [ ] X.4 Bind every target result to the active acceptance lineage and preserved consumption/migration rules.
- [ ] X.5 Obtain both candidate-bound audits before promoting any affected predicate.

## D. Required decision intake

- [ ] D.1 Freeze signed-intent verification location and exact native bindings before relation review or proving.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-acceptance ci
npm --prefix experiments/moriarty-acceptance run build
npm --prefix experiments/moriarty-acceptance test
lake -d experiments/moriarty-acceptance/formal build
npm --prefix experiments/moriarty-acceptance run verify-preview
python3 experiments/moriarty-acceptance/proof/run-reviewed.py --contract experiments/moriarty-acceptance/proof/resource-contract.json --preflight-only
python3 experiments/moriarty-acceptance/proof/run-reviewed.py --contract experiments/moriarty-acceptance/proof/resource-contract.json --execute
python3 experiments/moriarty-acceptance/proof/run-reviewed.py --contract experiments/moriarty-acceptance/proof/resource-contract.json --verify-retained
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC05/manifest.json`.
- [ ] 5.2 Bind inputs, outputs, commands, resource receipts, and the exact candidate digest.
- [ ] 5.3 Obtain independent Opus and GPT-6 result audits under the program protocol.
- [ ] 5.4 Resolve every blocking finding without widening the accepted predicate.
- [ ] 5.5 Recompute acceptance and update the program register.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Opus and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.
