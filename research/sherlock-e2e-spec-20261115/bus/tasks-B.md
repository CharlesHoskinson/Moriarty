# Slice B tasks in build order (for tasks.md)

Critical path: T11 + FP-3 + T12-SLOT15 + T24 + R_SL leaf key (with the statement bindings) -> T25 -> T12 gate -> T23 -> T15. Lines are in build order inside each phase.

## Before T25 (inputs of the single rekey)

- [ ] Commit the T11 files (`blind.rs`, `zkguard.rs`, `t11_zk.rs`, the `test-seed` features) and replace every constant `r_P` in `t7_chain` by `RunBlind` | EARS: ZKE-001, ZKE-005, ZKE-006, ZK-012 to ZK-014 | LOOP: T11 | after: none
- [ ] Run `eq_zk` and `slot_reuse` on the GPU and add the CPU leg and the socket-level check | EARS: ZKE-007, PDM-005, PDM-009, ZK-015 to ZK-017 | LOOP: T11 | after: T11 commit
- [ ] Commit FP-3 (`fp3.rs`, the `L` effect AIR, the lock link) with the before-fail and after-pass tests; add the refusal assert to `fp3_k_is_the_true_slice_count`; run a Carbon Quint lane | EARS: CLB-002, CLB-003, ANC-018, ANC-019 | LOOP: T10q (FP-3) | after: none
- [ ] Merge openvm `t12-slot15` (anchor slot 15 in `iface.rs`) with `t10q-ntfix`; add the three-constant cross-check | EARS: CLB-004, CLB-008, ANC-013 | LOOP: T12 | after: none
- [ ] Bind the R_SL statement: the outcome commitment in FP-1 (digest of `comm` and W), `prog_d` and `input_d` in the R_SL leaf; rerun the 6-or-9 bucket test and the rsl probe | EARS: LFP-010, LFP-011, LFP-012, ERS-006, ERS-007, ERS-012 | LOOP: T23, T24 | after: none
- [ ] Give the R_SL chip set 3 key leaves an index apart from `KEYDAG_IDX`; add the keygen test and the path C case with an R_SL program | EARS: UKS-003, NAG-004, LTR-007 | LOOP: T25 | after: statement bindings
- [ ] Run the rsl probe and the app proofs for all 142 suite calls (keccak, secp256k1 and jubjub kits included) on odin and huginn; add D3 on each R_SL app key; name the chip set 3 shape vector | EARS: LFP-002, LFP-003, LFP-005, SSG-001, RSL-001 to RSL-008 | LOOP: T24, T23 | after: statement bindings
- [ ] Fix GP2-C2.1 and remove the `#[ignore]` of `gp2_c21_pubv_checks_the_constant_row` | EARS: NAG-007, SSG-003 | LOOP: T10q | after: none
- [ ] Remove the F-3 menu code (`sched::admitted`, `sched::check`) and move the part (b) runner to `check_free` | EARS: STM-014, LPC-006 | LOOP: T16 | after: none
- [ ] Add the missing rows to the rekey list (FP-3, slot 15, range checks, statement bindings, R_SL key leaf index, era-check words or a recorded A2 or A3 failure) and the row-name check | EARS: UKS-010, RKY-002, EGV-005 | LOOP: T25 | after: the commits above; T22 tasks 2 and 3
- [ ] Build the integration branch from `tested` with every lane of the list; add the ancestor check | EARS: UKS-009, CLB-005, CLB-006 | LOOP: T25 | after: rekey list complete

## T25 (the single rekey)

- [ ] Run the keygen once; write the universal key manifest with status `prototype`; check the `L` bounds (2,000 columns, 100 interactions) and the MERGE-WORK bound with a 5,401-column refusal check; check the interface word count | EARS: UKS-001, UKS-002, UKS-004, UKS-007, LTR-003, NAG-005, CLB-001, RKY-001 | LOOP: T25 | after: integration branch
- [ ] Rerun every gate on the new keys (check-cases, V.9, V.10, pathc, pathc-d3, gp2, x7, d3work, iface-fixture, lfr-e3, i7-decode, sg6, `quint_regressions`), with the GP case map and the dependency check | EARS: UKS-006, LFP-001, LFP-006, LFP-007, NAG-001 to NAG-004, NAG-006, NAG-008, NAG-011, LTR-001, LTR-002, LTR-005, GPH-002, GPH-003, GPH-005, GPH-007, SSG-002, RKY-005 | LOOP: T25 | after: keygen
- [ ] Move the keys to huginn and odin with the manifest; add the start-up manifest check in the prover and the daemon | EARS: UKS-005, PDM-006, BXF-007 | LOOP: T25 | after: gates pass
- [ ] Rerun setup mode on the three boxes; add the key-file hash check around setup and settle the in-memory `keys()` question | EARS: STM-001 to STM-008, UKS-008 | LOOP: T16 | after: key transfer

## T12 gate

- [ ] Build the G3 legs (CPU prover, GPU prover, stark-backend verifier) and the GPU against CPU public-value test | EARS: SQG-014, GPV-002, LTR-005 | LOOP: T12 | after: T25
- [ ] Add the counterexample verdict count and the circuit change list with a verdict per row; attempt the 8 NOT ATTEMPTED findings | EARS: SSG-003, SSG-004, SSG-005, LPC-004, LPC-016 | LOOP: T13 | after: T25
- [ ] Run G1 to G5 and FP-8 on the T25 keys; write the gate record; set the manifest status | EARS: SSG-006, SSG-007, SQG-015, SQG-016, UKS-007 | LOOP: T12 | after: G3 legs, T13 verdicts

## T23 chain and the elastic runs

- [ ] Add the published records to the chain runner (`LeafProofsReady`, `ProofBundle`, `LfrProofReady` with `proof_sha256`, the `features` and `security` fields); no record when the stock verifier refuses | EARS: LFP-009, NAG-010, LTR-006, LTR-008, ZKE-002, SSG-007 | LOOP: T23 | after: T25
- [ ] Add the owned-box roster, the destination check for secrets and witnesses, the digest check and the muninn route to the box tooling; take `gpu.lock` and `heavy-slot` in the T23 and T25 runners | EARS: BXF-001 to BXF-004, BXF-007, BXF-014, ZKE-004 | LOOP: T23, T16 | after: none
- [ ] Prove the 142 R_SL calls through leaf, N1 to N3, `L` and `L_Fr` on odin and huginn with the box profile at `~/sherlock/setup/profile.json`; hand each `LfrProofReady` to the wrap | EARS: LFP-002, STM-016, ELA-002, ELA-007, STL-002 | LOOP: T23 | after: T12 gate, records
- [ ] Run the privacy scan on every settled call | EARS: ZKE-003 | LOOP: T23, T17 | after: first settled call
- [ ] T16 on the T25 keys: muninn `hw` run, muninn with huginn join, two run lengths, 1x to 8x run length, the wrong-order join case, the device per stage in the record | EARS: ELA-001, ELA-003 to ELA-007, LFP-008, NAG-009, GPH-006, LPC-006, LPC-007 | LOOP: T16 | after: setup reruns
- [ ] Add the scheduler tests asked by the review (probe minimum, VRAM-for-8 and RAM-for-2, no-fit refusal, one-byte-short early wrap, calibration fault) | EARS: STM-003, STM-005, STM-011, STM-012, STM-019 | LOOP: T16 | after: none
- [ ] Final timing on the T25 keys on huginn and odin; `L` and the leaf layer on the GPU; backend named in each record | EARS: GPV-001, GPV-004, GPV-005, GPV-007, PDM-001, PDM-004, LPC-005 | LOOP: T14, T14b | after: T25
- [ ] Daemon checks: socket mode, unknown operation, path C through the daemon, `stats` for the GUI bridge with no secret | EARS: PDM-002, PDM-003, PDM-007, PDM-008 | LOOP: T14b, T21 | after: T25

## T8c (no LOOP row; before T15)

- [ ] Add row T8c to LOOP.md; measure the complete MERGE-WORK kernel with STACKS and VALUE DIGEST against 5,400 columns | EARS: NAG-005, LPC-002, GPH-005 | LOOP: T8c | after: T25
- [ ] Build `L` over a SET root (the collection form) and the X8 in-circuit Compact verifier with its class name; real-proof runs with refusal cases; record the second rekey | EARS: GPH-002, GPH-004, GPH-008, LPC-003, BAT-009, BAT-012 | LOOP: T8c | after: T25
- [ ] Extend the interpreted-family leaf beyond 5 + 2 rules and give a program with both hash kits a proving route | EARS: GPH-001, LFP-003, EUI-010 | LOOP: T6 (groups after V), T8c | after: T25
