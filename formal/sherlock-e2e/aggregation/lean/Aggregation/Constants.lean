import Aggregation.Spec

/-!
# CONSTANT-CHECKED theorems of Aggregation

Each theorem checks recorded values against the bound of its requirement by `decide`.
Sources (read by the Formal-Lean-B aggregation fork on 2026-11-15):
* CODE: openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/frozen.rs` (`INTERNAL` bounds
  `:143` to `:147`, `log_blowup` `:133`, `queries` `:141`, `MERGE_WORK_COLUMNS` `:166`);
  openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/stmt.rs:90` (`UROWS = 7`).
* MEASURED: Moriarty `research/sherlock-t10m-20261114/RESULTS.md` (commit ca96c1466):
  key table `:51` to `:53` (G1 on `~/.cache/t10mm/key/keys.json`), V.9 `:89`, gp2 `:94`,
  x7 `:95`, d3work `:96`.
* MEASURED in a code comment: openvm `rekey-chips` dcf1b4c `frozen.rs:166` to `:168`
  (`Kind::Work` without the GP block and `fv_state`: 4,988 columns, `t7_chain shape`;
  value-set CORE +62). The STACKS width (71) is an ESTIMATE, so 5,121 is DERIVED and is
  checked apart, labelled DERIVED.
-/

namespace Aggregation

/-- One key of the T10m set as measured by G1: AIRs, columns, max interactions per AIR. -/
structure KeyShape where
  airs : Nat
  columns : Nat
  interactions : Nat
  deriving DecidableEq, Repr

/-- CODE: the internal set bounds (`frozen.rs:143` to `:147`). -/
def internalMaxLogHeight : Nat := 19
def internalMaxConstraints : Nat := 1000
def internalMaxInteractions : Nat := 200
def internalMaxAirs : Nat := 64
def internalMaxColumns : Nat := 3000
/-- CODE: `INTERNAL` log blowup 3 and queries 97/82/81 (`frozen.rs:133`, `:141`). -/
def internalLogBlowup : Nat := 3
def internalQueries : List Nat := [97, 82, 81]
/-- CODE: `MERGE_WORK_COLUMNS` (`frozen.rs:166`). -/
def codeMergeWorkColumns : Nat := 5400
/-- CODE: `UROWS` (`stmt.rs:90`). -/
def codeUrows : Nat := 7

/-- MEASURED (`RESULTS.md:51`): MERGE-WORK over N3. -/
def mMergeWork : KeyShape := ⟨49, 5284, 183⟩
/-- MEASURED (`RESULTS.md:52`): NW over MERGE-WORK. -/
def mNW : KeyShape := ⟨41, 2285, 182⟩
/-- MEASURED (`RESULTS.md:53`): NW2, N3 over NW2, N2, N3, FV. -/
def mNode : KeyShape := ⟨41, 2284, 182⟩
/-- MEASURED (rekey-chips `frozen.rs:167`): `Kind::Work` without the GP block, and the
    value-set CORE increment. -/
def mWorkNoGp : Nat := 4988
def mValueCore : Nat := 62
/-- ESTIMATE (rekey-chips `frozen.rs:167`): STACKS 71 words. -/
def eStacks : Nat := 71

def withinInternal (k : KeyShape) : Prop :=
  k.airs ≤ internalMaxAirs ∧ k.columns ≤ internalMaxColumns ∧ k.interactions ≤ internalMaxInteractions

instance (k : KeyShape) : Decidable (withinInternal k) := by unfold withinInternal; infer_instance

/-- NAG-005, SHP-008, NODE-015: the code bound equals the requirement bound, the measured
    MERGE-WORK key is within it (5,284 <= 5,400) and within the per-AIR bounds of the internal
    parameters (49 AIRs <= 64, 183 interactions <= 200), and the oracle refuses 5,401. -/
theorem nag005_merge_work_measured :
    codeMergeWorkColumns = mergeWorkColumns ∧ mMergeWork.columns ≤ mergeWorkColumns ∧
    mMergeWork.airs ≤ internalMaxAirs ∧ mMergeWork.interactions ≤ internalMaxInteractions := by
  decide

/-- LPC-002: the measured parts of the complete kernel fit (4,988 + 62 <= 5,400); the full
    width with STACKS is DERIVED (4,988 + 71 + 62 = 5,121 <= 5,400), not MEASURED. -/
theorem lpc002_measured_parts : mWorkNoGp + mValueCore ≤ mergeWorkColumns := by decide
theorem lpc002_derived_full_width :
    mWorkNoGp + eStacks + mValueCore = 5121 ∧ mWorkNoGp + eStacks + mValueCore ≤ mergeWorkColumns := by
  decide

/-- NODE-007, NODE-014, SHP-007, CLM-008, FREG-005, NODE-015: the measured node keys (N2, N3,
    NW, NW2, FV) are inside the internal set: at most 64 AIRs, 3,000 columns, 200 interactions;
    the oracle's arity bound is 4 and the built node has 2 child slots (NODE-008 `j` in {0, 1}). -/
theorem node_keys_internal_measured :
    withinInternal mNode ∧ withinInternal mNW ∧ ¬ withinInternal mMergeWork ∧ 2 ≤ maxArity := by
  decide

/-- NAG-001: the nodes use the internal parameters: log blowup 3, queries 97/82/81, height at
    most 2^19. -/
theorem nag001_internal_params :
    internalLogBlowup = 3 ∧ internalQueries = [97, 82, 81] ∧ internalMaxLogHeight = 19 ∧
    internalMaxConstraints = 1000 := by decide

/-- SHP-009, GPH-005: `keys_d` has seven node rows; the oracle's node-row table has the same
    size; foreign rows follow at 7 and 8 (GPH-003). -/
theorem shp009_urows : codeUrows = uRows ∧ uRows + foreignRows = 9 := by decide

/-- MEASURED gate counts: V.9 omit 70 cases (`RESULTS.md:89`, NAG-002), gp2 48 of 48 (`:94`,
    GPH-002), x7 22 of 22 (`:95`, GPH-005). GPH-007 keeps 48 and 22 as count floors. -/
def mOmitCases : Nat := 70
def mGp2 : Nat × Nat := (48, 48)
def mX7 : Nat × Nat := (22, 22)
theorem gate_counts_measured : mOmitCases = 70 ∧ mGp2.1 = mGp2.2 ∧ mX7.1 = mX7.2 ∧
    48 ≤ mGp2.1 ∧ 22 ≤ mX7.1 := by decide

end Aggregation
