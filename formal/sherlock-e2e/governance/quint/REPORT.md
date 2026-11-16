# Quint Formal Model Report — Bounded Context **Governance** (`e2e-governance`)

Independent Quint 0.32.0 specification (`governance.qnt`), scenario test suite (`governance_test.qnt`), 13 guard mutants, 6 reachability witnesses, and 20 ITF replay traces (`../traces/governance_main/t_{0..19}.itf.json`) derived solely from the assigned EARS requirements and `DDD.md` Governance rows.

---

## 1. Requirement Coverage Table

### 1.1 Own (`sherlock-e2e-prototype-20261115`) and Completion (`sherlock-completion-20261115`) IDs

| ID | Artifact (`governance.qnt` / `governance_test.qnt`) | Kind | Note |
|---|---|---|---|
| `EGV-001` | `inv_EGV_001`, `SEALED_THRESHOLD = 1`, `SEALED_COUNTER = 0`, `doDeploySealedEraLedger`, `gv_sealingReadbackPathEvidenceAndNoInPlaceVkUpdateTest`, mutant `governance_m_egv001_threshold_zero` | `CONSTANT-CHECKED` | Every usable deployed Sherlock contract sets maintenance authority `{committee: [], threshold: 1, counter: 0}` (`threshold = 0` is caught as unsealed). |
| `EGV-002` | `inv_EGV_002`, `doCatchUnsealedContractAtReadback`, `canAnnounceEra`, `gv_sealingReadbackPathEvidenceAndNoInPlaceVkUpdateTest`, mutant `governance_m_egv002_keep_sdk_committee` | `CONSTANT-CHECKED` | Any contract whose readback differs from `{[], 1, 0}` or expected entry keys is marked `markedUnusable = true` and barred from era announcements (`compact-js` 1-key default and `threshold = 0` caught). |
| `EGV-003` | `inv_EGV_003`, `doCatchUnsealedContractAtReadback`, `gv_sealingReadbackPathEvidenceAndNoInPlaceVkUpdateTest`, mutant `governance_m_egv003_unsealed_soundness_label` | `MODELED` | Any unsealed `SherlockLedger` reported in prototype outputs carries `reportLabel == LabelTransactionPathOnly` (never soundness evidence); tension with `EGV-002` noted in `Q-governance-5`. |
| `EGV-004` | `inv_EGV_004`, `doAnnounceEra`, `doRefuseAnnounceEra`, `gv_eraAnnounceQuorumGridLazyTransitionAndActivateTest`, mutant `governance_m_egv004_reuse_keyset` | `MODELED` | Each `keyset_d` maps to at most one era in `SherlockEraTable`, refusing duplicate `keyset_d` announcements; per-program ledger `ERT-003` collision noted in `Q-governance-2`. |
| `EGV-005` | `inv_EGV_005`, `doAddEraRowToRekeyList`, `doStartT25Rekey`, `doRefuseT25Rekey`, `gv_rekeyGatePrimaryInclusionAndUseTimeFallbackTest`, mutant `governance_m_egv005_rekey_no_era_no_fail` | `MODELED` | T25 rekey refuses to start unless `rekeyListHasEraRow` holds OR the T22 premise test recorded a failure on `A2` or `A3`; missing `A1`-failure branch noted in `Q-governance-4`. |
| `EGV-006` | `inv_EGV_006`, `doConsumerUseClaim`, `doRefuseConsumerUseClaim`, `gv_rekeyGatePrimaryInclusionAndUseTimeFallbackTest`, mutant `governance_m_egv006_fallback_ended_era_use` | `MODELED` | Where premise `A2` or `A3` fails (`isUseTimeFallbackActive`), consumer claim use checks `isEraActiveAt(gov, eraId, nowS)` and refuses claims of inactive/ended eras; conflict with peaceful era backwards compatibility and `CRR-006` *"first use"* wording noted in `Q-governance-3`. |
| `EGV-007` | `inv_EGV_007`, `QUORUM_Q = 12`, `TIMELOCK_TL = 70`, `GRID_STEP = 10`, `doAttemptVkUpdateOnDeployedContract`, `doAnnounceEra`, `doScheduleRosterRotation`, `gv_sealingReadbackPathEvidenceAndNoInPlaceVkUpdateTest`, `gv_eraAnnounceQuorumGridLazyTransitionAndActivateTest`, mutants `governance_m_egv007_in_place_vk_update` and `governance_m_egv007_subquorum_or_no_tl` | `CONSTANT-CHECKED` | Refuses verifier-key updates on deployed contracts; new key sets enter only as Fenrir-quorum-signed (`>= Q`) new eras after `TIMELOCK_TL` on `GRID_STEP`, and roster rotations keep claim keys untouched. |
| `EGV-008` | `inv_EGV_008`, `ALL_GOV_PARAMS`, `doWriteT15GovernanceFreeze`, `gv_t15OpenGovernanceParamsForkMigrationAndSpecGapsTest`, mutant `governance_m_egv008_omit_open_params` | `MODELED` | Any unruled parameter among `{"TL", "RTL", "Q", "n", "GRID", "SKEW"}` at T15 is listed as open and not stated as a rule; undefined role of `SKEW` noted in `Q-governance-6`. |
| `EGV-009` | `inv_EGV_009`, `doRunPremiseTest`, `doIncludeLedgerCall`, `doRefuseInactiveEraInclusion`, `gv_rekeyGatePrimaryInclusionAndUseTimeFallbackTest`, mutant `governance_m_egv009_include_after_end` | `MODELED` | T22 devnet premise test records pass/fail for all `3` premises `A1, A2, A3`; in primary mode, calls included after `end_eff` are refused with no ledger write. |
| `EGV-010` | `inv_EGV_010`, `doAnnounceProofVersionFork`, `doMigrateBeforeProofVersionFork`, `gv_t15OpenGovernanceParamsForkMigrationAndSpecGapsTest`, mutant `governance_m_egv010_fork_mutate_or_no_pred` | `MODELED` | Proof-version-stopping hard fork deploys a new era table with predecessor link and new sealed ledgers before fork height, mutates no old sealed contract key, and marks the snapshot root as a trust downgrade; missing table verb noted in `Q-governance-6`. |
| `LPC-012` | `SPEC-CHECKED candidate` (this Carbon Quint governance review and verdict workflow) | `SPEC-CHECKED candidate` | Factory tested-design-loop process rule requiring a Carbon Quint lead verdict file (`qa/gemini-quint/<t22 lane>/verdict.md`) for `sherlock-governance`; owns no on-chain governance state. |

### 1.2 Adopted Governance IDs (`sherlock-governance`, branch `t22-governance`)

| Adopted ID | Disposition in `e2e-governance` | Artifact / Reason |
|---|---|---|
| `CRR-004` | Modeled (`inv_EGV_006`, `canConsumerUseClaim`, mutant `governance_m_crr004_use_soundness_ended`) | Consumer contracts refuse all claims of `EraSoundnessEnded` eras. |
| `CRR-006` | Modeled (`inv_EGV_006`, `isUseTimeFallbackActive`, mutant `governance_m_egv006_fallback_ended_era_use`) | Use-time fallback refuses claims whose era is not active at use time when `A2` or `A3` fails; see `Q-governance-3`. |
| `CRR-007` | Modeled (`inv_EGV_006`, `doConsumerUseClaim`) | Consumer claim use sets `wroteSherlockState == false` (writes only its own nullifier). |
| `ELD-001` | Modeled (`inv_EGV_001`, `mkSealedEraLedger`) | Era ledger deploys with `{[], 1, 0}` and sealed `sealedEraId`. |
| `ELD-002` | Modeled (`inv_EGV_002`, `canAnnounceEra`, mutant `governance_m_egv002_keep_sdk_committee`) | Readback gate applied to each era ledger before any announcement names it. |
| `ELD-003` | Modeled (`inv_EGV_009`, `canIncludeLedgerCall`) | Primary-mode wrap-keyed entries check `isEraActiveAt(gov, eraId, nowS)` at inclusion. |
| `ELD-004` | Modeled (`inv_EGV_009`, `doRefuseInactiveEraInclusion`, mutant `governance_m_egv009_include_after_end`) | Refuses the whole call with no write when `table.active(era_id)` fails. |
| `ELD-005` | Modeled (`isContractSealedAndValid`, `gv_rekeyGatePrimaryInclusionAndUseTimeFallbackTest`) | Read entries keep native keys, write nothing, and allow reading claims of peacefully ended eras in primary mode. |
| `ELD-006` | Modeled (`inv_EGV_009`, `L_TX_LANE_LIMIT = 1024`, `governance_edge_min` `L_TX_LANE_LIMIT = 91`) | Wrap-keyed era ledger entries (`85` and `91` lanes) fit `<= L_TX`. |
| `ELD-007` | Modeled (`isUseTimeFallbackActive`, `canIncludeLedgerCall`, `inv_EGV_006`) | Leaves T20 entries unchanged and shifts era checking to consumer use when `A2` or `A3` fails. |
| `ERT-001` | Modeled (`inv_EGV_001`, `GENESIS_TABLE_CONTRACT`) | Era table deploys sealed with `{committee: [], threshold: 1, counter: 0}`. |
| `ERT-002` | Modeled (`inv_EGV_002`, `doCatchUnsealedContractAtReadback`) | Genesis readback gate rejects `compact-js` 1-key default and `threshold == 0`. |
| `ERT-003` | Modeled (`inv_EGV_004`, `canAnnounceEra`, mutant `governance_m_egv004_reuse_keyset`) | Refuses announcements reusing an earlier era's `keyset_d` or `ledgerId`; see `Q-governance-2`. |
| `ERT-004` | Modeled (`eraEndEff`, `isEraActiveAt`, `inv_ERT_004_005_007`, `gv_eraAnnounceQuorumGridLazyTransitionAndActivateTest`) | Accepting interval `[start(e), min(end(e), start(P)))` for non-cancelled, non-soundness-ended eras, including lazy transition without `activate`. |
| `ERT-006` | Modeled (`inv_EGV_004`) | Append-only era records preserving `initialLedgerId`, `initialKeysetD`, and `initialStartS`. |
| `ERT-009` | Modeled (`inv_EGV_007`, `GRID_STEP = 10`, `canAnnounceEra`, mutant `governance_m_egv007_subquorum_or_no_tl`) | Refuses off-grid start times (`startT % GRID_STEP != 0`). |
| `GOV-001` | Modeled (`inv_EGV_007`, `QUORUM_Q = 12`, `ROSTER_SIZE_N = 16`, mutant `governance_m_egv007_subquorum_or_no_tl`) | Governance verbs require `>= Q` signatures of the active roster. |
| `GOV-002` | Modeled (`inv_EGV_007`, `doAnnounceEra`, `TIMELOCK_TL = 70`, mutant `governance_m_egv007_subquorum_or_no_tl`) | `announce` requires no pending era and `startT >= nowS + TIMELOCK_TL`; lazy-pending interaction noted in `Q-governance-1`. |
| `GOV-003` | Modeled (`doActivateEra`, `canActivatePendingEra`, `gv_eraAnnounceQuorumGridLazyTransitionAndActivateTest`) | Permissionless `activate(M)` at `nowS >= start(M)` sets `end(N) = start(M)`, `current = M`, and clears `pending`. |
| `GOV-006` | Modeled (`doSoundnessEndEra`, `canSoundnessEnd`, `gv_rosterRotationAndSoundnessEndAfterLazyPendingStartTest`, mutant `governance_m_ert007_overwrite_end`) | Applies passed pending activation first, ends accepting era at `nowS`, marks `EraSoundnessEnded`, and cancels unstarted pending era; liveness outage noted in `Q-governance-1`. |
| `GOV-009` | Modeled (`inv_EGV_001`, `inv_EGV_007`) | No `MaintenanceUpdate` and no verb that freezes, seizes, or pauses an individual claim, ledger, or consumer. |

---

## 2. Findings (`Q-governance-1` .. `Q-governance-6`)

### `Q-governance-1`: `GOV-007` (`cancel`) omits clearing `pending`, `GOV-002` (`announce`) lacks `GOV-006`'s lazy-activation step, and `GOV-006` + `GOV-002` force a full `TL` total liveness outage after `soundness_end`

- **IDs**: `ERT-004`, `ERT-007`, `GOV-002`, `GOV-003`, `GOV-004`, `GOV-006`, `GOV-007`
- **Quoted sentences**:
  - `ERT-004` Scenario: *"WHEN block time passes `start(P)` and nobody has called `activate` THEN `active(N)` fails for the old era `N` and succeeds for `P`"*
  - `GOV-002`: *"When `Q` keys sign `announce(M, ledger_addr, keyset_d, T)` with no pending era and `T >= now + TL`, the era table SHALL add era `M` as `announced` with `start = T` and set `pending = M`."*
  - `GOV-006`: *"When `Q` keys sign `soundness_end(N)` for the era `N` that `active` accepts now, the era table SHALL first apply a pending activation whose start has passed, then set `end(N)` to the current time and `status(N) = soundness_ended`, and cancel a pending era that has not started, at once and for good."*
  - `GOV-007`: *"When `Q` keys sign `cancel(M)` before `start(M)`, the era table SHALL mark `M` `cancelled` and never use the id `M` again."*
- **Witness state / trace**:
  - (a) **Missing `pending` clear in `GOV-007`**: `GOV-007` marks `M` `cancelled` without stating *"and clear `pending`"* (unlike `GOV-003`), which under `GOV-002` (*"with no pending era"*) permanently blocks every subsequent `announce` (and if `pending` is not cleared or ignored in `ERT-004`, `start(M)` would still bound `end_eff(current)`). Similarly, if `now >= start(P)` has passed without an explicit `activate(P)` (`ERT-004` Scenario), `GOV-002` refuses `announce(M')` until a separate `activate(P)` call clears `pending` (unlike `GOV-006`, which auto-applies a passed pending activation).
  - (b) **Mandatory `TL` total outage after `soundness_end`**: In `t_0.itf.json` (steps 8–9), `soundnessEndEra` on era `0` at `nowS = 0` leaves zero active eras (`activeNowSet == Set()`), and `announceEra` for replacement era `2` is forced by `GOV-002`/`GOV-004` to set `startS = 70 >= nowS + TIMELOCK_TL` (`7` days recommended). Even worse, if a healthy replacement era `1` was already pending with `start(1) = 10` (`10` seconds away), `GOV-006` *unconditionally cancels* era `1`, forcing another full `TIMELOCK_TL` (`7` days) with zero active eras during which no Sherlock proof can settle.
- **What would close it**:
  - Explicitly clear `pending` in `GOV-007`, let `GOV-002` (`announce`) auto-apply a passed pending activation just as `GOV-006` does, and clarify whether `GOV-006` should preserve (or allow an emergency recovery `RTL`/`TL_emergency` for) an uncompromised pending replacement era when zero eras remain active.

### `Q-governance-2`: `ERT-003` and `ERT-006` assume one `ledger_addr` per era, colliding with per-program ledgers (`T22 O8` / `GOV-008` / `EGV-004`)

- **IDs**: `EGV-004`, `ERT-003`, `ERT-006`, `GOV-002`, `GOV-008`, `CRR-003`
- **Quoted sentences**:
  - `ERT-003`: *"If an announced era names a universal key set digest or a ledger that an earlier era holds, then the era table SHALL refuse the announcement."*
  - `ERT-006`: *"The era table SHALL keep every era record with the `ledger_addr`, `keyset_d` and `start` it had at `announce`."*
  - `GOV-008`: *"the `ledger` word names the program's era ledger where one ledger per program is kept (owner item O8) and is zero otherwise."*
- **Witness state**:
  - Pure witness `gap_q_gov_2_zero_ledger_addr_collides_under_ert003`: Under the prototype's one-ledger-per-program design (`SLC-005`), an era deploys `33..106` program ledgers attested via `GOV-008` rather than one era ledger. If `announce(era, ledger_addr = 0, keyset_d, T)` passes `0` as the dummy `ledger_addr` in both Era `0` and Era `1`, `ERT-003` (`era0.ledgerId == era1.ledgerId == 0`) refuses Era `1`; conversely, `ERT-003` does not enforce cross-era ledger uniqueness across leaves inside `GOV-008`'s attested Merkle roots.
- **What would close it**:
  - Exempt `ledger_addr == 0` (per-program ledger mode) from `ERT-003`'s single-`ledger_addr` equality check (or store the initial attestation root in `announce`) and require `GOV-008` attested `(era_id, C_P, lineage, ledger)` leaves to bind ledgers whose sealed `era_id` (`ELD-001`) equals the leaf's `era_id`.

### `Q-governance-3`: Use-time fallback (`EGV-006` / `CRR-006`) invalidates all unconsumed claims of peacefully ended eras, and `CRR-006`'s *"first use"* distinction is unobservable under `CRR-007`

- **IDs**: `EGV-006`, `EGV-007`, `ELD-005`, `ELD-007`, `CRR-004`, `CRR-006`, `CRR-007`
- **Quoted sentences**:
  - `EGV-006`: *"Where the premise test records a failed premise A2 or A3, when a consumer contract uses a claim, the consumer contract SHALL check `active(era_id)` on the era table."*
  - `ELD-005` Scenario: *"WHEN a consumer reads a claim of an ended era THEN the read succeeds, because read entries do not check the era"*
  - `CRR-006`: *"Where premise A2 or A3 fails, the consumer contract SHALL refuse the first use of a claim whose era is not active at the time of use."*
  - `CRR-007`: *"When a consumer contract uses a claim, the consumer contract SHALL write only its own nullifier."*
- **Witness trace**:
  - In `gv_rekeyGatePrimaryInclusionAndUseTimeFallbackTest`, Era `0` transitions peacefully to Era `1` at `nowS = 70` (`eraStatus(0) == EraNormal`, `isEraActiveAt(gov, 0, 70) == false`):
    - Under primary mode (`ELD-003`, `ELD-005`), an honest claim settled in Era `0` at `t = 0` remains usable in Era `1` (`canConsumerUseClaim(gov, 0) == true`), preserving backwards compatibility across rekeys (`EGV-007`).
    - Under fallback mode (`EGV-006` / `CRR-006`), `active(0)` returns `false` at `t >= 70`, permanently invalidating **every** unconsumed Era `0` claim upon peaceful upgrade to Era `1` (even though `EGV-006` Scenario only tested `soundness_end`, which `CRR-004` already refuses without `active(era_id)`!). Moreover, because each consumer writes only its own nullifier (`CRR-007`) and writes nothing to Sherlock contracts, a second consumer `C2` has no way to tell whether a claim was already used once during Era `0` (`CRR-006` *"first use"* vs `EGV-006` *every* use).
- **What would close it**:
  - Align `EGV-006`'s Scenario and prose with `CRR-006` by explicitly documenting that the use-time fallback sacrifices cross-era claim longevity on peaceful upgrades (requiring claims to be consumed before `end_eff(era)` or re-proved in the new era), and drop the unobservable word *"first"* in `CRR-006` (since each consumer enforces single use via its own nullifier anyway).

### `Q-governance-4`: Premise `A1` failure falls into a dead zone where neither primary (`ELD-003`) nor fallback (`ELD-007` / `EGV-005` / `EGV-006` / `CRR-006`) applies

- **IDs**: `EGV-005`, `EGV-006`, `EGV-009`, `ELD-003`, `ELD-007`, `CRR-006`
- **Quoted sentences**:
  - `EGV-009`: *"When the T22 premise test runs on a ledger 9.1.0.1 devnet, the premise test SHALL record a pass or fail result for each of the 3 premises A1, A2 and A3."*
  - `ELD-003`: *"Where premises A1 to A3 hold, the era ledger SHALL make one claimed call to `table.active(era_id)` in every wrap-keyed entry..."*
  - `EGV-005`: *"If the T25 rekey list holds no row for the era-check words and the T22 premise test holds no failed result for premise A2 or A3, then the rekey runner SHALL refuse to start."*
  - `EGV-006`: *"Where the premise test records a failed premise A2 or A3, when a consumer contract uses a claim, the consumer contract SHALL check `active(era_id)` on the era table."*
  - `ELD-007`: *"Where premise A2 or A3 fails, the era ledger SHALL keep the T20 entries unchanged and leave the era check to the consumer read rule CRR-006."*
- **Witness state**:
  - Pure witness `gap_q_gov_4_premise_a1_fail_neither_branch`: When `EGV-009` records `(A1 = false, A2 = true, A3 = true)`, `primaryEld003Applies == (A1 and A2 and A3) == false` AND `fallbackEgv006Applies == (not(A2) or not(A3)) == false`. Neither the ledger-inclusion era check (`ELD-003`) nor the consumer-use fallback (`ELD-007` / `EGV-006` / `CRR-006`) is triggered, and `EGV-005` still forces an era-check rekey row despite `A1` failing.
- **What would close it**:
  - Replace *"premise A2 or A3"* in `EGV-005`, `EGV-006`, `ELD-007`, and `CRR-006` with *"premise A1, A2 or A3"* (or specify the exact distinct remediation if `A1` fails).

### `Q-governance-5`: `EGV-002` forbids calls on unsealed contracts while `EGV-003` labels receipts of unsealed deployments without scoping to pre-gate legacy deployments

- **IDs**: `EGV-001`, `EGV-002`, `EGV-003`
- **Quoted sentences**:
  - `EGV-002`: *"If the readback of a deployed contract differs from the maintenance authority `{[], 1, 0}` or from the expected key of an entry, then the deploy runner SHALL stop and mark the contract as unusable."*
  - `EGV-002` Scenario: *"no announcement, attestation or call names the contract"*
  - `EGV-003`: *"If a deployed SherlockLedger is not sealed, then the prototype report SHALL label the receipts of that ledger as transaction-path evidence."*
- **Witness state**:
  - Under `EGV-001` and `EGV-002`, any unsealed deployment (`contractId = 50`) is marked unusable at deploy readback before any call can name it, producing zero receipts; `EGV-003` applies only to historical pre-`EGV-002` Preview deployments (such as T17(a) `740653...e274`), but its unrestricted wording appears to permit settling receipts on unsealed post-`EGV-002` contracts as long as they are labelled `"transaction-path evidence"`.
- **What would close it**:
  - Scope `EGV-003` explicitly to historical pre-`EGV-002` Preview rehearsal deployments (`T17(a)`), reaffirming that post-`EGV-002` runners never submit calls to an unsealed contract.

### `Q-governance-6`: `EGV-008` names an undefined governance parameter `SKEW`, and `EGV-010`'s predecessor link and trust-downgraded snapshot root have no entry in `ERT` or `GOV`

- **IDs**: `EGV-008`, `EGV-010`, `ERT-001..009`, `GOV-001..011`
- **Quoted sentences**:
  - `EGV-008`: *"If a governance parameter among `TL`, `RTL`, `Q`, `n`, `GRID` and `SKEW` has no owner ruling at T15, then the T15 freeze record SHALL list the parameter as open with its recommended value."*
  - `EGV-010`: *"If a Midnight hard fork is announced that stops the verification of the proof version of the deployed contracts, then the governance operators SHALL deploy a new era table and new era ledgers with a predecessor link before the fork height."*
  - `EGV-010` Scenario: *"one quorum-signed snapshot root of the old accepted roots is marked as a trust downgrade"*
- **Witness state**:
  - `TL`, `RTL`, `Q`, `n`, and `GRID` appear in `GOV-001..008` and `ERT-009`, whereas **`SKEW` appears nowhere in `ERT`, `ELD`, `GOV`, or `CRR`** — no normative EARS sentence defines what `SKEW` bounds (clock skew vs inclusion grace). Likewise, `ERT-001..009` and `GOV-001..011` define no constructor field for a `predecessor` era table and no quorum verb to import or flag a `trust_downgrade` snapshot root during an `EGV-010` proof-version migration.
- **What would close it**:
  - Add an EARS requirement defining `SKEW`'s role in timestamp/grid checks (or remove `SKEW` from `EGV-008` if superseded by `BAT-010`'s margin), and specify the constructor `predecessor_table` field and `import_snapshot` verb (or consumer rule) for `EGV-010`.

---

## 3. Mutant Table

Every mutant (`MUTANT_ID = 1..13`) weakens one refusal or binding guard while keeping honest paths reachable (verified non-vacuous: `reach_era_announce_and_activate` exits `1` on all 13 mutants).

| Mutant module | Target invariant | Exit code (`quint run`) | Non-vacuity (`reach_era_announce_and_activate`) | Guard weakened |
|---|---|---|---|---|
| `governance_m_egv001_threshold_zero` | `inv_EGV_001` | `1` (`[violation]`) | `1` (`[violation]`) | Deploys era ledger with `threshold = 0` (`0 < 0` unsealed bug) |
| `governance_m_egv002_keep_sdk_committee` | `inv_EGV_002` | `1` (`[violation]`) | `1` (`[violation]`) | Leaves `compact-js` 1-key committee contract usable and announces it |
| `governance_m_egv003_unsealed_soundness_label` | `inv_EGV_003` | `1` (`[violation]`) | `1` (`[violation]`) | Labels unsealed ledger receipts as `LabelSoundnessEvidence` |
| `governance_m_egv004_reuse_keyset` | `inv_EGV_004` | `1` (`[violation]`) | `1` (`[violation]`) | Announces a second era reusing `keyset_d = 10` |
| `governance_m_egv005_rekey_no_era_no_fail` | `inv_EGV_005` | `1` (`[violation]`) | `1` (`[violation]`) | Starts T25 rekey with no era row and no failed `A2`/`A3` premise |
| `governance_m_egv006_fallback_ended_era_use` | `inv_EGV_006` | `1` (`[violation]`) | `1` (`[violation]`) | Allows consumer use of an inactive era's claim under use-time fallback |
| `governance_m_egv007_in_place_vk_update` | `inv_EGV_007` | `1` (`[violation]`) | `1` (`[violation]`) | Mutates a deployed contract's verifier key in place |
| `governance_m_egv007_subquorum_or_no_tl` | `inv_EGV_007` | `1` (`[violation]`) | `1` (`[violation]`) | Announces a new era with `1 < Q` signer or `startT = nowS` (`0` timelock) |
| `governance_m_egv008_omit_open_params` | `inv_EGV_008` | `1` (`[violation]`) | `1` (`[violation]`) | Omits unruled governance parameters from T15 freeze and states them as rules |
| `governance_m_egv009_include_after_end` | `inv_EGV_009` | `1` (`[violation]`) | `1` (`[violation]`) | Includes call on inactive era in primary mode / records `< 3` premises |
| `governance_m_egv010_fork_mutate_or_no_pred` | `inv_EGV_010` | `1` (`[violation]`) | `1` (`[violation]`) | Migrates across proof-version fork without predecessor link or mutates sealed VK |
| `governance_m_ert007_overwrite_end` | `inv_ERT_004_005_007` | `1` (`[violation]`) | `1` (`[violation]`) | Writes `end(e)` more than once (`endWriteCount = 2`) |
| `governance_m_crr004_use_soundness_ended` | `inv_EGV_006` | `1` (`[violation]`) | `1` (`[violation]`) | Allows consumer to use a claim from a `soundness_ended` era |

---

## 4. State Vocabulary for Trace Replay

### 4.1 State Variables (`vars` in ITF JSON)

In `../traces/governance_main/t_{0..19}.itf.json`, Quint qualifies the module variables under `governance_main::governance::`:

| Quint `var` | ITF JSON state key | Type | Meaning |
|---|---|---|---|
| `gov` | `governance_main::governance::gov` | `GovState` | Complete observable state of **Governance** (deployed contracts and authority readbacks, `SherlockEraTable` eras, pending era and Fenrir roster state, T22 premise test verdicts and T25 rekey gate, included ledger calls, consumer claim uses, T15 parameter freeze, proof-version fork migration). |
| `lastOp` | `governance_main::governance::lastOp` | `str` | Name of the transition that produced the current state (`"init"` at step `0`). |

#### `GovState` fields (`governance_main::governance::gov`)

| Field | Type | Meaning |
|---|---|---|
| `nowS` | `int` | Block timestamp (`0`, advancing in steps of `GRID_STEP` / `TIMELOCK_TL`). |
| `contracts` | `int -> DeployedContractRecord` | Deployed era table (`100`), sealed era ledgers (`1..4`), and caught unsealed fixtures (`50, 51`; `EGV-001..003`, `EGV-007`). |
| `knownContractIds` | `Set[int]` | Active keys in `contracts`. |
| `eras` | `int -> EraRecord` | Append-only era table map (`eraId`, `ledgerId`, `keysetD`, `startS`, `initial*`, `signerCount`, `endIsSet`, `endS`, `endWriteCount`, `eraStatus`; `EGV-004`, `ERT`, `GOV`). |
| `knownEraIds` | `Set[int]` | Active era IDs in `eras` (`Set(0)` at genesis). |
| `currentEraId` | `int` | Last explicitly activated era ID (`0` at genesis; `GOV-003`). |
| `hasPendingEra` | `bool` | Whether an announced era is currently pending (`GOV-002`). |
| `pendingEraId` | `int` | Era ID of the pending era (`0` when none). |
| `rosterGen` | `int` | Monotonic Fenrir roster generation (`GOV-005`, `GOV-010`). |
| `activeRosterD` | `int` | Digest of the Fenrir operator roster currently in force (`500` at genesis). |
| `pendingRotation` | `bool` | Whether a `rotate_roster` timelock is pending (`GOV-005`, `GOV-011`). |
| `pendingRosterD` | `int` | Digest of the pending new roster (`0` when none). |
| `pendingRotationEffS` | `int` | Effective timestamp `nowS + ROTATE_TIMELOCK_RTL` of the pending rotation. |
| `signedVerbNonce` | `int` | Monotonic replay-protection nonce incremented by every quorum-signed verb (`GOV-010`). |
| `premiseTestRan` | `bool` | Whether the T22 devnet premise test has executed (`EGV-009`). |
| `recordedPremiseCount` | `int` | Number of premise verdicts recorded (`3` when `premiseTestRan`; `EGV-009`). |
| `premiseA1Pass`, `premiseA2Pass`, `premiseA3Pass` | `bool` | Recorded pass/fail results for premises `A1`, `A2`, `A3` (`EGV-005`, `EGV-006`, `EGV-009`). |
| `rekeyListHasEraRow` | `bool` | Whether `~/.cache/rekey-t11/LIST.md` holds the era-check row (`EGV-005`). |
| `rekeyStarted` | `bool` | Whether the T25 rekey runner was permitted to start (`EGV-005`). |
| `includedCalls` | `Set[IncludedCallRecord]` | Wrap-keyed ledger calls included on chain (`EGV-009`, `ELD-003`, `ELD-004`). |
| `consumerUses` | `Set[ConsumerUseRecord]` | Consumer claim discharges (`EGV-006`, `CRR-004`, `CRR-006`, `CRR-007`). |
| `ruledGovParams` | `Set[str]` | Subset of `{"TL", "RTL", "Q", "n", "GRID", "SKEW"}` with an owner ruling at T15 (`EGV-008`). |
| `t15FreezeWritten` | `bool` | Whether the T15 governance freeze record has been written (`EGV-008`). |
| `t15OpenParamsListed` | `Set[str]` | Parameters listed as open with recommended values in the T15 freeze record (`EGV-008`). |
| `t15StatedUnruledAsRule` | `bool` | Flag set if any unruled parameter was stated as a binding rule (`false` in honest runs; `EGV-008`). |
| `forkStopAnnounced` | `bool` | Whether a proof-version-stopping Midnight hard fork was announced (`EGV-010`). |
| `forkMigratedBeforeHeight` | `bool` | Whether pre-fork deployment of a new era table and sealed ledgers completed (`EGV-010`). |
| `newTableHasPredecessorLink` | `bool` | Whether the post-fork era table links to its predecessor (`EGV-010`). |
| `newSealedLedgersForNextProofVer` | `bool` | Whether new sealed ledgers were deployed for the new proof version (`EGV-010`). |
| `oldSealedContractsMutatedVk` | `bool` | Flag set if an old sealed contract gained a new key (`false`; `EGV-007`, `EGV-010`). |
| `snapshotRootMarkedTrustDowngrade` | `bool` | Whether the quorum-signed snapshot root is marked as a trust downgrade (`EGV-010` Scenario). |
| `lastRefusal` | `str` | Most recent refusal tag (`"none"`, `"readback_gate_failed_contract_unusable"`, `"vk_change_refused_new_era_required"`, `"duplicate_keyset_d"`, `"below_quorum_Q"`, `"off_grid_start"`, `"rekey_refused_missing_era_row_and_premise_test"`, `"era_inactive_at_inclusion_no_write"`, `"consumer_claim_use_refused"`). |

### 4.2 `lastOp` Values and Implemented EARS IDs

| `lastOp` value | Action in `governance.qnt` | Implemented EARS IDs |
|---|---|---|
| `"init"` | `init` | Initial state (sealed era table `100`, genesis era `0` on ledger `1`, staged sealed ledger `2`) |
| `"advanceTime"` | `doAdvanceTime` | `ERT-004`, `GOV-005` |
| `"deploySealedEraLedger"` | `doDeploySealedEraLedger` | `EGV-001`, `EGV-002`, `ELD-001`, `ELD-002`, `ELD-006` |
| `"catchUnsealedContractAtReadback"` | `doCatchUnsealedContractAtReadback` | `EGV-002`, `EGV-003`, `ERT-002`, `ELD-002` |
| `"attemptVkUpdateOnDeployedContract"` | `doAttemptVkUpdateOnDeployedContract` | `EGV-001`, `EGV-007`, `GOV-009` |
| `"announceEra"` | `doAnnounceEra` | `EGV-004`, `EGV-007`, `ERT-003`, `ERT-006`, `ERT-009`, `GOV-001`, `GOV-002`, `GOV-004`, `GOV-010` |
| `"refuseAnnounceEra"` | `doRefuseAnnounceEra` | `EGV-004`, `ERT-003`, `ERT-009`, `GOV-001`, `GOV-002` |
| `"activateEra"` | `doActivateEra` | `ERT-004`, `ERT-005`, `ERT-007`, `GOV-003` |
| `"cancelPendingEra"` | `doCancelPendingEra` | `GOV-007`, `GOV-010` |
| `"soundnessEndEra"` | `doSoundnessEndEra` | `ERT-004`, `ERT-007`, `GOV-001`, `GOV-006`, `GOV-010`, `CRR-004` |
| `"scheduleRosterRotation"` | `doScheduleRosterRotation` | `EGV-007`, `GOV-001`, `GOV-005`, `GOV-010` |
| `"cancelRosterRotation"` | `doCancelRosterRotation` | `GOV-001`, `GOV-010`, `GOV-011` |
| `"runPremiseTest"` | `doRunPremiseTest` | `EGV-009` |
| `"addEraRowToRekeyList"` | `doAddEraRowToRekeyList` | `EGV-005` |
| `"startT25Rekey"` | `doStartT25Rekey` | `EGV-005` |
| `"refuseT25Rekey"` | `doRefuseT25Rekey` | `EGV-005` |
| `"includeLedgerCall"` | `doIncludeLedgerCall` | `EGV-009`, `ELD-003`, `ELD-004`, `ELD-007` |
| `"refuseInactiveEraInclusion"` | `doRefuseInactiveEraInclusion` | `EGV-009`, `ELD-004` |
| `"consumerUseClaim"` | `doConsumerUseClaim` | `EGV-006`, `ELD-005`, `ELD-007`, `CRR-004`, `CRR-006`, `CRR-007` |
| `"refuseConsumerUseClaim"` | `doRefuseConsumerUseClaim` | `EGV-006`, `CRR-004`, `CRR-006` |
| `"writeT15GovernanceFreeze"` | `doWriteT15GovernanceFreeze` | `EGV-008` |
| `"announceProofVersionFork"` | `doAnnounceProofVersionFork` | `EGV-010` |
| `"migrateBeforeProofVersionFork"` | `doMigrateBeforeProofVersionFork` | `EGV-007`, `EGV-010` |

### 4.3 Concrete Instance Constants

| Constant | `governance_main` | `governance_edge_min` | `governance_edge_at_limit` | `governance_m_*` (`13` mutants) |
|---|---|---|---|---|
| `SEALED_THRESHOLD` | `1` | `1` | `1` | `1` |
| `SEALED_COUNTER` | `0` | `0` | `0` | `0` |
| `ROSTER_SIZE_N` | `16` | `3` | `16` | `16` |
| `QUORUM_Q` | `12` | `2` | `16` | `12` |
| `TIMELOCK_TL` | `70` | `10` | `70` | `70` |
| `ROTATE_TIMELOCK_RTL` | `30` | `10` | `30` | `30` |
| `GRID_STEP` | `10` | `1` | `10` | `10` |
| `L_TX_LANE_LIMIT` | `1024` | `91` | `1024` | `1024` |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 13` |

---

## 5. Model Abstractions

1. **Fenrir operator multisignatures (`GOV-001`, `GOV-010`)**: Distinct valid operator signatures over `(table_addr, verb, roster_gen, nonce)` are abstracted as `signerCount: int` (`1..ROSTER_SIZE_N`) plus monotonic `signedVerbNonce` and `rosterGen`.
2. **Timelocks and grid intervals (`TIMELOCK_TL`, `ROTATE_TIMELOCK_RTL`, `GRID_STEP`)**: Recommended `TL = 7 days` is scaled to `70` logical time units (`7 * GRID_STEP` with `GRID_STEP = 10`, and `10` with `GRID_STEP = 1` in `governance_edge_min`), preserving all ordering, divisibility (`startT % GRID_STEP == 0`), and strict half-open interval `[start(e), end_eff(e))` semantics.
3. **Maintenance authority and verifier keys (`EGV-001`, `EGV-002`, `EGV-007`)**: Ledger 9 maintenance authorities `{committee, threshold, counter}` are modelled as `(committeeCount, threshold, authCounter)` together with `entryKeysMatchExpected: bool` and `vkUpdatedAfterDeploy: bool`.
4. **Unbounded `end_eff` (`ERT-004`)**: An unset `end(e)` with no non-cancelled pending era `P != e` is encoded as `eraEndEff == -1` (unbounded), while `isEraActiveAt` treats `eff < 0` as `+infinity`.

---

## 6. Commands

Run from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/governance/quint`:

```bash
# 1. Typecheck specification and test suite
quint typecheck governance.qnt
# Expected exit code: 0 (no output on success)

quint typecheck governance_test.qnt
# Expected exit code: 0 (no output on success)

# 2. Deterministic scenario tests
quint test --main=governance_test --match='gv_.*Test' --seed=0x20261115 governance_test.qnt
# Expected exit code: 0
# Printed: 5 passing (1108ms)

# 3. Honest instances (main and edge bounds)
quint run governance.qnt --main governance_main --invariant inv --max-samples 2000 --max-steps 15 --seed 0x20261115
# Expected exit code: 0
# Printed: [ok] No violation found (4143ms at 483 traces/second).

quint run governance.qnt --main governance_edge_min --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 0
# Printed: [ok] No violation found (1654ms at 605 traces/second).

quint run governance.qnt --main governance_edge_at_limit --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 0
# Printed: [ok] No violation found (1755ms at 570 traces/second).

# 4. Non-vacuity reachability witnesses (all 6 MUST violate, exit code 1)
quint run governance.qnt --main governance_main --invariant reach_era_announce_and_activate --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (281ms at 25 traces/second).

quint run governance.qnt --main governance_main --invariant reach_soundness_end_and_consumer_refusal --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (270ms at 7 traces/second).

quint run governance.qnt --main governance_main --invariant reach_readback_catches_unsealed_sdk_deploy --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (290ms at 3 traces/second).

quint run governance.qnt --main governance_main --invariant reach_premise_test_fallback_and_rekey --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (215ms at 186 traces/second).

quint run governance.qnt --main governance_main --invariant reach_roster_rotation_and_vk_update_refusal --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (447ms at 58 traces/second).

quint run governance.qnt --main governance_main --invariant reach_t15_open_params_and_fork_migration --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1
# Printed: [violation] Found an issue (424ms at 7 traces/second).

# 5. Guard mutants (all 13 MUST violate their named invariant, exit code 1)
quint run governance.qnt --main governance_m_egv001_threshold_zero --invariant inv_EGV_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (519ms at 2 traces/second).

quint run governance.qnt --main governance_m_egv002_keep_sdk_committee --invariant inv_EGV_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (205ms at 5 traces/second).

quint run governance.qnt --main governance_m_egv003_unsealed_soundness_label --invariant inv_EGV_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (298ms at 3 traces/second).

quint run governance.qnt --main governance_m_egv004_reuse_keyset --invariant inv_EGV_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (445ms at 9 traces/second).

quint run governance.qnt --main governance_m_egv005_rekey_no_era_no_fail --invariant inv_EGV_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (237ms at 17 traces/second).

quint run governance.qnt --main governance_m_egv006_fallback_ended_era_use --invariant inv_EGV_006 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (413ms at 196 traces/second).

quint run governance.qnt --main governance_m_egv007_in_place_vk_update --invariant inv_EGV_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (229ms at 4 traces/second).

quint run governance.qnt --main governance_m_egv007_subquorum_or_no_tl --invariant inv_EGV_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (195ms at 10 traces/second).

quint run governance.qnt --main governance_m_egv008_omit_open_params --invariant inv_EGV_008 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (159ms at 6 traces/second).

quint run governance.qnt --main governance_m_egv009_include_after_end --invariant inv_EGV_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (208ms at 10 traces/second).

quint run governance.qnt --main governance_m_egv010_fork_mutate_or_no_pred --invariant inv_EGV_010 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (302ms at 10 traces/second).

quint run governance.qnt --main governance_m_ert007_overwrite_end --invariant inv_ERT_004_005_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (222ms at 27 traces/second).

quint run governance.qnt --main governance_m_crr004_use_soundness_ended --invariant inv_EGV_006 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Expected exit code: 1 — [violation] Found an issue (220ms at 27 traces/second).

# 6. Replay trace generation (20 ITF JSON traces)
mkdir -p /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/governance/traces/governance_main
quint run governance.qnt --main governance_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/governance/traces/governance_main/t_{seq}.itf.json'
# Expected exit code: 0
# Printed: [ok] No violation found (897ms at 22 traces/second).
```
