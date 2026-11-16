# Sherlock E2E Key Management — Independent Quint Formal Specification Report

- **Lane**: `e2e-key-management` (Gemini 4 Carbon)
- **Bounded Context**: Key Management (`sherlock-e2e-prototype-20261115`, `sherlock-completion-20261115`, adopted `SPL-005`, `SQG-011`)
- **Quint Artifacts**:
  - Model: `key_management.qnt` (`key_management`, `key_management_main`, `key_management_edge_min`, `key_management_edge_at_limit`, and 15 guard/binding mutants `key_management_m_*`)
  - Requirement tests: `key_management_test.qnt` (`16 passing` scenario `run` tests)
  - ITF replay traces: `../traces/key_management_main/t_0.itf.json` .. `t_19.itf.json` (20 traces, 15 steps max, seed `0x20261115`)

---

## 1. Requirement Coverage Table (ID Table)

Every own EARS ID (`UKS-001` .. `UKS-010`), completion EARS ID (`RKY-001`, `RKY-002`, `RKY-005`), and cited adopted EARS ID (`SPL-005`, `SQG-011`) is accounted for below.

| ID | Artifact (`inv_*`, action + test, consistency `val`) | Kind | Note |
|---|---|---|---|
| `UKS-001` | `inv_UKS_001`; actions `startAndRunKeygen`, `buildProgramKeyRoot`, `verifyAndSettleFullTree`, `verifyAndSettlePathC`; test `km_Uks001TwoSuiteProgramsOneKeysetTwoRootsTest`; mutant `key_management_m_uks001_split_keyset` | `MODELED` | Enforces one 4-key universal key set (`internalRecKeyId`, `lKeyId`, `lfrKeyId`, `wrapKeyId`) and one universal manifest `sha256` across settled Compact suite programs, with distinct per-program `key_root` digests (`t02 increment` vs `d04 withdraw`). See Findings `Q-key-management-1`, `Q-key-management-2`. |
| `UKS-002` | `inv_UKS_002`, `consistency_constants`; action `startAndRunKeygen`; test `km_Uks002SevenNodeRowsMergeWork4To6Test`; mutant `key_management_m_uks002_short_keys_d` | `MODELED` / `CONSTANT-CHECKED` | Constrains `keysD.nodeRowCount == NODE_ROWS_REQUIRED` (`7`), distinct MERGE-WORK keys in rows `4..6` (`NW`, `NW2`, `N3` over `NW2`), and `foreignRowCount >= 1` (`FOREIGN_ROWS_COUNT = 2` in main, `1` in `edge_min`). See Finding `Q-key-management-6`. |
| `UKS-003` | `inv_UKS_003`, `consistency_constants`; actions `buildProgramKeyRoot`, `refuseCollidingRslLeafIdx`, `verifyAndSettlePathC`; test `km_Uks003RslLeafDistinctAndPathCOpensTest`; mutant `key_management_m_uks003_rsl_idx_collision` | `MODELED` / `CONSTANT-CHECKED` | Constrains R_SL chip set 3 leaf index (`RSL_LEAF_IDX_CFG = 4`) to differ from `KEYDAG_IDX` (`3`) and chip sets `0..2` (5 pairwise-distinct leaf positions), and checks distinct leaf openings on Path C. See Finding `Q-key-management-3`. |
| `UKS-004` | `inv_UKS_004`; actions `startAndRunKeygen`, `appendRowAfterKeygen`; test `km_Uks004ManifestFieldsAndListChangeSecondKeygenTest`; mutant `key_management_m_uks004_manifest_unbinds` | `MODELED` | Binds the universal key manifest to all 4 universal key digests, `keys_d`, `LIST.md` `sha256`, and the 3 repo commits (`openvm`, `stark-backend`, `Moriarty`), forbids per-program keys in the manifest, and sets `secondKeygenNeeded` on `LIST.md` change. See Findings `Q-key-management-1`, `Q-key-management-5`. |
| `UKS-005` | `inv_UKS_005`; actions `startProofOnBox`, `refuseStartProofManifestMismatch`, `distributeKeysToBox`; test `km_Uks005HuginnOldManifestRefusedTest`; mutant `key_management_m_uks005_stale_box_proves` | `MODELED` | Forbids any fleet box (`muninn`, `huginn`, `odin`) whose `installedManifestSha` differs from `manifest.manifestSha256` from starting or running a proof, and records both digests on refusal. See Finding `Q-key-management-1`. |
| `UKS-006` | `inv_UKS_006`, `consistency_constants`; actions `runGatesAndD3Pass`, `runGatesFailX7`, `distributeKeysToBox`; test `km_Uks006FailingX7BlocksDoneAndBoxTransferTest`; mutant `key_management_m_uks006_skip_failed_gate` | `MODELED` / `CONSTANT-CHECKED` | Requires all `REQUIRED_GATE_COUNT = 12` regression gates to rerun and pass on the current `keygenEpoch` before `rekeyPhase == PhaseRekeyDone` or any box receives the new keys. See Finding `Q-key-management-5`. |
| `UKS-007` | `inv_UKS_007`, `consistency_constants`; actions `startAndRunKeygen`, `passSecurityGateAllChecks`; test `km_Uks007PrototypeUntilT12GatePassesTest`; mutant `key_management_m_uks007_premature_qualified` | `MODELED` / `CONSTANT-CHECKED` | Requires `manifest.statusMark == MarkPrototype` whenever the 6-check T12 security gate (`G1..G5` and `FP-8`, `SECURITY_GATE_CHECK_COUNT = 6`) has not passed on the active keygen epoch. |
| `UKS-008` | `inv_UKS_008`; actions `runSetupModeClean`, `abortSetupOnKeyTamper`; test `km_Uks008SetupModeChangesNoKeyOrPlanDTest`; mutant `key_management_m_uks008_setup_mutates_key` | `MODELED` | Verifies that completed `setup` mode on any box preserves `installedManifestSha`, `keyFileCount`, `selectedShape`, `planD`, and `proofEnteringValue` unchanged, and aborts on key tamper. See Finding `Q-key-management-6`. |
| `UKS-009` | `inv_UKS_009`; actions `checkoutNonDescendantTree`, `refuseRekeyNonDescendantTree`, `startAndRunKeygen`; test `km_Uks009NonDescendantTreeRefusedTest`; mutant `key_management_m_uks009_non_descendant_tree` | `MODELED` | Refuses to start keygen unless the keygen git tree is a descendant of every commit named in `LIST.md`. |
| `UKS-010` | `inv_UKS_010`, `consistency_constants`; actions `dropListRowBeforeRekey`, `refuseRekeyMissingRows`, `startAndRunKeygen`; test `km_Uks010MissingFp3RowRefusedTest`; mutant `key_management_m_uks010_missing_list_row` | `MODELED` / `CONSTANT-CHECKED` | Refuses to start keygen unless all `REQUIRED_LIST_ROW_COUNT = 6` key-changing rows (`FP-3`, slot 15, canonical range, R_SL statement bindings, R_SL key leaf index, era-check words) are in `LIST.md`, naming missing rows on refusal. See Finding `Q-key-management-4`. |
| `RKY-001` | `inv_RKY_001`; actions `startAndRunKeygen`, `appendRowAfterKeygen`; test `km_Rky001OneRekeyAndSecondRekeyAfterAppendTest`; mutant `key_management_m_rky001_stale_wrap_key` | `MODELED` | Binds regenerated `L`, `L_Fr`, and `wrap` keys to one `LIST.md` digest (`lastKeygenListSha`) per keygen and sets `secondKeygenNeeded` if a row is added after keygen. See Findings `Q-key-management-1`, `Q-key-management-2`. |
| `RKY-002` | `inv_RKY_002`; actions `markRowUncommitted`, `refuseRekeyUncommittedRow`, `startAndRunKeygen`; test `km_Rky002UncommittedSlot15RowRefusedTest`; mutant `key_management_m_rky002_uncommitted_row` | `MODELED` | Refuses to start keygen when any row in `LIST.md` (e.g., `RowAnchorSlot15` / `T12-SLOT15`) has no commit on its branch. |
| `RKY-005` | `inv_RKY_005`, `consistency_constants`; actions `runGatesAndD3Pass`, `runD3FailGrindable100Point9`; test `km_Rky005Grindable100Point9BitsFailsGateTest`; mutant `key_management_m_rky005_grindable_100_9` | `MODELED` / `CONSTANT-CHECKED` | Requires `d3MinTermDeciBits >= MIN_TERM_DECIBITS` (`1000` = `100.0` bits) and `d3MinGrindableDeciBits >= MIN_GRINDABLE_DECIBITS` (`1010` = `101.0` bits; `1009` = `100.9` bits fails), alongside wrap bounds `MAX_WRAP_K = 21` and `MAX_WRAP_VK_BYTES = 50000`. See Finding `Q-key-management-5`. |
| `SPL-005` *(adopted)* | `inv_SPL_005`; actions `verifyAndSettleFullTree`, `verifyAndSettlePathC`, `refuseUnlistedChildKey`; test `km_Spl005FullTreeAndPathCInclusionAndRefusalTest`; mutant `key_management_m_spl005_unlisted_child_key` | `MODELED` | Adopted from `sherlock-shard-plan`: checks child key commitment inclusion into `key_root` on full-tree settlement and dual leaf openings (`rslLeafIdx` + `KEYDAG_IDX`) plus public `X` binding on Path C, refusing unlisted child keys. |
| `SQG-011` *(adopted)* | `inv_SQG_011`; actions `freezeUniversalKeys`, `refuseFreezeBeforeGate`, `passSecurityGateAllChecks`; test `km_Sqg011RefuseFreezeBeforeGateAllowAfterTest`; mutant `key_management_m_sqg011_freeze_before_gate` | `MODELED` | Adopted from `sherlock-security-gate`: forbids `universalKeysFrozen` until `G1..G4` pass with `G5` and `FP-8` closes (`SECURITY_GATE_CHECK_COUNT = 6`), refusing freeze requests before the gate. |

---

## 2. Findings (`Q-key-management-1` .. `Q-key-management-6`)

### `Q-key-management-1` — Unspecified epoch boundary across a second keygen leaves in-flight jobs and settled suite proofs straddling two manifests
- **IDs**: `UKS-001`, `UKS-004`, `UKS-005`, `RKY-001` (`SQG-011`)
- **Quoted sentences**:
  - `UKS-001`: *"The Sherlock prover SHALL prove and verify every program of the Compact suite under one universal key set that has one internal-recursive key, one `L` key, one `L_Fr` key and one wrap key."* / Scenario: *"GIVEN proofs of `t02 increment` and `d04 withdraw` WHEN both are settled THEN both records name the same universal key manifest sha256 and two different `key_root` values"*
  - `UKS-004` Scenario (*A second keygen*): *"GIVEN a manifest and a later change to `~/.cache/rekey-t11/LIST.md` WHEN the manifest is checked THEN the sha256 of `~/.cache/rekey-t11/LIST.md` differs and the record shows that a second keygen is needed"*
  - `RKY-001` Scenario: *"WHEN a row is added after the keygen THEN a second keygen is needed and the record shows it"*
  - `UKS-005`: *"If the key manifest of a box differs from the key manifest of the rekey, then the Sherlock prover on that box SHALL refuse to start a proof."*
- **Witness trace / state**:
  - Found directly by `quint run` (seed `0x20275da9`) before adding explicit cross-epoch drain/reset guards:
    1. `init` -> `startAndRunKeygen(7)` (epoch 1 writes manifest `1848`) -> `runGatesAndD3Pass` -> `distributeKeysToBox(BoxOdin)` -> `buildProgramKeyRoot(ProgT02Increment, 4)` -> `startProofOnBox(BoxOdin, ProgT02Increment)` (`BoxOdin.proofRunning = true` under manifest `1848`; or `ProgT02Increment` settles under `1848`).
    2. `appendRowAfterKeygen` (`RowPostRekeyExtra` added to `LIST.md`, `secondKeygenNeeded = true`) -> `startAndRunKeygen(7)` (epoch 2 writes manifest `2922`).
    3. At this state, `BoxOdin.proofRunning == true` while `BoxOdin.installedManifestSha == 1848 != 2922` (`UKS-005` only forbade *starting* a proof, not draining/aborting already-running jobs when the active rekey manifest rotates), and any proof settled in epoch 1 (`usedManifestSha == 1848`) coexists with a proof settled in epoch 2 (`usedManifestSha == 2922`), violating `UKS-001`. Furthermore, neither `RKY-001` nor `UKS-004` states that a key set frozen under `SQG-011` cannot be overwritten in place by a second keygen without opening a new governance era (`keyset_d`).
- **What would close it**:
  - Specify in `UKS-004` / `UKS-005` / `RKY-001` that (1) a second keygen creates a distinct keyset epoch (`keyset_d`) rather than mutating a frozen key set in place, (2) in-flight jobs under an older manifest are drained or aborted before switching the active rekey manifest, and (3) `UKS-001`'s suite settlement invariant ("same universal key manifest sha256") is scoped to a single manifest / governance era, invalidating prior-epoch suite settlement records when a second keygen supersedes an un-frozen prototype key set.

### `Q-key-management-2` — `RKY-001` omits the internal-recursive key and `keys_d` from the keys regenerated during rekey
- **IDs**: `RKY-001`, `UKS-001`, `UKS-002`, `UKS-004`, `UKS-010`
- **Quoted sentences**:
  - `RKY-001`: *"When the rekey runs, the rekey runner SHALL regenerate the `L`, `L_Fr` and wrap keys once from every row of `~/.cache/rekey-t11/LIST.md`."*
  - `UKS-001`: *"The Sherlock prover SHALL prove and verify every program of the Compact suite under one universal key set that has one internal-recursive key, one `L` key, one `L_Fr` key and one wrap key."*
  - `UKS-002`: *"The key generator SHALL build `keys_d` from 7 node rows and the foreign rows of the universal key record."*
  - `UKS-004`: *"When the rekey completes, the rekey runner SHALL write a universal key manifest that holds the sha256 of every universal key file, `keys_d`, ... "*
- **Witness state**:
  - Under `UKS-010`, `~/.cache/rekey-t11/LIST.md` carries key-changing rows (`RowRslStmtBind` for `LFP-010..LFP-012`, `RowRslKeyLeafIdx` for `UKS-003`, `RowFp3`) that alter leaf/node recursion statements and `keys_d`. If a runner implements `RKY-001` literally and regenerates only `{L, L_Fr, wrap}` while leaving the internal-recursive key (`UKS-001`) and `keys_d` (`UKS-002`) from `T10m`, the manifest (`UKS-004`) mixes `T10m` internal-recursive/`keys_d` artifacts with `T25` `L`/`L_Fr`/`wrap` keys and fails node/bridge verification. In addition, `UKS-001` says *"one internal-recursive key"* whereas `UKS-002` names 7 node rows (including three distinct MERGE-WORK keys in rows 4, 5, and 6).
- **What would close it**:
  - Amend `RKY-001` to state that the rekey runner regenerates the complete universal key set (internal-recursive node key(s), `keys_d`, `L`, `L_Fr`, and `wrap`) from `LIST.md`, and clarify in `UKS-001` / `UKS-002` how "one internal-recursive key" relates to the 7 node program rows committed in `keys_d`.

### `Q-key-management-3` — `UKS-003` EARS text permits R_SL leaf collision with chip sets `0..2` and attributes per-program `key_root` construction to the single rekey
- **IDs**: `UKS-003`, `UKS-001`, `UKS-004`
- **Quoted sentences**:
  - `UKS-003`: *"When the rekey adds R_SL chip set 3 to `key_root`, the key generator SHALL give the R_SL key leaves an index that differs from the path C DAG leaf index `KEYDAG_IDX`."*
  - `UKS-003` Verification (non-normative): *"NEW: a keygen test that builds `key_root` with chip sets 0 to 3 and the DAG leaf, and checks 5 distinct leaf positions..."*
  - `UKS-001` Scenario: *"THEN both records name the same universal key manifest sha256 and two different `key_root` values"*
  - `UKS-004` Status: *"Per-program keys are not in this manifest: a new program changes `key_root` only (SPL-004)."*
- **Witness state**:
  - Take `chip0Idx = 0, chip1Idx = 1, chip2Idx = 2, KEYDAG_IDX = 3`, and choose `rslLeafIdx = 2`. This state satisfies the literal EARS requirement of `UKS-003` (`rslLeafIdx = 2 != 3 = KEYDAG_IDX`), yet collides with chip set `2` (`Set(0, 1, 2, 2, 3).size() == 4 < 5`), violating the 5-distinct-leaf property assumed by Verification prose. Moreover, `UKS-003`'s trigger *"When the rekey adds R_SL chip set 3 to `key_root`"* treats `key_root` as an artifact built once during the universal rekey, whereas `UKS-001` and `UKS-004` establish that `key_root` is built *per program* outside the universal rekey manifest.
- **What would close it**:
  - Rewrite `UKS-003`'s normative EARS sentence to: *"When per-program key generation builds `key_root` with chip sets `0` to `3` and the path C DAG leaf `KEYDAG_IDX`, the key generator SHALL assign pairwise-distinct leaf indices to chip sets `0` to `3` and `KEYDAG_IDX` (5 distinct leaf positions), and the universal `L`/node verifiers SHALL open R_SL chip set 3 at that distinct index."*

### `Q-key-management-4` — `UKS-010` unconditional EARS refusal contradicts the `ELD-007` governance fallback documented in its own rationale and `DDD.md`
- **IDs**: `UKS-010`
- **Quoted sentences**:
  - `UKS-010`: *"If `~/.cache/rekey-t11/LIST.md` lacks a row for FP-3, the anchor slot 15, the canonical range checks, the R_SL statement bindings of LFP-010 to LFP-012, the R_SL key leaf index of UKS-003 or the era-check words, then the rekey runner SHALL refuse to start."*
  - `UKS-010` Status (`spec.md:141`): *"the era check enters the single rekey, or a recorded failure of premise A2 or A3 selects the fallback of ELD-007 (change on branch `t22-governance`) and takes the place of the row."*
- **Witness state**:
  - Suppose premise `A2` or `A3` records a failure and selects the `ELD-007` fallback *"in place of"* `RowEraCheck` (`currentListRows = {RowFp3, RowAnchorSlot15, RowCanonicalRange, RowRslStmtBind, RowRslKeyLeafIdx}`). Under `UKS-010`'s normative EARS sentence (`missingMandatoryRows(km) == Set(RowEraCheck)`), the rekey runner MUST refuse to start (`RefusalMissingListRows(Set(RowEraCheck))`), blocking the documented fallback path.
- **What would close it**:
  - Amend `UKS-010`'s EARS condition from `"... or the era-check words"` to `"... or (the era-check words, unless a recorded failure of premise A2 or A3 substitutes an ELD-007 fallback row)"`.

### `Q-key-management-5` — `"When the rekey completes"` conflates keygen completion with post-gate rekey `DONE` across `UKS-004`, `UKS-006`, and `RKY-005`
- **IDs**: `UKS-004`, `UKS-006`, `RKY-005`
- **Quoted sentences**:
  - `UKS-004`: *"When the rekey completes, the rekey runner SHALL write a universal key manifest..."*
  - `UKS-006`: *"When the rekey completes, the rekey runner SHALL rerun the gates ... on the new keys."* / Scenario: *"GIVEN new keys and a failing `x7` case WHEN the runner records the table THEN the rekey is not DONE and no box receives the keys"*
  - `RKY-005`: *"When the rekey completes, the D3 calculator SHALL give every term at least 100 bits and every grindable term at least 101 bits on each new key."* / Scenario: *"WHEN a term is 100.9 bits and grindable THEN the gate fails"*
- **Witness state**:
  - All three requirements use the identical event trigger *"When the rekey completes"*, yet `UKS-006`'s scenario explicitly states that after this trigger runs and `x7` fails, *"the rekey is not DONE"*. In the model (`PhaseKeygenComplete` vs `PhaseRekeyGateFailed` vs `PhaseRekeyDone`), writing the manifest at `PhaseKeygenComplete` (`UKS-004`) followed by `runGatesFailX7` (`UKS-006`) or `runD3FailGrindable100Point9` (`RKY-005`) leaves a written `prototype` manifest on the keygen host whose keys failed qualification (`PhaseRekeyGateFailed`). The spec does not state whether `UKS-004`'s manifest is written only after `UKS-006` + `RKY-005` pass (`DONE`), or written at keygen completion and marked `failed`/quarantined if any gate in `UKS-006` or `RKY-005` fails.
- **What would close it**:
  - Distinguish **keygen completion** from **rekey `DONE`** in `UKS-004`, `UKS-006`, and `RKY-005`: trigger `UKS-006` and `RKY-005` *"When key generation completes"*, and either write the distributable manifest (`UKS-004`) only when all 12 gates and D3 pass (`rekey DONE`) or record a disqualifying `gate_failed` status in the manifest so no tool mistakes a failed keygen's manifest for a valid `prototype` manifest.

### `Q-key-management-6` — Unspecified foreign-row cardinality in `UKS-002` and unruled in-memory `keys()` reconstruction in `UKS-008`
- **IDs**: `UKS-002`, `UKS-008`
- **Quoted sentences**:
  - `UKS-002`: *"The key generator SHALL build `keys_d` from 7 node rows and the foreign rows of the universal key record."*
  - `UKS-008`: *"The setup mode SHALL NOT make, change or select a key, a shape, `plan_d` or a value that enters a proof."*
  - `UKS-008` Status (`spec.md:113`): *"Setup calls `keys()` to build the key set in memory for calibration ... whether that is 'making a key' under ST-007 is not ruled."*
- **Witness state**:
  - For `UKS-002`, `key_management_main` (`FOREIGN_ROWS_COUNT = 2`) and `key_management_edge_min` (`FOREIGN_ROWS_COUNT = 1`) both satisfy the EARS text because *"the foreign rows"* has no normative count or schema in `UKS-002` (only Status evidence mentions `"2 foreign rows"`). For `UKS-008`, `runSetupModeClean` preserves all on-disk key digests (`installedManifestSha`, `keyFileCount`), `selectedShape`, `planD`, and `proofEnteringValue`, which satisfies `UKS-008`'s scenario (*"each key file has the sha256 it had before and no key file is added"*), whereas an interpretation under which calling `keys()` in RAM during setup counts as *"making a key"* would make current `setup` violate `UKS-008`.
- **What would close it**:
  - State the exact foreign-row count and identities (e.g., the 2 foreign rows `L` and `L_Fr` or bridge foreign rows) normatively in `UKS-002`, and rule explicitly in `UKS-008` either that `setup` MUST deserialize existing key files from disk without calling `keys()`, or that deterministic in-memory loading/reconstruction verified against the on-disk manifest `sha256` is permitted.

---

## 3. Mutant Table

Each of the 15 invariants in `val inv` has a dedicated mutant module `key_management_m_<name>` (`MUTANT_ID = 1..15`) that weakens or removes its corresponding guard or binding. Under `quint run` (`--max-samples=1000 --max-steps=15 --seed=0x20261115`), every mutant exited `1` (`[violation]`) on its target invariant, AND every mutant also violated `reach_rekey_done_and_distributed` (`exit 1`), confirming that the honest rekey/distribution path remains live and no mutant failure is vacuous.

| Mutant Module | `MUTANT_ID` | Guard / Binding Removed or Weakened | Target Invariant | Observed Exit Code | Non-Vacuity (`reach_rekey_done_and_distributed`) |
|---|---:|---|---|---:|---:|
| `key_management_m_uks001_split_keyset` | `1` | Allows settling a suite program under a foreign universal key manifest (`manifestSha256 + 77`) | `inv_UKS_001` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_uks002_short_keys_d` | `2` | Allows `startAndRunKeygen` to build `keys_d` with `4` node rows instead of `7` | `inv_UKS_002` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_uks003_rsl_idx_collision` | `3` | Removes `fiveLeafIndicesDistinct` guard in `buildProgramKeyRoot`, allowing `rslLeafIdx == KEYDAG_IDX` (`3`) | `inv_UKS_003` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_uks004_manifest_unbinds` | `14` | Writes manifest with `openvmCommit = 0` and `holdsPerProgramKey = true` | `inv_UKS_004` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_uks005_stale_box_proves` | `4` | Removes `installedManifestSha == manifest.manifestSha256` guard in `startProofOnBox` | `inv_UKS_005` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_uks006_skip_failed_gate` | `5` | Transitions `runGatesFailX7` to `PhaseRekeyDone` despite `gatesPassedCount == 11 < 12` | `inv_UKS_006` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_uks007_premature_qualified` | `6` | Enables `markManifestQualifiedPrematurely` before `G1..G5 + FP-8` pass | `inv_UKS_007` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_uks008_setup_mutates_key` | `7` | Mutates `keyFileCount` and `planD` inside `runSetupModeClean` | `inv_UKS_008` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_uks009_non_descendant_tree` | `8` | Removes `treeDescendantOfAllRows` ancestor check in `canStartKeygen` | `inv_UKS_009` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_uks010_missing_list_row` | `9` | Removes `missingMandatoryRows(s).size() == 0` check in `canStartKeygen` | `inv_UKS_010` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_rky001_stale_wrap_key` | `15` | Leaves `wrapKeyListSha` bound to old `T10m` list (`100`) instead of `currentListSha` during keygen | `inv_RKY_001` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_rky002_uncommitted_row` | `10` | Removes `uncommittedRows` branch-commit check in `canStartKeygen` | `inv_RKY_002` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_rky005_grindable_100_9` | `11` | Accepts `100.9` grindable bits (`1009 >= 1000` instead of `>= 1010`) as `PhaseRekeyDone` | `inv_RKY_005` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_spl005_unlisted_child_key` | `12` | Skips `isChildKeyListedInRoot` check in `verifyAndSettleFullTree`, accepting unlisted key `9999` | `inv_SPL_005` | `1` (`[violation]`) | `1` (live) |
| `key_management_m_sqg011_freeze_before_gate` | `13` | Allows `freezeUniversalKeys` as soon as `manifest.written` before `G1..G5 + FP-8` pass | `inv_SQG_011` | `1` (`[violation]`) | `1` (live) |

---

## 4. State Vocabulary for Replay

### 4.1 Top-Level State Variables (`var`)
In ITF JSON files (`../traces/key_management_main/t_{seq}.itf.json`), Quint qualifies imported module variables under `key_management_main::key_management::<var>`:

| Quint `var` | ITF JSON Key | Quint Type | Meaning |
|---|---|---|---|
| `km` | `key_management_main::key_management::km` | `KeyMgmtState` | Full observable state record of the Key Management bounded context. |
| `lastOp` | `key_management_main::key_management::lastOp` | `str` | Name of the action that produced the state (`"init"` at index `0`, action identifier on every step). |

#### Fields of `km: KeyMgmtState`
- **Rekey list & git tree state**:
  - `currentListRows: Set[ListRowKind]` — Rows present in `~/.cache/rekey-t11/LIST.md` (`RowFp3`, `RowAnchorSlot15`, `RowCanonicalRange`, `RowRslStmtBind`, `RowRslKeyLeafIdx`, `RowEraCheck`, and post-keygen `RowPostRekeyExtra`).
  - `uncommittedRows: Set[ListRowKind]` — Subset of rows lacking a branch commit (`RKY-002`).
  - `treeDescendantOfAllRows: bool` — Whether `HEAD` of the keygen tree is a descendant of every commit named in `LIST.md` (`UKS-009`).
  - `currentListSha: int` — Deterministic abstract `sha256` of `currentListRows` (`200 + sum(rowWeight)`).
  - `treeOpenvmCommit: int`, `treeStarkBackendCommit: int`, `treeMoriartyCommit: int` — Abstract git commit IDs of the 3 repos in the keygen tree (`11`, `22`, `33`).
- **Keygen snapshot & phase**:
  - `keygenEpoch: int` — Number of completed keygen runs (`0` initially, `1` after T25 rekey, `2` after second keygen).
  - `rekeyPhase: RekeyPhase` — `PhaseRekeyIdle | PhaseKeygenComplete | PhaseRekeyGateFailed | PhaseRekeyDone`.
  - `lastKeygenListRows: Set[ListRowKind]`, `lastKeygenListSha: int` — Snapshot of `LIST.md` at the latest keygen (`RKY-001`, `UKS-004`, `UKS-010`).
  - `lastKeygenAllRowsCommitted: bool`, `lastKeygenTreeWasDescendant: bool` — Snapshot of `RKY-002` and `UKS-009` preconditions at keygen start.
  - `secondKeygenNeeded: bool` — Flag set when `LIST.md` diverges from the active manifest's `listSha256` (`UKS-004`, `RKY-001`).
- **Universal key artifacts**:
  - `universalKeySet: UniversalKeySetRec` — `{ epoch, internalRecKeyId, lKeyId, lfrKeyId, wrapKeyId, lKeyListSha, lfrKeyListSha, wrapKeyListSha, wrapLogK, wrapVkBytes }`.
  - `keysD: KeysDRec` — `{ built, nodeRowCount, row4NwKey, row5Nw2Key, row6N3Nw2Key, foreignRowCount, keysDDigest }` (`UKS-002`).
  - `manifest: ManifestRec` — `{ written, manifestSha256, internalRecSha, lKeySha, lfrKeySha, wrapKeySha, keysDDigest, listSha256, openvmCommit, starkBackendCommit, moriartyCommit, statusMark (MarkUnwritten | MarkPrototype | MarkQualified), holdsPerProgramKey }` (`UKS-004`, `UKS-007`).
- **Post-keygen & T12 security gates**:
  - `gateRunKeygenEpoch: int`, `gatesRerunCount: int`, `gatesPassedCount: int` — 12-gate rerun tracking (`UKS-006`).
  - `d3CheckedEpoch: int`, `d3MinTermDeciBits: int`, `d3MinGrindableDeciBits: int` — D3 security margin in tenths of a bit (`RKY-005`).
  - `secGateKeygenEpoch: int`, `secGatePassedChecks: Set[int]`, `universalKeysFrozen: bool` — T12 security gate (`G1..G5 + FP-8`, `1..6`) and key freeze flag (`UKS-007`, `SQG-011`).
- **Per-program `key_root`, fleet boxes, settled proofs, and refusals**:
  - `programKeyRoots: ProgramId -> ProgramKeyRootRec` — Per-program (`ProgT02Increment`, `ProgD04Withdraw`) Merkle root digest, leaf indices (`chip0Idx..chip2Idx`, `rslLeafIdx`, `keydagIdx`), and commitments (`rslKeyComm`, `keydagComm`) (`UKS-001`, `UKS-003`, `SPL-005`).
  - `boxes: BoxId -> BoxStateRec` — Per-box (`BoxMuninn`, `BoxHuginn`, `BoxOdin`) `installedManifestSha` (`100` = T10m initially), `keyFileCount`, `selectedShape`, `planD`, `proofEnteringValue`, `setupCompleted`, `preSetup*` snapshot fields, `proofRunning`, and `activeProg` (`UKS-005`, `UKS-006`, `UKS-008`).
  - `settledProofs: Set[SettledProofRec]` — Settled Compact suite proof records (`progId`, `usedManifestSha`, `used*Key`, `usedKeyRootDigest`, `usedPathC`, `openedChipLeafIdx`, `openedDagLeafIdx`, `childKeyComm`, `publicX`, `expectedX`) (`UKS-001`, `UKS-003`, `SPL-005`).
  - `lastRefusal: RefusalRecord` — Tagged union recording the latest refusal reason and diagnostic payload (`RefusalNone`, `RefusalMissingListRows`, `RefusalUncommittedRow`, `RefusalNonDescendantTree`, `RefusalRekeyGateFailed`, `RefusalD3SecurityFailed`, `RefusalCollidingRslLeafIdx`, `RefusalManifestMismatch`, `RefusalSetupKeyTamper`, `RefusalUnlistedChildKey`, `RefusalFreezeBeforeSecurityGate`).

### 4.2 `lastOp` Values and Implemented Requirements

| `lastOp` Value | Requirement ID(s) | Effect Summary |
|---|---|---|
| `"init"` | All (`UKS-001..010`, `RKY-001,002,005`, `SPL-005`, `SQG-011`) | Initializes 6 mandatory `LIST.md` rows, clean descendant tree, 3 fleet boxes on old T10m manifest (`100`), `keygenEpoch = 0`. |
| `"dropListRowBeforeRekey"` | `UKS-010` | Removes one mandatory row from `currentListRows` prior to keygen to exercise missing-row refusal. |
| `"markRowUncommitted"` | `RKY-002` | Marks a `LIST.md` row as uncommitted on its branch prior to keygen. |
| `"checkoutNonDescendantTree"` | `UKS-009` | Sets `treeDescendantOfAllRows = false` prior to keygen. |
| `"prepareValidListAndTree"` | `UKS-009`, `UKS-010`, `RKY-002` | Restores all 6 mandatory rows, commits all rows, and checks out a clean integration descendant tree. |
| `"refuseRekeyMissingRows"` | `UKS-010` | Refuses rekey start when `missingMandatoryRows(km)` is non-empty; sets `RefusalMissingListRows(missing)`. |
| `"refuseRekeyUncommittedRow"` | `RKY-002` | Refuses rekey start when a listed row is uncommitted; sets `RefusalUncommittedRow(r)`. |
| `"refuseRekeyNonDescendantTree"` | `UKS-009` | Refuses rekey start when `not(treeDescendantOfAllRows)`; sets `RefusalNonDescendantTree`. |
| `"startAndRunKeygen"` | `RKY-001`, `UKS-001`, `UKS-002`, `UKS-004`, `UKS-007`, `UKS-009`, `UKS-010`, `RKY-002` | Advances `keygenEpoch`, generates universal 4-key set, builds 7-node-row `keys_d`, and writes `MarkPrototype` manifest. |
| `"runGatesAndD3Pass"` | `UKS-006`, `RKY-005` | Reruns all 12 regression gates (12/12 pass) and D3 check (`>= 100.0` / `>= 101.0` bits), advancing `rekeyPhase` to `PhaseRekeyDone`. |
| `"runGatesFailX7"` | `UKS-006` | Reruns 12 gates with `x7` failing (`11/12`); sets `PhaseRekeyGateFailed` and `RefusalRekeyGateFailed(7)`. |
| `"runD3FailGrindable100Point9"` | `RKY-005` | Evaluates D3 with a `100.9`-bit (`1009` deci-bit) grindable term; sets `PhaseRekeyGateFailed` and `RefusalD3SecurityFailed((1025, 1009))`. |
| `"distributeKeysToBox"` | `UKS-006`, `UKS-005` | Installs `manifest.manifestSha256` on box `b` when `rekeyPhase == PhaseRekeyDone`. |
| `"runSetupModeClean"` | `UKS-008` | Runs `setup` mode on box `b`, snapshotting and preserving all key digests, `selectedShape`, `planD`, and `proofEnteringValue`. |
| `"abortSetupOnKeyTamper"` | `UKS-008` | Aborts `setup` (`setupCompleted = false`) and records `RefusalSetupKeyTamper(b)` when key tamper is detected. |
| `"buildProgramKeyRoot"` | `UKS-001`, `UKS-003` | Builds per-program `key_root` with chip sets `0..3` and `KEYDAG_IDX` at 5 pairwise-distinct leaf indices. |
| `"refuseCollidingRslLeafIdx"` | `UKS-003` | Refuses colliding R_SL leaf index (`rslIdx == KEYDAG_IDX` or in `0..2`); sets `RefusalCollidingRslLeafIdx`. |
| `"refuseStartProofManifestMismatch"` | `UKS-005` | Refuses proof start on box `b` with stale manifest; sets `RefusalManifestMismatch((b, boxSha, rekeySha))`. |
| `"startProofOnBox"` | `UKS-005` | Starts proof for program `p` on box `b` when `rekeyPhase == PhaseRekeyDone` and `installedManifestSha == manifest.manifestSha256`. |
| `"verifyAndSettleFullTree"` | `UKS-001`, `SPL-005` | Verifies child key inclusion in `key_root` (`N1`), forwards `key_root`, and records settled full-tree proof. |
| `"verifyAndSettlePathC"` | `UKS-001`, `UKS-003`, `SPL-005` | Opens `rslLeafIdx` and `KEYDAG_IDX` at distinct leaf positions in `L`, checks public `X` in `wrap`, and settles Path C proof. |
| `"refuseUnlistedChildKey"` | `SPL-005` | Refuses proof whose child key (`9999`) is unlisted in `key_root`; sets `RefusalUnlistedChildKey((p, 9999))`. |
| `"passSecurityGateAllChecks"` | `UKS-007`, `SQG-011` | Records all 6 checks (`G1..G5` and `FP-8`) passed on current `keygenEpoch` and updates `manifest.statusMark` to `MarkQualified`. |
| `"markManifestQualifiedPrematurely"` | `UKS-007` *(mutant `6` only)* | Mutant-only transition (`MUTANT_ID == 6`; disabled in `key_management_main`) that sets `MarkQualified` before security gate passes. |
| `"refuseFreezeBeforeGate"` | `SQG-011` | Refuses freeze request before `G1..G5 + FP-8` pass; sets `RefusalFreezeBeforeSecurityGate`. |
| `"freezeUniversalKeys"` | `SQG-011` | Sets `universalKeysFrozen = true` once `securityGateAllPassed(km)` holds. |
| `"appendRowAfterKeygen"` | `RKY-001`, `UKS-004` | Appends `RowPostRekeyExtra` to `LIST.md` after keygen, updating `currentListSha` and setting `secondKeygenNeeded = true`. |

### 4.3 Instance Constants

| Constant | `key_management_main` | `key_management_edge_min` | `key_management_edge_at_limit` | `key_management_m_*` | Meaning |
|---|---:|---:|---:|---:|---|
| `NODE_ROWS_REQUIRED` | `7` | `7` | `7` | `7` | Node rows in `keys_d` (`UKS-002`) |
| `FOREIGN_ROWS_COUNT` | `2` | `1` | `2` | `2` | Foreign rows in `keys_d` (`UKS-002`) |
| `KEYDAG_IDX` | `3` | `3` | `3` | `3` | Path C DAG leaf index (`UKS-003`) |
| `RSL_LEAF_IDX_CFG` | `4` | `4` | `4` | `4` | R_SL chip set 3 leaf index (`UKS-003`) |
| `REQUIRED_LIST_ROW_COUNT` | `6` | `6` | `6` | `6` | Mandatory key-changing rows in `LIST.md` (`UKS-010`) |
| `REQUIRED_GATE_COUNT` | `12` | `12` | `12` | `12` | Regression gates rerun on new keys (`UKS-006`) |
| `SECURITY_GATE_CHECK_COUNT` | `6` | `6` | `6` | `6` | `G1..G5` + `FP-8` checks (`UKS-007`, `SQG-011`) |
| `MIN_TERM_DECIBITS` | `1000` | `1000` | `1000` | `1000` | D3 non-grindable floor: `100.0` bits (`RKY-005`) |
| `MIN_GRINDABLE_DECIBITS` | `1010` | `1010` | `1010` | `1010` | D3 grindable floor: `101.0` bits (`RKY-005`) |
| `MAX_WRAP_K` | `21` | `21` | `21` | `21` | Wrap log-degree ceiling `k <= 21` |
| `MAX_WRAP_VK_BYTES` | `50000` | `50000` | `50000` | `50000` | Wrap VK byte ceiling `<= 50,000` |
| `DEFAULT_NON_GRIND_DECIBITS` | `1025` | `1000` | `1000` | `1025` | Honest D3 non-grindable score (`102.5` vs exact `100.0`) |
| `DEFAULT_GRIND_DECIBITS` | `1030` | `1010` | `1010` | `1030` | Honest D3 grindable score (`103.0` vs exact `101.0`) |
| `DEFAULT_WRAP_VK_BYTES` | `1325` | `1` | `50000` | `1325` | Honest wrap VK size (`1,325` vs `1` vs `50,000` bytes) |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 15` | `0` = sound model; `1..15` = single-guard mutant |

---

## 5. Model Abstractions

1. **Cryptographic Digests and Poseidon2 Commitments**: `sha256` digests (key files, `LIST.md`, manifest) and Poseidon2 Merkle roots/commitments (`keys_d`, `key_root`, child key commitments, Path C public `X`) are modelled as injective positive `int` expressions over `(keygenEpoch, programId, rowWeights, repoCommits)` so collision-resistance holds without bit-vector hashing.
2. **Real-Valued D3 Security Margins**: Fractional bit margins in `RKY-005` (`100` bits, `101` bits, `100.9` bits) are represented exactly as integer tenths of a bit (`deci-bits`: `1000`, `1010`, `1009`), avoiding floating-point arithmetic.
3. **Compact Suite and Prover Fleet Cardinality**: The 208-call Compact suite (`UKS-001`) is abstracted to the two named scenario programs (`ProgT02Increment`, `ProgD04Withdraw`), and the prover fleet is modelled by the three named hosts (`BoxMuninn`, `BoxHuginn`, `BoxOdin`).
4. **Git Commit Ancestry and Branch State**: `git merge-base --is-ancestor` (`UKS-009`) and branch commit existence (`RKY-002`) are abstracted to boolean/set predicates (`treeDescendantOfAllRows`, `uncommittedRows`) snapshotted at keygen execution.
5. **12 Regression Gates and 6 Security Gate Checks**: The 12 named T25 gates (`UKS-006`) and 6 T12 security checks (`G1..G5` and `FP-8` in `UKS-007` / `SQG-011`) are abstracted to pass counts and check-ID sets (`1.to(6)`), with explicit failure actions for `x7` (`runGatesFailX7`) and `100.9`-bit grindable D3 (`runD3FailGrindable100Point9`).
6. **Bounded Keygen Epochs**: `keygenEpoch` is bounded to `<= 2` (initial T25 rekey plus one second keygen triggered by `appendRowAfterKeygen`) to keep state-space exploration dense while exercising multi-epoch transitions.

---

## 6. Commands

Run every command below from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/key-management/quint`:

```bash
# 1. Typecheck model and test suite
quint typecheck key_management.qnt
# Expected exit code: 0
# Printed line: (no output on success)

quint typecheck key_management_test.qnt
# Expected exit code: 0
# Printed line: (no output on success)

# 2. Requirement scenario unit tests (16 tests)
quint test --main=key_management_test --match='km_.*Test' --seed=0x20261115 key_management_test.qnt
# Expected exit code: 0
# Printed line: 16 passing (8592ms)

# 3. Main and edge instance safety invariant checks
quint run --main=key_management_main --invariant=inv --max-samples=2000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 0
# Printed line: [ok] No violation found (39147ms at 51 traces/second).

quint run --main=key_management_edge_min --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 0
# Printed line: [ok] No violation found (12047ms at 83 traces/second).

quint run --main=key_management_edge_at_limit --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 0
# Printed line: [ok] No violation found (10143ms at 99 traces/second).

# 4. Non-vacuity reachability witnesses on main instance (negated goals: all MUST exit 1)
quint run --main=key_management_main --invariant=reach_rekey_done_and_distributed --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (198ms at 5 traces/second).

quint run --main=key_management_main --invariant=reach_two_suite_programs_settled --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (895ms at 70 traces/second).

quint run --main=key_management_main --invariant=reach_manifest_mismatch_refused --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (369ms at 3 traces/second).

quint run --main=key_management_main --invariant=reach_gate_or_d3_failure_refused --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (429ms at 5 traces/second).

quint run --main=key_management_main --invariant=reach_pre_rekey_guard_refused --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (427ms at 7 traces/second).

quint run --main=key_management_main --invariant=reach_qualified_and_frozen --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (266ms at 4 traces/second).

quint run --main=key_management_main --invariant=reach_second_keygen_triggered --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (628ms at 3 traces/second).

# 5. Guard and binding mutants (all 15 MUST exit 1)
quint run --main=key_management_m_uks001_split_keyset --invariant=inv_UKS_001 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (529ms at 30 traces/second).

quint run --main=key_management_m_uks002_short_keys_d --invariant=inv_UKS_002 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (297ms at 3 traces/second).

quint run --main=key_management_m_uks003_rsl_idx_collision --invariant=inv_UKS_003 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (419ms at 2 traces/second).

quint run --main=key_management_m_uks004_manifest_unbinds --invariant=inv_UKS_004 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (434ms at 2 traces/second).

quint run --main=key_management_m_uks005_stale_box_proves --invariant=inv_UKS_005 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (297ms at 3 traces/second).

quint run --main=key_management_m_uks006_skip_failed_gate --invariant=inv_UKS_006 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (751ms at 3 traces/second).

quint run --main=key_management_m_uks007_premature_qualified --invariant=inv_UKS_007 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (760ms at 3 traces/second).

quint run --main=key_management_m_uks008_setup_mutates_key --invariant=inv_UKS_008 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (791ms at 1 traces/second).

quint run --main=key_management_m_uks009_non_descendant_tree --invariant=inv_UKS_009 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (349ms at 20 traces/second).

quint run --main=key_management_m_uks010_missing_list_row --invariant=inv_UKS_010 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (1226ms at 2 traces/second).

quint run --main=key_management_m_rky001_stale_wrap_key --invariant=inv_RKY_001 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (209ms at 5 traces/second).

quint run --main=key_management_m_rky002_uncommitted_row --invariant=inv_RKY_002 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (887ms at 35 traces/second).

quint run --main=key_management_m_rky005_grindable_100_9 --invariant=inv_RKY_005 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (361ms at 8 traces/second).

quint run --main=key_management_m_spl005_unlisted_child_key --invariant=inv_SPL_005 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (452ms at 27 traces/second).

quint run --main=key_management_m_sqg011_freeze_before_gate --invariant=inv_SQG_011 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 key_management.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (428ms at 2 traces/second).

# 6. Generate 20 ITF replay traces
quint run key_management.qnt --main key_management_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/key-management/traces/key_management_main/t_{seq}.itf.json'
# Expected exit code: 0
# Printed line: [ok] No violation found (2477ms at 8 traces/second).
```
