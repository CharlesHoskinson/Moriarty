## ADDED Requirements

### Requirement: SLC-001 The prototype contract has four entries
SLC-001: The SherlockLedger contract SHALL export exactly 4 entries: `acceptRun`, `acceptCollection`, `rootAccepted` and `runAccepted`.

Verification: `t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/t20-ledger-test.out` (test "entries"); merge check LPC-010; exists in part.

#### Scenario: No openClaim entry
- **GIVEN** the compiled prototype contract
- **WHEN** its entries are listed
- **THEN** there is no `openClaim` and no `descend`

#### Scenario: Status of SLC-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: LDG-001 to LDG-005 as amended on `t20-batched` (sherlock-modular-leaf-20261111). Supersedes the T10 form with `openClaim` (`t17-preview@9e1e7f398:tools/sherlock-sdk/sherlock-ledger.compact:55-61`), which stays as `sherlock-ledger-t10.compact` for the T17 (a) record only. Evidence: `t20-batched@5dc1be2ee:tools/sherlock-sdk/sherlock-ledger.compact:54-76`. Check: `t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/t20-ledger-test.out` (test "entries"); merge check LPC-010. The form is on branch `t20-batched`, not merged (LPC-010), and has the anchor in field 0.

### Requirement: SLC-002 The anchor is sealed in slot 15
SLC-002: The SherlockLedger contract SHALL hold the anchor in top-level ledger slot 15 as a sealed field that only the constructor writes.

Verification: MISSING for the contract (no compiled slot-15 contract exists); ANC-014 after the T25 rekey; to build.

#### Scenario: A user-writable anchor cell is refused
- **GIVEN** a contract whose slot 15 is written by an entry
- **WHEN** the Mycroft compiler seals the contract
- **THEN** the compiler refuses the contract

#### Scenario: Status of SLC-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: ANC-014, ANC-015 (sherlock-completion-20261115); WRP-007 for the wrap side; the Mycroft refusal is slice A. Evidence: the anchor is field 0 today, `t20-batched@5dc1be2ee:tools/sherlock-sdk/sherlock-ledger.compact:35`; pick confirmed by the owner, llm-wiki `wiki/log.md:4098-4107`. Check: MISSING for the contract (no compiled slot-15 contract exists); ANC-014 after the T25 rekey.

### Requirement: SLC-003 Write entries hold the universal wrap key
SLC-003: When the deploy runner deploys a SherlockLedger, the deploy runner SHALL install the universal wrap verifying key on `acceptRun` and on `acceptCollection`.

Verification: deploy readback in T17 (a); STL-003 in row T23; exists in part.

#### Scenario: Installed keys are read back
- **GIVEN** a new deployment
- **WHEN** the indexer state is read
- **THEN** each write entry holds a key byte-equal to `--vk`

#### Scenario: Status of SLC-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: STL-003; narrows MID-002 to the write entries. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:17`, `RUNBOOK.md:28`. Check: deploy readback in T17 (a); STL-003 in row T23. Observed on the T10 three-entry form only.

### Requirement: SLC-004 Read entries hold native keys
SLC-004: When the deploy runner deploys a SherlockLedger, the deploy runner SHALL install the compiled native key of each read entry on `rootAccepted` and on `runAccepted`.

Verification: test `W-2 read entry key` in `t20-ledger-test.out` for the runtime rule. MISSING: a four-entry deploy path with per-entry key readback and one real consumer call; exists in part.

#### Scenario: A wrap-keyed read entry is refused by the runtime
- **GIVEN** a consumer that binds `rootAccepted`
- **WHEN** the deployed key differs from the compiled one
- **THEN** the runtime refuses to bind the callee

#### Scenario: Status of SLC-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: ELD-005 (governance). Supersedes MID-002 for read entries (owner-confirmed T20 pick "native-keyed reads", LOOP.md:106; decision D-073). Evidence: `t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:67`; the runtime test builds the state in memory (`tools/sherlock-sdk/t20-ledger.test.mjs`); the Preview runner of T17 installs the wrap key on every entry (`t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:646-649`). Check: test `W-2 read entry key` in `t20-ledger-test.out` for the runtime rule. MISSING: a four-entry deploy path with per-entry key readback and one real consumer call.

### Requirement: SLC-005 One ledger for each program
SLC-005: When the deploy runner deploys a ledger for an R_SL program, the deploy runner SHALL set the constructor anchor to the `c_p` of that program.

Verification: T17 (a) R4; per-program deploys in row T23 (OPEN: `prog_d` of a chip-set-3 program is not defined, MAP.md section 5); exists in part.

#### Scenario: Program B does not settle on the ledger of program A
- **GIVEN** the ledger of program A
- **WHEN** program B's arguments and wrap proof are submitted
- **THEN** the node refuses with custom error 115

#### Scenario: Status of SLC-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: N-11 (confirmed AFK pick, LOOP.md:87); ZK-012 for the fresh `r_P` of each deployment. Evidence: `L_Fr` requires `A_0 = C_P`, openvm `t10mm@5b7ada8:crates/sherlock-lfr/src/lverify.rs:1836-1878`; T17 R4, `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:25`; MAP.md section 1 and pick 3 (branch `t23-args`, staged, not committed). Check: T17 (a) R4; per-program deploys in row T23 (OPEN: `prog_d` of a chip-set-3 program is not defined, MAP.md section 5).

### Requirement: SLC-006 The argument builder runs on the private bundle
SLC-006: When the argument builder receives a `sherlock-accept-bundle/1` on the proving box, the argument builder SHALL return the 4 public words `c_p`, `input_pub`, `outcome_pub` and `claim_pub` as one `StatementPublics` value.

Verification: `tools/t23-suite/test_accept_args.py`, tests `T17Receipt.test_words` and `test_ledger_args_bytes` (the whole file: 16 pass, MAP.md:54); the builder is staged, not committed; exists in part.

#### Scenario: The T17 arguments are reproduced
- **GIVEN** the T17 (a) bundle
- **WHEN** `accept_args.py` runs
- **THEN** the runner file `ledger-args.json` (`c_p`, `input_pub`, `outcome_pub`, `anchor`) is byte-equal to the accepted file, and `acceptRun` takes the first three words

#### Scenario: Status of SLC-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: STL-001 (completion). `StatementPublics` is the published type of slice A (ERS-010); the bundle with the preimages is private and stays on the box (HLP-003, HLP-004). Evidence: `t23-args` (staged, not committed) `research/sherlock-t23-args-20261115/MAP.md:23,43,49-51`; contract side `t17-preview@9e1e7f398:tools/sherlock-sdk/sherlock-ledger.compact:41-45`. Check: `tools/t23-suite/test_accept_args.py`, tests `T17Receipt.test_words` and `test_ledger_args_bytes` (the whole file: 16 pass, MAP.md:54); the builder is staged, not committed.

### Requirement: SLC-007 Outcome of a Mycroft call
SLC-007: When the argument builder maps a Mycroft call that halts, the argument builder SHALL form the outcome as `Halted([comm, E_0 .. E_(L_e-1)])` with the effect words in call order.

Verification: tests `MycroftCall.*` and `test_negative_changed_outcome_word`. Proof binding is a GAP: the R_SL leaf statement carries zeros, openvm `rekey-chips@2b59f9b:crates/sherlock-leaf/src/rsl/air.rs:341-345`; exists in part.

#### Scenario: A changed effect word changes the claim
- **GIVEN** the call file of `t02-counter-example.increment`
- **WHEN** the last effect word changes
- **THEN** `outcome_pub` and `claim_pub` change and `c_p` and `input_pub` do not

#### Scenario: Status of SLC-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. Evidence: MAP.md:72 (pick 1) and MAP.md:66 (branch `t23-args`, staged). Check: tests `MycroftCall.*` and `test_negative_changed_outcome_word`. Proof binding is a GAP: the R_SL leaf statement carries zeros, openvm `rekey-chips@2b59f9b:crates/sherlock-leaf/src/rsl/air.rs:341-345`.

### Requirement: SLC-008 Input of a Mycroft call
SLC-008: When the argument builder maps a Mycroft call, the argument builder SHALL set `val_d` to the value digest of the call tape, with `fuel` 0, `stack_limit` 0 and the 8-word run salt that the prover supplies.

Verification: tests `MycroftCall.*` for the tape mapping with zero fuel and stack. MISSING: a production test of the prover's CSPRNG salt, and the proof binding (no R_SL INIT opens `input_d`, MAP.md:83); exists in part.

#### Scenario: Two private witnesses give two inputs
- **GIVEN** two calls with equal public inputs and different private tape words
- **WHEN** the builder runs
- **THEN** the two `input_pub` words differ

#### Scenario: Status of SLC-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: ZK-006, ZK-011 (salt and public digests); slice B (ZKE) owns the CSPRNG draw of the salt. Decision D-071. Evidence: MAP.md:73 (pick 2, branch `t23-args`, staged); the adapter takes the salt as an argument (`tools/t23-suite/accept_args.py`, `bundle_from_mycroft_call`). Check: tests `MycroftCall.*` for the tape mapping with zero fuel and stack. MISSING: a production test of the prover's CSPRNG salt, and the proof binding (no R_SL INIT opens `input_d`, MAP.md:83). `test_enc1` covers the digest form only.

### Requirement: SLC-009 Public words are cross-checked
SLC-009: If a recomputed public word differs from lane 10, 20, 34 or 39 of the reported call vector, then the argument builder SHALL raise `Mismatch` and write no argument file.

Verification: tests `test_call_vector_lanes` and `test_r1_vector_refused`; exists in part.

#### Scenario: The R1 vector is caught
- **GIVEN** the T17 R1 call vector with `outcome_pub + 1`
- **WHEN** the builder checks the lanes
- **THEN** `Mismatch` is raised

#### Scenario: Status of SLC-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. Evidence: MAP.md:49,63-64; lane map openvm `t10mm@5b7ada8:crates/sherlock-lfr/src/iface.rs:104,138-139`. Check: tests `test_call_vector_lanes` and `test_r1_vector_refused`. The builder is staged, not committed.

### Requirement: SLC-010 A natively refused call is not settled
SLC-010: If the native transcript refuses a suite call, then the settle runner SHALL send no `acceptRun` for that call.

Verification: T23 dry run (0 mismatch, 62 refused-as-native); live run OPEN; exists in part.

#### Scenario: Refused-as-native is counted, not settled
- **GIVEN** the 62 calls that native refuses
- **WHEN** the suite settles
- **THEN** the table counts 62 refused-as-native and holds no transaction for them

#### Scenario: Status of SLC-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: STL-005. Evidence: MAP.md:75 (pick 4); openvm `rekey-chips@2b59f9b:crates/sherlock-leaf/src/rsl/mod.rs:39-41`; dry run `t23-suite@daa1ca745`. Check: T23 dry run (0 mismatch, 62 refused-as-native); live run OPEN. The dry run is not a live pass; STL-004 to STL-006 keep the live checks.

### Requirement: SLC-011 Instance reuse rule
SLC-011: When the replay of a suite program starts on an instance whose state differs from the deploy-block state, the settle runner SHALL deploy a new instance for that program.

Verification: Part A plan of `parta.py` (exists). MISSING: a two-call sequence test and the same rule in the settle path; exists in part.

#### Scenario: A dependent call sequence keeps its state
- **GIVEN** a counter program with two calls in sequence on a fresh instance
- **WHEN** the second call runs
- **THEN** the runner keeps the instance, because the rule applies once at the start of the replay

#### Scenario: Status of SLC-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: Decision D-084. Evidence: LOOP.md:45 (LEAD PICK "instance reuse only when state equals deploy-block state"), confirmed by the owner, llm-wiki `wiki/log.md:4107`; `t23-suite@16836929b:tools/t23-suite/parta.py:48-60` decides reuse once per program from the probe field `stateEqualsDeployBlockState`. Check: Part A plan of `parta.py` (exists). MISSING: a two-call sequence test and the same rule in the settle path.

### Requirement: SLC-012 No live settle of an unbound statement
SLC-012: If the leaf statement of a call does not bind `prog_d`, `input_d` and `outcome_d`, then the settle runner SHALL refuse the live settle of the call.

Verification: MISSING (no gate in `t23.py`). GAP owner: row T23 driver, after the T25 rekey; to build.

#### Scenario: Today's R_SL probe is refused
- **GIVEN** an R_SL proof whose FP-1 public values carry zeros for the statement
- **WHEN** a live settle is asked for
- **THEN** the runner refuses and names the unbound parts

#### Scenario: Status of SLC-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: STL-001, STL-002 (completion); ERS-006 to ERS-008 and ERS-012 (slice A); slice B owns the proving side. Evidence: MAP.md:81-86,90 ("live settle stays off until the GAPs of section 5 close"); openvm `rekey-chips@2b59f9b:crates/sherlock-leaf/src/rsl/air.rs:341-345`. Check: MISSING (no gate in `t23.py`). GAP owner: row T23 driver, after the T25 rekey.
