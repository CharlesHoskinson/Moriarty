# PCD ledger-anchored acceptance

## Why

Midnight verifies each contract-call proof against the operation key stored in contract state. Contract-call proofs use a Blake2b transcript, and `verify_proof` accepts only Poseidon zk-stdlib proofs. A contract call therefore cannot re-verify its predecessor's proof. The current MC03 → MC04 → MC05 chain makes mandatory Preview acceptance wait for a recursive route that cannot carry contract proofs and that needs the unreleased `ledger-10`.

This package changes the terminal evidence-gated decision in one way. On-ledger history compliance follows by induction from constrained genesis, immutable operation keys and head read-then-write discipline. Recursion serves only bounded certificates. The hard gate is unchanged: verification-enabled mandatory acceptance on Midnight Preview.

The user directed that Midnight recursion be assumed to reach production within a few months. Certificates therefore stay release scope.

## What changes

- Add eighteen requirements in four capabilities owned by MC03, MC04, MC05 and MC06.
- Re-root stage `f3` on the Stage 0 seam instead of `f1` and `f2`. Keep `f2` in `release`.
- Remove MC03 from the MC04 and MC05 dependency lists.
- Add in-place amendment notes to affected MC03–MC06 scenarios. Requirement headings and locked task lines do not change.

## Capabilities

### New capabilities
- `pcd-ledger-correspondence`: seam, fused step relation, head discipline, deploy audit and bounds freeze (MC04).
- `pcd-mandatory-acceptance`: claim discharge, genesis and termination, digests, freshness, migration and property certificate (MC05).
- `pcd-composition`: split and join, release and reclaim, handoff and operators (MC06).
- `pcd-certificates`: bounded native certificates, segment certificates and dependency stops (MC03).

### Modified capabilities
- None. MC03–MC06 carry in-place amendment notes in their unarchived change directories.

## Impact

Affected artifacts:
- `openspec/moriarty-completion-program.json`: `f3` prerequisites, native-track stage purposes, MC03 and MC04 fields, MC04 and MC05 dependencies, feasibility stage text and the RP02 reason.
- `openspec/sprints/`: `sprints.json`, `coverage.json`, `report-lessons.json`, `asset-study.json`, `package-task-map.json` titles, the new `pcd-integration.json`, `verify.py`, `test_verify.py`, SP04, SP06, SP09, SP10 and the sprint README.
- `openspec/PCD-INTEGRATION-2026-09-11.md`, `openspec/PCD-ROADMAP-2026-09-11.md`, `ROADMAP.md`, the Charter and the September 9 refinement note.
- MC03–MC06 READMEs and scenario notes, and MC04/MC05 dependency prose.

Non-goals:
- No proof, campaign, deployment or public transaction runs.
- No resource ceiling changes. Certificate campaigns need a separate user-approved resource amendment.
- The SP01 sprint document, the September 7 reconciliation and the AFK change package are not edited.
- Site copy, `typed-schemas.md`, `bounds.json` and the other follow-ups are listed in the [amendment](../../PCD-INTEGRATION-2026-09-11.md#follow-ups-outside-this-revision).

The supersession and locked-task tables are in the [PCD integration amendment](../../PCD-INTEGRATION-2026-09-11.md#supersessions).
