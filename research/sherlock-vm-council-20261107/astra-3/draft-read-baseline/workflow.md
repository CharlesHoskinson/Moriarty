# DoctorWatson workflow: implementation and deployment

Status: DRAFT v1. Adopted from fable-3's `workflow-proposed.md` with the council's ISSUE resolutions. Every command exists at `8fd34ed79` unless listed in §8 "Exact additions" (verified from `scripts/doctorwatson --help`, `models/doctorwatson/factory/CONTRACT.md`, `qa/tiers/*.sh`, `.githooks/pre-commit`). Requirements SWF-001..011.

## 1. Engine and truth

- Engine: `scripts/doctorwatson` (`python3 -m factory`, `models/doctorwatson/factory/`). Loop: `AGENTS.md:13-20`.
- Truth: `qa/sherlock/predicates.yaml` (what must hold), `qa/sherlock/status.json` and `runs.jsonl` (what was observed), `qa/sherlock/budgets.json` (measured caps), git (observed source). Nothing else is state (`CONTRACT.md:116-119`).
- Not used: `scripts/doctorwatson.sh --inspect/--execute/--cleanup` and `models/doctorwatson/{delivery,run_selected,reconciliation,cleanup}.py` (superseded 2026-10-24, `docs/DOCTORWATSON*.md:3`).
- Reviews are inputs; the only reviewer wait is the bounded T3 cold review (`AGENTS.md:22-25`; `scripts/qa/check-f11.sh`). The AgentBus is a doorbell, never state (`CONTRACT.md:116-135`).

## 2. Requirement -> predicate -> check -> evidence -> gate

Every ID in `requirements.md` maps to at least one row in `predicates.yaml` with `id, set, title, tier, check, heavy, budget?, depends, source, holdout, network?, retired?`. `source` is `openspec/changes/sherlock-vm-consensus-20261107/requirements.md#<id>` or a `specs/<cap>/spec.md#requirement-...` anchor.

1. No executable check yet: `check: "scripts/qa/unimplemented.sh <ID>"` (exit 2), visible as failing in `next`. No row is omitted to improve counts.
2. Behavioural requirement: a visible regression test (`check-uNNN.sh` pattern, `scripts/qa/check-u021.sh:5-19`) and an evaluator-owned holdout under `qa/holdout/cases/`; the holdout is the arbiter. Interface frozen first by `qa/holdout/contracts/<ID>.json` and `scripts/doctorwatson contract-check <ID>`.
3. Heavy check: names a `budget` key; first run measures, later runs are capped at 2x (`CONTRACT.md:78`); same `cmd` and `head` retry after failure needs `--hypothesis` (`CONTRACT.md:96`).
4. Milestone exit: a C-style composite (`scripts/qa/check-c.sh`, `depends` = member rows).
5. `traceability.md` is regenerated from `predicates.yaml` + `status.json` by `scripts/qa/check-traceability.py --render`; it is never hand-maintained after M1.

Sets: new rows go into the existing sets. A-set: recursion chain rows (`A2p`, `A3p`, `A4p`, `A6p`, `A8p`, `A9p`, `A10p` with `network: preview`; `A11` keygen regression; `A12` leaf shape; `A13` chain negatives). U-set: `U023a..U023f` fragment and dispatch-partition receipts, `U023` unchanged. B-set: `B4p`, `B8p`, `B9p` preview rows, non-gating for C4. F-set: `F14` traceability, `F15` target pins, `F16` retired-row test. The headline counts therefore stay comparable (fable-3; no new set letter unless the owner wants one, §8 E4).

## 3. Operations (mutating steps)

A predicate observes; it never deploys, calls or spends. A mutating step is an operation (SWF-003), listed in `tasks.md` §"Operations" with:

| Field | Meaning |
|---|---|
| `op` | id, e.g. `OP-PV-DEPLOY` |
| `preconditions` | predicate ids passing in `status.json` at the current head |
| `command` | `scripts/doctorwatson heavy --id <op> --budget <key> [--hypothesis ...] -- <cmd>` |
| `postcondition` | the read-only predicate that becomes checkable afterwards |
| `authorization` | `none` (local), `owner` (public network), `owner+checklist` (mainnet, T4) |
| `receipt` | committed path `qa/sherlock/receipt-traces/<op>/receipt.json` (tx hash, block, address, key lineage, state SHA-256, pins row, chain identity) |
| `rollback` | §7 |

Pattern in use: `doctorwatson heavy --id A4deploy -- node tools/sherlock-sdk/deploy-current.mjs`; `heavy --id A8 -- node tools/sherlock-sdk/call-current.mjs --submit` (`docs/sherlock/stagenet-deployment.md:190-191,300`). A postcondition pass does not count until the receipt is committed in the same commit as `scripts/doctorwatson state-commit` (SWF-005).

## 4. Tiers and gates

| Tier | Runner | Content | When |
|---|---|---|---|
| T0 | `qa/tiers/t0.sh` (pre-commit hook) | holdout verify, `validate`, factory unit tests, profile isolation, `cargo check` of touched crates, fmt; new: F14 traceability, F15 static pins | every commit |
| T1 | `qa/tiers/t1.sh` | `soundness_mutation` (after the SAS-003 fix), workspace tests, `check --tier T1` | before merge to `main` |
| T2 | `qa/tiers/t2.sh` | `preflight`, `check --tier T2` (heavy proofs under budgets) | per milestone with native work |
| T3 | `qa/tiers/t3.sh [--review]` | checklist, optional cold review, `check --tier T3` (read-only network predicates); new: live target identity step (E3) | per public-network milestone |
| T4 | new `qa/tiers/t4.sh` | mainnet checklist, owner authorization, rollback rehearsal receipt, `check --tier T4` | mainnet only |

Gate rule: a milestone is complete when `scripts/sherlock-next` names no failing non-retired predicate among the milestone's rows at the current head and `scripts/doctorwatson stop-rule` exits 0. A `BLOCKED-BY` row is failing.

## 5. Daily loop

1. `scripts/sherlock-next`; work the first failing predicate (retired rows are listed, not selected).
2. Heavy work only via `scripts/doctorwatson heavy --id <ID> --budget <key> -- <cmd>`; queue with `scripts/doctorwatson q submit` when the lock is busy; never plain `cargo test` for proofs.
3. On failure read `qa/sherlock/traces/<run>.log`; retry with `--hypothesis`; after two same-class failures shrink the reproducer (`AGENTS.md:17-18`).
4. `scripts/doctorwatson check <ID>`; commit code or tests (hook runs T0; docs-only commits need `DW_DOCS_COMMIT=1`); `scripts/doctorwatson state-commit`.
5. `scripts/doctorwatson stop-rule`; on exit 6 stop and bring the owner the trace and hypothesis (SWF-006).
6. Helpers via `scripts/doctorwatson spawn --vendor ... --as ... --role ... --brief ...` (sandbox `read-only` unless implementer/lead).

## 6. Deployment path

Order: `undeployed` (local docker at 9.1.0.1: node :9944, indexer :8088, proof server :6300, pre-funded genesis wallet) -> Preview (ledger 9.1.0.1, specVersion 2001100, new chain since 2026-11-07, faucet) -> Mainnet (Blockfrost project, real NIGHT/DUST, no faucet). Sources: https://docs.midnight.network/guides/networks-and-environments, `wiki/midnight-ledger-9-hardfork.md`. Stagenet (`shielded.tools`, specVersion 2000000) is retired for new qualification; its rows carry `retired:` and remain inspectable (SWF-011); a stagenet call is an optional diagnostic only.

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
| Fees | B8-shape vs `min_block_price` and factors | `B8p` | `B8m` |

Pre-deploy checklist from ledger 9: `validatePassword` before any transaction (`HANDOFF-2026-10-26.md:46`); at least two DUST spends (#838); maintenance authority declared or contract marked disposable (SMN-007); `TooManyPublicInputs` and 256-op cap checked (SMN-006); wallet seed backed up encrypted after every deployment.

## 7. Rollback

Chain state is not reversible; settle cases consume statements (`stagenet-deployment.md:228-231`). Rollback means (SWF-010):
1. A failed or orphaned deployment is recorded `orphaned` with address and block; the fixture is re-pinned to the last good address; nothing is deleted (`runs.jsonl` append-only).
2. A maintained contract may receive a replacement key by maintenance update (`IrInsert`/`IrRemove`, ledger 9.1.0.0 notes); midnight-node #2375 (`VerifierKeyRemove` fails with `BalanceCheckOverspend` on 9.1.0.0 clients) is checked first. Inserting a key does not revoke an old accepted key; an unsafe old key means redeploy and retire the address.
3. A contract without maintenance authority is redeployed at a new address; the old one is listed `retired`.
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
| E7 | Stale flag: a passing entry whose run `head` != HEAD, or whose pins/key-manifest hash differ from the current files, is reported `stale` by `next` | `status.py:view`, `cli.py:cmd_next` | small |
| E8 | Receipt-first readers; candidate by manifest, not mtime; any-call acceptance removed | `scripts/qa/check-a8.sh`, `check-a10.sh`, new `check-*p.sh` | script edits |
| E9 | Amend `openspec/MORIARTY-COMPLETION-PROGRAM.md:269` or keep mainnet blocked | docs | owner decision |
| E10 | `retired: "<reason or anchor>"` row field; `next` skips retired rows for FIRST FAILING and prints `RETIRED: ...`; factory unit test | `predicates.py` schema, `status.py:34-47`, `cli.py`, `models/doctorwatson/factory/tests/` | about 20 lines |
| E11 | Heavy concurrency (two proofs when the sum fits the cap) requires a second lock class; out of scope unless the owner asks | `heavy.py` | named, not planned |

Everything else runs on existing commands.

## 9. What does not count

A `check` whose script is `unimplemented.sh` (exit 2); a `BLOCKED-BY` row; a receipt only in `target/`; a `stale` entry; a pass on stagenet for a Preview or mainnet claim; SDK WASM `wellFormed` acceptance (`stagenet-deployment.md:184-186`); a systemd `Result=success` when the `heavy` line says exit 4 (`HANDOFF-2026-10-26.md:16`); a reviewer verdict; any `.md` statement without a `runs.jsonl` line behind it.
