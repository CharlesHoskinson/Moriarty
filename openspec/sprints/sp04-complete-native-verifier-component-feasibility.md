# SP04: Complete native verifier component feasibility implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use the existing task contract for routine authorized edits and checks; expand only an unresolved interface or a resource-controlled campaign into an execution note. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Establish a complete native-to-ledger verifier route before expensive financial proving.

**Architecture:** This sprint contributes to MC03, MC04; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP01. Stage scope: f0a, f1-fixtures, f1. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| inspect | `experiments/moriarty-native-ivc-r3/` | Existing native experiment and preserved failed encoding |
| create | `experiments/moriarty-ledger-adapter/probes/README.md` | Pinned interfaces and component test recipes |
| create | `experiments/moriarty-ledger-adapter/probes/run.py` | Admitted P1/P2/P3 dispatch and retained verification |
| create | `experiments/moriarty-ledger-adapter/probes/resource-contract.json` | Explicit limits and stop conditions |
| create | `experiments/moriarty-ledger-adapter/probes/fixtures.json` | Independent non-loan fixture provenance |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP04.1: Freeze component authorship after F0 go

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP04.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Implement the selected route only after F0a preparation admission. Freeze actual Rust module paths and command arguments in execution/SP04.md against pinned sources. Specify canonical export bytes and witness availability. Review implemented source and a conservative resource amendment before F1.
- [ ] Verify: Missing complete verifier API, unbounded constraint estimate or unavailable current reviewer prevents F1. Merely choosing a source family does not prove Preview deployment compatibility.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP04` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP04.2: Generate independent small fixtures

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP04.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Use the reconciliation recipes: native transcript reference, a separately admitted small non-loan Poseidon IVC derivative and verifier-test accumulator. Retain generation commands, valid witnesses, public artifact hashes and negative controls. Keep private witness bytes outside Git.
- [ ] Verify: Fixture generation never depends on the MC03 financial proof. Fixture source, transcript, key and SRS identity are independently checked.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP04` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP04.3: Run all three component predicates

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP04.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] P1 compares full ported IVC transcript/preparation/carried accumulator in the selected outer stack. P2 checks canonical export/import roundtrip. P3 constrains the complete final accumulator/pairing decision with nontrivial valid and direct-assignment-invalid inputs.
- [ ] Verify: P1 and P3 pass in the actual chosen outer circuit stack. Native reference tests alone do not pass them. All three are required before F2; a separately admitted P2 may proceed after F0/F0a when P1/P3 remain incomplete or failed. P2 has no exemption from F0/F0a admission.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP04` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
python3 experiments/moriarty-ledger-adapter/probes/run.py --contract experiments/moriarty-ledger-adapter/probes/resource-contract.json --preflight-only
python3 experiments/moriarty-ledger-adapter/probes/run.py --contract experiments/moriarty-ledger-adapter/probes/resource-contract.json --execute
python3 experiments/moriarty-ledger-adapter/probes/run.py --contract experiments/moriarty-ledger-adapter/probes/resource-contract.json --verify-retained
```

Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.

## Report-informed acceptance refinement

**Decide whether the full native verifier works.** The [refinement contract](../ROADMAP-REFINEMENT-2026-09-09.md) and [lesson/case crosswalk](report-lessons.json) add the following acceptance details to the existing task IDs. These remain specified-only.

- [ ] SP04.1/.2: port canonical codecs, native export and outer verification from pinned sources; use independently generated non-loan fixtures so admission does not require the loan proof it is meant to permit.
- [ ] SP04.3: all P1/P2/P3 controls remain mandatory in the selected stack, including carried accumulators and final pairing, nontrivial positive fixtures and direct-assignment negatives. Native reference success alone cannot close outer verification.
- [ ] The first deliverable is a bounded, decisive complete-verifier result. Failed fit stops that route under existing limits; continue eligible language work without a new infrastructure campaign.

## Exit gate

F1 passes for its exact stack, fixtures and candidate. If no route fits, preserve interface-blocked status and stop dependent native/ledger campaigns.
