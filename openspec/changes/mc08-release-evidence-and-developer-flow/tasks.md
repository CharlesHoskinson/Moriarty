# Tasks: Completion evidence and developer acceptance

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Close the seven requested workstreams only after reproducible evidence, independent audits, and a usable developer workflow.

**Dependencies:** MC01, MC02, MC03, MC04, MC05, MC06, MC07.

**Implementation root:** `experiments/moriarty-release-check/`.

**Interfaces:** CompletionRecord maps each requested checklist item and legacy G01-G24 gate to exact evidence and an explicit scoped status. A release record cannot inherit approval from a different source digest or a planning audit.

## 1. Implement the final evidence gate

- [ ] 1.1 Write missing-package, stale-source, absent-auditor, fake-proof, and omitted-target tests before the release checker.
- [ ] 1.2 Require exact source, command, receipt, and audit digests for every closed predicate.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Exercise the developer workflow

- [ ] 2.1 Run loan, swap, rejected intent, unavailable witness, pending redemption, restart, and conflict scenarios.
- [ ] 2.2 Keep local simulation, proof verification, submission, and finality visibly distinct.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Run both independent final audits

- [ ] 3.1 Prepare a sealed candidate bundle after deterministic checks. Request exact Fable and fresh GPT-6 review.
- [ ] 3.2 Resolve blocking findings and recheck changed evidence. Unavailable or substituted reviewers cannot approve.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Publish the local completion dossier

- [ ] 4.1 Write deliverables/moriarty-completion-program-2026-09-07/README.md and the G01-G24 crosswalk. Preserve scope limitations.
- [ ] 4.2 Close the runtime goal only when all requested predicates pass. Keep broader release blockers explicit.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## D. Required decision intake

- [ ] D.1 Separate deterministic re-execution from retained proof/chain re-verification; pin and verify the acquired SRS.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-release-check ci
npm --prefix experiments/moriarty-release-check run build
npm --prefix experiments/moriarty-release-check test
npm --prefix experiments/moriarty-release-check run verify -- --program openspec/moriarty-completion-program.json
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC08/manifest.json`.
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
