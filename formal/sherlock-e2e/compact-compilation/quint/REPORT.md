# Quint Formal Specification Report — Bounded Context: Compact Compilation (`e2e-compact-compilation`)

- **Specification**: [compact_compilation.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/compact-compilation/quint/compact_compilation.qnt)
- **Scenario Tests**: [compact_compilation_test.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/compact-compilation/quint/compact_compilation_test.qnt)
- **ITF Replay Traces (`20` traces)**: [traces/compact_compilation_main/](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/compact-compilation/traces/compact_compilation_main) (`t_0.itf.json` .. `t_19.itf.json`)

---

## 1. Requirements Coverage Table

Every own EARS ID (`12` IDs from `sherlock-e2e-prototype-20261115`: `ECC-001..005`, `ECC-007`, `ECC-009..014`) and completion EARS ID (`4` IDs from `sherlock-completion-20261115`: `CF-013`, `CF-014`, `LPC-013`, `LPC-019`) assigned to this lane appears below (`16` rows), followed by adopted `CF-001`, `CF-002`, `CF-004`, and `CF-012` (`sherlock-processor-final-20261111`).

| ID | Artifact (`inv_*`, action + test, consistency `val`) | Kind | Note |
|---|---|---|---|
| `ECC-001` | `inv_ECC_001`, `consistency_constants`, `doCompileSingleCircuit`, `cc_counterKeccakJubjubAndSecp256k1CompileTest` | CONSTANT-CHECKED | Requires Stage 1 compilation to read only stock `compactc 0.36.0` `ZKIR 3.1` (`ZKIR_SUPPORTED_VERSION == 31`) + `contract-info.json` with `not(usedPatchedCompactcPass)`. Killed by `compact_compilation_m_ecc001_patched_or_wrong_zkir`. |
| `ECC-002` | `inv_ECC_002`, `doRunFactoryGatesAndOracle`, `cc_factoryGatesOracleReplayPremiseS1CfL1AndSpecGapsTest` | MODELED | Blocks marking the compiler usable for settled calls (`usableForSettledCall`) unless `gatesShPassed` holds and zero unargued compiler mutants survive (`not(hasUnarguedSurvivingMutant)`). Killed by `compact_compilation_m_ecc002_settle_surviving_mutant`. |
| `ECC-003` | `inv_ECC_003`, `consistency_constants`, `doCompileSingleCircuit`, `cc_counterKeccakJubjubAndSecp256k1CompileTest` | CONSTANT-CHECKED | Requires 1 emitted image and 1 emitted ABI per accepted proof circuit with matching `selector`, `abiTapeLen == TAPE_HEADER_WORDS (3) + slotsCount`, and `abiLE == codeEffectWords`. Killed by `compact_compilation_m_ecc003_bad_abi_layout`. ABI field discrepancy with `CF-002` reported in `Q-compact-compilation-1`. |
| `ECC-004` | `inv_ECC_004`, `doCheckWitnessInvarianceAndStepBound`, `cc_witnessInvarianceStaticStepCountAndPlanWithoutWitnessTest` | MODELED | Requires serialized image SHA-256 (`imageShaTape1 == imageShaTape2`) and ABI SHA-256 (`abiShaTape1 == abiShaTape2`) to be equal across distinct witness tapes of one entry. Killed by `compact_compilation_m_ecc004_witness_dependent_bytes`. |
| `ECC-005` | `inv_ECC_005`, `SINSTR_SHAPES`, `consistency_constants`, `doCompileSingleCircuit`, `cc_counterKeccakJubjubAndSecp256k1CompileTest` | CONSTANT-CHECKED | Restricts emitted instructions to the `SINSTR_SHAPE_COUNT == 13` `SInstr` shapes (`<= RSL_RULE_COUNT == 16`) with `not(universalKeysChanged)`. Killed by `compact_compilation_m_ecc005_illegal_sinstr_shape`. |
| `ECC-007` | `inv_ECC_007`, `doCompileSingleCircuit`, `cc_counterKeccakJubjubAndSecp256k1CompileTest` | MODELED | Requires `keccak256` circuits (`CircKeccakValid`) to lower to one `Chip` instruction of `R_SL_KECCAK256` (`keccakUsesChipRule and not(keccakUsedTier3OrRefused)`). Killed by `compact_compilation_m_ecc007_keccak_tier3_or_refuse`. |
| `ECC-009` | `inv_ECC_009`, `consistency_constants`, `gap_q_cc_4_slot15_passes_spike_fails_deploy`, `doCompileSingleCircuit`, `cc_counterKeccakJubjubAndSecp256k1CompileTest`, `cc_orderedRefusalPipelineAndUnknownCurve25519Test` | CONSTANT-CHECKED | Requires `anchorSlotUsed == DEPLOY_ANCHOR_SLOT (15)` in deployable profiles, zero spike constants (`not(hasSpikeConstants)`), no user `W` access at root key `15`, and `MYC-SEAL-021/seal` refusal when user ledger state occupies slot `15`. Killed by `compact_compilation_m_ecc009_spike_slot_or_collision`. Normative refusal omission reported in `Q-compact-compilation-4`. |
| `ECC-010` | `inv_ECC_010`, `computeFirstFailingStage`, `doCompileSingleCircuit`, `cc_orderedRefusalPipelineAndUnknownCurve25519Test` | MODELED | Enforces `unsupportedCount == 0` and strict 5-stage refusal precedence (`1: read -> 2: irVersion -> 3: inputTypes -> 4: lowering -> 5: seal`). Killed by `compact_compilation_m_ecc010_wrong_refusal_order`. |
| `ECC-011` | `inv_ECC_011`, `doRunFactoryGatesAndOracle`, `cc_factoryGatesOracleReplayPremiseS1CfL1AndSpecGapsTest` | MODELED | Requires any golden-vector divergence between the Mycroft compiler and Lean `OracleMain` to fail `oracleGatePassed` and `gatesShPassed` while naming the vector (`namedDiffVector != "none"`). Killed by `compact_compilation_m_ecc011_ignore_oracle_diff`. |
| `ECC-012` | `inv_ECC_012`, `consistency_constants`, `doRunCompileCliBatch`, `cc_compileCliBatchAdmitAndAllOrNothingRefusalTest` | CONSTANT-CHECKED | When all `BATCH_CIRCUIT_COUNT` requested proof circuits are admitted, writes `1` image file and `1` ABI file per circuit and exits with code `0`. Killed by `compact_compilation_m_ecc012_missing_abi_or_nonzero`. |
| `ECC-013` | `inv_ECC_013`, `gap_q_cc_3_ecc013_abi_omission_and_multi_refusal_order`, `doRunCompileCliBatch`, `cc_compileCliBatchAdmitAndAllOrNothingRefusalTest` | MODELED | When any requested circuit in `sherlock compile` is refused, exits non-zero, prints `printedRefusalCode`, and writes `0` image and `0` ABI files (`writtenImageCount == 0 and writtenAbiCount == 0`). Killed by `compact_compilation_m_ecc013_partial_output_on_refusal`. Partial-ABI and multi-refusal gaps reported in `Q-compact-compilation-3`. |
| `ECC-014` | `inv_ECC_014`, `consistency_constants`, `doCompileSingleCircuit`, `cc_counterKeccakJubjubAndSecp256k1CompileTest` | CONSTANT-CHECKED | Requires `Point<Jubjub>` / `Scalar<Jubjub>` circuits (`CircJubjubValid`) to read `JUBJUB_POINT_WORDS == 2` raw tape words for points and enforce on-curve + subgroup checks (`jubjubCurveAndSubgroupChecked`). Killed by `compact_compilation_m_ecc014_jubjub_missing_curve_check`. |
| `CF-013` | `inv_CF_013`, `doCompileSingleCircuit`, `cc_counterKeccakJubjubAndSecp256k1CompileTest` | MODELED | Requires `Scalar<Secp256k1>` and `Point<Secp256k1>` inputs/values (`CircSecp256k1Valid`) to lower to the `Secp256k1` `R_SL` kit (`secp256k1LoweredToKit`) without `InputType` refusal. Killed by `compact_compilation_m_cf013_refuse_secp256k1_input`. |
| `CF-014` | `inv_CF_014`, `doCompileSingleCircuit`, `cc_orderedRefusalPipelineAndUnknownCurve25519Test` | MODELED | Refuses unknown ZKIR input types lacking an `R_SL` kit (`CircUnknownCurve25519`) at `STAGE_INPUT_TYPES` (`3`) naming `"InputType:Scalar<Curve25519>"`. Killed by `compact_compilation_m_cf014_admit_unknown_curve25519`. |
| `LPC-013` | `inv_LPC_013`, `gap_q_cc_2_sorries_vs_wfail_and_cf012_axioms`, `doRunFactoryGatesAndOracle`, `cc_factoryGatesOracleReplayPremiseS1CfL1AndSpecGapsTest` | MODELED | Enforces `gates.sh` pass conditions and `lakeBuildWfailPassed iff (sorriesRemainingCount == 0)`. Interaction with open `SORRIES.md` items reported in `Q-compact-compilation-2`. |
| `LPC-019` | `inv_LPC_019`, `doEvaluatePremiseAndKitClosure`, `cc_factoryGatesOracleReplayPremiseS1CfL1AndSpecGapsTest` | MODELED | Requires Mycroft design closure (`mycroftDesignClosed`) to either discharge premise `S1` (`s1Discharged`) or keep `S1` as a named hypothesis documented in the report (`s1KeptAsNamedHypothesis and s1NamedInReport`). Killed by `compact_compilation_m_lpc019_close_with_unnamed_s1`. |

### Adopted IDs Modeled in `compact_compilation.qnt`

| ID | Owning Change | Artifact | Kind | Note |
|---|---|---|---|---|
| `CF-001` | `sherlock-processor-final-20261111` | `inv_CF_001`, `doCompileSingleCircuit`, `cc_counterKeccakJubjubAndSecp256k1CompileTest` | MODELED | Enforces `isPrototypeBuild -> not(usesLflattenedTap)` (Stage 1 ZKIR `3.1` per `ECC-001`) and `not(isPrototypeBuild) -> usesLflattenedTap` (Stage 2 `Lflattened` tap). |
| `CF-002` | `sherlock-processor-final-20261111` | `inv_CF_002`, `CF002_FRONTEND_COMMANDS`, `consistency_constants`, `gap_q_cc_1_ecc003_vs_cf002_abi_fields`, `doCheckWitnessInvarianceAndStepBound`, `cc_witnessInvarianceStaticStepCountAndPlanWithoutWitnessTest` | CONSTANT-CHECKED | Models witness-free `sherlock plan` (`planReadNoWitness`) and the `6`-command frontend surface; ABI field mismatch against `ECC-003` witnessed in `gap_q_cc_1_ecc003_vs_cf002_abi_fields` (`Q-compact-compilation-1`). |
| `CF-004` | `sherlock-processor-final-20261111` | `inv_CF_004`, `consistency_constants`, `doCheckWitnessInvarianceAndStepBound`, `cc_witnessInvarianceStaticStepCountAndPlanWithoutWitnessTest` | CONSTANT-CHECKED | Requires static step count `stepCountTape1 == stepCountTape2` across tapes and `spendStepCount <= MAX_SPEND_STEPS (550)`. Killed by `compact_compilation_m_cf004_tape_dependent_steps`. |
| `CF-012` | `sherlock-processor-final-20261111` | `inv_CF_012`, `doEvaluatePremiseAndKitClosure`, `cc_factoryGatesOracleReplayPremiseS1CfL1AndSpecGapsTest` | MODELED | Blocks reporting the Compact Frontend kit complete (`cfKitReportedComplete`) unless `cfL1Proved` and `onlyStandardLeanAxioms` (`propext`, `Quot.sound`, `Classical.choice`) hold. |

---

## 2. Findings

### `Q-compact-compilation-1`: `ECC-003`'s Emitted ABI Schema Omits Per-Builtin Counts Required by Adopted `CF-002` (and `CF-002`'s Six `sherlock <verb>` Commands Do Not Exist)

- **IDs**: `ECC-003`, `ECC-012`, `CF-002`
- **Quoted EARS Sentences**:
  - `ECC-003`: *"ECC-003: When the Mycroft compiler accepts a proof circuit, the Mycroft compiler SHALL emit one program image with `selector`, `n_out` and `code`, together with one ABI with `selector`, `tape_len`, `slots`, `l_e`, `n_e` and `kits`."*
  - `CF-002`: *"The frontend SHALL provide `sherlock compile`, `sherlock plan`, `sherlock simulate`, `sherlock deploy`, `sherlock prove` and `sherlock call`. `abi.json` SHALL give per entry the step count `N_e`, the effect words `L_e`, the tape length and the count per builtin."*
  - `ECC-012`: *"ECC-012: When `sherlock compile` admits every requested proof circuit of a `compactc` build directory, the Mycroft compiler SHALL write one image file and one ABI file per proof circuit and exit with code 0."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_cc_1_ecc003_vs_cf002_abi_fields` (`compact_compilation.qnt:214-218`): `ECC-003`'s 6-field ABI schema `Set("selector", "tape_len", "slots", "l_e", "n_e", "kits")` records only coarse kit membership (`kits`), omitting `CF-002`'s per-builtin invocation count (`builtin_counts`), so `sherlock plan` (`CF-002` Scenario) cannot read per-builtin chip heights from `abi.json` alone without re-scanning `code`. In addition, `CF-002` and `ECC-012` mandate `sherlock <verb>` subcommands while `ECC-012` Status (`spec.md:130-131`) states that no `sherlock` binary exists (deferred to post-`T15` by `D-085`).
- **Closing Fix**:
  - Either add `builtin_counts` to `ECC-003`'s ABI schema or amend `CF-002` (like `CF-001` was amended for `D-016`) to state that in the prototype `kits` + `code` supersede `builtin_counts` in `abi.json` and standalone binaries supersede the unified `sherlock` CLI until after `T15`.

### `Q-compact-compilation-2`: `LPC-013`'s `lake build --wfail` Gate and `CF-012`'s Standard-Axioms-Only Rule Conflict With Open `SORRIES.md` Premises in `ECC-002` and Partial `S1`-Only Closure in `LPC-019`

- **IDs**: `ECC-002`, `LPC-013`, `LPC-019`, `CF-012`
- **Quoted EARS Sentences**:
  - `ECC-002`: *"ECC-002: The Mycroft compiler SHALL pass every gate of `mycroft/factory/gates.sh` on the MYC requirements of the modules frontend, values, lowering, witness-tape, ledger-ops, builtins and refusals."* (Status `line 27`: *"premises S1 and S2, the builtin parameters, wide arguments and the typed secp256k1 layout stay open in `mycroft/lean/SORRIES.md:13-14`, `:83`, and CF-L1 is open (CF-012)."*)
  - `LPC-013`: *"LPC-013: When the T18 (d) deliverables are committed, the Mycroft factory SHALL pass `gates.sh` and `lake build --wfail`."*
  - `LPC-019`: *"LPC-019: When the Mycroft design closes, the Mycroft Lean library SHALL discharge premise S1 or keep it as a named hypothesis."*
  - `CF-012`: *"The kit SHALL be reported complete only when CF-L1 is proved with an axiom report that holds only `propext`, `Quot.sound` and `Classical.choice`."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_cc_2_sorries_vs_wfail_and_cf012_axioms` (`compact_compilation.qnt:224-229`) and `cc_factoryGatesOracleReplayPremiseS1CfL1AndSpecGapsTest`:
    1. Every Lean declaration containing `sorry` emits `declaration uses 'sorry'`, which fails `lake build --wfail` (`LPC-013`). Even after `LPC-019` converts `S1` into a theorem parameter (`s1AsNamedHypothesis == true`), the remaining `4` open items listed in `ECC-002` Status (`S2`, builtin parameters, wide arguments, typed secp256k1 layout; `remainingSorriesAfterS1 == 4`) still fail `lake build --wfail`.
    2. Replacing those `sorry` tokens with custom Lean `axiom` declarations to silence `--wfail` violates `CF-012`'s requirement that the axiom report contain only `propext`, `Quot.sound`, and `Classical.choice`.
- **Closing Fix**:
  - Extend `LPC-019` to require **every** open premise in `mycroft/lean/SORRIES.md` (`S1`, `S2`, builtin parameters, wide arguments, typed secp256k1 layout) to be either proved or threaded as explicit theorem-level `Hypothesis` parameters (zero `sorry` and zero custom `axiom` declarations), so `lake build --wfail` (`LPC-013`) and `CF-012`'s axiom check agree.

### `Q-compact-compilation-3`: Normative `ECC-013` Forbids Writing Image Files on Refusal But Omits Partial ABI Files (`ECC-012`) and Cross-Circuit Refusal Ordering (`ECC-010`)

- **IDs**: `ECC-013`, `ECC-012`, `ECC-010`
- **Quoted EARS Sentences**:
  - `ECC-012`: *"ECC-012: When `sherlock compile` admits every requested proof circuit of a `compactc` build directory, the Mycroft compiler SHALL write one image file and one ABI file per proof circuit and exit with code 0."*
  - `ECC-013`: *"ECC-013: If the Mycroft compiler refuses one requested circuit, then the compile command SHALL exit with a non-zero code, print the refusal code of `refusal_code` and write no image file for the request."*
  - `ECC-010`: *"ECC-010: When `mycroft-suite` runs on the 33 compiled suite programs, the Mycroft compiler SHALL give each proof circuit either an image or the refusal code of its first failing check in the order read, IR version, input types, lowering and seal."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_cc_3_ecc013_abi_omission_and_multi_refusal_order` (`compact_compilation.qnt:234-239`):
    1. **Partial ABI files**: When circuit `1` succeeds and circuit `2` fails in a `sherlock compile` batch, writing circuit `1`'s `.abi.json` while deleting/withholding images (`writtenImages == 0, writtenPartialAbis == 1`) satisfies literal `ECC-013` (*"write no image file for the request"*), contradicting `ECC-013` Verification (*"an empty output directory"*) and `Status` (*"No partial output"*). (`inv_ECC_013` enforces both `writtenImageCount == 0` and `writtenAbiCount == 0`.)
    2. **Cross-circuit refusal determinism**: `ECC-010` defines check precedence *within* a single circuit (`read -> irVersion -> inputTypes -> lowering -> seal`), not *across* circuits in a build directory. When two requested circuits in one `sherlock compile` batch fail (`"MYC-REF-032/irVersion"` and `"InputType:Scalar<Curve25519>"`), `ECC-013`'s singular *"print the refusal code"* is ambiguous unless circuit evaluation order is fixed.
- **Closing Fix**:
  - Change `ECC-013` to *"write no image file and no ABI file for the request"*, and specify deterministic circuit evaluation order (e.g., `contract-info.json` declaration order or UTF-8 lexicographic circuit-name order) when multiple circuits in a request fail.

### `Q-compact-compilation-4`: Normative `ECC-009` Omits Refusing User Ledger Fields at Slot `15` (`MYC-SEAL-021`) and Leaves Suite vs Deploy Profile Divergence Under `ECC-010`

- **IDs**: `ECC-009`, `ECC-010`, `ECC-011`
- **Quoted EARS Sentences**:
  - `ECC-009`: *"ECC-009: Where the Mycroft compiler builds a deployable contract, the Mycroft compiler SHALL use top-level slot 15 as `anchor_slot`."*
  - `ECC-009` Verification & Status: *"Verification: test T-SLOT15 (compile t02 `increment` in the deploy profile, W addresses no root key 15, and a contract with a field at slot 15 is refused with the MYC-SEAL-021 code); to build."* / *"The spike driver has `SPIKE_SLOT = 0` (`mycroft/compiler/src/main.rs:30`), and the refusal code is an open item (`REPORT.md:103-115`)."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_cc_4_slot15_passes_spike_fails_deploy` (`compact_compilation.qnt:244-248`) and mutant `compact_compilation_m_ecc009_spike_slot_or_collision`:
    1. Normative `ECC-009` mandates setting `anchor_slot = 15` in a deployable build, without normatively stating that a contract whose own state occupies top-level slot `15` (`userUsesSlot15 == true`) **must be refused** so its ledger state cannot alias or overwrite the Sherlock anchor cell.
    2. Because `ECC-009` is gated by `Where the Mycroft compiler builds a deployable contract`, non-deployable `mycroft-suite` / golden runs (`nonDeployStage == STAGE_NONE`) can accept a 16-field contract (or collide at `SPIKE_SLOT = 0`) while deployable compilation (`deployStage == STAGE_SEAL`) refuses it.
- **Closing Fix**:
  - Add an explicit unwanted-behavior clause to `ECC-009` (*"If a contract declares a top-level ledger field at slot 15, then the Mycroft compiler SHALL refuse the circuit at the seal check with code `MYC-SEAL-021`"*), and run `mycroft-suite` (`ECC-010`) under the `anchor_slot = 15` deploy profile rather than `SPIKE_SLOT = 0`.

---

## 3. Mutant Table

All `16` guard mutants (`MUTANT_ID = 1..16`) were executed with `quint run compact_compilation.qnt --main <mutant_module> --invariant <target_inv> --max-samples 1000 --max-steps 15 --seed 0x20261115`. Every mutant was killed (`exit code 1`, `[violation] Found an issue`), and every mutant was verified **non-vacuous** by confirming `reach_witness_invariance_and_factory_gates_pass` is still violated (`exit code 1`) on that mutant module.

| Mutant Module | `MUTANT_ID` | Invariant It Must Break | Exit Code Seen | Non-Vacuous (`reach_witness_invariance_and_factory_gates_pass`) |
|---|---:|---|---:|---|
| `compact_compilation_m_ecc001_patched_or_wrong_zkir` | `1` | `inv_ECC_001` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_ecc002_settle_surviving_mutant` | `2` | `inv_ECC_002` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_ecc003_bad_abi_layout` | `3` | `inv_ECC_003` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_ecc004_witness_dependent_bytes` | `4` | `inv_ECC_004` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_ecc005_illegal_sinstr_shape` | `5` | `inv_ECC_005` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_ecc007_keccak_tier3_or_refuse` | `6` | `inv_ECC_007` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_ecc009_spike_slot_or_collision` | `7` | `inv_ECC_009` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_ecc010_wrong_refusal_order` | `8` | `inv_ECC_010` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_ecc011_ignore_oracle_diff` | `9` | `inv_ECC_011` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_ecc012_missing_abi_or_nonzero` | `10` | `inv_ECC_012` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_ecc013_partial_output_on_refusal` | `11` | `inv_ECC_013` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_ecc014_jubjub_missing_curve_check` | `12` | `inv_ECC_014` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_cf013_refuse_secp256k1_input` | `13` | `inv_CF_013` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_cf014_admit_unknown_curve25519` | `14` | `inv_CF_014` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_cf004_tape_dependent_steps` | `15` | `inv_CF_004` | `1` | Yes (`exit 1`) |
| `compact_compilation_m_lpc019_close_with_unnamed_s1` | `16` | `inv_LPC_019` | `1` | Yes (`exit 1`) |

---

## 4. State Vocabulary for Replay

### 4.1 Top-Level State Variables (`var`) and ITF Keys

| Quint `var` | Exact ITF JSON Key in `t_{0..19}.itf.json` | Quint Type | Meaning |
|---|---|---|---|
| `cc` | `compact_compilation_main::compact_compilation::cc` | `CompilerState` | Full observable state of the Compact Compilation bounded context. |
| `lastOp` | `compact_compilation_main::compact_compilation::lastOp` | `str` | Action name written by `init` (`"init"`) and every step to drive oracle replay. |

### 4.2 Sum Types and Nested Record Types

- `CircuitCase`: `CircCounterValid | CircKeccakValid | CircJubjubValid | CircSecp256k1Valid | CircUnknownCurve25519 | CircMultiFailIrAndSeal | CircSlot15Collision`.
- `SingleCompileState` (`cc.lastCompile`):
  - `evaluated: bool` — True once a single-circuit compile action has executed.
  - `circuitCase: CircuitCase` — Which representative circuit fixture was compiled.
  - `isPrototypeBuild: bool` — True for the Stage 1 ZKIR `3.1` prototype (`ECC-001`, `CF-001`).
  - `usedPatchedCompactcPass: bool` — True if a patched `compactc` pass was used (`false`, `ECC-001`).
  - `usesLflattenedTap: bool` — True iff Stage 2 `Lflattened` tap is active (`CF-001`).
  - `zkirVersion: int` — Input ZKIR version (`31` = `3.1`, `32` = `3.2`, `ECC-001`, `ECC-013`).
  - `deployProfile: bool` — True when compiling in the deployable contract profile (`ECC-009`).
  - `userUsesSlot15: bool` — True when the contract's user ledger fields occupy top-level slot `15` (`ECC-009`).
  - `firstFailingStage: int` — Earliest failing check in `1..5` (`0` = none, `1` = `read`, `2` = `irVersion`, `3` = `inputTypes`, `4` = `lowering`, `5` = `seal`; `ECC-010`).
  - `accepted: bool` — True iff the circuit compiled to an image and ABI.
  - `reportedStage: int` — Refusal stage reported by the compiler (`0` when accepted; `ECC-010`).
  - `refusalCode: str` — Named `MYC-REF-*` / `InputType:*` / `MYC-SEAL-021` code (`"none"` when accepted; `ECC-010`, `ECC-013`, `CF-014`).
  - `unsupportedCount: int` — Count of circuits classified as `unsupported` (`0`, `ECC-010`).
  - `emittedImageCount: int`, `emittedAbiCount: int` — Emitted artifact counts (`1` each when accepted, `0` when refused; `ECC-003`).
  - `imageSelector: int`, `abiSelector: int` — Entry selector recorded in image and ABI (`ECC-003`).
  - `slotsCount: int`, `abiTapeLen: int` — Word slot count and ABI `tape_len` (`3 + slotsCount`, `ECC-003`).
  - `codeEffectWords: int`, `abiLE: int`, `abiNE: int` — Effect word count in `code`, ABI `l_e`, and static step count `n_e` (`ECC-003`).
  - `maxEmittedShapeId: int` — Highest `SInstr` shape ID emitted (`1..13`, `ECC-005`).
  - `universalKeysChanged: bool` — True if compiling a contract altered universal keys (`false`, `ECC-005`).
  - `anchorSlotUsed: int` — Anchor ledger slot (`15` in deployable profile, `ECC-009`).
  - `hasSpikeConstants: bool` — True if `SPIKE_SLOT`, `CP_STAND_IN`, or `SALT` leaked into deployable output (`false`, `ECC-009`).
  - `effectAddressesRoot15: bool` — True if user effects `W` address root key `15` (`false`, `ECC-009`).
  - `keccakUsesChipRule: bool`, `keccakUsedTier3OrRefused: bool` — Lowering outcome for `keccak256` (`ECC-007`).
  - `jubjubPointWordsRead: int`, `jubjubCurveAndSubgroupChecked: bool` — Tape-word count (`2`) and curve/subgroup checks for `Point<Jubjub>` (`ECC-014`).
  - `secp256k1LoweredToKit: bool`, `secp256k1RefusedInputType: bool` — Lowering outcome for `Secp256k1` values/inputs (`CF-013`).
  - `unknownInputTypeAdmitted: bool` — True if an unknown curve type (`Curve25519`) was admitted (`false`, `CF-014`).
- `DeterminismState` (`cc.det`):
  - `checked: bool`, `imageShaTape1: int`, `imageShaTape2: int`, `abiShaTape1: int`, `abiShaTape2: int` (`ECC-004`), `stepCountTape1: int`, `stepCountTape2: int`, `spendStepCount: int` (`CF-004`), `planReadNoWitness: bool` (`CF-002`).
- `BatchCompileState` (`cc.batch`):
  - `executed: bool`, `requestedCount: int`, `refusedCircuitCount: int`, `writtenImageCount: int`, `writtenAbiCount: int`, `cliExitCode: int`, `printedRefusalCode: str` (`ECC-012`, `ECC-013`).
- `FactoryGateState` (`cc.gates`):
  - `evaluated: bool`, `hasUnarguedSurvivingMutant: bool`, `gatesShPassed: bool`, `usableForSettledCall: bool` (`ECC-002`), `hasGoldenOracleDiff: bool`, `oracleGatePassed: bool`, `namedDiffVector: str` (`ECC-011`), `sorriesRemainingCount: int`, `lakeBuildWfailPassed: bool` (`LPC-013`).
- `PremiseClosureState` (`cc.premises`):
  - `mycroftDesignClosed: bool`, `s1Discharged: bool`, `s1KeptAsNamedHypothesis: bool`, `s1NamedInReport: bool` (`LPC-019`), `cfL1Proved: bool`, `onlyStandardLeanAxioms: bool`, `cfKitReportedComplete: bool` (`CF-012`).
- Monotone top-level `CompilerState` counters/flags:
  - `acceptedCompileRuns: int`, `refusedCompileRuns: int`, `keccakCompileOk: bool`, `jubjubCompileOk: bool`, `secp256k1CompileOk: bool`, `allValidBatchCompleted: bool`, `refusedBatchCompleted: bool`.

### 4.3 `lastOp` Values and Implemented Requirements

| `lastOp` Value | Action in `compact_compilation.qnt` | Implemented EARS IDs |
|---|---|---|
| `"init"` | `init` | Initial compiler state (`ECC-001..005`, `ECC-007`, `ECC-009..014`, `CF-013`, `CF-014`, `LPC-013`, `LPC-019`, `CF-001..002`, `CF-004`, `CF-012`) |
| `"compileSingleCircuit"` | `doCompileSingleCircuit` | `ECC-001`, `ECC-003`, `ECC-005`, `ECC-007`, `ECC-009`, `ECC-010`, `ECC-014`, `CF-013`, `CF-014`, `CF-001` |
| `"checkWitnessInvarianceAndStepBound"` | `doCheckWitnessInvarianceAndStepBound` | `ECC-004`, `CF-002`, `CF-004` |
| `"runCompileCliBatch"` | `doRunCompileCliBatch` | `ECC-012`, `ECC-013` |
| `"runFactoryGatesAndOracle"` | `doRunFactoryGatesAndOracle` | `ECC-002`, `ECC-011`, `LPC-013` |
| `"evaluatePremiseAndKitClosure"` | `doEvaluatePremiseAndKitClosure` | `LPC-019`, `CF-012` |

### 4.4 Constants of Concrete Instances

| Constant | `compact_compilation_main` | `compact_compilation_edge_min` | `compact_compilation_edge_at_limit` | Mutant Modules (`m_*`) |
|---|---:|---:|---:|---:|
| `SINSTR_SHAPE_COUNT` | `13` | `13` | `13` | `13` |
| `RSL_RULE_COUNT` | `16` | `16` | `16` | `16` |
| `DEPLOY_ANCHOR_SLOT` | `15` | `15` | `15` | `15` |
| `TAPE_HEADER_WORDS` | `3` | `3` | `3` | `3` |
| `JUBJUB_POINT_WORDS` | `2` | `2` | `2` | `2` |
| `ZKIR_SUPPORTED_VERSION` | `31` | `31` | `31` | `31` |
| `MAX_SPEND_STEPS` | `550` | `550` | `550` | `550` |
| `BATCH_CIRCUIT_COUNT` | `2` | `1` | `4` | `2` |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 16` |

---

## 5. Model Abstractions

1. **Representative Circuit Archetypes (`CircuitCase`)**: Rather than parsing JSON ZKIR 3.1 ASTs in Quint, 7 representative circuit fixtures (`CircCounterValid`, `CircKeccakValid`, `CircJubjubValid`, `CircSecp256k1Valid`, `CircUnknownCurve25519`, `CircMultiFailIrAndSeal`, `CircSlot15Collision`) span every accepted builtin/kit path and every stage of `ECC-010`'s 5-stage refusal pipeline.
2. **Instruction Shapes and ZKIR Versions as Bounded Integers**: The 13 `SInstr` constructors are enumerated in `SINSTR_SHAPES` and bounded via `maxEmittedShapeId in 1..13`; ZKIR semantic versions `3.1` and `3.2` are encoded as integers `31` and `32`.
3. **Serialized File Byte Equality via Digest Representatives**: `ECC-004`'s byte-for-byte image and ABI invariance across call tapes is modeled via abstract SHA-256 integer digests (`9001`, `9002` vs `9999`, `9998` under witness-dependent mutation).
4. **Factory Gate & Lean `--wfail` Outcomes**: `gates.sh` mutation killing, `OracleMain` golden-vector comparison, `SORRIES.md` counts, and Lean axiom reports are modeled by their observable pass/fail booleans, remaining-sorry counts, and named mismatch identifiers.

---

## 6. Commands

Run all commands below from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/compact-compilation/quint`:

```bash
# 1. Typecheck base specification + instances + mutants (expected exit: 0; printed: empty)
quint typecheck compact_compilation.qnt
# Exit: 0 | Output: (none)

# 2. Typecheck deterministic scenario test module (expected exit: 0; printed: empty)
quint typecheck compact_compilation_test.qnt
# Exit: 0 | Output: (none)

# 3. Run deterministic scenario tests (expected exit: 0)
quint test --main=compact_compilation_test --match='cc_.*Test' --seed=0x20261115 compact_compilation_test.qnt
# Exit: 0 | Printed: 5 passing (580ms)

# 4. Honest simulation: main instance (expected exit: 0)
quint run compact_compilation.qnt --main compact_compilation_main --invariant inv --max-samples 2000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (1736ms at 1152 traces/second).

# 5. Honest simulation: minimum-batch edge instance (expected exit: 0)
quint run compact_compilation.qnt --main compact_compilation_edge_min --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (966ms at 1035 traces/second).

# 6. Honest simulation: upper-batch edge instance (expected exit: 0)
quint run compact_compilation.qnt --main compact_compilation_edge_at_limit --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (931ms at 1074 traces/second).

# 7. Reachability witnesses on compact_compilation_main (all 5 MUST violate -> expected exit: 1)
quint run compact_compilation.qnt --main compact_compilation_main --invariant reach_keccak_jubjub_and_secp256k1_compiled --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (88ms at 11 traces/second).

quint run compact_compilation.qnt --main compact_compilation_main --invariant reach_batch_compile_admit_and_refuse --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (112ms at 9 traces/second).

quint run compact_compilation.qnt --main compact_compilation_main --invariant reach_ordered_refusals_and_unknown_type --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (109ms at 128 traces/second).

quint run compact_compilation.qnt --main compact_compilation_main --invariant reach_witness_invariance_and_factory_gates_pass --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (172ms at 17 traces/second).

quint run compact_compilation.qnt --main compact_compilation_main --invariant reach_premise_s1_and_cfl1_closure --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (185ms at 11 traces/second).

# 8. Guard mutants (all 16 MUST violate their target invariant -> expected exit: 1)
quint run compact_compilation.qnt --main compact_compilation_m_ecc001_patched_or_wrong_zkir --invariant inv_ECC_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (120ms at 8 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_ecc002_settle_surviving_mutant --invariant inv_ECC_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (111ms at 9 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_ecc003_bad_abi_layout --invariant inv_ECC_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (92ms at 11 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_ecc004_witness_dependent_bytes --invariant inv_ECC_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (108ms at 9 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_ecc005_illegal_sinstr_shape --invariant inv_ECC_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (131ms at 8 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_ecc007_keccak_tier3_or_refuse --invariant inv_ECC_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (116ms at 17 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_ecc009_spike_slot_or_collision --invariant inv_ECC_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (110ms at 9 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_ecc010_wrong_refusal_order --invariant inv_ECC_010 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (143ms at 42 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_ecc011_ignore_oracle_diff --invariant inv_ECC_011 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (111ms at 9 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_ecc012_missing_abi_or_nonzero --invariant inv_ECC_012 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (163ms at 12 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_ecc013_partial_output_on_refusal --invariant inv_ECC_013 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (111ms at 27 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_ecc014_jubjub_missing_curve_check --invariant inv_ECC_014 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (147ms at 82 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_cf013_refuse_secp256k1_input --invariant inv_CF_013 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (121ms at 8 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_cf014_admit_unknown_curve25519 --invariant inv_CF_014 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (152ms at 66 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_cf004_tape_dependent_steps --invariant inv_CF_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (84ms at 12 traces/second).

quint run compact_compilation.qnt --main compact_compilation_m_lpc019_close_with_unnamed_s1 --invariant inv_LPC_019 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (195ms at 5 traces/second).

# 9. Generate 20 ITF replay traces (expected exit: 0)
mkdir -p /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/compact-compilation/traces/compact_compilation_main
quint run compact_compilation.qnt --main compact_compilation_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/compact-compilation/traces/compact_compilation_main/t_{seq}.itf.json'
# Exit: 0 | Printed: [ok] No violation found (276ms at 72 traces/second).
```
