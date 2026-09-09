# SP11: Full financial and formal conformance implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use the existing task contract for routine authorized edits and checks; expand only an unresolved interface or a resource-controlled campaign into an execution note. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Close every required financial behavior across semantics, proofs, compiler mapping and actual acceptance.

**Architecture:** This sprint contributes to MC01, MC04, MC05, MC06, MC07; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP07, SP08, SP10. Stage scope: finance. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| create | `experiments/moriarty-conformance/coverage/manifest.json` | Frozen source, fixture, action and behavior identities |
| create | `experiments/moriarty-conformance/coverage/episodes.json` | Behavior-derived campaign set |
| create | `experiments/moriarty-conformance/src/coverage.ts` | Independent coverage and evidence validator |
| create | `experiments/moriarty-conformance/tests/coverage.test.mjs` | Missing, stale, skipped and altered evidence controls |
| create | `experiments/moriarty-conformance/proof/relation-spec.md` | Financial extension qualification |
| create | `experiments/moriarty-conformance/proof/resource-contract.json` | Reviewed actual episode resource allocation |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP11.1: Freeze behavior-level coverage

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP11.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Bind 277 ACTUS fixtures, 18 executable types, 32 taxonomy dispositions, 72 original DeFi rows, DA01-DA24, held-outs and required RP01 cases. Preserve source versions and complete result fields. Derive episodes from behavior, not taxonomy counts.
- [ ] Verify: Missing-row, missing-field, altered oracle, duplicate identity, skipped test and permissive tolerance mutations fail coverage. Additional source-pinned requirements remain present.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP11` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP11.2: Requalify every expanded domain

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP11.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Run financial traces and required mechanized claims through K, evaluator, lowering, native certificates and actual acceptance. Separate semantic, profile-proof, local-acceptance and Preview-acceptance columns. Reprove domains affected by SP07/SP08/SP10 and freeze the final accepted lineage.
- [ ] Verify: No unsupported required constructor or behavior can close through a narrow proof. Shared evidence requires a checked coverage argument, never inferred equivalence.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP11` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP11.3: Execute bounded coverage campaigns

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP11.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Freeze the complete episode manifest and live resource reservation. Revise the historical 349-episode ceiling through a recorded reviewed resource decision if behavior coverage requires it. Run admitted batches and retain every failure.
- [ ] Verify: No behavior is dropped to fit a cap. Two public samples do not close full Preview coverage. Each required row has actual evidence or a valid proved reduction to covered cases; unresolved rows stay open.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP11` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
npm --prefix experiments/moriarty-conformance run actus -- --all --all-fields
npm --prefix experiments/moriarty-conformance run defi -- --all-rows
npm --prefix experiments/moriarty-conformance run verify-coverage
python3 experiments/moriarty-conformance/proof/run-reviewed.py --contract experiments/moriarty-conformance/proof/resource-contract.json --verify-retained
```

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Report-informed acceptance refinement

**Qualify the complete financial and formal scope.** The [refinement contract](../ROADMAP-REFINEMENT-2026-09-09.md) and [lesson/case crosswalk](report-lessons.json) add the following acceptance details to the existing task IDs. These remain specified-only.

- [ ] SP11.1: freeze every original target and adopted supplemental behavior, with source version, positive/negative expectation and closing owner. Keep the modern 24-case benchmark and three source gaps separate from the historical denominators.
- [ ] SP11.2: each required behavior has distinct semantic, profile-proof, local-acceptance and Preview-acceptance evidence columns. A reviewed proved reduction may map multiple rows to a qualified domain; samples alone never establish that reduction.
- [ ] SP11.3: requalify every expanded domain and required network condition under matching relevant versions. Preserve all failures and the existing 349-episode/resource history; derive actual episode needs before separately reviewed changes to any resource envelope.

## Exit gate

No missing required behavior, field, proof obligation or acceptance link. MC07 and changed earlier packages have current candidate-specific audits.
