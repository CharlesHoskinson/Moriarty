# Proposed `workflow.md` for `openspec/changes/sherlock-vm-consensus-20261107/`

Author: fable-3. Proposed text for the chair; only the chair edits the change directory. Every command below exists today unless it is listed under "Exact additions". Paths are relative to the checkout root; verified at `8fd34ed79` on 2026-11-07 (`scripts/doctorwatson --help`, each subcommand `--help`, `models/doctorwatson/factory/CONTRACT.md`, `qa/tiers/*.sh`, `.githooks/pre-commit`).

## 1. Engine and truth

- Engine: `scripts/doctorwatson` (`python3 -m factory`, `models/doctorwatson/factory/`). The loop is `AGENTS.md:13-20`.
- Truth: `qa/sherlock/predicates.yaml` (what must hold), `qa/sherlock/status.json` and `qa/sherlock/runs.jsonl` (what was observed), `qa/sherlock/budgets.json` (measured caps), git (what source was observed). Nothing else is state (`CONTRACT.md:116-119`).
- Not used: `scripts/doctorwatson.sh --inspect/--execute/--cleanup` and the plan/attempt/reconciliation files under `models/doctorwatson/*.py`. They are history (`docs/DOCTORWATSON*.md:3`).
- Reviews are inputs. The only wait on a reviewer is the bounded T3 cold review (`AGENTS.md:22-25`; `scripts/qa/check-f11.sh`).

## 2. Requirement → predicate → check → evidence → gate

Every requirement ID in `requirements.md` maps to at least one row in `qa/sherlock/predicates.yaml`. A row has `id`, `set`, `title`, `tier`, `check`, `heavy`, `budget` (if heavy), `depends`, `source`, `holdout` (`CONTRACT.md:33-53`). `source` is the anchor of the requirement in this change (`openspec/changes/sherlock-vm-consensus-20261107/requirements.md#<id>` or `specs/<cap>/spec.md#requirement-…`).

Rules:
1. A requirement with no executable check yet gets `check: "scripts/qa/unimplemented.sh <ID>"` (exit 2). It is visible in `next` as failing. No row is ever omitted to improve the counts.
2. A behavioural requirement has two checks: a visible regression test (the `check-uNNN.sh` pattern: `cargo test` or `python3 -m unittest` filters that must match at least one test, `scripts/qa/check-u021.sh:5-19`) and an evaluator-owned holdout scenario under `qa/holdout/cases/` (`qa/holdout/README.md`). The holdout is the arbiter.
3. Before a holdout is authored, the implementer commits `qa/holdout/contracts/<ID>.json` and `scripts/doctorwatson contract-check <ID>` exits 0 (`qa/holdout/contracts/README.md`).
4. A heavy check (`heavy: true`) names a `budget` key. The first run measures; later runs are capped at 2× (`CONTRACT.md:78`). A failed heavy run at the same `cmd` and `head` needs `--hypothesis` (`CONTRACT.md:96`).
5. A composite requirement (a milestone exit) is a C-style row whose `check` is `scripts/qa/check-c.sh` and whose `depends` lists its member rows (`predicates.yaml:495-548`).
6. The traceability table in `traceability.md` is generated from `predicates.yaml` plus `status.json`, never hand-maintained. Its columns: requirement → design section → predicate → check → last run (`runs.jsonl` line) → tier/gate → evidence class.

## 3. Operations (mutating steps)

A predicate observes; it never deploys, calls or spends. A mutating step is an **operation**. Operations are listed in `tasks.md` in this form and run only by hand or by the owner-authorized driver:

| Field | Meaning |
|---|---|
| `op` | id, e.g. `OP-PV-DEPLOY` |
| `preconditions` | predicate ids that must pass in `status.json` at the current head |
| `command` | `scripts/doctorwatson heavy --id <op> --budget <key> [--hypothesis …] -- <cmd>` (the `heavy` wrapper records trace, wall, RSS, head) |
| `postcondition` | the read-only predicate that becomes checkable afterwards (e.g. `A8-preview`) |
| `authorization` | `none` (local), `owner` (public network), `owner+checklist` (mainnet) |
| `receipt` | committed path under `qa/sherlock/receipt-traces/<op>/` (tx hash, block, address, key lineage, state SHA-256, pins) |
| `rollback` | what is done if the postcondition fails (see §7) |

Pattern already in use: `doctorwatson heavy --id A4deploy -- node tools/sherlock-sdk/deploy-current.mjs` and `heavy --id A8 -- node tools/sherlock-sdk/call-current.mjs --submit` (`docs/sherlock/stagenet-deployment.md:190-191,300`).

A postcondition pass does not count until the receipt is committed in the same commit as the `state-commit` (`scripts/doctorwatson state-commit`). Receipts in `target/` were lost with the previous host (`docs/sherlock/HANDOFF-2026-10-26.md:7`).

## 4. Tiers and gates

| Tier | Runner | Content | When |
|---|---|---|---|
| T0 | `qa/tiers/t0.sh` (also the pre-commit hook) | holdout verify, `validate`, factory unit tests, profile isolation, `cargo check` of touched crates, fmt; **new:** traceability check, static target-pins check | every commit |
| T1 | `qa/tiers/t1.sh` | `soundness_mutation`, workspace tests, `check --tier T1` | before a PR/merge to `main` |
| T2 | `qa/tiers/t2.sh` | `preflight`, `check --tier T2` (heavy proofs under budgets) | per milestone with native work |
| T3 | `qa/tiers/t3.sh [--review]` | checklist, optional background cold review, `check --tier T3` (read-only network predicates); **new:** live target identity step | per public-network milestone |
| T4 | **new** `qa/tiers/t4.sh` | mainnet checklist with owner authorization, `check --tier T4` | mainnet only |

Gate rule: a milestone is complete when `scripts/sherlock-next` names no failing predicate of that milestone's rows at the current head and `scripts/doctorwatson stop-rule` exits 0. A `BLOCKED-BY` row is failing.

## 5. Daily loop (agents)

1. `scripts/sherlock-next`; work the first failing predicate.
2. Heavy work only via `scripts/doctorwatson heavy --id <ID> --budget <key> -- <cmd>`; queue with `scripts/doctorwatson q submit` when the lock is busy; never plain `cargo test` for proofs.
3. On failure read `qa/sherlock/traces/<run>.log`; retry with `--hypothesis`; after two same-class failures, shrink the reproducer (`AGENTS.md:17-18`).
4. `scripts/doctorwatson check <ID>`; commit code or tests (hook runs T0; docs-only commits are rejected unless `DW_DOCS_COMMIT=1`); `scripts/doctorwatson state-commit`.
5. `scripts/doctorwatson stop-rule`; on exit 6 stop and bring the owner the trace and hypothesis. Do not re-run unrelated passing predicates to reset the counter.
6. Helpers: `scripts/doctorwatson spawn --vendor … --as … --role … --brief …` (sandbox `read-only` unless implementer/lead). Bus events are data (`CONTRACT.md:124-126`).

## 6. Deployment path

Order: `undeployed` (local docker: node :9944, indexer :8088, proof server :6300) → Preview (ledger 9.1.0.1, `specVersion 2001100`, new chain since 2026-11-07, faucet) → Mainnet (Blockfrost project, real NIGHT/DUST, no faucet). Sources: `https://docs.midnight.network/guides/networks-and-environments.md`, `wiki/midnight-ledger-9-hardfork.md`. Stagenet (`shielded.tools`, `specVersion 2000000`) is an optional diagnostic only; its evidence is class 4 on a non-production runtime (RPC read 2026-11-07).

Per network, the same predicate shapes are re-earned with a network suffix and a fixture `crates/mori-recursion/tests/fixtures/<network>-verifier.json` (or the run-decider manifest, fable-2 I7):

| Step | Local (`undeployed`) | Preview | Mainnet |
|---|---|---|---|
| Pins | `qa/sherlock/target-pins.json` row | same, plus chain genesis hash | same, plus Blockfrost endpoints |
| Keys | keygen regression vs manifest | same | same, frozen |
| Local acceptance/refusal | A6-shape predicate (`validate_call`, proof-verifying on) | required before deploy | required before deploy |
| Deploy | op, `authorization: none` | op, `owner` | op, `owner+checklist` (T4) |
| Call | op | op | op |
| Inclusion | A8-shape (indexer `ContractCall` bound to `stmt`) | A8-preview | A8-mainnet |
| Rejections | A7/A9-shape holdout | `rejections-preview` | `rejections-mainnet` (only non-consuming cases) |
| Readback | A10-shape (submitter-independent, manifest-selected, not mtime) | A10-preview | A10-mainnet |
| Fees | B8-shape measured against `min_block_price` and the three factors | required | required |

Pre-deploy checklist items that come from ledger 9: wallet password passes `validatePassword` before any transaction (`HANDOFF-2026-10-26.md:302-307`); pay with at least two DUST spends (ledger issue #838); contract has a maintenance authority or is declared disposable; `TooManyPublicInputs` checked for the decider statement; 256-operation cap checked.

## 7. Rollback

Chain state is not reversible; the A9 settle cases consume statements (`docs/sherlock/stagenet-deployment.md:228-231`). Rollback therefore means:
1. A failed or orphaned deployment is recorded in its receipt as `orphaned` with address and block; the fixture is re-pinned to the last good address; nothing is deleted (`runs.jsonl` is append-only, `CONTRACT.md:23`).
2. A contract with a maintenance authority may receive a replacement key via maintenance update (`IrInsert`/`IrRemove`, ledger 9.1.0.0 notes); known defect midnight-node #2375 (`VerifierKeyRemove` fails with `BalanceCheckOverspend` on 9.1.0.0 clients) is checked before relying on removal. Inserting a new key does not revoke an old accepted key; if the old key is unsafe, redeploy and record the old address as retired.
3. A contract without a maintenance authority is redeployed at a new address; the old one stays callable and is listed as retired.
4. Git side: `git revert` of the fixture/receipt commit, then `scripts/doctorwatson check <affected ids>` and `state-commit`.
5. Mainnet: rollback is rehearsed on Preview first (deploy → bad key → maintenance replace → readback) and the rehearsal receipt is committed before the T4 checklist can pass.

## 8. Exact additions (not present today)

| # | Addition | Where | Size |
|---|---|---|---|
| E1 | Traceability check: every requirement ID in this change's `requirements.md` has ≥1 predicate; every `source:` anchor resolves; every row has a visible check or holdout | `scripts/qa/check-traceability.py` + `predicates.yaml` row `F14` (T0) + `qa/tiers/t0.sh` `dw_step` | one Python file (stdlib + PyYAML), two YAML lines, one shell line |
| E2 | Target pins: per-network row with ledger crate rev, node, Compact toolchain, runtime, midnight-js, wallet-sdk, proof-server image, indexer, genesis hash; static compare with `Cargo.toml`/SDK lockfile | `qa/sherlock/target-pins.json` + `scripts/qa/check-target-pins.py` + row `F15` (T0) | one JSON, one Python file |
| E3 | Live target identity: `midnight_ledgerVersion`, `state_getRuntimeVersion.specVersion`, `chain_getBlockHash(0)` of the pinned RPC equal the pins row; exit 3 if unreachable | `qa/tiers/t3.sh` `dw_step` calling `check-target-pins.py --live <network>` | one shell line |
| E4 | New set letter if the chair wants a sixth headline count | `models/doctorwatson/factory/predicates.py:11` (`SETS`) | one line; counts follow automatically (`status.py:36`, `cli.py:38`) |
| E5 | Tier T4 and runner | `predicates.py:12` (`TIERS`), `cli.py:196` (`--tier` choices), `qa/tiers/t4.sh` | two lines plus one shell script modelled on `t3.sh` |
| E6 | Optional `network:` field in the predicate schema (validated against the pins file) | `predicates.py` schema (`:95-160`) | small |
| E7 | Stale flag: a passing entry whose run `head` ≠ HEAD is reported `stale` by `next` (passes unchanged) | `status.py:view`, `cli.py:cmd_next` | small; no schema change |
| E8 | Receipt predicates read committed receipts first (`qa/sherlock/receipt-traces/<op>/receipt.json`), indexer second; candidate selected by manifest, not mtime | `scripts/qa/check-a8.sh`, `check-a10.sh`, new `check-*-preview.sh` | script edits |
| E9 | Amend `openspec/MORIARTY-COMPLETION-PROGRAM.md:269` ("make no mainnet submission") or declare mainnet out of scope for this change | docs | owner decision |
| E10 | Legacy-row retirement (astra-3's point): optional `retired: "<anchor or reason>"` field on a predicate row. `validate` accepts it; `status.view` excludes retired rows from `firstFailing` and reports them as a separate `retired` count; `next` prints `RETIRED: A8 (stagenet, see …)` lines after the first failing line; results stay in `status.json`/`runs.jsonl`. Acceptance test in `models/doctorwatson/factory/tests/`: with a failing retired row A8 and a new unimplemented row A8-preview, `next` names A8-preview and still lists A8 as retired. No fake `depends` edge, no row deletion, history inspectable. Extension (astra-3): `check --tier T`/`--set S` sweeps skip retired rows (so `qa/tiers/t3.sh`'s `check --tier T3` does not fail on them); an explicitly named `check A8` still runs; `validate` rejects an active row whose `depends` names a retired row; composites are rewired to the preview rows. Acceptance covers `next`, `check --tier T3`, `check --set A`, explicit `check A8` | `predicates.py` schema (`:95-160`) and `_select` in `cli.py:50`, `status.py:view`, `cli.py:cmd_next`, unit tests | about 40 lines |

| E11 | Heavy concurrency (second lock class) | `heavy.py` | named, not planned |
| E12 | Evaluator-owned: `qa/holdout/run.sh` gains `rejections-preview` (today only `rejections-local|rejections-stagenet`, `run.sh:40-41`); `qa/holdout/lib/stagenet_check.py:19` indexer URL and `produce-a9.sh` parametrised by network; `qa/holdout/contracts/U023.json` pins `network` per target (CONTRACT CHANGE); `MANIFEST.sha256` resealed with `DW_ROLE=evaluator` | `qa/holdout/**` | evaluator task |
| E13 | SDK scripts parametrised by network: a session module built from `tools/sherlock-sdk/stagenet-session.mjs` using Midnight.js 5 `providers.config{networkId,ttlSeconds}` (no `setNetworkId`, removed in 5.0.0); `--network` on `deploy-run.mjs`, `call-run.mjs`, `readback-run.mjs` (today hard-coded `stagenet`: `deploy-run.mjs:42`, `call-run.mjs:65,128`, `readback-run.mjs:30,106`); a reject-suite driver; per-network wallet directory | `tools/sherlock-sdk/` | implementer task |

Everything else in this workflow runs on existing commands.

## 9. What does not count

A `check` whose script is `unimplemented.sh` (exit 2); a `BLOCKED-BY` row; a receipt only in `target/`; a `status.json` entry whose run `head` is not the current head (stale); a pass on stagenet for a mainnet claim; SDK WASM `wellFormed` acceptance (`stagenet-deployment.md:184-186`); a systemd `Result=success` when the `heavy` line says exit 4 (`HANDOFF-2026-10-26.md:16`); a reviewer verdict; any `.md` statement without a `runs.jsonl` line behind it.
