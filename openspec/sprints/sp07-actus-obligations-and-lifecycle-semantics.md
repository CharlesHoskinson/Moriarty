# SP07: ACTUS obligations and lifecycle semantics implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use superpowers:writing-plans to expand each task into a reviewed executable packet before behavioral edits. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Implement scheduled financial obligations, starting with the held-out capitalization and refinance cases.

**Architecture:** This sprint contributes to MC01, MC07; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP03. Stage scope: actus-semantics. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| create | `experiments/moriarty-conformance/src/actus.ts` | Pinned event and state comparison adapter |
| create | `experiments/moriarty-language/library/actus/README.md` | Type-specific source programs and source mapping |
| create | `experiments/moriarty-conformance/fixtures/heldouts/nam19.json` | Independent zero-payoff capitalization trace |
| create | `experiments/moriarty-conformance/fixtures/heldouts/refinance.json` | Old/new debt identity and authority trace |
| create | `experiments/moriarty-conformance/tests/actus.test.mjs` | All-field event, schedule and rounding controls |
| modify | `experiments/moriarty-language/formal/k/moriarty.k` | Required financial Core extensions |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP07.1: Resolve financial source discrepancies

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP07.md`.
- [ ] Pin normative sources and fixture revisions for DS-01 through DS-07. Freeze calendar, day-count, event order, rounding, currency and ANN initialization rules. Preserve all 32 taxonomy dispositions and 18 executable type scopes.
- [ ] Verify: Every present result-field kind is compared without arbitrary tolerance. An unresolved required discrepancy blocks its family; no fixture is silently excluded.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP07` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP07.2: Implement the held-outs first

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP07.md`.
- [ ] Implement NAM19 capitalization and accepted refinance in source, typed Core, evaluator and K. Keep nominal debt separate from immediate payoff and token settlement. Preserve old/new obligation identities and authorized liabilities.
- [ ] Verify: Zero cash flow may still capitalize debt. Refinance must be accepted when authorized and rejected for debt erasure, duplicate novation or increased unsigned liability.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP07` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP07.3: Complete scheduled contract families

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP07.md`.
- [ ] Implement all 277 pinned fixtures through shared language libraries and necessary versioned Core extensions. Partition batches by actual contract type and event behavior. Define bounded schedules, partial payment, maturity/default and obligation-preserving exhaustion.
- [ ] Verify: Every required fixture has an independently checked complete semantic trace. K/evaluator/lowering changes reopen affected claims; proof and ledger qualification finish in SP11.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP07` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP07.3 batch boundaries

Use separate executable packets for these pinned types: ANN, CLM, CAPFL, CSH, CEC, COM, LAX, FXOUT, FUTUR, CEG, LAM, NAM, OPTNS, SWPPV, PAM, STK, SWAPS, UMP. Start the NAM batch with NAM19. Each packet lives at `openspec/sprints/execution/SP07-TYPE.md`, with TYPE replaced by its listed identifier. Its exit evidence records every fixture and complete field comparison, source digest, changed K/Core constructors, current semantic claims and charged resources. A failed type remains open; passing another type cannot close it. Refinance has its own held-out packet under SP07.2.

## Shared file integration order

SP07 owns the first financial additions to shared `formal/k/**` and `src/successor/**` files. SP08 drafts its libraries and independent expectations concurrently, then rebases and integrates affected shared files after SP07's reviewed change. The orchestrator assigns one writer for each exact shared file in the executable packet and serializes their merges. No two admitted packets may own the same shared file simultaneously. Re-run affected financial and correspondence checks after integration.

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
npm --prefix experiments/moriarty-conformance test
npm --prefix experiments/moriarty-conformance run actus -- --all --all-fields
python3 experiments/moriarty-language/formal/k/run.py traces --all
```

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Exit gate

Complete ACTUS semantic evidence and held-out implementation. MC07 remains open until native, compiler and acceptance coverage are requalified.
