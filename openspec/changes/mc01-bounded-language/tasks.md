# Tasks: Bounded language and semantic contract

Status: source specification approved by both named auditors; implementation verification in progress. See `evidence/moriarty-completion-program-2026-09-07/MC01/source-approval-05/source-freeze.json`.

**Goal:** Define and implement the Moriarty authoring language over a finite typed Core.

**Dependencies:** Program charter.

**Implementation root:** `experiments/moriarty-language/`.

**Interfaces:** parse(source) -> Result<AgreementAST, Diagnostic[]>; check(ast, profile) -> Result<TypedAgreement, Diagnostic[]>; elaborate(typed) -> CoreProgram; evaluate(program, state, action, authority, observations) -> Rejected | Complete | Pending. All wire encodings carry schema and semantic versions.

## 1. Freeze the language profile

- [x] 1.1 Write the grammar, numeric profile, transition judgments, and bounds. Classify DS-01 through DS-07 dependencies. Resolve foundational ambiguities before freezing; retain target-specific gaps for MC07.
- [x] 1.2 Review traceability against the complete target matrices. Require both named auditors before freezing this profile.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Implement the frontend

- [ ] 2.1 Add positive roundtrip fixtures and negative syntax/type/bounds tests before parser and checker changes. Implement canonical AST and diagnostics.
- [ ] 2.2 Run the frontend suite, including source-location and canonical-encoding mutations.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Implement semantic elaboration

- [ ] 3.1 Add independent loan/swap expected traces before implementing elaboration and evaluation. Preserve exact-plan and outcome modes.
- [ ] 3.2 Compare every state field, obligation, authority delta, and effect against the existing evaluator.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Implement the initial Compact mapping

- [ ] 4.1 Specify representable Core operations and rejection diagnostics. Add local lowering tests and source maps.
- [ ] 4.2 Compile generated supported examples. Record unsupported mappings as failures; do not claim general correspondence.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## D. Required decision intake

- [x] D.1 Define foundational numeric and extension requirements before freezing the first profile.
- [x] D.2 Classify profile-version changes and their signature, proof, theorem, and audit invalidation rules.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-language ci
npm --prefix experiments/moriarty-language run build
npm --prefix experiments/moriarty-language test
npm --prefix experiments/moriarty-developer-mock run build
npm --prefix experiments/moriarty-developer-mock test
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC01/manifest.json`.
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
- [ ] R.2 Check the added `Target-driven successor profile` requirement against independent positive and rejection evidence before closure.
