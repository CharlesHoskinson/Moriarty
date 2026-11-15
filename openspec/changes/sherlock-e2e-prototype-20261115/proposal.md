# Proposal: the Sherlock end-to-end prototype (`sherlock-e2e-prototype-20261115`)

Status: written requirements. This change builds, keys, proves and measures nothing.

## Why

Owner order 2026-11-15: "We need end-to-end OpenSpecs to finish the prototype of Sherlock. Have 3 fable 5.2 and 3 astra agents coordinate. Make sure the entire system is covered. Have a look at the local work in drwatt for architecture as well. We need an MADR and DDD for the project as well."

Sherlock is a universal, elastic proving co-processor for Midnight (`research/sherlock-final-design-20261111/TERMINOLOGY.md`). The requirements of the system sit in several changes and lane branches. No one change shows the whole path from a Compact source file to a settled proof on Preview, and no document maps every component and every LOOP row to its requirements.

## What Changes

- One spec per capability of the end-to-end path, in three slices: A (language, execution, integration), B (proving), C (wrap, settlement, operations). Each requirement is one EARS sentence with an ID, a `Verification:` line and one or more scenarios.
- `architecture/DDD.md`: bounded contexts, context map, aggregates, domain events, ubiquitous language.
- `architecture/decisions/`: one MADR record per real decision, with `INDEX.md`.
- `architecture/COVERAGE.md`: every component and every LOOP row mapped to a capability and EARS IDs.
- `design.md` and `tasks.md`: the build order to the working prototype.

## Reuse rule

This change does not restate a requirement that exists. It cites the ID. The cited changes are:

| Short name | Change | Where it is on 2026-11-15 |
|---|---|---|
| processor-final | `sherlock-processor-final-20261111` (governing) | this branch, with the 15 audit fixes of 95643d85f |
| modular-leaf | `sherlock-modular-leaf-20261111` | this branch |
| completion | `sherlock-completion-20261115` (62 EARS: RSL, RKY, STL, LPC, and CF-013, CF-014, ANC-013 to ANC-019, LW-020 to LW-022, ZK-012 to ZK-019, SQG-014 to SQG-016) | this branch, 0f859938e (the spec audit, merged by fast-forward) |
| vm-consensus | `sherlock-vm-consensus-20261107` (GP-001 to GP-004, SRC) | this branch |
| Mycroft | `mycroft/spec/` (277 EARS, prefix MYC) | worktree `mycroft-factory`, HEAD d1e8eed8d plus uncommitted files |
| governance | `sherlock-governance` | branch `t22-governance` |
| setup | `SETUP-SPEC.md` (ST, PF, SC) | branch `t16-elastic` a7a7b8867 |

A status scenario that says "worktree `spec-audit`" was written before the merge and means this branch. Where this change supersedes a cited requirement, the new requirement says so and names a decision record.

## Capabilities

Slice A: `sherlock-e2e-compact-compiler` (ECC), `sherlock-e2e-rsl-statement` (ERS), `sherlock-e2e-rsl-kits` (EKT), `sherlock-e2e-universal-interpreter` (EUI), `sherlock-e2e-compact-suite` (ECS), `sherlock-e2e-user-flow` (EUF). Slices B and C: see `architecture/COVERAGE.md` and `research/sherlock-e2e-spec-20261115/bus/CLAIMS.md`.

## Hard rules (owner-gated, never changed by this change)

Never k = 22 (every circuit k at most 21). Zero knowledge end to end; seeded proofs test-only; no plain witness to helpers. 100-bit security. One universal key set. Ledger 9.1.0.1 verifier unchanged. Wrap VK at most 50,000 bytes. One BLS12-381 wrap; leaves and nodes are small-field STARKs. Every proof is a fixed-size shard. GP-1 to GP-4 never narrowed. Midnight writes batched, one root per checkpoint. Case A only; case B later through Veritee. `K_MAX = 2`. MERGE-WORK 5,400 columns. The `x_j` lock form is canon. F-3 is replaced by setup mode. Design parameters, keys and key handling stay open until the end-to-end prototype.

## Canon rule

Only the owner's words are canon. A decision record states its authority: OWNER, LEAD PICK confirmed, or LEAD PICK for T15.

## Non-goals

No code, no key, no proof, no deploy, no push. No edit of a lane branch or of another worktree.
