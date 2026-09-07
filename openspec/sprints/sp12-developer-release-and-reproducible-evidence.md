# SP12: Developer release and reproducible evidence implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use superpowers:writing-plans to expand each task into a reviewed executable packet before behavioral edits. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Deliver a cold-start developer workflow whose advertised guarantees match the accepted implementation.

**Architecture:** This sprint contributes to MC08; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP11. Stage scope: release. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| create | `experiments/moriarty-release-check/src/verify.ts` | Final evidence and lineage checker |
| create | `experiments/moriarty-release-check/tests/release.test.mjs` | Stale, missing and fake evidence rejection |
| create | `docs/developer-guide.md` | Author/check/simulate/sign/prove/submit/finality guide |
| modify | `README.md` | Implemented language and explicit remaining release limits |
| modify | `ROADMAP.md` | Evidence-backed completion status |
| create | `deliverables/moriarty-completion-program-2026-09-07/README.md` | Seven-goal and G01-G24 completion crosswalk |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP12.1: Finish developer commands and API

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP12.md`.
- [ ] Expose authoring, formatting, checking, simulation, canonical authorization display/signing, proving, submission and finality inspection. Explain fees, debt, locks, recovery rights, trust assumptions and residual duties. Keep mocked data out of the accepted workflow.
- [ ] Verify: A cold-start developer runs valid loan/swap, rejected intent, stale observation, unavailable witness, pending redemption, restart, conflict, activation-window, revoked-key/spec and consumption-preserving migration scenarios. Unknown signed semantics rejects.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP12` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP12.2: Verify a fresh checkout

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP12.md`.
- [ ] Re-execute deterministic frontend, K, theorem and conformance checks with pinned dependencies. Verify SRS acquisition digests. Re-verify retained native proofs and canonical chain receipts separately from fresh transactions.
- [ ] Verify: No hidden local file or private key is needed to verify public evidence. The guide distinguishes retained receipt verification from a newly funded public campaign.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP12` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP12.3: Audit the final release claims

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP12.md`.
- [ ] Bind every MC package and all G01-G24 obligations to exact candidate/source/profile/deployment evidence. Run missing-audit, stale-hash, fake-proof and omitted-row controls. Obtain independent exact Fable 5.1 medium and fresh GPT-6 audits. Apply Humanizer to developer prose.
- [ ] Verify: Close only accepted predicates. Unperformed pilots, licensing, baselines or broader assurances stay explicit release blockers; this sprint does not infer production readiness.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP12` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP12.4: Publish the accepted result

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP12.md`.
- [ ] Update the roadmap, developer guide, canonical wiki navigation and completion dossier. Publish reviewed work under standing authority. Close the runtime goal only if its actual full objective is fulfilled.
- [ ] Verify: No test count, planning review, merged branch or expended budget substitutes for product acceptance.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP12` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
npm --prefix experiments/moriarty-release-check test
npm --prefix experiments/moriarty-release-check run verify -- --program openspec/moriarty-completion-program.json
openspec validate --all --strict
```

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Exit gate

A reproducible bounded Midnight developer release with all requested language/PCD/financial predicates closed and broader release limitations stated.
