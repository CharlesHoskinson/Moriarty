# SP03: Executable bounded semantics in K implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use the existing task contract for routine authorized edits and checks; expand only an unresolved interface or a resource-controlled campaign into an execution note. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Execute Moriarty Core in K and establish scoped language metatheorems and evaluator correspondence.

**Architecture:** This sprint contributes to MC01, MC04, MC05; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP02. Stage scope: successor-semantics. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| create | `experiments/moriarty-language/formal/k/toolchain.lock.json` | Pinned K and backend acquisition digests |
| create | `experiments/moriarty-language/formal/k/moriarty.k` | Core configuration and operational rules |
| create | `experiments/moriarty-language/formal/k/claims.k` | Termination, type preservation and rejection claims |
| create | `experiments/moriarty-language/formal/k/claims.json` | Claim domains, assumptions and proof status |
| create | `experiments/moriarty-language/formal/k/run.py` | Reproducible compile, trace and proof wrapper |
| create | `experiments/moriarty-language/formal/k/fixtures/partial-payment.json` | Independent surviving-obligation case |
| modify | `experiments/moriarty-language/src/evaluate.ts` | Only reviewed compatibility changes |
| create | `experiments/moriarty-language/src/successor/evaluate.ts` | Successor reference evaluator |
| modify | `experiments/moriarty-language/src/cli.ts` | Simulation command using the successor evaluator |
| create | `experiments/moriarty-language/tests/k-correspondence.test.mjs` | K/Core/evaluator trace comparison |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP03.1: Pin a small K toolchain and kernel

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP03.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Define a JSON-to-Core loader and bounded configuration: pre-state, staged next-state, environment, effects, duties and work. Start with checked arithmetic, guards, one staged update and a surviving obligation. Specify run.py commands and inputs before installing or compiling under an admitted resource contract.
- [ ] Verify: Partial payment of 30 against a due of 100 leaves 70 due. Independent arithmetic, K and evaluator agree. Overflow, zero denominator and exhausted work reject without observable partial writes.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP03` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP03.2: Cover all admitted Core constructors

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP03.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Extend K only for constructors traced by SP01. Give explicit Complete, Pending and Rejected observations, arithmetic rounding and terminal rejection rules. Serialize the same complete observation record as the evaluator. Implement the CLI simulate command using this successor evaluator. Display pre-state, candidate effects, remaining duties and work. Reject unsupported constructors rather than silently abstracting them.
- [ ] Verify: Every admitted constructor has positive and distinguishing invalid traces. Simulation CLI, evaluator and K agree; the atomic sequential evaluator is not silently reused. Parser-to-Core elaboration has a separate preservation obligation; K execution is not source-parser equivalence.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP03` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP03.3: Prove the bounded semantic claims

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP03.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] State determinism, progress-or-defined-rejection, type preservation, finite termination and residual-duty conservation with explicit domains. SP03 requires determinism, progress-or-defined-rejection, type preservation, finite termination, residual-duty conservation and elaboration/evaluator correspondence for the frozen base Core. For each, name a K claim or a justified mechanized bridge before proving. Discharge every required claim for that domain. Record assumptions and trusted translation boundaries. Demonstrate a falsified candidate invariant to check the proof workflow.
- [ ] Verify: Any required base-Core theorem still open blocks SP03 completion; financial/compiler/ledger extensions retain their later gates. No unproved required theorem is labeled complete. Differential testing supports, but does not replace, correspondence proofs. K models Moriarty Core, never archived ZKIR.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP03` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
python3 experiments/moriarty-language/formal/k/run.py compile
python3 experiments/moriarty-language/formal/k/run.py traces --all
python3 experiments/moriarty-language/formal/k/run.py prove --claims experiments/moriarty-language/formal/k/claims.json
npm --prefix experiments/moriarty-language test
```

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Report-informed acceptance refinement

**Make K execute the financial distinctions.** The [refinement contract](../ROADMAP-REFINEMENT-2026-09-09.md) and [lesson/case crosswalk](report-lessons.json) add the following acceptance details to the existing task IDs. These remain specified-only.

- [ ] SP03.1: first demonstrate 30 paid against due100 leaves70; separately TX02 uses interest-first P100/I10/pay7→P100/I3. Keep the policies explicit rather than treating the examples as conflicting arithmetic.
- [ ] SP03.2: compare complete ordered effects, requests, residual principal/interest/fees and work between independent expectations, K and evaluator. Cover wrong units, rounding, repeated accrual, rollback and callback/intermediate observations as admitted constructors expand.
- [ ] SP03.3: discharge determinism, progress-or-defined-rejection, type preservation, finite termination, residual-duty conservation and elaboration/evaluator correspondence for the frozen base domain. Record later financial extensions as reopening affected claims; passing traces are supporting evidence.

## Exit gate

Working successor simulation CLI, runnable K semantics for the frozen base Core, checked metatheorems and an explicit elaboration/evaluator correspondence domain. Later financial extensions reopen changed claims.
