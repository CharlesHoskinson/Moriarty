## ADDED Requirements

### Requirement: ERS-001 The call record `rsl-call/1`
ERS-001: When the `mycroft-emit` tool writes one suite call, the `mycroft-emit` tool SHALL write one JSON file with `schema` equal to `rsl-call/1` and the fields `id`, `circuit`, `label`, `index`, `kits`, `selector`, `n_out`, `l_e`, `n_e`, `tape_len`, `program`, `tape`, `native_words` and `native_comm`.

Verification: schema test T-CALL-SCHEMA of ERS-014 over the five fixtures `crates/sherlock-leaf/tests/data/*.rsl.json`; fixtures exist, test to build.

#### Scenario: File name of a call
- **WHEN** the tool emits call 0 of circuit `th3` of `s01-natives`
- **THEN** the file is `s01-natives.th3.th3.0.rsl.json`

#### Scenario: Status of ERS-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (openvm `rekey-chips` dcf1b4c, `crates/sherlock-leaf/tools/mycroft-emit/src/main.rs:120-125`). The format has no schema file and no `prog_d` field; the execution tests read call files from `$RSL_CALLS` or `~/.cache/rekey-chips/calls` (`tests/rsl.rs:18-22`) and check no schema. References: MYC-TAPE-001, MYC-TAPE-008 (D-020).

### Requirement: ERS-002 The call record and the statement record are private
ERS-002: While a call record or an `RslStatement` exists, the Sherlock toolchain SHALL keep the record on boxes that the user owns.

Verification: transport test T-PRIVATE-RECORDS (a move of a call record or an `RslStatement` to a host outside the box roster is refused) plus the field-level scan of the T17 runbook over the transaction and the ledger state; scan exists, transport test to build.

#### Scenario: Record on a helper box
- **WHEN** the lead moves a call record to huginn or odin
- **THEN** the box belongs to the user (case A), and a host outside the roster is refused

#### Scenario: Status of ERS-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: scan MEASURED on T17 (a) (`research/sherlock-t17-preview-20261114/RESULTS.md` on `t17-preview` 9e1e7f398); OPEN for R_SL calls and for the transport test. A value scan cannot show absence: W can hold disclosed witness words (MYC-TAPE-012). References: ZK-001, ZK-002, ZK-018, MYC-CLM-009, HLP-004, ZKE-004, D-002, D-010.

### Requirement: ERS-003 One PUBLIC row per statement word
ERS-003: The R_SL chip set SHALL hold one PUBLIC row for each statement word, the word `comm` first and the effect words W after it, in order.

Verification: test `rsl_public_values_are_the_statement` (`crates/sherlock-leaf/tests/rsl.rs:144`); exists.

#### Scenario: No tape word in the statement rows
- **WHEN** the PUBLIC trace of `t02` `increment` is built
- **THEN** its rows are `comm` and W in byte limbs, and no tape word, witness word or `rand` appears

#### Scenario: Status of ERS-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (`crates/sherlock-leaf/src/rsl/air.rs:198-214`, `compile.rs:101-111` field `public`). The PUBLIC rows are trace rows, not public values of the app proof (ERS-012). MYC-CHIP-004 states the sealed form `[bind, comm] ++ anchor ++ W ++ zeros` of `2 + L_TX` words; the prototype uses the run form (D-022). References: MYC-CHIP-004, MYC-TAPE-010.

### Requirement: ERS-005 The openvm executor equals native on `comm` and W
ERS-005: When the R_SL executor of openvm halts a suite call, the R_SL executor SHALL give a `comm` equal to `native_comm` and effect words W equal, word for word, to `native_words`.

Verification: test `rsl_suite_exec_equal_native`, fields `words_equal_native` and `comm_equal_native` (`crates/sherlock-leaf/tests/rsl_exec.rs:55`); exists.

#### Scenario: Words differ
- **WHEN** W differs from `native_words` in one word
- **THEN** `rsl::prove_call` returns status `different` and makes no proof (`crates/sherlock-leaf/src/rsl/mod.rs:46-52`)

#### Scenario: Status of ERS-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: MEASURED 142 of 142 executed vectors on odin r5 (`LIST.md:30`); the 6 kit calls are OPEN (RSL-004, RSL-005, RSL-006). The meaning of `comm` and W is MYC-LOW-009, MYC-TRN-014, CF-003 and MYC-LOW-010; this requirement adds the check on the second implementation (`compile.rs:889-893`). References: CF-003, MYC-LOW-009, MYC-LOW-010, MYC-TRN-014, MYC-ACC-007.

### Requirement: ERS-006 An R_SL program has a `prog_d`
ERS-006: When the Sherlock planner admits an R_SL program, the Sherlock planner SHALL give the program a `prog_d` by the identity formula of MYC-CLM-023 over the image, the plan, `key_root` and the machine version.

Verification: test T-RSL-PROGD: (a) with image, plan, `key_root` and machine version fixed, a change of unrelated universal-key metadata keeps `prog_d`; (b) a change of one bound input changes `prog_d` and the anchor; (c) two distinct compiled images give two digests; to build.

#### Scenario: Two images
- **WHEN** two distinct compiled images are admitted under one universal key set
- **THEN** their `prog_d` values differ and the universal keys are byte-equal

#### Scenario: Status of ERS-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. `src/rsl` has no `prog_d` (`prog_d` appears in `crates/sherlock-lfr/src/iface.rs:285` only); GAP row of `research/sherlock-t23-args-20261115/MAP.md:35-41` (worktree `t23-args`, staged). MYC-KEY-003 and MYC-KEY-008 differ on whether a rekey changes `prog_d`; the test fixes the inputs and takes no side. Owner lane: rekey-chips with T25 (LFP-011). References: MYC-CLM-023, MYC-KEY-003, MYC-KEY-004, MYC-KEY-008, CPR-003.

### Requirement: ERS-007 `input_d` binds the tape
ERS-007: When the leaf proves an R_SL call, the leaf SHALL bind the `val_d` inside `input_d` to the value digest of the tape that the app consumed.

Verification: negative test T-RSL-INPUT (the tape is changed while the input salt, `fuel`, `stack_limit` and the resulting `input_d` are kept; the leaf proof is refused); to build.

#### Scenario: Swapped tape
- **WHEN** a prover replaces the tape after `input_d` is fixed
- **THEN** the leaf proof fails

#### Scenario: Status of ERS-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. No R_SL INIT exists (MAP.md:35-41 GAP). `input_d = P2(SHK_IN, salt, val_d, fuel_lo, fuel_hi, stack_limit)` with `val_d = vd(tape)` and `fuel`, `stack_limit` 0 is LEAD PICK 2 of MAP.md:70-75 (D-071). The 8-word salt of `input_d` and the salt word of the tape are two fields. Owner lane: rekey-chips with T25 (LFP-012). References: INIT-001 to INIT-008 (modular-leaf), ZK-013.

### Requirement: ERS-008 The outcome of a halted Compact call
ERS-008: When an R_SL call halts, the R_SL statement builder SHALL set the outcome kind to Halted and the outcome payload to the value digest of the list of `comm` followed by W.

Verification: `tools/t23-suite/accept_args.py` tests (16 pass) reproduce the T17 `ledger-args.json` byte for byte; exists for the form. The proof binding is ERS-012.

#### Scenario: Outcome of a counter call
- **WHEN** `t02` `increment` halts with `comm` and three effect words
- **THEN** the payload is `vd([comm, E_0, E_1, E_2])` and `outcome_d` is the tagged digest of kind Halted over it

#### Scenario: Status of ERS-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: form BUILT (MAP.md:39, :47-66, LEAD PICK 1): `outcome_d = P2(SHK_OUTCOME_V2, 2, kind, payload)` with payload `vd([comm, E_0 .. E_(L_e-1)])` as a Cons/Nil list of Fr values; proof binding OPEN (ERS-012). References: STL-001, SLC-007, D-071.

### Requirement: ERS-009 A natively refused Compact call has no statement
ERS-009: If the R_SL executor rejects a Compact call on the T23 settlement route, then the R_SL statement builder SHALL emit no `RslStatement` for the call.

Verification: test `rsl_rejected_tapes_refused` (`tests/rsl.rs:75`) and the suite rows refused-as-native; exists.

#### Scenario: Failed assert
- **WHEN** a Compact call fails an `assert` of the contract
- **THEN** the status is `refused`, and no proof and no `acceptRun` call exist for it

#### Scenario: Status of ERS-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (`mod.rs:43-45` returns status `refused`); LEAD PICK 4 of MAP.md: refused calls are not settled on this route (D-071). The sentence names the T23 settlement route, so GP-001 is not narrowed: the same program keeps a provable Rejected outcome under the general route (EUI-005, GP-001; FAL-EXEC-4, LEAD PICK for T15). References: MYC-LOW-007, MYC-REF-037, MYC-REF-038.

### Requirement: ERS-010 The record `RslStatement`
ERS-010: The R_SL statement builder SHALL write for each halted call one `RslStatement` record with exactly the fields `prog_d`, `input_d`, `outcome_d`, `comm` and `W`.

Verification: schema test T-RSL-STATEMENT (field names, types and lengths; no other field); to build.

#### Scenario: Settlement projection
- **WHEN** the argument builder maps an `RslStatement` and a deployment salt `r_P` to `acceptRun` arguments on the proving box
- **THEN** it derives `c_p`, `input_pub`, `outcome_pub` and `claim_pub`, and only those four words leave the box

#### Scenario: Status of ERS-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. Today the fields `comm` and `eff` of `mycroft-call/1` feed `accept_args.py` (MAP.md:39). The record is private to boxes the user owns (ERS-002): plain `prog_d` and the raw digests are not published (ZK-002, MYC-CLM-009). The external projection is `StatementPublics` (slice C, SLC-006). Consumers: slice B (leaf input, LFP), the argument builder of slice C on the proving box (HLP-003). References: `architecture/DDD.md` section 6.

### Requirement: ERS-011 The statement does not depend on the box
ERS-011: When two boxes execute the same call record, the R_SL executor SHALL give byte-equal public words on both boxes.

Verification: run `rsl_suite` on odin and on huginn and compare the canonical `comm` and W of each call on muninn (the result files hold timings, `mod.rs:70-76`, so whole-file hashes differ); to run.

#### Scenario: aarch64 and x86_64
- **WHEN** muninn and odin execute `d04` call 11
- **THEN** `comm` and W are byte-equal

#### Scenario: Status of ERS-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN: one box only has run (odin, 142 of 142 equal to native, `LIST.md:30`); huginn half queued (LOOP.md Finish plan). References: LPC-007, SPL-011.

### Requirement: ERS-012 The app statement is bound into the recursion statement
ERS-012: When an R_SL app proof enters recursion, the R_SL statement adapter SHALL bind the constrained `comm` and the ordered W of the PUBLIC rows to the outcome commitment that recursion consumes.

Verification: negative test T-RSL-STMT-BIND on real proofs: (a) an honest app trace with the outcome commitment of a second outcome is refused by the leaf or by N1; (b) on an entry with more than 54 effect words (a01 `test_all`, 81 words) a change of the last effect word is refused; to build.

#### Scenario: Forged outcome
- **WHEN** a prover keeps the honest trace and supplies the commitment of another `comm`
- **THEN** no accepted leaf proof exists

#### Scenario: Status of ERS-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. `air.rs:199-203` marks the binding as a GAP, and the FP-1 probe carries zeros (`air.rs:216-222`), so no recursion proof authenticates a Compact outcome today. FP-1 has 54 BabyBear words with assigned fields (`crates/sherlock-leaf/src/core.rs:43-44`), so the binding is a digest, not the raw words; slice B names the FP-1 field (LFP-010). Owner lane: rekey-chips, before T25. Found by the Astra-A draft; form corrected by Astra-A review R-02. References: MYC-CHIP-004, ERS-008, test `rsl_fp1_public_values_on_air0_only` (`tests/rsl.rs:171`).

### Requirement: ERS-013 One call record format
ERS-013: When the Mycroft compiler emits a call for the prover, the Mycroft compiler SHALL write the call in schema `rsl-call/1` with the program as an array of instruction objects.

Verification: round-trip test T-CALL-FORMAT (every call file that `mycroft-suite` emits loads in `compile::build`); to build.

#### Scenario: Suite emit
- **WHEN** `mycroft-suite` runs with `MYCROFT_EMIT_DIR` set
- **THEN** each file has `schema` equal to `rsl-call/1` and `rsl::prove_call` reads it

#### Scenario: Status of ERS-013
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. `mycroft-suite` writes `mycroft-call/1` with the program as a debug string (`mycroft/compiler/src/bin/mycroft-suite.rs:104-113`, worktree `mycroft-factory`, uncommitted); a second tool, `mycroft-emit` in the openvm tree, writes `rsl-call/1`. References: ERS-001, D-020.

### Requirement: ERS-014 A malformed call record is a named refusal
ERS-014: If a call record lacks a field or holds a field of a wrong type, then the R_SL compiler SHALL return `Refuse::Unsupported` with the field name.

Verification: negative parser test T-CALL-SCHEMA (a record with no `tape`, a non-decimal word, a `program` string); to build.

#### Scenario: Missing tape
- **WHEN** a record has no `tape` field
- **THEN** the status is `unsupported` with the reason "tape", and the process does not panic

#### Scenario: Status of ERS-014
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. `compile::build` converts JSON without checks (`crates/sherlock-leaf/src/rsl/compile.rs:589-625`), so a malformed record can panic. References: ERS-001, LW-021.

### Requirement: ERS-015 Per-program app keys sit under `key_root`
ERS-015: Where an R_SL program has a per-program app key, the Sherlock planner SHALL commit the app key under `key_root` inside `prog_d`.

Verification: test T-RSL-KEYROOT (a leaf proof over an app key outside `key_root` is refused, as path C refuses a child key outside `key_root`); to build.

#### Scenario: App key outside the root
- **WHEN** a prover uses an app key that `key_root` does not hold
- **THEN** the first node refuses the leaf proof

#### Scenario: Status of ERS-015
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN for R_SL; BUILT for chip sets 0 to 2 (LOOP.md rows T7, T7c: 25 path C cases). The clause "a preprocessing key per program" of MYC-REF-052 (`mycroft/spec/specs/refusals/spec.md:497-498`) conflicts with the per-program R_SL app key (`src/rsl/mod.rs:12-13`) and is superseded: universal means the node, `L`, `L_Fr` and wrap keys (D-004). References: CPR-003, MYC-KEY-006, ERS-006.
