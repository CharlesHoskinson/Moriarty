# Tasks: build order to the working prototype

Each task names the EARS IDs it closes and its LOOP row. The critical path is LOOP.md "Finish plan": T11 + FP-3 + T12-SLOT15 + T24 + the R_SL universal leaf key, then T25 (the single rekey), the T12 gate, T23 (chain and Preview settle), T15 (interface freeze). A task is done only when the `Verification:` check of each named requirement passes on the integration branch. This change ticks nothing: it builds nothing.

## 0. Gates before the single rekey (T25)

Lead ruling 2026-11-15: T25 does not start before the items of this section land. Each one changes a keyed constraint, and T25 makes the universal key set of the prototype in one pass from one list (RKY-001). One exception is recorded and ordered: the `L` collection form of row T8c is a second rekey after T25 and before T15 (D-049, section 0b). The era check is not an exception: it enters T25, or its fallback is selected first (D-078).

- [ ] 0.1 Lane rsl-bind (openvm branch `rsl-bind` from `rekey-int` 971bc67): bind `comm`, W, `prog_d`, `input_d` and `outcome_d` of an R_SL call into the recursion statement (EARS: ERS-006, ERS-007, ERS-008, ERS-012, ERS-015, LFP-010 to LFP-012, NAG-008, SLC-012; LOOP: T25; after: none)
- [ ] 0.2 Lane T22 era-check: add the era-check row to the rekey list, or record a failed premise A2 or A3 and select the ELD-007 fallback (EARS: EGV-005, EGV-009; LOOP: T22 tasks 2 and 3; after: none; D-078)
- [ ] 0.3 Give the R_SL key leaves their own index: `KEYDAG_IDX = 3` collides with the planned R_SL `key_root` id 3 (EARS: UKS-003; LOOP: T25; after: none)
- [ ] 0.4 Add the canonical ALU and Poseidon input range checks and their row in the rekey list (EARS: RSL-008, EKT-011, UKS-010; LOOP: T25; after: none)
- [ ] 0.5 Remove the F-3 schedule menu code from `sched.rs` and from the T16 (b) runner (EARS: STM-014; LOOP: T16; after: none; D-014)
- [ ] 0.6 Fix the CONFIRMED counterexample GP2-C2.1 with its regression test (EARS: NAG-007, SSG-003; LOOP: T10q, T13; after: none)
- [ ] 0.7 Complete the rekey list: rows for FP-3, slot 15, the range checks, the R_SL statement bindings, the R_SL key leaf index and the era-check words (EARS: UKS-010, RKY-001, RKY-002; LOOP: T25; after: 0.1 to 0.6)

## 0b. Row T8c (to add to LOOP.md)

LOOP.md names T8c in row T8b and has no row for it. Lead ruling 2026-11-15 (D-049): T25 runs first; the `L` collection form is a recorded second rekey before T15.

- [ ] 0b.1 Add row T8c to LOOP.md: `L` over SET (the collection form), the X8 in-circuit Compact verifier, and the MERGE-WORK width with STACKS and VALUE DIGEST MEASURED against 5,400 columns (EARS: GPH-004, GPH-005, GPH-008, NAG-005, BAT-009, BAT-012; LPC-002, LPC-003; LOOP: T8c; after: T25; D-012, D-049)

## 1. Slice A: language, execution, integration

- [ ] 1.1 Commit the Mycroft factory work (compiler, `factory/`, SUITE-COVERAGE) on `mycroft-factory` (EARS: ECC-002, ECC-010, ECC-011; LOOP: T18 (d); after: none)
- [ ] 1.2 Commit `SECP-INTERFACE.md` into the repository and write the Jubjub interface file (EARS: EKT-009, EKT-010; LOOP: T24; after: none)
- [ ] 1.3 Mycroft lowering of `keccak256` to `R_SL_KECCAK256`; amend MYC-BLT-007 (EARS: ECC-007; LOOP: T24; after: 1.1)
- [ ] 1.4 Mycroft lowering of secp256k1 values, committed with its tests (EARS: CF-013; MYC-FE-013 to MYC-FE-020; LOOP: T24; after: 1.1)
- [ ] 1.5 Mycroft Jubjub input types (both `j02` calls) (EARS: ECC-014; LOOP: T24; after: 1.2)
- [ ] 1.6 One call format: `mycroft-suite` writes `rsl-call/1`; schema checks in `compile::build` (EARS: ERS-013, ERS-014, ERS-001; LOOP: T23; after: 1.1)
- [ ] 1.7 Deploy profile with anchor slot 15 and the MYC-SEAL-021 refusal code (EARS: ECC-009; ANC-015; LOOP: T12; after: 1.1)
- [ ] 1.8 Tests T-CHIP-RULES and T-RULES-D (rule table closed, `rules_d` in the key set) (EARS: ECC-004, ECC-005, EKT-001; LOOP: T25; after: 1.3, 1.4)
- [ ] 1.9 Forged-row test for the Jubjub kit; MEASURED kit runs for keccak256, secp256k1, Jubjub on odin (EARS: EKT-006, EKT-007, EKT-003, EKT-011; LOOP: T24; after: 1.3, 1.4, 1.5)
- [ ] 1.10 `RslStatement` record and its schema test (EARS: ERS-010, ERS-008, ERS-009; LOOP: T23; after: 1.6)
- [ ] 1.11 Slice A part of lane rsl-bind (task 0.1): the statement form, the T-RSL-PROGD, T-RSL-INPUT and T-RSL-STMT-BIND cases, and the check that the builder output equals the bound statement (EARS: ERS-006, ERS-007, ERS-012, ERS-015; LOOP: T25; after: 1.10)
- [ ] 1.12 Suite runner: kit reasons from the machine version, the compared count, the call identity map (EARS: ECS-006, ECS-012, ECS-013, ECS-011, ECS-014; LOOP: T23; after: 1.3, 1.4, 1.5)
- [ ] 1.13 Execute all 208 calls on odin and huginn and compare result files on muninn (EARS: ERS-005, ERS-011, EUI-006, ECC-010; LOOP: T23; after: 1.9, 1.12)
- [ ] 1.14 Demonstration of both rule families under the T25 keys (EARS: EUI-001, EUI-002, EUI-005; LOOP: T25, T17 (c); after: T25)
- [ ] 1.15 T23 Part B live run and the final suite table (EARS: ECS-003 to ECS-005, ECS-009, ECS-010, EUF-006, EUF-010; LOOP: T23; after: T25, T12 gate, 1.11, 1.13)
- [ ] 1.16 First joined run of the whole flow, then the clean-box run (EARS: EUF-001, EUF-002, EUF-005, EUF-007 to EUF-009, EUF-011, EUF-012, EUF-014, ERS-002; LOOP: T17 (c); after: 1.15)
- [ ] 1.17 Present the suite table, the gate logs and the receipts for one `key_root` at the freeze (EARS: EUF-013; LOOP: T15; after: 1.16)
- [ ] 1.18 After the freeze: the `sherlock compile` command, the tier 3 kit, the sealed form, Stage 2, a path for mixed-hash programs (EARS: ECC-012, ECC-013, EKT-004; MYC-BLT-007; LOOP: none (after T15); after: T15)
- [ ] 1.19 The UIREQ retention map (each UIREQ ID retained, mapped or superseded) (EARS: EUI-009; LOOP: none (gap); after: none)

## 2. Slice B: proving

- [ ] 2.1 Commit the T11 files (`blind.rs`, `zkguard.rs`, `t11_zk.rs`, the `test-seed` features) and replace every constant `r_P` in `t7_chain` by `RunBlind` (EARS: ZKE-001, ZKE-005, ZKE-006, ZK-012 to ZK-014; LOOP: T11; after: none)
- [ ] 2.2 Run `eq_zk` and `slot_reuse` on the GPU and add the CPU leg and the socket-level check (EARS: ZKE-007, PDM-005, PDM-009, ZK-015 to ZK-017; LOOP: T11; after: T11 commit)
- [ ] 2.3 Commit FP-3 (`fp3.rs`, the `L` effect AIR, the lock link) with the before-fail and after-pass tests; add the refusal assert to `fp3_k_is_the_true_slice_count`; run a Carbon Quint lane (EARS: CLB-002, CLB-003, ANC-018, ANC-019; LOOP: T10q (FP-3); after: none)
- [ ] 2.4 Merge openvm `t12-slot15` (anchor slot 15 in `iface.rs`) with `t10q-ntfix`; add the three-constant cross-check (EARS: CLB-004, CLB-008, ANC-013; LOOP: T12; after: none)
- [ ] 2.5 Bind the R_SL statement: the outcome commitment in FP-1 (digest of `comm` and W), `prog_d` and `input_d` in the R_SL leaf; rerun the 6-or-9 bucket test and the rsl probe (EARS: LFP-010, LFP-011, LFP-012, ERS-006, ERS-007, ERS-012; LOOP: T23, T24; after: none)
- [ ] 2.6 Give the R_SL chip set 3 key leaves an index apart from `KEYDAG_IDX`; add the keygen test and the path C case with an R_SL program (EARS: UKS-003, NAG-004, LTR-007; LOOP: T25; after: statement bindings)
- [ ] 2.7 Run the rsl probe and the app proofs for all 142 suite calls (keccak, secp256k1 and jubjub kits included) on odin and huginn; add D3 on each R_SL app key; name the chip set 3 shape vector (EARS: LFP-002, LFP-003, LFP-005, SSG-001, RSL-001 to RSL-008; LOOP: T24, T23; after: statement bindings)
- [ ] 2.8 Fix GP2-C2.1 and remove the `#[ignore]` of `gp2_c21_pubv_checks_the_constant_row` (EARS: NAG-007, SSG-003; LOOP: T10q; after: none)
- [ ] 2.9 Remove the F-3 menu code (`sched::admitted`, `sched::check`) and move the part (b) runner to `check_free` (EARS: STM-014, LPC-006; LOOP: T16; after: none)
- [ ] 2.10 Add the missing rows to the rekey list (FP-3, slot 15, range checks, statement bindings, R_SL key leaf index, era-check words or a recorded A2 or A3 failure) and the row-name check (EARS: UKS-010, RKY-002, EGV-005; LOOP: T25; after: the commits above; T22 tasks 2 and 3)
- [ ] 2.11 Build the integration branch from `tested` with every lane of the list; add the ancestor check (EARS: UKS-009, CLB-005, CLB-006; LOOP: T25; after: rekey list complete)
- [ ] 2.12 Run the keygen once; write the universal key manifest with status `prototype`; check the `L` bounds (2,000 columns, 100 interactions) and the MERGE-WORK bound with a 5,401-column refusal check; check the interface word count (EARS: UKS-001, UKS-002, UKS-004, UKS-007, LTR-003, NAG-005, CLB-001, RKY-001; LOOP: T25; after: integration branch)
- [ ] 2.13 Rerun every gate on the new keys (check-cases, V.9, V.10, pathc, pathc-d3, gp2, x7, d3work, iface-fixture, lfr-e3, i7-decode, sg6, `quint_regressions`), with the GP case map and the dependency check (EARS: UKS-006, LFP-001, LFP-006, LFP-007, NAG-001 to NAG-004, NAG-006, NAG-008, NAG-011, LTR-001, LTR-002, LTR-005, GPH-002, GPH-003, GPH-005, GPH-007, SSG-002, RKY-005; LOOP: T25; after: keygen)
- [ ] 2.14 Move the keys to huginn and odin with the manifest; add the start-up manifest check in the prover and the daemon (EARS: UKS-005, PDM-006, BXF-007; LOOP: T25; after: gates pass)
- [ ] 2.15 Rerun setup mode on the three boxes; add the key-file hash check around setup and settle the in-memory `keys()` question (EARS: STM-001 to STM-008, UKS-008; LOOP: T16; after: key transfer)
- [ ] 2.16 Build the G3 legs (CPU prover, GPU prover, stark-backend verifier) and the GPU against CPU public-value test (EARS: SQG-014, GPV-002, LTR-005; LOOP: T12; after: T25)
- [ ] 2.17 Add the counterexample verdict count and the circuit change list with a verdict per row; attempt the 8 NOT ATTEMPTED findings (EARS: SSG-003, SSG-004, SSG-005, LPC-004, LPC-016; LOOP: T13; after: T25)
- [ ] 2.18 Run G1 to G5 and FP-8 on the T25 keys; write the gate record; set the manifest status (EARS: SSG-006, SSG-007, SQG-015, SQG-016, UKS-007; LOOP: T12; after: G3 legs, T13 verdicts)
- [ ] 2.19 Add the published records to the chain runner (`LeafProofsReady`, `ProofBundle`, `LfrProofReady` with `proof_sha256`, the `features` and `security` fields); no record when the stock verifier refuses (EARS: LFP-009, NAG-010, LTR-006, LTR-008, ZKE-002, SSG-007; LOOP: T23; after: T25)
- [ ] 2.20 Add the owned-box roster, the destination check for secrets and witnesses, the digest check and the muninn route to the box tooling; take `gpu.lock` and `heavy-slot` in the T23 and T25 runners (EARS: BXF-001 to BXF-004, BXF-007, BXF-014, ZKE-004; LOOP: T23, T16; after: none)
- [ ] 2.21 Prove the 142 R_SL calls through leaf, N1 to N3, `L` and `L_Fr` on odin and huginn with the box profile at `~/sherlock/setup/profile.json`; hand each `LfrProofReady` to the wrap (EARS: LFP-002, STM-016, ELA-002, ELA-007, STL-002; LOOP: T23; after: T12 gate, records)
- [ ] 2.22 Run the privacy scan on every settled call (EARS: ZKE-003; LOOP: T23, T17; after: first settled call)
- [ ] 2.23 T16 on the T25 keys: muninn `hw` run, muninn with huginn join, two run lengths, 1x to 8x run length, the wrong-order join case, the device per stage in the record (EARS: ELA-001, ELA-003 to ELA-007, LFP-008, NAG-009, GPH-006, LPC-006, LPC-007; LOOP: T16; after: setup reruns)
- [ ] 2.24 Add the scheduler tests asked by the review (probe minimum, VRAM-for-8 and RAM-for-2, no-fit refusal, one-byte-short early wrap, calibration fault) (EARS: STM-003, STM-005, STM-011, STM-012, STM-019; LOOP: T16; after: none)
- [ ] 2.25 Final timing on the T25 keys on huginn and odin; `L` and the leaf layer on the GPU; backend named in each record (EARS: GPV-001, GPV-004, GPV-005, GPV-007, PDM-001, PDM-004, LPC-005; LOOP: T14, T14b; after: T25)
- [ ] 2.26 Daemon checks: socket mode, unknown operation, path C through the daemon, `stats` for the GUI bridge with no secret (EARS: PDM-002, PDM-003, PDM-007, PDM-008; LOOP: T14b, T21; after: T25)
- [ ] 2.27 Add row T8c to LOOP.md; measure the complete MERGE-WORK kernel with STACKS and VALUE DIGEST against 5,400 columns (EARS: NAG-005, LPC-002, GPH-005; LOOP: T8c; after: T25)
- [ ] 2.28 Build `L` over a SET root (the collection form) and the X8 in-circuit Compact verifier with its class name; real-proof runs with refusal cases; record the second rekey (EARS: GPH-002, GPH-004, GPH-008, LPC-003, BAT-009, BAT-012; LOOP: T8c; after: T25)
- [ ] 2.29 Extend the interpreted-family leaf beyond 5 + 2 rules and give a program with both hash kits a proving route (EARS: GPH-001, LFP-003, EUI-010; LOOP: T6 (groups after V), T8c; after: T25)

## 3. Slice C: wrap, settlement, operations

- [ ] 3.1 Commit the staged `t23-args` builder and MAP.md; commit openvm `t12-slot15` (EARS: FRC-010, SLC-006, WRP-007; LOOP: T23, T12; after: none)
- [ ] 3.2 Add the k = 22 refusal test and the VK size guard with its test to the wrap tool (EARS: WRP-002, WRP-004; LOOP: T25; after: none)
- [ ] 3.3 Add the `cargo metadata` gate for the four ledger crates and a negative fixture for the ledger check (EARS: LVR-002, LVR-005; LOOP: T12; after: none)
- [ ] 3.4 Run the T22 premise test A1 to A3 on a ledger 9.1.0.1 devnet and record pass or fail (EARS: EGV-009; LOOP: T22; after: none)
- [ ] 3.5 Put the era-check row into the rekey list, or record the failed premise; add the check to the rekey runner (EARS: EGV-005, EGV-006; LOOP: T22, T25; after: premise test)
- [ ] 3.6 Move the anchor to slot 15 in `decode.rs` and in SherlockLedger; compile the T20 form with slot 15 (EARS: WRP-007, SLC-001, SLC-002; LOOP: T12, T25; after: T10q)
- [ ] 3.7 Merge `t20-batched` 5dc1be2ee into the integration branch and record ancestry and suites (EARS: SLC-001, FRC-015; LOOP: T20; after: T10q)
- [ ] 3.8 Build the sealed deploy tool with the readback gate (authority and every entry key; native keys on read entries) (EARS: EGV-001, EGV-002, SLC-003, SLC-004; LOOP: T22; after: T20 merge)
- [ ] 3.9 Change the runner default TTL to 3,600 s; pin `K_MAX` = 2 in `sherlock_plan` at every TTL, with tests (EARS: BAT-005, BAT-006, SCL-007; LOOP: T20; after: T20 merge)
- [ ] 3.10 Negative tests of the chain pin, the wallet pin, the scrub with marker secrets, the receipts allow-list, the mode check before the read (EARS: PVW-001, PVW-002, WAL-002, WAL-003, WAL-005, WAL-010; LOOP: T17; after: none)
- [ ] 3.11 Runner recovery: exclusive verdicts, an included control, the unknown inclusion state, artifact hash checks, fault tests (EARS: PVW-007, PVW-008, PVW-012, FRC-001 to FRC-006, FRC-013, FRC-014; LOOP: T17; after: none)
- [ ] 3.12 Qualify the wrap on the T25 keys: rows, VK bytes, LW-003 to LW-006 and LW-014 results, ledger check; write the gate record of the key (EARS: WRP-001, WRP-003, WRP-008 to WRP-012, LVR-001 to LVR-004; LOOP: T25, T12; after: T25 keygen)
- [ ] 3.13 Add the gate-record check to the deploy runner and the readiness gate to the settle runner (EARS: PVW-013, SLC-012; LOOP: T17, T23; after: wrap qualification)
- [ ] 3.14 T23 driver: box-private bundle, four public words back, result binding by sha256 and call vector, muninn check, per-call resume, privacy scan and times per call (EARS: HLP-001 to HLP-004, HLP-008, SLC-009, FRC-012, OBS-003, OBS-007; LOOP: T23; after: T25 keygen)
- [ ] 3.15 Add the huginn or odin prover hook to the Preview runner (EARS: PVW-010; LOOP: T17 (c); after: T25 keygen)
- [ ] 3.16 Deploy one sealed SherlockLedger for each suite program on Preview and record the genesis era (EARS: SLC-005, PVW-009, EGV-004, BAT-004; LOOP: T23, T17 (c); after: T12 gate)
- [ ] 3.17 Run T17 (c): the honest call and the four controls on the T25 keys, with a second program (EARS: PVW-005 to PVW-008, OBS-001, OBS-002, LVR-006; LOOP: T17 (c); after: deploy)
- [ ] 3.18 Settle the suite calls by `acceptRun` and write the suite table with deploy and transaction counts (EARS: SLC-010, SLC-011, BAT-003, BAT-004; LOOP: T23; after: T17 (c))
- [ ] 3.19 Case A configuration test (a host of another owner is refused) (EARS: HLP-005, HLP-007; LOOP: T11; after: T25 keygen)
- [ ] 3.20 Validium contract and its Preview run (EARS: PVW-011; LOOP: T19; after: T17 (b))
- [ ] 3.21 Prover GUI page: stages, source, per-box schedule, live and replay (EARS: OBS-012; LOOP: T21; after: T14b daemon)
- [ ] 3.22 Epoch operator: close rule, era-aware close and admission, membership data, leaf list; collection submit when the `L` collection form exists (EARS: BAT-001, BAT-002, BAT-007 to BAT-011; LOOP: T20, T8c; after: T8c)
- [ ] 3.23 SDK outside the factory: pins and box roster from user configuration; one end-to-end script (EARS: SCL-001, SCL-010; LOOP: T15; after: T23)
- [ ] 3.24 T15 freeze record: open governance parameters, collection settlement line, case B note, no push (EARS: EGV-008, BAT-012, HLP-006, FRC-011; LOOP: T15; after: all rows)
