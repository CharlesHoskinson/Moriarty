# Quint Formal Model Report — Bounded Context: Bridge (`e2e-bridge`)

- **Context**: Bridge (`sherlock-e2e-prototype-20261115` + completion `sherlock-completion-20261115`)
- **Quint Version**: `0.32.0`
- **Deliverables**:
  - Model: `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/bridge/quint/bridge.qnt`
  - Deterministic unit & edge tests: `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/bridge/quint/bridge_test.qnt`
  - Replay traces (20 ITF JSON traces): `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/bridge/traces/bridge_main/t_0.itf.json` .. `t_19.itf.json`

---

## 1. Requirement Coverage Table

Every own EARS ID (`LTR-001`..`LTR-008`, `CLB-001`..`CLB-008`) and completion EARS ID (`ANC-013`..`ANC-019`, `LW-020`..`LW-022`) assigned to **Bridge** is accounted for below. Per `PROMPT.md`, no Adopted IDs are assigned to this context (`(none)`). Cross-referenced IDs cited in prose (`ANC-002` lock operand form, `ANC-012` chunk cap, `SQG-014`/`SSG-006` transcript gate) are exercised indirectly through `CLB-005`/`ANC-017`, `CLB-007`, and `LTR-005`. Status-report scenarios (`Status of ...` on `2026-11-15`) record repository build/commit metadata rather than transition state.

| ID | Artifact (`inv_*`, action + test, consistency `val`) | Kind | Note |
| :--- | :--- | :--- | :--- |
| `LTR-001` | `inv_LTR_001`; `proveLCallOrChunk`, `bridgeCallEndToEnd`; `br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest`; mutant `bridge_m_l_wrong_field_or_hash` | `MODELED` | Enforces every accepted `L` proof is proved over `FieldBabyBear` with `HashMidnightPoseidonFr` and stock-verified under the admitted `L` key. |
| `LTR-002` | `inv_LTR_002`; `proveAndVerifyLfr`, `bridgeCallEndToEnd`; `br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest`; mutant `bridge_m_lfr_uses_extension_field` | `MODELED` | Enforces every accepted `L_Fr` proof uses `FieldBls12381Fr` with extension degree `1` (no extension field) under the admitted `L_Fr` key. |
| `LTR-003` | `inv_LTR_003`; `startRekeyAndAdmitKeys`, `refuseLKeyOverBounds`; `br_LTR_003_CLB_008_LW_022_RekeyKeyBoundsAndWhirShapeTest`, `br_LTR_003_OverBoundsKeyBlockedFromStartTest`, `br_EdgeExactBounds_AtLimitKeyAdmittedAndOverLimitRefusedTest`; mutant `bridge_m_allow_l_key_over_bounds` | `MODELED`, `CONSTANT-CHECKED` | Constrains admitted `L` keys to `lKeyCols <= L_MAX_COLS` (`2,200`) and `lKeyMaxInteractions <= L_MAX_INTERACTIONS` (`100`); refuses `2,201`-column and `101`-interaction candidates; see also `Q-bridge-5` on `k` overloading (`k <= 21` in `LTR-003` Status). |
| `LTR-004` | `inv_LTR_004`; `buildLfrMachine`, `refuseTapeWithoutExpose`; `br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest`; mutant `bridge_m_accept_plain_write_tape` | `MODELED` | Enforces `L_Fr` `Machine::new` refuses any tape that writes public words with a plain write instead of `Tape::expose`. |
| `LTR-005` | `inv_LTR_005`; `buildLfrMachine`, `proveAndVerifyLfr`, `refuseTamperedTranscriptChallenge`; `br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest`; mutant `bridge_m_accept_wrong_transcript` | `MODELED` | Enforces `L_Fr` tape transcript replay outputs equal the stock `L` Fiat-Shamir challenges and refuses altered challenge words. |
| `LTR-006` | `inv_LTR_006`; `publishLfrReady`, `verifyTransferAndWrap`, `refuseCorruptedProofTransfer`; `br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest`; mutant `bridge_m_wrap_starts_on_corrupted_transfer` | `MODELED` | Binds `LfrProofReady` (`proofPathId`, `proofSha256`, `lfrVkPrehash`, `publicWordsCount`, `claimPub`) to the verified `L_Fr` proof and blocks `Wrap` when moved bytes fail SHA-256 verification on `muninn`; see `Q-bridge-6`. |
| `LTR-007` | `inv_LTR_007`; `proveLCallOrChunk`, `proveAndVerifyLfr`, `bridgeCallEndToEnd`; `br_LTR_007_PathCAndFullTreeSameUniversalKeysTest`; mutant `bridge_m_split_pathc_and_full_keys` | `MODELED` | Enforces identical `L` key SHA-256 and `L_Fr` key SHA-256 across 1-leaf `TreePathC` calls and 3-leaf `TreeFull` runs. |
| `LTR-008` | `inv_LTR_008`; `publishLfrReady`, `refuseForgedAnchorLfrNoRecord`; `br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest`; mutant `bridge_m_publish_ready_on_refused_lfr` | `MODELED` | Enforces no `LfrProofReady` record is published for an `L_Fr` proof refused by the stock verifier (e.g., forged `prog_d` anchor); see `Q-bridge-6`. |
| `CLB-001` | `inv_CLB_001`, `consistency_CLB_001_post_rekey_nonzero_delta`; `proveLCallOrChunk`; `br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest`, `br_ConsistencyAndSpecContradictionsTest`; mutant `bridge_m_wrong_iface_layout` | `MODELED`, `CONSTANT-CHECKED` | Enforces `publicWordsCount == IFACE_PUBLIC_WORDS` (`12,166`) and `laneMapPairs == IFACE_LANE_PAIRS` (`5`); `consistency_CLB_001_post_rekey_nonzero_delta` witnesses contradiction `Q-bridge-1`. |
| `CLB-002` | `inv_CLB_002`; `proveLCallOrChunk`, `refuseShiftedEffectLaneWrite`; `br_CLB_002_003_006_ANC_016_018_019_LAndWrapRefusalsTest`; mutant `bridge_m_accept_shifted_effect_lane` | `MODELED` | Refusal side of `ANC-018` (`FP-3`): `L` `EffectAir` refuses any witness where a call-vector effect lane word was not written by the guest at that lane. |
| `CLB-003` | `inv_CLB_003`; `proveLCallOrChunk`, `refuseWrongChunkCountInL`; `br_CLB_002_003_006_ANC_016_018_019_LAndWrapRefusalsTest`; mutant `bridge_m_accept_wrong_chunk_count` | `MODELED` | Refusal side of `ANC-019`: refuses any chunked run whose stated chunk count `k` differs from its true slice count; see `Q-bridge-5`. |
| `CLB-004` | `inv_CLB_004`; `proveAndVerifyLfr`; `br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest`, `br_CLB_005_ANC_017_TwoChunkLinkedAndEqualRunTagDistinctLocksTest`; mutant `bridge_m_lfr_uses_slot0_or_writes_mode0_anchor` | `MODELED`, `CONSTANT-CHECKED` | Enforces `ANCHOR_SLOT == 15` on every `L_Fr` anchor read and chunk-only anchor write, keeping ordinary calls anchor-write-free and slot `0` as a user word. |
| `CLB-005` | `inv_CLB_005`, `consistency_ANC_017_CLB_005_literal_lock_read_write`; `proveAndVerifyLfr`, `refuseSplicedOrAlteredChunkLock`; `br_CLB_005_ANC_017_TwoChunkLinkedAndEqualRunTagDistinctLocksTest`, `br_ChunkOrderDependencyBlockedTest`; mutant `bridge_m_accept_spliced_lock` | `MODELED` | Refuses any chunk carrying a lock operand outside its 7-input sponge `(c, pack(run_tag), k, d_in, d_out, pack(C_after), idx)`; `consistency_ANC_017_CLB_005_literal_lock_read_write` witnesses lock-read index contradiction `Q-bridge-2`. |
| `CLB-006` | `inv_CLB_006`, `consistency_mode0_slot15_outside_template_blocked`; `proveLCallOrChunk`, `refuseMode0AnchorTemplateOnIface`; `br_CLB_002_003_006_ANC_016_018_019_LAndWrapRefusalsTest`; mutant `bridge_m_iface_allows_mode0_anchor_write` | `MODELED` | Enforces `iface::decode_with` refuses mode-`0` calls carrying the chunk anchor-write template; `consistency_mode0_slot15_outside_template_blocked` witnesses lane-scope gap `Q-bridge-3`. |
| `CLB-007` | `inv_CLB_007`; `planCallChunks`, `refuseCallOverKMax`; `br_CLB_007_ChunkPlannerKMax2AndRefusalTest`, `br_EdgeMinK1_SingleSliceWrapsAndTwoSliceRefusedTest`; mutant `bridge_m_planner_allows_over_kmax` | `MODELED`, `CONSTANT-CHECKED` | Enforces chunk planner refuses calls requiring `> K_MAX` (`2` in `bridge_main`, `1` in `bridge_edge_min_k1`) chunks before proving starts; see `Q-bridge-5`. |
| `CLB-008` | `inv_CLB_008`, `consistency_CLB_008_catches_zero_and_sdk_drift`; `startRekeyAndAdmitKeys`, `refuseRekeySlotMismatch`; `br_LTR_003_CLB_008_LW_022_RekeyKeyBoundsAndWhirShapeTest`; mutant `bridge_m_rekey_starts_on_slot_mismatch` | `MODELED`, `CONSTANT-CHECKED` | Enforces rekey runner refuses to start if `LFR_CONFIG_SLOT != WRAP_DECODER_CONFIG_SLOT`; `consistency_CLB_008_catches_zero_and_sdk_drift` witnesses `0 == 0` / 3rd-copy gap `Q-bridge-4`. |
| `ANC-013` | `inv_ANC_013`; `verifyTransferAndWrap`, `refuseWrapWrongSlotPin`; `br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest`; mutant `bridge_m_wrap_allows_slot0_pin` | `MODELED`, `CONSTANT-CHECKED` | Enforces `Wrap` pins lane `6` (anchor read key) and lane `15` (chunk write key) to key constant `ANCHOR_SLOT = 15` and refuses slot `0`. |
| `ANC-014` | `inv_ANC_014`; `compileAndDeployContract`; `br_ANC_014_015_ContractDeploySlot15AndCompilerRefusalTest`; mutant `bridge_m_ledger_stores_cp_in_slot0` | `MODELED`, `CONSTANT-CHECKED` | Enforces deployed `SherlockLedger` contracts store `C_P` in top-level slot `15` and leave slot `0` as a contract user field. |
| `ANC-015` | `inv_ANC_015`; `compileAndDeployContract`, `refuseCollidingSlot15Contract`; `br_ANC_014_015_ContractDeploySlot15AndCompilerRefusalTest`; mutant `bridge_m_compiler_allows_slot15_field` | `MODELED`, `CONSTANT-CHECKED` | Enforces Mycroft compiler refuses any contract whose top-level layout reaches index `15` (`16` fields) before deployment. |
| `ANC-016` | `inv_ANC_016`, `consistency_mode0_slot15_outside_template_blocked`; `verifyTransferAndWrap`, `refuseMode0AnchorTemplateInWrap`; `br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest`, `br_CLB_002_003_006_ANC_016_018_019_LAndWrapRefusalsTest`; mutant `bridge_m_wrap_allows_mode0_anchor_write` | `MODELED` | Enforces `Wrap` refuses mode-`0` witnesses carrying the anchor-write template in lanes `11..19` and `21`; see `Q-bridge-3` on writes to slot `15` at lanes `>= 22`. |
| `ANC-017` | `inv_ANC_017`, `consistency_ANC_017_CLB_005_literal_lock_read_write`; `proveAndVerifyLfr`, `refuseSplicedOrAlteredChunkLock`; `br_CLB_005_ANC_017_TwoChunkLinkedAndEqualRunTagDistinctLocksTest`; mutant `bridge_m_accept_spliced_lock` | `MODELED` | Binds lock operands to `(c, run_tag, k, d_in, d_out, pack(C_after), idx)` so equal-`run_tag` runs with distinct `d_in` (`Call 3` vs `Call 4`, `runTag = 444`) cannot splice chunks; see `Q-bridge-2`. |
| `ANC-018` | `inv_ANC_018`; `proveLCallOrChunk`, `refuseShiftedEffectLaneWrite`; `br_CLB_002_003_006_ANC_016_018_019_LAndWrapRefusalsTest`; mutant `bridge_m_accept_shifted_effect_lane` | `MODELED` | Normative binding side of `CLB-002` (`FP-3`): binds every call-vector effect lane in `L` to the guest effect word written at that lane. |
| `ANC-019` | `inv_ANC_019`; `proveLCallOrChunk`, `refuseWrongChunkCountInL`; `br_CLB_002_003_006_ANC_016_018_019_LAndWrapRefusalsTest`; mutant `bridge_m_accept_wrong_chunk_count` | `MODELED` | Normative binding side of `CLB-003`: binds stated `k` to the slice count through the forward claim chain `C_j = vd(Cons(rec_j, C_(j+1)))`; see `Q-bridge-5`. |
| `LW-020` | `inv_LW_020`; `proveAndVerifyLfr`, `refuseMemBusTypeMismatchInCircuit`; `br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest`; mutant `bridge_m_membus_untyped_balance` | `MODELED` | Enforces `L_Fr` `MEM_BUS` carries `TY_BB = 0` or `TY_FR = 1` (`TyBb` / `TyFr`) as its 6th element and fails balance on cross-type reads. |
| `LW-021` | `inv_LW_021`; `buildLfrMachine`, `refuseIllTypedTapeInMachineNew`; `br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest`; mutant `bridge_m_machine_accepts_ill_typed_tape` | `MODELED` | Enforces `L_Fr` host `Machine::new` refuses tapes with ill-typed operands (e.g., passing an `Fr` cell to `BbAlu`) before proving. |
| `LW-022` | `inv_LW_022`; `startRekeyAndAdmitKeys`, `verifyTransferAndWrap`, `refuseInvalidWhirShape`; `br_LTR_003_CLB_008_LW_022_RekeyKeyBoundsAndWhirShapeTest`; mutant `bridge_m_wrap_skips_whir_shape_check` | `MODELED` | Enforces `Wrap` host `WhirShape::check` verifies `whirLogRs >= whirFoldingK` and refuses shapes with `log_rs < k` (`3 < 4`) before synthesizing cells. |

---

## 2. Findings

### `Q-bridge-1` — `CLB-001` Unconditional `12,166` Public-Word `SHALL` Clause Contradicts Its Own Post-Rekey Scenario and `FP-3` / Slot-15 Interface Changes

- **IDs**: `CLB-001` (interacting with `CLB-002`, `CLB-004`, `ANC-013`, `ANC-018`)
- **Quoted Sentences**:
  - `sherlock-e2e-call-binding/spec.md:4`:
    > "CLB-001: The `L` interface SHALL expose 12,166 public words with 5 lane-map pairs."
  - `sherlock-e2e-call-binding/spec.md:9-11`:
    > "- **GIVEN** the T25 interface
    > - **WHEN** `layout_full_is_12166` runs
    > - **THEN** the count is 12,166 or the record names the new count and the change of `~/.cache/rekey-t11/LIST.md` that caused it"
  - `sherlock-e2e-call-binding/spec.md:15`:
    > "- **THEN** it reads: BUILT on the T10m keys. FP-3 and slot 15 change the interface; the word count after T25 is not measured. Owner lane: T25."
- **Witness State / Check**:
  - Pure consistency check `consistency_CLB_001_post_rekey_nonzero_delta` (`bridge.qnt:1614-1615`) evaluates `(IFACE_PUBLIC_WORDS + 14) == IFACE_PUBLIC_WORDS` (`12180 == 12166 == false`) and fails under `quint run --main=bridge_main --invariant=consistency_CLB_001_post_rekey_nonzero_delta` (`exit 1`, `[violation]`).
  - Safety mutant `bridge_m_wrong_iface_layout` (`publicWordsCount = 12180`) breaks `inv_CLB_001` (`exit 1`) even though `CLB-001`'s own Scenario (`spec.md:11`) explicitly allows a post-T25 count `!= 12,166` when recorded against `~/.cache/rekey-t11/LIST.md`, and `CLB-002`/`ANC-018` (`FP-3`) + `CLB-004`/`ANC-013` (`slot 15`) mandate the exact T25 interface changes that `spec.md:15` states alter the interface.
- **What Closes It**:
  - Either parameterize `CLB-001`'s normative `SHALL` sentence by the frozen T25 interface constant (e.g., `IFACE_FULL_PUBLIC_WORDS` pinned in `frozen.rs` and verified equal across `L`, `L_Fr`, `LfrProofReady`, and `Wrap`), or measure the post-T25 public-word count after merging `FP-3` + `t12-slot15` and update both the `SHALL` literal and Scenario so no disjunctive "or the record names the new count" escape remains in the normative check.

### `Q-bridge-2` — `ANC-017` and `CLB-005` Specify `(pack(C_(j+1)), j)` for Both Lock Writes and Lock Reads, Making Chunk `j` Unable to Read Chunk `j - 1`'s Lock

- **IDs**: `ANC-017`, `CLB-005` (interacting with `ANC-019`)
- **Quoted Sentences**:
  - `sherlock-anchor-chunks/spec.md:48`:
    > "ANC-017: When a chunk call reads or writes a lock, the `L_Fr` circuit SHALL compute the lock operand `x_j` of ANC-002 from the run's `c`, `run_tag`, `k`, `d_in`, `d_out`, the FP-3 link `pack(C_(j+1))` and `j`."
  - `sherlock-e2e-call-binding/spec.md:60`:
    > "CLB-005: If a chunk carries a lock whose operand `x_j` differs from `sponge(U_LOCK; c, pack(run_tag), k, d_in, d_out, pack(C_(j+1)), j)` of its run, with `C_(j+1)` the forward claim chain of the run after chunk `j` (ANC-019), then the `L_Fr` circuit SHALL refuse the chunk."
- **Witness State / Check**:
  - Consider `Call 3` (`cP = 3001`, `runTag = 444`, `k = 2`, `dIn = 503`, `dOut = 603`), cut into Chunk `0` (`j = 0`, `C_1 = chainLinkAt(3, 2, 1) = 3111`) and Chunk `1` (`j = 1`, `C_2 = chainLinkAt(3, 2, 2) = 0`):
    - Chunk `0` (`j = 0`) **writes** lock `x_0 = computeLockOperand(3001, 444, 2, 503, 603, 3111, 0) = 80173884`.
    - Chunk `1` (`j = 1`) must **read** the lock written by Chunk `0` (`x_0 = 80173884`, computed from `(pack(C_1), 0) = (pack(C_j), j - 1)`).
    - However, `ANC-017` ("When a chunk call **reads or writes** a lock...") and `CLB-005` ("If a chunk carries a lock whose operand `x_j` differs from `sponge(... pack(C_(j+1)), j)`...") require Chunk `1`'s carried read lock to equal `x_1 = computeLockOperand(3001, 444, 2, 503, 603, 0, 1) = 80170147 != 80173884`.
  - Pure consistency check `consistency_ANC_017_CLB_005_literal_lock_read_write` (`bridge.qnt:1619-1622`) checks `x0WrittenByChunk0 == x1LiteralReadByChunk1` and fails (`exit 1`, `[violation]`). An implementation following the literal EARS text either refuses Chunk `1` in `L_Fr` when Chunk `1` carries `x_0`, or makes Chunk `1` read `x_1` which no prior chunk ever wrote to the ledger.
- **What Closes It**:
  - Distinguish the incoming read-lock operand from the outgoing write-lock operand in both `ANC-017` and `CLB-005`: for chunk `j` (`0 <= j < k`), the **outgoing lock write** (present iff `j + 1 < k`) uses `(pack(C_(j+1)), j)` (`x_j`), whereas the **incoming lock read** (present iff `j > 0`) uses `(pack(C_j), j - 1)` (`x_(j-1)`), where `C_j` is chunk `j`'s pre-chunk forward claim-chain link from `ANC-019`.

### `Q-bridge-3` — `CLB-006` and `ANC-016` Only Guard Mode-0 Anchor Writes in Template Lanes `11..19` and `21`, Leaving Slot `15` Writable at User Lanes `>= 22` for Non-Mycroft Guests

- **IDs**: `CLB-006`, `ANC-016`, `CLB-002`, `ANC-018`, `ANC-015`, `CLB-004`, `ANC-013`
- **Quoted Sentences**:
  - `sherlock-e2e-call-binding/spec.md:74, 79`:
    > "CLB-006: If an ordinary call (mode 0) carries the anchor-write template, then the interface decoder `iface::decode_with` SHALL refuse the call."
    > "- **GIVEN** an ordinary call with the write template in lanes 11 to 19 and 21"
  - `sherlock-anchor-chunks/spec.md:37`:
    > "ANC-016: If an ordinary call (mode 0) carries the anchor-write template in lanes 11 to 19 and 21, then the wrap circuit SHALL refuse the witness."
  - `sherlock-e2e-call-binding/spec.md:18`:
    > "CLB-002: If a call-vector effect lane holds a word that the guest did not write at that lane, then the `L` effect AIR SHALL refuse the witness."
  - `sherlock-anchor-chunks/spec.md:26, 34`:
    > "ANC-015: If a contract's ledger layout uses top-level index 15, then the Mycroft compiler SHALL refuse the contract before deploy and name the colliding field."
    > "... status on 2026-11-15 is: OPEN: the census test is missing on the mycroft branches."
- **Witness State / Check**:
  - Pure consistency check `consistency_mode0_slot15_outside_template_blocked` (`bridge.qnt:1626-1630`) checks whether the `CLB-006`/`ANC-016` lane guard (`lanes 11..19 and 21`) catches a mode-`0` write to `ANCHOR_SLOT = 15` emitted at user effect lane `22`; it evaluates to `false` (`exit 1`, `[violation]`).
  - Specifically: if a guest program on the universal `R_SL` interpreter (either a contract not compiled by Mycroft, or any 16-field contract while `ANC-015` remains `OPEN`) executes a ledger write targeting top-level index `15` in user effect lanes `>= 22` during an ordinary call (`mode 0`), then:
    1. `CLB-002` / `ANC-018` (`FP-3`) **accepts**, because the guest genuinely wrote those words at lane `22`;
    2. `CLB-006` (`iface::decode_with`) **accepts**, because lanes `11..19` and `21` do not carry the chunk anchor-write template;
    3. `ANC-016` (`Wrap`) **accepts**, because lanes `11..19` and `21` are clear.
    Consequently, the ordinary call overwrites `C_P` in ledger slot `15` without `L`, `L_Fr`, or `Wrap` refusing it.
- **What Closes It**:
  - Either enforce in `L` (`EffectAir`) / `L_Fr` (`iface`) / `Wrap` (`decode.rs`) that **no user effect lane** (`>= 22`, or any lane in mode `0`) may address reserved top-level ledger key `ANCHOR_SLOT = 15`, or explicitly state as a trust assumption in `ANC-015` + `ANC-016` that `C_P` integrity in mode `0` relies on every deployed `C_P` having passed Mycroft's `ANC-015` static slot-disjointness check (closing `ANC-015` as a hard prerequisite before any mode-`0` proof is trusted).

### `Q-bridge-4` — `CLB-008` Checks Only `lfrSlot != wrapDecoderSlot`, Missing Both `0 == 0` Pre-Merge Drift and the Third/Fourth Anchor-Slot Copies (`SherlockLedger` & Mycroft)

- **IDs**: `CLB-008`, `CLB-004`, `ANC-013`, `ANC-014`, `ANC-015`
- **Quoted Sentences**:
  - `sherlock-e2e-call-binding/spec.md:102, 104, 113`:
    > "CLB-008: If the anchor slot constant of the `L_Fr` interface differs from the anchor slot constant of the wrap decoder, then the rekey runner SHALL refuse to start."
    > "Verification: NEW: a check script in the T25 runner that reads the three constants. To build."
    > "Evidence: openvm `t12-slot15` 4ea3d05 `crates/sherlock-lfr/src/iface.rs:112`; Moriarty `t12-slot15` 40a86bea5 `tools/sherlock-wrap/src/decode.rs:70` (dirty) and `tools/sherlock-sdk/anchor-slot.mjs:14` (untracked). Three copies of one constant."
  - `sherlock-e2e-call-binding/spec.md:46`:
    > "CLB-004: The `L_Fr` interface SHALL use the ledger slot constant `ANCHOR_SLOT` = 15 for the anchor read of every call and for the anchor write of a chunk call."
  - `sherlock-anchor-chunks/spec.md:15`:
    > "ANC-014: When the SherlockLedger contract is deployed, the SherlockLedger contract SHALL store `C_P` in top-level slot 15."
- **Witness State / Check**:
  - Pure consistency check `consistency_CLB_008_catches_zero_and_sdk_drift` (`bridge.qnt:1634-1639`) evaluates `clb008LiteralCheck(lfrSlot, wrapSlot) := (lfrSlot == wrapSlot)` on two concrete drift configurations and fails (`exit 1`, `[violation]`):
    1. `(lfrSlot = 0, wrapSlot = 0)`: `0 == 0` satisfies `CLB-008`'s literal EARS rule even though both violate `CLB-004` and `ANC-013` (`== 15`), which is the exact pre-`t12-slot15` baseline (`t10mm` `5b7ada8`);
    2. `(lfrSlot = 15, wrapSlot = 15, sdkLedgerSlot = 0)`: `15 == 15` satisfies `CLB-008`'s literal EARS rule even though `CLB-008`'s own Verification and Evidence (`spec.md:104, 113`) note there are **three** copies (`iface.rs`, `decode.rs`, `anchor-slot.mjs` / `SherlockLedger` in `ANC-014`), plus Mycroft's reserved index in `ANC-015`.
- **What Closes It**:
  - Tighten `CLB-008`'s EARS `SHALL` sentence to: *"If any anchor slot constant across the `L_Fr` interface (`iface.rs`), the wrap decoder (`decode.rs`), `SherlockLedger` (`anchor-slot.mjs`), or the Mycroft layout check (`ANC-015`) differs from `15`, then the rekey runner SHALL refuse to start and name every component's value."*

### `Q-bridge-5` — Cross-Chunk Boundary Scope in `CLB-003`/`ANC-019`, Host-Only `K_MAX = 2` (`CLB-007`), and Three-Way Overloading of Identifier `k`

- **IDs**: `CLB-003`, `ANC-019`, `CLB-005`, `CLB-007`, `LTR-003`, `LW-022`
- **Quoted Sentences**:
  - `sherlock-e2e-call-binding/spec.md:32, 43`:
    > "CLB-003: If a chunked run states a chunk count `k` that differs from its slice count, then the `L` effect AIR SHALL refuse the witness."
    > "In `~/.cache/fp3/logs/after2.log` this test prints 'tape accepts' and 'L effect AIR accepts' and shows no refusal."
  - `sherlock-anchor-chunks/spec.md:70, 73-74`:
    > "ANC-019: When a run is cut into `k` chunks, the `L` circuit SHALL bind `k` to the slice count of the run through the forward claim chain."
    > "- **WHEN** a prover claims `k = 2` for a run of 3 slices
    > - **THEN** the `L` constraint set refuses the proof"
  - `sherlock-e2e-call-binding/spec.md:88`:
    > "CLB-007: If a schedule cuts one call into more than `K_MAX` = 2 chunks, then the chunk planner SHALL refuse the call before a proof starts."
  - `sherlock-e2e-l-transcript/spec.md:43`:
    > "... keeps k <= 21 and the wrap VK at or below 50,000 B."
  - `sherlock-l-and-wrap/spec.md:26`:
    > "LW-022: If a verifying-key WHIR shape has `log_rs < k`, then the wrap host SHALL refuse the shape before it synthesizes a cell."
- **Witness State / Analysis**:
  1. **Single-chunk vs cross-chunk `k` enforcement**: Each `L` proof proves **one** chunk `j` with incoming link `C_j` and outgoing link `C_(j+1)` (`bridge.qnt:555-556`). On chunk `0` of a 3-slice run (`C_0 -> C_1 -> C_2 -> C_3 = 0`), if a dishonest prover claims `k = 2` (`ANC-019` Scenario) or `k = 3` on a 2-slice run (`CLB-003` Scenario), a single non-terminal `L` chunk proof (`j = 0`) only sees one step `C_0 = vd(Cons(rec_0, C_1))` with `C_1 != Nil`; `L` alone can enforce boundary conditions (`j == 0` starts at `C_0`, and `j == k - 1 <=> C_(j+1) == Nil`), while closing the chain across all `k` steps requires `L_Fr` + ledger lock consumption (`CLB-005`/`ANC-017`) to force chunks `0, 1, ..., k - 1` to link into `C_k == Nil`. Attributing full refusal to a single-chunk `L` `EffectAir` test without checking terminal `j == k - 1 <=> C_(j+1) == Nil` + lock closure explains why `CLB-003` Status observes `"L effect AIR accepts"`.
  2. **Host-only `K_MAX = 2`**: `CLB-007` places `K_MAX = 2` solely in the host chunk planner (`planCallChunks` / `refuseCallOverKMax`); neither `L`, `L_Fr`, nor `Wrap` constrains `statedK <= K_MAX` in-circuit, so a custom prover bypassing the host planner can prove `k >= 3` chunks unless an in-circuit or `Wrap` check also enforces `k <= K_MAX`.
  3. **Three-way overloading of `k`**: Across the Bridge specs, bare `k` denotes (a) chunk count per call (`CLB-003`, `CLB-005`, `ANC-017`, `ANC-019`, bounded by `K_MAX = 2`), (b) wrap PLONK log2 circuit size (`k <= 21` in `LTR-003` Status), and (c) WHIR folding factor (`log_rs < k` in `LW-022`).
- **What Closes It**:
  - State the exact per-chunk `L` boundary constraints in `CLB-003` and `ANC-019` (`(j == k - 1) <=> (C_(j+1) == Nil)` plus `j < k`), note that end-to-end slice-count soundness across `j = 0..k-1` composes `ANC-019` with `CLB-005` lock consumption, decide whether `k <= K_MAX` (`CLB-007`) must also be enforced in `Wrap` / `iface`, and disambiguate `k_chunks`, `k_plonk`, and `k_whir` in the specs.

### `Q-bridge-6` — `LTR-006` Triggers `LfrProofReady` Publication Unconditionally on Prover Completion While `LTR-008` Only Guards Explicit Stock-Verifier Refusal, and `claim_pub` Is Undefined on Non-Terminal Chunks

- **IDs**: `LTR-006`, `LTR-008`
- **Quoted Sentences**:
  - `sherlock-e2e-l-transcript/spec.md:74`:
    > "LTR-006: When the `L_Fr` prover completes a proof, the `L_Fr` prover SHALL publish one `LfrProofReady` record with the proof file path, the field `proof_sha256`, the `L_Fr` key pre-hash, the public words and `claim_pub`."
  - `sherlock-e2e-l-transcript/spec.md:107`:
    > "LTR-008: If the stock verifier refuses an `L_Fr` proof, then the `L_Fr` prover SHALL publish no `LfrProofReady` record for that proof."
  - `architecture/DDD.md:148`:
    > "`LfrProofReady` (`LTR-006`) | Bridge -> Aggregation | `L_Fr` STARK proved and stock-verified on `odin`; carries path, `proof_sha256`, key pre-hash, public words, `claim_pub`."
- **Witness State / Analysis**:
  1. **Trigger Ordering Gap**: `LTR-006`'s event trigger (`"When the L_Fr prover completes a proof"`) fires before stock verification is mentioned, while `LTR-008` only forbids publication `"If the stock verifier refuses an L_Fr proof"`. If stock verification is skipped, disabled by a CLI flag, or run asynchronously after `LfrProofReady` is written, neither `LTR-006` nor `LTR-008` is literally violated even though `DDD.md:148` intends stock verification to be a mandatory precondition (`canPublishLfrReady` in `bridge.qnt:683` models the stronger `stockVerified == true` precondition, and `bridge_m_publish_ready_on_refused_lfr` breaks `inv_LTR_008`).
  2. **Undefined `claim_pub` on Chunk `j < k - 1` / Collection Runs**: `LTR-006` requires `LfrProofReady` to carry singular `claim_pub`, without specifying what `claim_pub` holds when an `L_Fr` proof proves non-terminal chunk `0` of a 2-chunk call (where final call outputs are not yet settled until chunk `1`) or a 3-leaf `TreeFull` collection run.
- **What Closes It**:
  - Amend `LTR-006`'s trigger to *"When the `L_Fr` prover completes a proof **and the stock `L_Fr` verifier accepts it** (`LTR-008`), the `L_Fr` prover SHALL publish one `LfrProofReady` record..."*, and define the type and value of `claim_pub` for ordinary calls, non-terminal/terminal chunks (`j < k - 1` vs `j == k - 1`), and multi-leaf `TreeFull` runs.

---

## 3. Mutant Table

All 23 `bridge_m_*` mutant modules weaken or remove one specific guard or binding while keeping all honest transitions enabled. Because three pairs of own + completion IDs (`CLB-002`/`ANC-018`, `CLB-003`/`ANC-019`, and `CLB-005`/`ANC-017`) specify the refusal and normative-binding sides of the exact same mechanism, those three mutants are checked against both paired `inv_<ID>` invariants, giving **26 `(mutant, invariant)` checks covering all 26 EARS IDs**. Every check exited with code `1` (`[violation]`), while `bridge_main` passed `inv` (`exit 0`, 2,000 samples) and violated all 9 `reach_*` witnesses (`exit 1`), confirming **zero vacuous mutants**.

| Mutant Module | Target Invariant | Exit Code | Printed Summary Line (`--max-samples=1000 --max-steps=15 --seed=0x20261115`) |
| :--- | :--- | :---: | :--- |
| `bridge_m_l_wrong_field_or_hash` | `inv_LTR_001` | `1` | `[violation] Found an issue (301ms at 3 traces/second).` |
| `bridge_m_lfr_uses_extension_field` | `inv_LTR_002` | `1` | `[violation] Found an issue (613ms at 2 traces/second).` |
| `bridge_m_allow_l_key_over_bounds` | `inv_LTR_003` | `1` | `[violation] Found an issue (1024ms at 4 traces/second).` |
| `bridge_m_accept_plain_write_tape` | `inv_LTR_004` | `1` | `[violation] Found an issue (812ms at 1 traces/second).` |
| `bridge_m_accept_wrong_transcript` | `inv_LTR_005` | `1` | `[violation] Found an issue (309ms at 3 traces/second).` |
| `bridge_m_wrap_starts_on_corrupted_transfer` | `inv_LTR_006` | `1` | `[violation] Found an issue (268ms at 4 traces/second).` |
| `bridge_m_split_pathc_and_full_keys` | `inv_LTR_007` | `1` | `[violation] Found an issue (478ms at 2 traces/second).` |
| `bridge_m_publish_ready_on_refused_lfr` | `inv_LTR_008` | `1` | `[violation] Found an issue (975ms at 1 traces/second).` |
| `bridge_m_wrong_iface_layout` | `inv_CLB_001` | `1` | `[violation] Found an issue (328ms at 3 traces/second).` |
| `bridge_m_accept_shifted_effect_lane` | `inv_CLB_002` | `1` | `[violation] Found an issue (1148ms at 1 traces/second).` |
| `bridge_m_accept_shifted_effect_lane` | `inv_ANC_018` | `1` | `[violation] Found an issue (343ms at 3 traces/second).` |
| `bridge_m_accept_wrong_chunk_count` | `inv_CLB_003` | `1` | `[violation] Found an issue (288ms at 3 traces/second).` |
| `bridge_m_accept_wrong_chunk_count` | `inv_ANC_019` | `1` | `[violation] Found an issue (313ms at 3 traces/second).` |
| `bridge_m_lfr_uses_slot0_or_writes_mode0_anchor` | `inv_CLB_004` | `1` | `[violation] Found an issue (477ms at 2 traces/second).` |
| `bridge_m_accept_spliced_lock` | `inv_CLB_005` | `1` | `[violation] Found an issue (533ms at 6 traces/second).` |
| `bridge_m_accept_spliced_lock` | `inv_ANC_017` | `1` | `[violation] Found an issue (365ms at 8 traces/second).` |
| `bridge_m_iface_allows_mode0_anchor_write` | `inv_CLB_006` | `1` | `[violation] Found an issue (326ms at 3 traces/second).` |
| `bridge_m_planner_allows_over_kmax` | `inv_CLB_007` | `1` | `[violation] Found an issue (285ms at 4 traces/second).` |
| `bridge_m_rekey_starts_on_slot_mismatch` | `inv_CLB_008` | `1` | `[violation] Found an issue (258ms at 4 traces/second).` |
| `bridge_m_wrap_allows_slot0_pin` | `inv_ANC_013` | `1` | `[violation] Found an issue (255ms at 4 traces/second).` |
| `bridge_m_ledger_stores_cp_in_slot0` | `inv_ANC_014` | `1` | `[violation] Found an issue (306ms at 3 traces/second).` |
| `bridge_m_compiler_allows_slot15_field` | `inv_ANC_015` | `1` | `[violation] Found an issue (497ms at 2 traces/second).` |
| `bridge_m_wrap_allows_mode0_anchor_write` | `inv_ANC_016` | `1` | `[violation] Found an issue (453ms at 4 traces/second).` |
| `bridge_m_membus_untyped_balance` | `inv_LW_020` | `1` | `[violation] Found an issue (310ms at 3 traces/second).` |
| `bridge_m_machine_accepts_ill_typed_tape` | `inv_LW_021` | `1` | `[violation] Found an issue (278ms at 4 traces/second).` |
| `bridge_m_wrap_skips_whir_shape_check` | `inv_LW_022` | `1` | `[violation] Found an issue (346ms at 3 traces/second).` |

---

## 4. State Vocabulary for Replay

In exported ITF traces (`traces/bridge_main/t_{seq}.itf.json`), Quint prefixes each state variable of `bridge` imported inside `bridge_main` as `bridge_main::bridge::<var>`.

### 4.1 State Variables (`var`)

| Quint `var` | ITF JSON Key | Quint Type | Meaning |
| :--- | :--- | :--- | :--- |
| `lastOp` | `bridge_main::bridge::lastOp` | `str` | Name of the transition that produced the current state (`"init"` at step `0`). |
| `lastArgs` | `bridge_main::bridge::lastArgs` | `StepArgs` (`{ primaryId: int, secondaryId: int, refusal: RefusalCode }`) | Replay driver arguments naming the entity ID (`kId`, `cId`, `callId`, `pSpecId`, or refusal target ID) and `RefusalCode` (`RefNone` on positive steps). |
| `rekeyState` | `bridge_main::bridge::rekeyState` | `RekeyStateRec` | Rekey runner status (`started`), slot configs (`lfrSlotCfg`, `wrapDecoderSlotCfg`, `sdkLedgerSlotCfg`), admitted `L` key metrics (`admittedLKeyId`, `lKeyCols`, `lKeyMaxInteractions`), universal `L` / `L_Fr` VK IDs, pre-hash and SHA-256 digests, and `WhirShape::check` state (`whirShapeChecked`, `whirLogRs`, `whirFoldingK`). |
| `compiledContracts` | `bridge_main::bridge::compiledContracts` | `int -> ContractRec` | Map `contractId -> ContractRec` of contracts accepted by Mycroft (`ANC-015`) and deployed on `SherlockLedger` (`ANC-014`, storing `C_P` in slot `15` and keeping slot `0` as user field). |
| `chunkPlans` | `bridge_main::bridge::chunkPlans` | `int -> ChunkPlanRec` | Map `callId -> ChunkPlanRec` of calls scheduled by the host chunk planner (`chunkCount <= K_MAX`, `CLB-007`). |
| `lProofs` | `bridge_main::bridge::lProofs` | `int -> LProofRec` | Map `pSpecId -> LProofRec` of stock-verified `L` STARK proofs (`LTR-001`, `LTR-007`), public interface layout (`12,166` words, `5` lane pairs, `CLB-001`), `EffectAir` lane binding (`CLB-002`, `ANC-018`), forward claim chain (`CLB-003`, `ANC-019`), `iface::decode_with` mode-`0` check (`CLB-006`), and stock Fiat-Shamir challenges (`stockChallengesD`). |
| `lfrMachines` | `bridge_main::bridge::lfrMachines` | `int -> LfrMachineRec` | Map `pSpecId -> LfrMachineRec` of `L_Fr` host `Machine::new` checks (`usedTapeExpose` for `LTR-004`, `operandsWellTyped` for `LW-021`, `replayedChallengesD` for `LTR-005`). |
| `lfrProofs` | `bridge_main::bridge::lfrProofs` | `int -> LfrProofRec` | Map `pSpecId -> LfrProofRec` of proved `L_Fr` STARKs (`FieldBls12381Fr`, `extDegree == 1`, `LTR-002`), typed `MEM_BUS` (`LW-020`), `ANCHOR_SLOT = 15` read/write discipline (`CLB-004`), 7-operand incoming/outgoing chunk locks (`CLB-005`, `ANC-017`), and stock verification flag (`LTR-008`). |
| `readyRecords` | `bridge_main::bridge::readyRecords` | `int -> LfrProofReadyRec` | Map `pSpecId -> LfrProofReadyRec` of published `LfrProofReady` records (`LTR-006`, `LTR-008`) carrying `proofPathId`, `proofSha256`, `lfrVkPrehash`, `publicWordsCount`, `claimPub`, `movedBytesSha256`, and `muninnCheckPassed`. |
| `wrapAcceptances` | `bridge_main::bridge::wrapAcceptances` | `int -> WrapAcceptanceRec` | Map `pSpecId -> WrapAcceptanceRec` of `Wrap` acceptances on `muninn` after transfer SHA-256 match (`LTR-006`), `WhirShape::check` (`LW-022`), lane `6` & `15` key pin `15` (`ANC-013`), and mode-`0` anchor-template refusal (`ANC-016`). |
| `refusalLog` | `bridge_main::bridge::refusalLog` | `Set[RefusalEvent]` | Monotonic set of recorded `{ code: RefusalCode, targetId: int }` refusal events across keygen, Mycroft, planner, `L`, `iface`, `L_Fr`, `muninn`, and `Wrap`. |

### 4.2 `lastOp` Values and Implemented Requirements

| `lastOp` Value | `lastArgs` (`primaryId`, `secondaryId`, `refusal`) | Implemented EARS IDs |
| :--- | :--- | :--- |
| `"init"` | `(0, 0, RefNone)` | Initial empty Bridge state prior to rekey/deployment. |
| `"startRekeyAndAdmitKeys"` | `(kId, 1, RefNone)` | `CLB-008`, `LTR-003`, `LW-022` |
| `"compileAndDeployContract"` | `(cId, ANCHOR_SLOT, RefNone)` | `ANC-014`, `ANC-015` |
| `"planCallChunks"` | `(callId, sliceCount, RefNone)` | `CLB-007` |
| `"proveLCallOrChunk"` | `(pSpecId, callId, RefNone)` | `LTR-001`, `LTR-007`, `CLB-001`, `CLB-002`, `CLB-003`, `CLB-006`, `ANC-018`, `ANC-019` |
| `"buildLfrMachine"` | `(pSpecId, pSpecId, RefNone)` | `LTR-004`, `LW-021`, `LTR-005` |
| `"proveAndVerifyLfr"` | `(pSpecId, callId, RefNone)` | `LTR-002`, `LTR-005`, `LTR-007`, `CLB-004`, `CLB-005`, `ANC-017`, `LW-020` |
| `"publishLfrReady"` | `(pSpecId, 6000 + pSpecId, RefNone)` | `LTR-006`, `LTR-008` |
| `"verifyTransferAndWrap"` | `(pSpecId, pSpecId, RefNone)` | `LTR-006`, `ANC-013`, `ANC-016`, `LW-022` |
| `"bridgeCallEndToEnd"` | `(pSpecId, callId, RefNone)` | `LTR-001`..`008`, `CLB-001`..`008`, `ANC-013`..`019`, `LW-020`..`022` (composes any missing prerequisite stages for `pSpecId` using the exact per-stage transformers) |
| `"refuseRekeySlotMismatch"` | `(15, 0, RefRekeySlotMismatch)` | `CLB-008` |
| `"refuseLKeyOverBounds"` | `(kId, 0, RefLKeyBounds)` | `LTR-003` (`kId = 3`: `2201 > 2200` cols; `kId = 4`: `101 > 100` interactions) |
| `"refuseInvalidWhirShape"` | `(3, 4, RefWhirShapeLogRsLtK)` | `LW-022` (`log_rs = 3 < k = 4`) |
| `"refuseCollidingSlot15Contract"` | `(2, 15, RefCompilerSlot15Collision)` | `ANC-015` (16-field contract `cId = 2`) |
| `"refuseCallOverKMax"` | `(callId, K_MAX, RefExceedsKMax)` | `CLB-007` (`callId = 5` in `bridge_main`; `callId = 3, 4, 5` in `bridge_edge_min_k1`) |
| `"refuseShiftedEffectLaneWrite"` | `(81, 12, RefShiftedEffectLane)` | `CLB-002`, `ANC-018` |
| `"refuseWrongChunkCountInL"` | `(82, 3, RefWrongChunkCount)` | `CLB-003`, `ANC-019` |
| `"refuseMode0AnchorTemplateOnIface"` | `(83, 0, RefMode0AnchorTemplateIface)` | `CLB-006` |
| `"refuseTapeWithoutExpose"` | `(84, 0, RefMissingTapeExpose)` | `LTR-004` |
| `"refuseIllTypedTapeInMachineNew"` | `(85, 1, RefIllTypedTapeOperand)` | `LW-021` |
| `"refuseTamperedTranscriptChallenge"` | `(86, 1, RefTranscriptChallengeMismatch)` | `LTR-005` |
| `"refuseMemBusTypeMismatchInCircuit"` | `(87, 0, RefMemBusTypeImbalance)` | `LW-020` |
| `"refuseSplicedOrAlteredChunkLock"` | `(3, 4, RefChunkLockMismatch)` | `CLB-005`, `ANC-017` |
| `"refuseForgedAnchorLfrNoRecord"` | `(89, 0, RefStockLfrForgedAnchor)` | `LTR-008` |
| `"refuseCorruptedProofTransfer"` | `(90, 0, RefCorruptedProofTransfer)` | `LTR-006` |
| `"refuseWrapWrongSlotPin"` | `(91, 0, RefWrapWrongSlotPin)` | `ANC-013` |
| `"refuseMode0AnchorTemplateInWrap"` | `(92, 11, RefMode0AnchorTemplateWrap)` | `ANC-016` |
| `"mutantPublishOnRefusedLfr"` | `(99, 99, RefNone)` | Disabled in all honest instances (`MUTANT == "none"`); enabled only in `bridge_m_publish_ready_on_refused_lfr` to test `inv_LTR_008`. |

### 4.3 Instance Constants

| Constant | `bridge_main` | `bridge_edge_min_k1` | `bridge_edge_exact_bounds` | `bridge_m_*` (23 modules) |
| :--- | :---: | :---: | :---: | :--- |
| `L_MAX_COLS` | `2200` | `2200` | `2200` | `2200` |
| `L_MAX_INTERACTIONS` | `100` | `100` | `100` | `100` |
| `L_FP3_COLS` | `2071` | `1995` | `2200` | `2071` |
| `IFACE_PUBLIC_WORDS` | `12166` | `12166` | `12166` | `12166` |
| `IFACE_LANE_PAIRS` | `5` | `5` | `5` | `5` |
| `ANCHOR_SLOT` | `15` | `15` | `15` | `15` |
| `K_MAX` | `2` | `1` | `2` | `2` |
| `LFR_CONFIG_SLOT` | `15` | `15` | `15` | `15` |
| `WRAP_DECODER_CONFIG_SLOT` | `15` | `15` | `15` | `15` (`0` in `bridge_m_rekey_starts_on_slot_mismatch`) |
| `SDK_LEDGER_CONFIG_SLOT` | `15` | `15` | `15` | `15` |
| `MUTANT` | `"none"` | `"none"` | `"none"` | Respective mutant tag string |

---

## 5. Model Abstractions

1. **Cryptographic Hashes, Sponges, and Transcripts**:
   - Midnight Poseidon over `BLS12-381 Fr`, SHA-256 over proof/key files, forward claim-chain links `C_j = vd(Cons(rec_j, C_(j+1)))`, and the 7-operand lock sponge `sponge(U_LOCK; c, pack(run_tag), k, d_in, d_out, pack(C_after), idx)` are modeled as collision-free deterministic integer encodings (`chainLinkAt`, `computeLockOperand`, `computeTranscriptChallenges`, `computeProofSha256`) over small finite integers, with `Nil` represented by `0`.
2. **STARK / PLONK Constraint Systems and Stock Verifiers**:
   - Polynomial commitments, FRI/WHIR openings, BabyBear/BLS12-381 field arithmetic, LogUp `MEM_BUS` multiset sums, and Halo2/PLONK gate synthesis in `Wrap` are abstracted as explicit typed record attributes (`baseField`, `extDegree`, `hashConfig`, `memBusSixthElem`, `memBusBalanced`, `effectLanesMatchGuest`, `forwardChainValid`, `lane6ReadKeyConst`, `lane15ChunkWriteKeyConst`, `whirShapeChecked`) and boolean verifier outcomes (`stockVerified`, `wrapAccepted`).
3. **Finite Catalogs of Keys, Contracts, Calls, and Proof Slots**:
   - Four `L` key candidates (`1..4`: post-FP3 `2,071` cols, exact limit `2,200` cols / `100` interactions, over-column `2,201`, over-interaction `101`), two WHIR shapes (`log_rs = 5, k = 4` vs `log_rs = 3, k = 4`), two contracts (`4`-field valid vs `16`-field colliding on index `15`), five representative calls (`Call 1` 1-leaf `TreePathC`, `Call 2` 3-leaf `TreeFull`, `Call 3` & `Call 4` 2-chunk calls sharing `runTag = 444` with distinct `dIn = 503` vs `599`, and `Call 5` 3-slice call exceeding `K_MAX = 2`), and six proof slots (`pSpecId` `1..6`) represent the state space without unbounded allocation.
4. **Cross-Machine File Transfer (`odin` -> `muninn`)**:
   - File-system paths and moved byte streams in `LTR-006` are modeled as integer `proofPathId` (`6000 + pSpecId`) and digests (`proofSha256`, `movedBytesSha256`, `muninnCheckPassed`).
5. **Process / Build Status Metadata**:
   - Repository git-branch status strings (`BUILT`, `OPEN`, `GAP`, `PARTIAL`, commit hashes, and file line numbers in `Status of ...` scenarios) and wrap circuit log-row / VK byte-size budgets mentioned in `LTR-003` Status prose (`k <= 21`, `VK <= 50,000 B`, owned by Aggregation's `WVK-*` specs) are not duplicated as fake transition state in Bridge.

---

## Commands

Run from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/bridge/quint`:

```bash
# 1. Typecheck model and test suite
quint typecheck bridge.qnt
# Expected exit code: 0
# Printed: (no output)

quint typecheck bridge_test.qnt
# Expected exit code: 0
# Printed: (no output)

# 2. Deterministic unit and edge-instance tests
quint test --main=bridge_test --match='br_.*' --seed=0x20261115 bridge_test.qnt
# Expected exit code: 0
# Printed: 10 passing (4532ms)

quint test --main=bridge_edge_min_k1_test --match='br_.*' --seed=0x20261115 bridge_test.qnt
# Expected exit code: 0
# Printed: 1 passing (425ms)

quint test --main=bridge_edge_exact_bounds_test --match='br_.*' --seed=0x20261115 bridge_test.qnt
# Expected exit code: 0
# Printed: 1 passing (456ms)

# 3. Main and edge-instance invariant runs
quint run --main=bridge_main --invariant=inv --max-samples=2000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 0
# Printed: [ok] No violation found (10549ms at 190 traces/second).

quint run --main=bridge_edge_min_k1 --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 0
# Printed: [ok] No violation found (4753ms at 210 traces/second).

quint run --main=bridge_edge_exact_bounds --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 0
# Printed: [ok] No violation found (5647ms at 177 traces/second).

# 4. Generate 20 ITF replay traces for the Lean oracle
mkdir -p /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/bridge/traces/bridge_main
quint run bridge.qnt --main bridge_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/bridge/traces/bridge_main/t_{seq}.itf.json'
# Expected exit code: 0
# Printed: [ok] No violation found (974ms at 21 traces/second).

# 5. Negated reachability witnesses (non-vacuity; every witness MUST exit 1 with [violation])
quint run --main=bridge_main --invariant=reach_rekey_and_contract_deployed --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (231ms at 4 traces/second).

quint run --main=bridge_main --invariant=reach_ordinary_call_wrapped --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (305ms at 7 traces/second).

quint run --main=bridge_main --invariant=reach_pathc_and_full_tree_same_keys --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (258ms at 19 traces/second).

quint run --main=bridge_main --invariant=reach_two_chunk_call_linked_and_wrapped --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (304ms at 10 traces/second).

quint run --main=bridge_main --invariant=reach_equal_run_tag_distinct_runs_proved --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (264ms at 4 traces/second).

quint run --main=bridge_main --invariant=reach_key_whir_and_kmax_refusals --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (406ms at 5 traces/second).

quint run --main=bridge_main --invariant=reach_compiler_and_rekey_refusals --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (325ms at 28 traces/second).

quint run --main=bridge_main --invariant=reach_l_and_iface_refusals --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (474ms at 4 traces/second).

quint run --main=bridge_main --invariant=reach_lfr_and_wrap_refusals --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (329ms at 3 traces/second).

# 6. Pure consistency contradiction witnesses (Q-bridge-1 .. Q-bridge-4)
quint run --main=bridge_main --invariant=consistency_CLB_001_post_rekey_nonzero_delta --max-samples=1 --max-steps=1 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (260ms at 4 traces/second).

quint run --main=bridge_main --invariant=consistency_ANC_017_CLB_005_literal_lock_read_write --max-samples=1 --max-steps=1 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (276ms at 4 traces/second).

quint run --main=bridge_main --invariant=consistency_mode0_slot15_outside_template_blocked --max-samples=1 --max-steps=1 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (322ms at 3 traces/second).

quint run --main=bridge_main --invariant=consistency_CLB_008_catches_zero_and_sdk_drift --max-samples=1 --max-steps=1 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (416ms at 2 traces/second).

# 7. Safety mutants (all 26 checks across 23 mutant instances MUST exit 1 with [violation])
quint run --main=bridge_m_l_wrong_field_or_hash --invariant=inv_LTR_001 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (301ms at 3 traces/second).

quint run --main=bridge_m_lfr_uses_extension_field --invariant=inv_LTR_002 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (613ms at 2 traces/second).

quint run --main=bridge_m_allow_l_key_over_bounds --invariant=inv_LTR_003 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (1024ms at 4 traces/second).

quint run --main=bridge_m_accept_plain_write_tape --invariant=inv_LTR_004 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (812ms at 1 traces/second).

quint run --main=bridge_m_accept_wrong_transcript --invariant=inv_LTR_005 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (309ms at 3 traces/second).

quint run --main=bridge_m_wrap_starts_on_corrupted_transfer --invariant=inv_LTR_006 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (268ms at 4 traces/second).

quint run --main=bridge_m_split_pathc_and_full_keys --invariant=inv_LTR_007 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (478ms at 2 traces/second).

quint run --main=bridge_m_publish_ready_on_refused_lfr --invariant=inv_LTR_008 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (975ms at 1 traces/second).

quint run --main=bridge_m_wrong_iface_layout --invariant=inv_CLB_001 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (328ms at 3 traces/second).

quint run --main=bridge_m_accept_shifted_effect_lane --invariant=inv_CLB_002 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (1148ms at 1 traces/second).

quint run --main=bridge_m_accept_shifted_effect_lane --invariant=inv_ANC_018 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (343ms at 3 traces/second).

quint run --main=bridge_m_accept_wrong_chunk_count --invariant=inv_CLB_003 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (288ms at 3 traces/second).

quint run --main=bridge_m_accept_wrong_chunk_count --invariant=inv_ANC_019 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (313ms at 3 traces/second).

quint run --main=bridge_m_lfr_uses_slot0_or_writes_mode0_anchor --invariant=inv_CLB_004 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (477ms at 2 traces/second).

quint run --main=bridge_m_accept_spliced_lock --invariant=inv_CLB_005 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (533ms at 6 traces/second).

quint run --main=bridge_m_accept_spliced_lock --invariant=inv_ANC_017 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (365ms at 8 traces/second).

quint run --main=bridge_m_iface_allows_mode0_anchor_write --invariant=inv_CLB_006 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (326ms at 3 traces/second).

quint run --main=bridge_m_planner_allows_over_kmax --invariant=inv_CLB_007 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (285ms at 4 traces/second).

quint run --main=bridge_m_rekey_starts_on_slot_mismatch --invariant=inv_CLB_008 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (258ms at 4 traces/second).

quint run --main=bridge_m_wrap_allows_slot0_pin --invariant=inv_ANC_013 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (255ms at 4 traces/second).

quint run --main=bridge_m_ledger_stores_cp_in_slot0 --invariant=inv_ANC_014 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (306ms at 3 traces/second).

quint run --main=bridge_m_compiler_allows_slot15_field --invariant=inv_ANC_015 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (497ms at 2 traces/second).

quint run --main=bridge_m_wrap_allows_mode0_anchor_write --invariant=inv_ANC_016 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (453ms at 4 traces/second).

quint run --main=bridge_m_membus_untyped_balance --invariant=inv_LW_020 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (310ms at 3 traces/second).

quint run --main=bridge_m_machine_accepts_ill_typed_tape --invariant=inv_LW_021 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (278ms at 4 traces/second).

quint run --main=bridge_m_wrap_skips_whir_shape_check --invariant=inv_LW_022 --max-samples=1000 --max-steps=15 --seed=0x20261115 bridge.qnt
# Expected exit code: 1
# Printed: [violation] Found an issue (346ms at 3 traces/second).
```
