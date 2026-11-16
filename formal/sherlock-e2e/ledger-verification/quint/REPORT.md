# Quint Formal Model Report — Bounded Context **Ledger Verification** (`e2e-ledger-verification`)

Independent Quint 0.32.0 specification (`ledger_verification.qnt`), scenario test suite (`ledger_verification_test.qnt`), 11 guard mutants, 6 reachability witnesses, and 20 ITF replay traces (`../traces/ledger_verification_main/t_{0..19}.itf.json`) derived solely from the assigned EARS requirements and `DDD.md` Ledger Verification rows.

---

## 1. Requirement Coverage Table

### 1.1 Own (`sherlock-e2e-prototype-20261115`) and Completion (`sherlock-completion-20261115`) IDs

> Completion EARS IDs assigned to this context: `(none)`.

| ID | Artifact (`ledger_verification.qnt` / `ledger_verification_test.qnt`) | Kind | Note |
|---|---|---|---|
| `LVR-001` | `inv_LVR_001`, `PINNED_LEDGER_REV_ID = 9101`, `doCheckManifestPass`, `doRunLedgerCheckTool`, `lv_honestCheckAndTenTamperControlsRefusedTest`, mutant `ledger_verification_m_lvr001_unpinned_verifier` | `CONSTANT-CHECKED` | Every `sherlock-wrap-ledger-check` run uses `VerifierKey::verify` of stock `midnight-transient-crypto` at ledger 9.1.0.1 (`rev a2b30128`, modelled as `PINNED_LEDGER_REV_ID = 9101`) after passing the manifest gate. |
| `LVR-002` | `inv_LVR_002`, `PINNED_LEDGER_CRATE_COUNT = 4`, `REQUIRED_LEDGER_CRATES`, `doCheckManifestPass`, `doRefuseManifestOverride`, `lv_manifestGateAndPatchCratesIoTest`, mutant `ledger_verification_m_lvr002_allow_path_override` | `CONSTANT-CHECKED` | Resolves all `4` crates (`midnight-transient-crypto`, `midnight-serialize`, `midnight-base-crypto`, `midnight-storage-core`) to `rev a2b30128` via git or `[patch.crates-io]`, refusing `path` overrides and other revs; unchecked `midnight-zk-stdlib` / short-prefix gap noted in `Q-ledger-verification-2`. |
| `LVR-003` | `inv_LVR_003`, `doRunLedgerCheckTool`, `lv_honestCheckAndTenTamperControlsRefusedTest`, `lv_g4KeyRefusedByReaderTest`, mutant `ledger_verification_m_lvr003_accept_g4_key` | `MODELED` | Loads `wrap.vk` via `VerifierKey::deserialize` in `MidnightVK` framing and refuses `G4LaneGateKey` and malformed key framings. |
| `LVR-004` | `inv_LVR_004`, `CHANGED_LANE_CONTROL_COUNT = 7`, `FLIPPED_BYTE_CONTROL_COUNT = 3`, `CALL_VECTOR_LEN = 1026`, `namedControlLanes`, `doRunLedgerCheckTool`, `lv_honestCheckAndTenTamperControlsRefusedTest`, mutant `ledger_verification_m_lvr004_skip_tamper_controls` | `CONSTANT-CHECKED` | Every accepted-key check verifies all `7` changed-lane controls (`{0, 10, 20, 34, 39, middle, last}`) and all `3` flipped-byte controls; underspecification of lane indices, byte offsets, and deserialization errors noted in `Q-ledger-verification-3`. |
| `LVR-005` | `inv_LVR_005`, `doRunLedgerCheckTool`, `lv_acceptedControlOrRefusedHonestExitsNonzeroTest`, mutants `ledger_verification_m_lvr005_zero_exit_accepted_ctrl` and `ledger_verification_m_lvr005_zero_exit_refused_honest` | `MODELED` | Exits with nonzero status (`exitCode != 0`, `verdictOk == false`) whenever any tamper control is accepted, the honest proof is refused, or `wrap.vk` is refused; missing `wrap.vk` / parse failure clause in literal `LVR-005` noted in `Q-ledger-verification-3`. |
| `LVR-006` | `inv_LVR_006`, `NODE_INVALID_PROOF_ERR_CODE = 115`, `doBuildLedgerCall`, `doRunNativeValidator`, `doSubmitValidatedCall`, `lv_nativeValidatorBeforeSubmitAndRefusalNoSubmitTest`, mutant `ledger_verification_m_lvr006_submit_without_native` | `CONSTANT-CHECKED` | Settle runner runs the native ledger 9.1.0.1 validator on every built call before submit, recording `"failed to verify proof: Invalid proof"` (equivalent to node custom error `115`) on invalid/tampered calls. |
| `LVR-007` | `inv_LVR_007`, `doRunNativeValidator`, `doSubmitValidatedCall`, `lv_nativeValidatorBeforeSubmitAndRefusalNoSubmitTest`, mutant `ledger_verification_m_lvr007_submit_natively_refused` | `MODELED` | Natively refused calls are never submitted (`not(submittedToNode)`), are marked refused in run state (`markedRefusedInState`), and flag `needsFreshProveOnRerun`; restriction to *"the honest call"* noted in `Q-ledger-verification-4`. |
| `LVR-008` | `inv_LVR_008`, `doConnectSettleNode`, `doRefuseSecondGenesisForFolder`, `lv_chainIdentitySecondGenesisRefusalAndPreviewPinTest`, mutant `ledger_verification_m_lvr008_allow_second_genesis` | `MODELED` | First connect for a state folder records `(recordedGenesisHash, recordedNodeBuildId)` and refuses any second genesis hash with `"same local chain as the deploy"`; missing normative refusal/version-update clauses noted in `Q-ledger-verification-1`. |

### 1.2 Adopted IDs (`sherlock-processor-final-20261111`)

| Adopted ID | Disposition in `e2e-ledger-verification` | Artifact / Reason |
|---|---|---|
| `LW-017` | Modeled (`inv_LW_017`, `inv_LVR_003`, `lv_g4KeyRefusedByReaderTest`, mutant `ledger_verification_m_lw017_report_g4_accepted`) | Ledger 9.1.0.1 accepts only `MidnightVkZkStdLib` keys, refuses `G4LaneGateKey` at key parsing, and forbids reporting any G4 figure as ledger-accepted (`not(g4ReportedAccepted)`). |
| `MID-006` | Modeled (Preview pin: `inv_MID_006`, `PINNED_PREVIEW_NODE_BUILD_ID = 21101`, `doRecordPreviewQualification`, `doRefusePreviewPinMismatch`, mutant `ledger_verification_m_mid006_preview_pin_drift`) | Preview qualification record requires `previewReportedNodeBuildId == previewPinnedNodeBuildId == 21101` (`2.1.101-bbd616b1`); Mainnet governance procedure `SWF-009` is an external operations rule (`SPEC-CHECKED candidate`) owned outside Ledger Verification. |

---

## 2. Findings (`Q-ledger-verification-1` .. `Q-ledger-verification-5`)

### `Q-ledger-verification-1`: `LVR-008` normative EARS sentence specifies only first-connect recording, omitting its Scenario's second-chain refusal, subsequent node-version updates, and `MID-006` Preview build enforcement

- **IDs**: `LVR-008`, `MID-006`
- **Quoted sentences**:
  - `LVR-008`: *"When the settle runner first connects to a node for a state folder, the settle runner SHALL record the genesis hash and the node version in the run state."*
  - `LVR-008` Scenario: *"GIVEN a state folder that holds the genesis hash of an earlier connect WHEN the runner connects to a node with another genesis hash THEN the runner stops with 'same local chain as the deploy'"*
  - `MID-006`: *"The Preview qualification SHALL name the node build that the Preview node reports, `2.1.101-bbd616b1`. The Preview pin SHALL record the same build."*
- **Witness state / trace**:
  - Under `LVR-008`'s literal normative trigger (`When the settle runner first connects...`), connecting folder `1` first to `(genesisHash = 100, nodeBuildId = 21101)` and then reconnecting folder `1` to `(genesisHash = 200, nodeBuildId = 21199)` satisfies `LVR-008`'s normative sentence (`st.chain ??= chain` recorded `(100, 21101)` on first connect) while violating `LVR-008`'s Scenario (`"same local chain as the deploy"`, caught by `inv_LVR_008` on mutant `ledger_verification_m_lvr008_allow_second_genesis`) and leaving later node-version changes (`21101 -> 21199` at the same genesis `100`) unrecorded in `recordedNodeBuildId` (`spec.md:104`). In addition, `"same local chain as the deploy"` hardcodes `"local chain"` even on Preview runs (`MID-006`).
- **What would close it**:
  - Add two explicit normative EARS sentences to `LVR-008`: (1) *"If a subsequent connect for a state folder observes a genesis hash different from the recorded genesis hash, then the settle runner SHALL stop before submitting with a chain-mismatch refusal"*, and (2) *"When a subsequent connect at the recorded genesis hash observes a different node or runtime version, the settle runner SHALL append `(nodeVersion, runtimeVersion, height, timestamp)` to the run state and, on Preview, refuse if `nodeVersion` differs from the `MID-006` pin."*

### `Q-ledger-verification-2`: `LVR-002` checks only 4 `midnight-ledger` crates by an 8-hex prefix, leaving `[patch.crates-io]` overrides on `midnight-zk-stdlib` / `midnight-proofs` (`LW-017`) unguarded

- **IDs**: `LVR-001`, `LVR-002`, `LW-017`
- **Quoted sentences**:
  - `LVR-001`: *"The ledger check tool SHALL verify each wrap proof with `VerifierKey::verify` of the stock `midnight-transient-crypto` crate at ledger tag 9.1.0.1 (rev `a2b30128`)."*
  - `LVR-002`: *"The ledger check crate SHALL resolve each of the 4 ledger crates `midnight-transient-crypto`, `midnight-serialize`, `midnight-base-crypto` and `midnight-storage-core` to git rev `a2b30128` of midnight-ledger."*
  - `LVR-002` Scenario: *"a `[patch.crates-io]` entry that selects rev `a2b30128` passes, and published crates such as `midnight-circuits` are not checked"*
  - `LW-017` Scenario: *"WHEN a G4 wrap key is installed on ledger 9 without a change to `midnight-zk-stdlib` THEN `VerifierKey::verify` refuses it at key parsing"*
- **Witness state**:
  - Pure witness `gap_q_lvr_2_stdlib_patch_passes_4crate_check` (verified in `lv_chainIdentitySecondGenesisRefusalAndPreviewPinTest`):
  - With `deps = mkPinnedCrateDeps(true)` (all 4 named `midnight-*` ledger crates resolved to `a2b30128`) and `stdlibStock = false` (a local `[patch.crates-io]` override modifying `midnight-zk-stdlib` or `midnight-proofs` to accept G4 gates), `allFourLedgerCratesPinned(deps)` evaluates to `true`, passing `LVR-002` while invalidating `LVR-001`'s stock-verifier premise and `LW-017`'s G4 refusal guarantee. Also, `LVR-001`/`LVR-002` state an 8-hex prefix `a2b30128` in normative text while `LVR-001` Scenario names the full 40-hex SHA-1 `a2b30128ca4b660b73a068e29b56b48ede8c9a74`.
- **What would close it**:
  - Pin the full 40-character commit `a2b30128ca4b660b73a068e29b56b48ede8c9a74` and upstream repository URL in `LVR-002`, and forbid `[patch.crates-io]` / `path` overrides on transitive verifier crates (`midnight-zk-stdlib`, `midnight-circuits`, `midnight-proofs`, `midnight-curves`) or verify `Cargo.lock` cryptographic checksums.

### `Q-ledger-verification-3`: `LVR-004` and `LVR-005` underspecify tamper-control lane indices/byte offsets, proof-deserialization failures, and nonzero exit when `LVR-003` refuses `wrap.vk`

- **IDs**: `LVR-003`, `LVR-004`, `LVR-005`
- **Quoted sentences**:
  - `LVR-003`: *"When the ledger check tool loads `wrap.vk`, the ledger check tool SHALL read the key through `VerifierKey::deserialize` in the `MidnightVK` framing."*
  - `LVR-004`: *"When the ledger check tool verifies a wrap proof, the ledger check tool SHALL also verify 7 vectors with one changed public input and 3 proofs with one flipped byte."*
  - `LVR-004` Scenario: *"THEN lane 0, lane 10 (anchor), lanes 20 and 34 (`claim_pub`), lane 39 (`outcome_pub`), the middle lane and the last lane are each changed by 1 and refused"*
  - `LVR-005`: *"If the ledger verifier accepts a tamper control or refuses the honest proof, then the ledger check tool SHALL exit with a nonzero status."*
- **Witness state**:
  - (a) Pure witness `gap_q_lvr_3_short_vector_lane_collision`: `LVR-004`'s normative sentence neither names the 7 lanes nor states a minimum call-vector length (`1,026`), and for vector lengths `40, 41, 68, 69, 78, 79` the Scenario formula `{0, 10, 20, 34, 39, N/2, N-1}` suffers index collisions (`namedControlLanes(40).size() == 5`, `namedControlLanes(41).size() == 6`, `namedControlLanes(68).size() == 6`, `namedControlLanes(78).size() == 6`). `LVR-004` also leaves the 3 flipped-byte positions/bitmasks unspecified and does not state whether a flipped proof byte rejected by `Proof::deserialize` (compressed BLS12-381 point off-curve) before `VerifierKey::verify` counts as a refused control.
  - (b) In `lv_g4KeyRefusedByReaderTest` (`bundleId = 2`, `vkFraming = G4LaneGateKey`), `VerifierKey::deserialize` refuses `wrap.vk` in `LVR-003` before `VerifierKey::verify` runs on any proof; `LVR-005`'s literal `If` clause (*"accepts a tamper control or refuses the honest proof"*) does not explicitly cover key-deserialization or public-input JSON parse failures.
- **What would close it**:
  - Move the exact 7 lane indices (`0, 10, 20, 34, 39, 513, 1025` for `N = 1,026`), modular `Fr` `+ 1` arithmetic, and the 3 deterministic proof byte offsets/masks into normative `LVR-004` (allowing refusal at either `Proof::deserialize` or `VerifierKey::verify`), and expand `LVR-005` to require nonzero exit whenever key deserialization (`LVR-003`), input parsing, honest verification, or any of the 10 tamper controls fails.

### `Q-ledger-verification-4`: `LVR-007` scopes its no-submit rule to *"the honest call"* and omits proof-cache invalidation required by its Scenario

- **IDs**: `LVR-006`, `LVR-007`
- **Quoted sentences**:
  - `LVR-006`: *"When a ledger call is built, the settle runner SHALL run the native ledger 9.1.0.1 validator on the call before the submit."*
  - `LVR-006` Scenario: *"GIVEN the tampered call R1 WHEN the native validator runs THEN the result is 'failed to verify proof: Invalid proof', the same refusal the node gives as custom error 115"*
  - `LVR-007`: *"If the native ledger validator refuses the honest call, then the settle runner SHALL submit nothing and mark the proven call as refused."*
  - `LVR-007` Scenario: *"GIVEN an honest call that the native validator refuses WHEN the runner stops THEN the state records the refusal and the next run proves a new call"*
- **Witness state**:
  - For a tampered or corrupted call (`callId = 2`, `isHonestCall = false`, `proofValidOnLedger9 = false` in `lv_nativeValidatorBeforeSubmitAndRefusalNoSubmitTest`), `LVR-006` runs the native validator and receives `"failed to verify proof: Invalid proof"`, yet literal `LVR-007` (*"refuses **the honest call**"*) does not forbid submitting non-honest calls unless an explicit negative-test flag is gated. Furthermore, `LVR-007`'s normative sentence marks the call `refused` without stating that cached proof artifacts are invalidated (`needsFreshProveOnRerun = true`) so the next run proves a fresh call rather than remaining permanently stuck in `refused`.
- **What would close it**:
  - Generalize `LVR-007` to *"If the native ledger validator refuses a built ledger call (outside an explicit negative-conformance node-error test), then the settle runner SHALL submit nothing, mark the call attempt refused, and invalidate its cached proof so a subsequent run proves a fresh call."*

### `Q-ledger-verification-5`: No EARS requirement binds `sherlock-wrap-ledger-check` (`LVR-001..005` `WrapVerdict`) as a precondition to the settle runner (`LVR-006..007`)

- **IDs**: `LVR-001`, `LVR-004`, `LVR-005`, `LVR-006`, `LVR-007`
- **Quoted sentences**:
  - `LVR-004`: *"When the ledger check tool verifies a wrap proof, the ledger check tool SHALL also verify 7 vectors with one changed public input and 3 proofs with one flipped byte."*
  - `LVR-006`: *"When a ledger call is built, the settle runner SHALL run the native ledger 9.1.0.1 validator on the call before the submit."*
- **Witness trace**:
  - In `t_0.itf.json` (steps 1–2) and `lv_nativeValidatorBeforeSubmitAndRefusalNoSubmitTest`, `buildLedgerCall` -> `runNativeValidator` -> `submitValidatedCall` satisfies all of `LVR-006..008` even when `checkRuns == Set()` (`sherlock-wrap-ledger-check` never ran on the `WrapOutput`, so none of `LVR-004`'s 10 tamper controls were checked).
- **What would close it**:
  - Add a requirement in `sherlock-e2e-ledger-verifier` (or cross-reference `HLP-001`) stating that the settle runner SHALL refuse to build or submit a ledger call from a `WrapOutput` folder unless a matching `WrapVerdict` (`exitCode == 0`, `changed_lane_refused == 7`, `flipped_proof_byte_refused == 3`) is recorded for that `WrapOutput`'s SHA-256.

---

## 3. Mutant Table

Every mutant (`MUTANT_ID = 1..11`) weakens one refusal or binding guard while keeping honest paths reachable (verified non-vacuous: `reach_honest_ledger_check_pass` exits `1` on all 11 mutants).

| Mutant module | Target invariant | Exit code (`quint run`) | Non-vacuity (`reach_honest_ledger_check_pass`) | Guard weakened |
|---|---|---|---|---|
| `ledger_verification_m_lvr001_unpinned_verifier` | `inv_LVR_001` | `1` (`[violation]`) | `1` (`[violation]`) | Runs `ledger-check` with unpinned verifier rev `9999 != 9101` (`usedStockVerifierVerify = false`) |
| `ledger_verification_m_lvr002_allow_path_override` | `inv_LVR_002` | `1` (`[violation]`) | `1` (`[violation]`) | Passes manifest gate when `midnight-transient-crypto` uses `LocalPathOverride` |
| `ledger_verification_m_lvr003_accept_g4_key` | `inv_LVR_003` | `1` (`[violation]`) | `1` (`[violation]`) | Allows `VerifierKey::deserialize` to accept `G4LaneGateKey` |
| `ledger_verification_m_lvr004_skip_tamper_controls` | `inv_LVR_004` | `1` (`[violation]`) | `1` (`[violation]`) | Omits `lane 39` (`6` lanes instead of `7`) and checks only `2` flipped-byte proofs |
| `ledger_verification_m_lvr005_zero_exit_accepted_ctrl` | `inv_LVR_005` | `1` (`[violation]`) | `1` (`[violation]`) | Returns `exitCode = 0` when a flipped-byte tamper control verifies |
| `ledger_verification_m_lvr005_zero_exit_refused_honest` | `inv_LVR_005` | `1` (`[violation]`) | `1` (`[violation]`) | Returns `exitCode = 0` when the honest proof is refused |
| `ledger_verification_m_lvr006_submit_without_native` | `inv_LVR_006` | `1` (`[violation]`) | `1` (`[violation]`) | Submits a built ledger call before running the native 9.1.0.1 validator |
| `ledger_verification_m_lvr007_submit_natively_refused` | `inv_LVR_007` | `1` (`[violation]`) | `1` (`[violation]`) | Submits a call after the native validator refused it |
| `ledger_verification_m_lvr008_allow_second_genesis` | `inv_LVR_008` | `1` (`[violation]`) | `1` (`[violation]`) | Overwrites state folder genesis (`100 -> 200`) instead of stopping on second chain |
| `ledger_verification_m_lw017_report_g4_accepted` | `inv_LW_017` | `1` (`[violation]`) | `1` (`[violation]`) | Sets `g4ReportedAccepted = true` on a `G4LaneGateKey` bundle |
| `ledger_verification_m_mid006_preview_pin_drift` | `inv_MID_006` | `1` (`[violation]`) | `1` (`[violation]`) | Writes Preview qualification record with `reportedBuild = 21199 != 21101` |

---

## 4. State Vocabulary for Trace Replay

### 4.1 State Variables (`vars` in ITF JSON)

In `../traces/ledger_verification_main/t_{0..19}.itf.json`, Quint qualifies the module variables under `ledger_verification_main::ledger_verification::`:

| Quint `var` | ITF JSON state key | Type | Meaning |
|---|---|---|---|
| `lvr` | `ledger_verification_main::ledger_verification::lvr` | `LvrState` | Complete observable state of **Ledger Verification** (Cargo manifest dependency resolution, staged `WrapOutput` bundles, `sherlock-wrap-ledger-check` verdicts, state-folder chain records, pre-submit native validator calls, Preview qualification pin). |
| `lastOp` | `ledger_verification_main::ledger_verification::lastOp` | `str` | Name of the action that produced the current state (`"init"` at step `0`). |

#### `LvrState` fields (`ledger_verification_main::ledger_verification::lvr`)

| Field | Type | Meaning |
|---|---|---|
| `crateDeps` | `Set[CrateDepRecord]` | Dependency resolution records (`crateName`, `sourceKind`, `revId`) for the 4 `midnight-*` ledger crates (`LVR-002`). |
| `stdlibUnmodified` | `bool` | Whether `midnight-zk-stdlib` is stock 9.1.0.1 (`LW-017`). |
| `manifestGateChecked` | `bool` | Whether the Cargo metadata manifest gate has run. |
| `manifestGatePassed` | `bool` | Whether the manifest gate passed (`LVR-002`). |
| `lastManifestError` | `str` | Most recent manifest refusal (`"none"`, `"local_path_override_refused"`, `"wrong_git_rev_refused"`). |
| `bundles` | `int -> WrapBundleInput` | Staged `WrapOutput` bundles (`bundleId`, `vkFraming`, `publicInputCount`, `honestProofValid`, `forgedControlPassesVerifier`). |
| `knownBundleIds` | `Set[int]` | Active keys in `bundles` (`1` honest; `2..4` negative fixtures). |
| `nextCheckId` | `int` | Monotonic ID counter (`1..7`) for `checkRuns`. |
| `checkRuns` | `Set[LedgerCheckRunRecord]` | `sherlock-wrap-ledger-check` run records (`WrapVerdict` fields + `exitCode` + `g4ReportedAccepted`; `LVR-001`, `LVR-003..005`, `LW-017`). |
| `folders` | `int -> FolderChainRecord` | Settle-runner state folders (`connectedOnce`, `initialGenesisHash`, `recordedGenesisHash`, `recordedNodeBuildId`, `observedNodeBuilds`, `chainMismatchStopped`, `lastConnectStopReason`; `LVR-008`). |
| `knownFolderIds` | `Set[int]` | Pre-populated folder IDs `Set(1, 2)`. |
| `calls` | `int -> SettleCallRecord` | Built ledger calls (`isHonestCall`, `proofValidOnLedger9`, `nativeValidatorRan`, `nativeValidatorAccepted`, `nativeRefusalMsg`, `equivalentNodeCustomError`, `markedRefusedInState`, `needsFreshProveOnRerun`, `submittedToNode`; `LVR-006`, `LVR-007`). |
| `knownCallIds` | `Set[int]` | Active keys in `calls`. |
| `previewQualWritten` | `bool` | Whether a Preview qualification record has been written (`MID-006`). |
| `previewReportedNodeBuildId` | `int` | Node build reported by Preview when `previewQualWritten` (`21101` = `"2.1.101-bbd616b1"`). |
| `previewPinnedNodeBuildId` | `int` | Preview pin recorded in qualification (`21101`; `MID-006`). |
| `lastRunnerError` | `str` | Most recent settle-runner error (`"none"`, `"failed to verify proof: Invalid proof"`, `"same local chain as the deploy"`, `"preview_node_build_pin_mismatch"`). |

### 4.2 `lastOp` Values and Implemented EARS IDs

| `lastOp` value | Action in `ledger_verification.qnt` | Implemented EARS IDs |
|---|---|---|
| `"init"` | `init` | Initial state |
| `"checkManifestPass"` | `doCheckManifestPass` | `LVR-001`, `LVR-002`, `LW-017` |
| `"refuseManifestOverride"` | `doRefuseManifestOverride` | `LVR-002` |
| `"stageWrapBundle"` | `doStageWrapBundle` | `LVR-001`, `LVR-003`, `LVR-004`, `LVR-005`, `LW-017` |
| `"runLedgerCheckTool"` | `doRunLedgerCheckTool` | `LVR-001`, `LVR-003`, `LVR-004`, `LVR-005`, `LW-017` |
| `"connectSettleNode"` | `doConnectSettleNode` | `LVR-008` |
| `"refuseSecondGenesisForFolder"` | `doRefuseSecondGenesisForFolder` | `LVR-008` (Scenario: second chain refused) |
| `"buildLedgerCall"` | `doBuildLedgerCall` | `LVR-006`, `LVR-007` |
| `"runNativeValidator"` | `doRunNativeValidator` | `LVR-006`, `LVR-007` |
| `"submitValidatedCall"` | `doSubmitValidatedCall` | `LVR-006`, `LVR-007` |
| `"recordPreviewQualification"` | `doRecordPreviewQualification` | `MID-006` |
| `"refusePreviewPinMismatch"` | `doRefusePreviewPinMismatch` | `MID-006` |

### 4.3 Concrete Instance Constants

| Constant | `ledger_verification_main` | `ledger_verification_edge_min` | `ledger_verification_edge_at_limit` | `ledger_verification_m_*` (`11` mutants) |
|---|---|---|---|---|
| `PINNED_LEDGER_CRATE_COUNT` | `4` | `4` | `4` | `4` |
| `CHANGED_LANE_CONTROL_COUNT` | `7` | `7` | `7` | `7` |
| `FLIPPED_BYTE_CONTROL_COUNT` | `3` | `3` | `3` | `3` |
| `CALL_VECTOR_LEN` | `1026` | `42` | `65536` | `1026` |
| `PINNED_LEDGER_REV_ID` | `9101` | `9101` | `9101` | `9101` |
| `PINNED_PREVIEW_NODE_BUILD_ID` | `21101` | `21101` | `21101` | `21101` |
| `NODE_INVALID_PROOF_ERR_CODE` | `115` | `115` | `115` | `115` |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 11` |

---

## 5. Model Abstractions

1. **Git commits and node build strings**: Full commit SHA `a2b30128ca4b660b73a068e29b56b48ede8c9a74` (tag `9.1.0.1`) is modelled as `PINNED_LEDGER_REV_ID = 9101` (with `9999` representing any other commit), and Preview node build `"2.1.101-bbd616b1"` is modelled as `PINNED_PREVIEW_NODE_BUILD_ID = 21101` (`21102`/`21199` for other builds).
2. **Cryptographic pairing verification and key deserialization**: `VerifierKey::deserialize` (`MidnightVK`) and `VerifierKey::verify` are abstracted via `VkFraming` (`MidnightVkZkStdLib | G4LaneGateKey | MalformedVkFraming`) and boolean validity attributes (`honestProofValid`, `forgedControlPassesVerifier`) on `WrapBundleInput`.
3. **Tamper controls**: The 7 changed-public-input controls are represented as the exact computed lane-index set `namedControlLanes(CALL_VECTOR_LEN)` (`{0, 10, 20, 34, 39, CALL_VECTOR_LEN / 2, CALL_VECTOR_LEN - 1}`), and the 3 flipped-byte proof controls as integer counts (`flippedByteCheckedCount`, `flippedByteRefusedCount`).
4. **Node RPC and genesis hashes**: Chain genesis hashes are small positive integers (`100` vs `200`), and node RPC proof rejection is represented by `NODE_INVALID_PROOF_ERR_CODE = 115` paired with the native error string `"failed to verify proof: Invalid proof"`.

---

## 6. Commands

Run from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/ledger-verification/quint`:

```bash
# 1. Typecheck specification and test suite
quint typecheck ledger_verification.qnt
# Expected exit code: 0 (no output on success)

quint typecheck ledger_verification_test.qnt
# Expected exit code: 0 (no output on success)

# 2. Deterministic scenario tests
quint test --main=ledger_verification_test --match='lv_.*Test' --seed=0x20261115 ledger_verification_test.qnt
# Expected exit code: 0
# Printed: 6 passing (1199ms)

# 3. Honest instances (main and edge bounds)
quint run ledger_verification.qnt --main ledger_verification_main --invariant inv --max-samples 2000 --max-steps 15 --seed 0x20261115
# Expected exit code: 0
# Printed: [ok] No violation found (2961ms at 675 traces/second).

quint run ledger_verification.qnt --main ledger_verification_edge_min --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 0
# Printed: [ok] No violation found (1925ms at 519 traces/second).

quint run ledger_verification.qnt --main ledger_verification_edge_at_limit --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 0
# Printed: [ok] No violation found (1032ms at 969 traces/second).

# 4. Non-vacuity reachability witnesses (all 6 MUST violate, exit code 1)
quint run ledger_verification.qnt --main ledger_verification_main --invariant reach_honest_ledger_check_pass --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (428ms at 9 traces/second).

quint run ledger_verification.qnt --main ledger_verification_main --invariant reach_ledger_check_nonzero_exit --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (358ms at 17 traces/second).

quint run ledger_verification.qnt --main ledger_verification_main --invariant reach_manifest_override_refused --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (461ms at 2 traces/second).

quint run ledger_verification.qnt --main ledger_verification_main --invariant reach_native_validate_and_submit --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (171ms at 29 traces/second).

quint run ledger_verification.qnt --main ledger_verification_main --invariant reach_native_refusal_no_submit_115 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (656ms at 17 traces/second).

quint run ledger_verification.qnt --main ledger_verification_main --invariant reach_second_genesis_refused_and_preview_pinned --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (314ms at 10 traces/second).

# 5. Guard mutants (all 11 MUST violate their named invariant, exit code 1)
quint run ledger_verification.qnt --main ledger_verification_m_lvr001_unpinned_verifier --invariant inv_LVR_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (341ms at 6 traces/second).

quint run ledger_verification.qnt --main ledger_verification_m_lvr002_allow_path_override --invariant inv_LVR_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (280ms at 7 traces/second).

quint run ledger_verification.qnt --main ledger_verification_m_lvr003_accept_g4_key --invariant inv_LVR_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (259ms at 27 traces/second).

quint run ledger_verification.qnt --main ledger_verification_m_lvr004_skip_tamper_controls --invariant inv_LVR_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (220ms at 18 traces/second).

quint run ledger_verification.qnt --main ledger_verification_m_lvr005_zero_exit_accepted_ctrl --invariant inv_LVR_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (208ms at 149 traces/second).

quint run ledger_verification.qnt --main ledger_verification_m_lvr005_zero_exit_refused_honest --invariant inv_LVR_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (415ms at 14 traces/second).

quint run ledger_verification.qnt --main ledger_verification_m_lvr006_submit_without_native --invariant inv_LVR_006 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (253ms at 16 traces/second).

quint run ledger_verification.qnt --main ledger_verification_m_lvr007_submit_natively_refused --invariant inv_LVR_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (191ms at 241 traces/second).

quint run ledger_verification.qnt --main ledger_verification_m_lvr008_allow_second_genesis --invariant inv_LVR_008 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (176ms at 17 traces/second).

quint run ledger_verification.qnt --main ledger_verification_m_lw017_report_g4_accepted --invariant inv_LW_017 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (270ms at 107 traces/second).

quint run ledger_verification.qnt --main ledger_verification_m_mid006_preview_pin_drift --invariant inv_MID_006 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (252ms at 8 traces/second).

# 6. Replay trace generation (20 ITF JSON traces)
mkdir -p /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/ledger-verification/traces/ledger_verification_main
quint run ledger_verification.qnt --main ledger_verification_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/ledger-verification/traces/ledger_verification_main/t_{seq}.itf.json'
# Expected exit code: 0
# Printed: [ok] No violation found (542ms at 37 traces/second).
```
