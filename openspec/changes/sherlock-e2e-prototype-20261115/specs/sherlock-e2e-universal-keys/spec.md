## ADDED Requirements

### Requirement: UKS-001 One key set for every program
UKS-001: The Sherlock prover SHALL prove and verify every program of the Compact suite under one universal key set that has one internal-recursive key, one `L` key, one `L_Fr` key and one wrap key.

Verification: T23 suite table: one universal key manifest sha256 for all 208 calls; per program, `key_root` and the inclusion of its keys are checked apart (SPL-005); RSL-001 probe per call. Exists.

#### Scenario: Two suite programs
- **GIVEN** proofs of `t02 increment` and `d04 withdraw`
- **WHEN** both are settled
- **THEN** both records name the same universal key manifest sha256 and two different `key_root` values

#### Scenario: Status of UKS-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN until T25 and T23. Evidence: Moriarty 3dc9001a9 `research/sherlock-final-design-20261111/ARCHITECTURE.md:201` to `:204` (keys). Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:58`: `key_root` 0fade1a4, `keys_d` 15ed5153 on the T10m tree. LOOP row T25.

### Requirement: UKS-002 keys_d has 7 node rows
UKS-002: The key generator SHALL build `keys_d` from 7 node rows and the foreign rows of the universal key record.

Verification: gates `gp2` and `x7` (`t8_gp gp2`, `t8_gp x7`; Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:94`, Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:95`) on the rekeyed `keys_d`. Exists.

#### Scenario: Rows 4 to 6
- **GIVEN** a MERGE-WORK chain NW, NW2, N3 over NW2
- **WHEN** the node opens `keys_d`
- **THEN** rows 4, 5 and 6 hold the three keys

#### Scenario: Status of UKS-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on the T10m keys (LOOP T10m). Evidence: openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/stmt.rs:90` (`UROWS = 7`); openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:457` (7 node rows) and `:459` (2 foreign rows). Owner ruling 2026-11-14: `keys_d` with 7 rows (LOOP, "Owner rulings 2026-11-14").

### Requirement: UKS-003 The R_SL key leaves have their own index
UKS-003: When the rekey adds R_SL chip set 3 to `key_root`, the key generator SHALL give the R_SL key leaves an index that differs from the path C DAG leaf index `KEYDAG_IDX`.

Verification: NEW: a keygen test that builds `key_root` with chip sets 0 to 3 and the DAG leaf, and checks 5 distinct leaf positions; then gate `pathc` and the rsl probe on one `key_root`. To build.

#### Scenario: Path C with an R_SL program
- **GIVEN** a one-leaf R_SL call
- **WHEN** `L` opens the DAG leaf and the chip set 3 key leaf
- **THEN** the two openings use different leaf positions and both pass

#### Scenario: Status of UKS-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: found by this spec. Owner lane: rekey-chips, before T25. Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:363` sets `KEYDAG_IDX = 3`; the key-root leaves have set ids 0 to 2 (`t7_chain.rs:436` to `:460`). `~/.cache/rekey-t11/LIST.md:17` plans "key_root menu id 3" for R_SL. The two uses of 3 collide.

### Requirement: UKS-004 The rekey writes a key manifest
UKS-004: When the rekey completes, the rekey runner SHALL write a universal key manifest that holds the sha256 of every universal key file, `keys_d`, the sha256 of `~/.cache/rekey-t11/LIST.md` and the openvm, stark-backend and Moriarty commits of the keygen tree.

Verification: NEW: `sha256sum -c` of the manifest on muninn, huginn and odin after the key transfer. To build.

#### Scenario: A second keygen
- **GIVEN** a manifest and a later change to `~/.cache/rekey-t11/LIST.md`
- **WHEN** the manifest is checked
- **THEN** the sha256 of `~/.cache/rekey-t11/LIST.md` differs and the record shows that a second keygen is needed

#### Scenario: Status of UKS-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN until T25. Evidence: RKY-001 names the keygen record. LOOP T25 done-when: "keys generated once". Key files of the T10m set: `~/.cache/t10mm` (LOOP T10m). Per-program keys are not in this manifest: a new program changes `key_root` only (SPL-004). Slice C uses this manifest as `keyset_d` of an era (EGV-004, D-077) and for the wrap qualification record (WRP-012, PVW-013). Narrowed after Astra-B review R-07.

### Requirement: UKS-005 A box with other keys does not prove
UKS-005: If the key manifest of a box differs from the key manifest of the rekey, then the Sherlock prover on that box SHALL refuse to start a proof.

Verification: T16 join run, case `join_manifests_equal_this_process`; NEW: a start-up check in the prover daemon. Partly to build.

#### Scenario: huginn holds the T10m keys
- **GIVEN** huginn with the old key set after T25
- **WHEN** a leaf job arrives
- **THEN** the prover refuses the job and names both manifest digests

#### Scenario: Status of UKS-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL: the join check is BUILT (Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:66`); the start-up check is a gap. Owner lane: t16-elastic. Evidence: openvm `t16-elastic` 1baf605 `crates/sherlock-wrap-export/examples/common/t16.rs:771` (case `join_manifests_equal_this_process`); SC-014 and PF-006 of `SETUP-SPEC.md` (Moriarty `t16-elastic` a7a7b8867, `research/sherlock-t16-elastic-20261114/`).

### Requirement: UKS-006 Every gate reruns on the new keys
UKS-006: When the rekey completes, the rekey runner SHALL rerun the gates check-cases, V.9 omit, V.10 check-chain, pathc, pathc-d3, gp2, x7, d3work, iface-fixture, lfr-e3, i7-decode and sg6 on the new keys.

Verification: the T25 RESULTS table: one PASS line per gate with the log path. Exists.

#### Scenario: One gate fails
- **GIVEN** new keys and a failing `x7` case
- **WHEN** the runner records the table
- **THEN** the rekey is not DONE and no box receives the keys

#### Scenario: Status of UKS-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN until T25. Evidence: Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:84` to Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:99` list the gates; Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/run.sh:26` to `:41` and `research/sherlock-t10-bound-20261114/chain.sh:88` to `:150` run them. LOOP T25 done-when.

### Requirement: UKS-007 Prototype keys are marked
UKS-007: While the security gate of SSG-006 (G1 to G5 and FP-8) has not passed on a key set, the key manifest SHALL mark that key set as `prototype`.

Verification: NEW: the manifest field `status`; the Preview deploy script of PVW reads it. To build.

#### Scenario: Keys before the gate
- **GIVEN** T25 keys and an unfinished T12 gate
- **WHEN** a manifest is read
- **THEN** the status reads `prototype`

#### Scenario: Status of UKS-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP. Owner lane: T25. Evidence: LOOP T10: "prototype keys". LOOP loop rules: "The T12 key freeze is a prototype freeze". SQG-011.

### Requirement: UKS-008 Setup mode makes, changes and selects no key
UKS-008: The setup mode SHALL NOT make, change or select a key, a shape, `plan_d` or a value that enters a proof.

Verification: NEW: sha256 of each key file before and after `setup`, with a check for added and deleted files; a test that changes one key file during setup and expects the failure; the calibration proofs verify under the key files on disk (ST-005). To build.

#### Scenario: Setup on a new box
- **GIVEN** odin with the T25 keys
- **WHEN** the setup mode runs
- **THEN** each key file has the sha256 it had before and no key file is added

#### Scenario: Status of UKS-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: ST-007 has no named check, and the in-memory `keys()` call needs a ruling or a load from disk. Owner lane: t16-elastic. Evidence: ST-007 of `SETUP-SPEC.md` (Moriarty `t16-elastic` a7a7b8867, `research/sherlock-t16-elastic-20261114/`) (`SETUP-SPEC.md:15`), carried in full. Setup calls `keys()` to build the key set in memory for calibration (openvm `t16-elastic` 1baf605 `crates/sherlock-wrap-export/examples/common/t16.rs:672`): whether that is "making a key" under ST-007 is not ruled. Widened after Astra-B review R-16.

### Requirement: UKS-009 The keygen tree holds every lane of the list
UKS-009: If the keygen tree of the rekey is not a descendant of every commit that `~/.cache/rekey-t11/LIST.md` names, then the rekey runner SHALL refuse to start.

Verification: NEW: `git merge-base --is-ancestor <commit> HEAD` for each row of `~/.cache/rekey-t11/LIST.md` in the T25 runner; then the NT-2 and NT-3 tests of CLB-005 on that tree. To build.

#### Scenario: rekey-chips without the lock fix
- **GIVEN** a keygen tree at openvm `rekey-chips` dcf1b4c
- **WHEN** the runner checks the row of commit e3b407d
- **THEN** the check fails and no key is made

#### Scenario: Status of UKS-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP. Owner lane: T25 (integration branch from `tested`). Evidence: The lanes are separate trees today: openvm `t10q-ntfix` e3b407d (extended lock) is not an ancestor of openvm `rekey-chips` dcf1b4c (short lock at openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/src/iface.rs:289`) or of openvm `t12-slot15` 4ea3d05; openvm `fp3` d2abc38 is dirty; openvm `t11-zk` cce8c1d is dirty. LOOP T1 used `git merge-base --is-ancestor` for the same purpose. Found by the Astra-B draft (G-03).

### Requirement: UKS-010 The rekey list holds every key-changing row
UKS-010: If `~/.cache/rekey-t11/LIST.md` lacks a row for FP-3, the anchor slot 15, the canonical range checks, the R_SL statement bindings of LFP-010 to LFP-012, the R_SL key leaf index of UKS-003 or the era-check words, then the rekey runner SHALL refuse to start.

Verification: NEW: a row-name check in the T25 runner before keygen, beside the commit check of RKY-002. To build.

#### Scenario: The list of 2026-11-15
- **GIVEN** `~/.cache/rekey-t11/LIST.md` with no FP-3 row
- **WHEN** the rekey runner starts
- **THEN** the runner stops and names the missing rows

#### Scenario: Status of UKS-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: the list has all six rows; the runner check is TO BUILD. Owner lanes: fp3, t12-slot15, rekey-chips, t22-governance; the T11 lane owns the list. Evidence: `~/.cache/rekey-t11/LIST.md` of 2026-11-15 11:38 (sha256 5b35df53...) has FP-3 at `LIST.md:44` to `:46`, slot 15 at `:37`, the R_SL statement bindings (RSL-BIND) at `:51`, the era check at `:57`, the R_SL key leaf index at `:63` and the canonical range checks at `:64` (the earlier count "6 rows missing" is stale, F-B-KM-1). The era-check row is the integrator ruling of 2026-11-15 10:28 on EGV-005 (slice C, D-078, OWNER 2026-11-15): the era check enters the single rekey, or a recorded failure of premise A2 or A3 selects the fallback of ELD-007 (change on branch `t22-governance`) and takes the place of the row. RKY-002 requires a commit for each row that exists.
