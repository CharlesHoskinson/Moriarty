# Quint Formal Specification Report — Bounded Context: Placement (Settlement Edge) (`e2e-placement`)

- **Specification**: [placement.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/placement/quint/placement.qnt)
- **Scenario Tests**: [placement_test.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/placement/quint/placement_test.qnt)
- **ITF Replay Traces (`20` traces)**: [traces/placement_main/](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/placement/traces/placement_main) (`t_0.itf.json` .. `t_19.itf.json`)

---

## 1. Requirements Coverage Table

Every own EARS ID (`8` IDs from `sherlock-e2e-prototype-20261115`: `HLP-001` .. `HLP-008`; `0` completion IDs assigned to this context) appears below, followed by adopted `ZK-009` (`sherlock-processor-final-20261111`).

| ID | Artifact (`inv_*`, action + test, consistency `val`) | Kind | Note |
|---|---|---|---|
| `HLP-001` | `inv_HLP_001`, `doMuninnCheckBoxResult`, `doSubmitSettledCall`, `pl_honestBoxWrapKeepAndSettleTest` | MODELED | Requires every box wrap proof in `ResKept` (`KEEP`) to be checked on `muninn` (`muninnChecked`) and accepted by the ledger `9.1.0.1` verifier (`verifierInvoked and ledger9101VerifiedOk`). Killed by `placement_m_hlp001_keep_unverified`. |
| `HLP-002` | `inv_HLP_002`, `canSubmitJobCall`, `doMuninnCheckBoxResult`, `doSubmitSettledCall`, `pl_refusedBoxWrapDroppedAndReprovedTest` | MODELED | Drops any box wrap proof refused by `muninn`'s ledger `9.1.0.1` check (`ResDropped`, `reproveRequested == true`) and blocks the settle runner from submitting any call that lacks a `ResKept` verdict. Killed by `placement_m_hlp002_submit_refused_wrap`. Negative-formulation gap reported in `Q-placement-4`. |
| `HLP-003` | `inv_HLP_003`, `consistency_constants`, `canBoxReturnResult`, `doBoxFinishAndReturnResult`, `pl_honestBoxWrapKeepAndSettleTest`, `pl_caseAOnlyUnownedProverAndWalletPlacementRefusalsTest` | CONSTANT-CHECKED | Enforces that the proving box runs the argument builder (`argBuilderRanOnBox`), keeps `sherlock-accept-bundle/1` on the box (`not(acceptBundleLeftBox)`), returns `PUBLIC_STATEMENT_WORDS == 4` public words (`c_p`, `input_pub`, `outcome_pub`, `claim_pub`) with `WrapOutput`, and matches the call-vector lanes when kept. Killed by `placement_m_hlp003_ship_accept_bundle`. Four-word vs three-lane mismatch reported in `Q-placement-1`. |
| `HLP-004` | `inv_HLP_004`, `canBoxReturnResult`, `doBoxFinishAndReturnResult`, `pl_honestBoxWrapKeepAndSettleTest`, `pl_caseAOnlyUnownedProverAndWalletPlacementRefusalsTest` | MODELED | Ensures `prog_d`, `r_P`, the run salt, and the call tape never appear in returned prove-result files (`not(privatePartInResult)`) or leave the user's owned box roster (`not(privatePartLeftOwnedBoxes)`). Killed by `placement_m_hlp004_leak_private_in_result`. Single-box vs owned-roster tension reported in `Q-placement-2`. |
| `HLP-005` | `inv_HLP_005`, `consistency_constants`, `canAcceptProverHostConfig`, `doEvaluateProverConfig`, `pl_caseAOnlyUnownedProverAndWalletPlacementRefusalsTest` | CONSTANT-CHECKED | Enforces `SDK_OUTSOURCE_COMMANDS == 0`, `SDK_OUTSOURCE_OPTIONS == 0`, refuses configurations naming a host outside `ownedRoster`, and guarantees `unownedArtifactsSent == Set()` for witnesses, STARK proofs, checkpoints, and shard plans. Killed by `placement_m_hlp005_allow_unowned_prover`. Omission of wrap-layer PLONK MSM/NTT vs `ZK-009` reported in `Q-placement-3`. |
| `HLP-006` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Routing any Case B non-owned helper proving proposal to a separate OpenSpec change on the Veritee sealed Sessions model without editing current requirements is a specification-triage / governance process rule with no runtime machine state. |
| `HLP-007` | `inv_HLP_007`, `consistency_constants`, `canBoxSignOrSubmitTx`, `doRequestTxSignOrSubmitOnBox`, `pl_caseAOnlyUnownedProverAndWalletPlacementRefusalsTest` | CONSTANT-CHECKED | Restricts the wallet directory to `MAX_WALLET_BOXES == 1` designated settlement machine (`BOX_MUNINN`) and requires walletless proving boxes (`huginn`, `odin`) to refuse transaction sign/submit jobs and post the refusal on the box bus (`walletlessTxRefusalsOnBus`). Killed by `placement_m_hlp007_proving_box_submits_tx`. Vacuity when a wallet directory is copied onto a proving box reported in `Q-placement-4`. |
| `HLP-008` | `inv_HLP_008`, `consistency_constants`, `doMuninnCheckBoxResult`, `pl_badSha256AndSwappedJobCallVectorDroppedPreVerifyTest` | CONSTANT-CHECKED | Drops any received box file whose `actualSha256 != reportedSha256` or whose call vector (`lanes 10, 20, 34, 39` of the `1,026`-lane vector) differs from the requested job's call vector **before** invoking the `9.1.0.1` proof verifier (`not(verifierInvoked)`). Killed by `placement_m_hlp008_accept_bad_sha256` and `placement_m_hlp008_accept_other_job_vec`. Uncomputability of `expectedVec` on `muninn` under `HLP-004` reported in `Q-placement-1`. |

### Adopted IDs Modeled in `placement.qnt`

| ID | Owning Change | Artifact | Kind | Note |
|---|---|---|---|---|
| `ZK-009` | `sherlock-processor-final-20261111` (`sherlock-zero-knowledge/spec.md:75-81`) | `inv_ZK_009`, `consistency_constants`, `canRunWrapPlacement`, `doCheckWrapAndBlindingPlacement`, `pl_zk009PlacementGuardsAndSpecGapWitnessesTest` | CONSTANT-CHECKED | Prohibits STARK/`L` helpers, remote unmasked wrap MSM/NTT/quotient/Merkle operations, helper sponge state, blinding-slot reuse or helper-made slots, external STARK aggregators, and `> 1` `ContractCall` per wrap (`MAX_CALLS_PER_WRAP == 1`). Killed by `placement_m_zk009_remote_unmasked_msm`, `placement_m_zk009_reuse_or_helper_slot`, and `placement_m_zk009_multi_call_wrap`. |

---

## 2. Findings

### `Q-placement-1`: `HLP-008` Requires `muninn` to Compare a Job's Expected Call Vector (Lanes `20`, `34`, `39`), Which `muninn` Cannot Compute Without Violating `HLP-003` and `HLP-004` (Plus `4`-Word vs `3`-Lane Mismatch)

- **IDs**: `HLP-008`, `HLP-003`, `HLP-004`
- **Quoted EARS Sentences**:
  - `HLP-008`: *"HLP-008: If the SHA-256 of a received box file differs from the box report or the public call vector differs from the call vector of the requested job, then the muninn ledger check SHALL drop the result before the proof is verified."*
  - `HLP-008` Scenario (*A valid proof of another job*): *"GIVEN a wrap output of job 12 returned for job 13 / WHEN muninn compares the call vector with job 13 / THEN lanes 20, 34 and 39 differ and the result is dropped"*
  - `HLP-003`: *"HLP-003: When a box finishes the proof of a call, the box SHALL run the argument builder and return to muninn only the 4 public words `c_p`, `input_pub`, `outcome_pub` and `claim_pub` with the wrap output."*
  - `HLP-003` Scenario (*muninn compares the four words*): *"GIVEN a `t23-prove-result/1` from odin / WHEN muninn runs `check_prove_result` / THEN the four words equal the recomputed lanes and no private part is in the result"*
  - `HLP-004`: *"HLP-004: The proving box SHALL keep `prog_d`, the `r_P` reference, the run salt and the call tape in box-private state."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_placement_1_job_swap_and_four_words_vs_three_lanes` (`placement.qnt:267-278`):
    1. **Uncomputable `expectedVec` on `muninn`**: Lanes `20` and `34` (`claim_pub`) and lane `39` (`outcome_pub`) depend on `r_P`, the fresh per-run salt, and executing the private call tape (`sherlock-accept-bundle/1`). Under `HLP-004` and `HLP-003`, those secrets stay on the proving box (`odin`/`huginn`), and the argument builder runs *on the proving box* after proving to produce `StatementPublics`. If `odin` mistakenly returns job `12`'s complete `(StatementPublics, WrapOutput)` package when queried for job `13` of the same program (`cp == 500`), `wordsConsistentWithVec(pubs12, vec12)` is `true` and lane `10` (`cp == 500`) matches! For `muninn` to already possess job `13`'s expected lanes `20`, `34`, `39` (`130`, `130`, `230`) to detect `120 != 130` and `220 != 230`, `muninn` would either have to hold the private tape, `r_P`, and run salt itself (violating `HLP-004` / `HLP-003`) or pre-commit `(claim_pub, outcome_pub)` before dispatching the prove job.
    2. **Four words vs three call-vector values**: `HLP-003` returns `4` words (`c_p`, `input_pub`, `outcome_pub`, `claim_pub`) and its Scenario asserts *"the four words equal the recomputed lanes"*, whereas `DDD.md:95, 98` states that `acceptRun`'s `1,026`-lane call vector carries only `3` of those values (`c_p` at lane `10`, `claim_pub` at lanes `20` & `34`, `outcome_pub` at lane `39`) and does **not** carry `input_pub` (`1012`) as a call-vector lane (`claim_pub` is a hash of `(c_p, input_pub)`).
- **Closing Fix**:
  - Specify how `muninn` binds a prove request to a job without holding box-private secrets (e.g., either have the caller/taper compute and record `(c_p, input_pub, claim_pub)` at job dispatch time before proving, or carry an explicit signed/hashed `job_id` in `BoxResult` checked alongside `claim_pub = H(c_p, input_pub)` and the `3` call-vector lanes `10`, `20`/`34`, `39`), and reword `HLP-003` Scenario to state that `c_p`, `claim_pub`, and `outcome_pub` equal lanes `10`, `20`/`34`, and `39` while `claim_pub` recomputes from `(c_p, input_pub)`.

### `Q-placement-2`: Single-Box Confinement in `HLP-004` (*"Private parts never leave the box"*) Contradicts Multi-Box Case A Dispatch and Two-Host Elastic Proving (`HLP-005`, `DDD.md`)

- **IDs**: `HLP-004`, `HLP-005`
- **Quoted EARS Sentences**:
  - `HLP-004` (Heading & EARS): *"Requirement: HLP-004 Private parts never leave the box / HLP-004: The proving box SHALL keep `prog_d`, the `r_P` reference, the run salt and the call tape in box-private state."*
  - `HLP-005`: *"HLP-005: The Sherlock SDK SHALL provide 0 commands and 0 options that send a witness, a STARK proof, a prover checkpoint or a shard plan to a host that the user does not own."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_placement_2_box_private_vs_owned_roster_private` (`placement.qnt:283-288`): in the Case A multi-box roster (`ownedRoster == Set(BOX_MUNINN, BOX_HUGINN, BOX_ODIN)`), either `muninn` compiles/tapes `rsl-call/1` (`DDD.md:180`) and dispatches `(prog_d, r_P, call_tape)` to `huginn`/`odin`, or two owned boxes (`huginn` + `odin`) co-prove shards of one prepared run sharing `(prog_d, input, r_P, input_salt)` (`DDD.md:151, 294`: *"secrets, witnesses and STARK proofs move only to boxes of the roster (ZKE-004)"*). Literal single-box confinement (*"never leave the box"* / *"box-private state"*) forbids both owned-fleet dispatch and multi-box elastic shard proving.
- **Closing Fix**:
  - Distinguish the two boundaries in `HLP-004`: (1) `prog_d`, `r_P`, the run salt, and the call tape SHALL stay within **user-owned roster boxes** (Case A, `HLP-005`), and (2) **prove-result artifacts crossing the settlement edge** (`t23-prove-result/1`, `WrapOutput`) SHALL contain zero `prog_d`, `r_P`, salt, or tape words.

### `Q-placement-3`: `HLP-005`'s Four-Item Outsource Ban Omits Wrap-Layer PLONK Intermediates (Unmasked MSM/NTT/Quotient/Merkle), Sponge State, and Blinding Slots Prohibited by Adopted `ZK-009`

- **IDs**: `HLP-005`, `HLP-006`, `ZK-009`
- **Quoted EARS Sentences**:
  - `HLP-005`: *"HLP-005: The Sherlock SDK SHALL provide 0 commands and 0 options that send a witness, a STARK proof, a prover checkpoint or a shard plan to a host that the user does not own."*
  - `HLP-006` Scenario: *"GIVEN a request to outsource the wrap MSM / WHEN the lead triages the request / THEN the request goes to a new change and no requirement of this change is edited"* (Note also typo `seald` for `sealed` in `HLP-006` line 74: *"Veritee seald Sessions model"*.)
  - `ZK-009`: *"The design SHALL NOT allow a helper (case B of ZK-001) that proves a STARK layer or `L`, a remote unmasked MSM, NTT, quotient or Merkle hash, the full sponge state on a helper, reuse of a blinding slot, or a helper-made slot. No aggregator SHALL receive STARK proofs. One wrap SHALL settle one `ContractCall`."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_placement_3_hlp005_misses_zk009_wrap_msm` (`placement.qnt:293-297`) and mutant `placement_m_zk009_remote_unmasked_msm`: outsourcing an unmasked BLS12-381 PLONK wrap MSM/NTT polynomial vector (the exact example in `HLP-006`'s Scenario!) or using a helper-made blinding slot sends `0` witnesses, `0` STARK proofs, `0` prover checkpoints, and `0` shard plans (`sentHlp005Items == Set()`), satisfying literal `HLP-005` while violating `ZK-009`.
- **Closing Fix**:
  - Expand `HLP-005` to cover all proving/wrapping secrets and intermediates named in `HLP-004` and `ZK-009` (including `L_Fr`/wrap witnesses, unmasked MSM/NTT/quotient/Merkle inputs, sponge states, blinding slots, `prog_d`, `r_P`, and `sherlock-accept-bundle/1`), and fix the `seald` -> `sealed` spelling in `HLP-006`.

### `Q-placement-4`: Negative-Only Formulations of `HLP-007` and `HLP-002` Leave Wallet-on-Proving-Box Configurations and Pre-Verification Drops Unguarded

- **IDs**: `HLP-007`, `HLP-002`, `HLP-001`, `HLP-008`
- **Quoted EARS Sentences**:
  - `HLP-007` (Heading & EARS): *"Requirement: HLP-007 The wallet is not on a proving box / HLP-007: If a job asks a box that holds no wallet directory to sign or submit a transaction, then the box SHALL refuse the job."*
  - `HLP-002`: *"HLP-002: If the muninn ledger check refuses a wrap proof of a box, then the settle runner SHALL submit no call that carries the proof."*
  - `HLP-001`: *"HLP-001: When a box returns a wrap proof, the muninn ledger check SHALL verify the proof with the ledger 9.1.0.1 verifier before the result is kept."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_placement_4_negative_guards_vacuity` (`placement.qnt:305-312`):
    1. **`HLP-007` vacuity when a wallet directory exists on a proving box**: `HLP-007`'s normative sentence never forbids placing or copying a wallet directory onto a remote proving box (`odin`/`huginn`). If `odinHoldsWalletDir == true`, the antecedent `not(odinHoldsWalletDir)` is `false`, so literal `HLP-007` (`not(odinHoldsWalletDir) implies not(odinSubmitsTx)`) evaluates to `true` even when `odin` signs and submits transactions—contradicting the requirement's title (*"The wallet is not on a proving box"*) and `DDD.md:48, 50` (*"the one machine that holds the wallet"*).
    2. **`HLP-002` negative trigger vs unchecked / `HLP-008`-dropped proofs**: `HLP-002` blocks submission only when *"the muninn ledger check refuses a wrap proof"*. Neither an unchecked box file (`ResReturnedUnchecked`, before `muninn` runs) nor a valid proof of job `12` dropped *before* proof verification under `HLP-008` (`verifierRefusedJob12ValidProof == false`) was refused by the proof verifier; `canSubmitJobCall` in `placement.qnt` closes this only by enforcing the positive guard `j.disposition == ResKept and j.muninnChecked and j.ledger9101VerifiedOk`.
- **Closing Fix**:
  - Add an explicit placement clause to `HLP-007` (*"At most 1 designated settlement host in a run's roster SHALL hold a wallet directory; remote proving boxes SHALL hold no wallet directory"*), and rephrase `HLP-002` positively (*"The settle runner SHALL submit a call carrying a box wrap proof only while that box result holds a `KEEP` verdict bound to that job"*).

---

## 3. Mutant Table

All `11` guard mutants (`MUTANT_ID = 1..11`) were executed with `quint run placement.qnt --main <mutant_module> --invariant <target_inv> --max-samples 1000 --max-steps 15 --seed 0x20261115`. Every mutant was killed (`exit code 1`, `[violation] Found an issue`), and every mutant was verified **non-vacuous** by confirming `reach_honest_box_wrap_kept_and_submitted` is still violated (`exit code 1`) on that mutant module.

| Mutant Module | `MUTANT_ID` | Invariant It Must Break | Exit Code Seen | Non-Vacuous (`reach_honest_box_wrap_kept_and_submitted`) |
|---|---:|---|---:|---|
| `placement_m_hlp001_keep_unverified` | `1` | `inv_HLP_001` | `1` | Yes (`exit 1`) |
| `placement_m_hlp002_submit_refused_wrap` | `2` | `inv_HLP_002` | `1` | Yes (`exit 1`) |
| `placement_m_hlp003_ship_accept_bundle` | `3` | `inv_HLP_003` | `1` | Yes (`exit 1`) |
| `placement_m_hlp004_leak_private_in_result` | `4` | `inv_HLP_004` | `1` | Yes (`exit 1`) |
| `placement_m_hlp005_allow_unowned_prover` | `5` | `inv_HLP_005` | `1` | Yes (`exit 1`) |
| `placement_m_hlp007_proving_box_submits_tx` | `6` | `inv_HLP_007` | `1` | Yes (`exit 1`) |
| `placement_m_hlp008_accept_bad_sha256` | `7` | `inv_HLP_008` | `1` | Yes (`exit 1`) |
| `placement_m_hlp008_accept_other_job_vec` | `8` | `inv_HLP_008` | `1` | Yes (`exit 1`) |
| `placement_m_zk009_remote_unmasked_msm` | `9` | `inv_ZK_009` | `1` | Yes (`exit 1`) |
| `placement_m_zk009_reuse_or_helper_slot` | `10` | `inv_ZK_009` | `1` | Yes (`exit 1`) |
| `placement_m_zk009_multi_call_wrap` | `11` | `inv_ZK_009` | `1` | Yes (`exit 1`) |

---

## 4. State Vocabulary for Replay

### 4.1 Top-Level State Variables (`var`) and ITF Keys

| Quint `var` | Exact ITF JSON Key in `t_{0..19}.itf.json` | Quint Type | Meaning |
|---|---|---|---|
| `plc` | `placement_main::placement::plc` | `PlacementState` | Cohesive observable state of the Placement (settlement edge) bounded context. |
| `lastOp` | `placement_main::placement::lastOp` | `str` | Action label set by `init` (`"init"`) and every transition step to drive oracle replay. |

### 4.2 Sum Types and Nested Record Types

- `ResultDisposition`: `ResNone | ResReturnedUnchecked | ResKept | ResDropped`.
- `BoxReturnFault`: `FaultNone | FaultBadSha | FaultWrongJobVec | FaultInvalidProof` (action input parameter tag).
- `CallVecSummary`: `{ cpLane10: int, claimLane20: int, claimLane34: int, outcomeLane39: int }` (the 4 binding lanes of the `1,026`-lane wrap call vector).
- `StatementPublicsRec`: `{ cp: int, inputPub: int, outcomePub: int, claimPub: int }` (`HLP-003` 4-word `StatementPublics` projection).
- `JobRecord` (value type of `plc.jobs: int -> JobRecord`, keyed by `JOB_IDS = Set(12, 13)`):
  - `jobId: int` — Job identifier (`12` or `13`, matching `HLP-008` Scenario).
  - `expectedVec: CallVecSummary` — Expected call-vector lanes for `jobId` (`cpLane10 = 500`, `claimLane20/34 = 120` vs `130`, `outcomeLane39 = 220` vs `230`).
  - `resultPresent: bool` — True once a proving box returned a `BoxResult` for `jobId`.
  - `fromBox: int` — Host ID of the proving box (`2` = `huginn`, `3` = `odin`, `4` = extra owned box).
  - `reportedSha256: int` — SHA-256 digest reported on the box bus (`HLP-008`).
  - `actualSha256: int` — Actual SHA-256 digest of the received result file on `muninn` (`HLP-008`).
  - `returnedPublics: StatementPublicsRec` — Four public words returned by the box's argument builder (`HLP-003`).
  - `returnedPublicWordCount: int` — Count of public words returned (`4`, `HLP-003`).
  - `returnedVec: CallVecSummary` — Public call-vector lanes in the received `WrapOutput` (`HLP-008`).
  - `proofCryptoValid: bool` — Whether the received `WrapOutput` proof bytes pass Midnight ledger `9.1.0.1` verification (`HLP-001`, `HLP-002`).
  - `argBuilderRanOnBox: bool` — True iff `accept_args.py` ran on the proving box (`HLP-003`).
  - `acceptBundleLeftBox: bool` — True if `sherlock-accept-bundle/1` left the proving box (`false`, `HLP-003`).
  - `privatePartInResult: bool` — True if `prog_d`, `r_P`, `runSalt`, or `callTape` appeared in the returned result file (`false`, `HLP-004`).
  - `wordsMatchCallVecLanes: bool` — True iff `returnedPublics` agrees with `returnedVec` on lanes `10`, `20`, `34`, `39` (`HLP-003`).
  - `verifierInvoked: bool` — True iff `muninn` invoked the ledger `9.1.0.1` proof verifier (`false` when dropped early by `HLP-008`).
  - `muninnChecked: bool` — True once `muninn` processed the returned `BoxResult` (`HLP-001`, `HLP-008`).
  - `ledger9101VerifiedOk: bool` — True iff ledger `9.1.0.1` accepted the proof (`HLP-001`, `HLP-002`).
  - `disposition: ResultDisposition` — Current lifecycle disposition (`ResNone`, `ResReturnedUnchecked`, `ResKept`, `ResDropped`).
  - `reproveRequested: bool` — True when a dropped box result is queued to be proved again (`HLP-002`).
  - `submittedToChain: bool` — True once the settle runner submitted the call carrying this wrap proof (`HLP-002`).

### 4.3 `PlacementState` Fields

| Field | Type | Meaning |
|---|---|---|
| `ownedRoster` | `Set[int]` | Set of user-owned Case A machine IDs (`1` = `muninn`, `2` = `huginn`, `3` = `odin`, `4` = extra owned box). |
| `walletBoxes` | `Set[int]` | Set of machines holding a wallet directory (`Set(1)` = `muninn` only, `HLP-007`). |
| `jobs` | `int -> JobRecord` | Per-job `BoxResult` acceptance, `WrapVerdict`, and settlement submission state (`12`, `13`). |
| `privatePartLeftOwnedBoxes` | `bool` | True if any private witness/secret left the user's owned box roster (`false`, `HLP-004`, `HLP-005`). |
| `unownedHostConfigAccepted` | `bool` | True if a configuration naming an unowned host (`99`) was accepted (`false`, `HLP-005`). |
| `unownedArtifactsSent` | `Set[str]` | Set of artifact kinds (`"witness"`, `"stark_proof"`, `"checkpoint"`, `"shard_plan"`) sent to unowned hosts (`Set()`, `HLP-005`). |
| `caseAConfigsAccepted` | `int` | Count of accepted user-owned Case A prover configurations (`HLP-005`). |
| `unownedConfigRefusals` | `int` | Count of refused unowned-host configurations (`HLP-005`). |
| `boxesThatSignedOrSubmitted` | `Set[int]` | Set of boxes that signed or submitted a transaction (`<= Set(1)`, `HLP-007`). |
| `walletlessTxRefusalsOnBus` | `Set[int]` | Set of walletless proving boxes that refused a sign/submit job and posted the refusal on the bus (`HLP-007`). |
| `helperProvedStarkOrL` | `bool` | True if an unowned helper proved a STARK layer or `L` (`false`, `ZK-009`). |
| `remoteUnmaskedWrapOp` | `bool` | True if an unmasked wrap MSM, NTT, quotient, or Merkle hash ran remotely (`false`, `ZK-009`). |
| `helperHoldsFullSpongeState` | `bool` | True if an unowned helper held the full sponge state (`false`, `ZK-009`). |
| `blindingSlotReused` | `bool` | True if a blinding slot was reused (`false`, `ZK-009`). |
| `helperMadeBlindingSlot` | `bool` | True if a blinding slot originated on a helper (`false`, `ZK-009`). |
| `aggregatorReceivedStarkProofs` | `bool` | True if an external aggregator received STARK proofs (`false`, `ZK-009`). |
| `callsPerWrap` | `int` | Number of `ContractCall`s settled per wrap proof (`1`, `ZK-009`). |
| `userBatchOptInValid` | `bool` | True iff user batching is opt-in with `1` batch digest per `acceptRun` (`ZK-009`). |
| `localBlindedWrapsCompleted` | `int` | Count of valid Case A local blinded single-call wraps completed (`ZK-009`). |
| `zk009Refusals` | `int` | Count of `ZK-009` helper/wrap placement violations refused (`ZK-009`). |
| `lastRefusal` | `str` | Diagnostic tag of the most recent placement/check refusal (`"none"` initially). |

### 4.4 `lastOp` Values and Implemented Requirements

| `lastOp` Value | Action in `placement.qnt` | Implemented EARS IDs |
|---|---|---|
| `"init"` | `init` | Initial placement state (`HLP-001..005`, `HLP-007`, `HLP-008`, `ZK-009`) |
| `"boxFinishAndReturnResult"` | `doBoxFinishAndReturnResult` | `HLP-003`, `HLP-004`, `HLP-008` |
| `"muninnCheckBoxResult"` | `doMuninnCheckBoxResult` | `HLP-001`, `HLP-002`, `HLP-003`, `HLP-008` |
| `"submitSettledCall"` | `doSubmitSettledCall` | `HLP-001`, `HLP-002` |
| `"evaluateProverConfig"` | `doEvaluateProverConfig` | `HLP-005` |
| `"requestTxSignOrSubmitOnBox"` | `doRequestTxSignOrSubmitOnBox` | `HLP-007` |
| `"checkWrapAndBlindingPlacement"` | `doCheckWrapAndBlindingPlacement` | `ZK-009` |

### 4.5 Constants of Concrete Instances

| Constant | `placement_main` | `placement_edge_min` | `placement_edge_at_limit` | Mutant Modules (`m_*`) |
|---|---:|---:|---:|---:|
| `PUBLIC_STATEMENT_WORDS` | `4` | `4` | `4` | `4` |
| `CALL_VECTOR_LANES` | `1026` | `1026` | `1026` | `1026` |
| `LANE_ANCHOR_CP` | `10` | `10` | `10` | `10` |
| `LANE_CLAIM_PUB_A` | `20` | `20` | `20` | `20` |
| `LANE_CLAIM_PUB_B` | `34` | `34` | `34` | `34` |
| `LANE_OUTCOME_PUB` | `39` | `39` | `39` | `39` |
| `SDK_OUTSOURCE_COMMANDS` | `0` | `0` | `0` | `0` |
| `SDK_OUTSOURCE_OPTIONS` | `0` | `0` | `0` | `0` |
| `MAX_WALLET_BOXES` | `1` | `1` | `1` | `1` |
| `MAX_CALLS_PER_WRAP` | `1` | `1` | `1` | `1` |
| `OWNED_PROVER_BOX_COUNT` | `2` | `1` | `3` | `2` |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 11` |

---

## 5. Model Abstractions

1. **Boxes and Hosts as Symbolic Integers**: Owned roster machines (`muninn = 1`, `huginn = 2`, `odin = 3`, optional 3rd prover box `= 4`) and unowned helper hosts (`99`) are represented as distinct small integers.
2. **Call Vector Projected to Its Four Binding Lanes**: Instead of allocating a `1,026`-element list per job, `CALL_VECTOR_LANES = 1026` and the exact binding lane indices (`LANE_ANCHOR_CP = 10`, `LANE_CLAIM_PUB_A = 20`, `LANE_CLAIM_PUB_B = 34`, `LANE_OUTCOME_PUB = 39`) are checked as constants while `CallVecSummary` stores the 4 lane values (`cpLane10`, `claimLane20`, `claimLane34`, `outcomeLane39`) that `HLP-003` and `HLP-008` inspect.
3. **SHA-256 Digests, Field Words, and PLONK Verification as Finite Values**: File SHA-256 digests (`7012`, `7013`, plus `+1` on corruption), public field words (`500`, `120`/`130`, `220`/`230`, `1012`/`1013`), and Midnight ledger `9.1.0.1` `VerifierKey::verify` outcomes (`proofCryptoValid: bool`) are abstracted into finite integers and booleans.
4. **Two Representative Jobs (`12` and `13`)**: Job IDs `12` and `13` directly instantiate `HLP-008`'s cross-job substitution scenario (*job `12` wrap output returned for job `13`* under shared `c_p = 500`) as well as independent honest/refused/reproved lifecycles.
5. **OpenSpec Change-Triage Governance**: `HLP-006` (requiring Case B outsourced-proving proposals to be opened as a separate Veritee sealed Sessions OpenSpec change) is an authoring/governance workflow rule rather than a runtime machine transition and is recorded as a `SPEC-CHECKED candidate`.

---

## 6. Commands

Run all commands below from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/placement/quint`:

```bash
# 1. Typecheck base specification + instances + mutants (expected exit: 0; printed: empty)
quint typecheck placement.qnt
# Exit: 0 | Output: (none)

# 2. Typecheck deterministic scenario test module (expected exit: 0; printed: empty)
quint typecheck placement_test.qnt
# Exit: 0 | Output: (none)

# 3. Run deterministic scenario tests (expected exit: 0)
quint test --main=placement_test --match='pl_.*Test' --seed=0x20261115 placement_test.qnt
# Exit: 0 | Printed: 5 passing (1581ms)

# 4. Honest simulation: main instance (expected exit: 0)
quint run placement.qnt --main placement_main --invariant inv --max-samples 2000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (4264ms at 469 traces/second).

# 5. Honest simulation: minimum-roster edge instance (expected exit: 0)
quint run placement.qnt --main placement_edge_min --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (1223ms at 818 traces/second).

# 6. Honest simulation: upper-roster edge instance (expected exit: 0)
quint run placement.qnt --main placement_edge_at_limit --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (1836ms at 545 traces/second).

# 7. Reachability witnesses on placement_main (all 5 MUST violate -> expected exit: 1)
quint run placement.qnt --main placement_main --invariant reach_honest_box_wrap_kept_and_submitted --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (173ms at 6 traces/second).

quint run placement.qnt --main placement_main --invariant reach_refused_box_wrap_dropped_and_reproved --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (122ms at 33 traces/second).

quint run placement.qnt --main placement_main --invariant reach_bad_sha_or_other_job_dropped_pre_verify --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (276ms at 4 traces/second).

quint run placement.qnt --main placement_main --invariant reach_unowned_helper_and_remote_msm_refused --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (298ms at 7 traces/second).

quint run placement.qnt --main placement_main --invariant reach_walletless_proving_box_refuses_tx_on_bus --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (180ms at 28 traces/second).

# 8. Guard mutants (all 11 MUST violate their target invariant -> expected exit: 1)
quint run placement.qnt --main placement_m_hlp001_keep_unverified --invariant inv_HLP_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (241ms at 4 traces/second).

quint run placement.qnt --main placement_m_hlp002_submit_refused_wrap --invariant inv_HLP_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (275ms at 7 traces/second).

quint run placement.qnt --main placement_m_hlp003_ship_accept_bundle --invariant inv_HLP_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (280ms at 4 traces/second).

quint run placement.qnt --main placement_m_hlp004_leak_private_in_result --invariant inv_HLP_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (173ms at 6 traces/second).

quint run placement.qnt --main placement_m_hlp005_allow_unowned_prover --invariant inv_HLP_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (274ms at 7 traces/second).

quint run placement.qnt --main placement_m_hlp007_proving_box_submits_tx --invariant inv_HLP_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (199ms at 5 traces/second).

quint run placement.qnt --main placement_m_hlp008_accept_bad_sha256 --invariant inv_HLP_008 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (320ms at 3 traces/second).

quint run placement.qnt --main placement_m_hlp008_accept_other_job_vec --invariant inv_HLP_008 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (259ms at 27 traces/second).

quint run placement.qnt --main placement_m_zk009_remote_unmasked_msm --invariant inv_ZK_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (189ms at 11 traces/second).

quint run placement.qnt --main placement_m_zk009_reuse_or_helper_slot --invariant inv_ZK_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (185ms at 5 traces/second).

quint run placement.qnt --main placement_m_zk009_multi_call_wrap --invariant inv_ZK_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (301ms at 3 traces/second).

# 9. Generate 20 ITF replay traces (expected exit: 0)
mkdir -p /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/placement/traces/placement_main
quint run placement.qnt --main placement_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/placement/traces/placement_main/t_{seq}.itf.json'
# Exit: 0 | Printed: [ok] No violation found (314ms at 64 traces/second).
```
