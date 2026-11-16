# Sherlock E2E Assurance — Independent Quint Formal Specification Report

- **Lane**: `e2e-assurance` (Gemini 4 Carbon)
- **Bounded Context**: Assurance (`sherlock-e2e-zero-knowledge`, `sherlock-e2e-stark-security`, completion `SQG-014..016`, `ZK-012..019`, `LPC-004`, `LPC-016`, adopted `CPR-001`, `DG-001`, `SQG-001`, `ZK-001`, `ZK-002`, `ZK-004`, `ZK-007`)
- **Quint Artifacts**:
  - Model: `assurance.qnt` (`assurance`, `assurance_main`, `assurance_edge_min`, `assurance_edge_at_limit`, and 24 guard/binding mutants `assurance_m_*`)
  - Requirement tests: `assurance_test.qnt` (`8 passing` multi-step scenario `run` tests)
  - ITF replay traces: `../traces/assurance_main/t_0.itf.json` .. `t_19.itf.json` (20 traces, 15 steps max, seed `0x20261115`)

---

## 1. Requirement Coverage Table (ID Table)

Every own EARS ID (`ZKE-001` .. `ZKE-007`, `SSG-001` .. `SSG-007`), completion EARS ID (`SQG-014` .. `SQG-016`, `ZK-012` .. `ZK-019`, `LPC-004`, `LPC-016`), and cited adopted EARS ID (`CPR-001`, `DG-001`, `SQG-001`, `ZK-001`, `ZK-002`, `ZK-004`, `ZK-007`) is accounted for below.

| ID | Artifact (`inv_*`, action + test, consistency `val`) | Kind | Note |
|---|---|---|---|
| `ZKE-001` | `inv_ZKE_001`, `consistency_constants`; actions `prepareRunAndLog`, `refuseConstantRpExit4`; test `as_Zke001And005And006AndZk012And013AndSsg007PrepareRunsTest`; mutant `assurance_m_zke001_const_rp_without_feature` | `MODELED` / `CONSTANT-CHECKED` | Forbids constant `r_P` (`RandConstant`) in builds lacking `test-seed`, exiting with `EXIT_CODE_SEED_REFUSED = 4` and writing no proof. See Finding `Q-assurance-4`. |
| `ZKE-002` | `inv_ZKE_002`; actions `settleWrapProofAndScan`, `refuseSettleTestSeedProof`; test `as_Zke002And003And007AndZk014To017WrapAndSettlePrivacyTest`; mutant `assurance_m_zke002_settle_test_seed` | `MODELED` | Blocks any wrap proof produced by a `test-seed` binary (`builtWithTestSeed == true`) from Preview settlement. See Finding `Q-assurance-4`. |
| `ZKE-003` | `inv_ZKE_003`; action `settleWrapProofAndScan`; test `as_Zke002And003And007AndZk014To017WrapAndSettlePrivacyTest`; mutant `assurance_m_zke003_publish_secret_class` | `MODELED` | Requires settled calls to publish only call vector + blinded wrap proof with `0` hits across all 5 secret classes (witness, STARK proof, `r_P`, salt, plain `prog_d`). |
| `ZKE-004` | `inv_ZKE_004`, `consistency_constants`; actions `transferSecretFileToBox`, `refuseSecretTransferOutsideRoster`; test `as_Zke004AndZk018AndZk001OwnedRosterAndCaseBRefusalTest`; mutant `assurance_m_zke004_transfer_secret_outside_roster` | `MODELED` / `CONSTANT-CHECKED` | Refuses moving any of the 5 secret file classes (`deploy.secret`, `run.secret`, witness, trace, STARK proof) outside `ownedRoster` and preserves `SECRET_FILE_MODE = 600`. |
| `ZKE-005` | `inv_ZKE_005`; actions `prepareRunAndLog`, `refuseMissingDeploySecret`; test `as_Zke001And005And006AndZk012And013AndSsg007PrepareRunsTest`; mutant `assurance_m_zke005_redraw_rp_when_missing` | `MODELED` | Refuses run preparation when `deploy.secret` is absent on the box (`odin` before transfer), never redrawing a new `r_P`. |
| `ZKE-006` | `inv_ZKE_006`; action `prepareRunAndLog`; test `as_Zke001And005And006AndZk012And013AndSsg007PrepareRunsTest`; mutant `assurance_m_zke006_debug_prints_secret` | `MODELED` | Ensures `Debug` formatting of leaf blinder types emits `0` words of `r_P` or input salt into `debugLogs`. |
| `ZKE-007` | `inv_ZKE_007`; action `makeWrapProof`; test `as_Zke002And003And007AndZk014To017WrapAndSettlePrivacyTest`; mutant `assurance_m_zke007_deterministic_wrap_blinder` | `MODELED` | Ensures proving one statement twice on CPU and twice on GPU yields 4 pairwise-distinct byte strings, all accepted by ledger `9.1.0.1`. See Finding `Q-assurance-6`. |
| `SSG-001` | `inv_SSG_001`, `consistency_constants`; actions `checkAppKeyD3`, `refuseLowD3AppKey`; test `as_Ssg001And002AndCpr001AppD3AndWrapBoundsTest`; mutant `assurance_m_ssg001_accept_low_d3_app_key` | `MODELED` / `CONSTANT-CHECKED` | Requires qualified per-program R_SL app keys (`d04 withdraw`, 16,346 cols) to pass `MIN_D3_TERM_DECIBITS = 1000` (`100.0` bits) and `MIN_D3_GRINDABLE_DECIBITS = 1010` (`101.0` bits). See Finding `Q-assurance-1`. |
| `SSG-002` | `inv_SSG_002`; actions `auditCratesAndWrapBoundsPass`, `refusePlonkInStarkOrWrapAboveK21`; test `as_Ssg001And002AndCpr001AppD3AndWrapBoundsTest`; mutant `assurance_m_ssg002_allow_plonk_in_stark_crate` | `MODELED` | Fails the T25 crate gate if `sherlock-leaf` or `sherlock-lfr` holds any PLONK/KZG dependency (`midnight-proofs`, `halo2`). |
| `SSG-003` | `inv_SSG_003`; actions `refuseSecurityGateEvaluation`, `completeT12ChecksAndFixes`, `writePassingGateRecordAndQualify`; test `as_Ssg003And004And006AndSqg014And016SecurityGateQualificationTest`; mutant `assurance_m_ssg003_pass_gate_with_confirmed_cx` | `MODELED` | Blocks security gate pass while `unfixedConfirmedCxCount > 0` (e.g. `GP2-C2.1` CONFIRMED without fix commit). See Finding `Q-assurance-2`. |
| `SSG-004` | `inv_SSG_004`, `consistency_constants`; action `writePassingGateRecordAndQualify`; test `as_Ssg003And004And006AndSqg014And016SecurityGateQualificationTest`; mutant `assurance_m_ssg004_omit_not_attempted_rows` | `MODELED` / `CONSTANT-CHECKED` | Requires written gate records to list all `NOT_ATTEMPTED_INIT_COUNT` (`8` in main, `1` in `edge_min`) `NOT ATTEMPTED` counterexample rows as open. |
| `SSG-005` | `inv_SSG_005`; action `landAndAuditCircuitChange`; test `as_Ssg005AndLpc004And016CircuitChangeAndVerdictGateTest`; mutant `assurance_m_ssg005_missing_circuit_change_row` | `MODELED` | Requires every landed post-`c4bd312` circuit change to have a `CHANGES.md` row naming commit, AIR, and Quint lane. See Finding `Q-assurance-6`. |
| `SSG-006` | `inv_SSG_006`; actions `writePassingGateRecordAndQualify`, `refuseSecurityGateEvaluation`; test `as_Ssg003And004And006AndSqg014And016SecurityGateQualificationTest`; mutant `assurance_m_ssg006_qualify_on_stale_g5` | `MODELED` | Writes gate record naming `activeManifestSha` and promotes manifest from `KeyPrototype` to `KeyQualified` only when `G1..G5 + FP-8` pass on the active key parameters (refusing stale `T10m` `G5` mappings). See Finding `Q-assurance-2`. |
| `SSG-007` | `inv_SSG_007`; action `prepareRunAndLog`; test `as_Zke001And005And006AndZk012And013AndSsg007PrepareRunsTest`; mutant `assurance_m_ssg007_claim_qualified_before_gate` | `MODELED` | Marks `securityClaimInRecord == SecNotQualified` in every run record written before the security gate passes on the active key set. |
| `SQG-014` | `inv_SQG_014`, `consistency_constants`; action `runG3TranscriptCheck`; test `as_Ssg003And004And006AndSqg014And016SecurityGateQualificationTest`; mutant `assurance_m_sqg014_pass_g3_on_leg_mismatch` | `MODELED` / `CONSTANT-CHECKED` | Completion ID: requires all `G3_TRANSCRIPT_LEGS = 5` transcript legs (CPU, GPU, stark-backend verifier, `L` in `L_Fr`, wrap) to agree on rekeyed keys. |
| `SQG-015` | `inv_SQG_015`, `consistency_constants`; actions `completeT12ChecksAndFixes`, `recordGateCheckFailure`; test `as_Sqg015DoubleGateFailureStopsT12RowTest`; mutant `assurance_m_sqg015_ignore_double_gate_fail` | `MODELED` / `CONSTANT-CHECKED` | Completion ID: runs `G4` (with `L_Fr`), `G1`, `G2`, `FP-8`, and ledger `9.1.0.1` check on new keys, and stops row `T12` upon `MAX_GATE_FAILURES_AFK = 2` failures. See Finding `Q-assurance-2`. |
| `SQG-016` | `inv_SQG_016`; actions `refuseSecurityGateEvaluation`, `writePassingGateRecordAndQualify`; test `as_Ssg003And004And006AndSqg014And016SecurityGateQualificationTest`; mutant `assurance_m_sqg016_pass_with_unverified_step` | `MODELED` | Completion ID: blocks gate pass whenever `unverifiedRunbookSteps > 0`. See Finding `Q-assurance-2`. |
| `ZK-012` | `inv_ZK_012`, `consistency_constants`; actions `stageWorldReadableDeploySecret`, `refuseBadModeDeploySecret`, `prepareRunAndLog`; test `as_Zke001And005And006AndZk012And013AndSsg007PrepareRunsTest`; mutant `assurance_m_zk012_accept_bad_secret_mode` | `MODELED` / `CONSTANT-CHECKED` | Completion ID: draws `r_P` once per deployment from OS CSPRNG into `deploy.secret` with `SECRET_FILE_MODE = 600` and refuses group/world-readable files (`644`). See Finding `Q-assurance-4`. |
| `ZK-013` | `inv_ZK_013`; action `prepareRunAndLog`; test `as_Zke001And005And006AndZk012And013AndSsg007PrepareRunsTest`; mutant `assurance_m_zk013_reuse_run_salt` | `MODELED` | Completion ID: draws a fresh `inputSalt` in `run.secret` per prepared run so two runs of one input have distinct salts and distinct `claimPub`. See Findings `Q-assurance-4`, `Q-assurance-5`. |
| `ZK-014` | `inv_ZK_014`, `consistency_constants`; actions `makeWrapProof`, `refuseEnvSeedInReleaseWrapExit4`; test `as_Zke002And003And007AndZk014To017WrapAndSettlePrivacyTest`; mutant `assurance_m_zk014_allow_gw_det_in_release` | `MODELED` / `CONSTANT-CHECKED` | Completion ID: non-`test-seed` wrap build receiving `SHERLOCK_TEST_SEED` / `GW_DET` / `GW_CHIP_SEED` / `GW_SEED` exits `4` and writes no proof. See Finding `Q-assurance-4`. |
| `ZK-015` | `inv_ZK_015` (`inv_ZKE_007`); action `makeWrapProof`; test `as_Zke002And003And007AndZk014To017WrapAndSettlePrivacyTest`; mutant `assurance_m_zke007_deterministic_wrap_blinder` | `MODELED` | Completion ID: two wrap proofs of one statement differ and both pass ledger `9.1.0.1` verification. See Finding `Q-assurance-6`. |
| `ZK-016` | `inv_ZK_016`; actions `makeWrapProof`, `refuseReplayedBlindingSlot`; test `as_Zke002And003And007AndZk014To017WrapAndSettlePrivacyTest`; mutant `assurance_m_zk016_allow_replayed_slot` | `MODELED` | Completion ID: enforces one-use `blindingSlotId` consumption and refuses replayed slot IDs. See Finding `Q-assurance-3`. |
| `ZK-017` | `inv_ZK_017`; action `makeWrapProof`; test `as_Zke002And003And007AndZk014To017WrapAndSettlePrivacyTest`; mutant `assurance_m_zke007_deterministic_wrap_blinder` | `MODELED` | Completion ID: GPU wrap proofs on `odin`/`huginn` draw fresh blinders from `OsRng`. |
| `ZK-018` | `inv_ZK_018`; actions `prepareRunAndLog`, `makeWrapProof`, `refuseCaseBUnownedHost`; test `as_Zke004AndZk018AndZk001OwnedRosterAndCaseBRefusalTest`; mutant `assurance_m_zk018_allow_case_b_unowned_job` | `MODELED` | Completion ID: refuses Case B configurations sending leaf, node, `L`, `L_Fr`, or wrap jobs to `HostExternalHelper`. |
| `ZK-019` | `consistency_zk019`; test `as_ConsistencyAndZk019AxiomsTest` | `SPEC-CHECKED candidate` | Static Lean 4 `#print axioms` build check (`pv2_inj`, `pv2_count_det` depend only on `propext`, `Quot.sound`, `Classical.choice`) with no runtime state transition; `consistency_zk019` checks the 3-axiom set excludes `sorryAx`. |
| `LPC-004` | `inv_LPC_004`; action `landAndAuditCircuitChange`; test `as_Ssg005AndLpc004And016CircuitChangeAndVerdictGateTest`; mutant `assurance_m_ssg005_missing_circuit_change_row` | `MODELED` | Completion ID: requires every closed post-`c4bd312` circuit change to run a Carbon Quint lane and file a lead verdict. See Finding `Q-assurance-6`. |
| `LPC-016` | `inv_LPC_016`; action `evaluateLoopRowForT15`; test `as_Ssg005AndLpc004And016CircuitChangeAndVerdictGateTest`; mutant `assurance_m_lpc016_count_done_without_verdict` | `MODELED` | Completion ID: counts a DONE row as not DONE for `T15` unless a lead verdict exists in `qa/gemini-quint`. |
| `CPR-001`, `DG-001`, `SQG-001`, `ZK-001`, `ZK-002`, `ZK-004`, `ZK-007` *(adopted)* | `inv_ADOPTED_ASSURANCE`, `consistency_constants`; actions `auditCratesAndWrapBoundsPass`, `refusePlonkInStarkOrWrapAboveK21`, `refuseWrapKeyFixingLTranscript`, `settleWrapProofAndScan`; tests `as_Ssg001And002AndCpr001AppD3AndWrapBoundsTest`, `as_Zke002And003And007AndZk014To017WrapAndSettlePrivacyTest`; mutant `assurance_m_cpr001_allow_wrap_k22` | `MODELED` / `CONSTANT-CHECKED` | Adopted (`sherlock-processor-final-20261111`): wrap `k in 19..21` (`CPR-001`, `DG-001`), 100-bit security claimed only after `G1..G5` (`SQG-001`), Case A owned-host execution (`ZK-001`), published view restricted to call vector + wrap proof with private `L` transcript and salted anchor `C_P`/`A_j` (`ZK-002`, `ZK-004`, `ZK-007`). |

---

## 2. Findings (`Q-assurance-1` .. `Q-assurance-6`)

### `Q-assurance-1` — `SSG-001` EARS text omits the `101`-bit grindable floor stated in its own Scenario
- **IDs**: `SSG-001` (compare `SQG-001`)
- **Quoted sentences**:
  - `SSG-001` EARS (`sherlock-e2e-stark-security/spec.md:4`): *"When a per-program R_SL app key is generated, the D3 calculator SHALL print every term of the key at 100 bits or more."*
  - `SSG-001` Scenario (`spec.md:9-11`): *"GIVEN the app key of `d04 withdraw` (16,346 columns) WHEN `fp8_d3` reads the key THEN every term is at least 100 bits and every grindable term at least 101 bits"*
- **Witness state**:
  - A per-program R_SL app key with `minTermDeciBits = 1005` (`100.5` bits) on a grindable term satisfies `SSG-001`'s normative EARS sentence (`>= 100` bits) while failing `SSG-001`'s Scenario (`< 101` bits) and `fp8_d3`'s margin rule (`AssureRefusalLowAppKeyD3((1020, 1005))`). Also note that *"SHALL print ... at 100 bits or more"* is phrased as an output promise on the calculator rather than a key-rejection guard when a key falls below the bound!
- **What would close it**:
  - Rewrite `SSG-001` to: *"When a per-program R_SL app key is generated, the key generator SHALL run the D3 calculator and SHALL refuse the key unless every term is at least 100 bits and every grindable term is at least 101 bits."*

### `Q-assurance-2` — `SSG-006`'s unconditional qualification trigger contradicts `SSG-003` (CONFIRMED defects), `SQG-016` (UNVERIFIED steps), and `SQG-015` (ledger `9.1.0.1` check)
- **IDs**: `SSG-006`, `SSG-003`, `SQG-015`, `SQG-016`
- **Quoted sentences**:
  - `SSG-006`: *"When G1 to G4, G5 and FP-8 pass on a key set, the security gate SHALL write a gate record with the key manifest sha256 and the log path of each check."* / Scenario: *"WHEN the record is written THEN the key manifest status changes from `prototype` to `qualified`"*
  - `SSG-003`: *"If the counterexample record holds a row with verdict CONFIRMED and no fix commit, then the security gate SHALL report the gate as not passed."*
  - `SQG-016`: *"If a RUNBOOK step is UNVERIFIED, then the security gate SHALL report the gate as not passed."*
  - `SQG-015`: *"When the rekey completes, the security gate SHALL run G4 with the `L_Fr` part, G1, G2, FP-8 and the ledger 9.1.0.1 check on the new keys."*
- **Witness state**:
  - Take `g1..g5` and `FP-8` passed on `T25_MANIFEST_SHA`, while `unfixedConfirmedCxCount == 1` (`GP2-C2.1` CONFIRMED without a fix commit, `SSG-003`) or `unverifiedRunbookSteps > 0` (`SQG-016`, e.g., the ledger `9.1.0.1` check of `SQG-015` has no log). `SSG-006` mandates writing the passing gate record and flipping the manifest to `qualified`, whereas `SSG-003` and `SQG-016` mandate reporting the gate as **not passed**!
- **What would close it**:
  - Strengthen `SSG-006`'s `When` condition to require `G1..G5`, `FP-8`, and the ledger `9.1.0.1` check (`SQG-015`) all verified with logs (`unverifiedRunbookSteps == 0`, `SQG-016`) AND zero unfixed `CONFIRMED` counterexamples (`SSG-003`).

### `Q-assurance-3` — `ZK-016` ("reuses a blinding slot" on `wrapd` or STARK worker) contradicts `ZK-001` ("No PCS masking... in case A") and `D-040` (no slot IDs or slot pool)
- **IDs**: `ZK-016`, `ZK-001`, `ZK-017`
- **Quoted sentences**:
  - `ZK-016` (`sherlock-completion-20261115/specs/sherlock-zero-knowledge/spec.md:48-52`): *"If a request to `wrapd serve` or to the STARK worker reuses a blinding slot, then the daemon SHALL refuse the request."* / Scenario: *"WHEN a client replays a slot id THEN the daemon refuses the request and makes no proof"*
  - `ZK-001` (`sherlock-processor-final-20261111/specs/sherlock-zero-knowledge/spec.md:4`): *"... No PCS masking SHALL be added while every STARK layer stays in case A."*
  - `DDD.md:304`: *"'Blinding slot' is the PRD-002 term and names no code object today (D-040)"*
- **Witness state**:
  1. Under `ZK-001` (owner pick `T11-3`), the STARK worker in Case A adds **no PCS masking** and holds **no blinding slots**; its proofs are deterministic functions of `(program, tape, r_P, input_salt)`.
  2. Under `ZK-017` and `D-040`, `wrapd serve` draws fresh `OsRng` blinders internally per prove call and exposes no client-addressable `slot_id` parameter, making `ZK-016`'s scenario (*"WHEN a client replays a slot id"*) untestable against the actual protocol interface.
- **What would close it**:
  - Rewrite `ZK-016` to reflect `T11-3` and `D-040`: remove *"or to the STARK worker"* in Case A, and state that `wrapd serve` draws fresh one-use `OsRng` blinders internally on each prove request, never reuses blinder randomness across requests, and rejects any client-supplied seed/blinder field outside `test-seed`.

### `Q-assurance-4` — Permission and seed-guard asymmetries across `ZKE-001`, `ZKE-002`, `ZK-012`, `ZK-013`, and `ZK-014`
- **IDs**: `ZKE-001`, `ZKE-002`, `ZKE-004`, `ZK-012`, `ZK-013`, `ZK-014`
- **Quoted sentences**:
  - `ZK-012`: *"When the prover prepares a deployment, the prover SHALL draw `r_P` once from the OS CSPRNG and store it in `deploy.secret` with mode 0600."* (Scenario: refuses group/world bits).
  - `ZK-013`: *"When the prover prepares a run, the prover SHALL draw a fresh input salt and store it in `run.secret`."*
  - `ZKE-001`: *"If a build without the cargo feature `test-seed` reaches a code path that sets `r_P` from a constant, then the chain driver SHALL exit with code 4 and write no proof."*
  - `ZK-014`: *"If a build without the cargo feature `test-seed` receives `SHERLOCK_TEST_SEED`, `GW_DET`, `GW_CHIP_SEED` or `GW_SEED`, then the wrap prover SHALL exit with code 4 and write no proof."*
  - `ZKE-002`: *"If the prover binary of a run was built with the cargo feature `test-seed`, then the settlement runner SHALL refuse to submit the proof of that run to Preview."*
- **Witness state**:
  1. `run.secret` written with mode `0644` satisfies `ZK-013` literally (unlike `ZK-012`, `ZK-013` omits `mode 0600` and permission refusal), contradicting `ZKE-004`'s scenario assumption (*"the files keep mode 0600"*).
  2. In a non-`test-seed` leaf/chain binary, passing `SHERLOCK_TEST_SEED` to seed the `run.secret` input salt bypasses `ZKE-001` (which guards only constant `r_P`) and `ZK-014` (which names only *"the wrap prover"*).
  3. In `ZKE-002`, if `WrapOutput` reaches the settlement runner without a run-record `features` field (e.g., standalone `wrapd prove`), a `test-seed` proof is not fail-closed unless missing `features` metadata is also refused.
- **What would close it**:
  - Mandate `mode 0600` write + load-time permission refusal for `run.secret` in `ZK-013`; enforce `ZKE-001` + `ZK-014` exit-`4` guards across **both** leaf/chain and wrap provers for `r_P`, tape salt, `input_d` salt, and wrap blinders; and require `ZKE-002` to fail closed if `features` metadata is absent or unauthenticated.

### `Q-assurance-5` — `ZK-013` omits the distinct witness-tape salt (`D-071`) and leaves retry/resume salt preservation unspecified
- **IDs**: `ZK-013` (`ZKE-005`)
- **Quoted sentences**:
  - `ZK-013`: *"When the prover prepares a run, the prover SHALL draw a fresh input salt and store it in `run.secret`."* / Scenario: *"WHEN two runs of one input are prepared THEN their salts differ and their `claim_pub` differ"*
  - `DDD.md:137, 170`: *"`input_d` commits to the whole tape (D-071 pick 2), and the tape salt and the `input_d` salt are distinct fields whose randomness policy slice B owns (ZK-013)"* ... *"whether a retry of one logical run keeps its salt, which CRR-005 needs across eras."*
- **Witness state**:
  - Drawing only a single `input_d` salt in `run.secret` leaves the Mycroft witness-tape salt un-drawn (or constant), while redrawing `run.secret` on crash-recovery retry changes `claim_pub` and breaks idempotence / cross-era retry (`CRR-005`).
- **What would close it**:
  - Specify in `ZK-013` that `prepare` draws two independent OS CSPRNG salts (`tape_salt` and `input_d_salt`) into `run.secret` (`mode 0600`), whereas `retry`/`resume` loads the existing `run.secret` read-only (refusing if missing, parallel to `ZKE-005`).

### `Q-assurance-6` — `ZKE-007` ("byte strings are pairwise not equal") disagrees with `ZK-015` ("every byte range of the proof differs"), and `SSG-005` omits non-`openvm` circuit changes covered by `LPC-016`
- **IDs**: `ZKE-007`, `ZK-015`, `SSG-005`, `LPC-004`, `LPC-016`
- **Quoted sentences**:
  - `ZKE-007`: *"... the wrap prover SHALL give four proofs whose byte strings are pairwise not equal."*
  - `ZK-015` Scenario: *"WHEN two proofs of one call vector are compared THEN every byte range of the proof differs and both verify"*
  - `SSG-005`: *"When a circuit change lands on a lane branch after openvm c4bd312, the tested-design loop SHALL add one row with the commit, the AIR and the Quint lane to the circuit change list."*
  - `LPC-016`: *"If a row reaches DONE without a lead verdict in `qa/gemini-quint`, then the tested-design loop SHALL count the row as not DONE for T15."*
- **Witness state**:
  1. Two serialized PLONK proofs sharing a 4-byte format/length header satisfy `ZKE-007` (`proof1 != proof2`) while violating `ZK-015` Scenario (*"every byte range of the proof differs"*); conversely, randomizing only a trailing 32-byte scalar while leaving earlier KZG commitments unblinded satisfies `ZKE-007` while leaking witness correlation!
  2. A circuit/constraint change in `tools/sherlock-wrap/` (PLONK wrap circuit) or `sherlock-ledger.compact` is not an `openvm` commit after `c4bd312`, so `SSG-005`'s `git log c4bd312..` check over `openvm` skips it even though `LPC-016` requires a `qa/gemini-quint` verdict for those rows (`T11`, `T17`, `T22`).
- **What would close it**:
  - Harmonize `ZKE-007` and `ZK-015` to assert pairwise distinctness on **every parsed commitment point and opening evaluation** of the wrap proof, and extend `SSG-005` / `LPC-004` to track wrap-circuit and Compact-contract changes alongside `openvm` AIR changes.

---

## 3. Mutant Table

Each of the 24 refusal/binding guards has a dedicated mutant module `assurance_m_<name>` (`MUTANT_ID = 1..24`). Under `quint run` (`--max-samples=1000 --max-steps=15 --seed=0x20261115`), all 24 mutants exited `1` (`[violation]`) on their target invariant, and all 24 also violated `reach_wrap_proved_and_settled_clean` (`exit 1`), confirming non-vacuity.

| Mutant Module | `MUTANT_ID` | Guard / Binding Removed or Weakened | Target Invariant | Observed Exit Code | Non-Vacuity (`reach_wrap_proved_and_settled_clean`) |
|---|---:|---|---|---:|---:|
| `assurance_m_zke001_const_rp_without_feature` | `1` | Allows constant `r_P` path in a build without `test-seed` | `inv_ZKE_001` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_zke002_settle_test_seed` | `2` | Allows `settleWrapProofAndScan` to settle a `test-seed` proof on Preview | `inv_ZKE_002` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_zke003_publish_secret_class` | `3` | Publishes `SecPlainProgD` in settled call output and anchor cell | `inv_ZKE_003` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_zke004_transfer_secret_outside_roster` | `4` | Permits transferring secret/witness/STARK files to `HostExternalHelper` | `inv_ZKE_004` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_zke005_redraw_rp_when_missing` | `5` | Prepares run on `odin` without `deploy.secret` by drawing a fresh `r_P = 9999` | `inv_ZKE_005` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_zke006_debug_prints_secret` | `6` | Emits `r_P` and salt words in `RunBlind` `Debug` log lines | `inv_ZKE_006` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_zke007_deterministic_wrap_blinder` | `7` | Uses deterministic blinder stream yielding identical proof bytes across calls | `inv_ZKE_007` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_ssg001_accept_low_d3_app_key` | `8` | Accepts a per-program R_SL app key with `99.0` / `100.5` bits (`< 100 / 101`) | `inv_SSG_001` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_ssg002_allow_plonk_in_stark_crate` | `9` | Passes T25 crate audit when `sherlock-lfr` depends on PLONK/KZG | `inv_SSG_002` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_ssg003_pass_gate_with_confirmed_cx` | `10` | Passes security gate while `unfixedConfirmedCxCount == 1` (`GP2-C2.1`) | `inv_SSG_003` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_ssg004_omit_not_attempted_rows` | `11` | Writes gate record omitting the `8` `NOT ATTEMPTED` counterexample rows | `inv_SSG_004` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_ssg005_missing_circuit_change_row` | `12` | Passes change-list audit with an unlisted circuit change lacking a Quint lane | `inv_SSG_005` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_ssg006_qualify_on_stale_g5` | `13` | Qualifies `T25` manifest while `G5` still maps `T10m` parameters (`100 != 250`) | `inv_SSG_006` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_ssg007_claim_qualified_before_gate` | `14` | Writes `SecQualified` into run records before the T12 security gate passes | `inv_SSG_007` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_sqg014_pass_g3_on_leg_mismatch` | `15` | Marks `G3` passed when only `4` of `5` transcript legs agree | `inv_SQG_014` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_sqg015_ignore_double_gate_fail` | `16` | Fails to stop row `T12` when `gateFailureStreak >= 2` | `inv_SQG_015` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_sqg016_pass_with_unverified_step` | `17` | Passes security gate while `unverifiedRunbookSteps == 2 > 0` | `inv_SQG_016` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_zk012_accept_bad_secret_mode` | `18` | Prepares run using group/world-readable `deploy.secret` (`mode 0644 != 0600`) | `inv_ZK_012` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_zk013_reuse_run_salt` | `19` | Reuses static `inputSalt = 4444` across distinct prepared runs of the same input | `inv_ZK_013` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_zk014_allow_gw_det_in_release` | `20` | Generates wrap proof in a non-`test-seed` build when `GW_DET` / env seed is set | `inv_ZK_014` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_zk016_allow_replayed_slot` | `21` | Removes one-use check on `usedBlindingSlots`, allowing slot replay | `inv_ZK_016` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_zk018_allow_case_b_unowned_job` | `22` | Admits proving/wrap job on unowned `HostExternalHelper` (Case B) | `inv_ZK_018` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_lpc016_count_done_without_verdict` | `23` | Counts a DONE loop row (`T17a`) as DONE for `T15` without a lead verdict | `inv_LPC_016` | `1` (`[violation]`) | `1` (live) |
| `assurance_m_cpr001_allow_wrap_k22` | `24` | Passes crate/circuit gate with BLS12-381 wrap circuit at `k = 22 > 21` | `inv_ADOPTED_ASSURANCE` | `1` (`[violation]`) | `1` (live) |

---

## 4. State Vocabulary for Replay

### 4.1 Top-Level State Variables (`var`)
In ITF JSON traces (`../traces/assurance_main/t_{seq}.itf.json`), Quint qualifies variables under `assurance_main::assurance::<var>`:

| Quint `var` | ITF JSON Key | Quint Type | Meaning |
|---|---|---|---|
| `assure` | `assurance_main::assurance::assure` | `AssuranceState` | Cohesive observable state of the Assurance bounded context (deployment secrets, prepared runs, wrap proofs, blinding slots, settled-call privacy scans, secret-file transfers, debug logs, per-program D3 records, crate/circuit bounds, T12 security gate & counterexample ledger, `CHANGES.md` rows, T15 loop verdicts, exit code, and latest refusal). |
| `lastOp` | `assurance_main::assurance::lastOp` | `str` | Action name that produced the state (`"init"` at index `0`, exact action identifier on every step). |

#### Fields of `assure: AssuranceState`
- `ownedRoster: Set[BoxHost]` — Owned machines (`HostMuninn`, `HostHuginn`, `HostOdin`; `HostExternalHelper` is unowned Case B) (`ZKE-004`, `ZK-018`, `ZK-001`).
- `activeManifestSha: int` (`250` = T25), `manifestStatus: KeyQualificationStatus` (`KeyPrototype | KeyQualified`) — Active universal keyset digest and qualification mark (`SSG-006`, `SSG-007`).
- `deployedRP: int` (`7001`), `boxDeploySecrets: BoxHost -> DeploymentSecretState` — Per-box `deploy.secret` presence, permission mode (`600` vs `644`), `rPValue`, `rPSource`, and `drawCount` (`ZKE-001`, `ZKE-005`, `ZK-012`).
- `nextRunSeq: int`, `preparedRuns: Set[PreparedRunRec]` — Prepared runs recording `hostBox`, `builtWithTestSeed`, `deploySecretWasPresent`, `deploySecretMode`, `usedRP`, `rPSource`, `inputSalt`, `claimPub`, `securityClaimInRecord` (`SecNotQualified | SecQualified`), and `gateWasPassedAtWrite` (`ZKE-001`, `ZKE-005`, `SSG-007`, `ZK-012`, `ZK-013`, `ZK-018`).
- `nextProofSeq: int`, `wrapProofs: Set[WrapProofRec]`, `usedBlindingSlots: Set[int]` — Generated wrap proofs on `WrapCpu` / `WrapGpu` with `hostBox`, `builtWithTestSeed`, `envSeedPassed`, `blindingSlotId`, `usedOsRng`, `lTranscriptInPublicOrConst`, `proofBytesDigest`, and `ledger9101Verified` (`ZKE-007`, `ZK-014..018`).
- `settledCalls: Set[SettledCallRec]` — Settled Preview calls scanned for secret classes (`leakedSecretClasses: Set[SecretClass]`), `lTranscriptPublic`, and `anchorHoldsPlainProgD` (`ZKE-002`, `ZKE-003`, `ZK-002`, `ZK-004`, `ZK-007`).
- `admittedSecretTransfers: Set[SecretTransferRec]`, `debugLogs: Set[DebugLogRec]` — Admitted secret transfers (`ZKE-004`) and `Debug` log lines (`ZKE-006`).
- `checkedAppKeys: Set[AppKeyD3Rec]` — Per-program R_SL app key D3 scores (`SSG-001`).
- `t25CrateGatePassed: bool`, `starkCratesHavePlonkOrKzg: bool`, `wrapKeyLogK: int`, `wrapKeyFixesLTranscript: bool` — Dependency and wrap `k` audit state (`SSG-002`, `DG-001`, `CPR-001`, `ZK-004`).
- `unfixedConfirmedCxCount: int`, `notAttemptedCxIds: Set[int]`, `unverifiedRunbookSteps: int`, `g3MatchingLegsCount: int`, `g3Passed: bool`, `g1g2g4Fp8LedgerPassed: bool`, `g5MappedManifestSha: int`, `gateFailureStreak: int`, `t12RowStopped: bool`, `gatePassed: bool`, `gateRecordWritten: bool`, `gateRecordManifestSha: int`, `gateRecordOpenCxIds: Set[int]` — T12 security gate & counterexample tracking (`SSG-003`, `SSG-004`, `SSG-006`, `SQG-014..016`, `SQG-001`).
- `changesListAuditPassed: bool`, `circuitChanges: Set[CircuitChangeRec]`, `loopRows: Set[LoopRowRec]` — Post-`c4bd312` `CHANGES.md` rows and `qa/gemini-quint` verdict gates (`SSG-005`, `LPC-004`, `LPC-016`).
- `lastExitCode: int`, `lastRefusal: AssuranceRefusal` — Latest CLI exit code (`0` or `4`) and refusal diagnostic.

### 4.2 `lastOp` Values and Implemented Requirements

| `lastOp` Value | Requirement ID(s) | Effect Summary |
|---|---|---|
| `"init"` | All (`ZKE`, `SSG`, `SQG`, `ZK`, `LPC`, adopted) | Initializes T25 `KeyPrototype` manifest, `0600` `deploy.secret` on `muninn`/`huginn` (absent on `odin`), 1 unfixed `CONFIRMED` cx, 8 `NOT ATTEMPTED` cx rows, 2 `UNVERIFIED` steps, and stale T10m `G5` mapping. |
| `"transferSecretFileToBox"` | `ZKE-004`, `ZK-012`, `ZK-001` | Moves a secret/witness/trace/STARK file between owned roster boxes preserving mode `0600`. |
| `"refuseSecretTransferOutsideRoster"` | `ZKE-004`, `ZK-018`, `ZK-001` | Refuses moving a secret file to `HostExternalHelper` and names host + file class. |
| `"stageWorldReadableDeploySecret"` | `ZK-012` | Precondition helper setting `0644` permissions on a box's `deploy.secret`. |
| `"refuseBadModeDeploySecret"` | `ZK-012` | Refuses a `deploy.secret` whose mode has group/world bits (`!= 600`). |
| `"refuseMissingDeploySecret"` | `ZKE-005` | Refuses run preparation when `deploy.secret` is absent on a box, drawing no new `r_P`. |
| `"refuseConstantRpExit4"` | `ZKE-001` | Exits with code `4` and writes no proof when a non-`test-seed` build reaches a constant `r_P` path. |
| `"refuseCaseBUnownedHost"` | `ZK-018`, `ZK-001` | Refuses a proving/wrap configuration targeting unowned `HostExternalHelper` (Case B). |
| `"prepareRunAndLog"` | `ZKE-001`, `ZKE-005`, `ZKE-006`, `SSG-007`, `ZK-012`, `ZK-013`, `ZK-018` | Loads `r_P` from `0600` `deploy.secret`, draws fresh `inputSalt` in `run.secret`, logs `RunBlind` with `0` secret words, and writes `SecNotQualified` before T12 passes. |
| `"refuseEnvSeedInReleaseWrapExit4"` | `ZK-014` | Exits with code `4` and writes no proof when a non-`test-seed` wrap build receives `GW_DET` / seed env vars. |
| `"refuseReplayedBlindingSlot"` | `ZK-016` | Refuses a replayed `blindingSlotId`. |
| `"makeWrapProof"` | `ZKE-007`, `ZK-014..018`, `ZK-004` | Produces a ledger-`9.1.0.1`-verified CPU or GPU wrap proof with fresh one-use `OsRng` blinders and distinct proof bytes. |
| `"refuseSettleTestSeedProof"` | `ZKE-002` | Refuses submitting a `test-seed` wrap proof to Preview settlement. |
| `"settleWrapProofAndScan"` | `ZKE-002`, `ZKE-003`, `ZK-002`, `ZK-004`, `ZK-007` | Settles a non-`test-seed` wrap proof and verifies 0 leaked secret classes in call vector, tx, and contract state. |
| `"checkAppKeyD3"` | `SSG-001`, `SQG-001` | Verifies per-program R_SL app key (`>= 100.0` bits all terms, `>= 101.0` bits grindable). |
| `"refuseLowD3AppKey"` | `SSG-001` | Refuses a per-program app key below the `100.0`/`101.0`-bit D3 threshold. |
| `"auditCratesAndWrapBoundsPass"` | `SSG-002`, `DG-001`, `CPR-001`, `ZK-004` | Verifies STARK crates hold no PLONK/KZG dependency and wrap `k in 19..21`. |
| `"refusePlonkInStarkOrWrapAboveK21"` | `SSG-002`, `DG-001`, `CPR-001` | Fails T25 audit on PLONK/KZG dependency in STARK crates or wrap `k >= 22`. |
| `"refuseWrapKeyFixingLTranscript"` | `ZK-004` | Refuses a wrap key that fixes an `L`/`L_Fr` transcript challenge, query position, or `X` as public/constant. |
| `"runG3TranscriptCheck"` | `SQG-014`, `SQG-015` | Compares the 5 `G3` transcript legs (`< 5` fails `G3` and increments failure streak). |
| `"completeT12ChecksAndFixes"` | `SQG-015`, `SSG-003`, `SQG-016`, `SSG-006` | Fixes open `CONFIRMED` counterexamples, verifies runbook steps, runs `G1/G2/G3/G4/FP-8`/ledger `9.1.0.1`, and updates `G5` mapping to `T25`. |
| `"recordGateCheckFailure"` | `SQG-015` | Increments `gateFailureStreak` and stops row `T12` upon `2` failures (AFK rule). |
| `"refuseSecurityGateEvaluation"` | `SSG-003`, `SSG-006`, `SQG-016` | Reports security gate not passed due to `CONFIRMED` defect, stale `G5` mapping, or `UNVERIFIED` step. |
| `"writePassingGateRecordAndQualify"` | `SSG-004`, `SSG-006`, `SSG-007`, `SQG-001` | Writes passing gate record (listing `NOT ATTEMPTED` open IDs + `activeManifestSha`) and promotes manifest to `KeyQualified`. |
| `"landAndAuditCircuitChange"` | `SSG-005`, `LPC-004` | Records post-`c4bd312` circuit change in `CHANGES.md` with Carbon Quint lane and lead verdict. |
| `"evaluateLoopRowForT15"` | `LPC-016` | Evaluates `T15` readiness of loop rows, counting DONE only when `hasLeadVerdict == true`. |

### 4.3 Instance Constants

| Constant | `assurance_main` | `assurance_edge_min` | `assurance_edge_at_limit` | `assurance_m_*` | Meaning |
|---|---:|---:|---:|---:|---|
| `EXIT_CODE_SEED_REFUSED` | `4` | `4` | `4` | `4` | Process exit code on seed-guard refusal (`ZKE-001`, `ZK-014`) |
| `SECRET_FILE_MODE` | `600` | `600` | `600` | `600` | Required octal mode `0600` for secret files (`ZK-012`, `ZKE-004`) |
| `MIN_D3_TERM_DECIBITS` | `1000` | `1000` | `1000` | `1000` | Non-grindable D3 floor `100.0` bits (`SSG-001`, `SQG-001`) |
| `MIN_D3_GRINDABLE_DECIBITS` | `1010` | `1010` | `1010` | `1010` | Grindable D3 floor `101.0` bits (`SSG-001` Scenario) |
| `MIN_WRAP_K` | `19` | `19` | `19` | `19` | Minimum wrap log-degree `k = 19` (`CPR-001`) |
| `MAX_WRAP_K` | `21` | `21` | `21` | `21` | Maximum wrap log-degree `k = 21` (`DG-001`, `CPR-001`) |
| `G3_TRANSCRIPT_LEGS` | `5` | `5` | `5` | `5` | Transcript legs compared in `G3` (`SQG-014`) |
| `MAX_GATE_FAILURES_AFK` | `2` | `2` | `2` | `2` | Consecutive gate failures stopping row `T12` (`SQG-015`) |
| `NOT_ATTEMPTED_INIT_COUNT` | `8` | `1` | `8` | `8` | Initial `NOT ATTEMPTED` rows in `COUNTEREXAMPLES.md` (`SSG-004`) |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 24` | `0` = sound; `1..24` = single-guard/binding mutant |

---

## 5. Model Abstractions

1. **CSPRNG Words, Salts, and Proof Byte Strings**: `256`-bit `r_P` secrets (`ZK-012`), `run.secret` input salts (`ZK-013`), `claim_pub` commitments, and PLONK wrap proof byte strings (`ZKE-007`, `ZK-015`) are represented as injective integers (`8000 + seqN`, `20000 + 100 * stmtId + 10 * pSeq + devTag`) so distinct CSPRNG draws map injectively to distinct digests while constant/reused seeds collide deterministically.
2. **D3 Fractional Bit Counts**: D3 security margins (`100` bits, `101` bits, `99.5` / `100.5` bits) are represented in integer tenths of a bit (`deci-bits`: `1000`, `1010`, `995`, `1005`).
3. **Privacy & Debug Scans (`ZKE-003`, `ZKE-006`)**: Byte-level scanning of transactions, call vectors, ledger state, and `{:?}` log strings is abstracted to set membership over the 5 named secret classes (`SecWitnessWord`, `SecStarkProofBytes`, `SecDeployRP`, `SecInputSalt`, `SecPlainProgD`) and boolean leak flags on `DebugLogRec`.
4. **Static Lean 4 Kernel Axiom Check (`ZK-019`)**: Because `ZK-019` (`#print axioms` on `pv2_inj` and `pv2_count_det`) has no runtime state transition, it is classified as `SPEC-CHECKED candidate` per `PROMPT.md` item 2 and paired with the pure consistency check `consistency_zk019`.
5. **Bounded Run & Proof Sequences**: `nextRunSeq <= 6` and `nextProofSeq <= 8` bound set growth during random 15-step walks while accommodating the 4-proof CPU+GPU `ZKE-007` test plus a 5th `test-seed` refusal proof.

---

## 6. Commands

Run every command below from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/assurance/quint`:

```bash
# 1. Typecheck model and test suite
quint typecheck assurance.qnt
# Expected exit code: 0
# Printed line: (no output on success)

quint typecheck assurance_test.qnt
# Expected exit code: 0
# Printed line: (no output on success)

# 2. Requirement scenario unit tests (8 tests)
quint test --main=assurance_test --match='as_.*Test' --seed=0x20261115 assurance_test.qnt
# Expected exit code: 0
# Printed line: 8 passing (2786ms)

# 3. Main and edge instance safety invariant checks
quint run --main=assurance_main --invariant=inv --max-samples=2000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 0
# Printed line: [ok] No violation found (2095ms at 955 traces/second).

quint run --main=assurance_edge_min --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 0
# Printed line: [ok] No violation found (1301ms at 769 traces/second).

quint run --main=assurance_edge_at_limit --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 0
# Printed line: [ok] No violation found (1532ms at 653 traces/second).

# 4. Non-vacuity reachability witnesses on main instance (negated goals: all MUST exit 1)
quint run --main=assurance_main --invariant=reach_secret_transferred_and_odin_prepared --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (359ms at 226 traces/second).

quint run --main=assurance_main --invariant=reach_wrap_proved_and_settled_clean --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (244ms at 20 traces/second).

quint run --main=assurance_main --invariant=reach_two_runs_distinct_salts_and_claims --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (232ms at 4 traces/second).

quint run --main=assurance_main --invariant=reach_t12_gate_qualified_with_open_cx_listed --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (486ms at 10 traces/second).

quint run --main=assurance_main --invariant=reach_t12_stopped_on_double_failure --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (380ms at 3 traces/second).

quint run --main=assurance_main --invariant=reach_app_d3_and_crate_audit_and_verdict --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (267ms at 45 traces/second).

quint run --main=assurance_main --invariant=reach_exit4_and_gate_refusals --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (295ms at 3 traces/second).

# 5. Guard and binding mutants (all 24 MUST exit 1)
quint run --main=assurance_m_zke001_const_rp_without_feature --invariant=inv_ZKE_001 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (335ms at 6 traces/second).

quint run --main=assurance_m_zke002_settle_test_seed --invariant=inv_ZKE_002 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (330ms at 3 traces/second).

quint run --main=assurance_m_zke003_publish_secret_class --invariant=inv_ZKE_003 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (503ms at 10 traces/second).

quint run --main=assurance_m_zke004_transfer_secret_outside_roster --invariant=inv_ZKE_004 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (201ms at 5 traces/second).

quint run --main=assurance_m_zke005_redraw_rp_when_missing --invariant=inv_ZKE_005 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (223ms at 4 traces/second).

quint run --main=assurance_m_zke006_debug_prints_secret --invariant=inv_ZKE_006 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (390ms at 3 traces/second).

quint run --main=assurance_m_zke007_deterministic_wrap_blinder --invariant=inv_ZKE_007 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (256ms at 4 traces/second).

quint run --main=assurance_m_ssg001_accept_low_d3_app_key --invariant=inv_SSG_001 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (420ms at 10 traces/second).

quint run --main=assurance_m_ssg002_allow_plonk_in_stark_crate --invariant=inv_SSG_002 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (473ms at 4 traces/second).

quint run --main=assurance_m_ssg003_pass_gate_with_confirmed_cx --invariant=inv_SSG_003 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (293ms at 17 traces/second).

quint run --main=assurance_m_ssg004_omit_not_attempted_rows --invariant=inv_SSG_004 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (284ms at 18 traces/second).

quint run --main=assurance_m_ssg005_missing_circuit_change_row --invariant=inv_SSG_005 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (297ms at 3 traces/second).

quint run --main=assurance_m_ssg006_qualify_on_stale_g5 --invariant=inv_SSG_006 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (247ms at 20 traces/second).

quint run --main=assurance_m_ssg007_claim_qualified_before_gate --invariant=inv_SSG_007 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (226ms at 4 traces/second).

quint run --main=assurance_m_sqg014_pass_g3_on_leg_mismatch --invariant=inv_SQG_014 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (274ms at 15 traces/second).

quint run --main=assurance_m_sqg015_ignore_double_gate_fail --invariant=inv_SQG_015 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (209ms at 5 traces/second).

quint run --main=assurance_m_sqg016_pass_with_unverified_step --invariant=inv_SQG_016 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (292ms at 17 traces/second).

quint run --main=assurance_m_zk012_accept_bad_secret_mode --invariant=inv_ZK_012 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (290ms at 14 traces/second).

quint run --main=assurance_m_zk013_reuse_run_salt --invariant=inv_ZK_013 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (199ms at 5 traces/second).

quint run --main=assurance_m_zk014_allow_gw_det_in_release --invariant=inv_ZK_014 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (321ms at 9 traces/second).

quint run --main=assurance_m_zk016_allow_replayed_slot --invariant=inv_ZK_016 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (403ms at 32 traces/second).

quint run --main=assurance_m_zk018_allow_case_b_unowned_job --invariant=inv_ZK_018 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (352ms at 3 traces/second).

quint run --main=assurance_m_lpc016_count_done_without_verdict --invariant=inv_LPC_016 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (242ms at 4 traces/second).

quint run --main=assurance_m_cpr001_allow_wrap_k22 --invariant=inv_ADOPTED_ASSURANCE --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 assurance.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (353ms at 6 traces/second).

# 6. Generate 20 ITF replay traces
quint run assurance.qnt --main assurance_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/assurance/traces/assurance_main/t_{seq}.itf.json'
# Expected exit code: 0
# Printed line: [ok] No violation found (626ms at 32 traces/second).
```
