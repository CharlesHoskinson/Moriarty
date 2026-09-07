# Tasks: Compiler correspondence and durable ledger consumption

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Connect compiled semantics and native proof verification to durable Midnight effects and consumption.

**Dependencies:** MC01, MC02, MC03.

**Implementation root:** `experiments/moriarty-ledger-adapter/`.

**Interfaces:** LedgerAdapter consumes a versioned compiled program, typed proof artifact, full authority envelope, and predecessor identifiers. It returns Rejected, Unavailable, or FinalizedReceipt. Consumption identity is domain/principal/nonce plus ledger-consumed predecessor/output identities.

## 1. Establish the verifier adapter boundary

- [ ] 1.1 Pin application, aggregation, Compact, ZKIR, ledger, and key formats. Identify the supported verification entry point.
- [ ] 1.2 Run one positive compatibility probe and mutated proof/public-input controls before broad adapter work.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Implement a complete effect projection

- [ ] 2.1 Write effect omission and target mutation tests first. Map every supported source effect to actual ledger behavior.
- [ ] 2.2 Compare independent traces and reject any unrepresented effect or unverifiable external call.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Mechanize correspondence

- [ ] 3.1 State assumptions and the finite supported domain. Implement the source/Core/target relations and proof.
- [ ] 3.2 Run Lean without sorry or undeclared axioms. Keep trace tests separate from the theorem claim.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Implement durable consumption and recovery

- [ ] 4.1 Write crash-point, duplicate nonce, competing branch, and concurrent proposal tests first. Implement atomic ledger-backed consumption.
- [ ] 4.2 Require at most one finalized success for conflicting proposals and consistent restart recovery.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## P. Implement and review the native extension

These tasks precede any proof-dependent acceptance or correspondence claim in this package.
Establish the retained MC03 verifier interface before extending the relation. Requalify correspondence using the new loan/swap proofs.

- [ ] P.1 Write `experiments/moriarty-ledger-adapter/proof/relation-spec.md` and `experiments/moriarty-ledger-adapter/proof/campaign-cases.json` for adapter-profile-01.
- [ ] P.2 Add independent expected-field and rejection controls in `experiments/moriarty-ledger-adapter/proof/encoding-controls.rs` before changing the relation.
- [ ] P.3 Implement `experiments/moriarty-ledger-adapter/proof/harness.rs` and the separate verifier `experiments/moriarty-ledger-adapter/proof/verify-retained.rs`.
- [ ] P.4 Implement `experiments/moriarty-ledger-adapter/proof/run-reviewed.py` and freeze `experiments/moriarty-ledger-adapter/proof/resource-contract.json` under the charter allocation.
- [ ] P.5 Obtain both implemented-relation and resource-contract audits before executing the frozen campaign.
- [ ] P.6 Retain actual proofs and verifier controls. Stop on first failure without automatic retry.
- [ ] P.7 Recheck affected earlier package predicates and obtain updated candidate-bound audits.

## D. Required decision intake

- [ ] D.1 Record the required interface-intake outcome and reviewable wrapper/version-alignment alternatives before actionful dispatch.
- [ ] D.2 Pin the canonical acceptance lineage and migration/replay policy before its first deployment.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-ledger-adapter ci
npm --prefix experiments/moriarty-ledger-adapter run build
npm --prefix experiments/moriarty-ledger-adapter test
lake -d experiments/moriarty-ledger-adapter/formal build
npm --prefix experiments/moriarty-ledger-adapter run verify-preview
python3 experiments/moriarty-ledger-adapter/proof/run-reviewed.py --contract experiments/moriarty-ledger-adapter/proof/resource-contract.json --preflight-only
python3 experiments/moriarty-ledger-adapter/proof/run-reviewed.py --contract experiments/moriarty-ledger-adapter/proof/resource-contract.json --execute
python3 experiments/moriarty-ledger-adapter/proof/run-reviewed.py --contract experiments/moriarty-ledger-adapter/proof/resource-contract.json --verify-retained
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC04/manifest.json`.
- [ ] 5.2 Bind inputs, outputs, commands, resource receipts, and the exact candidate digest.
- [ ] 5.3 Obtain independent Fable 5.1 at medium effort and GPT-6 result audits under the program protocol.
- [ ] 5.4 Resolve every blocking finding without widening the accepted predicate.
- [ ] 5.5 Recompute acceptance and update the program register.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable 5.1 at medium effort and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.

## R. Three-report reconciliation

- [ ] R.1 Satisfy the applicable RP01/RP02/RP03 prerequisites in [the reconciliation](../../REPORT-RECONCILIATION-2026-09-07.md); preserve existing package acceptance and historical evidence.
- [ ] R.2 Check the added `Early complete verifier feasibility` requirement against independent positive and rejection evidence before closure.
