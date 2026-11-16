# Quint Formal Review Report — Leaf Proving (`e2e-leaf-proving`)

- **Lane**: `e2e-leaf-proving` (Gemini 4 Carbon, independent Quint modeller)
- **Bounded context**: **Leaf Proving** (`sherlock-e2e-prototype-20261115` + completion `sherlock-completion-20261115`)
- **Quint artifacts**:
  - Model: `leaf_proving.qnt` (`leaf_proving`, `leaf_proving_main`, 2 edge instances, 17 safety mutants, 1 literal-contradiction witness instance)
  - Deterministic tests: `leaf_proving_test.qnt` (`leaf_proving_test`, `leaf_proving_conflict_test`, `leaf_proving_edge_minimal_test` — 19 `run` tests)
  - ITF replay traces: `../traces/leaf_proving_main/t_0.itf.json` .. `t_19.itf.json` (20 traces, 15 steps max, seed `0x20261115`, `inv` holds on all states)

---

## 1. Requirement Coverage (ID Table)

### 1.1 Own and Completion EARS IDs (21 / 21)

| ID | Artifact | Kind | Note |
|---|---|---|---|
| `LFP-001` | `inv_LFP_001`, `proveRslShard`, `proveInterpretedShard`, `lp_LFP_001_BabyBearStarkTest`, mutant `leaf_proving_m_bad_field_config` | `MODELED` | Every app shard proof and leaf-layer proof uses `CfgBabyBearPoseidon2`, holds no BLS12-381 commitment (`hasBls12381 == false`), and passes the stock STARK verifier (`stockVerified == true`). |
| `LFP-002` | `inv_LFP_002`, `proveRslShard`, `n1AcceptLeaf`, `lp_LFP_002_RSL_001_UniversalPrehashTest`, witness `reach_rsl_9air_sha_proved`, mutant `leaf_proving_m_non_universal_prehash` | `MODELED`, `CONSTANT-CHECKED` | Constrains every proved R_SL shard to have one app proof under chip set `CHIP_SET_RSL = 3` and one leaf-layer proof whose `n1PreHash` equals `universalN1PreHash`, verified under one `internalRecVk`. Contradicts literal `LFP-003` on 9-AIR `d04` (`Q-leaf-proving-1`). |
| `LFP-003` | `inv_LFP_003`, `admitProgram`, `refuseProgramOverAirs`, `consistency_LFP_003_vs_RSL_002`, `lp_LFP_003_RSL_002_DualHashRefusedTest`, `lp_LiteralLfp003RefusesNineAirProgramsTest`, witness `reach_over_airs_refused`, mutants `leaf_proving_m_allow_over_8_airs`, `leaf_proving_m_lfp003_literal_8_vs_9` | `MODELED`, `CONSTANT-CHECKED` | Bound `LFP_MAX_APP_AIRS = 8` refuses dual-hash program 7 (`SHA-256 + Keccak-256`, 11 non-FP-1 / 12 total AIRs) before proving. Literal `"more than 8 app AIRs"` contradicts `RSL-002` (`9 AIRs with FP-1`) and `GP-001` (`Q-leaf-proving-1`, `Q-leaf-proving-2`). |
| `LFP-004` | `inv_LFP_004`, `admitProgram`, `refuseProgramOverCols`, `lp_LFP_004_AppKeyColumnBoundTest`, `lp_EdgeMinimalExact16346BoundAndDisabledKitsTest`, witness `reach_over_cols_refused`, mutant `leaf_proving_m_allow_over_30000_cols` | `MODELED`, `CONSTANT-CHECKED` | Bound `MAX_APP_VK_COLS = 30000` admits program 5 at `30,000` columns (`16,346` in `leaf_proving_edge_minimal`) and refuses program 8 at `30,001` columns. |
| `LFP-005` | `inv_LFP_005`, `proveRslShard`, `proveInterpretedShard`, `refuseRslShardTamper`, `refuseInterpretedShardTamper`, `lp_LFP_005_FixedShardHeightsAndSwapRefusedTest`, mutant `leaf_proving_m_swapped_air_heights` | `MODELED` | Every proved shard carries its admitted ordered `airHeights` vector; swapped AIR 1 / AIR 2 heights fail verification (`RefBadShardHeights`). Missing chip-set-3 vector & app-vs-leaf ambiguity reported in `Q-leaf-proving-4`. |
| `LFP-006` | `inv_LFP_006`, `proveRslShard`, `proveInterpretedShard`, `refuseRslShardTamper`, `refuseInterpretedShardTamper`, `lp_LFP_006_LFP_007_ProgDBindingAndN1RefusalTest`, mutant `leaf_proving_m_unconstrained_progd` | `MODELED` | Leaf AIR constrains public value slot `pubProgD` to the program's `progD` across both R_SL and Interpreted shards. |
| `LFP-007` | `inv_LFP_007`, `n1AcceptLeaf`, `n1RefuseWrongProgD`, `lp_LFP_006_LFP_007_ProgDBindingAndN1RefusalTest`, witnesses `reach_n1_accepted`, `reach_n1_wrong_progd_refused`, mutant `leaf_proving_m_n1_accepts_wrong_progd` | `MODELED` | N1 statement AIR accepts a leaf into `n1Accepted[runId]` iff `lp.pubProgD == run.progD` and refuses cross-program leaves (`RefN1WrongProgD`). |
| `LFP-008` | `inv_LFP_008`, `proveRslShard`, `proveInterpretedShard`, `lp_LFP_008_LFP_009_MultiShardAndReadyEventTest`, `lp_LFP_008_RekeyOnLongRunBlockedTest`, mutant `leaf_proving_m_rekey_on_long_run` | `MODELED`, `CONSTANT-CHECKED` | Runs exceeding `SHARD_STEP_CAP` scale by `shardCount == ceilDiv(totalSteps, SHARD_STEP_CAP)` while keeping `programKeys == admittedKeySnap` (`keyRoot` unchanged) and `universalKeys == GENESIS_UNIVERSAL_KEYS`. |
| `LFP-009` | `inv_LFP_009`, `publishLeafProofsReady`, `lp_LFP_008_LFP_009_MultiShardAndReadyEventTest`, `lp_LFP_009_PrematurePublishBlockedTest`, witness `reach_multi_shard_ready_published`, mutant `leaf_proving_m_bad_ready_event` | `MODELED`, `CONSTANT-CHECKED` | Event-driven publication of `LeafProofsReady` after all `shardCount` shards complete, carrying ordered per-shard `(proofFileId, proofSha256, public values)` and `LeafKeyRef {keyRoot, chipSetId, airCount}`. |
| `LFP-010` | `inv_LFP_010`, `proveRslShard`, `refuseRslShardTamper`, `lp_LFP_010_011_012_RslStatementBindingsTest`, mutant `leaf_proving_m_unbound_fp1_outcome` | `MODELED` | R_SL app AIR constrains `fp1OutcomeD == computeOutcomeDigest(traceComm, traceOrderedW)` and refuses mismatched FP-1 outcome words (`RefBadFp1Outcome`). |
| `LFP-011` | `inv_LFP_011`, `proveRslShard`, `refuseRslShardTamper`, `lp_LFP_010_011_012_RslStatementBindingsTest`, mutant `leaf_proving_m_zero_rsl_progd` | `MODELED` | Every proved R_SL leaf carries distinct positive planner `ers006ProgD` in `pubProgD`. Prover-fill vs AIR-constraint wording noted in `Q-leaf-proving-6`. |
| `LFP-012` | `inv_LFP_012`, `proveRslShard`, `refuseRslShardTamper`, `lp_LFP_010_011_012_RslStatementBindingsTest`, `lp_LFP_012_WrongTapeInputBlockedTest`, mutant `leaf_proving_m_unbound_tape_input` | `MODELED` | R_SL leaf AIR refuses any shard whose `consumedTapeD != pubInputD` (`RefWrongTapeInputD`). Multi-shard continuation & salted `INIT-007` tension reported in `Q-leaf-proving-5`. |
| `RSL-001` | `inv_RSL_001`, `proveRslShard`, `lp_LFP_002_RSL_001_UniversalPrehashTest`, mutant `leaf_proving_m_non_universal_prehash` | `MODELED` | Every R_SL leaf proof — including 4-PUBLIC-AIR `a05` (`progId = 1`) and 1-PUBLIC-AIR `t02`, `s01`, `d04`, `s02` — yields `n1PreHash == UNIVERSAL_N1_PREHASH`. Wording conflict `"the universal leaf key"` vs `SPL-004` reported in `Q-leaf-proving-3`. |
| `RSL-002` | `inv_RSL_002`, `admitProgram`, `proveRslShard`, `consistency_LFP_003_vs_RSL_002`, `lp_LFP_003_RSL_002_DualHashRefusedTest`, witness `reach_rsl_6air_proved`, mutant `leaf_proving_m_allow_over_8_airs` | `MODELED`, `CONSTANT-CHECKED` | Every R_SL app has `totalAppAirs` in `{RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9}` with `hasFp1 == true`, `shaInPresent == true`, and `shaInEmpty == true` when `hashKit == KitNone`. Contradicts literal `LFP-003` (`Q-leaf-proving-1`). |
| `RSL-003` | `inv_RSL_003`, `runSuiteCallProve`, `recordSuiteCallBlocked`, `lp_RSL_003_SuiteProverAndBlockedTest`, witnesses `reach_suite_proved`, `reach_suite_blocked`, mutant `leaf_proving_m_accept_unverified_suite` | `MODELED`, `CONSTANT-CHECKED` | Suite table (`<= SUITE_CALL_TOTAL = 142`) records `SuiteProved` iff backed by a stock-verified app proof and records `SuiteBlockedVerifierErr` (`verifierErrorRecorded == true`) on verifier failure. |
| `RSL-004` | `inv_RSL_004`, `admitProgram`, `proveRslShard`, `consistency_RSL_004_007_vs_LFP_003`, `lp_RSL_004_005_KeccakAndSecpKitTest`, `lp_EdgeMinimalExact16346BoundAndDisabledKitsTest`, witness `reach_rsl_9air_keccak_secp_proved` | `MODELED`, `CONSTANT-CHECKED` | Gated by `HAS_KECCAK_KIT`: `s02` (`progId = 5`) proves `keccak256` inside the 9-AIR bucket with `UNIVERSAL_N1_PREHASH` when enabled, and is refused admission when `HAS_KECCAK_KIT == false`. Dual-hash gap with `RSL-007`/`LFP-003` reported in `Q-leaf-proving-2`. |
| `RSL-005` | `inv_RSL_005`, `admitProgram`, `proveRslShard`, `lp_RSL_004_005_KeccakAndSecpKitTest`, `lp_EdgeMinimalExact16346BoundAndDisabledKitsTest`, witness `reach_rsl_9air_keccak_secp_proved` | `MODELED`, `CONSTANT-CHECKED` | Gated by `HAS_SECP256K1_KIT`: `s02` (`progId = 5`) proves secp256k1 builtins inside the 9-AIR bucket with `outcomeMatchesNative == true` when enabled, and is refused admission when `HAS_SECP256K1_KIT == false`. |
| `RSL-006` | `inv_RSL_006`, `proveRslShard`, `refuseRslShardTamper`, `lp_RSL_006_007_008_JubjubShaCanonicalChecksTest`, witness `reach_rsl_jubjub_proved`, mutant `leaf_proving_m_skip_jubjub_subgroup` | `MODELED`, `CONSTANT-CHECKED` | Jubjub call `s01` (`progId = 3`) adds `0` new AIRs (stays in 6-AIR bucket), checks `on_curve` and cofactor `JUBJUB_COFACTOR = 8` (`8Q = P`), and refuses out-of-subgroup points (`RefBadJubjubRow`). |
| `RSL-007` | `inv_RSL_007`, `proveRslShard`, `refuseRslShardTamper`, `consistency_RSL_004_007_vs_LFP_003`, `lp_RSL_006_007_008_JubjubShaCanonicalChecksTest`, witness `reach_rsl_9air_sha_proved`, mutant `leaf_proving_m_allow_misaligned_sha` | `MODELED`, `CONSTANT-CHECKED` | `persistentHash` (`d04`, `progId = 4`) proves on `SHA_IN` + `SHA_REQ` in the 9-AIR bucket and refuses unaligned segments (`RefMisalignedSha`). Dual-hash conflict with `RSL-004`/`LFP-003` reported in `Q-leaf-proving-2`. |
| `RSL-008` | `inv_RSL_008`, `proveRslShard`, `refuseRslShardTamper`, `lp_RSL_006_007_008_JubjubShaCanonicalChecksTest`, mutant `leaf_proving_m_skip_canonical_range` | `MODELED` | Non-canonical ALU or Poseidon input words (`p + x` in place of `x`) fail range checks (`RefNonCanonicalWord`) and never enter `appProofs`. |
| `LPC-018` | `consistency_LPC_018`, `lp_ConsistencyAndContradictionWitnessesTest` | `SPEC-CHECKED candidate` | `LPC-018` is a QA factory process obligation on lane `t3-airs` (*"If a parent constraint of T3 never fires in a test, then the Carbon lane t3-airs SHALL add a forged input that fires it"*) with no Leaf Proving runtime state; pure `val consistency_LPC_018` verifies every modeled tamper kind maps to a distinct non-`RefNone` refusal while `TamperNone` maps to `RefNone` (see `Q-leaf-proving-6`). |

### 1.2 Adopted EARS IDs Cited by Leaf Proving (10 IDs)

| ID | Owning Change | Artifact / Disposition | Kind | Note |
|---|---|---|---|---|
| `SPL-004` | `sherlock-processor-final-20261111` | `inv_SPL_004`, `admitProgram`, `lp_LFP_002_RSL_001_UniversalPrehashTest` | `MODELED` | Per-program `appVkId`, `leafVkId`, and `internalForLeafVkId` are distinct per program and committed under `keyRoot`, while `internalRecVk`, `lVk`, and `wrapVk` are universal constants. |
| `INIT-001` | `sherlock-modular-leaf-20261111` | `inv_INIT_001`, `proveInterpretedShard`, `refuseInterpretedShardTamper`, `lp_INIT_GenesisVsNonGenesisTest`, `lp_INIT_TamperedGenesisBlockedTest`, mutant `leaf_proving_m_init_witness_flag` | `MODELED` | At `hPre == 0`, Interpreted leaf pre-state is constrained field-by-field (`preStateMatchesInit == true` and `not(onlyInitDigestMatched)`). |
| `INIT-002` | `sherlock-modular-leaf-20261111` | `inv_INIT_002`, `proveInterpretedShard`, `refuseInterpretedShardTamper`, `lp_INIT_GenesisVsNonGenesisTest`, mutant `leaf_proving_m_init_witness_flag` | `MODELED` | Genesis selector is `hPre == 0` (not a witness flag); non-genesis shard `(5, 1)` at `hPre == 4` has `preStateMatchesInit == false` and is accepted via `prevPostMatchesPre`. |
| `INIT-003` | `sherlock-modular-leaf-20261111` | `inv_INIT_003`, `proveInterpretedShard`, `lp_INIT_GenesisVsNonGenesisTest` | `MODELED` | At `hPre == 0`, `openedProgRootMatches`, `openedInputSalted`, and `fuelLimbsBelow2Pow64` hold. |
| `INIT-004` | `sherlock-modular-leaf-20261111` | `inv_INIT_004`, `proveInterpretedShard`, `lp_INIT_GenesisVsNonGenesisTest` | `MODELED` | At `hPre == 0`, `genesisMemCanonical` and `genesisRecordKindsValid` (`1..=4`) hold. |
| `INIT-005` | `sherlock-modular-leaf-20261111` | Not modeled in Leaf Proving | Not modeled here | Per-step authenticated input tree lookup (`AuthIn` against `val_d` with no input scan/size bound) belongs to `Sherlock Execution` step semantics and explicitly awaits owner decision `AD-17` / `X11`. |
| `INIT-006` | `sherlock-modular-leaf-20261111` | Not modeled in Leaf Proving | Not modeled here | Per-step `view_root` / `VIEW_NONE` / `in_ty` inspection rules under form (b) belong to `Sherlock Execution` step rules and explicitly await owner decision `AD-17` and `T5-SCHEMAS.md` freeze (`O12`). |
| `INIT-007` | `sherlock-modular-leaf-20261111` | `inv_INIT_007`, `proveInterpretedShard`, `lp_INIT_GenesisVsNonGenesisTest` | `MODELED` | At `hPre == 0`, co-processor preimages of `prog_d` and salted `input_d` (`coprocessorFormsOpened`) are opened. |
| `INIT-008` | `sherlock-modular-leaf-20261111` | `inv_INIT_008`, `proveInterpretedShard`, `lp_INIT_GenesisVsNonGenesisTest` | `MODELED`, `CONSTANT-CHECKED` | Required INIT AIRs are present (`requiredInitAirsPresent`), fixed to `admittedHeights`, and fit `appVkColsUsed <= MAX_APP_VK_COLS` (`30,000`). |
| `GP-001` | `sherlock-vm-consensus-20261107` | `inv_LFP_008` (multi-shard fixed keys), `consistency_RSL_004_007_vs_LFP_003` | Partially modeled / cross-checked | Leaf Proving models fixed universal/per-program keys across multi-shard runs (`LFP-008`) and exposes the dual-hash universality gap (`Q-leaf-proving-2`). Step-counter `2^64` node bounds, stop/resume, and cross-run rejection belong to `Aggregation` and `Scheduling`. |

---

## 2. Findings

### Finding `Q-leaf-proving-1`: Direct numeric contradiction between `LFP-003` (`> 8` app AIRs refused) and `RSL-002` / `LFP-002` / `RSL-004` / `RSL-007` (`9` app AIRs with FP-1)

- **IDs**: `LFP-003`, `RSL-002`, `LFP-002`, `RSL-004`, `RSL-007`
- **Quoted sentences**:
  - `LFP-003` (`sherlock-e2e-leaf-proving/spec.md:37`):
    > "LFP-003: If the app shape of an R_SL program needs more than 8 app AIRs, then the leaf prover SHALL refuse the shape before it makes a proof."
  - `RSL-002` (`sherlock-rsl-chipset/spec.md:15`):
    > "RSL-002: The R_SL chip set SHALL give every R_SL app 6 or 9 AIRs with FP-1."
  - `LFP-002` Scenario 1 (`sherlock-e2e-leaf-proving/spec.md:23-25`):
    > "- **GIVEN** the R_SL program of suite call `d04 withdraw` with 9 AIRs
    > - **WHEN** the leaf prover proves the app shard and the leaf layer
    > - **THEN** the N1 pre-hash equals the universal N1 pre-hash"
- **Witness trace / state**:
  - `LFP-003` kept the pre-FP-1 constant `MAX_APP_AIRS = 8` (when the R_SL buckets were 5 or 8 AIRs), whereas FP-1 added one AIR to every R_SL app (`RSL-002`: 6 or 9 AIRs with FP-1).
  - Pure consistency check `consistency_LFP_003_vs_RSL_002` (`RSL_BUCKET_HIGH <= LFP_MAX_APP_AIRS`, i.e., `9 <= 8`) evaluates to `false` in `lp_ConsistencyAndContradictionWitnessesTest`.
  - In instance `leaf_proving_m_lfp003_literal_8_vs_9` (where `LFP-003` is applied literally as `totalAppAirs <= 8`), deterministic test `lp_LiteralLfp003RefusesNineAirProgramsTest` transitions `init -> admitProgram(1) -> refuseProgramOverAirs(4) -> refuseProgramOverAirs(5)` into state `admittedProgs == Set(1)`, `refusalLog == Set({ code: RefOverMaxAirs, targetId: 4 }, { code: RefOverMaxAirs, targetId: 5 })`, and `quint run --main=leaf_proving_m_lfp003_literal_8_vs_9 --invariant=reach_rsl_9air_sha_proved` exits `0` across 1,000 traces — proving 9-AIR `d04 withdraw` (`progId = 4`) and `s02 foreign` (`progId = 5`) can never be proved if `LFP-003` is read literally over total app AIRs.
- **What would close it**:
  - Amend `LFP-003` to state either *"needs more than 9 app AIRs with FP-1 (8 non-FP-1 app AIRs)"* or *"needs an app AIR count outside the 6-or-9 AIR bucket of RSL-002"*, aligning the EARS bound with `RSL-002` and `LFP-002` Scenario 1.

---

### Finding `Q-leaf-proving-2`: Contradiction between `LFP-003` + `RSL-002` and `GP-001` + `RSL-004` + `RSL-007` on programs calling both `persistentHash` and `keccak256`

- **IDs**: `LFP-003`, `RSL-002`, `RSL-004`, `RSL-007`, `GP-001`
- **Quoted sentences**:
  - `GP-001` (`sherlock-general-purpose/spec.md:4`):
    > "One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every run length below 2^64 and every outcome (`Halted(v)` for every value type as a digest of a canonical value encoding, `Rejected(code)`, `Exhausted(kind)`)."
  - `RSL-004` (`sherlock-rsl-chipset/spec.md:37`):
    > "RSL-004: Where the machine version has the keccak256 kit, when a circuit calls `keccak256`, the R_SL chip set SHALL prove the call inside the 6-or-9 AIR bucket."
  - `RSL-007` (`sherlock-rsl-chipset/spec.md:70`):
    > "RSL-007: When a circuit calls `persistentHash`, the R_SL chip set SHALL prove the hash on the SHA_IN and SHA_REQ AIRs with the openvm SHA-256 block hasher."
  - `LFP-003` Scenario 1 (`sherlock-e2e-leaf-proving/spec.md:42-44`):
    > "- **GIVEN** a program that calls `persistentHash` and `keccak256`
    > - **WHEN** the leaf prover builds the chip set
    > - **THEN** the leaf prover refuses the program and writes no proof"
- **Witness trace / state**:
  - Neither `RSL-004` nor `RSL-007` states a precondition excluding the other hash kit in the same circuit, and `GP-001` requires proving *every* program of the language. Yet `programCatalog(7)` (`hashKit == KitBothShaAndKeccak`, `nonFp1AppAirs == 11`, `totalAppAirs == 12`) is mandatorily refused by `LFP-003` (`lp_LFP_003_RSL_002_DualHashRefusedTest`: `init -> refuseProgramOverAirs(7)` leaves `admittedProgs == Set()`, `appProofs == Map()`, `leafProofs == Map()`, `refusalLog == Set({ code: RefOverMaxAirs, targetId: 7 })`), and `consistency_RSL_004_007_vs_LFP_003` evaluates to `false`.
- **What would close it**:
  - Either (a) add a 12-AIR chip-set bucket under `key_root` (or compiler split / interpreted fallback) so dual-hash circuits have a valid proving route satisfying `GP-001`, `RSL-004`, and `RSL-007`, or (b) scope `GP-001` for the R_SL fast path and add an explicit guard *"when the circuit calls at most one block-hash kit"* to `RSL-004` and `RSL-007`.

---

### Finding `Q-leaf-proving-3`: Entity contradiction between `RSL-001` ("the universal leaf key") and `SPL-004` / `LFP-002` (per-program leaf-layer keys under `key_root`)

- **IDs**: `RSL-001`, `SPL-004`, `LFP-002`
- **Quoted sentences**:
  - `RSL-001` (`sherlock-rsl-chipset/spec.md:4`):
    > "RSL-001: When the rsl probe proves a leaf over the R_SL app proof of a suite call, the universal leaf key SHALL give an N1 pre-hash equal to the universal N1 pre-hash."
  - `SPL-004` (`sherlock-shard-plan/spec.md:29`):
    > "Per program, the app key, the OpenVM leaf-layer key and the internal-for-leaf key of each chip set SHALL be committed under `key_root`, with the Fr commitment `X` of the child DAG per entry and tree path. The internal-recursive key, the `L` key and the wrap key SHALL be the same for every program."
  - `LFP-002` (`sherlock-e2e-leaf-proving/spec.md:18`):
    > "LFP-002: When the leaf prover receives an `RslStatement` and its `ExecutionTrace`, the leaf prover SHALL make, for each shard, one app proof under R_SL chip set 3 and one leaf-layer proof whose N1 pre-hash equals the universal N1 pre-hash."
- **Witness trace / state**:
  - `SPL-004` makes the OpenVM leaf-layer key **per-program** under `key_root` (not a single universal key). `LFP-002` was already reworded after review R-06 (as noted in `DDD.md` section 7 row `universal leaf key`), but `RSL-001` still normatively refers to *"the universal leaf key"*.
  - In `lp_LFP_002_RSL_001_UniversalPrehashTest`, after `admitProgram(1)` (`a05`, 6 AIRs) and `admitProgram(4)` (`d04`, 9 AIRs), `programKeys.get(1).leafVkId == 301 != 304 == programKeys.get(4).leafVkId` (`inv_SPL_004`), while both per-program leaf keys produce `leafN1PreHash == 9001 == UNIVERSAL_N1_PREHASH` verified under `universalKeys.internalRecVk == 9101`.
- **What would close it**:
  - Reword `RSL-001` to match `LFP-002` and `SPL-004`: *"the program's leaf-layer key under `key_root` SHALL give an N1 pre-hash equal to the universal N1 pre-hash."*

---

### Finding `Q-leaf-proving-4`: `LFP-005` leaves chip set 3 (`R_SL`) without a specified height vector and leaves app-AIR vs leaf-layer-AIR scope ambiguous

- **IDs**: `LFP-005`, `LFP-002`, `RSL-002`
- **Quoted sentences**:
  - `LFP-005` (`sherlock-e2e-leaf-proving/spec.md:65`):
    > "LFP-005: The leaf prover SHALL prove every leaf shard with the ordered AIR height vector of its admitted shape."
  - `LFP-005` Status Scenario (`sherlock-e2e-leaf-proving/spec.md:81`):
    > "GAP: chip set 3 has no named shape vector and no negative case."
  - `RSL-002` Scenario 1 (`sherlock-rsl-chipset/spec.md:18-19`):
    > "- **WHEN** a program has no hash
    > - **THEN** its app has 6 AIRs, with SHA_IN present and empty"
- **Witness trace / state**:
  - `LFP-005` requires proving every `"leaf shard"` with the ordered AIR height vector of its `"admitted shape"`, whereas `LFP-002` splits each R_SL shard into **two** proofs: an app proof (`AppProofRec`, 6 or 9 AIRs under chip set 3) and a leaf-layer recursion proof (`LeafProofRec`). No requirement specifies (1) the admitted height vector(s) for chip set 3, (2) whether `LFP-005` constrains the leaf-layer AIR heights, the R_SL app AIR heights, or both, or (3) what concrete trace height `"SHA_IN present and empty"` (`RSL-002`) takes in a STARK backend that requires positive power-of-two AIR heights.
  - Our model attaches symbolic `admittedHeights` to both `AppProofRec` and `LeafProofRec` (`inv_LFP_005`) and witnesses `TamperSwappedHeights` failure in `leaf_proving_m_swapped_air_heights`, but cannot check concrete chip-set-3 numbers from the spec text alone.
- **What would close it**:
  - Specify in `LFP-005` (or `RSL-002`) that both the per-shard app proof (6 or 9 AIRs, naming the minimum padded height for an empty `SHA_IN` AIR) and the leaf-layer proof carry fixed ordered power-of-two height vectors from the admitted `sherlock plan` shape menu (`SPL-006`).

---

### Finding `Q-leaf-proving-5`: `LFP-012` omits a genesis / multi-shard rule for `input_d` on continuation R_SL shards (`LFP-008`) and omits the `INIT-007` salted-envelope mapping

- **IDs**: `LFP-012`, `LFP-008`, `INIT-002`, `INIT-007`
- **Quoted sentences**:
  - `LFP-012` (`sherlock-e2e-leaf-proving/spec.md:168`):
    > "LFP-012: If the digest of the tape that an R_SL app consumed differs from the `input_d` of the leaf statement, then the leaf AIR SHALL refuse the shard."
  - `LFP-008` (`sherlock-e2e-leaf-proving/spec.md:112`):
    > "LFP-008: When a run has more steps than one leaf shape holds, the leaf prover SHALL add leaf shards and leave every per-program key and every universal key unchanged."
  - `INIT-002` (`sherlock-init-admission/spec.md:19`):
    > "The genesis selector SHALL be `g = is_zero(h_pre)` from the leaf public value `h_pre` (FP-1), not a witness flag. Every INIT constraint SHALL be multiplied by `g`. At `h_pre != 0` INIT SHALL constrain nothing."
  - `INIT-007` (`sherlock-init-admission/spec.md:78`):
    > "Under `sherlock-processor-final-20261111`, `prog_d` SHALL be `P2(DOM_PROG, image_d, plan_d, key_root, machine_version)` (SPL-003) and `input_d` SHALL be `P2(SHK_IN, salt, val_d, fuel_lo, fuel_hi, stack_limit)` with 8 salt words fixed at INIT (ZK-006)."
- **Witness trace / state**:
  - Unlike `INIT-002` (which gates genesis admission on `is_zero(h_pre)` so continuation shards `shardIdx > 0` in `runId = 5` pass without re-checking genesis `Init`), `LFP-012` states unconditionally that **every** R_SL shard is refused if the digest of the tape that shard consumed differs from `input_d`. In a multi-shard R_SL run (`runId = 2` with 3 shards or `runId = 3` with 2 shards under `LFP-008`), if shard `s > 0` consumes only its own tape slice rather than recommitting the entire run tape, literal `LFP-012` (`consumedTapeD != pubInputD`) refuses every honest continuation shard (`lp_LFP_012_WrongTapeInputBlockedTest`). In addition, `LFP-012` equates `"the digest of the tape"` directly with `"input_d"`, without stating how the tape digest sits inside `INIT-007`'s salted `P2(SHK_IN, salt, val_d, fuel_lo, fuel_hi, stack_limit)` preimage.
- **What would close it**:
  - State explicitly in `LFP-012` whether (a) every R_SL shard carries the full-run tape Merkle/Poseidon2 root `val_d` opened into `input_d = P2(SHK_IN, salt, val_d, ...)` at `h_pre = 0` (`INIT-002` / `INIT-007`) and forwarded across shards by N1/N2, or (b) each R_SL shard re-hashes the entire tape.

---

### Finding `Q-leaf-proving-6`: `LPC-018` is a QA-lane process rule assigned to Leaf Proving, and `LFP-011` states prover-fill rather than an R_SL AIR constraint

- **IDs**: `LPC-018`, `LFP-011`, `LFP-006`
- **Quoted sentences**:
  - `LPC-018` (`sherlock-loop-completion/spec.md:191`):
    > "LPC-018: If a parent constraint of T3 never fires in a test, then the Carbon lane t3-airs SHALL add a forged input that fires it."
  - `LFP-011` (`sherlock-e2e-leaf-proving/spec.md:154`):
    > "LFP-011: When the leaf prover proves a shard of an R_SL program, the leaf prover SHALL set the public value slot `PROG_D` to the `prog_d` of ERS-006 for that program."
  - `LFP-006` (`sherlock-e2e-leaf-proving/spec.md:84`):
    > "LFP-006: The leaf AIR SHALL constrain the public value slot `PROG_D` to the `prog_d` of the program that the shard runs."
- **Witness trace / state**:
  - First, `LPC-018` names a factory QA obligation on Carbon lane `t3-airs` (`G1Error::Optional`, `ShapeError::Lengths`, `from_wrapin` assert in Bridge/Wrap) with no Leaf Proving runtime state; it is recorded as a `SPEC-CHECKED candidate` backed by pure check `consistency_LPC_018 == true`.
  - Second, whereas `LFP-006`, `LFP-010`, and `LFP-012` are phrased as **AIR constraints** (`"the ... AIR SHALL constrain / refuse"`), `LFP-011` is phrased only as an honest-prover action (`"the leaf prover SHALL set"`). If an implementation of `LFP-011` only sets the public slot in the prover without binding `ERS-006`'s `prog_d` in the R_SL leaf AIR / `key_root` path (as mutant `leaf_proving_m_zero_rsl_progd` or `leaf_proving_m_unconstrained_progd` demonstrates, violating `inv_LFP_011` and `inv_LFP_006` in 3 steps: `init -> admitProgram(...) -> proveRslShard(..., TamperZeroRslProgD)`), a dishonest prover can forge `PROG_D` on an R_SL leaf.
- **What would close it**:
  - Reassign `LPC-018` to the `Assurance` context in the completion mapping, and strengthen `LFP-011` from *"the leaf prover SHALL set"* to *"the leaf prover SHALL set and the R_SL leaf AIR SHALL constrain"* `PROG_D` to the `ERS-006` `prog_d` bound to `key_root`.

---

## 3. Mutant Table

Every safety mutant below weakens or removes exactly **one** refusal or binding guard while leaving all honest transitions (`admitProgram(1..6)`, `proveRslShard(..., TamperNone)`, `proveInterpretedShard(..., TamperNone)`, `publishLeafProofsReady`, `n1AcceptLeaf`, `runSuiteCallProve(..., true)`) enabled — none of the 17 mutants is vacuous. In `leaf_proving_main`, `inv` passes (`exit 0`, 2,000 traces).

| Mutant Module | Target Invariant(s) | Guard Removed / Weakened | Exit Code | Observed Result (`--max-samples=1000 --max-steps=15 --seed=0x20261115`) |
|---|---|---|---|---|
| `leaf_proving_m_bad_field_config` | `inv_LFP_001` | Allows `TamperBadFieldCfg` (`CfgWrongFieldOrHash` / `hasBls12381 = true`) in shard proving | `1` | `[violation] Found an issue (347ms at 3 traces/second).` |
| `leaf_proving_m_non_universal_prehash` | `inv_LFP_002`, `inv_RSL_001` | Allows `TamperPrehashDrift` (`n1PreHash = 9999 != UNIVERSAL_N1_PREHASH`) on R_SL leaf proof | `1` / `1` | `[violation] Found an issue (348ms)` on `inv_LFP_002`; `(1906ms)` on `inv_RSL_001` |
| `leaf_proving_m_allow_over_8_airs` | `inv_LFP_003`, `inv_RSL_002` | Removes app-AIR bucket guard in `airCountAllowed`, admitting dual-hash program 7 (11 non-FP-1 / 12 total AIRs) | `1` / `1` | `[violation] Found an issue (446ms)` on `inv_LFP_003`; `(1392ms)` on `inv_RSL_002` |
| `leaf_proving_m_allow_over_30000_cols` | `inv_LFP_004` | Removes `appVkCols <= MAX_APP_VK_COLS` guard, admitting program 8 (`30,001` columns) | `1` | `[violation] Found an issue (336ms at 3 traces/second).` |
| `leaf_proving_m_swapped_air_heights` | `inv_LFP_005` | Allows `TamperSwappedHeights` (exchanging AIR 1 and AIR 2 heights) to verify under leaf key | `1` | `[violation] Found an issue (337ms at 3 traces/second).` |
| `leaf_proving_m_unconstrained_progd` | `inv_LFP_006` | Allows `TamperWrongProgDSlot` (`pubProgD = 999 != prog.progD`) in leaf AIR | `1` | `[violation] Found an issue (1718ms at 1 traces/second).` |
| `leaf_proving_m_n1_accepts_wrong_progd` | `inv_LFP_007` | Removes `lp.pubProgD == r.progD` check in `canN1AcceptLeaf`, letting N1 merge another program's leaf | `1` | `[violation] Found an issue (1643ms at 1 traces/second).` |
| `leaf_proving_m_rekey_on_long_run` | `inv_LFP_008` | Allows `TamperRekeyLongRun` to mutate `keyRoot` and `manifestSha256` on shard `sIdx > 0` of a multi-shard run | `1` | `[violation] Found an issue (1692ms at 1 traces/second).` |
| `leaf_proving_m_bad_ready_event` | `inv_LFP_009` | Allows `publishLeafProofsReady` before all shards of a multi-shard run are proved | `1` | `[violation] Found an issue (1843ms at 1 traces/second).` |
| `leaf_proving_m_unbound_fp1_outcome` | `inv_LFP_010` | Allows `TamperBadFp1Outcome` (`fp1OutcomeD = 0 != digest(comm, ordered W)`) in R_SL app AIR | `1` | `[violation] Found an issue (523ms at 2 traces/second).` |
| `leaf_proving_m_zero_rsl_progd` | `inv_LFP_011` | Allows `TamperZeroRslProgD` (`pubProgD = 0` instead of `ers006ProgD`) on R_SL leaf proof | `1` | `[violation] Found an issue (435ms at 2 traces/second).` |
| `leaf_proving_m_unbound_tape_input` | `inv_LFP_012` | Allows `TamperWrongTapeInput` (`consumedTapeD = inputD + 77 != pubInputD`) on R_SL leaf proof | `1` | `[violation] Found an issue (375ms at 3 traces/second).` |
| `leaf_proving_m_accept_unverified_suite` | `inv_RSL_003` | Allows `runSuiteCallProve(pId, false)` to mark a suite call `SuiteProved` without a stock-verified app proof | `1` | `[violation] Found an issue (624ms at 3 traces/second).` |
| `leaf_proving_m_skip_jubjub_subgroup` | `inv_RSL_006` | Allows `TamperJubjubOutSubgroup` (`JubjubOutSubgroup`, cofactor `0`) on Jubjub call `s01` | `1` | `[violation] Found an issue (1753ms at 2 traces/second).` |
| `leaf_proving_m_allow_misaligned_sha` | `inv_RSL_007` | Allows `TamperMisalignedSha` (`shaAligned = false`) on `persistentHash` call `d04` | `1` | `[violation] Found an issue (484ms at 4 traces/second).` |
| `leaf_proving_m_skip_canonical_range` | `inv_RSL_008` | Allows `TamperNonCanonicalWord` (`wordsCanonical = false`, e.g., `p + x`) on R_SL rows | `1` | `[violation] Found an issue (595ms at 2 traces/second).` |
| `leaf_proving_m_init_witness_flag` | `inv_INIT_001`, `inv_INIT_002` | Allows `TamperInitWitnessGate` at `hPre == 0` (witness flag / digest-only match instead of `is_zero(h_pre)` field check) | `1` / `1` | `[violation] Found an issue (1164ms)` on `inv_INIT_001`; `(1212ms)` on `inv_INIT_002` |
| `leaf_proving_m_lfp003_literal_8_vs_9` *(contradiction witness)* | `reach_rsl_9air_sha_proved` *(stays unviolated!)* | Enforces literal `LFP-003` (`totalAppAirs <= 8`), blocking 9-AIR `d04` (`progId = 4`) and `s02` (`progId = 5`) | `0` | `[ok] No violation found (51168ms at 20 traces/second).` (Confirms `Q-leaf-proving-1`) |

---

## 4. State Vocabulary for Replay

In ITF JSON files (`../traces/leaf_proving_main/t_{seq}.itf.json`), Quint qualifies each state variable as `"leaf_proving_main::leaf_proving::<varName>"`.

### 4.1 State Variables (`var`)

| Quint `var` | Quint Type | Meaning in Leaf Proving |
|---|---|---|
| `lastOp` | `str` | Name of the transition that produced the current state (`"init"` in state 0). |
| `lastArgs` | `StepArgs` (`{ progId: int, runId: int, shardIdx: int, slotId: int, refusal: RefusalCode }`) | Structured arguments of `lastOp` so the Lean replay driver can dispatch deterministically without guessing nondeterministic choices. |
| `universalKeys` | `UniversalKeyState` (`{ universalN1PreHash, internalRecVk, lVk, wrapVk, manifestSha256: int }`) | Universal key identities and manifest hash (`9001, 9101, 9102, 9103, 9200` at genesis). |
| `admittedProgs` | `Set[int]` | Set of admitted program IDs from `1..8` whose chip-set shape and VK column bounds passed planning/harness admission. |
| `programKeys` | `int -> ProgramKeyRec` | Per-program key bundle committed under `keyRoot` (`chipSetId`, `airCount`, `appVkId`, `leafVkId`, `internalForLeafVkId`, `appVkCols`, `leafN1PreHash`). |
| `admittedKeySnap` | `int -> ProgramKeyRec` | Immutable snapshot of each program's `ProgramKeyRec` at admission time, used by `inv_LFP_008` to verify multi-shard runs never rekey. |
| `appProofs` | `int -> AppProofRec` | Map from `slotId = runId * 10 + shardIdx` to R_SL chip-set-3 app shard proofs (STARK config, BLS flag, AIR counts, SHA/secp/Jubjub/canonical flags, `airHeights`, `traceComm`, `traceOrderedW`, `fp1OutcomeD`). |
| `leafProofs` | `int -> LeafProofRec` | Map from `slotId = runId * 10 + shardIdx` to leaf-layer proofs (both `FamRsl` and `FamInterpreted`), carrying `n1PreHash`, `airHeights`, `proofFileId`, `proofSha256`, `pubProgD`, `pubInputD`, `consumedTapeD`, `pubFp1OutcomeD`, `hPre`, and `INIT` flags. |
| `readyEvents` | `int -> LeafProofsReadyRec` | Map from `runId` to published `LeafProofsReady` event records (`entries: List[ReadyLeafEntry]` in shard order `0 .. shardCount - 1`, plus `leafKeyRef: { keyRoot, chipSetId, airCount }`). |
| `n1Accepted` | `int -> Set[int]` | Map from `runId` (`1..6`) to the set of leaf `slotId`s accepted by the N1 statement AIR for that run (`LFP-007`). |
| `suiteTable` | `int -> SuiteEntry` | Map from suite program ID (`1..5`) to `{ progId, disposition: SuitePending | SuiteProved | SuiteBlockedVerifierErr, appProofSlot, verifierErrorRecorded }` (`RSL-003`). |
| `refusalLog` | `Set[RefusalEvent]` | Accumulated set of `{ code: RefusalCode, targetId: int }` refusals emitted by admission, shard AIR/host checks, or N1 (`targetId` is `progId` for program refusals, `slotId` for shard refusals, and `runId * 100 + slotId` for N1 refusals). |

### 4.2 `lastOp` Values and Implemented Requirements

| `lastOp` Value | `lastArgs` Fields Populated | Implemented EARS IDs | Transition Semantics |
|---|---|---|---|
| `"init"` | all `0`, `RefNone` | Genesis setup | Initializes `universalKeys = GENESIS_UNIVERSAL_KEYS`, empty maps/sets for proofs/keys/events/refusals, empty `n1Accepted[1..6]`, and `SuitePending` entries in `suiteTable[1..5]`. |
| `"admitProgram"` | `progId` | `SPL-004`, `RSL-002`, `LFP-003`, `LFP-004`, `RSL-004`, `RSL-005` | Admits `progId` when kit flags, AIR bucket, and `appVkCols <= MAX_APP_VK_COLS` hold; records per-program keys in `programKeys` and `admittedKeySnap`. |
| `"refuseProgramOverAirs"` | `progId`, `refusal = RefOverMaxAirs` | `LFP-003`, `RSL-002` | Refuses an R_SL program whose app AIR count violates `airCountAllowed` before any proof is written; logs `{ RefOverMaxAirs, progId }`. |
| `"refuseProgramOverCols"` | `progId`, `refusal = RefOverVkCols` | `LFP-004`, `INIT-008` | Refuses a program with `appVkCols > MAX_APP_VK_COLS`; logs `{ RefOverVkCols, progId }`. |
| `"proveRslShard"` | `progId`, `runId`, `shardIdx`, `slotId` | `LFP-001`, `LFP-002`, `LFP-005`, `LFP-006`, `LFP-008`, `LFP-010`..`012`, `RSL-001`, `RSL-002`, `RSL-004`..`008` | Produces both `appProofs[slotId]` (chip set 3) and `leafProofs[slotId]` (`n1PreHash = UNIVERSAL_N1_PREHASH`) for shard `shardIdx` of R_SL `runId`. |
| `"proveInterpretedShard"` | `progId`, `runId`, `shardIdx`, `slotId` | `LFP-001`, `LFP-005`, `LFP-006`, `LFP-008`, `INIT-001`..`004`, `INIT-007`, `INIT-008` | Produces `leafProofs[slotId]` for shard `shardIdx` of Interpreted `runId`, enforcing `INIT` constraints iff `hPre == 0`. |
| `"refuseRslShardTamper"` | `progId`, `runId`, `shardIdx`, `slotId`, `refusal` | `LFP-005`, `LFP-006`, `LFP-010`..`012`, `RSL-006`..`008` | Refuses a tampered R_SL shard witness/statement without writing `appProofs[slotId]` or `leafProofs[slotId]`; logs `{ refusal, slotId }`. |
| `"refuseInterpretedShardTamper"` | `progId`, `runId`, `shardIdx`, `slotId`, `refusal` | `LFP-005`, `LFP-006`, `INIT-001`..`004`, `INIT-007`, `INIT-008` | Refuses a tampered Interpreted shard (swapped heights, wrong `prog_d`, or bad genesis `INIT` at `hPre == 0`); logs `{ refusal, slotId }`. |
| `"publishLeafProofsReady"` | `progId`, `runId` | `LFP-009` | Once all `0 .. shardCount - 1` shards of `runId` exist in `leafProofs`, publishes `readyEvents[runId]` with ordered shard file/hash/public entries and `LeafKeyRef`. |
| `"n1AcceptLeaf"` | `progId`, `runId`, `slotId` | `LFP-002`, `LFP-006`, `LFP-007`, `RSL-001` | Adds `slotId` to `n1Accepted[runId]` when `leafProofs[slotId].pubProgD == runCatalog(runId).progD`. |
| `"n1RefuseWrongProgD"` | `progId`, `runId`, `slotId`, `refusal = RefN1WrongProgD` | `LFP-007` | Refuses merging `leafProofs[slotId]` into `runId` when `pubProgD != runCatalog(runId).progD`; logs `{ RefN1WrongProgD, runId * 100 + slotId }`. |
| `"runSuiteCallProve"` | `progId`, `slotId` | `RSL-003`, `LFP-001` | Transitions `suiteTable[progId]` from `SuitePending` to `SuiteProved` linked to stock-verified `appProofs[slotId]`. |
| `"recordSuiteCallBlocked"` | `progId`, `slotId = -1` | `RSL-003` | Transitions `suiteTable[progId]` from `SuitePending` to `SuiteBlockedVerifierErr` with `verifierErrorRecorded = true`. |

### 4.3 Concrete Instance Constants and Catalog Summary

| Constant | `leaf_proving_main` | `leaf_proving_edge_minimal` | `leaf_proving_edge_exact_bounds` | Meaning / Spec Source |
|---|---|---|---|---|
| `CHIP_SET_RSL` | `3` | `3` | `3` | R_SL chip set ID (`LFP-002`) |
| `LFP_MAX_APP_AIRS` | `8` | `8` | `8` | Max non-FP-1 app AIRs (`LFP-003`) |
| `RSL_BUCKET_LOW` | `6` | `6` | `6` | Low R_SL AIR bucket with FP-1 (`RSL-002`) |
| `RSL_BUCKET_HIGH` | `9` | `9` | `9` | High R_SL AIR bucket with FP-1 (`RSL-002`) |
| `MAX_APP_VK_COLS` | `30000` | `16346` | `30000` | App VK column cap (`LFP-004`; `16,346` is largest measured R_SL app) |
| `SHARD_STEP_CAP` | `4` | `1` | `2` | Steps per leaf shard (`LFP-008`) |
| `MAX_SHARDS_PER_RUN` | `3` | `2` | `3` | Max shards per modeled run (`LFP-008`, `LFP-009`) |
| `SUITE_CALL_TOTAL` | `142` | `5` | `5` | Suite call table cap (`RSL-003`) |
| `JUBJUB_COFACTOR` | `8` | `8` | `8` | Jubjub subgroup check cofactor `8Q = P` (`RSL-006`) |
| `HAS_KECCAK_KIT` | `true` | `false` | `true` | Machine version includes `keccak256` kit (`RSL-004`) |
| `HAS_SECP256K1_KIT` | `true` | `false` | `true` | Machine version includes `secp256k1` kit (`RSL-005`) |
| `MUTANT` | `"none"` | `"none"` | `"none"` | Active mutant tag (`"none"` in all sound instances) |

Fixed catalogs in `leaf_proving`:
- `programCatalog(1..8)`: `1 = a05` (R_SL, no hash, 4 PUBLIC AIRs, 5/6 AIRs, 12,000 cols), `2 = t02` (R_SL, no hash, 5/6 AIRs, 8,000 cols), `3 = s01` (R_SL, Jubjub, 5/6 AIRs, 14,000 cols), `4 = d04` (R_SL, SHA-256, 8/9 AIRs, 16,346 cols), `5 = s02` (R_SL, Keccak-256 + secp256k1, 8/9 AIRs, 30,000 cols), `6 = interp0` (Interpreted + INIT, 7 AIRs, 15,000 cols), `7 = badDualHash` (R_SL, SHA-256 + Keccak-256, 11/12 AIRs, 22,000 cols), `8 = badOverCols` (R_SL, 5/6 AIRs, 30,001 cols).
- `runCatalog(1..6)`: `1 -> prog 1` (1 shard), `2 -> prog 2` (`min(3, MAX_SHARDS_PER_RUN)` shards), `3 -> prog 4` (`min(2, MAX_SHARDS_PER_RUN)` shards), `4 -> prog 5` (1 shard), `5 -> prog 6` (`min(2, MAX_SHARDS_PER_RUN)` shards), `6 -> prog 3` (1 shard).

---

## 5. Model Abstractions

1. **Cryptographic primitives and digests**: BabyBear / BLS12-381 `Fr` field elements, Poseidon2 digests (`prog_d`, `input_d`, `key_root`, `n1PreHash`), SHA-256 proof file hashes (`proofSha256`, `manifestSha256`), and outcome commitments (`computeOutcomeDigest(comm, orderedW)`) are abstracted as collision-free small integers and deterministic polynomial folds over `List[int]`.
2. **STARK proofs and verifying keys**: OpenVM `BabyBearPoseidon2Config` app and leaf-layer proofs and verifying keys are represented as typed records (`AppProofRec`, `LeafProofRec`, `ProgramKeyRec`) carrying explicit structural and constraint-satisfaction fields (`starkCfg`, `hasBls12381`, `stockVerified`, `airHeights`, `appVkCols`, `n1PreHash`) rather than polynomial commitments or FRI query transcripts.
3. **Finite program and run universe**: Infinite program/trace inputs are abstracted to an 8-program catalog (`PROG_IDS = 1..8`) and 6-run catalog (`RUN_IDS = 1..6`) with `SHARD_STEP_CAP in {1, 2, 4}` and up to 3 shards per run, chosen to cover every equivalence class named in the specs (6-AIR no-hash, 4-PUBLIC-AIR `a05`, Jubjub micro-ops, 9-AIR SHA-256 `d04`, 9-AIR Keccak+secp256k1 `s02`, multi-shard Interpreted with `INIT`, dual-hash >8-AIR refusal, and >30,000-column VK refusal).
4. **Suite call table (`RSL-003`)**: The 142 R_SL suite calls are represented by the 5 distinct R_SL archetype calls (`SUITE_PROG_IDS = 1..5`) bounded by `SUITE_CALL_TOTAL = 142`.
5. **Kit and range constraints (`RSL-005`..`008`)**: Elliptic-curve arithmetic on secp256k1 and Jubjub (`on_curve`, `8Q = P`), SHA-256 ledger segment alignment, and BabyBear/limb canonical range checks (`x < p` vs `p + x`) are modeled as discrete algebraic state predicates (`outcomeMatchesNative`, `jubjubCheck`, `jubjubCofactorChecked`, `shaAligned`, `wordsCanonical`) and corresponding `TamperKind` transitions.
6. **Interpreted `INIT` admission (`INIT-001`..`008`)**: Genesis field-by-field admission, `is_zero(h_pre)` gating (`hPre == 0` vs `hPre != 0`), salted `input_d` / `prog_d` co-processor openings, and canonical genesis memory are modeled as explicit per-leaf booleans on `FamInterpreted` shards; per-step input-node lookups (`INIT-005`, `INIT-006`) belong to `Sherlock Execution` and are left outside Leaf Proving.

---

## 6. Commands

Every command below is written exactly as it should be rerun from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/leaf-proving/quint`:

1. **Typecheck base model & instances**
   - Command: `quint typecheck leaf_proving.qnt`
   - Expected exit code: `0`
   - Printed line: *(empty output on success)*

2. **Typecheck deterministic test suite**
   - Command: `quint typecheck leaf_proving_test.qnt`
   - Expected exit code: `0`
   - Printed line: *(empty output on success)*

3. **Run main deterministic unit, scenario, refusal, and consistency tests**
   - Command: `quint test leaf_proving_test.qnt --main=leaf_proving_test --match='lp_.*' --seed=0x20261115`
   - Expected exit code: `0`
   - Printed line: `17 passing (12275ms)`

4. **Run literal `LFP-003` vs `RSL-002` contradiction test (`Q-leaf-proving-1`)**
   - Command: `quint test leaf_proving_test.qnt --main=leaf_proving_conflict_test --match='lp_.*' --seed=0x20261115`
   - Expected exit code: `0`
   - Printed line: `1 passing (1567ms)`

5. **Run edge-minimal exact-bound (`16,346` cols) & disabled-kits test**
   - Command: `quint test leaf_proving_test.qnt --main=leaf_proving_edge_minimal_test --match='lp_.*' --seed=0x20261115`
   - Expected exit code: `0`
   - Printed line: `1 passing (1239ms)`

6. **Simulate master invariant `inv` on `leaf_proving_main`**
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=inv --max-samples=2000 --max-steps=15 --seed=0x20261115`
   - Expected exit code: `0`
   - Printed line: `[ok] No violation found (70303ms at 28 traces/second).`

7. **Simulate master invariant `inv` on `leaf_proving_edge_minimal`**
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_edge_minimal --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115`
   - Expected exit code: `0`
   - Printed line: `[ok] No violation found (25207ms at 40 traces/second).`

8. **Simulate master invariant `inv` on `leaf_proving_edge_exact_bounds`**
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_edge_exact_bounds --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115`
   - Expected exit code: `0`
   - Printed line: `[ok] No violation found (32160ms at 31 traces/second).`

9. **Reachability witnesses on `leaf_proving_main` (all 13 MUST violate)**
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_rsl_6air_proved --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (299ms at 3 traces/second).`
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_rsl_9air_sha_proved --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (616ms at 3 traces/second).`
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_rsl_9air_keccak_secp_proved --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (1931ms at 4 traces/second).`
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_rsl_jubjub_proved --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (393ms at 18 traces/second).`
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_interpreted_multishard_proved --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (571ms at 2 traces/second).`
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_multi_shard_ready_published --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (792ms at 6 traces/second).`
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_n1_accepted --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (457ms at 2 traces/second).`
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_n1_wrong_progd_refused --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (343ms at 3 traces/second).`
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_over_airs_refused --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (976ms at 1 traces/second).`
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_over_cols_refused --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (319ms at 3 traces/second).`
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_shard_tamper_refused --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (539ms at 2 traces/second).`
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_suite_proved --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (903ms at 3 traces/second).`
   - Command: `quint run leaf_proving.qnt --main=leaf_proving_main --invariant=reach_suite_blocked --max-samples=1000 --max-steps=15 --seed=0x20261115`
     - Expected exit code: `1` | Printed line: `[violation] Found an issue (2904ms at 0 traces/second).`

10. **Contradiction witness run (`Q-leaf-proving-1`: literal `LFP-003` makes 9-AIR `d04` unreachable)**
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_lfp003_literal_8_vs_9 --invariant=reach_rsl_9air_sha_proved --max-samples=1000 --max-steps=15 --seed=0x20261115`
    - Expected exit code: `0`
    - Printed line: `[ok] No violation found (51168ms at 20 traces/second).`

11. **Safety mutant runs (all 20 checks MUST violate with exit code `1`)**
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_bad_field_config --invariant=inv_LFP_001 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (347ms at 3 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_non_universal_prehash --invariant=inv_LFP_002 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (348ms at 3 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_non_universal_prehash --invariant=inv_RSL_001 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (1906ms at 1 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_allow_over_8_airs --invariant=inv_LFP_003 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (446ms at 2 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_allow_over_8_airs --invariant=inv_RSL_002 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (1392ms at 1 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_allow_over_30000_cols --invariant=inv_LFP_004 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (336ms at 3 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_swapped_air_heights --invariant=inv_LFP_005 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (337ms at 3 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_unconstrained_progd --invariant=inv_LFP_006 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (1718ms at 1 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_n1_accepts_wrong_progd --invariant=inv_LFP_007 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (1643ms at 1 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_rekey_on_long_run --invariant=inv_LFP_008 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (1692ms at 1 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_bad_ready_event --invariant=inv_LFP_009 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (1843ms at 1 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_unbound_fp1_outcome --invariant=inv_LFP_010 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (523ms at 2 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_zero_rsl_progd --invariant=inv_LFP_011 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (435ms at 2 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_unbound_tape_input --invariant=inv_LFP_012 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (375ms at 3 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_accept_unverified_suite --invariant=inv_RSL_003 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (624ms at 3 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_skip_jubjub_subgroup --invariant=inv_RSL_006 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (1753ms at 2 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_allow_misaligned_sha --invariant=inv_RSL_007 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (484ms at 4 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_skip_canonical_range --invariant=inv_RSL_008 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (595ms at 2 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_init_witness_flag --invariant=inv_INIT_001 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (1164ms at 2 traces/second).`
    - Command: `quint run leaf_proving.qnt --main=leaf_proving_m_init_witness_flag --invariant=inv_INIT_002 --max-samples=1000 --max-steps=15 --seed=0x20261115`
      - Expected exit code: `1` | Printed line: `[violation] Found an issue (1212ms at 2 traces/second).`

12. **Generate 20 ITF replay traces**
    - Command: `quint run leaf_proving.qnt --main leaf_proving_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/leaf-proving/traces/leaf_proving_main/t_{seq}.itf.json'`
    - Expected exit code: `0`
    - Printed line: `[ok] No violation found (951ms at 21 traces/second).`
