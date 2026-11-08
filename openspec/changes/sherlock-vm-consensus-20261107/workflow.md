# DoctorWatson workflow: implementation and deployment

Status: CONSENSUS v2.1 (council vote 6/6 ACCEPT at 2026-11-07 21:29Z; text unchanged from DRAFT v2.1; amended 2026-11-08 (3) by the owner decisions in `dissent.md` "Resolved 2026-11-08", "Resolved 2026-11-08 (2)" and "Resolved 2026-11-08 (3)" (E11 required as task 2.0; E16 memory guard; E17 seeded caps; gate decision files)). Adopted from fable-3's `workflow-proposed.md` with the council's ISSUE resolutions. Every command exists at `8fd34ed79` unless listed in §8 "Exact additions" (E1-E18; the M6 operation commands depend on E12/E13; E11, E16-E18 are the 2026-11-08 (3) factory work of task 2.0) (verified from `scripts/doctorwatson --help`, `models/doctorwatson/factory/CONTRACT.md`, `qa/tiers/*.sh`, `.githooks/pre-commit`). Requirements SWF-001..013.

## 1. Engine and truth

- Engine: `scripts/doctorwatson` (`python3 -m factory`, `models/doctorwatson/factory/`). Loop: `AGENTS.md:13-20`.
- Truth: `qa/sherlock/predicates.yaml` (what must hold), `qa/sherlock/status.json` and `runs.jsonl` (what was observed), `qa/sherlock/budgets.json` (measured caps and, since 2026-11-08 (3), seeded requirement caps marked `fixed: true`), `qa/sherlock/decisions/*.json` (committed gate decisions, E18), git (observed source). Nothing else is state (`CONTRACT.md:116-119`).
- Not used: `scripts/doctorwatson.sh --inspect/--execute/--cleanup` and `models/doctorwatson/{delivery,run_selected,reconciliation,cleanup}.py` (superseded 2026-10-24, `docs/DOCTORWATSON*.md:3`).
- Reviews are inputs; the only reviewer wait is the bounded T3 cold review (`AGENTS.md:22-25`; `scripts/qa/check-f11.sh`). The AgentBus is a doorbell, never state (`CONTRACT.md:116-135`).

## 2. Requirement -> predicate -> check -> evidence -> gate

Every ID in `requirements.md` maps to at least one row in `predicates.yaml` with `id, set, title, tier, check, heavy, budget?, depends, source, holdout, network?, retired?`. `source` is `openspec/changes/sherlock-vm-consensus-20261107/requirements.md#<id>` or a `specs/<cap>/spec.md#requirement-...` anchor.

1. No executable check yet: `check: "scripts/qa/unimplemented.sh <ID>"` (exit 2), visible as failing in `next`. No row is omitted to improve counts.
2. Behavioural requirement: a visible regression test (`check-uNNN.sh` pattern, `scripts/qa/check-u021.sh:5-19`) and an evaluator-owned holdout under `qa/holdout/cases/`; the holdout is the arbiter. Interface frozen first by `qa/holdout/contracts/<ID>.json` and `scripts/doctorwatson contract-check <ID>`.
3. Heavy check: names a `budget` key; first run measures, later runs are capped at 2x (`CONTRACT.md:78`); same `cmd` and `head` retry after failure needs `--hypothesis` (`CONTRACT.md:96`). Exception (2026-11-08 (3), SRC-015, E17): a budget with `fixed: true` is a requirement cap; `heavy` enforces it from the first run and never rewrites it (today `limits_for` treats a null measurement as unmeasured, so task 2.0 adds the `fixed` branch before any throughput row runs). Every heavy process runs under a cgroup memory cap (E16) and a concurrent set under `heavy --concurrent P --aggregate-cap 100G` (E11).
4. Milestone exit: a C-style composite (`scripts/qa/check-c.sh`, `depends` = member rows).
5. `traceability.md` is regenerated from `predicates.yaml` + `status.json` by `scripts/qa/check-traceability.py --render`; it is never hand-maintained after M1.

Sets: new rows go into the existing sets. A-set: recursion chain rows (`A2p`, `A3p`, `A4p`, `A6p`, `A8p`, `A9p`, `A10p` with `network: preview`; `A11` keygen regression; `A12` leaf shape; `A13` chain negatives). U-set: `U023a..U023f` fragment and dispatch-partition receipts, `U023` unchanged. B-set: Sherlock's synthetic-rollup qualification on FSYN-generated data (amended 2026-11-08 (2); SWF-013, SSRD-029): the local rows `B1`-`B3`, `B6`, `B7`, then the Preview rows `B4p`, `B5p`, `B8p`, `B9p`, then the Mainnet rows `B4m`, `B5m`, `B8m`, `B9m`; outside the C4 closure by owner decision ("keep them out of C4"), so they gate only the synthetic-rollup gates (tasks 6.6, 7.4) and never C4. No B row reads Fenrir files or depends on a Fenrir system (SWF-014; until task 1.9 lands, B1-B3 still read a Fenrir checkout, recorded in their rows). F-set: `F14` traceability, `F15` target pins, `F16` retired-row test, `F17` no-Fenrir-path guard (placeholder until task 1.9); since 2026-11-08 (3): `F18` memory guard, `F19` profile harness ready, `F20` Quint aggregation model, `F21` Lean composition theorems, `F22` tree-node row probe, `F23` GPU toolchain smoke (optional). A-set additions (2026-11-08 (3)): `A15` C0 chain profile, `A16` node step at k=19, `A17` thread scaling, `A18` `entry 7;` throughput, `A19`/`A19b` fact(3) and list123 throughput, `A20` S-slot leaf, `A23` three concurrent nodes; optional until their gate keeps them: `A21` GPU node step, `A22` tree fold, `A24`-`A27` GPU kernels and integration. Gate decisions are committed JSON under `qa/sherlock/decisions/` (`speed-levers.json` for G-A/G-A'/G-B/G-C), written by the lead, read by the rows they unlock. The headline counts therefore stay comparable (fable-3; no new set letter unless the owner wants one, §8 E4).

## 3. Operations (mutating steps)

A predicate observes; it never deploys, calls or spends. A mutating step is an operation (SWF-003), listed in `tasks.md` §"Operations" with:

| Field | Meaning |
|---|---|
| `op` | id, e.g. `OP-PV-DEPLOY` |
| `preconditions` | predicate ids passing in `status.json` with their dependency closure unchanged (not stale) |
| `command` | `scripts/doctorwatson heavy --id <op> --budget <key> [--hypothesis ...] -- <cmd>` |
| `postcondition` | the read-only predicate that becomes checkable afterwards |
| `authorization` | `none` (local), `owner` (public network), `owner+checklist` (mainnet, T4) |
| `receipt` | committed path `qa/sherlock/receipt-traces/<op>/receipt.json` (tx hash, block, address, key lineage, state SHA-256, pins row, chain identity) |
| `rollback` | §7 |

Pattern in use: `doctorwatson heavy --id A4deploy -- node tools/sherlock-sdk/deploy-current.mjs`; `heavy --id A8 -- node tools/sherlock-sdk/call-current.mjs --submit` (`docs/sherlock/stagenet-deployment.md:190-191,300`). Commit order (SWF-005): the receipt is committed first (code/receipt commit, hash-bound to the check), then `scripts/doctorwatson check <postcondition>` and `scripts/doctorwatson state-commit`, which commits only `status.json` and `runs.jsonl` (`models/doctorwatson/factory/evalloop.py:39-46`). An uncommitted receipt is ineligible; a committed receipt followed by its state commit is eligible.

Freshness (SAS-002): eligibility follows the declared dependency closure (source closure, pins, key manifests, chain identity), not HEAD equality; a state-only or receipt-only commit preserves eligibility only while every declared dependency hash (including relied-on receipts and artifacts) is unchanged; the run's HEAD is kept for provenance. E7's HEAD flag is an interim conservative signal only.

Note (fable-3 W4): `validate` today checks only `id/set/tier/title/check/heavy/holdout/budget/depends` (`predicates.py:85-130`); `network:` and `retired:` rows pass validation but are inert until E6/E10 land; ids are free strings, so `A2p`, `U023a` are valid.

## 4. Tiers and gates

| Tier | Runner | Content | When |
|---|---|---|---|
| T0 | `qa/tiers/t0.sh` (pre-commit hook) | holdout verify, `validate`, factory unit tests, profile isolation, `cargo check` of touched crates, fmt; new: F14 traceability, F15 static pins; since 2026-11-08 (3): F18 memory guard (factory tests), F19 profile harness, F20 `quint test` of the aggregation model, F21 `lake build SherlockSpec` (exit 2 while the toolchain is absent), F23 GPU smoke (optional) | every commit |
| T1 | `qa/tiers/t1.sh` | `soundness_mutation` (after the SAS-003 fix), workspace tests, `check --tier T1` | before merge to `main` |
| T2 | `qa/tiers/t2.sh` | `preflight` (with `MemAvailable` and swap since 2.0), `check --tier T2` (heavy proofs under budgets; since 2026-11-08 (3) every process under a 48 GiB cgroup cap, concurrent sets under `--concurrent P --aggregate-cap 100G`, throughput rows under seeded caps) | per milestone with native work |
| T3 | `qa/tiers/t3.sh [--review]` | checklist, optional cold review, `check --tier T3` (read-only network predicates); new: live target identity step (E3) | per public-network milestone |
| T4 | new `qa/tiers/t4.sh` | mainnet checklist, owner authorization, rollback rehearsal receipt, `check --tier T4` | mainnet only |

Gate rule: a milestone is complete when `scripts/sherlock-next` names no failing and no stale mandatory (non-retired, non-optional) predicate among the milestone's rows, every such row's declared dependency closure is unchanged since its passing run (SAS-002; the run's HEAD is provenance, not the rule), and `scripts/doctorwatson stop-rule` exits 0. A `BLOCKED-BY` row is failing. Optional rows (E14) are reported, not gating; an unfiltered whole-catalog run (`check --include-optional`) stays available as a broader report.

Milestone row sets (2026-11-08, amended 2026-11-08 (2)): the Sherlock M6 gate is the C4 dependency closure. The synthetic-rollup B rows are mandatory (SWF-013) but sit outside that closure by owner decision ("keep them out of C4"), so they gate only the synthetic-rollup Preview gate (task 6.6) and the synthetic-rollup Mainnet gate (task 7.4). Today `qa/tiers/t2.sh` and `qa/tiers/t3.sh` sweep a whole tier (`check --tier T2|T3`), and that sweep now includes the B rows. B3 is a heavy row. Until E15 scopes the tier runners to a milestone row set, the Sherlock M6 gate is read from the C4 closure (`scripts/doctorwatson check C4` and its members), not from the result of a whole-tier sweep. A failing B row in a whole-tier sweep does not block Sherlock.

## 5. Daily loop

1. `scripts/sherlock-next`; work the first failing predicate (retired rows are listed, not selected).
2. Heavy work only via `scripts/doctorwatson heavy --id <ID> --budget <key> -- <cmd>`; queue with `scripts/doctorwatson q submit` when the lock is busy; never plain `cargo test` for proofs. Since 2026-11-08 (3): no heavy run before task 2.0 (F18) has landed; every heavy process is capped at 48 GiB by the kernel and a concurrent set at 100 GiB (SRC-016); heavy tests are `#[ignore]` in plain `cargo test` and `RUST_TEST_THREADS <= 4` for the prover crates.
3. On failure read `qa/sherlock/traces/<run>.log`; retry with `--hypothesis`; after two same-class failures shrink the reproducer (`AGENTS.md:17-18`).
4. `scripts/doctorwatson check <ID>`; commit code or tests (hook runs T0; docs-only commits need `DW_DOCS_COMMIT=1`); `scripts/doctorwatson state-commit`.
5. `scripts/doctorwatson stop-rule`; on exit 6 stop and bring the owner the trace and hypothesis (SWF-006).
6. Helpers via `scripts/doctorwatson spawn --vendor ... --as ... --role ... --brief ...` (sandbox `read-only` unless implementer/lead).

## 6. Deployment path

Order: `undeployed` (local docker at 9.1.0.1: node :9944, indexer :8088, proof server :6300, pre-funded genesis wallet) -> Preview (ledger 9.1.0.1, specVersion 2001100, new chain since 2026-11-07, faucet) -> Mainnet (Blockfrost project, real NIGHT/DUST, no faucet). Sources: https://docs.midnight.network/guides/networks-and-environments, `wiki/midnight-ledger-9-hardfork.md`. Stagenet (`shielded.tools`, specVersion 2000000) is retired for new qualification; its rows carry `retired:` and remain inspectable (SWF-011). No stagenet diagnostic call will be made (owner decision 2026-11-08: "not testing on old stagenet"). The synthetic-rollup B rows follow the same order: local first, then Preview, then Mainnet (SWF-013).

Mainnet (2026-11-08 amendment, `openspec/MORIARTY-COMPLETION-PROGRAM.md:270-283`): a mainnet operation is a T4 operation. It runs only after local Docker at 9.1.0.1 has passed, the Preview qualification receipts are committed, the Preview rollback rehearsal receipt is committed and the pre-deploy checklist below has passed. Each mainnet operation also needs explicit owner authorization at the time it runs, within an owner-set per-operation spend ceiling and with no automatic retry (SWF-009). The amendment authorizes no specific spend.

| Step | `undeployed` | Preview | Mainnet |
|---|---|---|---|
| Pins | `target-pins.json` row | same + genesis hash | same + Blockfrost endpoints |
| Keys | keygen regression vs manifest (SRC-008/009) | same | same, frozen |
| Local acceptance/refusal | A6-shape (`validate_call`, proof-verifying on) | required before deploy | required before deploy |
| Deploy | op, `none` | `OP-PV-DEPLOY`, `owner` | `OP-MN-DEPLOY`, `owner+checklist` |
| Call | op | `OP-PV-CALL` | `OP-MN-CALL` |
| Inclusion | A8-shape bound to `stmt` | `A8p` | `A8m` |
| Rejections | A9-shape holdout | `A9p` (`rejections-preview`) | `A9m` (non-consuming cases only) |
| Readback | A10-shape, manifest-selected | `A10p` | `A10m` |
| Fees | B8-shape vs `min_block_price` and factors (Sherlock evidence in the A8-shape receipt, SMN-005) | `A8p`; synthetic rollup `B8p` | `A8m`; synthetic rollup `B8m` |
| Synthetic-rollup batch (FSYN data) | B1-B3, B6, B7 local | `B4p`, `B5p`, `B8p`, `B9p` (`OP-PV-SYNTH-ROLLUP`) | `B4m`, `B5m`, `B8m`, `B9m` (`OP-MN-SYNTH-ROLLUP`) |

Pre-deploy checklist from ledger 9: `validatePassword` before any transaction (`HANDOFF-2026-10-26.md:46`); at least two DUST spends (#838); maintenance authority declared, held by the owner, and equal to the identity published in `target-pins.json` (SMN-007); `TooManyPublicInputs` and 256-op cap checked (SMN-006); wallet seed backed up encrypted after every deployment.

## 7. Rollback

Chain state is not reversible; settle cases consume statements (`stagenet-deployment.md:228-231`).

Maintenance authority (decision 2026-11-08, SMN-007): `SherlockRun` is deployed WITH a maintenance authority on Preview and on Mainnet, because items 2 and 5 below depend on maintenance key replacement and the mainnet rollback must be rehearsed on Preview. This is a trust assumption: whoever holds the authority can install a verifier key, so the authority can change what the contract accepts. The owner holds it. Its public identity is published in `qa/sherlock/target-pins.json`, which holds no secrets. Before every maintenance update, A11 (keygen regression) passes with its dependency closure unchanged; after it, a receipt of the observed on-chain key state is committed under `qa/sherlock/receipt-traces/<op>/`. midnight-node #2375 is checked before any key removal. A later change may freeze or renounce the authority once the keys are stable (`tasks.md` "Later changes").

Rollback means (SWF-010):
1. A failed or orphaned deployment is recorded `orphaned` with address and block; the fixture is re-pinned to the last good address; nothing is deleted (`runs.jsonl` append-only).
2. A maintained contract may receive a replacement key by maintenance update (`IrInsert`/`IrRemove`, ledger 9.1.0.0 notes), after a passing A11 regression; midnight-node #2375 (`VerifierKeyRemove` fails with `BalanceCheckOverspend` on 9.1.0.0 clients) is checked first. Inserting a key does not revoke an old accepted key, and a client re-pin revokes nothing on chain: the receipt records the observed on-chain key state after the update. An unsafe old key that cannot be removed means redeploy, retire the address and record the residual exposure.
3. A contract without maintenance authority is redeployed at a new address; the old one is listed `retired`. A contract holding active state (funds, duties, consumed statements) is never discarded: recovery migrates it or records the exposure; only empty instances are abandoned. This item covers earlier or orphaned instances; new `SherlockRun` deployments in this change carry an authority (SMN-007).
4. Git: `git revert` of the fixture/receipt commit, then `scripts/doctorwatson check <affected ids>` and `state-commit`.
5. Mainnet rollback is rehearsed on Preview first (deploy -> bad key -> maintenance replace -> readback) and the rehearsal receipt is committed before `t4.sh` can pass (SWF-009).

## 8. Exact additions (not present at `8fd34ed79`)

| # | Addition | Where | Size |
|---|---|---|---|
| E1 | Traceability check (every requirement ID has >= 1 row; every `source:` anchor resolves; every row has a visible check or holdout; `--render` writes `traceability.md`) | `scripts/qa/check-traceability.py`; row `F14` (T0); `qa/tiers/t0.sh` step | one Python file (stdlib + PyYAML) |
| E2 | Target pins: per-network row (ledger crate rev, node, Compact toolchain, runtime, midnight-js, wallet-sdk, proof-server image, indexer, genesis hash, specVersion); static compare with `Cargo.toml`/SDK lockfile | `qa/sherlock/target-pins.json`; `scripts/qa/check-target-pins.py`; row `F15` (T0) | one JSON, one Python file |
| E3 | Live target identity: `midnight_ledgerVersion`, `state_getRuntimeVersion.specVersion`, `chain_getBlockHash(0)` equal the pins row; exit 3 if unreachable | `qa/tiers/t3.sh` step calling `check-target-pins.py --live <network>` | one shell line |
| E4 | Optional new set letter | `models/doctorwatson/factory/predicates.py:11` | one line; not required (new rows use A/U/B/F) |
| E5 | Tier T4 and runner | `predicates.py:12` (`TIERS`), `cli.py` `--tier` choices, `qa/tiers/t4.sh` | two lines + one script modelled on `t3.sh` |
| E6 | Optional `network:` field validated against the pins file; `validate` rule: T4 row depends on a T3 row with the same key lineage | `predicates.py` schema | small |
| E7 | Freshness by declared dependency closure: each row declares `deps:` (source paths/globs, spec/contract files, pins row, key manifests, chain identity, receipt hashes); `next` reports a passing entry `stale` iff any declared dependency changed since its run (git diff over the declared paths plus hash compare for pins/manifests/receipts); no blanket directory exclusion; a state-only or receipt-only commit preserves eligibility; the run `head` is kept as provenance; a HEAD-only comparison may be shipped first as an interim conservative signal but is not the gate | `predicates.py` schema, `status.py:view`, `cli.py:cmd_next` | small-medium |
| E8 | Receipt-first readers; candidate by manifest, not mtime; any-call acceptance removed | `scripts/qa/check-a8.sh`, `check-a10.sh`, new `check-*p.sh` | script edits |
| E9 | Done 2026-11-08 (owner decision "amend the program for mainnet"): `openspec/MORIARTY-COMPLETION-PROGRAM.md:270-283` supersedes the `:269` mainnet ban, which stays visible struck through. Mainnet submissions are permitted only as T4 operations after local Docker at 9.1.0.1 has passed, the Preview qualification receipts and the Preview rollback rehearsal receipt are committed and the pre-deploy checklist has passed; each operation still needs explicit owner authorization when it runs, under an owner-set per-operation spend ceiling with no automatic retry (program resource table). The amendment authorizes no specific spend | docs | done (task 7.1) |
| E10 | `retired: "<reason or anchor>"` row field; `next` skips retired rows for FIRST FAILING and prints `RETIRED: ...`; `check --tier`/`--set` sweeps skip them (`cli.py _select`); explicit `check <ID>` still runs; `validate` refuses an active row depending on a retired row; composites rewired; factory unit test covering `next`, `check --tier T3`, `check --set A`, `check A8` | `predicates.py` schema, `status.py:34-47`, `cli.py:50`, `models/doctorwatson/factory/tests/` | about 40 lines |
| E11 | Heavy concurrency: `heavy --concurrent P --aggregate-cap 100G -- CMD` (or P commands from a file) runs P processes in one parent cgroup scope with `MemoryMax=100G` and per-child scopes at the per-process cap, takes the lock once, records per-process and aggregate peaks, P and the makespan (`runs.jsonl` gains `aggregateRssBytes`, `concurrency`); `queue.acquire_turn` admits a process only while the sum of admitted caps (heavy and gpu) stays <= 100 GiB. **Required since 2026-11-08 (3)** (owner: "48 GB per process, 100 GB aggregate"; SRC-016); task 2.0; needed by the leaf workers (A18, A19), the concurrency probe (A23) and the tree (A22) | `heavy.py`, `queue.py`, `budgets.py` (`cap.aggregateRssBytes`) | task 2.0 |
| E16 | Memory guard (2026-11-08 (3), SRC-016): `run_command` wraps the job in `systemd-run --user --scope --collect -q -p MemoryMax=<cap> -p MemorySwapMax=0` when `systemd-run` exists (else the RSS poll, receipt `cgroup: false`); `cap_rss` is never None (`DW_DEFAULT_RSS_CAP` 48 GiB) and never above 48 GiB; OOM reported as `capExceeded: "rss"` from the scope's `memory.events` or exit 137; preflight adds `MemAvailable >= cap + DW_MEM_RESERVE_GIB (20)` and `swap used <= 4 GiB`, exit 3; the m2 test commands get `RUST_TEST_THREADS=4`; factory unit tests (F18) | `heavy.py`, `preflight.py`, `models/doctorwatson/factory/tests/` | task 2.0 |
| E17 | Seeded requirement caps (2026-11-08 (3), SRC-015): `budgets.json` entries may carry `fixed: true`, `cap.aggregateRssBytes` and `measured.aggregateRssBytes`/`measured.concurrency`; `limits_for` returns a fixed cap even while `measured.wallSeconds` is null; `record_measurement` writes the measurement but leaves a fixed cap unchanged; `budgets.py` ignores the new fields until then, so the seeded entries are inert placeholders | `budgets.py`, `heavy.py:416,448` | task 2.0 |
| E18 | Gate decision files (2026-11-08 (3)): `qa/sherlock/decisions/speed-levers.json` (G-A, G-A', G-B, G-C: C0 metrics d, m, q, U, f3, wall per thread count, node rows and k, aggregate peaks; decisions: thread count, k19, S, GPU open/closed, tree open/closed, with the C0 metric behind each) and the SRC-013 U/F decision JSON; written by the lead, committed, read by the predicates they unlock (A20, A21, A22 check them before running) | `qa/sherlock/decisions/` | tasks G-A, 4.4 |
| E12 | Evaluator-owned: parametrise the rejection suite and the U023 holdout by network (`qa/holdout/run.sh:40-41` accepts only `rejections-local|rejections-stagenet`; `qa/holdout/lib/stagenet_check.py:19` hardcodes the stagenet indexer; `produce-a9.sh` is stagenet-only; `qa/holdout/contracts/U023.json` pins `network:'stagenet'`); contract change on `U023.json`; reseal `MANIFEST.sha256` under `DW_ROLE=evaluator` | `qa/holdout/` (evaluator) | contract change + reseal; tasks 6.3/6.5 name the evaluator |
| E14 | `optional: true` row field: `next` FIRST FAILING and `check --tier`/`--set` sweeps skip optional rows unless `--include-optional`; explicit `check <ID>` runs them; `validate` refuses a mandatory row depending on an optional row; regression test: an unimplemented optional B4p or a failing existing optional B6/B7 leaves the Preview gate eligible, an unimplemented mandatory A/U/F row blocks it; applies to every retained B row (B1-B3, B6, B7, B4p, B5p, B8p, B9p) and the S>1 experiment row (since 2026-11-08 (3) the optional rows are F23, A20, A21, A22 and A24-A27, §2); Sherlock fee/security evidence keeps its own mandatory row; when a conditionally guarded feature (S>1, an optimization, a formal claim) is adopted for the qualified candidate its rows drop `optional:` and the triggered clauses become mandatory (astra-2); E1's traceability check accepts a retired row's historical check and requires `source:` on optional rows (fable-3) (astra-3 optional-B note and v2 cleanup; `qa/tiers/t3.sh:64`, `cli.py:50-67`, `status.py:34-47`). Amended 2026-11-08: no B row is optional any more (SWF-013); the regression test keeps its synthetic B4p/B6/B7 fixture ids, which name no real row state | `predicates.py` schema, `cli.py _select`, `status.py:view`, `models/doctorwatson/factory/tests/` | about 30 lines (landed in task 1.0) |
| E15 | Milestone-scoped tier runners (added 2026-11-08 with SWF-013): `t2.sh`/`t3.sh` select a named milestone row set and its dependency closure (for example `--closure C4` for Sherlock M6, the B rows for the synthetic-rollup gates) instead of a whole tier, as SWF-012 already requires; until it lands, the Sherlock gate is read from the C4 closure (§4) | `cli.py _select`, `qa/tiers/t2.sh`, `qa/tiers/t3.sh`, factory unit test | small; task 1.8 |
| E13 | Network-parametrised SDK scripts: a session module built on Midnight.js 5 `providers.config{networkId,ttlSeconds}` (no `setNetworkId`), `--network` on `deploy-run.mjs`, `call-run.mjs`, `readback-run.mjs`, a reject-suite driver, per-network wallet dir and endpoints (`stagenet-session.mjs:24-26,31,41`, `deploy-run.mjs:16,42`, `call-run.mjs:29,65`, `readback-run.mjs:30` are stagenet-only today) | `tools/sherlock-sdk/` | created in task 6.1; M6 commands do not exist until then |

Everything else runs on existing commands.

## 9. What does not count

A `check` whose script is `unimplemented.sh` (exit 2); a `BLOCKED-BY` row; a throughput row whose cap was rewritten from a measurement instead of seeded (SRC-015); a run without a cgroup cap after task 2.0 (`cgroup: false` is recorded, not accepted, for a throughput row); a GPU row without the byte-for-byte comparison against the CPU proof (SRC-018); a lever adopted without its C0 metric in `speed-levers.json` (SRC-011); a receipt only in `target/`; a `stale` entry; a pass on stagenet for a Preview or mainnet claim; SDK WASM `wellFormed` acceptance (`stagenet-deployment.md:184-186`); a systemd `Result=success` when the `heavy` line says exit 4 (`HANDOFF-2026-10-26.md:16`); a reviewer verdict; any `.md` statement without a `runs.jsonl` line behind it.
