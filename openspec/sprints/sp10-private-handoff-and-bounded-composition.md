# SP10: Private handoff and bounded composition implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use superpowers:writing-plans to expand each task into a reviewed executable packet before behavioral edits. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Allow independent participants to continue, split and join private financial histories without losing duties or authority limits.

**Architecture:** This sprint contributes to MC06; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP09. Stage scope: composition. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| create | `experiments/moriarty-composition/spec/operators.md` | Five operator semantics and compatibility judgments |
| create | `experiments/moriarty-composition/spec/handoff.schema.json` | Artifact recipients, availability and recovery ownership |
| create | `experiments/moriarty-composition/tests/composition.test.mjs` | Residual, aliasing and duplicate predecessor controls |
| create | `experiments/moriarty-composition/proof/relation-spec.md` | Private successor and actual branch/join relation |
| create | `experiments/moriarty-composition/isolation/compose.yaml` | Separate OS/container secret boundaries |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP10.1: Realize every required operator

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP10.md`.
- [ ] Implement sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging using the RP01 definitions. Check read/write compatibility and explicit ordering. Partition and conserve a global finite work measure.
- [ ] Verify: A required unsupported operator leaves MC06 open. Shared-state conflicts cannot masquerade as disjoint branches; async Pending does not claim atomic completion.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP10` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP10.2: Demonstrate independent private continuation

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP10.md`.
- [ ] Inventory proof, commitment openings, witness fragments, recipients and recovery responsibility. Run Alice and Bob under distinct OS users or isolated containers. Give Bob only the allowed handoff package and prove the successor.
- [ ] Verify: Denied-read evidence establishes harness separation. Missing allowed witness causes an explicit unavailable outcome. Plain shared-process private directories are insufficient.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP10` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP10.3: Prove split/join and exercise recovery

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP10.md`.
- [ ] Generate actual split, branch and join proofs with distinct predecessor identities and compatible policies. Test duplicate inputs, excessive fan-in, authority amplification, debt erasure, reset budgets, fill/cancel races and unavailable handoff recovery.
- [ ] Verify: The same acceptance path enforces durable consumption and residual duties. Scoped privacy evidence states leakage and availability assumptions. Changed MC04/MC05 domains are requalified.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP10` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
npm --prefix experiments/moriarty-composition test
npm --prefix experiments/moriarty-composition run verify-isolated
npm --prefix experiments/moriarty-composition run verify-preview
python3 experiments/moriarty-composition/proof/run-reviewed.py --contract experiments/moriarty-composition/proof/resource-contract.json --verify-retained
```

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Exit gate

Real isolated continuation and all required bounded composition operators accepted under the requalified native/ledger lineage.
