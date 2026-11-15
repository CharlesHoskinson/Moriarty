## ADDED Requirements

### Requirement: RSL-001 One universal leaf key for every R_SL app
RSL-001: When the rsl probe proves a leaf over the R_SL app proof of a suite call, the universal leaf key SHALL give an N1 pre-hash equal to the universal N1 pre-hash.

#### Scenario: One universal leaf key for every R_SL app
- **WHEN** the probe proves a leaf over the a05 app, which has 4 PUBLIC AIRs
- **THEN** its N1 pre-hash equals the N1 pre-hash of t02, s01 and d04

#### Scenario: Verification and status of RSL-001
- **WHEN** the status is reported
- **THEN** the source is hard rule one universal key set (LOOP.md line 86), the verification is rsl probe `n1_pre_hash_equals_universal_n1` true for a05, t02, s01 and d04 (odin run r10, openvm `rekey-chips` 6f773b2), and the status on 2026-11-15 is: DONE: LIST.md records odin r10 on 6f773b2 MEASURED, true for a05, t02, s01, d04 and z02. r8 on 2b59f9b gave FALSE for a05.

### Requirement: RSL-002 The 6-or-9 AIR bucket
RSL-002: The R_SL chip set SHALL give every R_SL app 6 or 9 AIRs with FP-1.

#### Scenario: The 6-or-9 AIR bucket
- **WHEN** a program has no hash
- **THEN** its app has 6 AIRs, with SHA_IN present and empty

#### Scenario: Verification and status of RSL-002
- **WHEN** the status is reported
- **THEN** the source is built design, openvm `rekey-chips` 5d77cfc and 6f773b2 (LEAD PICK, inside the single rekey list that the owner confirmed on 2026-11-15), the verification is AIR count of the rsl probe over the 142 suite calls (odin r10), and the status on 2026-11-15 is: BUILT, not MEASURED over the suite.

### Requirement: RSL-003 App-level proof of every suite call
RSL-003: When the suite prover runs the 142 R_SL suite calls, the R_SL app prover SHALL make one app proof per call that the stock verifier accepts.

#### Scenario: App-level proof of every suite call
- **WHEN** one call proof fails the stock verifier
- **THEN** the suite table records the call as BLOCKED with the verifier error

#### Scenario: Verification and status of RSL-003
- **WHEN** the status is reported
- **THEN** the source is row T23 (LOOP.md line 45), the verification is `rsl_suite` prove, odin and huginn halves (FINISH-PLAN queue), 142 of 142 accepted, and the status on 2026-11-15 is: OPEN: odin r5 MEASURED 142 of 142 calls execute equal to native words and `comm`. That is execution, not proof.

### Requirement: RSL-004 keccak256 kit
RSL-004: Where the machine version has the keccak256 kit, when a circuit calls `keccak256`, the R_SL chip set SHALL prove the call inside the 6-or-9 AIR bucket.

#### Scenario: keccak256 kit
- **WHEN** s02 calls `keccak256`
- **THEN** the leaf pre-hash equals the universal pre-hash

#### Scenario: Verification and status of RSL-004
- **WHEN** the status is reported
- **THEN** the source is owner 2026-11-15 "yes to both" (wiki log 09:06), the verification is `s02-foreign/keccak` words and `comm` equal to native, and RSL-001 holds for s02, and the status on 2026-11-15 is: BUILT: openvm `rekey-chips` bbf6ad3, c9601df, 699539c (row T24). No MEASURED run on file.

### Requirement: RSL-005 secp256k1 kit
RSL-005: Where the machine version has the secp256k1 kit, when a circuit calls `ec_mul`, `ec_mul_generator`, `into_coordinates` or `reconstitute_field` on secp256k1 values, the R_SL chip set SHALL prove the call inside the 6-or-9 AIR bucket.

#### Scenario: secp256k1 kit
- **WHEN** an ECDSA verification call with a valid signature runs
- **THEN** the outcome equals the native transcript

#### Scenario: Verification and status of RSL-005
- **WHEN** the status is reported
- **THEN** the source is owner 2026-11-15 "yes to both" (wiki log 09:06), the verification is the two s02 secp256k1 calls equal to native, and RSL-001 holds for them, and the status on 2026-11-15 is: BUILT: openvm `rekey-chips` c281a5f, `tests/rsl_secp.rs` against k256 0.13.4 (row T24). No MEASURED run on file.

### Requirement: RSL-006 jubjub operations as micro-ops
RSL-006: When a circuit calls a jubjub operation, the R_SL chip set SHALL check `on_curve` and `in_subgroup` (`8Q = P`) with micro-op sequences and no new AIR.

#### Scenario: jubjub operations as micro-ops
- **WHEN** a point outside the subgroup enters `ecMul`
- **THEN** the R_SL chip set refuses the row

#### Scenario: Verification and status of RSL-006
- **WHEN** the status is reported
- **THEN** the source is built design, openvm `rekey-chips` d8e5671 (in the single rekey list), the verification is `rsl_jubjub` passes on the rekey head, and the status on 2026-11-15 is: BUILT: end-to-end kit openvm 637ca2e (`rsl_jubjub_s01_ec_call`). No MEASURED run on file.

### Requirement: RSL-007 SHA-256 kit
RSL-007: When a circuit calls `persistentHash`, the R_SL chip set SHALL prove the hash on the SHA_IN and SHA_REQ AIRs with the openvm SHA-256 block hasher.

#### Scenario: SHA-256 kit
- **WHEN** a persistentHash segment does not fit the ledger alignment
- **THEN** the host refuses the call

#### Scenario: Verification and status of RSL-007
- **WHEN** the status is reported
- **THEN** the source is built design, openvm `rekey-chips` 4e78660 and 9364edb, the verification is `rsl_sha` 3 of 3 (odin r5, 1,335.5 s, MEASURED), and the status on 2026-11-15 is: BUILT and MEASURED on odin.

### Requirement: RSL-008 Canonical input range checks
RSL-008: If an ALU or Poseidon input word of an R_SL row is not canonical, then the R_SL chip set SHALL refuse the row.

#### Scenario: Canonical input range checks
- **WHEN** a prover writes `p + x` in place of `x`
- **THEN** the row fails its range check

#### Scenario: Verification and status of RSL-008
- **WHEN** the status is reported
- **THEN** the source is rekey list of wiki log 06:24 and lane start 06:50 (LEAD PICK), the verification is a forged non-canonical word per AIR, refused by the constraint checker, and the status on 2026-11-15 is: GAP: `~/.cache/rekey-t11/LIST.md` has no row for this change.
