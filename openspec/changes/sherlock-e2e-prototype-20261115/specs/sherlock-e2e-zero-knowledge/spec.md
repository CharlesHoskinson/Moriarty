## ADDED Requirements

### Requirement: ZKE-001 No constant r_P outside a test build
ZKE-001: If a build without the cargo feature `test-seed` reaches a code path that sets `r_P` from a constant, then the chain driver SHALL exit with code 4 and write no proof.

Verification: tests `blind::tests::os_words_are_fresh_and_below_p` and `zkguard::tests::seeded_export_refused_without_feature` (logs `~/.cache/t11zk/logs/unit-blind.log`, `unit-zkguard.log`). NEW: a source check that no constant `r_P` is outside `cfg(feature = "test-seed")`. Partly to build.

#### Scenario: A production chain run
- **GIVEN** a build without `test-seed`
- **WHEN** the chain driver prepares a run
- **THEN** `r_P` comes from `deploy.secret` and no constant path runs

#### Scenario: Status of ZKE-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN: the T11 files are not committed. Owner lane: t11-zk. Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:493`, `:927`, `:1293`, `:1373`, `:1793` set `r_P = 1000 + i`. openvm `t11-zk` cce8c1d, dirty tree: `t7_chain.rs:1387` to `:1391` call `RunBlind::prepare` (`crates/sherlock-leaf/src/blind.rs:180`, untracked; `REFUSED_EXIT = 4` at `blind.rs:26`; `seeded_allowed` at `blind.rs:70`). LOOP T11.

### Requirement: ZKE-002 A test-seed binary does not settle
ZKE-002: If the prover binary of a run was built with the cargo feature `test-seed`, then the settlement runner SHALL refuse to submit the proof of that run to Preview.

Verification: NEW: the run record field `features`; a test of the settlement runner with a record that lists `test-seed`. To build.

#### Scenario: A seeded proof reaches the settle step
- **GIVEN** a wrap proof made by a `test-seed` build
- **WHEN** the runner prepares the transaction
- **THEN** the runner refuses and sends no transaction

#### Scenario: Status of ZKE-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP. Owner lanes: t11-zk (the field), t23-suite (the refusal). Evidence: Features: openvm `t11-zk` cce8c1d `crates/sherlock-leaf/Cargo.toml:28` and `crates/sherlock-wrap-export/Cargo.toml:38` (dirty). ZK-014 covers a build without the feature; no rule covers a build with it. BRIEF hard rule: "seeded proofs test-only".

### Requirement: ZKE-003 Nothing but the call vector and the wrap proof is published
ZKE-003: When a call is settled, the privacy scan SHALL find no witness word, no STARK proof bytes, no `r_P`, no input salt and no plain `prog_d` in the call vector, the transaction and the ledger state.

Verification: the privacy scan of the T17 runbook on every settled call of T23 (run by OBS, slice C). Exists.

#### Scenario: The scan after a settled call
- **GIVEN** a settled call of `t02 increment`
- **WHEN** the scan reads the transaction and the contract state
- **THEN** the scan reports 0 hits for the 5 secret classes

#### Scenario: Status of ZKE-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT for T17 (a) on the T10m keys. Rerun on the T25 keys with the fresh `r_P` and salt. Evidence: LOOP T10a: "no private R operand in call vector, transcript or ledger state (MEASURED)". LOOP T17 (a): "privacy scans pass". ZK-002, ZK-004, ZK-007.

### Requirement: ZKE-004 Secrets and witnesses stay inside the owned boxes
ZKE-004: If a file transfer of `deploy.secret`, `run.secret`, a witness file, an execution trace or a STARK proof names a destination host outside the owned-box roster of the user, then the box bus SHALL refuse the transfer.

Verification: NEW: a destination check in the transfer step of BXF-007 against the roster of BXF-001, with a positive test (odin to huginn) and a refusal test for each of the 5 file classes. To build.

#### Scenario: A witness goes to another owned box
- **GIVEN** a prepared run on muninn and odin in the roster
- **WHEN** the run files move to odin over the SSH link
- **THEN** the transfer is admitted and the files keep mode 0600

#### Scenario: A witness goes to a rented host
- **GIVEN** a transfer request for `run.secret` to a host that is not in the roster
- **WHEN** the bus checks the request
- **THEN** the bus refuses the transfer and logs the file class

#### Scenario: Status of ZKE-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP. Owner lane: box tooling (lead). Evidence: openvm `t11-zk` cce8c1d `crates/sherlock-leaf/src/blind.rs:180` to `:187` (`deploy.secret`, `run.secret`). ZK-001 permits these files inside the owned-device trust domain; ZK-018 refuses case B. No code enforces either today. Owner 2026-11-15 09:05 (`~/Projects/Moriarty/llm-wiki/wiki/log.md:4111` to `~/Projects/Moriarty/llm-wiki/wiki/log.md:4113`): case A is any box the user owns; case B waits for the Veritee model. Corrected after Astra-B review R-01: the first text banned transfers between owned boxes.

### Requirement: ZKE-005 A missing deployment secret stops the call
ZKE-005: If `deploy.secret` of a deployed program is absent at call time, then the Sherlock prover SHALL refuse the call.

Verification: NEW: a unit test of `RunBlind::load` with no file; test `blind::tests::secret_round_trip_is_0600` for the present case. To build.

#### Scenario: A new box without the secret
- **GIVEN** odin with the keys and no `deploy.secret`
- **WHEN** a call for the deployed program starts
- **THEN** the prover refuses the call and draws no new `r_P`

#### Scenario: Status of ZKE-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN: the file is untracked. Owner lane: t11-zk. Evidence: openvm `t11-zk` cce8c1d `crates/sherlock-leaf/src/blind.rs:187` (`RunBlind::load`). `C_P = H(SHK_PROGC, r_P, prog_d)` (`research/sherlock-final-design-20261111/ARCHITECTURE.md:209`): a new `r_P` gives another `C_P`, which the deployed contract refuses.

### Requirement: ZKE-006 Blinders do not reach logs
ZKE-006: The blinder types of the leaf crate SHALL print no secret word in their `Debug` output.

Verification: the test above on the T25 tree; the privacy scan of ZKE-003 on the box logs. Exists.

#### Scenario: A debug print
- **GIVEN** a `RunBlind` value
- **WHEN** a log line formats it with `{:?}`
- **THEN** the line holds no word of `r_P` or the salt

#### Scenario: Status of ZKE-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN: not committed. Owner lane: t11-zk. Evidence: Test `blind::tests::debug_never_prints_words` (openvm `t11-zk` cce8c1d `crates/sherlock-leaf/src/blind.rs`, untracked; log `~/.cache/t11zk/logs/unit-blind.log`).

### Requirement: ZKE-007 Proofs of one statement differ on each backend and across backends
ZKE-007: When the wrap prover proves one statement twice on the CPU backend and twice on the GPU backend, the wrap prover SHALL give four proofs whose byte strings are pairwise not equal.

Verification: test `eq_zk` on the GPU; NEW: the same test on the CPU wrap entry point and a cross comparison on one public statement; each proof passes the ledger 9.1.0.1 verifier on muninn. Partly to build.

#### Scenario: Four proofs
- **GIVEN** one statement and the T25 keys
- **WHEN** the CPU backend and the GPU backend each prove it twice
- **THEN** the 4 byte strings are pairwise not equal and the ledger verifier accepts each

#### Scenario: Status of ZKE-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN: the T11 GPU half is queued on odin (LOOP Finish plan). Owner lane: t11-zk. Evidence: Test `eq_zk` proves twice on one GPU `Resident` (Moriarty `t11-zk` 71c4c001b `tools/sherlock-wrap-gpu/tests/t11_zk.rs:36` to `:56`, `#[ignore]`, untracked, no recorded run); it has no CPU leg. Moriarty `t14-gpu` 3fd18b166 `research/sherlock-t14-gpu-20261114/RESULTS.md:20`: CPU and GPU call proofs are not byte-equal. ZK-015 and ZK-017 are the reused texts. Reworked after Astra-B review R-31: a fixed blinder stream per device would pass a cross-device check alone.
