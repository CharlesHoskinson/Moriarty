## ADDED Requirements

### Requirement: PDM-001 The daemon keeps the wrap state resident
PDM-001: While `wrapd serve` runs, the prover daemon SHALL keep the SRS, the relation, the proving key, the verifying key, the fixed-base tables and the CUDA context loaded.

Verification: the warm against cold record: huginn 113.01 s one process per call, 71.59 s with the daemon (`~/Projects/Moriarty/llm-wiki/wiki/log.md:3691`); rerun on the T25 keys (LPC-005). Exists.

#### Scenario: Two calls in a row
- **GIVEN** a daemon that served one call
- **WHEN** a second call arrives
- **THEN** the daemon loads no key and no SRS for the second call

#### Scenario: Status of PDM-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED on huginn on the T10m keys. muninn timed sets queued (Moriarty `t14b` 5e54263da `research/sherlock-t14b-20261114/RESULTS.md:7`). Evidence: Moriarty `t14b` 5e54263da `tools/sherlock-wrap-gpu/src/bin/wrapd.rs:16` and `:161` (`wrapd serve KEYFIX K CONFIG SOCKET`), `tools/sherlock-wrap-gpu/src/serve.rs:207`; Moriarty `t14b` 5e54263da `research/sherlock-t14b-20261114/RESULTS.md:15`.

### Requirement: PDM-002 The socket is private to the user
PDM-002: When the prover daemon starts, the prover daemon SHALL create its Unix socket with mode 0600 under `$XDG_RUNTIME_DIR`.

Verification: NEW: `stat -c %a` of the socket in the daemon start script; a connect attempt from another user fails. To build.

#### Scenario: Another user
- **GIVEN** a second local user
- **WHEN** that user connects to the socket
- **THEN** the connection is refused by the file mode

#### Scenario: Status of PDM-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. The named check is a gap. Owner lane: t14b. Evidence: Moriarty `t14b` 5e54263da `tools/sherlock-wrap-gpu/src/serve.rs:210`; `daemon.sh:14` and `:20` (`$XDG_RUNTIME_DIR/t14b-*.sock`). PRD-001.

### Requirement: PDM-003 The daemon serves four operations
PDM-003: If a request names an operation other than `prove`, `call`, `stats` or `quit`, then the prover daemon SHALL answer with an error and keep serving.

Verification: NEW: a socket test with the operation `dump`. To build.

#### Scenario: An unknown operation
- **GIVEN** a running daemon
- **WHEN** a client sends `dump`
- **THEN** the client receives an error and the next `stats` request succeeds

#### Scenario: Status of PDM-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (the four operations). The negative test is a gap. Owner lane: t14b. Evidence: Moriarty `t14b` 5e54263da `tools/sherlock-wrap-gpu/src/serve.rs:137` to `:200`.

### Requirement: PDM-004 The STARK worker keeps the STARK keys on the device
PDM-004: While the STARK worker runs in serve mode, the STARK worker SHALL keep the STARK keys, the `L` key and the `L_Fr` key loaded on the device.

Verification: NEW: a warm against cold record of the STARK worker alone in serve mode, on the T25 keys. To build.

#### Scenario: A second run
- **GIVEN** a worker that proved one run
- **WHEN** a second run arrives
- **THEN** the worker generates no key and reads no key file

#### Scenario: Status of PDM-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT; MEASURED evidence OPEN (F-B-BF-2). No record on file times the STARK worker alone with its keys resident: `~/.cache/box-bus/all.log:42` is the w1 wrap baseline ("STARK lfr_call half not run"), and the whole-call records include the wrap daemon (huginn h79, `~/Projects/Moriarty/llm-wiki/wiki/log.md:3691`; odin w5, `log.md:3935`). Owner lane: T14b. Evidence: `openvm-t14b` `crates/sherlock-wrap-export/examples/common/t14.rs:30` to `:48` (`T14B_SERVE`); Moriarty `t14b` 5e54263da `research/sherlock-t14b-20261114/RESULTS.md:15`.

### Requirement: PDM-005 Blinders are drawn at prove time inside the daemon
PDM-005: When the prover daemon serves a `prove` or `call` request, the prover daemon SHALL draw the blinders of that wrap proof from the OS CSPRNG inside the daemon process.

Verification: test `eq_zk` through the daemon (Moriarty `t11-zk` 71c4c001b `tools/sherlock-wrap-gpu/tests/t11_zk.rs:36`); NEW: the same check over the socket. Partly to build.

#### Scenario: Two served proofs of one statement
- **GIVEN** a daemon and one statement
- **WHEN** the client asks twice
- **THEN** the two proofs differ byte for byte and both verify

#### Scenario: Status of PDM-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. No recorded run of `eq_zk`. Owner lane: t11-zk. Evidence: Moriarty `t14b` 5e54263da `tools/sherlock-wrap-gpu/src/serve.rs:114` to `:117` (fresh `OsRng` per proof). No blinding-slot pool exists in the code. D-040 keeps OD-13 and PRD-002; owner ruling 2026-11-15: the per-proof draw satisfies one-use slots, so no slot pool is built. Astra-B review R-26.

### Requirement: PDM-006 The daemon serves only the loaded key set
PDM-006: If a request names a key manifest sha256 that differs from the key set the daemon loaded, then the prover daemon SHALL refuse the request.

Verification: NEW: a socket test with a wrong manifest digest. To build.

#### Scenario: A daemon from before the rekey
- **GIVEN** a daemon with the T10m keys
- **WHEN** a T25 call arrives
- **THEN** the daemon refuses the call and names both digests

#### Scenario: Status of PDM-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP. Owner lane: t14b, after T25. Evidence: UKS-005. `wrapd serve KEYFIX` loads one key set at start (Moriarty `t14b` 5e54263da `tools/sherlock-wrap-gpu/src/bin/wrapd.rs:161`). No request field carries a key manifest today.

### Requirement: PDM-007 Path C through the daemon
PDM-007: When a one-leaf call arrives at the prover daemon, the prover daemon SHALL serve the call on path C.

Verification: NEW: the `stats` reply names the path of the last call; a served one-leaf suite call (`t02 increment`). To build.

#### Scenario: A one-leaf Compact call
- **GIVEN** the daemon and the call `t02 increment`
- **WHEN** the call is served
- **THEN** `stats` reads path C and no internal node proof was made

#### Scenario: Status of PDM-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: PRD-003 and PRD-004 have no recorded check. Owner lane: t14b. Evidence: PRD-004 (path C and the daemon come together). Path C is BUILT in the chain driver (LOOP T7c); the T14b records do not state the path of the served call, and no PRD status is on file in the t14b research files.

### Requirement: PDM-008 The daemon feeds the prover GUI without secrets
PDM-008: When the prover GUI bridge sends a `stats` request, the prover daemon SHALL answer with the stage, the shard counts and the stage times of the running call and with no witness word.

Verification: NEW: a bridge test that reads `stats` during a served call and scans the reply for the 5 secret classes of ZKE-003. To build.

#### Scenario: The GUI during a call
- **GIVEN** a served call in its node stage
- **WHEN** the bridge asks for `stats`
- **THEN** the reply names the node stage and holds no tape word

#### Scenario: Status of PDM-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN: the bridge is not built (LOOP T21 stage 1). Owner lane: prover-gui. Evidence: Row T21 (LOOP T21: "a local bridge to the Sherlock proof server (T14b daemon)"; requirements on branch `prover-gui`, `prover-gui/requirements/`). The `stats` operation: Moriarty `t14b` 5e54263da `tools/sherlock-wrap-gpu/src/serve.rs:137` to `:200`. The GUI page is OBS (slice C).

### Requirement: PDM-009 No blinder state crosses requests
PDM-009: The prover daemon SHALL give a proof of statement X after a proof of statement Y the same distribution as a first proof of X, with no blinder of the earlier request in use.

Verification: test `slot_reuse`; NEW: the same check for the STARK worker and over the socket. Partly to build.

#### Scenario: X after Y
- **GIVEN** a seeded test build, statements X and Y
- **WHEN** the daemon proves Y, then X
- **THEN** the proof of X equals the proof of X from a fresh daemon with the same seed

#### Scenario: Status of PDM-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN: no recorded run. Owner lane: t11-zk. Evidence: Test `slot_reuse` compares X after Y with a fresh X on `Resident` objects (Moriarty `t11-zk` 71c4c001b `tools/sherlock-wrap-gpu/tests/t11_zk.rs:62` to `:87`, `test-seed` only, `#[ignore]`, untracked, no recorded run). It needs no pool. The STARK worker has no such test. Added after Astra-B review R-26.
