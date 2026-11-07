# Sherlock VM consensus change (council run `sherlock-vm-council-20261107`)

Status: DRAFT v2.1 (revision round 1 plus cleanup), 2026-11-07. Written by the council chair (`fable-1`) from six independent positions, cross-critiques and the issue round recorded under `research/sherlock-vm-council-20261107/`. Base: `main` at `8fd34ed79`. This change specifies; it executes nothing. Every "measured" figure cites the run or file that recorded it.

## Why

Sherlock's general-purpose recursion is designed and partly measured but not finished:

- The owner's goal is "Sherlock on stagenet with full general-purpose recursion" (`docs/sherlock/stagenet-gp-recursion.md:11`). No general program has been proved end to end; the universal leaf has never been keyed (`docs/sherlock/stagenet-gp-recursion.md:96,235`).
- The factory has reported first failure A8 since 2026-10-26 (`qa/sherlock/status.json`, head `7e22b5823`), counts A 7/10, B 4/9, C 5/6, U 34/35, F 11/13, and `scripts/doctorwatson stop-rule` has been in STOP (exit 6) for about 295 h with no recorded escalation (`research/sherlock-vm-council-20261107/fable-3/position.md` §0).
- The U023 dispatch receipt run stopped at a 16,000 s wall cap without a receipt (`qa/sherlock/runs.jsonl`, run `20261026T105549Z-U023-dispatch-receipt`; `docs/sherlock/HANDOFF-2026-10-26.md:16`).
- Midnight Mainnet moved to ledger 9.1.0.1 on 2026-11-06 (`wiki/midnight-ledger-9-hardfork.md`; https://docs.midnight.network/relnotes/ledger-9). The workspace pins a ledger-9 release candidate: `midnight-ledger` rev `04c9c5d9` (2026-07-31), `midnight-proofs =0.8.3`, `midnight-circuits =7.2.4`, `midnight-zk-stdlib =2.3.5` (`Cargo.toml:28-42`); zkir-v3 `3.0.0-rc.2` (`crates/mori-ledger/src/profile.rs:35`). The "stagenet" the A/B evidence was earned on (`rpc.stagenet.shielded.tools`) reports specVersion 2000000, an rc-era runtime; Preview and Mainnet report 2001100 (`fable-3/position.md` §6 RPC reads, `fable-2/position.md` M6).
- The GP leaf producer emits six public lanes while the IVC node consumes seven (`crates/mori-backend/src/sherlock_c6_child.rs:817-834`; `crates/mori-recursion/src/ivc.rs:191-201`).

## What changes

1. **VM (keep, refine).** The 39-rule reference machine, the helper-microstep owner engine, the bounded images and `StepInv`, and in-circuit dispatch derived from the pre-state are kept. The leaf becomes one universal, one-hot-gated, S-step relation (or a committed finite family if measurement says so), with a versioned layout and qualification configuration bound into its identity. See `design.md` §2.
2. **Recursion (keep, close the interface).** Leaf -> linear self-recursive IVC node -> run decider -> `SherlockRun.acceptRun` is kept. The leaf statement is frozen at seven lanes including `steps`; `steps` and height are range-checked; the decider binds outcome and result from the proved state; keys are authenticated as a tuple. See `design.md` §3.
3. **Assurance (new requirements).** A typed evidence record, dependency fingerprints, honest negative-evidence accounting, dispatch coverage by committed equivalence class, and completeness/soundness stated as obligations per rule. See `assurance.md`.
4. **Workflow (factory only).** The change runs on `scripts/doctorwatson` (`next`/`check`/`heavy`/`stop-rule`); operations (mutating network steps) are separated from predicates; new T0 checks for traceability and target pins; a `network:` field and a T4 (mainnet) tier are specified as exact additions. See `workflow.md`.
5. **Midnight target.** Re-pin to the ledger 9.1.0.1 line with a keygen regression; Preview (new chain, genesis-pinned) is the qualification target; stagenet rows are frozen as history; Mainnet is an owner-authorized operation. See `design.md` §5.

## What is superseded

| Superseded | By | Note |
|---|---|---|
| `openspec/changes/mc03-native-recursive-proof/` (financial IVC on `midnight-aggregation` traits; every task unchecked; R3 blocked at k17) | the Sherlock IVC node (`crates/mori-recursion/src/ivc.rs`) | MC03's security and financial obligations are retained by reference (astra-2 critique); only its implementation route is superseded. |
| `openspec/changes/sherlock-stagenet-recursive-rollup/` SRPOC-001..009 as stagenet acceptance | `SMN-*` and `SWF-008/009` (Preview rows, `network:` field) | B predicates stay in the factory, re-targeted, and do not gate U023/C4. |
| The A102/A80 two-child parent (`crates/mori-recursion/src/parent.rs`, `sherlock_link.rs`) as a production path | the IVC node | kept as regression for A1-A10. |
| The original binary-tree-of-nodes proposal and the sequential-anchored-segments fallback (`docs/sherlock/stagenet-gp-recursion.md:146,170-174`) | linear IVC node | recorded fallbacks, not built. |
| `wiki/midnight-repositories.md` ledger-8 pin row; `docs/sherlock/stagenet-deployment.md:112-114` SDK pins | `wiki/midnight-ledger-9-hardfork.md`; `qa/sherlock/target-pins.json` (new) | |
| Speedup rows C1-C10 of `wiki-llm/recursion-perf/CONSENSUS.md` as plan items | the C0 phase profile (`openspec/changes/recursion-phase-profile/`) | adopt a lever only after its measured share. |
| `scripts/doctorwatson.sh` plan/attempt/adoption routes | `scripts/doctorwatson` factory | already superseded 2026-10-24; this change does not reference them. |

Not superseded: UIREQ-001..035 (`openspec/changes/sherlock-universal-interpreter-design/`), UISEC-001..012 (`security-requirements.md`), ZR01-16, MNR01-08, the Candidate B interfaces, the recursion-phase-profile change (RPP-001..010), and `AGENTS.md`. New requirement IDs refine them and cite them row by row in `traceability.md`.

## Exit of this change

This change is complete when the Preview qualification record for P1 (`fact(3)`), P2 (`list123`) and P3 (fresh program) exists under the new keys (U023 scenario 2, C4 flips) and every mandatory security and assurance gate for the claimed profile (`sherlock-helper-microstep/1`, the frozen inventory, Preview) passes: SVM-001..009, SRC-001..010, SRC-012, SRC-013, SAS-001..007, SAS-010..014, SWF-001..008, SWF-010..012, SMN-001..007. Conditional clauses apply whenever their trigger occurs and are then mandatory: SAS-008 whenever an optimization is enabled (otherwise recorded "not applicable: no optimization enabled"), SAS-009 whenever a formal property is claimed (otherwise "not applicable: no formal claim"), SRC-014 whenever S>1 is adopted (vacuous at S=1). Only work outside the claim may stay open: SRC-011 (throughput, optional), SWF-009 and M7 (mainnet), the `/2` profile, financial and generic-ADT extensions. An explicitly open or unexecuted applicable gate does not make the change complete (astra-2, astra-3). The Mainnet operation (`SWF-009`) is specified here and executed only on owner authorization after `openspec/MORIARTY-COMPLETION-PROGRAM.md:269` is amended.
