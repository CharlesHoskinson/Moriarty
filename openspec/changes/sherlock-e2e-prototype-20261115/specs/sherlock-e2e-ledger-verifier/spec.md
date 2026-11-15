## ADDED Requirements

### Requirement: LVR-001 The ledger verifier is the stock 9.1.0.1 verifier
LVR-001: The ledger check tool SHALL verify each wrap proof with `VerifierKey::verify` of the stock `midnight-transient-crypto` crate at ledger tag 9.1.0.1 (rev `a2b30128`).

Verification: `sherlock-wrap-ledger-check <dir>` on the T25 wrap output; T17 (a) record `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:29`; exists.

#### Scenario: The check tool builds from the pinned ledger revision
- **GIVEN** the crate `tools/sherlock-wrap/ledger-check`
- **WHEN** its dependency graph is listed
- **THEN** each midnight ledger crate comes from rev `a2b30128ca4b660b73a068e29b56b48ede8c9a74`

#### Scenario: Status of LVR-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. References: MID-001, LW-017 (sherlock-processor-final-20261111); hard rule "ledger 9.1.0.1 verifier unchanged" (LOOP.md:102). Evidence: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/ledger-check/Cargo.toml:1-2,14-23`; `ledger-check/src/main.rs:44-46`. Check: `sherlock-wrap-ledger-check <dir>` on the T25 wrap output; T17 (a) record `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:29`.

### Requirement: LVR-002 Ledger crates resolve to the stock revision
LVR-002: The ledger check crate SHALL resolve each of the 4 ledger crates `midnight-transient-crypto`, `midnight-serialize`, `midnight-base-crypto` and `midnight-storage-core` to git rev `a2b30128` of midnight-ledger.

Verification: MISSING as a gate. Task: a `cargo metadata` check of the four package sources; to build.

#### Scenario: A local override fails the gate
- **GIVEN** a `path` override or another rev for `midnight-transient-crypto` in the manifest
- **WHEN** the manifest gate runs
- **THEN** the gate fails; a `[patch.crates-io]` entry that selects rev `a2b30128` passes, and published crates such as `midnight-circuits` are not checked

#### Scenario: Status of LVR-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: MID-001, LW-017, processor-final. Evidence: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/ledger-check/Cargo.toml:1-2,14-15,19-23`; the wrap crate excludes the check crate from its workspace, `tools/sherlock-wrap/Cargo.toml:10`. Check: MISSING as a gate. Task: a `cargo metadata` check of the four package sources.

### Requirement: LVR-003 The key passes the ledger's own reader
LVR-003: When the ledger check tool loads `wrap.vk`, the ledger check tool SHALL read the key through `VerifierKey::deserialize` in the `MidnightVK` framing.

Verification: every `sherlock-wrap-ledger-check` run; T10m record (LOOP.md:31); exists.

#### Scenario: A G4 key is not read
- **GIVEN** a key with G4 lane gates
- **WHEN** the ledger reader loads it
- **THEN** the reader refuses the key

#### Scenario: Status of LVR-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. References: LW-017, MID-001. Evidence: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/ledger-check/src/main.rs:42-44`; M7-0 MEASURED on a toy key (LW-017 text). Check: every `sherlock-wrap-ledger-check` run; T10m record (LOOP.md:31).

### Requirement: LVR-004 Tamper controls on every check
LVR-004: When the ledger check tool verifies a wrap proof, the ledger check tool SHALL also verify 7 vectors with one changed public input and 3 proofs with one flipped byte.

Verification: the `E5JSON` record of each run (`changed_lane_refused`, `flipped_proof_byte_refused`); T10m and T17 (a) runs; exists.

#### Scenario: The named lanes are covered
- **GIVEN** an honest wrap output
- **WHEN** the check runs
- **THEN** lane 0, lane 10 (anchor), lanes 20 and 34 (`claim_pub`), lane 39 (`outcome_pub`), the middle lane and the last lane are each changed by 1 and refused

#### Scenario: Status of LVR-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/ledger-check/src/main.rs:51-58,67-71`. Check: the `E5JSON` record of each run (`changed_lane_refused`, `flipped_proof_byte_refused`); T10m and T17 (a) runs.

### Requirement: LVR-005 A passed control fails the check
LVR-005: If the ledger verifier accepts a tamper control or refuses the honest proof, then the ledger check tool SHALL exit with a nonzero status.

Verification: MISSING a negative fixture; the assert is exercised only on passing runs; to build.

#### Scenario: An accepted control stops the lane
- **GIVEN** a wrap output whose flipped-byte control verifies
- **WHEN** the check ends
- **THEN** the process panics and the caller sees a nonzero status

#### Scenario: Status of LVR-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. Evidence: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/ledger-check/src/main.rs:87`. Check: MISSING a negative fixture; the assert is exercised only on passing runs.

### Requirement: LVR-006 Native check before submit
LVR-006: When a ledger call is built, the settle runner SHALL run the native ledger 9.1.0.1 validator on the call before the submit.

Verification: T17 (a) run record; rerun in LPC-008; exists.

#### Scenario: The native result equals the node result
- **GIVEN** the tampered call R1
- **WHEN** the native validator runs
- **THEN** the result is "failed to verify proof: Invalid proof", the same refusal the node gives as custom error 115

#### Scenario: Status of LVR-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RUNBOOK.md:12`, `RESULTS.md:29`; `t17.sh:22`. Check: T17 (a) run record; rerun in LPC-008.

### Requirement: LVR-007 A natively refused honest call is not submitted
LVR-007: If the native ledger validator refuses the honest call, then the settle runner SHALL submit nothing and mark the proven call as refused.

Verification: MISSING a test that injects a native refusal; to build.

#### Scenario: A rerun proves again
- **GIVEN** an honest call that the native validator refuses
- **WHEN** the runner stops
- **THEN** the state records the refusal and the next run proves a new call

#### Scenario: Status of LVR-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:681,712-713`. Check: MISSING a test that injects a native refusal.

### Requirement: LVR-008 Chain identity in the run state
LVR-008: When the settle runner first connects to a node for a state folder, the settle runner SHALL record the genesis hash and the node version in the run state.

Verification: `receipts.json` of T17 (a) for the first record. MISSING: a later node version at the same genesis is not recorded; task for STL-006: record each observed node and runtime version with height and time; exists in part.

#### Scenario: A second chain is refused for one state folder
- **GIVEN** a state folder that holds the genesis hash of an earlier connect
- **WHEN** the runner connects to a node with another genesis hash
- **THEN** the runner stops with "same local chain as the deploy"

#### Scenario: Status of LVR-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: MID-006 (node build `2.1.101-bbd616b1`), STL-006 (hard-fork split of the comparison), completion. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:225-229` (`st.chain ??= chain`: only the first observation is saved). Check: `receipts.json` of T17 (a) for the first record. MISSING: a later node version at the same genesis is not recorded; task for STL-006: record each observed node and runtime version with height and time.
