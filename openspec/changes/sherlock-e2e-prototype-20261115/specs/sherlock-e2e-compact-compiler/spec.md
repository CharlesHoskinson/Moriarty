## ADDED Requirements

### Requirement: ECC-001 Stage 1 input is the stock compactc output
ECC-001: When a developer compiles a contract for Sherlock, the Mycroft compiler SHALL read only the ZKIR 3.1 files and the `contract-info.json` that stock `compactc` 0.36.0 writes.

Verification: run `mycroft-suite <suite dir> <out.json>` over the 33 compiled suite programs (`mycroft/compiler/src/bin/mycroft-suite.rs:10`); exists.

#### Scenario: Stock build directory
- **WHEN** `mycroft-suite` reads the `compactc` 0.36.0 build of `t02-counter-example`
- **THEN** it compiles `increment` with no patched `compactc` pass

#### Scenario: Status of ECC-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT, MEASURED 105 proof circuits and 1 pure export (`mycroft/factory/REPORT.md:8`, `mycroft/tests/compact-suite/RESULTS.md:5`). References: MYC-FE-001; supersedes the `Lflattened` tap of CF-001 for the prototype (D-016).

### Requirement: ECC-002 The Mycroft requirement set binds the compiler
ECC-002: The Mycroft compiler SHALL pass every gate of `mycroft/factory/gates.sh` on the MYC requirements of the modules frontend, values, lowering, witness-tape, ledger-ops, builtins and refusals.

Verification: unit `t18d-final` runs `gates.sh` (KRS emit, oracle-equal, golden, coverage, mutations, compare); exists.

#### Scenario: A surviving mutant
- **WHEN** a mutation of the compiler survives with no written argument
- **THEN** `gates.sh` fails and the compiler is not used for a settled call

#### Scenario: Status of ECC-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: MEASURED oracle-equal 104, coverage 865 covered and 41 argued and 0 open, mutations 58 of 62 killed with 4 argued survivors, compare 207 equal and 0 different (`REPORT.md:12-18`); files staged, not committed. The gates are a finite conformance result: premises S1 and S2, the builtin parameters, wide arguments and the typed secp256k1 layout stay open in `mycroft/lean/SORRIES.md:13-14`, `:83`, and CF-L1 is open (CF-012). References: LPC-013, LPC-019, MYC-LOW-019, CF-012.

### Requirement: ECC-003 One program image and one ABI per circuit
ECC-003: When the Mycroft compiler accepts a proof circuit, the Mycroft compiler SHALL emit one program image with `selector`, `n_out` and `code`, together with one ABI with `selector`, `tape_len`, `slots`, `l_e`, `n_e` and `kits`.

Verification: test module `tests_lowering.rs` and the `mycroft-suite` output `out/suite-coverage.json`; exists.

#### Scenario: ABI of a counter
- **WHEN** `t02-counter-example` `increment` compiles
- **THEN** the ABI gives `tape_len = 3 + slots`, and `l_e` equals the count of effect words in `code`

#### Scenario: Status of ECC-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (`mycroft/compiler/src/lower.rs:40-62`, `lower.rs:187-205`). References: MYC-LOW-012, CF-002 (`abi.json`).

### Requirement: ECC-004 Image and ABI bytes do not depend on a witness
ECC-004: When `mycroft-suite` serializes one circuit for two prepared calls with different tapes, the Mycroft compiler SHALL write byte-equal image files and byte-equal ABI files.

Verification: compiler-boundary test T-IMAGE-BYTES (same source and configuration, two calls, compare the serialized image and ABI); to build.

#### Scenario: Two tapes of one entry
- **WHEN** two calls of one entry have different tapes
- **THEN** the image file and the ABI file have equal sha256 values for both calls

#### Scenario: Status of ECC-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN at the compiler boundary. Downstream evidence only: `rsl_key_independent_of_tape` compares the lowered micro-ops of emitted files (openvm `rekey-chips` dcf1b4c, `crates/sherlock-leaf/tests/rsl.rs:60`). The behaviour itself is MYC-FE-009 and MYC-FE-010; this requirement adds the serialization check. References: MYC-FE-009, MYC-FE-010, CF-004.

### Requirement: ECC-005 The emitted instructions are the 13 straight-line shapes
ECC-005: The Mycroft compiler SHALL emit only the 13 instruction shapes of `SInstr`: `Field`, `Mov`, `Eq`, `Select`, `Lt`, `Divmod2`, `Range`, `AssertEq`, `Assert`, `Tape`, `Eff`, `Chip` and `Ret`.

Verification: test T-CHIP-RULES (every `SInstr` constructor and every `kitsOf` entry of a suite program is a row of the R_SL rule table `compile::RULES`); to build.

#### Scenario: A contract adds no rule
- **WHEN** a new contract compiles
- **THEN** its image uses only the 13 shapes, and no chip set, rule or universal key changes

#### Scenario: Status of ECC-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: code BUILT (`lower.rs:23-37`; rule table of 16 rules at openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/rsl/compile.rs:73-90`), test OPEN. References: MYC-LOW-013; the rule lists of MYC-CHIP-001 and ARCHITECTURE.md section 1.7 are superseded by the built table (D-017).

### Requirement: ECC-007 keccak256 lowers to the keccak256 chip rule
ECC-007: When a circuit calls `keccak256`, the Mycroft compiler SHALL emit one `chip` instruction that the R_SL rule `R_SL_KECCAK256` proves.

Verification: `mycroft-suite` compiles the s02 `keccak256` call and test `rsl_suite_exec_equal_native` gives W and `comm` equal to native for it; to build.

#### Scenario: s02 keccak call
- **WHEN** the s02 `keccak256` circuit compiles
- **THEN** the image holds a `chip` instruction of the keccak256 kit and no tier 3 refusal

#### Scenario: Status of ECC-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. The rule exists (`compile.rs:73-90`, tests in `crates/sherlock-leaf/tests/rsl_keccak.rs`); MYC-BLT-007 still sends `keccak256` to a tier 3 kit, and `t23.py:141-152` still blocks the call as deferred. References: RSL-004, owner 2026-11-15 "yes to both" (D-019); supersedes the `keccak256` clause of MYC-BLT-007.

### Requirement: ECC-009 Deployable profiles use anchor slot 15
ECC-009: Where the Mycroft compiler builds a deployable contract, the Mycroft compiler SHALL use top-level slot 15 as `anchor_slot`.

Verification: test T-SLOT15 (compile t02 `increment` in the deploy profile, W addresses no root key 15, and a contract with a field at slot 15 is refused with the MYC-SEAL-021 code); to build.

#### Scenario: Spike constants stay out of a deploy
- **WHEN** a deployable image is built
- **THEN** it holds no value of `SPIKE_SLOT`, `CP_STAND_IN` or `SALT` from `main.rs:30-36`

#### Scenario: Status of ECC-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. The spike driver has `SPIKE_SLOT = 0` (`mycroft/compiler/src/main.rs:30`), and the refusal code is an open item (`REPORT.md:103-115`). References: ANC-013, ANC-015, MYC-SEAL-005, MYC-SEAL-021 (D-021).

### Requirement: ECC-010 Every suite circuit compiles or has a named refusal
ECC-010: When `mycroft-suite` runs on the 33 compiled suite programs, the Mycroft compiler SHALL give each proof circuit either an image or one refusal code.

Verification: `mycroft/compiler/SUITE-COVERAGE.md` and `out/suite-coverage.json`; exists.

#### Scenario: Count of unsupported circuits
- **WHEN** the coverage table is written
- **THEN** the column unsupported reads 0

#### Scenario: Status of ECC-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: MEASURED 33 programs, 106 circuits, 208 calls: 139 equal, 62 refused as native, 1 pure, 6 blocked on kits, 0 unsupported (`REPORT.md:8-9`). The 6 blocked calls close with ECC-007, CF-013 and RSL-006.

### Requirement: ECC-011 The compiler equals the Lean oracle
ECC-011: When the factory gates replay the golden vectors, the Mycroft compiler SHALL give the statement that the Lean oracle `OracleMain` gives for each vector.

Verification: gate oracle-equal of `gates.sh`, and the Quint replay under `mycroft/factory/replay/`; exists.

#### Scenario: Oracle difference
- **WHEN** one golden vector differs between the Rust compiler and the Lean oracle
- **THEN** the gate fails and names the vector

#### Scenario: Status of ECC-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: MEASURED 104 of 104 equal; Lean oracle against Quint 1,226 of 1,226 transitions agree (`REPORT.md:10-17`). References: MYC-LOW-010, MYC-TRN-017.

### Requirement: ECC-012 The compile command
ECC-012: When `sherlock compile` admits every requested proof circuit of a `compactc` build directory, the Mycroft compiler SHALL write one image file and one ABI file per proof circuit and exit with code 0.

Verification: CLI test T-COMPILE-CLI on `t02-counter-example`; to build.

#### Scenario: Compile of a counter
- **WHEN** the developer runs `sherlock compile` on the build of `t02-counter-example`
- **THEN** the output directory holds an image and an ABI for `increment`

#### Scenario: Status of ECC-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. No `sherlock` binary exists; the binaries are `mycroft-t18c`, `mycroft-kink` and `mycroft-suite`, with no subcommands (`mycroft/compiler/Cargo.toml:15-22`). The prototype runs `mycroft-suite` behind the runbook; the one-binary command is work after T15 (D-085). References: CF-002; the settlement commands are SCL (slice C).

### Requirement: ECC-013 A refusal is a non-zero exit with its code
ECC-013: If the Mycroft compiler refuses one requested circuit, then the compile command SHALL exit with a non-zero code, print the refusal code of `refusal_code` and write no image file for the request.

Verification: CLI test T-COMPILE-REFUSE (a request with one accepted circuit and one circuit whose ZKIR version field reads 3.2 gives `MYC-REF-032/irVersion` and an empty output directory); to build.

#### Scenario: Wrong IR version
- **WHEN** one of two requested circuits states ZKIR version 3.2
- **THEN** the command prints `MYC-REF-032/irVersion` and writes no image

#### Scenario: Status of ECC-013
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: codes BUILT (`pipeline.rs:125-137`), command OPEN. No partial output is a LEAD PICK for T15 of this change. References: MYC-FE-002, MYC-FE-011.

### Requirement: ECC-014 Jubjub inputs are typed tape words
ECC-014: When a circuit has an input of a Jubjub point type or a Jubjub scalar type, the Mycroft compiler SHALL read the input as tape words in the native public-input encoding.

Verification: `mycroft-suite` compiles both `j02` calls with W equal to native; to build.

#### Scenario: Point input
- **WHEN** a `j02` circuit takes a Jubjub point
- **THEN** the image reads two tape words and checks that the point is on the curve and in the subgroup

#### Scenario: Status of ECC-014
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. The input loop accepts `Scalar<BLS12-381>` and the secp256k1 types and refuses every other input type with `FeRefusal::InputType` (`mycroft/compiler/src/zkir.rs:570-583`, uncommitted); the s01 `ec` call covers Jubjub values made inside a circuit, not Jubjub inputs. Found by the Astra-A draft. References: MYC-FE-021, MYC-VAL-014, RSL-006, EKT-010.
