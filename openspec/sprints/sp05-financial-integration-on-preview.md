# SP05: Financial integration on Preview implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use the existing task contract for routine authorized edits and checks; expand only an unresolved interface or a resource-controlled campaign into an execution note. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Settle bounded loan and swap operations and compare every finalized financial effect.

**Architecture:** This sprint contributes to MC02, MC04; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP01. Stage scope: i2. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| create | `experiments/moriarty-midnight-financial/package.json` | Local and Preview commands |
| create | `experiments/moriarty-midnight-financial/fixtures/loan.json` | Independent loan trace and token mapping |
| create | `experiments/moriarty-midnight-financial/fixtures/swap.json` | Independent swap trace and token mapping |
| create | `experiments/moriarty-midnight-financial/src/differential.ts` | Full-effect comparison |
| create | `experiments/moriarty-midnight-financial/tests/differential.test.mjs` | Recipient, fee, asset and obligation mutations |
| inspect | `experiments/moriarty-midnight-network/` | Reusable Docker, wallet and finality integration |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP05.1: Freeze independently derived fixtures

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP05.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Use accepted atomic source and RP01-MC02. Specify counterparties, custody, ledger token versus nominal denomination, before-state, gross debit, fee, change, net credit and every residual duty. Reserve cleanup and public attempts before submission.
- [ ] Verify: Loan and swap expected records are derived independently of generated outputs. Wrong recipient, token, denomination, fee or debt identity causes comparison failure.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP05` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP05.2: Implement and exercise Docker settlement

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP05.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Generate financial Compact entry points and a complete receipt decoder. Compare initial state and all finalized effects with local expectations. Test failed transactions leave no financial state mutation. Reuse network utilities without exposing wallet secrets.
- [ ] Verify: Both local cases settle with real test-asset movement. A pure arithmetic kernel or a stored numeric answer cannot pass.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP05` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP05.3: Run the separately admitted Preview campaign

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP05.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Use existing Preview identities. Submit at most two attempts per case within the live gross-spend and submission reservation. Retain transaction bytes, indexed SUCCESS, canonical finalized block and exact readback. Preserve unsuccessful attempts.
- [ ] Verify: Actual loan and swap effects match the independent records. Keep this I2 financial.compact contract and its historical receipts uncertified. Later F3/mandatory acceptance qualifies the financial behavior through acceptance.compact with new evidence.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP05` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
npm --prefix experiments/moriarty-midnight-financial run build
npm --prefix experiments/moriarty-midnight-financial test
npm --prefix experiments/moriarty-midnight-financial run preview -- --case loan --max-attempts 2
npm --prefix experiments/moriarty-midnight-financial run preview -- --case swap --max-attempts 2
```

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Report-informed acceptance refinement

**Finalize actual financial loan and swap effects on Preview.** The [refinement contract](../ROADMAP-REFINEMENT-2026-09-09.md) and [lesson/case crosswalk](report-lessons.json) add the following acceptance details to the existing task IDs. These remain specified-only.

- [ ] SP05.1/.2: reuse the reviewed fixed-case comparator and custody wrappers as scoped utilities. Finish the real production bindings for asset IDs, participant roles, UTXO/signed-offer ownership, gross debit, fees, change and canonical finality. Existing skip-zk wrappers are not actual custody evidence.
- [ ] SP05.3: pass local Docker transaction checks, then finalize both loan and swap on Preview and compare every material effect with independent expectations. Wrong payer, wrong recipient and omitted-fee controls must exercise the callable production path.
- [ ] Publish each actual transaction ID and observed status. I2 proves financial integration only; its receipt remains an uncertified probe until SP09 verifies the mandatory acceptance lineage.

## Exit gate

MC02 integration evidence only. Neither hello-world settlement nor these uncertified transactions establishes mandatory PCD.
