# Quint Formal Model Report — Bounded Context **Wrap** (`e2e-wrap`)

Independent Quint 0.32.0 specification (`wrap.qnt`), scenario test suite (`wrap_test.qnt`), 24 guard mutants, 7 reachability witnesses, and 20 ITF replay traces (`../traces/wrap_main/t_{0..19}.itf.json`) derived solely from the assigned EARS requirements and `DDD.md` Wrap rows.

---

## 1. Requirement Coverage Table

### 1.1 Own (`sherlock-e2e-prototype-20261115`) and Completion (`sherlock-completion-20261115`) IDs

> Note: `sherlock-e2e-wrap/spec.md` jumps directly from `WRP-004` to `WRP-006`; `WRP-005` does not exist in `spec.md` or `PROMPT.md` (see `Q-wrap-6`).

| ID | Artifact (`wrap.qnt` / `wrap_test.qnt`) | Kind | Note |
|---|---|---|---|
| `WRP-001` | `inv_WRP_001`, `doReceiveHonestLfrCall`, `doProveWrapCpu`, `doProveWrapGpu`, `wp_honestCpuWrapAndOutputTest`, mutant `wrap_m_wrp001_wrong_scheme` | `MODELED` | Each verified wrap run responds to a received `LfrProofReady` call and produces `proofCountInRun == 1` PLONK proof with KZG over BLS12-381 (`PlonkKzgBls12381`) and `Blake2bTranscript`; missing refusal qualifier noted in `Q-wrap-6`. |
| `WRP-002` | `inv_WRP_002`, `MAX_WRAP_K = 21`, `doKeygenWrap`, `doRefuseKeygenAboveK21`, `wp_refuseK22Test`, mutant `wrap_m_wrp002_allow_k22` | `CONSTANT-CHECKED` | Requests with `requestedK > MAX_WRAP_K` (`22`) stop before keygen with `stopMessage == "never k = 22"` and write no key; missing lower bound `k >= 19` noted in `Q-wrap-1`. |
| `WRP-003` | `inv_WRP_003`, `MAX_ROWS_AT_K21 = 1782573`, `doKeygenWrap`, `doRefuseKeygenRowGate`, `wp_refuseRowGateAndBoundaryFitTest`, mutant `wrap_m_wrp003_ignore_row_gate` | `CONSTANT-CHECKED` | Cost-model row counts above `1,782,573` at `k = 21` stop before keygen without writing a key; T10m measured `1,685,845` and boundary `1,782,573` fit; missing per-`k` bound at `k in {19, 20}` noted in `Q-wrap-1`. |
| `WRP-004` | `inv_WRP_004`, `MAX_VK_BYTES = 50000`, `doKeygenWrap`, `doRefuseVkTooLarge`, `wp_refuseVkOver50000BytesTest`, mutant `wrap_m_wrp004_write_oversize_vk` | `CONSTANT-CHECKED` | Any serialized wrap VK `> 50,000` bytes stops without writing `"wrap.vk"` (measured T10m `1,325` and boundary `50,000` pass); interaction with `WRP-009` 3-file atomicity noted in `Q-wrap-4`. |
| `WRP-006` | `inv_WRP_006`, `doProveWrapCpu`, `doProveWrapGpu`, `doRefuseSeededBlinderInProd`, `wp_twoProdProofsDifferAndSeededProdRefusedTest`, mutant `wrap_m_wrp006_reuse_blinder` | `MODELED` | Every production wrap proof draws a fresh blinder from `OsCsprng`, so any two production proofs (including two proofs of one call) differ byte-for-byte (`proofBytesTag`) while both verify; tension with `test-seed` (`ZK-003`/`GPW-003`) noted in `Q-wrap-2`. |
| `WRP-007` | `inv_WRP_007`, `ANCHOR_SLOT_CONST = 15`, `ANCHOR_READ_LANE = 6`, `CHUNK_WRITE_LANE = 15`, `validAnchorPrologue`, `doRefuseInvalidWrapWitness`, `wp_refuseForgedAnchorSlotAndAcceptChunkSlot15Test`, mutant `wrap_m_wrp007_skip_slot15_pin` | `CONSTANT-CHECKED` | Pins anchor read key (`lane 6 == 15`) on every call, requires chunk write prologue iff `ChunkMode` (`ANC-016`), and pins `lane 15 == 15` on chunk calls; unconditioned literal `lane 15 == 15` defect on `OrdMode` calls noted in `Q-wrap-3`. |
| `WRP-008` | `inv_WRP_008`, `doConfigureExpectVk`, `doRefuseExpectVkMismatch`, `wp_expectVkMismatchRefusedAndMatchAcceptedTest`, mutant `wrap_m_wrp008_ignore_expect_vk` | `MODELED` | When `E6_EXPECT_VK` is configured and `vkId != expectedVkId`, the wrap tool stops (`"the wrap key differs"`) with `writtenFiles == Set()`; optional env-var gap noted in `Q-wrap-4`. |
| `WRP-009` | `inv_WRP_009`, `WRAP_OUTPUT_FILE_COUNT = 3`, `WRAP_OUTPUT_FILES`, `doProveWrapCpu`, `doProveWrapGpu`, `wp_honestCpuWrapAndOutputTest`, mutant `wrap_m_wrp009_partial_output` | `CONSTANT-CHECKED` | Every library-verified wrap proof writes all `3` `WrapOutput` files `{"wrap.vk", "wrap.proof", "wrap.public.json"}` and unverified/stopped runs write `Set()`; ordering conflict with post-verify `WRP-004`/`WRP-008` noted in `Q-wrap-4`. |
| `WRP-010` | `inv_WRP_010`, `doKeygenWrap`, `doDeployProgramLedger`, `wp_universalKeyAcrossProgramsAndModesTest`, mutant `wrap_m_wrp010_per_program_vk` | `MODELED` | Enforces singleton `eraWrapVkSet == Set(cpuWrapKey.vkId)` shared across all programs, both modes (`OrdMode`, `ChunkMode`), and all Sherlock write-entry deployments in the era. |
| `WRP-011` | `inv_WRP_011`, `doConfigureGpuProver`, `doProveWrapGpu`, `wp_gpuSharesKeyDetSeedMatchesAndCpuFallbackTest`, mutant `wrap_m_wrp011_divergent_gpu_vk` | `MODELED` | Whenever GPU wrap proving is configured or produces a proof, `gpuWrapVkId == cpuWrapKey.vkId`. |
| `WRP-012` | `inv_WRP_012`, `LW_QUAL_CHECK_COUNT = 5`, `REQUIRED_LW_CHECKS`, `doRecordT25LwPasses`, `doCloseT25Qualification`, `doRefuseIncompleteT25Qualification`, `wp_t25QualificationAll5LwChecksAndLw014Lw016Test`, mutant `wrap_m_wrp012_missing_lw004` | `CONSTANT-CHECKED` | Closing `t25Qualified` (and deploying with `usesT25Key`) requires pass results for all `5` checks `{"LW-003", "LW-004", "LW-005", "LW-006", "LW-014"}`; omission of `LW-002`/`LW-013`/`LW-016` from the 5-item list noted in `Q-wrap-5`. |
| `RKY-003` | `inv_RKY_003`, `MAX_WRAP_K = 21`, `MAX_ROWS_AT_K21 = 1782573`, `doCompleteRekeyWrapMeasurement`, `doRefuseRekeyBoundsOverflow`, `wp_rekeyBoundsAndSpecGapWitnessesTest`, mutant `wrap_m_rky003_accept_rekey_row_overflow` | `CONSTANT-CHECKED` | Completed rekey records `rekeyMeasuredK == 21` and `0 < rekeyMeasuredRows <= 1,782,573`; row overflow stops rekey and sets `rekeyLeverListWritten = true`. |
| `RKY-004` | `inv_RKY_004`, `MAX_VK_BYTES = 50000`, `doCompleteRekeyWrapMeasurement`, `doRefuseRekeyBoundsOverflow`, `wp_rekeyBoundsAndSpecGapWitnessesTest`, mutant `wrap_m_rky004_vk_overflow` | `CONSTANT-CHECKED` | Completed rekey records `0 < rekeyMeasuredVkBytes <= 50,000`; byte-length overflow stops rekey. |

### 1.2 Adopted IDs (`sherlock-processor-final-20261111`)

| Adopted ID | Disposition in `e2e-wrap` | Artifact / Reason |
|---|---|---|
| `ANC-001` | Modeled (`inv_ANC_001` + `inv_WRP_007`, mutant `wrap_m_anc001_plain_progd`) | Anchor slot constant `15`, never holds plain `prog_d`, and user code never addresses slot `15` in accepted wraps (compiler refusal `MYC-SEAL-021` itself belongs to Compact Compilation). |
| `DG-002` | Modeled (`inv_DG_002`, `MIN_WRAP_K = 19`, `usableRows85Pct`, mutant `wrap_m_dg002_k20_row_overflow`) | Enforces `k in {19, 20, 21}`, `rows <= floor(0.85 * (2^k - 7))`, and `ZkStdLibArch` chips; exposes `Q-wrap-1` against `WRP-002` + `WRP-003`. |
| `DG-003` | Modeled (`inv_WRP_004`, `inv_RKY_004`, `MAX_VK_BYTES = 50000`) | Wrap verifier key `<= 50,000` bytes. |
| `GPW-001` | Modeled (`inv_GPW_003`, `doConfigureGpuProver`, `doProveWrapCpu`, `wp_gpuSharesKeyDetSeedMatchesAndCpuFallbackTest`) | GPU proving path with stock CPU fallback when `usableGpuPresent == false`, and byte-equal seeded output. |
| `GPW-003` | Modeled (`inv_GPW_003`, `doProveSeededDetWrap`, mutant `wrap_m_gpw003_det_mismatch`) | Under `TestSeedBuild` + `GW_DET=1`, CPU and GPU wrap proofs of one call and seed share one `proofBytesTag` (proof SHA-256) and pass verification. |
| `GPW-007` | Not modelled here (`SPEC-CHECKED candidate`) | Operations/benchmark reporting rule requiring run records, host load, and `heavy-run` serialization for `MEASURED` timing claims; owns no Wrap protocol state. |
| `LW-002` | Modeled (Wrap-side clause: `inv_LW_002`, `LFR_TRANSCRIPT_U = 7`, mutant `wrap_m_lw002_unverified_lfr`) | Wrap verifies `L_Fr` natively in `Fr` under the frozen `L_Fr` key and `u = 7` transcript (`L`-inside-`L_Fr` schedule verification belongs to Bridge). |
| `LW-003` | Modeled (`inv_LW_003`, `CALL_VECTOR_PUBLIC_INPUTS = 1026`, mutant `wrap_m_lw003_public_leak`) | Call vector has `1,026` public inputs (`lane 0 = binding_input`, `lane 1 = comm`, constrained zero padding, no leaked private word). |
| `LW-004` | Modeled (`inv_LW_004`, `validLw004`, mutant `wrap_m_lw004_bad_field_log`) | Native field-log replay check (`Canon`, bounded quotients, canonical squeezes, `pv` `eq`-binding, padding, reserved `R` entries) required on every accepted wrap. |
| `LW-005` | Modeled (`inv_LW_005`, `expectedAInTag`, `expectedAOutTag`, mutant `wrap_m_lw005_swapped_tag`) | Enforces `popeq == a_in`, `DIGEST` log provenance, `SHK_PROGC` vs `SHK_LOCK` tags across `OrdMode` and `ChunkMode` (`j = 1..k`), `lane 0 == bind`, and `layout_d == keyConst`. |
| `LW-006` | Modeled (`inv_LW_006`, mutant `wrap_m_lw006_x_mismatch`) | Canonical `X` limbs equal to the cached-trace commitment of `L`. |
| `LW-012` | Modeled (`inv_WRP_006`, `inv_ZK_003`) | Fresh CSPRNG blinders in production; seeded wrap proofs allowed only in test mode. |
| `LW-013` | Modeled (`inv_DG_002`, `inv_WRP_002`) | `ZkStdLibArch` circuit at `k <= 21` with no foreign-field, pairing, SHA chip, or G4 lane gate. |
| `LW-014` | Modeled (`inv_LW_014`, `MIN_LOG_RESERVATION_R = 8`, `doRefuseKeygenLw014`, mutant `wrap_m_lw014_short_log_cap`) | Keygen refuses `logCap < MIN_LOG_RESERVATION_R`, frozen `L`/`L_Fr` key drift, or a second call-time `Fr` fallback engine. |
| `LW-016` | Modeled (`inv_LW_016`, `doRecordT25LwPasses`, `doRefuseIncompleteT25Qualification`, mutant `wrap_m_lw016_all_nop_qual`) | An `L_cached` all-`NOP` benchmark qualifies no gate (`not(wrp.t25UsedAllNopBench)`). |
| `MID-001` | Partially modelled / external (`inv_WRP_001`, `inv_DG_002`) | Wrap-side output format (`ZkStdLibArch` + PLONK/KZG/BLS12-381/Blake2b) is modelled; external Midnight ledger 9.1.0.1 `VerifierKey::verify` execution belongs to Ledger Verification. |
| `MID-002` | Modeled (`inv_WRP_010`, `doDeployProgramLedger`) | Sherlock write entries (`acceptRun`, `acceptCollection`) install the universal wrap VK while read entries keep native keys (`readEntriesKeepNativeVk == true`). |
| `MID-003` | Modeled (`consistency_constants`, `inv_WRP_004`, `inv_LW_003`) | Checks `MAX_VK_BYTES <= 50,000` and `CALL_VECTOR_PUBLIC_INPUTS == 1,026 <= 16,384 (2^14) <= 65,536` (`tx <= 1,048,576` bytes belongs to Settlement/Network Operations). |
| `ZK-003` | Modeled (`inv_ZK_003`, `doProveSeededDetWrap`, `doRefuseSeededBlinderInProd`, mutant `wrap_m_zk003_seeded_in_prod`) | Seeded blinders accepted solely in `TestSeedBuild` under `GW_DET=1` and refused in `ProdBuild`. |

---

## 2. Findings (`Q-wrap-1` .. `Q-wrap-6`)

### `Q-wrap-1`: `WRP-002` and `WRP-003` leave `k < 21` (`k <= 18` and `k in {19, 20}`) unguarded against row overflow, conflicting with `DG-002` (and `RKY-003` hardcodes `k = 21`)

- **IDs**: `WRP-002`, `WRP-003`, `RKY-003`, `DG-002`
- **Quoted sentences**:
  - `WRP-002`: *"If a caller asks for a wrap circuit size above k = 21, then the wrap tool SHALL stop before keygen with the message `never k = 22`."*
  - `WRP-003`: *"If the cost model of the wrap relation gives more than 1,782,573 rows at k = 21, then the wrap tool SHALL stop before keygen."*
  - `RKY-003`: *"When the rekey completes, the rekey runner SHALL measure the wrap at `k = 21` with at most 1,782,573 rows."*
  - `DG-002`: *"The wrap SHALL fit 85 percent of the `2^k - 7` usable rows at the smallest k of 19, 20 and 21 at which the measured wrap rows fit."*
- **Witness state / trace**:
  - Pure witness `gap_q_wrap_1_k20_row_overflow` (verified in `wp_rekeyBoundsAndSpecGapWitnessesTest`) and mutant `wrap_m_dg002_k20_row_overflow`:
  - At `requestedK = 20` and T10m measured `measuredRows = 1,685,845`:
    - `WRP-002` passes because `20 <= 21` (`inv_WRP_002` exits `0` on `wrap_m_dg002_k20_row_overflow`),
    - `WRP-003` does not fire because `requestedK != 21` and `1,685,845 <= 1,782,573` (`inv_WRP_003` exits `0` on `wrap_m_dg002_k20_row_overflow`),
    - Yet `1,685,845 > 891,283 = floor(0.85 * (2^20 - 7))` and `1,685,845 > 1,048,569 = 2^20 - 7`, overflowing both `DG-002` (`inv_DG_002` exits `1`) and the entire `2^20` evaluation domain. Furthermore, `WRP-002` does not refuse `k <= 18` even though `DG-002` restricts `k` to `{19, 20, 21}`, and `RKY-003` hardcodes `k = 21` even if `DG-002`'s smallest-fitting-`k` rule selects `k = 20` or `k = 19`.
- **What would close it**:
  - Require `k in {19, 20, 21}` in `WRP-002` (refusing `k < 19` as well as `k > 21`), parameterize `WRP-003` as `rows <= floor(0.85 * (2^k - 7))` (`445,638` at `k = 19`, `891,283` at `k = 20`, `1,782,573` at `k = 21`), and align `RKY-003` with `DG-002`'s chosen `k`.

### `Q-wrap-2`: Unqualified OS CSPRNG mandate in `WRP-006` contradicts deterministic `test-seed` (`GW_DET=1`) GPU/CPU byte-identity proving in `ZK-003`, `LW-012`, `GPW-001`, and `GPW-003`

- **IDs**: `WRP-006`, `ZK-003`, `LW-012`, `GPW-001`, `GPW-003`
- **Quoted sentences**:
  - `WRP-006`: *"The wrap prover SHALL draw the blinders of each wrap proof from the operating system CSPRNG."*
  - `LW-012`: *"Every wrap proof SHALL use fresh CSPRNG blinders. A seeded wrap proof SHALL exist only in a test mode (ZK-003)."*
  - `ZK-003`: *"A seeded blinder SHALL be accepted only by a build with the cargo feature `test-seed` (ZK-014). Byte-identity tests of the GPU wrap SHALL use the test-only seed patch under `GW_DET=1` in a `test-seed` build."*
  - `GPW-003`: *"Under `GW_DET=1`, every CPU and GPU configuration SHALL give one proof SHA-256."*
- **Witness state**:
  - In `WrapProofRunRecord` with `buildMode = TestSeedBuild`, `gwDet = true`, `detSeed = 42`, `blinderSource = SeededDet`, CPU (`device = CpuDevice`) and GPU (`device = GpuDevice`) produce matching `proofBytesTag` as mandated by `GPW-001`/`GPW-003`/`ZK-003`, which violates `WRP-006`'s unqualified normative sentence (`blinderSource == OsCsprng` on every wrap proof; only `WRP-006`'s non-normative Scenario mentions *"in a production build"*). Conversely, drawing from the OS CSPRNG under `GW_DET=1` yields distinct CPU and GPU proof bytes, violating `GPW-003`.
- **What would close it**:
  - Qualify `WRP-006`'s normative EARS sentence explicitly: *"Where the wrap prover is built without the `test-seed` feature, the wrap prover SHALL draw the blinders of each wrap proof from the operating system CSPRNG; a seeded blinder SHALL be accepted only in a `test-seed` build under `ZK-003`."*

### `Q-wrap-3`: Literal `WRP-007` requires `lane 15 == 15` on every anchor prologue, rejecting valid ordinary (`ORD`) calls that carry no chunk anchor write

- **IDs**: `WRP-007`, `ANC-001`, `LW-003`, `LW-005`
- **Quoted sentences**:
  - `WRP-007`: *"When the wrap decoder checks the anchor prologue, the wrap circuit SHALL require the anchor read key in lane 6 and the chunk write key in lane 15 to equal the key constant 15."*
  - `ANC-001`: *"The anchor cell SHALL be the top-level ledger slot 15, a key constant of the wrap (`ANCHOR_SLOT`, template lanes 6 and 15). ... User code SHALL NOT address slot 15."*
  - `LW-003`: *"The wrap SHALL have exactly the call vector of `2 + L_TX` words as public inputs, with `L_TX` = 1,024 (OD-7, OD-15). Lane 0 SHALL be `binding_input`. Lane 1 SHALL be `comm`. Unused lanes SHALL be constrained zero."*
- **Witness state**:
  - Pure witness `gap_q_wrap_3_ord_lane15_zero` (`mkHonestLfrEvent(1, 1, OrdMode, 1, 1)`):
  - In `OrdMode` (non-chunked call, `chunkCount = 1`), the anchor cell is read in lane `6` (`lane6ReadKey = 15`), while the chunk anchor-write template in lanes `11..19` and `21` is absent (`hasChunkWritePrologue = false`; `WRP-007` Status `spec.md:85` explicitly notes `ANC-016`: *"an ordinary call with the anchor-write template in lanes 11 to 19 and 21 is refused"*). Unused lane `15` is constrained to `0` by `LW-003` (or holds a user effect key `!= 15` by `ANC-001`). Literal unconditioned `WRP-007` (`lane 6 == 15` **and** `lane 15 == 15`) therefore rejects every valid `OrdMode` call (`0 != 15`).
- **What would close it**:
  - Amend `WRP-007` to require `lane 6 == 15` on every call (`ORD` and `CHUNK`), require `lane 15 == 15` *when `mode == CHUNK`*, and forbid the chunk anchor-write prologue when `mode == ORD` (`ANC-016`).

### `Q-wrap-4`: Unconditional `WRP-009` 3-file write on library verification conflicts with post-prove `WRP-004`/`WRP-008` stops, `WRP-004` permits partial 2-file `WrapOutput`, and `WRP-008` is bypassed when `E6_EXPECT_VK` is unset

- **IDs**: `WRP-004`, `WRP-008`, `WRP-009`, `WRP-010`
- **Quoted sentences**:
  - `WRP-004`: *"If the serialized wrap verifying key is larger than 50,000 bytes, then the wrap tool SHALL stop without writing `wrap.vk`."*
  - `WRP-008`: *"If the generated wrap key differs from the key file named by `E6_EXPECT_VK`, then the wrap tool SHALL stop before writing an output file."*
  - `WRP-009`: *"When the library verifier accepts a new wrap proof, the wrap tool SHALL write the 3 files `wrap.vk`, `wrap.proof` and `wrap.public.json` into the output folder."*
  - `WRP-010`: *"The Sherlock key set SHALL hold exactly 1 wrap verifying key for every program, both modes and every ledger deployment of one era."*
- **Witness state**:
  - (a) Suppose the wrap tool proves and `libraryVerified = true` (`WRP-009` trigger), and then serializes/checks the key (`wrap.rs:81-92`) finding `vkBytes = 50,001` (`WRP-004`) or `vkId != expectedVkId` (`WRP-008`): `WRP-009` unconditionally requires writing all 3 files `{"wrap.vk", "wrap.proof", "wrap.public.json"}`, whereas `WRP-004` forbids writing `"wrap.vk"` and `WRP-008` forbids writing any output file. Moreover, `WRP-004` only forbids `"wrap.vk"`, leaving `{"wrap.proof", "wrap.public.json"}` legal under `WRP-004` alone despite breaking `WrapOutput` 3-file atomicity (`WRP-009`).
  - (b) When `expectVkSet = false` (`E6_EXPECT_VK` unset in the caller's environment), `WRP-008`'s `If` guard is vacuously false, allowing a regenerated key that drifted from the era's deployed universal key (`WRP-010`) to write `WrapOutput` undetected.
- **What would close it**:
  - Enforce `vkBytes <= 50,000` (`WRP-004`) and mandatory equality against the pinned universal key manifest (`WRP-008`, `WRP-010`) *before* proving (or condition `WRP-009` on both checks passing), and make `WRP-004` forbid writing *any* of the 3 `WrapOutput` files.

### `Q-wrap-5`: `WRP-012` limits the T25 wrap qualification gate record to 5 `LW` IDs, omitting `LW-002` (`L_Fr` verifier inside wrap), `LW-013` (`ZkStdLib` architecture), and `LW-016` (no all-`NOP` `L_cached` qualification)

- **IDs**: `WRP-012`, `LW-002`, `LW-013`, `LW-016`
- **Quoted sentences**:
  - `WRP-012`: *"When the wrap is qualified on the T25 key set, the wrap qualification record SHALL hold a pass result for each of the 5 requirements LW-003, LW-004, LW-005, LW-006 and LW-014."*
  - `LW-002`: *"The wrap SHALL verify the proof of `L_Fr` natively in Fr under the frozen `L_Fr` key and the `L_Fr` transcript (u = 7, HDM-001)."*
  - `LW-016`: *"The wrap SHALL be qualified on OD-24 option E (step E5), as the stdlib wrap of `L_Fr` over the complete `L` interface ... with `R` as real non-`NOP` entries, on the full tree and on path C at the worst admitted shapes. ... A benchmark on `L_cached` with an all-`NOP` log SHALL qualify no gate (FZ-3)."*
- **Witness state**:
  - In mutant `wrap_m_lw016_all_nop_qual`, a T25 qualification record with `t25PassedChecks = Set("LW-003", "LW-004", "LW-005", "LW-006", "LW-014")` collected on an `L_cached` all-`NOP` benchmark (`t25UsedAllNopBench = true`) or without a pass check for `LW-002` satisfies `inv_WRP_012` completely while violating `inv_LW_016` (`LW-016`).
- **What would close it**:
  - Expand `WRP-012`'s required T25 qualification record to include `LW-002`, `LW-013`, and `LW-016` (attesting real non-`NOP` `R` entries over the full `12,166`-word `L` interface on both the full tree and path C).

### `Q-wrap-6`: Unqualified `WRP-001` `SHALL produce` upon `LfrProofReady` lacks an `Unless` clause for forged witnesses or key-gate stops, and `WRP-005` is missing from `spec.md`

- **IDs**: `WRP-001`, `WRP-002`, `WRP-003`, `WRP-004`, `WRP-007`, `WRP-008` (and missing `WRP-005`)
- **Quoted sentences**:
  - `WRP-001`: *"When an `LfrProofReady` event arrives for a ledger call, the wrap prover SHALL produce exactly 1 midnight-proofs PLONK proof with KZG over BLS12-381 and the Blake2b transcript."*
- **Witness state**:
  - When `LfrProofReady` arrives with a forged slot `lane6ReadKey = 3 != 15` (`callId = 3` in `doReceiveForgedLfrCall`), or while `expectVkSet = true` with `expectedVkId = 99 != 1`, `WRP-007` / `WRP-008` require refusing the run (`proofCountInRun = 0`), contradicting `WRP-001`'s unconditional obligation to produce `1` proof on every `LfrProofReady` arrival. Separately, `sherlock-e2e-wrap/spec.md` skips from `WRP-004` (`spec.md:45`) to `WRP-006` (`spec.md:59`) with no tombstone for `WRP-005`.
- **What would close it**:
  - Condition `WRP-001` on passing the wrap key gates (`WRP-002`, `WRP-003`, `WRP-004`, `WRP-008`) and circuit/decoder witness checks (`WRP-007`, `ANC-001`, `LW-002..006`), and record a tombstone note for unused ID `WRP-005`.

---

## 3. Mutant Table

Every mutant (`MUTANT_ID = 1..24`) weakens exactly one refusal or binding guard while preserving all honest execution paths (verified non-vacuous: `reach_cpu_and_gpu_wrap_output` exits `1` on all 24 mutants).

| Mutant module | Target invariant | Exit code (`quint run`) | Non-vacuity (`reach_cpu_and_gpu_wrap_output`) | Guard weakened |
|---|---|---|---|---|
| `wrap_m_wrp001_wrong_scheme` | `inv_WRP_001` | `1` (`[violation]`) | `1` (`[violation]`) | Emits `NonPlonkScheme` and `proofCountInRun = 2` on CPU wrap run |
| `wrap_m_wrp002_allow_k22` | `inv_WRP_002` | `1` (`[violation]`) | `1` (`[violation]`) | Allows keygen at `requestedK = 22` (`> MAX_WRAP_K`) |
| `wrap_m_wrp003_ignore_row_gate` | `inv_WRP_003` | `1` (`[violation]`) | `1` (`[violation]`) | Allows keygen at `k = 21` with `measuredRows = 1,782,574` |
| `wrap_m_wrp004_write_oversize_vk` | `inv_WRP_004` | `1` (`[violation]`) | `1` (`[violation]`) | Writes `"wrap.vk"` when `vkBytes = 50,001` |
| `wrap_m_wrp006_reuse_blinder` | `inv_WRP_006` | `1` (`[violation]`) | `1` (`[violation]`) | Uses `ConstantBad` (`blinderId = 0`) in `ProdBuild` |
| `wrap_m_wrp007_skip_slot15_pin` | `inv_WRP_007` | `1` (`[violation]`) | `1` (`[violation]`) | Skips decode step 5b slot-`15` / mode prologue pin |
| `wrap_m_wrp008_ignore_expect_vk` | `inv_WRP_008` | `1` (`[violation]`) | `1` (`[violation]`) | Writes `WrapOutput` despite `E6_EXPECT_VK` mismatch |
| `wrap_m_wrp009_partial_output` | `inv_WRP_009` | `1` (`[violation]`) | `1` (`[violation]`) | Writes only `{"wrap.vk", "wrap.proof"}` (omits `"wrap.public.json"`) |
| `wrap_m_wrp010_per_program_vk` | `inv_WRP_010` | `1` (`[violation]`) | `1` (`[violation]`) | Installs a second distinct wrap VK (`vkId = 2`) on deployment |
| `wrap_m_wrp011_divergent_gpu_vk` | `inv_WRP_011` | `1` (`[violation]`) | `1` (`[violation]`) | Configures GPU prover with `gpuWrapVkId != cpuWrapKey.vkId` |
| `wrap_m_wrp012_missing_lw004` | `inv_WRP_012` | `1` (`[violation]`) | `1` (`[violation]`) | Closes `t25Qualified` with `"LW-004"` missing |
| `wrap_m_rky003_row_overflow` | `inv_RKY_003` | `1` (`[violation]`) | `1` (`[violation]`) | Completes rekey with `rekeyMeasuredRows = 1,782,574` |
| `wrap_m_rky004_vk_overflow` | `inv_RKY_004` | `1` (`[violation]`) | `1` (`[violation]`) | Completes rekey with `rekeyMeasuredVkBytes = 50,001` |
| `wrap_m_dg002_k20_row_overflow` | `inv_DG_002` | `1` (`[violation]`) | `1` (`[violation]`) | Uses only literal `WRP-002`+`WRP-003` row check (`k = 20`, `1,685,845` rows) |
| `wrap_m_zk003_seeded_in_prod` | `inv_ZK_003` | `1` (`[violation]`) | `1` (`[violation]`) | Accepts `SeededDet` blinder in `ProdBuild` |
| `wrap_m_gpw003_det_mismatch` | `inv_GPW_003` | `1` (`[violation]`) | `1` (`[violation]`) | Perturbs GPU `proofBytesTag` (`+ 777`) under `TestSeedBuild` + `GW_DET=1` |
| `wrap_m_lw002_unverified_lfr` | `inv_LW_002` | `1` (`[violation]`) | `1` (`[violation]`) | Accepts unverified `L_Fr` proof / wrong transcript `u = 5` |
| `wrap_m_lw003_public_leak` | `inv_LW_003` | `1` (`[violation]`) | `1` (`[violation]`) | Accepts leaked private word and nonzero unused lane padding |
| `wrap_m_lw004_bad_field_log` | `inv_LW_004` | `1` (`[violation]`) | `1` (`[violation]`) | Accepts witness failing native field-log replay |
| `wrap_m_lw005_swapped_tag` | `inv_LW_005` | `1` (`[violation]`) | `1` (`[violation]`) | Accepts `TagLock` on an `OrdMode` call |
| `wrap_m_lw006_x_mismatch` | `inv_LW_006` | `1` (`[violation]`) | `1` (`[violation]`) | Accepts `X` unequal to cached `L` trace commitment |
| `wrap_m_lw014_short_log_cap` | `inv_LW_014` | `1` (`[violation]`) | `1` (`[violation]`) | Allows keygen with `logCap = 0 < MIN_LOG_RESERVATION_R` |
| `wrap_m_lw016_all_nop_qual` | `inv_LW_016` | `1` (`[violation]`) | `1` (`[violation]`) | Records T25 qualification passes from an all-`NOP` `L_cached` benchmark |
| `wrap_m_anc001_plain_progd` | `inv_ANC_001` | `1` (`[violation]`) | `1` (`[violation]`) | Accepts plain `prog_d` in anchor cell and user write to slot `15` |

---

## 4. State Vocabulary for Trace Replay

### 4.1 State Variables (`vars` in ITF JSON)

In `../traces/wrap_main/t_{0..19}.itf.json`, Quint qualifies the two module variables under `wrap_main::wrap::`:

| Quint `var` | ITF JSON state key | Type | Meaning |
|---|---|---|---|
| `wrp` | `wrap_main::wrap::wrp` | `WrapState` | Complete observable state of the **Wrap** context (keys, keygen attempts, `LfrProofReady` calls, proof runs, `WrapOutput` files, deployments, rekey measurements, T25 qualification record). |
| `lastOp` | `wrap_main::wrap::lastOp` | `str` | Name of the transition that produced the current state (`"init"` at state index `0`). |

#### `WrapState` fields (`wrap_main::wrap::wrp`)

| Field | Type | Meaning |
|---|---|---|
| `eraKeyInitialized` | `bool` | Whether the active era's universal wrap key has been generated. |
| `eraWrapVkSet` | `Set[int]` | Set of wrap VK IDs installed in the current era (`Set(1)` once initialized; `WRP-010`). |
| `cpuWrapKey` | `WrapKeyRecord` | Active CPU wrap key (`vkId`, `circuitK`, `measuredRows`, `vkBytes`, `arch`, `hasForeignOrG4Chip`, `logCap`, `lAndLfrKeysMatchFrozen`, `hasSecondFrFallback`). |
| `gpuWrapVkId` | `int` | VK ID configured on the GPU wrap prover (`0` when unconfigured; equals `cpuWrapKey.vkId` when configured; `WRP-011`). |
| `gpuConfigured` | `bool` | Whether GPU wrap proving is active. |
| `usableGpuPresent` | `bool` | Whether a usable GPU is present on the host (`GPW-001` CPU fallback when `false`). |
| `expectVkSet` | `bool` | Whether `E6_EXPECT_VK` is set in the wrap tool environment (`WRP-008`). |
| `expectedVkId` | `int` | VK ID named by `E6_EXPECT_VK` (`0` when unset; `1` matching; `99` stale/mismatching). |
| `nextAttemptId` | `int` | Monotonic ID counter (`1..7`) for `keygenAttempts`. |
| `keygenAttempts` | `Set[KeygenAttemptRecord]` | History of accepted and refused keygen / VK-size attempts (`WRP-002`, `WRP-003`, `WRP-004`, `LW-014`). |
| `lfrEvents` | `int -> LfrCallEvent` | Map from `callId` to received `LfrProofReady` payload and call-binding attributes. |
| `knownCallIds` | `Set[int]` | Domain of `lfrEvents` (`1, 2` for honest calls; `3, 4` for forged calls). |
| `nextRunId` | `int` | Monotonic ID counter (`1..9`) for `proofRuns`. |
| `freshBlinderSeq` | `int` | Monotonic OS CSPRNG blinder sequence counter (`WRP-006`). |
| `proofRuns` | `Set[WrapProofRunRecord]` | Executed wrap prover runs, including `device`, `buildMode`, `gwDet`, `blinderSource`, `blinderId`, `proofBytesTag`, `libraryVerified`, `stoppedOnVkMismatch`, and `writtenFiles`. |
| `lastRefusalReason` | `str` | Most recent refusal tag (`"none"`, `"never k = 22"`, `"row_gate_exceeded"`, `"vk_size_exceeded"`, `"lw014_keygen_refused"`, `"seeded_blinder_refused_in_prod"`, `"witness_refused"`, `"the wrap key differs"`, `"rekey_bounds_exceeded"`, `"t25_qualification_incomplete"`, `"all_nop_bench_qualifies_no_gate"`). |
| `deployments` | `Set[ProgramDeployRecord]` | Deployed program ledgers (`progId`, `installedWriteVkId`, `readEntriesKeepNativeVk`, `usesT25Key`; `WRP-010`, `WRP-012`, `MID-002`). |
| `rekeyCompleted` | `bool` | Whether rekey wrap measurement completed (`RKY-003`, `RKY-004`). |
| `rekeyMeasuredK` | `int` | Circuit `k` recorded at rekey completion (`21` when completed; `RKY-003`). |
| `rekeyMeasuredRows` | `int` | Cost-model row count measured at rekey completion (`RKY-003`). |
| `rekeyMeasuredVkBytes` | `int` | Serialized wrap VK byte length measured at rekey completion (`RKY-004`). |
| `rekeyLeverListWritten` | `bool` | Set to `true` when rekey stops due to row/VK bound overflow (`RKY-003` Scenario). |
| `t25PassedChecks` | `Set[str]` | Subset of `{"LW-003", "LW-004", "LW-005", "LW-006", "LW-014"}` holding pass results on T25 (`WRP-012`). |
| `t25UsedAllNopBench` | `bool` | Whether an `L_cached` all-`NOP` benchmark was admitted into `t25PassedChecks` (`false` in honest runs; `LW-016`). |
| `t25Qualified` | `bool` | Whether the T25 wrap qualification gate record is closed (`WRP-012`). |

### 4.2 `lastOp` Values and Implemented EARS IDs

| `lastOp` value | Action in `wrap.qnt` | Implemented EARS IDs |
|---|---|---|
| `"init"` | `init` | Initial state |
| `"keygenWrap"` | `doKeygenWrap` | `WRP-002`, `WRP-003`, `WRP-004`, `WRP-010`, `DG-002`, `DG-003`, `LW-013`, `LW-014` |
| `"refuseKeygenAboveK21"` | `doRefuseKeygenAboveK21` | `WRP-002` (`"never k = 22"`) |
| `"refuseKeygenRowGate"` | `doRefuseKeygenRowGate` | `WRP-003`, `DG-002` |
| `"refuseVkTooLarge"` | `doRefuseVkTooLarge` | `WRP-004`, `DG-003`, `MID-003` |
| `"refuseKeygenLw014"` | `doRefuseKeygenLw014` | `LW-014` |
| `"configureGpuProver"` | `doConfigureGpuProver` | `WRP-011`, `GPW-001` |
| `"configureExpectVk"` | `doConfigureExpectVk` | `WRP-008` |
| `"receiveHonestLfrCall"` | `doReceiveHonestLfrCall` | `WRP-001`, `ANC-001`, `LW-002`, `LW-003`, `LW-004`, `LW-005`, `LW-006` |
| `"receiveForgedLfrCall"` | `doReceiveForgedLfrCall` | `WRP-007`, `ANC-001`, `LW-002`, `LW-003`, `LW-004`, `LW-005`, `LW-006` |
| `"proveWrapCpu"` | `doProveWrapCpu` | `WRP-001`, `WRP-004`, `WRP-006`, `WRP-007`, `WRP-008`, `WRP-009`, `GPW-001`, `LW-012`, `ZK-003` |
| `"proveWrapGpu"` | `doProveWrapGpu` | `WRP-001`, `WRP-006`, `WRP-007`, `WRP-008`, `WRP-009`, `WRP-011`, `GPW-001` |
| `"proveSeededDetWrap"` | `doProveSeededDetWrap` | `GPW-001`, `GPW-003`, `LW-012`, `ZK-003` |
| `"refuseSeededBlinderInProd"` | `doRefuseSeededBlinderInProd` | `WRP-006`, `LW-012`, `ZK-003` |
| `"refuseInvalidWrapWitness"` | `doRefuseInvalidWrapWitness` | `WRP-007`, `ANC-001`, `LW-002`, `LW-003`, `LW-004`, `LW-005`, `LW-006` |
| `"refuseExpectVkMismatch"` | `doRefuseExpectVkMismatch` | `WRP-008` |
| `"deployProgramLedger"` | `doDeployProgramLedger` | `WRP-010`, `WRP-012`, `MID-002` |
| `"completeRekeyWrapMeasurement"` | `doCompleteRekeyWrapMeasurement` | `RKY-003`, `RKY-004` |
| `"refuseRekeyBoundsOverflow"` | `doRefuseRekeyBoundsOverflow` | `RKY-003`, `RKY-004` |
| `"recordT25LwQualificationPasses"` | `doRecordT25LwPasses` | `WRP-012`, `LW-016` |
| `"closeT25Qualification"` | `doCloseT25Qualification` | `WRP-012`, `LW-016` |
| `"refuseIncompleteT25Qualification"` | `doRefuseIncompleteT25Qualification` | `WRP-012`, `LW-016` |

### 4.3 Concrete Instance Constants

| Constant | `wrap_main` | `wrap_edge_min` | `wrap_edge_at_limit` | `wrap_m_*` (`24` mutants) |
|---|---|---|---|---|
| `MIN_WRAP_K` | `19` | `19` | `21` | `19` |
| `MAX_WRAP_K` | `21` | `19` | `21` | `21` |
| `MAX_ROWS_AT_K21` | `1782573` | `445638` | `1782573` | `1782573` |
| `MAX_VK_BYTES` | `50000` | `1325` | `50000` | `50000` |
| `CALL_VECTOR_PUBLIC_INPUTS` | `1026` | `1026` | `1026` | `1026` |
| `ANCHOR_SLOT_CONST` | `15` | `15` | `15` | `15` |
| `ANCHOR_READ_LANE` | `6` | `6` | `6` | `6` |
| `CHUNK_WRITE_LANE` | `15` | `15` | `15` | `15` |
| `WRAP_OUTPUT_FILE_COUNT` | `3` | `3` | `3` | `3` |
| `LW_QUAL_CHECK_COUNT` | `5` | `5` | `5` | `5` |
| `LFR_TRANSCRIPT_U` | `7` | `7` | `7` | `7` |
| `MIN_LOG_RESERVATION_R` | `8` | `1` | `8` | `8` |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 24` |

---

## 5. Model Abstractions

1. **Cryptographic primitives and proof bytes**: BLS12-381 PLONK KZG proving, Blake2b transcripts, and SHA-256 digests are abstracted as finite tags (`ProofScheme`, `TranscriptKind`, `vkId: int`, `blinderId: int`, and injective `computeProofBytesTag(callId, vkId, blinderId)`). This preserves collision-freeness across distinct CSPRNG draws (`WRP-006`) and exact byte equality under shared `(callId, vkId, detSeed)` in `TestSeedBuild` + `GW_DET=1` (`GPW-003`).
2. **`1,026`-word call vector and anchor prologue**: Rather than storing a 1,026-element `Fr` array per event, `LfrCallEvent` stores `publicInputCount` (`1026`), explicit boolean validity flags for `lane 0 == binding_input`, `lane 1 == comm`, zero-constrained unused lanes, absence of private-word leaks (`LW-003`), explicit integers for `lane6ReadKey` and `lane15WriteKey`, and `hasChunkWritePrologue` (`WRP-007`, `ANC-001`).
3. **In-circuit `L_Fr`, field-log, and DAG commitment checks**: Native `Fr` WHIR/sumcheck verification of `L_Fr` (`LW-002`), native field-log replay (`LW-004`), `DIGEST`/tag checks (`LW-005`), and canonical `X` commitment comparison (`LW-006`) are represented as structured finite attributes on `LfrCallEvent` (`lfrValidInFr`, `lfrTranscriptU`, `fieldLogValid`, `aInTag`, `aOutTag`, `dagXCanonAndEqCached`) with dedicated forged-event generators (`forgeryKind = 1..8`).
4. **Filesystem output (`WrapOutput`)**: The output directory is modelled as `writtenFiles: Set[str]` over `"wrap.vk"`, `"wrap.proof"`, and `"wrap.public.json"` (`WRP-004`, `WRP-008`, `WRP-009`).
5. **Bounded trace pools**: Random simulation bounds `nextAttemptId <= 6`, `nextRunId <= 8`, `callId in {1, 2, 3}`, and `progId in {1, 2}` so 15-step traces exercise both multi-run CSPRNG freshness (`WRP-006`) and CPU/GPU byte-identity pairs (`GPW-003`) without state explosion.

---

## 6. Commands

Run from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/wrap/quint`:

```bash
# 1. Typecheck specification and test suite
quint typecheck wrap.qnt
# Expected exit code: 0 (no output on success)

quint typecheck wrap_test.qnt
# Expected exit code: 0 (no output on success)

# 2. Deterministic scenario tests
quint test --main=wrap_test --match='wp_.*Test' --seed=0x20261115 wrap_test.qnt
# Expected exit code: 0
# Printed: 11 passing (4080ms)

# 3. Honest instances (main and edge bounds)
quint run wrap.qnt --main wrap_main --invariant inv --max-samples 2000 --max-steps 15 --seed 0x20261115
# Expected exit code: 0
# Printed: [ok] No violation found (3040ms at 658 traces/second).

quint run wrap.qnt --main wrap_edge_min --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 0
# Printed: [ok] No violation found (1978ms at 506 traces/second).

quint run wrap.qnt --main wrap_edge_at_limit --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 0
# Printed: [ok] No violation found (1719ms at 582 traces/second).

# 4. Non-vacuity reachability witnesses (all 7 MUST violate, exit code 1)
quint run wrap.qnt --main wrap_main --invariant reach_cpu_and_gpu_wrap_output --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (380ms at 45 traces/second).

quint run wrap.qnt --main wrap_main --invariant reach_two_prod_proofs_differ --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (394ms at 43 traces/second).

quint run wrap.qnt --main wrap_main --invariant reach_det_seed_gpu_cpu_byte_equal --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (484ms at 915 traces/second).

quint run wrap.qnt --main wrap_main --invariant reach_keygen_k22_and_row_refusals --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (312ms at 13 traces/second).

quint run wrap.qnt --main wrap_main --invariant reach_witness_and_expect_vk_refusals --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (448ms at 138 traces/second).

quint run wrap.qnt --main wrap_main --invariant reach_ord_and_chunk_modes_two_programs --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (587ms at 1218 traces/second).

quint run wrap.qnt --main wrap_main --invariant reach_t25_qualification_and_rekey --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (243ms at 82 traces/second).

# 5. Guard mutants (all 24 MUST violate their named invariant, exit code 1)
quint run wrap.qnt --main wrap_m_wrp001_wrong_scheme --invariant inv_WRP_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (707ms at 11 traces/second).

quint run wrap.qnt --main wrap_m_wrp002_allow_k22 --invariant inv_WRP_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (471ms at 2 traces/second).

quint run wrap.qnt --main wrap_m_wrp003_ignore_row_gate --invariant inv_WRP_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (549ms at 22 traces/second).

quint run wrap.qnt --main wrap_m_wrp004_write_oversize_vk --invariant inv_WRP_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (633ms at 14 traces/second).

quint run wrap.qnt --main wrap_m_wrp006_reuse_blinder --invariant inv_WRP_006 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (403ms at 20 traces/second).

quint run wrap.qnt --main wrap_m_wrp007_skip_slot15_pin --invariant inv_WRP_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (282ms at 39 traces/second).

quint run wrap.qnt --main wrap_m_wrp008_ignore_expect_vk --invariant inv_WRP_008 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (342ms at 70 traces/second).

quint run wrap.qnt --main wrap_m_wrp009_partial_output --invariant inv_WRP_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (335ms at 24 traces/second).

quint run wrap.qnt --main wrap_m_wrp010_per_program_vk --invariant inv_WRP_010 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (385ms at 52 traces/second).

quint run wrap.qnt --main wrap_m_wrp011_divergent_gpu_vk --invariant inv_WRP_011 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (375ms at 21 traces/second).

quint run wrap.qnt --main wrap_m_wrp012_missing_lw004 --invariant inv_WRP_012 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (488ms at 16 traces/second).

quint run wrap.qnt --main wrap_m_rky003_row_overflow --invariant inv_RKY_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (401ms at 2 traces/second).

quint run wrap.qnt --main wrap_m_rky004_vk_overflow --invariant inv_RKY_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (406ms at 10 traces/second).

quint run wrap.qnt --main wrap_m_dg002_k20_row_overflow --invariant inv_DG_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (626ms at 10 traces/second).

quint run wrap.qnt --main wrap_m_zk003_seeded_in_prod --invariant inv_ZK_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (288ms at 3 traces/second).

quint run wrap.qnt --main wrap_m_gpw003_det_mismatch --invariant inv_GPW_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (776ms at 571 traces/second).

quint run wrap.qnt --main wrap_m_lw002_unverified_lfr --invariant inv_LW_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (303ms at 36 traces/second).

quint run wrap.qnt --main wrap_m_lw003_public_leak --invariant inv_LW_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (305ms at 36 traces/second).

quint run wrap.qnt --main wrap_m_lw004_bad_field_log --invariant inv_LW_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (378ms at 29 traces/second).

quint run wrap.qnt --main wrap_m_lw005_swapped_tag --invariant inv_LW_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (354ms at 31 traces/second).

quint run wrap.qnt --main wrap_m_lw006_x_mismatch --invariant inv_LW_006 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (544ms at 20 traces/second).

quint run wrap.qnt --main wrap_m_lw014_short_log_cap --invariant inv_LW_014 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (290ms at 3 traces/second).

quint run wrap.qnt --main wrap_m_lw016_all_nop_qual --invariant inv_LW_016 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (208ms at 38 traces/second).

quint run wrap.qnt --main wrap_m_anc001_plain_progd --invariant inv_ANC_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (294ms at 37 traces/second).

# 6. Replay trace generation (20 ITF JSON traces)
mkdir -p /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/wrap/traces/wrap_main
quint run wrap.qnt --main wrap_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/wrap/traces/wrap_main/t_{seq}.itf.json'
# Expected exit code: 0
# Printed: [ok] No violation found (1080ms at 19 traces/second).
```
