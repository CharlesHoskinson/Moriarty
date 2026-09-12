# SP06: Real recursive financial history implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use the existing task contract for routine authorized edits and checks; expand only an unresolved interface or a resource-controlled campaign into an execution note. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Produce and independently verify an off-ledger segment certificate over the Moriarty step.

**Architecture:** This sprint contributes to MC03; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites. The [PCD integration](../PCD-INTEGRATION-2026-09-11.md) repurposes it for bounded native certificates; the ledger-anchored core does not wait for it.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP01, SP04. Stage scope: f2. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record. Certificate campaigns also need a reviewed resource amendment.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| inspect | `experiments/moriarty-native-ivc-r3/` | Retired fixed relation and preserved k17 failure; history only |
| inspect | `evidence/pcd-midnight-native-2026-09-11/` | Reproduced IVC segment measurements |
| create | Segment certificate root recorded in `openspec/sprints/execution/native-paths.json` at `native-path-freeze` | Segment relation specification, resource contract, frozen campaign entry point and artifact schema |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP06.1: Review the replacement encoding

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP06.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Freeze the RP01-MC03 statement subset of the Moriarty step that segments fold. Bind program digest, head, revision, state commitment, authority, observations, effects and work. Retire the fixed 54-limb relation; its k17 failure stays failed evidence. Confirm the outer relation constrains `vk_repr` and the state decider.
- [ ] Verify: The segment relation review explains why it differs from the failed encoding. Source review, E3 evidence and an approved resource amendment precede proving. A fresh folder does not reset charged time.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP06` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP06.2: Produce and retain the native episode

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP06.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Run only admitted E5 segments at 1, 10 and 100 steps, with segment length at most 16 until benchmarked. Retain real certificates, canonical statements, key/SRS digests and resource receipts. Stop on the first failed control or resource ceiling.
- [ ] Verify: Each segment certificate checks its predecessor accumulator in the native relation. A host hash chain, MockProver run or nonrecursive re-proof fails acceptance.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP06` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP06.3: Verify in a fresh process

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP06.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Import only retained public bytes and explicitly inventoried artifacts. Verify natively, then import the final state through the certificate entry point on the pinned `ledger-10` devnet. Mutate proof bytes, public state, context, key and accumulator independently.
- [ ] Verify: Valid certificates pass and every invalid control rejects. This closes segment certificates only; the general DSL relation remains assigned to SP09.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP06` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## Verification entry points

Commands are frozen at `native-path-freeze` and the SP06.1 review. The retired `experiments/moriarty-native-ivc-r3/successor/` commands are not entry points. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Report-informed acceptance refinement

**Produce and independently verify segment certificates.** The [refinement contract](../ROADMAP-REFINEMENT-2026-09-09.md), [lesson/case crosswalk](report-lessons.json) and [PCD integration](../PCD-INTEGRATION-2026-09-11.md) add the following acceptance details to the existing task IDs. These remain specified-only.

- [ ] SP06.1/.2: implement the segment relation only after atomic acceptance, RP01-MC03, path freeze, E3 and an approved resource amendment. Bind arithmetic constants, head identity, liabilities and work.
- [ ] SP06.3: verify retained serialized artifacts in a separate process; mutate proof bytes, state, context, keys and accumulators. The ledger discharges the accumulator pairing, not a host verdict.
- [ ] Keep the segment scope visible. This milestone does not establish general source execution, private branching or on-ledger consumption.

## Exit gate

Actual independently verified segment certificates for the admitted profile, imported through a `ledger-10` certificate entry point, with current result audits. No general language completion claim.
