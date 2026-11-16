# Quint Formal Specification & Soundness Report — Aggregation (`e2e-aggregation`)

- **Bounded Context**: Aggregation (`sherlock-e2e-node-aggregation`, `sherlock-e2e-gp-hooks`, `sherlock-loop-completion`)
- **Spec Module**: [aggregation.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/aggregation/quint/aggregation.qnt)
- **Test Module**: [aggregation_test.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/aggregation/quint/aggregation_test.qnt)
- **Replay Traces**: `../traces/aggregation_main/t_0.itf.json` .. `t_19.itf.json` (20 15-step ITF traces generated under `--seed 0x20261115`)

---

## 1. Requirement Coverage Table (Own, Completion, and Adopted EARS IDs)

### 1.1 Own & Completion EARS IDs

| ID | Artifact (`aggregation.qnt` / `aggregation_test.qnt`) | Kind | Note |
|---|---|---|---|
| `NAG-001` | `inv_NAG_001`, actions `proveN1`, `proveN2Merge`, `proveN3Root`, `proveRunNodeTree`, test `ag_NAG_001_002_003_NodeChainKeyRootAndOmitAirTest`, witness `reach_n1_n2_n3_chain`, mutant `aggregation_m_wrong_node_params` | `MODELED` | Enforces that every accepted `N1`, `N2`, and `N3` node proof uses `FieldBabyBear`, `ParamInternal`, and verifies under `universalKeys.internalRecVk`. |
| `NAG-002` | `inv_NAG_002`, action `refuseMissingChildAir`, test `ag_NAG_001_002_003_NodeChainKeyRootAndOmitAirTest`, witness `reach_forgeries_refused`, mutant `aggregation_m_accept_omitted_air` | `MODELED` | Parent node refuses any child proof omitting `StmtAir`, `ProofShapeAir`, or `PublicValuesAir` (`RefMissingRequiredAir`). |
| `NAG-003` | `inv_NAG_003`, actions `proveN1`, `refuseN1WrongKeyRoot`, test `ag_NAG_001_002_003_NodeChainKeyRootAndOmitAirTest`, mutant `aggregation_m_unopened_key_root` | `MODELED` | Every accepted `N1` proof opens `prog_d = P2(DOM_PROG, image_d, plan_d, key_root, machine_version)` matching its program metadata and forwards `key_root` to its parent; mismatched `key_root` witnesses are refused (`RefWrongProgDOpening`). |
| `NAG-004` | `inv_NAG_004`, actions `proveN2Merge`, `proveN3Root`, `proveRunNodeTree`, test `ag_NAG_004_MixedChipSetsMergedTest`, witness `reach_mixed_chipset_merged`, mutant `aggregation_m_mixed_chipset_wrong_keyroot` | `MODELED` | Merges multi-chip-set shards (`chipSetId` `0` and `3`) of one `key_root` under `N2` and `N3`; cross-family state-digest gap documented in Finding `Q-aggregation-3`. |
| `NAG-005` | `inv_NAG_005`, const `MERGE_WORK_MAX_COLS = 5400`, actions `admitMergeWorkKey`, `refuseMergeWorkKeyOverCols`, test `ag_NAG_005_LPC_002_MergeWorkColumnBoundTest`, witness `reach_mw_key_admitted_and_refused`, mutant `aggregation_m_allow_mw_over_5400` | `CONSTANT-CHECKED` | Key generator admits `MERGE-WORK` keys with `colCount <= MERGE_WORK_MAX_COLS` (`5,121` and `5,400`) and refuses `5,401` and `5,417` (`RefMergeWorkCols`). |
| `NAG-006` | `inv_NAG_006`, action `refuseT10qForgery`, test `ag_NAG_006_007_011_T10qForgeriesGp2C21AndHonestInputsTest`, witness `reach_forgeries_refused`, mutant `aggregation_m_accept_t10q_forgery` | `MODELED` | Refuses all four T10q forged inputs (`MW-F01` false carry, `AFK-F7` shape collision, `GP2-F4.2` unbound nominal type, and unlinked 1-child `AFK-F9(b)`); host/ROM vs node-AIR layer misattribution noted in `Q-aggregation-6`. |
| `NAG-007` | `inv_NAG_007`, action `refuseGp2C21Forgery`, test `ag_NAG_006_007_011_T10qForgeriesGp2C21AndHonestInputsTest`, witness `reach_forgeries_refused`, mutant `aggregation_m_gp2_c21_unfixed` | `MODELED` | Refuses the `GP2-C2.1` forged witness (`pubvConstRowChecked`); open unfixed defect in leaf `CoreAir` `R_PUBV` and layer misattribution documented in Finding `Q-aggregation-6`. |
| `NAG-008` | `inv_NAG_008`, actions `proveLRun`, `refuseLWrongOutcome`, test `ag_NAG_008_009_010_LOutcomeChildLinksAndProofBundleTest`, mutant `aggregation_m_l_unbound_outcome` | `MODELED` | `L` accepts single-run terminal `N3` roots only when `lOutcomeD == root.outcomeD` and refuses mismatched outcome witnesses (`RefWrongOutcomeAtL`); conflict with collection-form `L` documented in `Q-aggregation-5`. |
| `NAG-009` | `inv_NAG_009`, actions `proveN1`, `proveN2Merge`, `refuseDisjointOrSwappedChildren`, test `ag_NAG_008_009_010_LOutcomeChildLinksAndProofBundleTest`, mutant `aggregation_m_skip_child_link_check` | `MODELED` | Enforces contiguous child step intervals (`lp.hEnd == rp.hStart`) and linked state digests (`lp.postD == rp.preD`) on `N1`/`N2` segment merges; omitted `N3`/`MERGE-WORK` scope noted in `Q-aggregation-4`. |
| `NAG-010` | `inv_NAG_010`, action `publishProofBundle`, tests `ag_NAG_008_009_010_LOutcomeChildLinksAndProofBundleTest`, `ag_NAG_010_PrematureBundleBlockedTest`, witness `reach_proof_bundle_published`, mutant `aggregation_m_incomplete_proof_bundle` | `MODELED` | Published `ProofBundle` requires completed leaf, `N1`, `N2`, `N3`, and `L`/`L_Fr` entries, matching `proofPathId`/`proofSha256`, `claimPub`, and `keyManifestSha256`. |
| `NAG-011` | `inv_NAG_011`, actions `proveN1`, `proveOneChildMergeWork`, test `ag_NAG_006_007_011_T10qForgeriesGp2C21AndHonestInputsTest`, witness `reach_honest_t10q_accepted` | `MODELED` | Accepts honest nonzero-`reg_root` `N1` witnesses (`AFK-F6`, `regRoot == 77`) preserving `reg_root` across merges and accepts honest linked 1-child `MERGE-WORK` witnesses (`AFK-F9(b)`). |
| `GPH-001` | `inv_GPH_001`, actions `proveRunNodeTree`, `proveLRun`, test `ag_GPH_001_BothRuleFamiliesSameUniversalKeysTest`, witness `reach_both_families_proved_same_keys` | `MODELED` | Proves both `FamInterpreted` (Run 1) and `FamRsl` (Run 2) through `N1`, `N2`, `N3`, `L`, and `L_Fr` under immutable `genesisUniversalKeys` without regeneration. |
| `GPH-002` | `inv_GPH_002`, actions `proveGp2Seal`, `proveGp2SealWithDeps`, `sealRunNodeTree`, `proveGp2JoinSet`, `proveGp2DischargeDeps`, `refuseGp2ForgedDep`, `refuseMixedRegRootJoin`, tests `ag_GPH_002_GP_002_CrossRunConsumeAndForgedDepRefusedTest`, `ag_GPH_002_UnprovedDepDischargeBlockedTest`, witness `reach_gp2_dep_discharged`, mutant `aggregation_m_accept_forged_dep` | `MODELED` | Exercises `SEAL`, `SEAL-WITH-DEPS`, `JOIN`, and `DL-STEP`/`DESCEND` cross-run claim consumption; blocks unproved dependencies and refuses forged dependency witnesses (`RefUnprovedOrUnsealedDep`). |
| `GPH-003` | `inv_GPH_003`, const `KEYS_D_FOREIGN_ROWS`, actions `proveN3ForeignUnit`, `refuseUnlistedFvUnit`, test `ag_GPH_003_004_GP_003_ForeignUnitAndCompactAdapterTest`, witness `reach_fv_unit_and_compact_adapter`, mutant `aggregation_m_accept_unlisted_fv_key` | `CONSTANT-CHECKED` | `N3` accepts `StmtFv` unit proofs iff `fvVkRow` belongs to `universalKeys.foreignVkRows` (`1..KEYS_D_FOREIGN_ROWS`) and refuses unlisted foreign keys (`RefUnlistedForeignKey`); tree-level mismatch with `GPH-004` documented in `Q-aggregation-2`. |
| `GPH-004` | `inv_GPH_004`, const `HAS_X8_COMPACT_VERIFIER`, actions `adaptCompactForeignProof`, `refuseUnboundCompactAdapter`, tests `ag_GPH_003_004_GP_003_ForeignUnitAndCompactAdapterTest`, `ag_EdgeT25_Exact5121AdmittedAndUnbuiltX8BlockedTest`, witness `reach_fv_unit_and_compact_adapter`, mutant `aggregation_m_unbound_foreign_adapter` | `MODELED` | Requires `HAS_X8_COMPACT_VERIFIER`, registered Compact VK, and authenticated `AdapterCompactZkir3` (`compact-zkir3-kzg-bls12381-blake2b/1`) binding in circuit; refuses unregistered or unbound adapter witnesses (`RefUnboundCompactAdapter`). |
| `GPH-005` | `inv_GPH_005`, action `provePcdWorkTree`, test `ag_GPH_005_GP_004_TwoPcdTreeShapesSameClaimTest`, witness `reach_pcd_two_shapes_same_claim`, mutant `aggregation_m_pcd_shape_changes_claim` | `MODELED` | Proves both a balanced tree (`ShapeBalanced`, node `71`) and a chain (`ShapeChain`, node `72`) over the same four `MERGE-WORK` units under universal keys and verifies `claimD` equality (`9501 == 9501`). |
| `GPH-006` | `inv_GPH_006`, actions `proveRunNodeTree`, `proveLRun`, test `ag_GPH_006_SuspendResumeEqualClaimPubTest`, witness `reach_suspended_run_equal_claim_pub`, mutant `aggregation_m_suspend_alters_claim_pub` | `MODELED` | Verifies that suspended-and-resumed Run 4 (`huginn` `[0,8)` + `odin` `[8,12)`) produces the exact same `claimPub` as uninterrupted Run 1 on the same statement. |
| `GPH-007` | `inv_GPH_007`, consts `GP2_CASE_FLOOR = 48`, `X7_CASE_FLOOR = 22`, actions `runGpGateHonest`, `runGpGateTamperedCaseMap`, tests `ag_GPH_007_HonestGpGatePassTest`, `ag_GPH_007_TamperedCaseMapFailsGateTest`, witness `reach_gp_gate_case_map_failed`, mutant `aggregation_m_count_only_gp_gate` | `CONSTANT-CHECKED` | Gate passes iff `caseMapMatchesBaseline` holds and counts meet `(48, 22)`; replacing a refusal case with a duplicate accept case (count stays `48`) fails the gate (`GateFailedCaseMap`). |
| `GPH-008` | `inv_GPH_008`, consts `HAS_L_COLLECTION_KEY`, `CLAIMS_TREE_DEPTH = 20`, actions `buildThreeClaimSetRoot`, `proveLCollectionOverSet`, tests `ag_GPH_008_LPC_003_LCollectionThreeClaimsAndForgedSetRefusedTest`, `ag_EdgeT25_LCollectionBlockedWhenUnkeyedTest`, witness `reach_l_collection_3claims`, mutant `aggregation_m_l_accepts_forged_set_root` | `MODELED` | Proves collection form of `L` over a `SET` root of 3 claims from 3 programs (`nodeId = 52`), exposing `claimsRoot` with verified depth-20 Merkle memberships for all 3 claims. |
| `LPC-001` | `consistency_LPC_001`, `hookRowStatus`, tests `ag_ConsistencyAndSpecContradictionsTest`, `ag_EdgeT25_Exact5121AdmittedAndUnbuiltX8BlockedTest`, `ag_EdgeT25_LCollectionBlockedWhenUnkeyedTest` | `SPEC-CHECKED candidate` | Factory loop-accounting rule requiring `I-13` and the four GP hook rows to be recorded as `FROZEN` (with real-proof evidence) or `FREEZE-PENDING` (with a measured reason) when row `T8b` closes; `consistency_LPC_001` checks that unbuilt `HAS_X8_COMPACT_VERIFIER` / `HAS_L_COLLECTION_KEY` map consistently to `HookFreezePending`. |
| `LPC-002` | `inv_LPC_002`, `consistency_LPC_002_scenario_vs_NAG_005`, consts `MERGE_WORK_MAX_COLS = 5400`, `MW_BASE_COLS = 4988`, `MW_VALUE_DIGEST_DELTA = 62`, `MW_STACKS_DELTA = 71`, actions `admitMergeWorkKey`, `refuseMergeWorkKeyOverCols`, test `ag_NAG_005_LPC_002_MergeWorkColumnBoundTest`, mutant `aggregation_m_allow_mw_over_5400` | `CONSTANT-CHECKED` | Enforces `colCount <= 5,400` when `STACKS` (`+71`) and `VALUE DIGEST` (`+62`) land in the inline kernel (`4,988 + 62 + 71 = 5,121 <= 5,400`); Scenario contradiction (`5,417` raising the bound) reported in Finding `Q-aggregation-1`. |
| `LPC-003` | `inv_LPC_003`, consts `HAS_X8_COMPACT_VERIFIER`, `HAS_L_COLLECTION_KEY`, actions `adaptCompactForeignProof`, `proveLCollectionOverSet`, `refuseLCollectionForgedSetRoot`, test `ag_GPH_008_LPC_003_LCollectionThreeClaimsAndForgedSetRefusedTest`, mutant `aggregation_m_l_accepts_forged_set_root` | `MODELED` | Exercises in-circuit `X8` Compact verification and `L` over `SET` root while refusing forged `SET` roots (`RefForgedSetRootAtL`); omission of `X12` (promised by `GPH-002` Status) reported in Finding `Q-aggregation-5`. |

### 1.2 Adopted EARS IDs Cited by Aggregation Specs

| ID | Owning Change | Artifact (`aggregation.qnt` / `aggregation_test.qnt`) | Kind | Note |
|---|---|---|---|---|
| `CPR-005` | `sherlock-processor-final-20261111` | `inv_CPR_005`, action `runGpGateFailedDerived`, test `ag_DG_010_CPR_005_FailedDerivedGateKeepsGpOpenTest`, mutant `aggregation_m_narrow_gp_on_gate_fail` | `MODELED` | Ensures a failed derived gate leaves GP targets open and never narrows `GP-001..GP-004` or `GP-ZK` (`not(gateStatus.gpTargetsNarrowed)`). |
| `DG-010` | `sherlock-processor-final-20261111` | `inv_DG_010`, action `runGpGateFailedDerived`, test `ag_DG_010_CPR_005_FailedDerivedGateKeepsGpOpenTest`, mutant `aggregation_m_narrow_gp_on_gate_fail` | `MODELED` | Derived gates test GP capability without narrowing `GP-001..GP-004` or `GP-ZK` upon gate failure. |
| `GP-002` | `sherlock-vm-consensus-20261107` | `inv_GP_002`, actions `proveGp2Seal`, `proveGp2SealWithDeps`, `proveGp2JoinSet`, `proveGp2DischargeDeps`, `refuseGp2ForgedDep`, tests `ag_GPH_002_GP_002_CrossRunConsumeAndForgedDepRefusedTest`, `ag_GPH_002_UnprovedDepDischargeBlockedTest`, mutant `aggregation_m_accept_forged_dep` | `MODELED` | Consumed claims in `StmtSet` are terminal sealed claims verified in-circuit rather than raw intermediate segments. |
| `GP-003` | `sherlock-vm-consensus-20261107` | `inv_GP_003`, actions `adaptCompactForeignProof`, `refuseUnboundCompactAdapter`, test `ag_GPH_003_004_GP_003_ForeignUnitAndCompactAdapterTest`, mutant `aggregation_m_unbound_foreign_adapter` | `MODELED` | Foreign proofs contributing `StmtSet` claims bind to `REGISTERED_COMPACT_VK` under authenticated `VALID_REG_ROOT` and forbid unbound statement relabeling. |
| `GP-004` | `sherlock-vm-consensus-20261107` | `inv_GP_004`, action `provePcdWorkTree`, test `ag_GPH_005_GP_004_TwoPcdTreeShapesSameClaimTest`, mutant `aggregation_m_pcd_shape_changes_claim` | `MODELED` | `shapeD` uses address-free descriptors (`not(shapeUsesPhysState)`) and differs across `ShapeBalanced` vs `ShapeChain` (`8101 != 8102`) while yielding identical `claimD` (`9501`). |

---

## 2. Findings (`Q-aggregation-1` .. `Q-aggregation-6`)

### `Q-aggregation-1` — Direct Contradiction Between `LPC-002` Scenario and `NAG-005` / `LPC-002` Requirement on the 5,400-Column `MERGE-WORK` Bound

- **IDs**: `LPC-002`, `NAG-005`
- **Quoted Sentences**:
  - `NAG-005` (`sherlock-e2e-node-aggregation/spec.md:56, 61, 63`):
    > *"If a MERGE-WORK key has more than 5,400 columns, then the key generator SHALL refuse the key."*
    > *"The bound does not change without the owner."*
    > *"The earlier figure 5,417 (LOOP lead decisions) is superseded."*
  - `LPC-002` (`sherlock-loop-completion/spec.md:20, 27-29`):
    > *"When the STACKS AIR and VALUE DIGEST land in the inline kernel, the MERGE-WORK key SHALL have at most 5,400 columns."*
    > *"GIVEN a MERGE-WORK key with STACKS and VALUE DIGEST"*
    > *"WHEN the key has 5,417 columns"*
    > *"THEN row T8c splits CORE again or raises the bound with a measurement."*
- **Witness State / Check**:
  - Pure consistency check `consistency_LPC_002_scenario_vs_NAG_005` (`5417 <= MERGE_WORK_MAX_COLS`) evaluates to `false` in `aggregation_main` (`MERGE_WORK_MAX_COLS = 5400`), verified in `ag_ConsistencyAndSpecContradictionsTest`.
  - In `ag_NAG_005_LPC_002_MergeWorkColumnBoundTest`, candidate key `mwKeyId = 4` (`colCount = 5417`) is refused with `RefMergeWorkCols`; admitting it in mutant `aggregation_m_allow_mw_over_5400` violates both `inv_NAG_005` and `inv_LPC_002` (`exit 1`).
- **What Closes It**:
  - Amend `LPC-002`'s Scenario in `sherlock-loop-completion/spec.md` to remove *"or raises the bound with a measurement"* so that any >5,400-column key is refused by keygen unless the owner explicitly approves a bound change, matching `NAG-005` and `LPC-002`'s own `<= 5,400` SHALL clause.

### `Q-aggregation-2` — Foreign-Verifier Tree-Level Contradiction Between `GPH-003` (`N3` Direct Child) and `GPH-004` Scenario / `GPH-002` / `GP-003` (Consumed Inner-Run Claim)

- **IDs**: `GPH-003`, `GPH-004`, `GPH-002`, `GP-003`, `NAG-008`
- **Quoted Sentences**:
  - `GPH-003` (`sherlock-e2e-gp-hooks/spec.md:36`):
    > *"When a foreign-verifier unit proof is a child of N3, the N3 node SHALL accept the unit under the foreign rows of `keys_d`."*
  - `GPH-004` Scenario (`sherlock-e2e-gp-hooks/spec.md:54-56`):
    > *"GIVEN a Compact proof of class `compact-zkir3-kzg-bls12381-blake2b/1`"*
    > *"WHEN a Sherlock run consumes it"*
    > *"THEN the foreign verifier checks it in circuit and the registry row names the class."*
  - `GP-003` (`sherlock-vm-consensus-20261107/specs/sherlock-general-purpose/spec.md:52`):
    > *"Where a proof from another circuit is verified inside a Sherlock proof and contributes a claim, the system SHALL verify that circuit's proof in a Sherlock leaf or node, check membership of `(circuit_vk_d, statement_adapter_id)` in an authenticated circuit registry, and apply only that adapter when forming the claim."*
- **Witness State / Check**:
  - `consistency_GPH_003_vs_GPH_004_consumption` (`StmtFv == StmtSet`) evaluates to `false` in `aggregation_main` (`ag_ConsistencyAndSpecContradictionsTest`).
  - `GPH-003` places foreign-verifier unit proofs exclusively as direct children of root `N3` (`StmtFv`), authenticated only against universal `keys_d` (no `reg_root` or `statement_adapter_id`), whereas `GPH-004`'s Scenario and `GP-003` require a Sherlock run to *consume* the foreign claim via authenticated registry `reg_root` + statement adapter into GP-2 `StmtSet` dependency discharge (`SEAL-WITH-DEPS`, `JOIN`, `DL-STEP`, `DESCEND` below `N3`). Neither `GPH-003` nor `NAG-008` specifies how an `N3` node merges an `N2` Sherlock run with an `StmtFv` child into a root statement accepted by `L`.
- **What Closes It**:
  - Specify explicitly whether foreign-verifier units enter GP-2 `JOIN` / `DL-STEP` / `DESCEND` as sealed `StmtSet` claims via `(circuit_vk_d, statement_adapter_id)` in `reg_root` (at `N1`/`N2`/`N3`), or define the exact `N3` multi-child statement rule and `L` public-input layout when an `StmtFv` unit sits directly under `N3`.

### `Q-aggregation-3` — Cross-Family State-Linking and Node-Layer Gap in `NAG-004` vs `NAG-003`, `NAG-009`, and `DDD.md` (`EUI-001`)

- **IDs**: `NAG-004`, `NAG-003`, `NAG-009`
- **Quoted Sentences**:
  - `NAG-004` (`sherlock-e2e-node-aggregation/spec.md:43, 49-51`):
    > *"When the leaves of one run use more than one chip set of one `key_root`, the node prover SHALL merge the leaves under N2 and N3."*
    > *"GIVEN one run with a chip set 0 leaf and a chip set 3 leaf"*
    > *"WHEN N2 merges them"*
    > *"THEN N2 accepts and the stock verifier accepts N2."*
  - `NAG-009` (`sherlock-e2e-node-aggregation/spec.md:105`):
    > *"The N1 and N2 merge constraints SHALL check that the step interval of each child starts where the interval of the child before it ends and that its `pre_d` equals the `post_d` of the child before it."*
  - `DDD.md` §3 (`architecture/DDD.md:47`):
    > *"one rule family per program (EUI-001)"*
- **Witness State / Check**:
  - Modeled in Run 3 (`proveN1(5)` with `chipSetId = 0`, `proveN1(6)` with `chipSetId = 3` -> `proveN2Merge(3)` -> `proveN3Root(3)` in `ag_NAG_004_MixedChipSetsMergedTest`).
  - Two specification gaps surface when constructing Run 3: (1) `N2` merges `N1` node proofs (after `N1` opens `prog_d` and forwards `key_root` per `NAG-003`), not raw leaf proofs directly as `NAG-004`'s Scenario states; (2) in the architecture, chip set `0` is an interpreted-VM chip set carrying `(pre_d, post_d)` VM state digests, whereas chip set `3` (`R_SL`, `DDD.md:18`) is a straight-line circuit chip set carrying `(comm, W)` with no VM `(pre_d, post_d)` state digest to satisfy `NAG-009`'s `post_d == pre_d` check.
- **What Closes It**:
  - State in `NAG-004` that `N1` proves each shard under its chip-set row in `key_root` and `N2`/`N3` merge the resulting `N1`/`N2` proofs, and replace `chip set 3` in `NAG-004`'s Scenario with a second chip set of the same rule family (e.g., chip set `0` and chip set `1` / `2` of the interpreted VM) or define a cross-chip-set state-boundary digest rule.

### `Q-aggregation-4` — Incomplete Child-Linking Scope in `NAG-009` (`N3` and 2-Child `MERGE-WORK` Omitted)

- **IDs**: `NAG-009`, `NAG-001`, `NAG-004`, `NAG-006`, `NAG-011`
- **Quoted Sentences**:
  - `NAG-009` (`sherlock-e2e-node-aggregation/spec.md:105`):
    > *"The N1 and N2 merge constraints SHALL check that the step interval of each child starts where the interval of the child before it ends and that its `pre_d` equals the `post_d` of the child before it."*
  - `NAG-004` (`sherlock-e2e-node-aggregation/spec.md:43`):
    > *"When the leaves of one run use more than one chip set of one `key_root`, the node prover SHALL merge the leaves under N2 and N3."*
- **Witness State / Check**:
  - `inv_NAG_009` follows `NAG-009` literally (`Set(TierN1, TierN2).contains(np.tier) and np.gp2Mode == ModeSegMerge`). Because `NAG-009` names only `N1` and `N2`, multi-child segment merges at `N3` (referenced in `NAG-004`) and 2-child `MERGE-WORK` / `NW` / `NW2` work-tree merges (`NAG-006` and `NAG-011` name only the 1-child unlinked case `AFK-F9(b)`) have no EARS child-continuity requirement in `sherlock-e2e-node-aggregation`.
- **What Closes It**:
  - Extend `NAG-009` to cover every `SEG_MERGE` node across `N1`, `N2`, `N3` and every multi-child `MERGE-WORK` / `NW` / `NW2` work-merge node.

### `Q-aggregation-5` — Single-Run vs Collection-Form `L` Statement/Bundle Mismatch and Omitted `X12` Obligation in `LPC-003`

- **IDs**: `NAG-008`, `NAG-010`, `GPH-008`, `GPH-002`, `LPC-003`
- **Quoted Sentences**:
  - `NAG-008` (`sherlock-e2e-node-aggregation/spec.md:93`):
    > *"If a node root carries an outcome word that differs from the outcome word of a run, then the L circuit SHALL refuse the node root."*
  - `GPH-008` (`sherlock-e2e-gp-hooks/spec.md:100`):
    > *"When the SET root of a collection of claims is offered to L, the collection form of L SHALL verify the SET root and expose `claims_root` as a public word."*
  - `GPH-002` Status (`sherlock-e2e-gp-hooks/spec.md:27`):
    > *"What remains is a multi-run integration test that exercises cross-run consumption through the host pipeline and the X12 discharge model in the circuit (`docs/specs/GP2.md`); both are LPC-003."*
  - `LPC-003` (`sherlock-loop-completion/spec.md:32`):
    > *"When row T8c runs, the proving stack SHALL build the X8 Compact foreign verifier in circuit and prove L over a SET root."*
- **Witness State / Check**:
  - Modeled via separate state maps `lRunProofs` (single-run `L` bound to `outcomeD` and `ProofBundle` per `NAG-008`/`NAG-010`) and `lCollectionProofs` (collection-form `L` over 3-claim `SET` root `nodeId = 52` exposing `claimsRoot` and `regPub` per `GPH-008`/`LPC-003`).
  - Two specification gaps appear: (1) `NAG-008` and `NAG-010` assume `L` always checks a single run's `outcomeD` and emits a single-`claim_pub` `ProofBundle`, leaving the mode-dispatch tag between single-run `L` and collection-form `L` (under the same universal `L` VK) and the `ProofBundle` format for `SET` collections unspecified; (2) `GPH-002` Status defers the `X12` circuit discharge model to `LPC-003`, but `LPC-003`'s EARS text omits `X12` completely.
- **What Closes It**:
  - Add explicit mode-tag discrimination in `NAG-008` / `GPH-008` (single-run `CLAIM` mode vs collection `SET` mode under the universal `L` key), extend `NAG-010`'s `ProofBundle` specification for `SET` collection proofs, and add `X12` cross-run discharge explicitly to `LPC-003`.

### `Q-aggregation-6` — Misattributed Circuit Layer for `GP2-F4.2` and `GP2-C2.1` in `NAG-006` / `NAG-007` and Open Unfixed Defect `GP2-C2.1`

- **IDs**: `NAG-006`, `NAG-007`
- **Quoted Sentences**:
  - `NAG-006` (`sherlock-e2e-node-aggregation/spec.md:69`):
    > *"If a node witness carries any of the forged inputs of T10q findings MW-F01, AFK-F7, GP2-F4.2, or the unlinked one-child case of AFK-F9(b), then the node AIRs SHALL refuse the witness."*
  - `NAG-007` (`sherlock-e2e-node-aggregation/spec.md:81, 87-90`):
    > *"If a node witness carries the forged input of counterexample GP2-C2.1, then the node AIRs SHALL refuse the witness."*
    > *"GIVEN a witness shaped after GP2-C2.1"*
    > *"WHEN the node AIR checks the public value row"*
    > *"THEN at least one constraint is non-zero."*
- **Witness State / Check**:
  - Cited specification source `tests/quint-regressions/COUNTEREXAMPLES.md:74-75` states that `GP2-F4.2` was closed in **host `leaf::halted_vd` and ROM nominal-type encoding** (*"host and ROM only, no circuit key changes"*) and that `GP2-C2.1` is an **OPEN CONFIRMED defect** in the **leaf** `CoreAir` instruction rule `R_PUBV` (`crates/sherlock-leaf/src/core.rs:439-449`, which fails to check a ROM constant row), not a node-AIR constraint.
  - Mutant `aggregation_m_gp2_c21_unfixed` models the current unfixed `GP2-C2.1` state (`pubvConstRowChecked = false`) and immediately violates `inv_NAG_007` (`exit 1`, `[violation] Found an issue`).
- **What Closes It**:
  - Re-scope `GP2-C2.1` (and the leaf/ROM portion of `GP2-F4.2`) to the Leaf Proving specification (`CoreAir` `R_PUBV` ROM constant-row constraint + leaf key regeneration if leaf columns/constraints change), and close `GP2-C2.1` in `sherlock-leaf` with a negative witness regression test.

---

## 3. Safety Mutants Table

Every refusal and binding invariant was tested against a dedicated mutant module (`--max-samples=1000 --max-steps=15 --seed=0x20261115`) in which only the targeted guard is weakened while all honest actions remain enabled (non-vacuous).

| Mutant Module | Target Invariant | Observed Exit Code | Observed Output Summary | Non-Vacuity Note |
|---|---|---|---|---|
| `aggregation_m_wrong_node_params` | `inv_NAG_001` | `1` | `[violation] Found an issue (701ms at 1 traces/second).` | Admits an `N1` proof over `FieldWrong` / `ParamWrong`; honest `N1..N3` transitions remain active. |
| `aggregation_m_accept_omitted_air` | `inv_NAG_002` | `1` | `[violation] Found an issue (433ms at 2 traces/second).` | Disables `RefMissingRequiredAir` refusal and accepts an `N3` proof with `allChildrenHaveRequiredAirs = false`. |
| `aggregation_m_unopened_key_root` | `inv_NAG_003` | `1` | `[violation] Found an issue (423ms at 2 traces/second).` | Accepts an `N1` proof forwarding `keyRoot = 9999 != programCatalog(1).keyRoot`. |
| `aggregation_m_mixed_chipset_wrong_keyroot` | `inv_NAG_004` | `1` | `[violation] Found an issue (427ms at 2 traces/second).` | Accepts an `N2` proof merging `Set(0, 3)` under mismatched `forwardedKeyRoot = 9999`. |
| `aggregation_m_allow_mw_over_5400` | `inv_NAG_005` | `1` | `[violation] Found an issue (628ms at 2 traces/second).` | Removes `colCount <= MERGE_WORK_MAX_COLS` guard in `admitMergeWorkKey`, admitting `5,401`/`5,417`-column keys. |
| `aggregation_m_allow_mw_over_5400` | `inv_LPC_002` | `1` | `[violation] Found an issue (493ms at 2 traces/second).` | Same mutant admits inline-kernel `STACKS + VALUE DIGEST` keys exceeding `5,400` columns. |
| `aggregation_m_accept_t10q_forgery` | `inv_NAG_006` | `1` | `[violation] Found an issue (582ms at 2 traces/second).` | Accepts `ForgeryMwF01FalseCarry` (`whiLimbs16Bit = false`) instead of refusing with `RefT10qMwF01`. |
| `aggregation_m_gp2_c21_unfixed` | `inv_NAG_007` | `1` | `[violation] Found an issue (611ms at 2 traces/second).` | Models unfixed `GP2-C2.1` (`pubvConstRowChecked = false`), violating `inv_NAG_007`. |
| `aggregation_m_l_unbound_outcome` | `inv_NAG_008` | `1` | `[violation] Found an issue (423ms at 2 traces/second).` | Allows `L` to accept a single-run `N3` root (`outcomeD = 501`) with altered `lOutcomeD = 502`. |
| `aggregation_m_skip_child_link_check` | `inv_NAG_009` | `1` | `[violation] Found an issue (656ms at 2 traces/second).` | Accepts segment merge with `intervalsContiguous = false` and `stateDigestsLinked = false`. |
| `aggregation_m_incomplete_proof_bundle` | `inv_NAG_010` | `1` | `[violation] Found an issue (466ms at 2 traces/second).` | Allows `publishProofBundle` before `proveLRun`, emitting a bundle missing `N1..N3` and `L`/`L_Fr` entries. |
| `aggregation_m_accept_forged_dep` | `inv_GPH_002` | `1` | `[violation] Found an issue (560ms at 2 traces/second).` | Accepts `ModeDlStepDescend` node with `depsVerifiedInSet = false`. |
| `aggregation_m_accept_forged_dep` | `inv_GP_002` | `1` | `[violation] Found an issue (622ms at 2 traces/second).` | Same mutant accepts `StmtSet` node with `depsAllSealedTerminal = false`. |
| `aggregation_m_accept_unlisted_fv_key` | `inv_GPH_003` | `1` | `[violation] Found an issue (443ms at 2 traces/second).` | Accepts `StmtFv` child at `N3` with `fvVkRow = 99` outside `universalKeys.foreignVkRows`. |
| `aggregation_m_unbound_foreign_adapter` | `inv_GPH_004` | `1` | `[violation] Found an issue (556ms at 2 traces/second).` | Accepts Compact foreign proof with `UNREGISTERED_COMPACT_VK` and `AdapterUnboundRelabel`. |
| `aggregation_m_unbound_foreign_adapter` | `inv_GP_003` | `1` | `[violation] Found an issue (328ms at 3 traces/second).` | Same mutant breaks registry/adapter binding in `inv_GP_003`. |
| `aggregation_m_pcd_shape_changes_claim` | `inv_GPH_005` | `1` | `[violation] Found an issue (576ms at 56 traces/second).` | Makes `ShapeChain` (`72`) compute `claimD = 9599 != 9501` (`ShapeBalanced`, `71`) on the same 4 work units. |
| `aggregation_m_pcd_shape_changes_claim` | `inv_GP_004` | `1` | `[violation] Found an issue (1978ms at 16 traces/second).` | Same mutant breaks shape-independent claim equivalence in `inv_GP_004`. |
| `aggregation_m_suspend_alters_claim_pub` | `inv_GPH_006` | `1` | `[violation] Found an issue (685ms at 3 traces/second).` |Perturbs `claimPub` (`+999`) when `suspendedAndResumed == true` on Run 4 (`proveLRun(4)`). |
| `aggregation_m_count_only_gp_gate` | `inv_GPH_007` | `1` | `[violation] Found an issue (547ms at 18 traces/second).` | Marks `gateStatus.verdict = GatePassed` when count is `48` even though `caseMapMatchesBaseline == false`. |
| `aggregation_m_narrow_gp_on_gate_fail` | `inv_DG_010` | `1` | `[violation] Found an issue (328ms at 3 traces/second).` | Sets `gpTargetsNarrowed = true` when `runGpGateFailedDerived` runs. |
| `aggregation_m_narrow_gp_on_gate_fail` | `inv_CPR_005` | `1` | `[violation] Found an issue (432ms at 2 traces/second).` | Same mutant breaks `inv_CPR_005` (`not(gateStatus.gpTargetsNarrowed)`). |
| `aggregation_m_l_accepts_forged_set_root` | `inv_GPH_008` | `1` | `[violation] Found an issue (493ms at 2 traces/second).` | Accepts an `L` collection proof over nonexistent/forged `setNodeId = 999` with `allMembershipsVerified = false`. |
| `aggregation_m_l_accepts_forged_set_root` | `inv_LPC_003` | `1` | `[violation] Found an issue (427ms at 2 traces/second).` | Same mutant breaks `inv_LPC_003` forged-`SET` refusal guarantee. |

---

## 4. State Vocabulary for Replay

### 4.1 State Variables (`var`)

In ITF JSON files (`../traces/aggregation_main/t_{seq}.itf.json`), every variable is serialized under prefix `aggregation_main::aggregation::<var>`.

| Variable | Quint Type | ITF Key | Meaning |
|---|---|---|---|
| `lastOp` | `str` | `aggregation_main::aggregation::lastOp` | Name of the action taken in the transition (`"init"` in state `0`). |
| `lastArgs` | `StepArgs` | `aggregation_main::aggregation::lastArgs` | Structured action arguments `{ primaryId: int, secondaryId: int, forgery: ForgeryKind, refusal: RefusalCode }` for deterministic oracle replay. |
| `universalKeys` | `UniversalAggKeys` | `aggregation_main::aggregation::universalKeys` | Immutable universal verifier key set (`internalRecVk = 9101`, `lVk = 9102`, `lFrVk = 9103`, `keyManifestSha256 = 9200`, `nodeRowCount`, `foreignVkRows`). |
| `mwKeyRegistry` | `int -> MergeWorkKeyRec` | `aggregation_main::aggregation::mwKeyRegistry` | Admitted `MERGE-WORK` keys (`mwKeyId -> { mwKeyId, hasStacksAir, hasValueDigest, colCount }`). |
| `nodeProofs` | `int -> NodeProofRec` | `aggregation_main::aggregation::nodeProofs` | Verified node proofs indexed by `nodeId` (`11..20` `N1`, `21..25` `N2`, `31..35` `N3`, `41..43` `SEAL`, `45` `SEAL-WITH-DEPS`, `51..52` `JOIN`, `55` `DL-STEP/DESCEND`, `61..62` `N3` foreign units, `65` adapted Compact `StmtSet`, `70` 1-child `MERGE-WORK`, `71..72` balanced/chain PCD roots). |
| `lRunProofs` | `int -> LRunProofRec` | `aggregation_main::aggregation::lRunProofs` | Accepted single-run `L` and `L_Fr` proofs indexed by `runId` (`1..4`). |
| `lCollectionProofs` | `int -> LCollectionProofRec` | `aggregation_main::aggregation::lCollectionProofs` | Accepted collection-form `L` proofs over `StmtSet` roots indexed by `setNodeId` (`41, 51, 52, 55, 65`). |
| `proofBundles` | `int -> ProofBundleRec` | `aggregation_main::aggregation::proofBundles` | Published end-to-end `ProofBundle` artifacts indexed by `runId` (`1..4`). |
| `gateStatus` | `GateStatusRec` | `aggregation_main::aggregation::gateStatus` | Derived GP regression gate verdict, `gp2`/`x7` case counts, baseline case-map flag, and `gpTargetsNarrowed` flag. |
| `refusalLog` | `Set[RefusalEvent]` | `aggregation_main::aggregation::refusalLog` | Accumulated `{ code: RefusalCode, targetId: int }` records for refused forged/invalid inputs. |

### 4.2 `lastOp` Values and Implemented EARS IDs

| `lastOp` Value | `lastArgs` Convention | Implemented EARS IDs |
|---|---|---|
| `"init"` | `{ primaryId: 0, secondaryId: 0, forgery: ForgeryNone, refusal: RefNone }` | Genesis state initialization (`GPH-001`, `GPH-003`) |
| `"admitMergeWorkKey"` | `primaryId = kId` (`1..2` in honest instances) | `NAG-005`, `LPC-002` |
| `"refuseMergeWorkKeyOverCols"` | `primaryId = kId` (`3..4`), `refusal = RefMergeWorkCols` | `NAG-005`, `LPC-002` |
| `"proveN1"` | `primaryId = n1Id` (`1..10`), `secondaryId = 10 + n1Id` | `NAG-001`, `NAG-002`, `NAG-003`, `NAG-009`, `NAG-011`, `GPH-001` |
| `"proveN2Merge"` | `primaryId = rId` (`1..5`), `secondaryId = 20 + rId` | `NAG-001`, `NAG-002`, `NAG-004`, `NAG-009`, `GPH-001`, `GPH-006` |
| `"proveN3Root"` | `primaryId = rId` (`1..5`), `secondaryId = 30 + rId` | `NAG-001`, `NAG-002`, `NAG-004`, `GPH-001`, `GPH-006` |
| `"proveRunNodeTree"` | `primaryId = rId` (`1..5`), `secondaryId = 30 + rId` | `NAG-001`, `NAG-002`, `NAG-003`, `NAG-004`, `NAG-009`, `NAG-011`, `GPH-001`, `GPH-006` |
| `"proveLRun"` | `primaryId = rId` (`1..4`), `secondaryId = 30 + rId` | `NAG-008`, `GPH-001`, `GPH-006` |
| `"publishProofBundle"` | `primaryId = rId` (`1..4`), `secondaryId = 30 + rId` | `NAG-010` |
| `"proveGp2Seal"` | `primaryId = rId` (`1..3`), `secondaryId = 40 + rId` | `GPH-002`, `GP-002` |
| `"proveGp2SealWithDeps"` | `primaryId = 5`, `secondaryId = 45` | `GPH-002`, `GP-002` |
| `"sealRunNodeTree"` | `primaryId = rId` (`1, 2, 3, 5`), `secondaryId = 41, 42, 43, 45` | `NAG-001`, `NAG-003`, `NAG-009`, `GPH-002`, `GP-002` |
| `"proveGp2JoinSet"` | `primaryId = joinKind` (`1 -> 51`, `2 -> 52`), `secondaryId = outId` | `GPH-002`, `GP-002`, `GPH-008` |
| `"buildThreeClaimSetRoot"` | `primaryId = 3`, `secondaryId = 52` | `GPH-002`, `GP-002`, `GPH-008` |
| `"proveGp2DischargeDeps"` | `primaryId = setNodeId` (`41, 51, 52`), `secondaryId = 55` | `GPH-002`, `GP-002` |
| `"proveLCollectionOverSet"` | `primaryId = setNodeId` (`41, 51, 52, 55, 65`), `secondaryId = setNodeId` | `GPH-008`, `LPC-003` |
| `"proveN3ForeignUnit"` | `primaryId = fvRow` (`1..KEYS_D_FOREIGN_ROWS`), `secondaryId = 60 + fvRow` | `GPH-003` |
| `"adaptCompactForeignProof"` | `primaryId = 701`, `secondaryId = 65` | `GPH-004`, `GP-003`, `LPC-003` |
| `"proveOneChildMergeWork"` | `primaryId = 11`, `secondaryId = 70` | `NAG-011` |
| `"provePcdWorkTree"` | `primaryId = shapeChoice` (`1 -> 71`, `2 -> 72`), `secondaryId = 70 + shapeChoice` | `GPH-005`, `GP-004` |
| `"runGpGateHonest"` | `primaryId = 1` | `GPH-007` |
| `"runGpGateTamperedCaseMap"` | `primaryId = 2` | `GPH-007` |
| `"runGpGateFailedDerived"` | `primaryId = 3` | `DG-010`, `CPR-005` |
| `"refuseMissingChildAir"` | `primaryId = 91`, `forgery = ForgeryOmitRequiredAir`, `refusal = RefMissingRequiredAir` | `NAG-002` |
| `"refuseN1WrongKeyRoot"` | `primaryId = 92`, `forgery = ForgeryWrongKeyRoot`, `refusal = RefWrongProgDOpening` | `NAG-003` |
| `"refuseT10qForgery"` | `primaryId = 93`, `forgery` in `T10Q_FORGERIES`, `refusal = refusalOfT10q(forgery)` | `NAG-006` |
| `"refuseGp2C21Forgery"` | `primaryId = 94`, `forgery = ForgeryGp2C21PubvConstRow`, `refusal = RefGp2C21ConstRow` | `NAG-007` |
| `"refuseLWrongOutcome"` | `primaryId = 1`, `secondaryId = 31`, `forgery = ForgeryWrongLOutcome`, `refusal = RefWrongOutcomeAtL` | `NAG-008` |
| `"refuseDisjointOrSwappedChildren"` | `primaryId = 12`, `secondaryId = 11`, `forgery = ForgeryDisjointOrSwappedLink`, `refusal = RefDisjointOrSwappedChildren` | `NAG-009` |
| `"refuseGp2ForgedDep"` | `primaryId = 55`, `forgery = ForgeryUnprovedDep`, `refusal = RefUnprovedOrUnsealedDep` | `GPH-002`, `GP-002` |
| `"refuseMixedRegRootJoin"` | `primaryId = 59`, `refusal = RefMixedRegRootJoin` | `GPH-002`, `NAG-011` (`AFK-F3`) |
| `"refuseUnlistedFvUnit"` | `primaryId = 99`, `forgery = ForgeryUnlistedFvKey`, `refusal = RefUnlistedForeignKey` | `GPH-003` |
| `"refuseUnboundCompactAdapter"` | `primaryId = 799`, `forgery = ForgeryUnboundForeignAdapter`, `refusal = RefUnboundCompactAdapter` | `GPH-004`, `GP-003` |
| `"refuseLCollectionForgedSetRoot"` | `primaryId = 88`, `forgery = ForgeryForgedSetRoot`, `refusal = RefForgedSetRootAtL` | `GPH-008`, `LPC-003` |
| `"acceptMutantForgery"` | `primaryId = 99` (enabled **only** in `aggregation_m_*` mutant modules; disabled in all honest instances) | Mutant injection harness |

### 4.3 Instance Constants

| Constant | `aggregation_main` | `aggregation_edge_t25_pre_t8c` | `aggregation_edge_exact_5400` | `aggregation_m_<name>` (18 mutants) |
|---|---|---|---|---|
| `MERGE_WORK_MAX_COLS` | `5400` | `5121` | `5400` | `5400` |
| `MW_BASE_COLS` | `4988` | `4988` | `5267` | `4988` |
| `MW_VALUE_DIGEST_DELTA` | `62` | `62` | `62` | `62` |
| `MW_STACKS_DELTA` | `71` | `71` | `71` | `71` |
| `KEYS_D_NODE_ROWS` | `7` | `7` | `7` | `7` |
| `KEYS_D_FOREIGN_ROWS` | `2` | `1` | `2` | `2` |
| `GP2_CASE_FLOOR` | `48` | `4` | `48` | `48` |
| `X7_CASE_FLOOR` | `22` | `2` | `22` | `22` |
| `CLAIMS_TREE_DEPTH` | `20` | `2` | `20` | `20` |
| `HAS_X8_COMPACT_VERIFIER` | `true` | `false` | `true` | `true` |
| `HAS_L_COLLECTION_KEY` | `true` | `false` | `true` | `true` |
| `MUTANT` | `"none"` | `"none"` | `"none"` | `"<name>"` |

---

## 5. Model Abstractions

1. **Cryptographic Hashes & Commitments (`Poseidon2`, `SHA-256`, `Blake2b`)**: Modeled as collision-free deterministic integer encodings (`computeProgDigest`, `computeClaimDigest`, `computeClaimPub`, `computeSetRoot`) over small integer IDs rather than BabyBear/BN254 sponge permutations or 256-bit byte arrays.
2. **STARK, Groth16/BN254, and KZG/BLS12-381 Proofs & Verifier Keys**: Abstracted as typed records (`NodeProofRec`, `LRunProofRec`, `LCollectionProofRec`) carrying explicit algebraic/circuit property fields (`starkField`, `paramSet`, `verifiedVk`, `stockVerified`, `allChildrenHaveRequiredAirs`, `whiLimbs16Bit`, `shapeBindsCut`, `nominalTypeBound`, `pubvConstRowChecked`, `adapterBoundInCircuit`).
3. **Finite Program, Run, and Shard Catalog**: Bounded to 4 representative programs (`Prog 1` interpreted, `Prog 2` `R_SL`, `Prog 3` multi-chip-set under one `key_root`, `Prog 4` cross-run consumer), 6 runs (`Run 1` uninterrupted, `Run 2` `R_SL`, `Run 3` mixed chip sets `0` & `3`, `Run 4` suspended on `huginn` and resumed on `odin`, `Run 5` consumer with `deps`, `Run 6` non-terminal prefix), and 10 `N1` shards.
4. **Depth-20 `SET` Merkle Tree (`GPH-008`, `GP-002`)**: Abstracted as a finite `claimSet: Set[int]` paired with deterministic `claimsRoot = computeSetRoot(claimSet)` and `treeDepth = CLAIMS_TREE_DEPTH`; membership verification checks `claimSet.contains(d)` rather than explicit 20-sibling hash paths.
5. **Derived GP Gate Case Map (`GPH-007`)**: Abstracted as `{ verdict, gp2Count, x7Count, caseMapMatchesBaseline, gpTargetsNarrowed }` rather than enumerating all 70 (`48 + 22`) individual Rust test function names.

---

## 6. Commands

Run all commands below from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/aggregation/quint`:

```bash
# 1. Typecheck specification and test modules
quint typecheck aggregation.qnt
# Expected exit code: 0
# Printed summary: (empty stdout on clean typecheck)

quint typecheck aggregation_test.qnt
# Expected exit code: 0
# Printed summary: (empty stdout on clean typecheck)

# 2. Deterministic requirement & edge unit tests
quint test aggregation_test.qnt --main=aggregation_test --match='ag_.*' --seed=0x20261115
# Expected exit code: 0
# Printed summary: 17 passing (10796ms)

quint test aggregation_test.qnt --main=aggregation_edge_t25_test --match='ag_.*' --seed=0x20261115
# Expected exit code: 0
# Printed summary: 2 passing (1500ms)

# 3. Main and edge instance invariant checks
quint run aggregation.qnt --main=aggregation_main --invariant=inv --max-samples=2000 --max-steps=15 --seed=0x20261115
# Expected exit code: 0
# Printed summary: [ok] No violation found (36494ms at 55 traces/second).

quint run aggregation.qnt --main=aggregation_edge_t25_pre_t8c --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 0
# Printed summary: [ok] No violation found (22404ms at 45 traces/second).

quint run aggregation.qnt --main=aggregation_edge_exact_5400 --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 0
# Printed summary: [ok] No violation found (16570ms at 60 traces/second).

# 4. Negated reachability witnesses (all 13 MUST exit 1 with [violation])
quint run aggregation.qnt --main=aggregation_main --invariant=reach_n1_n2_n3_chain --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (1059ms at 1 traces/second).

quint run aggregation.qnt --main=aggregation_main --invariant=reach_mixed_chipset_merged --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (686ms at 1 traces/second).

quint run aggregation.qnt --main=aggregation_main --invariant=reach_both_families_proved_same_keys --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (385ms at 13 traces/second).

quint run aggregation.qnt --main=aggregation_main --invariant=reach_suspended_run_equal_claim_pub --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (422ms at 38 traces/second).

quint run aggregation.qnt --main=aggregation_main --invariant=reach_proof_bundle_published --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (985ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_main --invariant=reach_honest_t10q_accepted --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (800ms at 6 traces/second).

quint run aggregation.qnt --main=aggregation_main --invariant=reach_gp2_dep_discharged --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (434ms at 7 traces/second).

quint run aggregation.qnt --main=aggregation_main --invariant=reach_l_collection_3claims --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (730ms at 5 traces/second).

quint run aggregation.qnt --main=aggregation_main --invariant=reach_fv_unit_and_compact_adapter --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (690ms at 10 traces/second).

quint run aggregation.qnt --main=aggregation_main --invariant=reach_pcd_two_shapes_same_claim --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (630ms at 51 traces/second).

quint run aggregation.qnt --main=aggregation_main --invariant=reach_mw_key_admitted_and_refused --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (546ms at 11 traces/second).

quint run aggregation.qnt --main=aggregation_main --invariant=reach_forgeries_refused --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (381ms at 5 traces/second).

quint run aggregation.qnt --main=aggregation_main --invariant=reach_gp_gate_case_map_failed --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (378ms at 26 traces/second).

# 5. Safety mutant checks (all 24 MUST exit 1 with [violation])
quint run aggregation.qnt --main=aggregation_m_wrong_node_params --invariant=inv_NAG_001 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (701ms at 1 traces/second).

quint run aggregation.qnt --main=aggregation_m_accept_omitted_air --invariant=inv_NAG_002 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (433ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_unopened_key_root --invariant=inv_NAG_003 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (423ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_mixed_chipset_wrong_keyroot --invariant=inv_NAG_004 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (427ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_allow_mw_over_5400 --invariant=inv_NAG_005 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (628ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_allow_mw_over_5400 --invariant=inv_LPC_002 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (493ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_accept_t10q_forgery --invariant=inv_NAG_006 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (582ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_gp2_c21_unfixed --invariant=inv_NAG_007 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (611ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_l_unbound_outcome --invariant=inv_NAG_008 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (423ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_skip_child_link_check --invariant=inv_NAG_009 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (656ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_incomplete_proof_bundle --invariant=inv_NAG_010 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (466ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_accept_forged_dep --invariant=inv_GPH_002 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (560ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_accept_forged_dep --invariant=inv_GP_002 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (622ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_accept_unlisted_fv_key --invariant=inv_GPH_003 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (443ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_unbound_foreign_adapter --invariant=inv_GPH_004 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (556ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_unbound_foreign_adapter --invariant=inv_GP_003 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (328ms at 3 traces/second).

quint run aggregation.qnt --main=aggregation_m_pcd_shape_changes_claim --invariant=inv_GPH_005 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (576ms at 56 traces/second).

quint run aggregation.qnt --main=aggregation_m_pcd_shape_changes_claim --invariant=inv_GP_004 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (1978ms at 16 traces/second).

quint run aggregation.qnt --main=aggregation_m_suspend_alters_claim_pub --invariant=inv_GPH_006 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (685ms at 3 traces/second).

quint run aggregation.qnt --main=aggregation_m_count_only_gp_gate --invariant=inv_GPH_007 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (547ms at 18 traces/second).

quint run aggregation.qnt --main=aggregation_m_narrow_gp_on_gate_fail --invariant=inv_DG_010 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (328ms at 3 traces/second).

quint run aggregation.qnt --main=aggregation_m_narrow_gp_on_gate_fail --invariant=inv_CPR_005 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (432ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_l_accepts_forged_set_root --invariant=inv_GPH_008 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (493ms at 2 traces/second).

quint run aggregation.qnt --main=aggregation_m_l_accepts_forged_set_root --invariant=inv_LPC_003 --max-samples=1000 --max-steps=15 --seed=0x20261115
# Expected exit code: 1
# Printed summary: [violation] Found an issue (427ms at 2 traces/second).

# 6. Replay trace generation (20 ITF traces)
mkdir -p /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/aggregation/traces/aggregation_main
quint run aggregation.qnt --main aggregation_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/aggregation/traces/aggregation_main/t_{seq}.itf.json'
# Expected exit code: 0
# Printed summary: [ok] No violation found (3157ms at 6 traces/second).
```
