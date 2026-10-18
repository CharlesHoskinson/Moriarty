# SP01: Financial contract and execution admission implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use superpowers:writing-plans to expand each task into a reviewed executable packet before behavioral edits. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Freeze the financial design inputs and reconcile the current atomic candidate before successor implementation.

**Architecture:** This sprint contributes to MC01, MC03, MC04, MC05, MC06, MC07, MC08; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): current repository and retained evidence. Stage scope: atomic-prepare, atomic-accept, rp01-mc02, rp01-mc03, rp01-full, f0. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| inspect | `experiments/moriarty-language/src/frontend.ts` | Existing source-byte API and bounds admission |
| modify | `openspec/moriarty-completion-program.json` | Current candidate and admitted-stage references |
| create | `experiments/moriarty-language/spec/successor/semantic-contract.md` | Named types, effects, bounds, signing and extension rules |
| create | `experiments/moriarty-language/spec/successor/challenge-map.json` | RP01 source-to-behavior crosswalk |
| create | `evidence/moriarty-completion-program-2026-09-07/report-reconciliation/atomic-reconciliation.json` | Current checks versus inherited incomplete tasks |
| create | `openspec/sprints/execution/SP01.md` | Detailed admitted task plan |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP01.1: Reconcile the atomic candidate

- [ ] Inspect current source and retained audits first, without behavioral edits or verification dispatch. Identify confirmed gaps before prescribing corrections.
- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP01.md`.
- [ ] Read current implementation and scoped MC01 audits. Compare each recorded correction with the actual tree before editing. Run existing checks. Record remaining failures, obsolete commands and relocated worktree references. Preserve prior verdicts; accept only after current implementation reviews.
- [ ] Verify: The record binds HEAD, profile bytes, source digests, actual commands and each prior task disposition. An already fixed input boundary causes no redundant code change.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP01` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP01.2: Freeze the complete challenge map

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP01.md`.
- [ ] Import the immutable inventories listed in coverage.json. Derive independent traces for the three held-outs, eight intents and eight DeFi regression classes. Specify all five composition operators. Attach source scope, read/write footprint, full observations, assumptions, invalid mutation and closure owner to each row. Pin primary sources for required source gaps; give the twelve additional candidate products explicit acquisition or comparative dispositions.
- [ ] Verify: Full RP01 has no missing design row. SP01.6 and SP01.7 own separately closable subset records. Their completion does not require this full map. Unsupported implementation stays open with a closure sprint.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP01` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP01.3: Define the successor contract

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP01.md`.
- [ ] Use the existing LANGUAGE-DESIGN.md as a proposal. Decide grammar boundaries, type/effect judgments, rounding, nominal debt versus tokens, staged updates, privacy and request states. Define canonical exact-plan and outcome-intent signing/display schemas before authority freezes. Partition work and residual authority across pending and composition. State continuation, cancellation, revocation and migration rules.
- [ ] Verify: Each Core operation cites a financial trace or developer requirement. ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance are required. No bound-exhaustion path erases debt.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP01` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP01.4: Decide the complete native route early

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP01.md`.
- [ ] After a separately reviewed recorded F0 allocation exists, inspect the three existing RP02 source routes. Pin outer stack, SRS, arithmetic, finalizer fit estimates and deployment provenance. Record go/no-go criteria and artifact ownership. Stop the decision round at its admitted ceiling, at most 30 minutes. Preserve charges and all failed routes.
- [ ] Verify: Only a go decision may mark f0 complete; no-go or unresolved essential inputs mark it blocked. F0 go permits preparation only. No-go or missing essential source blocks SP04-dependent proving; independent language work continues. No automatic k17 retry or ledger-family guess.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP01` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP01.5: Freeze native path and command ownership

- [ ] After F0 go, write `openspec/sprints/execution/native-paths.json` before native implementation starts.
- [ ] Use `experiments/moriarty-native-ivc-r3/successor/` as the sole planned MC03 replacement root. Preserve historical revision-01 evidence.
- [ ] Assign the native export shim to MC03 and the outer transcript/finalizer port and probe orchestrator to MC04.
- [ ] Bind each planned entry point to its owner, source pin, allowed paths and interface. Do not claim absent commands exist.
- [ ] Complete `native-path-freeze` after both current source/path reviews. F0a must then implement and freeze the actual commands and candidate hashes before F1.
- [ ] Verify: no duplicate replacement tree or ambiguous export owner; F0a/F2 reject an absent path-freeze gate.

## SP01.6: Close the loan/swap design subset

- [ ] Create the `subsets.RP01-MC02` record in `semantic-challenges.json` using the existing atomic loan/swap source and independent expectations.
- [ ] Bind denominations, custody, roles, gross/net authority, complete state/effects and retained duties for those cases only.
- [ ] Record exact candidate/profile hashes, source scope, positive and invalid traces, and both scoped design reviews.
- [ ] Verify: this subset closes without SP01.2/SP01.3 full-design completion. It admits no corpus-wide or successor-language claim.

## SP01.7: Close the fixed native-statement design subset

- [ ] Create the `subsets.RP01-MC03` record in `semantic-challenges.json` from the fixed atomic episode and native relation requirements.
- [ ] Bind constants, arithmetic, genesis/predecessors, authority context, complete terminal state/effects and conserved work.
- [ ] Record exact candidate/profile hashes and both scoped design reviews independently of the full RP01 map.
- [ ] Verify: this subset closes without full financial library or successor design completion. F2 still requires atomic acceptance and all F1 controls.

## SP01.8: Close current atomic implementation acceptance

- [ ] Reconcile MC01 tasks 2.x frontend, 3.x elaboration and 4.x Compact mapping against the exact SP01.1 candidate.
- [ ] Require current frontend/evaluator checks, independent loan/swap traces and supported Compact positive/rejection evidence.
- [ ] Resolve the registered bounds and RESOURCE_BOUNDS disposition without admitting arbitrary custom bounds to manufacture failures.
- [ ] Obtain both current Fable/GPT-6 implementation-result audits and bind every accepted predicate to the candidate/profile hashes.
- [ ] Verify: atomic-accept closes only after atomic-prepare. The missing production acceptance backend remains MC04/MC05 work.

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
npm --prefix experiments/moriarty-language run build
npm --prefix experiments/moriarty-language test
npm --prefix experiments/moriarty-developer-mock run build
npm --prefix experiments/moriarty-developer-mock test
```

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Exit gate

Reviewed atomic disposition, full financial design map, separate subset gates and a recorded F0 decision. F0 may finish blocked without admitting SP04 or completing this product.
