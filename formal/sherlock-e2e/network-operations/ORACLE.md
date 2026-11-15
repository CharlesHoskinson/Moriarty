# Network Operations oracle: interface

Quint models must be written from the spec, not from this file. This file only fixes the
wire names, the operation codes and the outcome codes, so that a Quint trace can be replayed
through the oracle.

Context: Network Operations (slice C): `sherlock-e2e-preview-deployment` (PVW),
`-wallet` (WAL), `-sdk-cli` (SCL), `-observability` (OBS), `-recovery` (FRC), with
LPC-008..011, LPC-015, LPC-020 (completion) and SWF-009, ZK-002, ZK-007 (reused).
The model is the Preview run of one ledger call (start, chain check, state-folder check,
wallet checks, deploy, prove, balance, submit, outcome, readback, controls, finish) plus the
validium contract of PVW-011 / LPC-009. The wallet is modelled by flags only.

Binary (odin): `~/sherlock/formal-e2e/network-operations/lean/.lake/build/bin/oracle`.
Run with `LEAN_ABORT_ON_PANIC=1`. `--fields` prints the cell list.

## Cells (state), all required, exactly these keys

| cell | kind | meaning |
|---|---|---|
| phase | int | 0 not started, 1 started (TTL ok), 2 chain ok, 3 state folder ok, 4 wallet modes ok and phrase read, 5 wallet started, 6 deployed and confirmed, 9 stopped |
| dryRun | bool | the `dry-run` phase is selected (SCL-003); fixed by init |
| ttlCfg | int | `T10A_TTL_S` accepted at start (0 before) |
| chain | int | 0 unchecked, 1 devnet `undeployed1`, 2 pinned Preview genesis |
| stop | int | 0 running; 1 chain (PVW-001/002), 2 address (WAL-003), 3 modes (WAL-010), 4 TTL (SCL-007), 5 state-folder key (SCL-004), 7 balancing exhausted (FRC-004) |
| lastRefusal | int | last non-terminal refusal: 0 none, 6 gate record incomplete (PVW-013), 11 submit off the wallet machine (WAL-007), 13 refusals without readback (FRC-006), 14 box not owned (PVW-010, case A), 15 final call not on an owned GPU box (PVW-010) |
| walletRead | bool | the phrase was read (WAL-001) |
| walletStarted | bool | the wallet started (WAL-003) |
| lockHeld | bool | the wallet `flock` is held (WAL-004) |
| deploys | int | deploy transactions sent (FRC-002: at most 1) |
| deployPending | bool | a deploy was sent and not confirmed |
| gate | int | gate bits of the deployed key (RKY-003, RKY-004, RKY-005, T12, ZK-012..017), 0..31 |
| final | bool | the deploy is the final prototype deploy |
| proofs | int | calls proved |
| proofHost | int | host of the last proof: 0 wallet machine, 1 owned GPU box |
| saved | bool | a proven call is saved |
| savedRefused | bool | the saved call was refused by the node (FRC-005) |
| balanced | bool | the saved call is balanced, ready to submit |
| balanceAttempts | int | failed balancing attempts (FRC-004) |
| needResync | bool | a wallet resync is due before the next attempt |
| submitted | bool | the honest call awaits its outcome |
| honestSubmits | int | honest submissions |
| unknown | bool | inclusion state unknown (FRC-013) |
| included | bool | the honest call is included |
| receipts | int | inclusion receipts recorded (OBS-001) |
| errRecords | int | node errors and failed attempts recorded (OBS-002, OBS-008) |
| readback | bool | the honest call is read back |
| refusalsStarted | bool | the `refusals` step started (FRC-006) |
| controls | int | controls submitted (R1, R2, R4, R3 in order), 0..4 |
| controlsRefused | int | controls refused by the node |
| controlFails | int | controls with `included`, `not-included-timeout` or `no-node-verdict` |
| pendingControl | bool | a control awaits its outcome |
| lastVerdict | int | 0 none, 1 refused by the node, 2 `included`, 3 `not-included-timeout`, 4 `no-node-verdict` |
| netConns | int | network connections opened (SCL-003) |
| logLines | int | log lines written (OBS-005, WAL-002) |
| redactions | int | secrets replaced with `<seed>` (WAL-002) |
| finished | bool | the runner ended |
| exitCode | int | SCL-002: 0 passed, 4 failed verdict, 1 error (valid once `finished`) |
| vRoot | int | validium stored root |
| vBatches | int | validium batches accepted |
| vRefused | int | validium batches refused |

## Operations: `op = code * 1000000000 + arg`

| code | operation | arg | enabled when | EARS |
|---|---|---|---|---|
| 1 | start | TTL seconds | phase 0 | SCL-007 |
| 2 | chainCheck | 1 devnet, 2 pinned Preview, other = another chain (Mainnet included) | phase 1, not dry run | PVW-001, PVW-002 |
| 3 | stateCheck | 1 state-folder key equals `--vk`, 0 differs | phase 2 | SCL-004 |
| 4 | walletModes | 1 dir <= 0700 and file <= 0600, 0 wider | phase 3 | WAL-010, WAL-001 |
| 5 | walletAddr | 1 derived address equals the pin, 0 differs | phase 4 | WAL-003 |
| 6 | lock | 1 acquire, 0 release (release only with nothing balanced or in flight) | phase 5 or 6 | WAL-004 |
| 7 | deploy | gate bits (0..31) + 32 * final (0/1); a second deploy while one is pending reads the address back | phase 5, lock held | PVW-009, PVW-013, FRC-002 |
| 8 | deployConfirm | 0 | phase 5, deploy pending | FRC-002 |
| 9 | prove | host: 0 wallet machine, 1 owned GPU box, 2 a box not owned | phase 6, nothing saved, not included, not unknown | PVW-010 |
| 10 | rerun | 2 * TTL-left seconds + hashOk (0/1) | phase 6, not submitted, not unknown, not included | FRC-003, FRC-005, FRC-014 |
| 11 | balance | 1 ok, 0 transport error (retried), 2 other wallet error (stops at once) | phase 6, lock held, saved unrefused call, nothing in flight, no resync due | FRC-004, OBS-008 |
| 12 | submit | who + 10 * host; who 0 honest, 1..4 control number; host 0 wallet machine, 1 other | phase 6, lock held, nothing in flight; honest: balanced and not included; control: refusals started and who = controls + 1 | PVW-006, WAL-007, FRC-013 |
| 13 | outcome | 0 node refusal (JSON-RPC error), 1 included, 2 no block within 600 s, 3 no inclusion, no timeout, no node error | a call in flight | PVW-007, PVW-008, PVW-012, FRC-005, OBS-001, OBS-002 |
| 14 | resolve | 1 node/indexer show inclusion, 0 not included | unknown | FRC-013 |
| 15 | readbackOp | 1 present, 0 absent | included, not yet read back | FRC-006 |
| 16 | startRefusals | 0 | phase 6, not started | FRC-006 |
| 17 | log | 1 the message holds a secret, 0 not | not finished | WAL-002, OBS-005 |
| 18 | finish | 0 | not finished, nothing in flight | SCL-002 |
| 19 | resync | 0 | resync due | FRC-004 |
| 20 | batch (validium) | progOk (0/1) + 2 * inputRoot (0..999) + 2000 * newRoot | not finished | PVW-011, LPC-009 |

A disabled operation leaves the state unchanged in the spec; `--step` refuses it.

## Outcomes and refusals

- Spec responses are state outcomes: `stop` (terminal, `phase` 9), `lastRefusal`,
  `lastVerdict`, counters, `exitCode`. They never appear as `refused`.
- `--step` domain refusals (sorted list): `not-an-operation` (code or arg outside the table),
  `op-not-enabled` (operation not enabled in the state), `outside-state-domain` (cells out of
  range, see `validState`).
- Default mode: `{"protocol": "kink-implementation/1", "inputs": {"dryRun": 0|1, "ops": [...]}}`
  -> `{"end": "final", "cells": {...}}` or `{"end": "could-not-apply", "reason": CODES}`.

## Constants

`ttlMax` 1,209,600 s (SCL-007; 14 days), `reuseTtl` 300 s (FRC-003), `maxAttempts` 4
(FRC-004), `nControls` 4 (PVW-006), `gateAll` 31 (PVW-013, 5 checks).

## LEAD PICKs (spec ambiguities taken by the recommended reading)

1. FAL-NETOPS-1: controls start only after the readback (FRC-006 guards PVW-006's trigger).
2. FAL-NETOPS-2: a call resolved as not included after a timeout stays saved and may be
   reused by a rerun (FRC-013 scenario), although FRC-003 names "unsubmitted".
3. FAL-NETOPS-3: a transport error is retried up to 4 attempts in total (FRC-004); another
   wallet error stops at once (the spec is silent; the runner code does this).
4. SCL-002: a guard stop is an error (exit 1), a failed verdict exits 4.

## Replay

```
python3 ~/Projects/software-factory/plugins/formal/scripts/quint_replay.py \
  --oracle-json '["/home/charl/sherlock/formal-e2e/network-operations/lean/.lake/build/bin/oracle"]' \
  --traces <traces dir> --action-map <quint action>=<op integer> ... \
  --compare <cells the model has> --fill <oracle-only cells>=<default> --map <model>=<oracle>
```

The model's `lastOp` is a string; map each action with `--action-map`, or add an integer
op variable in the projection. Cells the model lacks are filled with their `init` values
(`init false`: every int 0, every bool false).
