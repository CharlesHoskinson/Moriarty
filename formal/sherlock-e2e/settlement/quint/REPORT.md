# Quint Formal Model Report — Bounded Context **Settlement** (`e2e-settlement`)

Independent Quint 0.32.0 specification (`settlement.qnt`), scenario test suite (`settlement_test.qnt`), 25 guard mutants, 6 reachability witnesses, and 20 ITF replay traces (`../traces/settlement_main/t_{0..19}.itf.json`) derived solely from the assigned EARS requirements and `DDD.md` Settlement rows.

---

## 1. Requirement Coverage Table

### 1.1 Own (`sherlock-e2e-prototype-20261115`) and Completion (`sherlock-completion-20261115`) IDs

| ID | Artifact (`settlement.qnt` / `settlement_test.qnt`) | Kind | Note |
|---|---|---|---|
| `SLC-001` | `inv_SLC_001`, `CONTRACT_ENTRY_COUNT = 4`, `PROTOTYPE_ENTRIES`, `doDeployProgramLedger`, `st_deployEntriesSlot15AndT25ReadbackTest`, mutant `settlement_m_slc001_open_claim_entry` | `CONSTANT-CHECKED` | Every deployed `SherlockLedger` exports exactly the `4` entries `{"acceptRun", "acceptCollection", "rootAccepted", "runAccepted"}` (no `"openClaim"`, no `"descend"`). |
| `SLC-002` | `inv_SLC_002`, `ANCHOR_SLOT = 15`, `doDeployProgramLedger`, `st_deployEntriesSlot15AndT25ReadbackTest`, mutant `settlement_m_slc002_unsealed_anchor` | `CONSTANT-CHECKED` | Every deployed `SherlockLedger` holds its anchor in top-level ledger slot `15` as a constructor-sealed field (`anchorSealedInConstructor == true`). |
| `SLC-003` | `inv_SLC_003`, `doDeployProgramLedger`, `st_deployEntriesSlot15AndT25ReadbackTest`, mutant `settlement_m_slc003_wrong_write_vk` | `MODELED` | Deploy runner installs `universalWrapVkId` on `acceptRun` and `acceptCollection`. |
| `SLC-004` | `inv_SLC_004`, `doDeployProgramLedger`, `st_deployEntriesSlot15AndT25ReadbackTest`, mutant `settlement_m_slc004_wrap_key_on_read_entry` | `MODELED` | Deploy runner installs compiled native keys on `rootAccepted` and `runAccepted` (`readEntriesKeepNativeVk == true`); unspecified read-entry signatures noted in `Q-settlement-5`. |
| `SLC-005` | `inv_SLC_005`, `doDeployProgramLedger`, `doAcceptRun`, `st_acceptRunGuardsAndSingleSettleReplayRefusalTest`, mutant `settlement_m_slc005_cross_program_settle` | `MODELED` | Constructor anchor equals `c_p` of the deployed program and refuses settling program B on program A's ledger; direct contradiction with adopted `LDG-001` noted in `Q-settlement-1`. |
| `SLC-006` | `inv_SLC_006`, `STATEMENT_PUBLIC_WORDS = 4`, `doBuildAcceptArgs`, `st_argumentBuilderEffectDeltaWitnessSaltAndLaneCheckTest` | `CONSTANT-CHECKED` | Argument builder maps `sherlock-accept-bundle/1` into `4`-word `StatementPublics {c_p, input_pub, outcome_pub, claim_pub}` (`acceptRun` takes the first 3 words). |
| `SLC-007` | `deriveOutcomeD`, `deriveOutcomePub`, `doBuildAcceptArgs`, `st_argumentBuilderEffectDeltaWitnessSaltAndLaneCheckTest` | `MODELED` | Halting Mycroft outcome forms `Halted([comm, E_0 .. E_(L_e-1)])` in call order; changing an effect word changes `outcome_pub` and `claim_pub` while preserving `c_p` and `input_pub`. |
| `SLC-008` | `inv_SLC_008`, `RUN_SALT_WORDS = 8`, `deriveInputD`, `doBuildAcceptArgs`, `st_argumentBuilderEffectDeltaWitnessSaltAndLaneCheckTest`, mutant `settlement_m_slc008_unsalted_or_nonzero_fuel` | `CONSTANT-CHECKED` | Mycroft input digest uses `fuel = 0`, `stack_limit = 0`, and `8` CSPRNG run-salt words; distinct private tapes yield distinct `input_pub` words. |
| `SLC-009` | `inv_SLC_009`, `doBuildAcceptArgs`, `doRefuseLaneMismatch`, `st_argumentBuilderEffectDeltaWitnessSaltAndLaneCheckTest`, mutant `settlement_m_slc009_write_args_on_lane_mismatch` | `MODELED` | Raises `"Mismatch"` and writes no argument file when recomputed public words differ from call-vector lanes `10, 20, 34, 39`; chunk-2 `lane 10` lock conflict and unmapped `input_pub` noted in `Q-settlement-2`. |
| `SLC-010` | `inv_SLC_010`, `doAcceptRun`, `doClassifySuiteCall`, `st_acceptRunGuardsAndSingleSettleReplayRefusalTest`, mutant `settlement_m_slc010_settle_natively_refused` | `MODELED` | Natively refused suite calls never send `acceptRun` (`wasNativeRefused == false` on all settled runs). |
| `SLC-011` | `inv_SLC_011`, `doStartSuiteReplay`, `st_chunkCapDefaultIntentTtlAndInstanceReuseTest`, mutant `settlement_m_slc011_reuse_dirty_instance` | `MODELED` | Deploys a fresh ledger instance when replay starts on state `!=` deploy-block state and reuses the instance when equal; interaction with `BAT-004` `<= 106` deploy ceiling noted in `Q-settlement-6`. |
| `SLC-012` | `inv_SLC_012`, `doAcceptRun`, `st_acceptRunGuardsAndSingleSettleReplayRefusalTest`, mutant `settlement_m_slc012_settle_unbound_stmt` | `MODELED` | Refuses live settlement whenever leaf statement fails to bind `prog_d`, `input_d`, and `outcome_d`. |
| `BAT-001` | `inv_BAT_001`, `EPOCH_WINDOW_S = 3600`, `MAX_CLAIMS_PER_EPOCH = 1048576`, `doArriveOperatorJob`, `doCloseEpoch`, `st_epochBatchCollectionAndConsumerSingleUseAcrossSourcesTest`, mutant `settlement_m_bat001_late_epoch_close` | `CONSTANT-CHECKED` | Closes non-empty epochs at the earlier of `3,600 s` after the first job and `1,048,576` (`2^20`) claims; interaction with `BAT-009` and `BAT-010` noted in `Q-settlement-3`. |
| `BAT-002` | `inv_BAT_002`, `doArriveOperatorJob`, `doAcceptRun`, `st_epochBatchCollectionAndConsumerSingleUseAcrossSourcesTest`, mutant `settlement_m_bat002_operator_accept_run` | `MODELED` | Operator-submitted jobs route to the epoch collection and send no `acceptRun`; lack of on-chain contract enforcement noted in `Q-settlement-4`. |
| `BAT-003` | `consistency_bat003_daily_batch_rates`, `st_eraEndBoundariesSuiteClassificationAndSpecGapsTest` | `SPEC-CHECKED candidate` | Reporting format requirement for settlement report tables (`24 * 3600 == 86400`, `24 < 492` daily writes checked purely; owns no protocol transition state). |
| `BAT-004` | `consistency_bat004_t23_counts`, `st_eraEndBoundariesSuiteClassificationAndSpecGapsTest` | `SPEC-CHECKED candidate` | Suite report summary rule (`208 - 62 - 1 == 145` user `acceptRun`s, `33 <= 106` deploys checked purely; tension with `SLC-005` program vs circuit wording and `SLC-011` dirty re-deploys noted in `Q-settlement-1` and `Q-settlement-6`). |
| `BAT-005` | `inv_BAT_005`, `K_MAX_CHUNKS = 2`, `doPlanEntryChunks`, `doRefuseOverChunkedEntry`, `st_chunkCapDefaultIntentTtlAndInstanceReuseTest`, mutant `settlement_m_bat005_allow_3_chunks` | `CONSTANT-CHECKED` | Refuses any entry requiring `> 2` chunks at plan time at every intent TTL (including `14,400 s`). |
| `BAT-006` | `inv_BAT_006`, `DEFAULT_INTENT_TTL_S = 3600`, `doBuildPreviewIntent`, `st_chunkCapDefaultIntentTtlAndInstanceReuseTest`, mutant `settlement_m_bat006_default_ttl_14400` | `CONSTANT-CHECKED` | Preview intents built without an explicit user TTL assign `3,600 s` (refusing legacy `14,400 s` default). |
| `BAT-007` | `inv_BAT_007`, `CLAIMS_TREE_DEPTH = 20`, `doCloseEpoch`, `st_epochBatchCollectionAndConsumerSingleUseAcrossSourcesTest`, mutant `settlement_m_bat007_missing_paths` | `CONSTANT-CHECKED` | Epoch close delivers slot index and `20` sibling words to each job's submitter. |
| `BAT-008` | `inv_BAT_008`, `CLAIM_LEAF_BYTES = 32`, `doCloseEpoch`, `doConsumerUseClaim`, `st_epochBatchCollectionAndConsumerSingleUseAcrossSourcesTest`, mutant `settlement_m_bat008_unpublish_leaf_list` | `CONSTANT-CHECKED` | Epoch close publishes the off-chain leaf list at `32` bytes per claim, and consumer use admits no non-member or withheld-list claim (`inv_LDG_004`). |
| `BAT-009` | `inv_BAT_009`, `doEnableLCollectionKey`, `doAcceptCollection`, `doRefuseCollectionWithoutLKey`, `st_epochBatchCollectionAndConsumerSingleUseAcrossSourcesTest`, mutant `settlement_m_bat009_collection_without_l_key` | `MODELED` | Refuses `acceptCollection` while `not(hasLCollectionKey)`; deadlock of operator jobs under `BAT-002` + `BAT-009` noted in `Q-settlement-3`. |
| `BAT-010` | `inv_BAT_010`, `COLLECTION_PROOF_TIME_S = 176`, `INCLUSION_TIME_S = 60`, `BLOCK_INTERVAL_S = 6`, `eraEpochCloseCutoff`, `doAnnounceEraEnd`, `doCloseEpoch`, `st_eraEndBoundariesSuiteClassificationAndSpecGapsTest`, mutant `settlement_m_bat010_epoch_past_era_cutoff` | `CONSTANT-CHECKED` | Closes open epochs at or before `end_eff - (176 + 60 + 6)` (`end_eff - 242 s`) when `end_eff` is announced so roots land strictly before `end_eff`. |
| `BAT-011` | `inv_BAT_011`, `INCLUSION_TIME_S = 60`, `doStartEraRun`, `doRefuseLateEraRun`, `st_eraEndBoundariesSuiteClassificationAndSpecGapsTest`, mutant `settlement_m_bat011_start_run_at_era_end` | `CONSTANT-CHECKED` | Refuses starting any run where `start + prove + inclusion >= end_eff` (including exact equality at `end_eff`). |
| `BAT-012` | `inv_BAT_012`, `doWriteT15FreezeRecord`, `st_epochBatchCollectionAndConsumerSingleUseAcrossSourcesTest`, mutant `settlement_m_bat012_omit_open_t15` | `MODELED` | Whenever the T15 freeze record is written with zero `acceptCollection` receipts, `t15ListsCollectionOpen == true`. |
| `STL-001` | `inv_SLC_006`, `inv_SLC_009`, `doBuildAcceptArgs`, `st_argumentBuilderEffectDeltaWitnessSaltAndLaneCheckTest` | `MODELED` | Derives `(c_p, input_pub, outcome_pub, claim_pub)` for `acceptRun` from the Mycroft statement and cross-checks wrap public inputs. |
| `STL-002` | `SPEC-CHECKED candidate` (cross-referenced by `inv_SLC_012`, `inv_STL_003`) | `SPEC-CHECKED candidate` | Multi-stage proving orchestration rule (`app -> leaf -> N1..N3 -> L -> L_Fr -> k=21 wrap` + muninn check) owned across Proving, Wrap, and Placement; Settlement guards statement binding (`SLC-012`) and rekeyed key readiness (`STL-003`). |
| `STL-003` | `inv_STL_003`, `ANCHOR_SLOT = 15`, `doPassRekeyAndT12`, `doDeployProgramLedger`, `doRefuseInstalledVkReadbackMismatch`, `st_deployEntriesSlot15AndT25ReadbackTest`, mutant `settlement_m_stl003_mismatched_readback_vk` | `CONSTANT-CHECKED` | After rekey + T12 pass, deploys `SherlockLedger` on Preview with the new wrap VK and slot `15` anchor, stopping if readback VK differs. |
| `STL-004` | `inv_STL_004`, `doAcceptRun`, `st_acceptRunGuardsAndSingleSettleReplayRefusalTest`, mutant `settlement_m_ldg002_replay_settled_claim` | `MODELED` | Settling a call via `acceptRun` sets `settledRuns[claim_pub].outcomePub == outcome_pub` once (`acceptCount == 1`). |
| `STL-005` | `inv_STL_005`, `doClassifySuiteCall`, `st_eraEndBoundariesSuiteClassificationAndSpecGapsTest`, mutant `settlement_m_stl005_hide_mismatch` | `MODELED` | Records settled calls differing from T18(a) native transcript as `ClassMismatch` and natively refused calls as `ClassRefusedAsNative`; salted `outcome_pub` vs plain transcript gap noted in `Q-settlement-2`. |
| `STL-006` | `consistency_bat004_t23_counts` | `SPEC-CHECKED candidate` | Part A native Compact rerun of 33 programs on Preview with hard-fork spec-version split belongs to Network Operations / Conformance (`NativeCallReference`) and mutates no `SherlockLedger` state. |

### 1.2 Adopted IDs (`sherlock-modular-leaf-20261111` & `sherlock-processor-final-20261111`)

| Adopted ID | Disposition in `e2e-settlement` | Artifact / Reason |
|---|---|---|
| `LDG-001` | Modeled (per-claim prohibition in `inv_LDG_003`; single-vs-per-program ledger contradiction reported in `Q-settlement-1`) | Enforces `perClaimLedgerWrites == 0` for collections; documents direct contradiction between `LDG-001`'s *"Two contracts with separate ledgers SHALL NOT be used"* and `SLC-005`'s *"One ledger for each program"*. |
| `LDG-002` | Modeled (`inv_LDG_002`, `inv_STL_004`, `inv_BAT_002`, `deriveClaimPub`, mutant `settlement_m_ldg002_replay_settled_claim`) | Computes `claim_pub = transientHash([C_P, input_pub, outcome_pub])`, settles each `claim_pub` at most once (`acceptCount == 1`), and restricts `acceptRun` to owner-submitted runs. |
| `LDG-003` | Modeled (`inv_LDG_003`, `doAcceptCollection`, mutant `settlement_m_ldg003_stale_reg_or_dup_root`) | Checks `reg_pub` against registry history, refuses duplicate `claims_root`, makes `0` per-claim writes, leaves `runs` unchanged, and leaks no `shape_d` / `set_root`. |
| `LDG-004` | Modeled (`inv_LDG_004`, `inv_SLC_001`, `doConsumerUseClaim`, mutant `settlement_m_ldg004_root_scoped_double_use`) | No `openClaim`/`descend`; Compact consumer verifies 20-sibling path + `rootAccepted`, writes `0` ledger entries on `SherlockLedger`, and keys nullifier by `claim_pub` alone so a claim settled in both `runs` and `claim_roots` is used at most once. |
| `LDG-005` | Modeled (`inv_LDG_004`, `inv_BAT_007`, `inv_BAT_008`, `CLAIMS_TREE_DEPTH = 20`) | Depth-20 claims tree with off-chain leaf list and 20-sibling paths; claims unusable before epoch collection acceptance or while list is withheld. |
| `ZK-006` | Modeled (`inv_SLC_008`, `RUN_SALT_WORDS = 8`, `deriveInputD`) | 8-word CSPRNG run salt inside `input_d`, never published in call arguments. |
| `ZK-011` | Modeled (`inv_SLC_006`, `deriveCP`, `deriveInputPub`, `deriveOutcomePub`, `st_argumentBuilderEffectDeltaWitnessSaltAndLaneCheckTest`) | `c_p` salted per deployment (`r_P`), `input_pub` from `input_d`, and `outcome_pub` hiding `outcome_d` under `input_d`'s run salt so two runs with identical outcomes and distinct salts give distinct `outcome_pub` values. |

---

## 2. Findings (`Q-settlement-1` .. `Q-settlement-6`)

### `Q-settlement-1`: Direct contradiction between adopted `LDG-001` (*"Two contracts with separate ledgers SHALL NOT be used"*) and `SLC-005` / `SLC-011` / `BAT-004` (*"One ledger for each program"*, *"at most 106 deploys"*), plus cross-program `acceptCollection` anchor conflict

- **IDs**: `LDG-001`, `LDG-003`, `SLC-001`, `SLC-002`, `SLC-005`, `SLC-011`, `BAT-002`, `BAT-004`
- **Quoted sentences**:
  - `LDG-001`: *"The LEDGER CONTRACT SHALL be one Compact contract with one shared state: `runs: claim_pub -> outcome_pub` for runs that their own users submit ..., `claim_roots`, one `claims_root` per accepted collection, and the registry root. ... Two contracts with separate ledgers SHALL NOT be used."*
  - `SLC-005`: *"SLC-005: When the deploy runner deploys a ledger for an R_SL program, the deploy runner SHALL set the constructor anchor to the `c_p` of that program."*
  - `SLC-011`: *"When the replay of a suite program starts on an instance whose state differs from the deploy-block state, the settle runner SHALL deploy a new instance for that program."*
  - `BAT-004` Scenario: *"GIVEN 33 programs, 106 circuits and 208 calls ... THEN the report shows 145 `acceptRun` transactions and at most 106 deploys (DERIVED: 208 - 62 - 1, and one ledger for each circuit under SLC-005)"*
- **Witness state / trace**:
  - Pure witness `gap_q_stl_1_ldg001_single_vs_slc005_multi_ledger` and trace `t_0.itf.json` (steps 3–4, deploying separate `SherlockLedger` instances for `progId = 1` with `anchorCP = 1007` and `progId = 2` with `anchorCP = 2007`):
  - (1) `LDG-001` forbids deploying two `SherlockLedger` contracts with separate ledgers (`maxContracts == 1`), whereas `SLC-005` deploys one contract per program (`33`) or per circuit (`106` in `BAT-004` Scenario — itself an internal terminology mismatch between "program" in `SLC-005` and "circuit" in `BAT-004`, cf. `DDD.md:23, 329`).
  - (2) Because each per-program `SherlockLedger` seals a single program's `c_p` in slot `15` (`SLC-002`, `SLC-005`), an operator epoch batching 3 jobs from 3 distinct programs `A, B, C` (`BAT-002` Scenario: *"the ledger sees 1 transaction, not 3"*) has no single shared `claim_roots` ledger on which 1 `acceptCollection` transaction can settle all 3 programs without conflicting with per-program slot-15 anchors.
- **What would close it**:
  - Explicitly supersede `LDG-001`'s single-contract clause for per-program `acceptRun` ledgers (and standardize `SLC-005` on "compiled circuit" vs "program"), while specifying which contract holds shared cross-program `claim_roots` (or how `acceptCollection`'s wrap prologue handles slot `15` when batching claims across multiple programs).

### `Q-settlement-2`: `SLC-009` cross-checks lanes `10, 20, 34, 39` without mapping words to lanes — omitting `input_pub`, failing on chunk 2 where `lane 10` holds lock `A_1 != c_p`, and leaving `STL-005`'s on-chain `outcome_pub` comparison underspecified under `ZK-011`

- **IDs**: `SLC-006`, `SLC-009`, `STL-004`, `STL-005`, `BAT-005`, `ZK-011`
- **Quoted sentences**:
  - `SLC-006`: *"When the argument builder receives a `sherlock-accept-bundle/1` on the proving box, the argument builder SHALL return the 4 public words `c_p`, `input_pub`, `outcome_pub` and `claim_pub` as one `StatementPublics` value."*
  - `SLC-009`: *"If a recomputed public word differs from lane 10, 20, 34 or 39 of the reported call vector, then the argument builder SHALL raise `Mismatch` and write no argument file."*
  - `STL-005`: *"If a settled outcome differs from the native Preview transcript of T18 (a), then the suite report SHALL record the call as a mismatch."*
  - `ZK-011`: *"`outcome_pub = PoseidonFr_4(SHK_OUTCOME, pack(P2(DOM_OUTHIDE, input_d, outcome_d)))`, so the outcome is hidden under the salt inside `input_d`"*
- **Witness state**:
  - (a) Pure witness `gap_q_stl_2_chunk2_lane10_lock_vs_cp`: By `DDD.md:98`, lane `10` is `anchor`, lanes `20` and `34` are `claim_pub`, and lane `39` is `outcome_pub`. First, `input_pub` does not appear in any of `{10, 20, 34, 39}`. Second, on chunk `j = 2` of an admitted `2`-chunk call (`BAT-005`: `K_MAX = 2`), lane `10` holds the lock value `A_1` (`SHK_LOCK`), NOT `c_p` (`cP = 1007 != lockA1 = 555555`), so comparing `c_p` against lane `10` raises a false `Mismatch` on chunk 2 (notice `SLC-006` Scenario `spec.md:81` quietly writes a 4th field `anchor` alongside `c_p` in `ledger-args.json`!).
  - (b) In `st_argumentBuilderEffectDeltaWitnessSaltAndLaneCheckTest` (`argId = 1` vs `argId = 3`), two runs with identical plain effect words (`outcomeD = 7100`) and different 8-word run salts produce distinct `outcomePub` values (`ZK-011`). Because `SherlockLedger` stores only salted `runs[claim_pub] = outcome_pub` on chain (`STL-004`), `STL-005` cannot compare "a settled outcome" against the plain native T18(a) transcript from ledger state alone without opening the private off-chain `sherlock-accept-bundle/1` preimage (`input_d`, `comm`, `W`).
- **What would close it**:
  - Specify the exact lane-to-word map in `SLC-009` (`lane 10 == a_in` where `a_in == c_p` on `ORD` / `j == 1` and `a_in == A_(j-1)` on `j == 2`, `lane 20 == claim_pub`, `lane 34 == claim_pub`, `lane 39 == outcome_pub`, with `input_pub` verified via `claim_pub == transientHash([c_p, input_pub, outcome_pub])`), and clarify in `STL-005` that conformance verifies both on-chain `runs[claim_pub] == outcome_pub` AND off-chain recomputation of `outcome_pub` from `input_d` and the native transcript's `[comm] ++ W`.

### `Q-settlement-3`: Operator jobs routed under `BAT-001` & `BAT-002` cannot settle while `BAT-009` blocks `acceptCollection`, and `BAT-001` omits `BAT-010`'s era-end cutoff

- **IDs**: `BAT-001`, `BAT-002`, `BAT-009`, `BAT-010`, `BAT-011`, `LDG-005`
- **Quoted sentences**:
  - `BAT-001`: *"When the first job of an epoch arrives, the epoch operator SHALL close the epoch at the earlier of 3,600 s after that job and 1,048,576 claims."*
  - `BAT-002`: *"If the submitter of a proved job is not the user who owns the run, then the settle runner SHALL route the job to the epoch collection and send no `acceptRun` for the job."*
  - `BAT-009`: *"While the universal key set holds no key for the `L` collection form, the settle runner SHALL refuse to submit an `acceptCollection` call."*
  - `BAT-010`: *"While an era end `end_eff` is announced, when the time reaches `end_eff` minus the collection proof time, the inclusion time and one block interval, the epoch operator SHALL close the open epoch."*
  - `LDG-005` Scenario: *"A claim SHALL be usable only after its epoch's collection is accepted."*
- **Witness trace**:
  - (a) In `st_epochBatchCollectionAndConsumerSingleUseAcrossSourcesTest` (before `doEnableLCollectionKey`): an operator job arrives (`doArriveOperatorJob`), cannot settle via `acceptRun` (`BAT-002`), closes its epoch at `t + 3,600 s` (`BAT-001`), and is then refused at `acceptCollection` (`BAT-009` because `hasLCollectionKey == false` on the T25 key set), stranding the job with no settlement path and making it permanently unusable (`LDG-005`).
  - (b) In `st_eraEndBoundariesSuiteClassificationAndSpecGapsTest`, when `end_eff = 942` is announced on an epoch opened at `t0 = 100`, `BAT-010` forces early close at `942 - 242 = 700 s`, whereas `BAT-001` literally specifies closing *at* `min(t0 + 3,600 = 3,700 s, 1,048,576 claims)` without incorporating `BAT-010`'s cutoff; moreover, `BAT-011` checks only `prove_time + incl_time < end_eff - now` (`60 s` margin), not `collection_proof_time + incl_time + block_interval` (`242 s`), allowing an operator job to be admitted in `(end_eff - 242, end_eff - 60)` after the `BAT-010` epoch-close deadline has already passed.
- **What would close it**:
  - Refuse operator-submitted batch jobs at admission while `not(hasLCollectionKey)` (`BAT-009`), unify `BAT-001` + `BAT-010` as `min(t0 + 3,600 s, end_eff - t_coll - t_incl - t_block, 1,048,576 claims)`, and require `now + prove_time + t_coll + t_incl + t_block < end_eff` in `BAT-011` for operator-batched runs.

### `Q-settlement-4`: `LDG-002`'s prohibition on operator `acceptRun` calls is unenforceable on chain by `SherlockLedger` and relies solely on off-chain runner cooperation (`BAT-002`)

- **IDs**: `BAT-002`, `LDG-002`
- **Quoted sentences**:
  - `LDG-002` Scenario: *"A job that an operator proves and settles SHALL NOT be settled by `acceptRun`; it SHALL settle through the epoch collection of LDG-006."*
  - `BAT-002`: *"If the submitter of a proved job is not the user who owns the run, then the settle runner SHALL route the job to the epoch collection and send no `acceptRun` for the job."*
- **Witness state**:
  - `acceptRun(C_P, input_pub, outcome_pub)` (`LDG-002`) takes only the 3 field elements `(C_P, input_pub, outcome_pub)` and checks no user/owner key or authorization witness (`spec.md:20`: *"the contract cannot tell an operator from a user"*). Any caller holding a valid wrap proof and `(C_P, input_pub, outcome_pub)` can invoke `acceptRun` directly on chain, bypassing `BAT-002`.
- **What would close it**:
  - Document explicitly in `LDG-002` and `BAT-002` that owner-vs-operator routing is an off-chain runner/economic policy rather than an on-chain contract invariant (or bind a sender/owner authorization tag if on-chain enforcement is intended).

### `Q-settlement-5`: `SLC-001` and `SLC-004` export `rootAccepted` and `runAccepted` without specifying their parameter/return semantics, and `LDG-002`/`LDG-003` allow duplicate membership across `runs` and multiple `claim_roots`

- **IDs**: `SLC-001`, `SLC-004`, `LDG-001`, `LDG-002`, `LDG-003`, `LDG-004`
- **Quoted sentences**:
  - `SLC-001`: *"The SherlockLedger contract SHALL export exactly 4 entries: `acceptRun`, `acceptCollection`, `rootAccepted` and `runAccepted`."*
  - `LDG-001`: *"`runs: claim_pub -> outcome_pub` ... `claim_roots`, one `claims_root` per accepted collection"*
  - `LDG-003`: *"`acceptCollection(reg_pub, claims_root)` ... SHALL refuse a `claims_root` already in `claim_roots`, and SHALL insert it."*
- **Witness trace**:
  - In `st_epochBatchCollectionAndConsumerSingleUseAcrossSourcesTest`, `claimPub = stl.builtArgs.get(2).claimPub` is settled in `runs` via `doAcceptRun` AND simultaneously included in `claimsRoot = 10002` via `doAcceptCollection` (and can also be included in a second distinct epoch `claimsRoot' != 10002` without triggering `LDG-003`'s `claims_root` uniqueness check). Only a consumer that properly implements `LDG-004` (nullifier keyed by `claim_pub` alone, tested via mutant `settlement_m_ldg004_root_scoped_double_use`) prevents double-spending across `runAccepted` and `rootAccepted`, yet `SLC-001..012` never specify the exact input/return signature of `runAccepted` (whether it checks `(claim_pub, outcome_pub)` or `claim_pub` alone) or `rootAccepted(claims_root) -> Bool`.
- **What would close it**:
  - Add EARS requirements in `sherlock-e2e-ledger-contract` defining the exact call signatures and return semantics of `rootAccepted(claims_root) -> Bool` and `runAccepted(claim_pub, outcome_pub) -> Bool` (or `runAccepted(claim_pub) -> Option<outcome_pub>`).

### `Q-settlement-6`: `BAT-004` Scenario's `<= 106` deploy ceiling conflicts with `SLC-011` dirty-state re-deployments

- **IDs**: `BAT-004`, `SLC-011`, `SLC-012`
- **Quoted sentences**:
  - `BAT-004` Scenario: *"GIVEN 33 programs, 106 circuits and 208 calls, of which 62 are refused as native and 1 is pure WHEN the suite settles each remaining call by its own user's `acceptRun` THEN the report shows 145 `acceptRun` transactions and at most 106 deploys"*
  - `SLC-011`: *"When the replay of a suite program starts on an instance whose state differs from the deploy-block state, the settle runner SHALL deploy a new instance for that program."*
- **Witness state**:
  - If all 106 circuits are deployed once (`106` deploys) and even one circuit starts a second independent replay sequence after its first sequence mutated contract state (`stateEqualsDeployBlockAtStart = false`), `SLC-011` requires deploying a `107`th instance, exceeding `BAT-004` Scenario's *"at most 106 deploys"*.
- **What would close it**:
  - State the deploy upper bound in `BAT-004` Scenario as `106 + dirty_replay_redeploys` (or one deploy per independent state-mutating replay sequence under `SLC-011`).

---

## 3. Mutant Table

Every mutant (`MUTANT_ID = 1..25`) weakens one refusal or binding guard while keeping honest paths reachable (verified non-vacuous: `reach_accept_run_and_consumer_use` exits `1` on all 25 mutants).

| Mutant module | Target invariant | Exit code (`quint run`) | Non-vacuity (`reach_accept_run_and_consumer_use`) | Guard weakened |
|---|---|---|---|---|
| `settlement_m_slc001_open_claim_entry` | `inv_SLC_001` | `1` (`[violation]`) | `1` (`[violation]`) | Exports legacy `"openClaim"` (`5` entries instead of `4`) |
| `settlement_m_slc002_unsealed_anchor` | `inv_SLC_002` | `1` (`[violation]`) | `1` (`[violation]`) | Places anchor in slot `0` unsealed instead of sealed slot `15` |
| `settlement_m_slc003_wrong_write_vk` | `inv_SLC_003` | `1` (`[violation]`) | `1` (`[violation]`) | Installs non-universal `writeVkId = 99` on write entries |
| `settlement_m_slc004_wrap_key_on_read_entry` | `inv_SLC_004` | `1` (`[violation]`) | `1` (`[violation]`) | Installs wrap VK instead of native keys on read entries |
| `settlement_m_slc005_cross_program_settle` | `inv_SLC_005` | `1` (`[violation]`) | `1` (`[violation]`) | Allows program 2's call to settle on program 1's ledger |
| `settlement_m_slc008_unsalted_or_nonzero_fuel` | `inv_SLC_008` | `1` (`[violation]`) | `1` (`[violation]`) | Builds Mycroft input with `0` salt words, `fuel = 10`, and leaked salt |
| `settlement_m_slc009_write_args_on_lane_mismatch` | `inv_SLC_009` | `1` (`[violation]`) | `1` (`[violation]`) | Writes `ledger-args` despite lane `10/20/34/39` mismatch |
| `settlement_m_slc010_settle_natively_refused` | `inv_SLC_010` | `1` (`[violation]`) | `1` (`[violation]`) | Sends `acceptRun` for a natively refused suite call |
| `settlement_m_slc011_reuse_dirty_instance` | `inv_SLC_011` | `1` (`[violation]`) | `1` (`[violation]`) | Reuses an instance whose state differs from deploy-block state |
| `settlement_m_slc012_settle_unbound_stmt` | `inv_SLC_012` | `1` (`[violation]`) | `1` (`[violation]`) | Settles a call whose leaf statement carries unbound zeros |
| `settlement_m_bat001_late_epoch_close` | `inv_BAT_001` | `1` (`[violation]`) | `1` (`[violation]`) | Closes epoch `100 s` past `firstJobTimeS + 3,600 s` |
| `settlement_m_bat002_operator_accept_run` | `inv_BAT_002` | `1` (`[violation]`) | `1` (`[violation]`) | Settles an operator-submitted job via per-job `acceptRun` |
| `settlement_m_bat005_allow_3_chunks` | `inv_BAT_005` | `1` (`[violation]`) | `1` (`[violation]`) | Admits `3` chunks at `TTL = 14,400 s` (`floor(14400 / 1755)`) |
| `settlement_m_bat006_default_ttl_14400` | `inv_BAT_006` | `1` (`[violation]`) | `1` (`[violation]`) | Uses legacy `14,400 s` default Preview intent TTL |
| `settlement_m_bat007_missing_paths` | `inv_BAT_007` | `1` (`[violation]`) | `1` (`[violation]`) | Closes epoch without delivering `20`-sibling Merkle paths |
| `settlement_m_bat008_unpublish_leaf_list` | `inv_BAT_008` | `1` (`[violation]`) | `1` (`[violation]`) | Closes epoch without publishing off-chain `32`-byte leaf list |
| `settlement_m_bat009_collection_without_l_key` | `inv_BAT_009` | `1` (`[violation]`) | `1` (`[violation]`) | Submits `acceptCollection` without the `L` collection key |
| `settlement_m_bat010_epoch_past_era_cutoff` | `inv_BAT_010` | `1` (`[violation]`) | `1` (`[violation]`) | Closes epoch at `end_eff` instead of `<= end_eff - 242 s` |
| `settlement_m_bat011_start_run_at_era_end` | `inv_BAT_011` | `1` (`[violation]`) | `1` (`[violation]`) | Starts era run whose `start + prove + incl >= end_eff` |
| `settlement_m_bat012_omit_open_t15` | `inv_BAT_012` | `1` (`[violation]`) | `1` (`[violation]`) | Writes T15 freeze record without listing collection open |
| `settlement_m_stl003_mismatched_readback_vk` | `inv_STL_003` | `1` (`[violation]`) | `1` (`[violation]`) | Completes T25 Preview deploy despite readback VK mismatch |
| `settlement_m_ldg002_replay_settled_claim` | `inv_LDG_002` | `1` (`[violation]`) | `1` (`[violation]`) | Permits a second `acceptRun` on an already settled `claim_pub` |
| `settlement_m_ldg003_stale_reg_or_dup_root` | `inv_LDG_003` | `1` (`[violation]`) | `1` (`[violation]`) | Accepts `acceptCollection` with stale/foreign `reg_pub` or duplicate root |
| `settlement_m_ldg004_root_scoped_double_use` | `inv_LDG_004` | `1` (`[violation]`) | `1` (`[violation]`) | Allows consumer to discharge the same `claim_pub` twice |
| `settlement_m_stl005_hide_mismatch` | `inv_STL_005` | `1` (`[violation]`) | `1` (`[violation]`) | Records a settled call differing from T18(a) as `ClassPass` |

---

## 4. State Vocabulary for Trace Replay

### 4.1 State Variables (`vars` in ITF JSON)

In `../traces/settlement_main/t_{0..19}.itf.json`, Quint qualifies the module variables under `settlement_main::settlement::`:

| Quint `var` | ITF JSON state key | Type | Meaning |
|---|---|---|---|
| `stl` | `settlement_main::settlement::stl` | `SettleState` | Complete observable state of **Settlement** (`SherlockLedger` deployments, `StatementPublics` argument records, `settledRuns`, epoch operator batches, `acceptedCollections`, consumer claim uses, chunk plans, Preview intents, suite replays, era-end run gates, T15 freeze status, suite classifications). |
| `lastOp` | `settlement_main::settlement::lastOp` | `str` | Name of the action that produced the current state (`"init"` at step `0`). |

#### `SettleState` fields (`settlement_main::settlement::stl`)

| Field | Type | Meaning |
|---|---|---|
| `nowS` | `int` | Monotonic logical clock in seconds (starts at `100`). |
| `universalWrapVkId` | `int` | Universal wrap verifying key ID (`1`). |
| `rekeyAndT12Passed` | `bool` | Whether the T25 rekey and T12 security gate have passed (`STL-003`). |
| `hasLCollectionKey` | `bool` | Whether the universal key set includes the `L` collection form key (`BAT-009`). |
| `eraEndAnnounced` | `bool` | Whether an era end `end_eff` has been announced (`BAT-010`, `BAT-011`). |
| `eraEndEffS` | `int` | Announced era end timestamp in seconds (`0` when unannounced). |
| `nextInstanceId` | `int` | Monotonic deployment counter for `SherlockLedger` contract instances (`SLC-005`, `SLC-011`). |
| `ledgers` | `int -> ProgramLedgerRecord` | Deployed `SherlockLedger` contracts keyed by `progId` (`SLC-001..005`, `STL-003`). |
| `knownLedgerProgIds` | `Set[int]` | Active program IDs in `ledgers`. |
| `builtArgs` | `int -> BuiltArgsRecord` | `StatementPublics` / `ledger-args` built by the argument builder (`SLC-006..009`, `STL-001`). |
| `knownArgIds` | `Set[int]` | Active argument bundle IDs in `builtArgs`. |
| `settledRuns` | `int -> SettledRunRecord` | On-chain `runs: claim_pub -> outcome_pub` state and audit metadata (`STL-004`, `LDG-002`). |
| `settledClaimPubs` | `Set[int]` | Domain of `settledRuns`. |
| `epochOpen` | `bool` | Whether an operator claim epoch is currently open (`BAT-001`). |
| `openEpochId` | `int` | Monotonic epoch identifier (`1, 2, ...`). |
| `openEpochFirstJobS` | `int` | Arrival timestamp of the open epoch's first job (`BAT-001`). |
| `openEpochClaims` | `Set[int]` | `claim_pub` leaves accumulated in the open epoch (`BAT-001`, `BAT-002`). |
| `closedEpochs` | `Set[ClosedEpochRecord]` | Closed operator epochs (`claimsRoot`, `memberClaims`, `deliveredSiblingCount`, `leafBytesPerClaim`, `eraEndAnnouncedAtClose`; `BAT-001`, `BAT-007`, `BAT-008`, `BAT-010`). |
| `acceptedCollections` | `int -> AcceptedCollectionRecord` | On-chain `claim_roots` records keyed by `claimsRoot` (`BAT-009`, `LDG-003`). |
| `acceptedRoots` | `Set[int]` | Domain of `acceptedCollections`. |
| `consumerUsedClaims` | `Set[int]` | Claims discharged by the consumer contract's claim-keyed nullifier (`LDG-004`). |
| `consumerDoubleUseOccurred` | `bool` | Flag set if a consumer ever discharges the same `claim_pub` twice (`false` in honest runs; `LDG-004`). |
| `consumerLedgerWritesOnUse` | `int` | Writes made to `SherlockLedger` during consumer claim use (`0`; `LDG-004`). |
| `chunkPlans` | `Set[ChunkPlanRecord]` | Admitted `sherlock_plan` chunk records (`BAT-005`). |
| `previewIntents` | `Set[PreviewIntentRecord]` | Built Preview settlement intents with assigned TTLs (`BAT-006`). |
| `suiteReplays` | `Set[SuiteReplayRecord]` | Suite program replay records tracking fresh deploy vs reuse (`SLC-011`). |
| `eraRunAttempts` | `Set[EraRunAttemptRecord]` | Started era run records checked against `end_eff` (`BAT-011`). |
| `suiteClassifications` | `Set[SuiteClassRecord]` | Conformance classifications (`ClassPass`, `ClassRefusedAsNative`, `ClassMismatch`, `ClassBlocked`; `SLC-010`, `STL-005`). |
| `t15FreezeWritten` | `bool` | Whether the T15 freeze record has been written (`BAT-012`). |
| `t15ListsCollectionOpen` | `bool` | Whether T15 lists batched collection settlement as open (`BAT-012`). |
| `lastRefusal` | `str` | Most recent refusal tag (`"none"`, `"installed_vk_readback_mismatch"`, `"Mismatch"`, `"native_refused"`, `"unbound_leaf_statement"`, `"operator_must_batch"`, `"missing_l_collection_form_key"`, `"chunk_cap_K_MAX_2_exceeded"`, `"run_outlives_era_end_eff"`). |

### 4.2 `lastOp` Values and Implemented EARS IDs

| `lastOp` value | Action in `settlement.qnt` | Implemented EARS IDs |
|---|---|---|
| `"init"` | `init` | Initial state |
| `"passRekeyAndT12"` | `doPassRekeyAndT12` | `STL-003` |
| `"enableLCollectionKey"` | `doEnableLCollectionKey` | `BAT-009` |
| `"deployProgramLedger"` | `doDeployProgramLedger` | `SLC-001`, `SLC-002`, `SLC-003`, `SLC-004`, `SLC-005`, `STL-003` |
| `"refuseInstalledVkReadbackMismatch"` | `doRefuseInstalledVkReadbackMismatch` | `STL-003` |
| `"buildAcceptArgs"` | `doBuildAcceptArgs` | `SLC-006`, `SLC-007`, `SLC-008`, `SLC-009`, `STL-001`, `ZK-006`, `ZK-011` |
| `"refuseLaneMismatch"` | `doRefuseLaneMismatch` | `SLC-009` |
| `"acceptRun"` | `doAcceptRun` | `SLC-005`, `SLC-010`, `SLC-012`, `BAT-002`, `STL-004`, `LDG-001`, `LDG-002` |
| `"refuseAcceptRun"` | `doRefuseAcceptRun` | `SLC-005`, `SLC-010`, `SLC-012`, `BAT-002`, `LDG-002` |
| `"arriveOperatorJob"` | `doArriveOperatorJob` | `BAT-001`, `BAT-002` |
| `"closeEpoch"` | `doCloseEpoch` | `BAT-001`, `BAT-007`, `BAT-008`, `BAT-010`, `LDG-005` |
| `"acceptCollection"` | `doAcceptCollection` | `BAT-009`, `LDG-001`, `LDG-003` |
| `"refuseCollectionWithoutLKey"` | `doRefuseCollectionWithoutLKey` | `BAT-009` |
| `"consumerUseClaim"` | `doConsumerUseClaim` | `BAT-007`, `BAT-008`, `LDG-001`, `LDG-004`, `LDG-005` |
| `"planEntryChunks"` | `doPlanEntryChunks` | `BAT-005` |
| `"refuseOverChunkedEntry"` | `doRefuseOverChunkedEntry` | `BAT-005` |
| `"buildPreviewIntent"` | `doBuildPreviewIntent` | `BAT-006` |
| `"startSuiteReplay"` | `doStartSuiteReplay` | `SLC-011` |
| `"announceEraEnd"` | `doAnnounceEraEnd` | `BAT-010`, `BAT-011` |
| `"startEraRun"` | `doStartEraRun` | `BAT-011` |
| `"refuseLateEraRun"` | `doRefuseLateEraRun` | `BAT-011` |
| `"writeT15FreezeRecord"` | `doWriteT15FreezeRecord` | `BAT-012` |
| `"classifySuiteCall"` | `doClassifySuiteCall` | `SLC-010`, `STL-005` |

### 4.3 Concrete Instance Constants

| Constant | `settlement_main` | `settlement_edge_min` | `settlement_edge_at_limit` | `settlement_m_*` (`25` mutants) |
|---|---|---|---|---|
| `CONTRACT_ENTRY_COUNT` | `4` | `4` | `4` | `4` |
| `ANCHOR_SLOT` | `15` | `15` | `15` | `15` |
| `STATEMENT_PUBLIC_WORDS` | `4` | `4` | `4` | `4` |
| `RUN_SALT_WORDS` | `8` | `8` | `8` | `8` |
| `EPOCH_WINDOW_S` | `3600` | `242` | `3600` | `3600` |
| `MAX_CLAIMS_PER_EPOCH` | `1048576` | `1` | `2` | `1048576` |
| `K_MAX_CHUNKS` | `2` | `1` | `2` | `2` |
| `DEFAULT_INTENT_TTL_S` | `3600` | `3600` | `3600` | `3600` |
| `CLAIMS_TREE_DEPTH` | `20` | `20` | `20` | `20` |
| `CLAIM_LEAF_BYTES` | `32` | `32` | `32` | `32` |
| `COLLECTION_PROOF_TIME_S` | `176` | `176` | `176` | `176` |
| `INCLUSION_TIME_S` | `60` | `60` | `60` | `60` |
| `BLOCK_INTERVAL_S` | `6` | `6` | `6` | `6` |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 25` |

---

## 5. Model Abstractions

1. **PoseidonFr, BabyBear packing, and `transientHash`**: `C_P`, `input_d`, `input_pub`, `outcome_d`, `outcome_pub`, and `claim_pub` (`SLC-006..008`, `LDG-002`, `ZK-006`, `ZK-011`) are computed by injective integer combinations (`deriveCP`, `deriveInputD`, `deriveInputPub`, `deriveOutcomeD`, `deriveOutcomePub`, `deriveClaimPub`) preserving exact functional dependencies (in particular, `outcome_pub` depends on both `input_d` and `outcome_d`, while `input_pub` is independent of `outcome_d`).
2. **Depth-20 Merkle claims tree (`LDG-005`, `BAT-007`, `BAT-008`)**: Rather than materializing `2^20` leaves, an epoch stores its finite `memberClaims: Set[int]`, injective `claimsRoot = epochId * 10000 + claimCount`, `deliveredSiblingCount` (`20`), and `leafBytesPerClaim` (`32`).
3. **Time budgets (`BAT-001`, `BAT-006`, `BAT-010`, `BAT-011`)**: Time is modelled in integer seconds (`nowS`), with `175.5 s` warm collection proving rounded conservatively to `COLLECTION_PROOF_TIME_S = 176` s.
4. **Wrap proof verification**: `acceptRun` and `acceptCollection` assume wrap proof validity was checked by the ledger/ACL (`e2e-ledger-verification`), focusing Settlement on statement binding, anchor `c_p` matching, single-settle maps, registry history validity, and batch/era rules.

---

## 6. Commands

Run from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/settlement/quint`:

```bash
# 1. Typecheck specification and test suite
quint typecheck settlement.qnt
# Expected exit code: 0 (no output on success)

quint typecheck settlement_test.qnt
# Expected exit code: 0 (no output on success)

# 2. Deterministic scenario tests
quint test --main=settlement_test --match='st_.*Test' --seed=0x20261115 settlement_test.qnt
# Expected exit code: 0
# Printed: 6 passing (2000ms)

# 3. Honest instances (main and edge bounds)
quint run settlement.qnt --main settlement_main --invariant inv --max-samples 2000 --max-steps 15 --seed 0x20261115
# Expected exit code: 0
# Printed: [ok] No violation found (3102ms at 645 traces/second).

quint run settlement.qnt --main settlement_edge_min --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 0
# Printed: [ok] No violation found (1495ms at 669 traces/second).

quint run settlement.qnt --main settlement_edge_at_limit --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 0
# Printed: [ok] No violation found (1495ms at 669 traces/second).

# 4. Non-vacuity reachability witnesses (all 6 MUST violate, exit code 1)
quint run settlement.qnt --main settlement_main --invariant reach_accept_run_and_consumer_use --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (391ms at 312 traces/second).

quint run settlement.qnt --main settlement_main --invariant reach_epoch_close_and_accept_collection --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (1087ms at 94 traces/second).

quint run settlement.qnt --main settlement_main --invariant reach_era_end_cutoff_and_late_run_refusal --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (435ms at 2 traces/second).

quint run settlement.qnt --main settlement_main --invariant reach_chunk_cap_default_ttl_and_dirty_redeploy --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (475ms at 63 traces/second).

quint run settlement.qnt --main settlement_main --invariant reach_t25_preview_deploy_and_t15_open_record --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (459ms at 9 traces/second).

quint run settlement.qnt --main settlement_main --invariant reach_suite_mismatch_and_refused_classes --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (1321ms at 3 traces/second).

# 5. Guard mutants (all 25 MUST violate their named invariant, exit code 1)
quint run settlement.qnt --main settlement_m_slc001_open_claim_entry --invariant inv_SLC_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (668ms at 3 traces/second).

quint run settlement.qnt --main settlement_m_slc002_unsealed_anchor --invariant inv_SLC_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (387ms at 5 traces/second).

quint run settlement.qnt --main settlement_m_slc003_wrong_write_vk --invariant inv_SLC_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (618ms at 3 traces/second).

quint run settlement.qnt --main settlement_m_slc004_wrap_key_on_read_entry --invariant inv_SLC_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (555ms at 4 traces/second).

quint run settlement.qnt --main settlement_m_slc005_cross_program_settle --invariant inv_SLC_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (295ms at 34 traces/second).

quint run settlement.qnt --main settlement_m_slc008_unsalted_or_nonzero_fuel --invariant inv_SLC_008 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (858ms at 2 traces/second).

quint run settlement.qnt --main settlement_m_slc009_write_args_on_lane_mismatch --invariant inv_SLC_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (697ms at 13 traces/second).

quint run settlement.qnt --main settlement_m_slc010_settle_natively_refused --invariant inv_SLC_010 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (778ms at 24 traces/second).

quint run settlement.qnt --main settlement_m_slc011_reuse_dirty_instance --invariant inv_SLC_011 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (357ms at 22 traces/second).

quint run settlement.qnt --main settlement_m_slc012_settle_unbound_stmt --invariant inv_SLC_012 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (379ms at 58 traces/second).

quint run settlement.qnt --main settlement_m_bat001_late_epoch_close --invariant inv_BAT_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (387ms at 18 traces/second).

quint run settlement.qnt --main settlement_m_bat002_operator_accept_run --invariant inv_BAT_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (729ms at 44 traces/second).

quint run settlement.qnt --main settlement_m_bat005_allow_3_chunks --invariant inv_BAT_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (2418ms at 1 traces/second).

quint run settlement.qnt --main settlement_m_bat006_default_ttl_14400 --invariant inv_BAT_006 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (672ms at 4 traces/second).

quint run settlement.qnt --main settlement_m_bat007_missing_paths --invariant inv_BAT_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (876ms at 8 traces/second).

quint run settlement.qnt --main settlement_m_bat008_unpublish_leaf_list --invariant inv_BAT_008 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (294ms at 24 traces/second).

quint run settlement.qnt --main settlement_m_bat009_collection_without_l_key --invariant inv_BAT_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (690ms at 28 traces/second).

quint run settlement.qnt --main settlement_m_bat010_epoch_past_era_cutoff --invariant inv_BAT_010 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (776ms at 23 traces/second).

quint run settlement.qnt --main settlement_m_bat011_start_run_at_era_end --invariant inv_BAT_011 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (1405ms at 9 traces/second).

quint run settlement.qnt --main settlement_m_bat012_omit_open_t15 --invariant inv_BAT_012 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (576ms at 2 traces/second).

quint run settlement.qnt --main settlement_m_stl003_mismatched_readback_vk --invariant inv_STL_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (333ms at 21 traces/second).

quint run settlement.qnt --main settlement_m_ldg002_replay_settled_claim --invariant inv_LDG_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (2693ms at 76 traces/second).

quint run settlement.qnt --main settlement_m_ldg003_stale_reg_or_dup_root --invariant inv_LDG_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (780ms at 168 traces/second).

quint run settlement.qnt --main settlement_m_ldg004_root_scoped_double_use --invariant inv_LDG_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (454ms at 515 traces/second).

quint run settlement.qnt --main settlement_m_stl005_hide_mismatch --invariant inv_STL_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (378ms at 37 traces/second).

# 6. Replay trace generation (20 ITF JSON traces)
mkdir -p /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/settlement/traces/settlement_main
quint run settlement.qnt --main settlement_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/settlement/traces/settlement_main/t_{seq}.itf.json'
# Expected exit code: 0
# Printed: [ok] No violation found (553ms at 36 traces/second).
```
