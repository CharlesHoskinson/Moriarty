# Oracle interface: Sherlock Execution (slice A)

Lean package `formal/sherlock-e2e/sherlock-execution/lean` (namespace `SherlockExec`, specs `sherlock-e2e-universal-interpreter` EUI, `sherlock-e2e-rsl-kits` EKT, `sherlock-e2e-rsl-statement` ERS). Built on odin with `lake build --wfail` (0 sorry, 81 theorems with pinned axioms, `#audit_ns SherlockExec` = `[]`). Binary on odin: `/home/charl/sherlock/formal-e2e/sherlock-execution/lean/.lake/build/bin/oracle`. Codec: `formal/sherlock-e2e/codec` (`SherlockCodec.Wire`).

Quint models must be written from the spec, not from this file. This file only fixes the wire names, the op codes and the abstract digest formulas a replay needs.

## Modes

- default: `{"protocol":"kink-implementation/1","inputs":{"ukey":INT,"ops":[OP,...]}}` gives `{"end":"final","cells":{...}}`, or `{"end":"could-not-apply","reason":"not-an-operation"}` when an op does not decode, or `outside-state-domain` for `ukey < 0`.
- `--step`: `{"state":{all cells},"op":OP}` gives `{"state":{...}}`, `{"refused":[CODE]}` or `{"error":TEXT}`.
- `--fields`: the cell list with kinds.

Step-domain refusal codes (never a spec response): `not-an-operation`, `op-not-enabled` (a disabled op leaves the state unchanged), `outside-state-domain`.

## Cells (35)

| cell | kind | meaning |
|---|---|---|
| phase | int | 0 idle, 1 loaded, 2 admitted, 3 tape checked, 4 executed, 5 statement built, 6 app proof accepted, 7 stopped, 8 closed with no statement (R_SL rejected), 9 in recursion |
| stop | int | 0 none, 1 malformed record (ERS-014), 2 mixed hash (EKT-004), 3 bad tape (EUI-007), 4 machine version names no family (EUI-001, LEAD PICK) |
| lastRefusal | int | 0 none, 1 kit relation violated (EKT-007), 2 offered commitment is not the statement's (ERS-012), 3 private record to a box the user does not own (ERS-002) |
| refusedField | int | index 1..14 of the missing or wrong-type field (ERS-014) |
| family | int | 0 none, 1 interpreted, 2 R_SL (EUI-001) |
| machineVersion, image, plan, keyRoot | int | admission inputs |
| progD | int | `prog_d` |
| hasAppKey, appKeyCommitted | bool | per-program app key; committed under key_root (ERS-015) |
| usesSha, usesKeccak | bool | the program calls persistentHash / keccak256 |
| tapeChecked | bool | the tape passed the wrapper check |
| tapeD | int | tape value digest |
| outcome | int | 0 none, 1 Halted, 2 Rejected, 3 Exhausted |
| aluOverflow | bool | an exact ALU result at or above its modulus (EKT-011) |
| nW | int | number of effect words |
| microOps | int | executed micro-op count |
| comm, wD | int | `comm` and the W digest |
| publicRows | int | PUBLIC rows (ERS-003) |
| box | int | executing box |
| stmtCount | int | 0 or 1 `RslStatement` |
| outcomeKind, outcomePayload | int | statement outcome kind and payload |
| inputVal | int | `val_d` bound inside `input_d` |
| proofsStarted | int | app proofs started |
| kitOk, proofAccepted | bool | last proof's kit check; app proof accepted |
| boundOutcome | int | commitment recursion consumed |
| publicsOut | int | StatementPublics sent |
| privateLeaks | int | private records that left owned boxes (always 0) |
| ukey | int | universal key set id (never changes) |

## Operations (`op = code * 1000000000 + arg`)

| code | op | arg | enabled when | IDs |
|---|---|---|---|---|
| 1 | load | `idx + 100*kind`; kind 0 well formed (idx 0), 1 field idx missing, 2 field idx wrong type (idx 1..14) | phase 0 | ERS-001, ERS-013, ERS-014 |
| 2 | admit | `image + 1000*plan + 10^4*keyRoot + 10^5*mv + 10^6*appKey + 10^7*sha + 10^8*keccak` (image < 1000, digits < 10, flags 0/1) | phase 1 | EUI-001, ERS-006, ERS-015, EKT-004 |
| 3 | tape | `flags + 16*tapeD`; flag bits 1 wrong selector, 2 non-canonical word, 4 missing word, 8 leftover word | phase 2 | EUI-007, MYC-REF-036/038, MYC-TAPE-006/007 |
| 4 | execute | `kind + 10*alu + 100*nW + 10^6*box` (kind 1 Halted, 2 Rejected, 3 Exhausted; nW < 10^4) | phase 3; kind 3 only for family 1; alu 1 only for family 2 | EUI-004..008, ERS-003, ERS-005, EKT-011 |
| 5 | build | 0 | phase 4 | ERS-007..010 |
| 6 | prove | 1 kit relations hold, 0 violated | phase 5 | EKT-007 |
| 7 | recurse | offered outcome commitment | phase 6 | ERS-012 |
| 8 | send | `10*pub + owned` (pub 1 StatementPublics, 0 call record/RslStatement; owned 1 the user owns the target) | phase >= 1; pub 1 needs stmtCount 1 | ERS-002 |
| 9 | newCall | 0 | phase 7, 8 or 9 | (reset; keeps ukey, privateLeaks) |

Responses: stops go to phase 7 with `stop`; `prove 0`, a wrong `recurse` and a private `send` to a non-owned box set `lastRefusal` and keep the phase. A rejected R_SL call goes to phase 8 at `build` with no statement (ERS-009); an interpreted Rejected or Exhausted run gets a statement.

## Abstract digests (compare them, or leave them out with `--drop`/`--compare`)

- `progD = image + 1000*plan + 10000*keyRoot + 100000*mv` (ERS-006)
- `comm = 2*image + 3*tapeD + 1`, `wD = 5*image + 7*tapeD + 2` (the native reference, ERS-005)
- `outcomePayload = 1000003*comm + wD` for Halted, else the outcome code (ERS-008)
- `microOps = image + 10` (EUI-008)
- `publicRows = 1 + nW` for Halted (ERS-003)

## LEAD PICKs

- EUI-001: a machine version other than 1 (interpreted) or 2 (R_SL) is refused at admission (stop 4). The spec says each program runs under exactly one named family and gives no response for none.
- EKT-004 applies only to the R_SL family (the sentence names the R_SL compiler); an interpreted program with both hashes is admitted.
- ERS-009 applies to every R_SL Rejected call, as the spec text states. The T23-route limitation in its status is recorded as finding FAL-EXEC-4.

## Replay

`python3 ~/Projects/software-factory/plugins/formal/scripts/quint_replay.py --oracle-json '["/home/charl/sherlock/formal-e2e/sherlock-execution/lean/.lake/build/bin/oracle"]' --traces <dir> --op-var lastOp --compare <cells> [--map model=cell] [--fill cell=default] [--drop modelOnlyVar]`. The model's `lastOp` must carry the integer op above, or use `--action-map name=integer` for an op with a fixed argument.
