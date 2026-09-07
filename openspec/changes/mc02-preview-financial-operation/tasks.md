# Tasks: Preview financial operation and differential settlement

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Settle bounded loan and swap examples on Preview and compare all effects with independent local expectations.

**Dependencies:** MC01.

**Implementation root:** `experiments/moriarty-midnight-financial/`.

**Interfaces:** FinancialRunInput binds program, initial state, bounded action, authority, observations, and explicit test-asset mapping. DifferentialReceipt binds local expected trace, complete observed ledger effects, transaction identifier, block hash, finality, and readback.

## 1. Freeze feasible test fixtures

- [ ] 1.1 Derive small loan and swap cases from existing packages. Pin denomination and token mappings. Reserve closure costs.
- [ ] 1.2 Check independent expected values and the available dedicated test balance before public work.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Implement differential local tests

- [ ] 2.1 Write local expected-effect and adverse-effect tests first. Implement generated Compact financial entry points and complete effect decoding.
- [ ] 2.2 Run positive and negative comparisons against local Docker before public submission.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Run the bounded Preview campaign

- [ ] 3.1 Use the existing Preview wallet. Preserve keys externally. Retain before-state, transaction bytes, all effects, and failed attempts.
- [ ] 3.2 Submit at most two public attempts per case. Require indexed SUCCESS, canonical node finality, and exact readback.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Reconcile integration evidence

- [ ] 4.1 Record actual commands, pins, source hashes, resource use, and the funded asset domain.
- [ ] 4.2 Require independent expected results and both audit verdicts before closing this integration package.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## D. Required decision intake

- [ ] D.1 Freeze distinct counterparty identities, funding, complete effects, and the package submission reservation.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-midnight-financial ci
npm --prefix experiments/moriarty-midnight-financial run build
npm --prefix experiments/moriarty-midnight-financial test
npm --prefix experiments/moriarty-midnight-financial run preview -- --case loan --max-attempts 2
npm --prefix experiments/moriarty-midnight-financial run preview -- --case swap --max-attempts 2
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC02/manifest.json`.
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
