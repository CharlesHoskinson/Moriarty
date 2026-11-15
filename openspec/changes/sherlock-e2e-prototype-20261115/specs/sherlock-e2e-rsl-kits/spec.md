## ADDED Requirements

### Requirement: EKT-001 The rule table of chip set 3
EKT-001: The R_SL chip set SHALL have exactly the 16 rules of `compile::RULES`: `R_SL_FIELD`, `R_SL_SELECT`, `R_SL_EQ`, `R_SL_LT`, `R_SL_DIVMOD2`, `R_SL_RANGE`, `R_SL_ASSERT`, `R_SL_TAPE`, `R_SL_EFF`, `R_SL_RET`, `R_SL_HALT`, `R_SL_SPONGE`, `R_SL_BYTES`, `R_SL_EC`, `R_SL_SHA256` and `R_SL_KECCAK256`.

Verification: test T-RULES-D (the digest of the rule table equals the chip-set rule digest recorded in the T25 key manifest); to build.

#### Scenario: A new kit rule
- **WHEN** a rule is added to the table after T25
- **THEN** `rules_d` changes and the change needs a new machine version (EUI-001)

#### Scenario: Status of EKT-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: table BUILT (openvm `rekey-chips` dcf1b4c, `crates/sherlock-leaf/src/rsl/compile.rs:73-90`). The chip-set rule digest (`src/rsl/mod.rs:10` calls it the `rules_d` of the chip set) is a digest of this table; it is not the per-leaf `rules_d` of CORE-006. secp256k1 has no rule name: `R_SL_SECP` is a counter name (`compile.rs:819`) and the typed path sits outside the table; gap: name the secp256k1 rule or state its micro-op path before T25 (rekey-chips lane). References: MYC-CHIP-001 (superseded list), ARCHITECTURE.md section 1.7 (estimate of 14), D-017.

### Requirement: EKT-002 Rules lower to micro-op AIRs
EKT-002: The R_SL chip set SHALL prove every rule with the AIRs WORD, ALU, PUBLIC, RANGE and SHA_IN, plus SHA_REQ, SHA_HASHER and BITWISE8 for a SHA-256 program or KECCAK_REQ, KECCAK_PERM and BITWISE8 for a keccak256 program.

Verification: test `rsl_air_count_in_universal_leaf_bucket` (`crates/sherlock-leaf/tests/rsl.rs:191`); exists.

#### Scenario: Program with no hash
- **WHEN** a program calls no hash
- **THEN** its app has 5 AIRs, 6 with the FP-1 AIR, and SHA_IN is empty

#### Scenario: Status of EKT-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (`crates/sherlock-leaf/src/rsl/trace.rs:4-10`, `:123-138`, `MAX_APP_AIRS = 8` at `:49`); LEAD PICK of the rekey-chips lane (`src/rsl/mod.rs:5-11`), inside the rekey list that the owner confirmed (wiki 2026-11-15 09:04). References: RSL-002, D-017.

### Requirement: EKT-003 Every emitted chip pair has a rule
EKT-003: When the R_SL compiler reads a `chip` instruction of a suite program, the R_SL compiler SHALL lower the pair of kit and builtin id to micro-ops.

Verification: `examples/rsl_suite.rs` over every `rsl-call/1` file of the suite gives no status `unsupported`; exists.

#### Scenario: Interface drift
- **WHEN** Mycroft emits a builtin id that `compile::build` does not know
- **THEN** the status is `unsupported` with the id, and the suite table counts the call as blocked

#### Scenario: Status of EKT-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: MEASURED 142 calls with no `unsupported` (odin r5, `LIST.md:30`); 144 call files exist after the kits (wiki log 2026-11-15 10:02), and the 2 ECDSA calls wait on the Mycroft secp256k1 frontend; the suite run over the kit calls is OPEN. References: RSL-003 to RSL-006, CF-013, ECC-007.

### Requirement: EKT-004 A mixed-hash program has a named capacity refusal
EKT-004: While the R_SL chip set has no path that proves SHA-256 and keccak256 in one program, if a program calls both `persistentHash` and `keccak256`, then the R_SL compiler SHALL refuse the program with `Refuse::Unsupported` and the text "persistentHash and keccak256 in one program".

Verification: test `rsl_sha_and_keccak_in_one_program_refused` (`crates/sherlock-leaf/tests/rsl_keccak.rs:150`); exists.

#### Scenario: Mixed hash program
- **WHEN** one circuit calls `persistentHash` and `keccak256`
- **THEN** the compiler names both and makes no key

#### Scenario: Status of EKT-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (`compile.rs:847`). This is an open implementation gap, not a language rule: a valid Compact program with both hashes has no R_SL proof today. The refusal is temporary and named; the capability is incomplete until a path keeps both hashes and the universal key rule (D-024). No suite program needs both. References: RSL-002, RSL-004, CPR-005.

### Requirement: EKT-005 The transient hash is the fixed-length Poseidon sponge
EKT-005: When a program calls `transientHash` on n words, the R_SL executor SHALL run the width-3 Poseidon permutation of midnight-circuits 7.3.0 as a fixed-length sponge with the start register `[0, 0, n]`.

Verification: test `rsl_transient_hash_fixed_length` (`crates/sherlock-leaf/tests/rsl_exec.rs:40`); exists.

#### Scenario: Three-word hash
- **WHEN** `s01` `th3` hashes three words
- **THEN** the result equals the native `transient_hash`

#### Scenario: Status of EKT-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (`compile.rs:336`, `:375`; `LIST.md` row ce309c6). This refines MYC-BLT-001: it fixes the sponge form (fixed length, start register) that MYC-BLT-001 leaves to the native definition. References: MYC-BLT-001, CF-005 (audit-fixed text).

### Requirement: EKT-006 Each kit is checked against a pinned reference
EKT-006: When the kit tests run, the R_SL kit test suite SHALL compare SHA-256 with crate `sha2`, keccak256 with the Ethereum vectors, Jubjub with midnight-curves 0.3.1, secp256k1 with k256 0.13.4 and Poseidon with midnight-circuits 7.3.0.

Verification: tests `rsl_sha_calls_prove`, `rsl_keccak_host_matches_vectors`, `rsl_jubjub_matches_native`, `rsl_secp_matches_k256` and `rsl_transient_hash_fixed_length`; exists.

#### Scenario: Reference version change
- **WHEN** the pinned version of a reference crate changes in `Cargo.toml`
- **THEN** the kit tests rerun before the kit is used for a settled call

#### Scenario: Status of EKT-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: MEASURED on huginn at 699539c and dcf1b4c: `rsl_jubjub` 4 of 4, `rsl_keccak` 6 of 6, `rsl_secp` 6 of 6; odin r12 `rsl`, `rsl_exec`, `rsl_sha` rc 0 (wiki log 2026-11-15 10:01, 10:02). Test locations: `tests/rsl_sha.rs:37`, `tests/rsl_keccak.rs:31`, `tests/rsl_jubjub.rs:22`, `tests/rsl_secp.rs:69`. References: RSL-004 to RSL-007.

### Requirement: EKT-007 A trace that violates a kit relation is refused
EKT-007: If a kit trace violates the kit relation for the admitted inputs, then the R_SL app verifier SHALL refuse the proof.

Verification: forged-cell tests with the inputs, the key and the statement unchanged: `rsl_forged_rows_refused`, `rsl_sha_forged_rows_refused`, `rsl_keccak_forged_rows_refused`, `rsl_keccak_wrong_request_key_refused` and `rsl_secp_word_soundness`; exists.

#### Scenario: Forged digest
- **WHEN** one byte of a SHA-256 digest row is changed and the inputs stay
- **THEN** the stock verifier refuses the app proof

#### Scenario: Status of EKT-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (`tests/rsl.rs:103`, `tests/rsl_sha.rs:51`, `tests/rsl_keccak.rs:105`, `:172`, `tests/rsl_secp.rs:253`). A valid witness that differs from the host trace stays admissible. No forged-cell test exists for Jubjub; gap, owner lane rekey-chips. References: MYC-CHIP-002, MYC-CHIP-003.

### Requirement: EKT-009 The secp256k1 interface is one file
EKT-009: The secp256k1 kit SHALL accept exactly the operations, type tags and word layouts of `SECP-INTERFACE.md` sections 2 to 4.

Verification: integration test T-SECP-EMIT (the s02 ECDSA call files that Mycroft emits run in `compile::build`, and their leaf pre-hash equals the universal one); to build.

#### Scenario: Non-canonical scalar
- **WHEN** a tape gives a `Scalar<Secp256k1>` at or above the group order
- **THEN** the kit rejects the call, as the native VM does

#### Scenario: Status of EKT-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN as an integration check. `rsl_secp_chip_program` (`tests/rsl_secp.rs:318`) runs a hand-written program, not Mycroft output. R_SL side BUILT (c281a5f, dcf1b4c), Mycroft side in the working tree (`mycroft/compiler/src/gir.rs:33-51`, `zkir.rs:118-170`). The interface file is in `~/.cache/rekey-chips/`, outside every repository; gap: commit it with T24. References: CF-013, MYC-FE-013 to MYC-FE-020, MYC-TAPE-015.

### Requirement: EKT-010 The Jubjub interface is written
EKT-010: The Jubjub kit SHALL accept exactly the operations and word layouts of one committed interface file that names `EcAdd`, `EcMul`, `EcMulGenerator`, `HashToCurve`, `FromCoordinates`, `IntoCoordinates` and `JubjubScalarFromNative`.

Verification: tests `rsl_jubjub_matches_native`, `rsl_jubjub_htc_constants` and `rsl_jubjub_s01_ec_call` against the file; file to build, tests exist.

#### Scenario: Point off the subgroup
- **WHEN** `FromCoordinates` receives a point outside the prime-order subgroup
- **THEN** the kit rejects the call

#### Scenario: Status of EKT-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. `gir.rs:52` states "no interface file exists" for `EcAdd` (LEAD PICK); dispatch at `compile.rs:656-688`. Owner lane: Mycroft with rekey-chips, row T24. References: RSL-006, MYC-BLT-006, MYC-FE-021.

### Requirement: EKT-011 ALU results are canonical
EKT-011: If an exact ALU micro-op gives a result at or above its modulus, then the R_SL executor SHALL reject the call.

Verification: test T-ALU-EXACT (an exact micro-op whose inputs give a result at or above the modulus is rejected); to build.

#### Scenario: Hint above the modulus
- **WHEN** a hint word is at or above the modulus of its micro-op
- **THEN** the executor rejects the call before a trace is made

#### Scenario: Status of EKT-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: code BUILT (`compile.rs:192`); the existing tests check non-canonical hints (`compile.rs:166`, `:177`; `rsl_secp_random_field_ops`, `rsl_rejected_tapes_refused`), which is a different condition. The range checks of canonical ALU and Poseidon inputs are a T25 rekey row with no `LIST.md` row (AUDIT.md M8). References: RSL-008, MYC-REF-036.
