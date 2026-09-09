# Tasks: Private witness handoff and bounded split/join

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Demonstrate independent private successor proving and bounded branch composition with residual authority and obligations.

**Dependencies:** MC05.

**Implementation root:** `experiments/moriarty-composition/`.

**Interfaces:** HandoffPackage lists each successor artifact, recipient, confidentiality, availability, and recovery owner. Split/Join bind distinct predecessor and output identities, compatible policies, residual authority, outstanding obligations, and a conserved global work budget.

## 1. Specify residual and handoff semantics

- [ ] 1.1 Define Complete, Pending, and Rejected outcomes with exact residual authority and obligations. Inventory witness ownership and leakage.
- [ ] 1.2 Review positive feasibility and global work-budget conservation before implementation.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Implement independent private handoff

- [ ] 2.1 Write missing-secret and forbidden-artifact tests first. Run Alice and Bob in isolated private directories.
- [ ] 2.2 Record artifact inventories and prove Bob cannot read Alice secrets through the harness.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Implement split and join

- [ ] 3.1 Write duplicate-predecessor, incompatible-policy, fan-in, and authority-amplification controls first. Implement bounded composition.
- [ ] 3.2 Verify real branch proofs, joined history, and residual conservation.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Validate ledger conflicts and recovery

- [ ] 4.1 Run competing-branch and unavailable-handoff recovery scenarios through actual acceptance.
- [ ] 4.2 Require durable consumption, explicit unavailable outcomes, and scoped confidentiality evidence.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## P. Implement and review the native extension

These tasks precede any proof-dependent acceptance or correspondence claim in this package.
Require private successor and real branch/join proofs. Requalify affected MC04 correspondence and MC05 acceptance.

- [ ] P.1 Write `experiments/moriarty-composition/proof/relation-spec.md` and `experiments/moriarty-composition/proof/campaign-cases.json` for composition-01.
- [ ] P.2 Add independent expected-field and rejection controls in `experiments/moriarty-composition/proof/encoding-controls.rs` before changing the relation.
- [ ] P.3 Implement `experiments/moriarty-composition/proof/harness.rs` and the separate verifier `experiments/moriarty-composition/proof/verify-retained.rs`.
- [ ] P.4 Implement `experiments/moriarty-composition/proof/run-reviewed.py` and freeze `experiments/moriarty-composition/proof/resource-contract.json` under the charter allocation.
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

- [ ] D.1 Probe retained-artifact successor continuation and two-input join; stop interface-blocked if either lacks a checked realization.
- [ ] D.2 Use distinct OS users or isolated containers and retain denied-read evidence for predecessor secrets.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-composition ci
npm --prefix experiments/moriarty-composition run build
npm --prefix experiments/moriarty-composition test
python3 experiments/moriarty-language/formal/k/run.py prove --claims experiments/moriarty-composition/formal/claims.json
npm --prefix experiments/moriarty-composition run verify-isolated
npm --prefix experiments/moriarty-composition run verify-preview
python3 experiments/moriarty-composition/proof/run-reviewed.py --contract experiments/moriarty-composition/proof/resource-contract.json --preflight-only
python3 experiments/moriarty-composition/proof/run-reviewed.py --contract experiments/moriarty-composition/proof/resource-contract.json --execute
python3 experiments/moriarty-composition/proof/run-reviewed.py --contract experiments/moriarty-composition/proof/resource-contract.json --verify-retained
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC06/manifest.json`.
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
- [ ] R.2 Check the added `Composition operators and witness ownership` requirement against independent positive and rejection evidence before closure.

## Sprint delivery schedule

[SP01, SP10, SP11](../../sprints/README.md) supplies task sequencing and required executable packets. Original package acceptance and RP stage admission remain authoritative. Sprint plans do not mark these tasks complete.

## Report-informed acceptance refinement

The [September 9 refinement](../../ROADMAP-REFINEMENT-2026-09-09.md) applies through [SP01](../../sprints/sp01-financial-contract-and-execution-admission.md#report-informed-acceptance-refinement), [SP10](../../sprints/sp10-private-handoff-and-bounded-composition.md#report-informed-acceptance-refinement), [SP11](../../sprints/sp11-full-financial-and-formal-conformance.md#report-informed-acceptance-refinement). Retain every original numbered task and requirement; the [task crosswalk](../../sprints/package-task-map.json) names its closing sprint task. Research-derived changes require the scoped positive/negative evidence and requalification owned there. Planning status does not promote this package or replace its existing admission gates.
