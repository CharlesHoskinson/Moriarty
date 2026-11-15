# Placement oracle: interface

Bounded context Placement (slice C), spec `sherlock-e2e-helper-machines` (HLP-001 to HLP-008) plus ZK-001, ZK-009, PRD-002, PRD-003. Lean package `formal/sherlock-e2e/placement/lean` (namespace `Placement`), executable `.lake/build/bin/oracle`, shared codec `formal/sherlock-e2e/codec` (`SherlockCodec.Wire`).

Quint models must be written from the spec, not from this file. This file only fixes the wire names, the operation codes and the outcome codes, so a replay can map the model's variables to the oracle's cells (`--map`, `--fill`, `--drop`).

## Build and run (odin)

```
cd ~/sherlock/formal-e2e/placement/lean && PATH=$HOME/.elan/bin:$PATH lake build --wfail
LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --fields     # cell names and kinds
LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step       # JSON Lines {"state", "op"}
```

Default mode: `{"protocol": "kink-implementation/1", "inputs": {"ops": [op, ...]}}` (no parameters) gives `{"end": "final", "cells": {...}}` for the run from `init`, or `could-not-apply` with `not-an-operation`.

## Cells (all 21 are required in every `--step` state)

| Cell | Kind | Meaning |
|---|---|---|
| `phase` | int | 0 idle, 1 dispatched to an owned box, 2 result received, 3 kept (bound and verified), 4 dropped, 5 submitted |
| `jobCv` | int | call vector id of the requested job |
| `boxOwned` | bool | the job's box is owned by the user (case A) |
| `resCv` | int | call vector id of the received result |
| `shaOk` | bool | the SHA-256 of the received file equals the box report |
| `resWords` | int | public words in the received result (4) |
| `resPrivate` | int | private parts in the received result (0) |
| `verifyRan` | bool | the ledger 9.1.0.1 check ran on this result |
| `verifyOk` | bool | the ledger check accepted the proof |
| `dropCode` | int | 0, 3 binding drop (HLP-008), 4 verifier drop (HLP-002) |
| `kept` | int | results kept so far |
| `submitted` | int | calls submitted so far |
| `lastRefusal` | int | 0, or the refusal code of the last refused request (table below) |
| `signed` | int | signatures made by a wallet holder |
| `signedNoWallet` | int | signatures made by a box with no wallet (stays 0) |
| `slotsFree`, `slotsUsed` | int | blinding slots in the pool, consumed slots |
| `helperSlots` | int | helper-made slots accepted (stays 0) |
| `wrapProofs` | int | wrap proofs made |
| `sentOwned` | int | private artefacts sent to an owned host |
| `sentNotOwned` | int | private artefacts sent to a host the user does not own (stays 0) |

## Operations (`op = code * 1000000000 + arg`)

| code | name | arg | enabled when | IDs |
|---|---|---|---|---|
| 1 | dispatch | `owned + 10 * cv` (owned 0/1) | phase 0 | HLP-005, ZK-001, PRD-003 |
| 2 | receive | `shaOk + 10 * cv` | phase 1 | HLP-003, HLP-004 |
| 3 | check | `verifierOk` 0/1 | phase 2 | HLP-001, HLP-002, HLP-008 |
| 4 | submit | 0 | phase 3 or 4 | HLP-002 |
| 5 | newJob | 0 | phase 3, 4 or 5 | |
| 6 | sign | `hasWallet` 0/1 | always | HLP-007 |
| 7 | makeSlot | `byHelper` 0/1 | always | PRD-002, ZK-009 |
| 8 | wrapProve | 0 | always | PRD-002 |
| 9 | exportSlot | 0 | always | PRD-002 |
| 10 | sendOffBox | `owned + 10 * kind`, kind 1 witness, 2 STARK proof, 3 checkpoint, 4 shard plan | always | HLP-005, PRD-003, ZK-001 |

Spec responses are state outcomes, never `--step` refusals:

| `lastRefusal` | meaning |
|---|---|
| 1 | host not owned (HLP-005, ZK-001, ZK-018, PRD-003) |
| 5 | submit of a dropped proof (HLP-002) |
| 6 | sign on a box with no wallet directory (HLP-007) |
| 7 | helper-made slot (PRD-002) |
| 8 | no free slot for wrap_prove (PRD-002) |
| 9 | slot export (PRD-002) |

`--step` refusals (domain only): `not-an-operation` (unknown code or arg), `op-not-enabled` (operation not enabled in the phase), `outside-state-domain` (`validState` fails).

## LEAD PICKs

- LP-1: both drops (binding, verifier) end the job; `submit` in phase 4 records `lastRefusal = 5`.
- LP-2: a private artefact sent to an owned host is allowed (case A, OD-9) and counted in `sentOwned`.
- The box result is modelled as 4 public words with no private part by construction (HLP-003, HLP-004 bind the box, not muninn).

## Replay

```
python3 ~/Projects/software-factory/plugins/formal/scripts/quint_replay.py \
  --oracle-json '["/home/charl/sherlock/formal-e2e/placement/lean/.lake/build/bin/oracle"]' \
  --traces <traces dir> --op-var lastOp --compare phase,dropCode,kept,submitted,lastRefusal ...
```

The model's `lastOp` must be mapped to the integer code above (`--action-map` for MBT traces, or a projection adapter); fill the oracle cells the model does not have with `--fill`.
