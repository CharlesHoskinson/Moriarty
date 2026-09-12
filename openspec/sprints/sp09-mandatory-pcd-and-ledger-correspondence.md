# SP09: Mandatory PCD and ledger correspondence implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use the existing task contract for routine authorized edits and checks; expand only an unresolved interface or a resource-controlled campaign into an execution note. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Accept financial transactions only when ledger-verified step proofs and ledger induction establish the language and signed authority predicates.

**Architecture:** This sprint contributes to MC01, MC04, MC05; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP03, SP05, SP07, SP08. Stage scope: f3, mandatory. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| create | `experiments/moriarty-acceptance/spec/statement.schema.json` | Canonical mandatory claim and execution statement |
| create | `experiments/moriarty-acceptance/proof/relation-spec.md` | General bounded Core relation and contract-property certificates |
| create | `experiments/moriarty-acceptance/src/accept.ts` | Complete verified acceptance boundary |
| create | `experiments/moriarty-acceptance/tests/acceptance.test.mjs` | Claim, intent, observation and replay mutations |
| create | `experiments/moriarty-ledger-adapter/formal/k/atomic.k` | K semantics/correspondence for the existing sequential atomic profile before F3 |
| create | `experiments/moriarty-ledger-adapter/formal/claims.json` | MC04 3.1-3.2 atomic and successor claim domains |
| create | `experiments/moriarty-ledger-adapter/proof/relation-spec.md` | Adapter-profile-01 loan/swap relation; MC04 P.1-P.7 |
| create | `experiments/moriarty-ledger-adapter/proof/campaign-cases.json` | Independent loan/swap native campaign cases |
| create | `experiments/moriarty-ledger-adapter/formal/correspondence.md` | Source/Core/K/Compact/proof/ledger theorem domains |
| create | `experiments/moriarty-ledger-adapter/tests/consumption.test.mjs` | Conflicts, restart, revocation and migration |
| modify | `experiments/moriarty-language/src/lower-compact.ts` | Accepted versioned Core projection |

The step-relation compiler output, head-discipline checker and deploy audit use roots recorded at `native-path-freeze` under the [PCD integration](../PCD-INTEGRATION-2026-09-11.md).

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP09.1: Close atomic F3 before extending it

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP09.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Use the Stage 0 seam; no SP06 or MC03 proof is an input. Complete MC04 P.1-P.7 for adapter-profile-01 as fused Compact step circuits for loan and swap, proved by the proof server and verified by a Rust harness against the compiled keys. Run E2 fit and stop if no variant fits k ≤ 17. Run E1 on Preview. Deploy through the custom immutable-authority path and pass the deploy audit. Connect the qualified relation to SP05 financial settlement. Pin actual Preview deployment, toolchain, key and SRS identity. Complete MC04 3.1-3.2 atomic source/Core/target correspondence, including head read-then-write discipline, before F3 promotion. Own the atomic K definition and claim manifest here, including all executable verification dependencies. Bootstrap the pinned K runner/toolchain if SP03 has not created it; serialize shared writers and preserve atomic sequential semantics separately. Discharge the mechanized loan/swap correspondence with explicit assumptions. Verify full output projection and unique head consumption before promoting atomic F3.
- [ ] Verify: Actual Preview loan/swap acceptance uses the qualified adapter-profile-01 proofs with verification enabled. Tampered proof, output or head rejects, and of two conflicting calls from one head at most one applies. A deployment that fails the audit, or lacks discharged atomic correspondence, cannot close F3. SP09.4 requalifies successor correspondence; it is not a prerequisite for authoring the atomic proof here.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP09` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP09.2: Implement the general bounded relation

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP09.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Replace the fixed-instance limitation with the accepted versioned Core transition relation. Bind all state/effects/liabilities/observations/work fields. Define contract-property certificate judgments checked by the deploy audit and bound in Π_P. Discharge all four claim families at their declared loci on every permitted path, including `Initialize`, `Terminate` and migration. Run sequential history at 10 and 100 steps on one head.
- [ ] Verify: At least two non-hardcoded agreements and adversarial traces exercise the general relation. Genericity requires constructor coverage and correspondence, not merely those examples. Missing claims or unsupported property evidence fail closed.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP09` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP09.3: Enforce authorization and policy lifecycle

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP09.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Implement authenticated observations, durable partial-fill budgets, signature domains, expiry/currentness, permitted calls/recipients and one-time predecessor consumption. Check Π_P bounds before expensive proving. Enforce block-time observation freshness, intent digest v2 in exact-head and outcome modes, a principal-threshold `Pause` and forward-declared migration (E4) instead of in-place key revocation.
- [ ] Verify: Proof-valid but intent-invalid execution rejects. Two individually valid conflicting spends cannot both finalize. Restart, cancellation, pause and migration cannot revive consumed authority or work. A maintenance update and a downgrade migration reject. Migration Preview evidence waits for ledger 9 on Preview.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP09` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP09.4: Prove compiler and acceptance correspondence

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP09.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Define complete observational projection across source elaboration, K, evaluator, Compact lowering, native relation and ledger application. Prove supported domains with explicit environment and cryptographic assumptions. Run mutation tests for omitted outputs, duplicated debt, changed policy and field-order/canonical-decoding attacks. Include head read-then-write and section invariants, and blind-write mutants, in the correspondence domain.
- [ ] Verify: Generated proof obligations are discharged for each advertised domain. Tests are separate supporting evidence. Unsupported projection or unresolved theorem blocks promotion.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP09` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP09.5: Requalify native and Preview mandatory acceptance

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP09.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Freeze the mandatory-claims-01 campaign and reviewed resources after implementation. Produce real extended proofs and verify retained bytes independently. Run local and admitted Preview acceptance controls under the new lineage, with deploy audits for every deployment.
- [ ] Verify: MC04/MC05 close only for the actual mandatory profile after current audits. The earlier atomic proof or uncertified I2 receipt does not inherit this scope.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP09` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## Native packet boundaries

SP09.1 expands into `execution/SP09-atomic-correspondence.md`, `execution/SP09-adapter-source.md`, `execution/SP09-adapter-native.md` and `execution/SP09-atomic-preview.md` under `openspec/sprints/`. The first two produce source/theorem candidates. Native execution requires subsequent current source/resource review and separate campaign admission. Preview execution follows actual native verification and atomic correspondence. Never combine preparation review and execution into one ungated command.

SP09.5 similarly separates mandatory source/relation review, native execution, retained verification and Preview qualification. Each command is frozen and admitted for its exact candidate; the umbrella task is not an execution grant.

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
npm --prefix experiments/moriarty-acceptance test
npm --prefix experiments/moriarty-ledger-adapter test
python3 experiments/moriarty-language/formal/k/run.py prove --claims experiments/moriarty-language/formal/k/claims.json
python3 experiments/moriarty-acceptance/proof/run-reviewed.py --contract experiments/moriarty-acceptance/proof/resource-contract.json --verify-retained
```

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Report-informed acceptance refinement

**Require complete proof and authority at ledger acceptance.** The [refinement contract](../ROADMAP-REFINEMENT-2026-09-09.md) and [lesson/case crosswalk](report-lessons.json) add the following acceptance details to the existing task IDs. These remain specified-only.

- [ ] SP09.1: close atomic F3 after I2, the Stage 0 seam, E2, E1 and atomic correspondence, without waiting for certificates or full successor ACTUS/DeFi. Verify actual loan/swap proofs on Preview and unique head consumption.
- [ ] SP09.2/.3: mandatory successor acceptance binds ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance on every permitted route, including genesis, termination and migration, through the claim discharge map. Bind request/valuation observations, complete effects, adapter/deployment/policy revisions and durable authority.
- [ ] SP09.3/.4: reject refund-reset budgets, hidden intermediate calls, donation-derived liability capacity, unbound upgrades, conflicting spends, replay after restart and migration that revives consumed authority. Prove nonvacuous correspondence for each advertised domain.
- [ ] SP09.5: reprove the expanded relation and obtain actual verification-enabled Preview evidence under that lineage. Atomic or I2 receipts cannot inherit general mandatory scope.

## Exit gate

Complete mandatory native acceptance, durable authorization/consumption and proved compiler/ledger correspondence for the admitted successor domain.
