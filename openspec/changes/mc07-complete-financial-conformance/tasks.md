# Tasks: Complete ACTUS and DeFi conformance

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Complete every required ACTUS fixture and DeFi target row with independently checked financial and proof evidence.

**Dependencies:** MC01, MC04, MC05, MC06.

**Implementation root:** `experiments/moriarty-conformance/`.

**Interfaces:** CoverageRow binds stable source/fixture identity, package implementation, independent oracle, complete expected fields, positive feasibility, negative controls, certificate, compiler mapping, and actual acceptance evidence. Totals derive from immutable inventories.

## 1. Freeze the full coverage oracle

- [ ] 1.1 Import immutable inventories and every expected result-field kind. Resolve source gaps using primary evidence.
- [ ] 1.2 Add missing-row, missing-field, altered-oracle, and permissive-tolerance rejection tests before the coverage gate.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Implement held-outs before broad expansion

- [ ] 2.1 Implement NAM19, accepted refinance, and pending redemption with independent expected traces.
- [ ] 2.2 Check zero-payoff capitalization, old/new debt identity, and carried unfilled obligations.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Complete financial packages incrementally

- [ ] 3.1 Implement each remaining ACTUS type and DeFi row through the shared Core. Preserve all numeric and source assumptions.
- [ ] 3.2 Run deterministic fixture batches; stop a failing family without marking it complete.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Attach certificates and target evidence

- [ ] 4.1 Bind each row to checked properties, compiler mapping, mandatory claims, and acceptance evidence.
- [ ] 4.2 Require no skipped or unimplemented required row. Recompute totals from the frozen inventories.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## P. Implement and review the native extension

These tasks precede any proof-dependent acceptance or correspondence claim in this package.
Keep all 277 ACTUS fixtures and 72 DeFi rows mandatory. Requalify affected earlier semantic, proof, compiler, and acceptance predicates.

- [ ] P.1 Write `experiments/moriarty-conformance/proof/relation-spec.md` and `experiments/moriarty-conformance/proof/campaign-cases.json` for financial-coverage-01.
- [ ] P.2 Add independent expected-field and rejection controls in `experiments/moriarty-conformance/proof/encoding-controls.rs` before changing the relation.
- [ ] P.3 Implement `experiments/moriarty-conformance/proof/harness.rs` and the separate verifier `experiments/moriarty-conformance/proof/verify-retained.rs`.
- [ ] P.4 Implement `experiments/moriarty-conformance/proof/run-reviewed.py` and freeze `experiments/moriarty-conformance/proof/resource-contract.json` under the charter allocation.
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

- [ ] D.1 Separate semantic, profile-proof, local-acceptance, and Preview-acceptance coverage; retain every mandatory target row.
- [ ] D.2 Derive and review row-specific DeFi expected traces; resolve ANN initialization using the explicit DS-03 derivation rule.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-conformance ci
npm --prefix experiments/moriarty-conformance run build
npm --prefix experiments/moriarty-conformance test
npm --prefix experiments/moriarty-conformance run actus -- --all --all-fields
npm --prefix experiments/moriarty-conformance run defi -- --all-rows
npm --prefix experiments/moriarty-conformance run verify-coverage
python3 experiments/moriarty-conformance/proof/run-reviewed.py --contract experiments/moriarty-conformance/proof/resource-contract.json --preflight-only
python3 experiments/moriarty-conformance/proof/run-reviewed.py --contract experiments/moriarty-conformance/proof/resource-contract.json --execute
python3 experiments/moriarty-conformance/proof/run-reviewed.py --contract experiments/moriarty-conformance/proof/resource-contract.json --verify-retained
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC07/manifest.json`.
- [ ] 5.2 Bind inputs, outputs, commands, resource receipts, and the exact candidate digest.
- [ ] 5.3 Obtain independent Fable and GPT-6 result audits under the program protocol.
- [ ] 5.4 Resolve every blocking finding without widening the accepted predicate.
- [ ] 5.5 Recompute acceptance and update the program register.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.
