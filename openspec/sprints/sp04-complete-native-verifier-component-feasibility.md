# SP04: Complete native verifier component feasibility implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use the existing task contract for routine authorized edits and checks; expand only an unresolved interface or a resource-controlled campaign into an execution note. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Decide whether bounded native certificates work on the ledger before certificate campaigns.

**Architecture:** This sprint contributes to MC03, MC04; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites. The [PCD integration](../PCD-INTEGRATION-2026-09-11.md) repurposes it for bounded native certificates; the ledger-anchored core does not wait for it.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): SP01. Stage scope: f0a, f1-fixtures, f1. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record. Certificate campaigns also need a reviewed resource amendment, because the measured outer circuits need k = 18–19.

## File and interface map

Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.

| Operation | Path | Responsibility |
| --- | --- | --- |
| inspect | `experiments/moriarty-native-ivc-r3/` | Existing native experiment and preserved failed encoding |
| inspect | `evidence/pcd-midnight-native-2026-09-11/` | Reproduced pull request 738 and IVC measurements |
| create | `experiments/moriarty-ledger-adapter/probes/README.md` | Pinned `ledger-10` interfaces and certificate test recipes |
| create | `experiments/moriarty-ledger-adapter/probes/run.py` | Admitted E3 dispatch and retained verification |
| create | `experiments/moriarty-ledger-adapter/probes/resource-contract.json` | Explicit limits, stop conditions and the approved resource amendment |
| create | `experiments/moriarty-ledger-adapter/probes/fixtures.json` | Independent non-loan certificate fixture provenance |

Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.

## SP04.1: Freeze component authorship after F0 go

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP04.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Author certificate relations only after F0a preparation admission, against pinned pull request 738 / `ledger-10` sources. Constrain `vk_repr` and the state decider in the outer relation. Add a guard-constant lint for `VerifyProof` and `InnerProof`. Freeze actual module paths and command arguments in execution/SP04.md. Review implemented source and the resource amendment before F1.
- [ ] Verify: A missing `ledger-10` interface, unbounded constraint estimate, unapproved resource amendment or unavailable current reviewer prevents F1. Pinning a source branch does not prove Preview deployment compatibility.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP04` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP04.2: Generate independent small fixtures

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP04.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Generate small non-loan Poseidon zk-stdlib inner proofs and a verifier-test accumulator. Retain generation commands, valid witnesses, public artifact hashes and negative controls. Keep private witness bytes outside Git.
- [ ] Verify: Fixture generation never depends on a loan or other financial proof. Fixture source, transcript, key and SRS identity are independently checked.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP04` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

## SP04.3: Run E3 certificate controls

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP04.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Deploy a certificate entry point with constant guards on a pinned `ledger-10` devnet. Submit a certificate-bearing call and record the ledger's accumulator pairing result. Measure the charged fee against validation work.
- [ ] Verify: The valid call is accepted. A free or mismatched guard, substituted `vk_repr`, unbound inner instance, inner proof for another key and tampered inner proof each reject, or the lint rejects the build. An outer proof over an invalid inner proof is not evidence until the ledger pairing accepts. Native reference tests alone do not pass E3.
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

**Decide whether bounded native certificates work.** The [refinement contract](../ROADMAP-REFINEMENT-2026-09-09.md), [lesson/case crosswalk](report-lessons.json) and [PCD integration](../PCD-INTEGRATION-2026-09-11.md) add the following acceptance details to the existing task IDs. These remain specified-only.

- [ ] SP04.1/.2: author certificate relations from pinned sources; use independently generated non-loan fixtures so admission does not require the proof it is meant to permit.
- [ ] SP04.3: all E3 negative controls remain mandatory, with nontrivial positive fixtures. The ledger checks pairings; nothing computes them in-circuit. Native reference success alone cannot close E3.
- [ ] The first deliverable is a bounded, decisive certificate result. Failed fit or a refused resource amendment stops the certificate route under existing limits; the ledger-anchored core and eligible language work continue.

## Exit gate

E3 passes for the pinned `ledger-10` build, fixtures and candidate, or certificate status is recorded blocked. The ledger-anchored core is unaffected.
