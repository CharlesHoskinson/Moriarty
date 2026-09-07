# SP06: Real recursive financial history implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use superpowers:writing-plans to expand each task into a reviewed executable packet before behavioral edits. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Produce and independently verify a real two-step native financial proof.

**Architecture:** This sprint contributes to MC03; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP01, SP04. Stage scope: f2. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| create | `experiments/moriarty-native-ivc-r3/successor/relation-spec.md` | Reviewed fixed financial statement |
| create | `experiments/moriarty-native-ivc-r3/successor/resource-contract.json` | Replacement encoding and explicit proving envelope |
| create | `experiments/moriarty-native-ivc-r3/successor/run-reviewed.py` | Frozen native campaign entry point |
| create | `experiments/moriarty-native-ivc-r3/successor/artifact-schema.json` | Retained proof and canonical public inputs |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP06.1: Review the replacement encoding

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP06.md`.
- [ ] Freeze the RP01-MC03 statement and changed hypothesis after k17 exhaustion. Bind program, profile, policy, predecessor, authority, observations, next state, effects and work. Confirm genuine recursive predecessor verification and constrained genesis.
- [ ] Verify: The encoding/resource decision explains why this attempt differs. Source review and F1 evidence precede proving. A fresh folder does not reset charged time.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP06` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP06.2: Produce and retain the native episode

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP06.md`.
- [ ] Run only the admitted two-step fixed financial episode. Retain the real proof, canonical statement, key/SRS digests and resource receipts. Stop on the first failed control or resource ceiling.
- [ ] Verify: The successor checks the predecessor proof in the native relation. A host hash chain, MockProver run or nonrecursive table re-proof fails acceptance.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP06` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP06.3: Verify in a fresh process

- [ ] Specify inputs, exact file ownership and independent expected results in `openspec/sprints/execution/SP06.md`.
- [ ] Import only retained public bytes and explicitly inventoried required artifacts. Check complete native finalization. Mutate proof bytes, public state, context, key and accumulator independently.
- [ ] Verify: Valid proof passes and every required invalid control rejects. This closes only the fixed episode; the general DSL relation remains assigned to SP09.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP06` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
python3 experiments/moriarty-native-ivc-r3/successor/run-reviewed.py --contract experiments/moriarty-native-ivc-r3/successor/resource-contract.json --preflight-only
python3 experiments/moriarty-native-ivc-r3/successor/run-reviewed.py --contract experiments/moriarty-native-ivc-r3/successor/resource-contract.json --execute
python3 experiments/moriarty-native-ivc-r3/successor/run-reviewed.py --contract experiments/moriarty-native-ivc-r3/successor/resource-contract.json --verify-retained
```

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Exit gate

Actual independently verified native IVC evidence for the fixed financial profile, with current result audits. No general language completion claim.
