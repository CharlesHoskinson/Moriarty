import Assurance.Spec

/-!
# CONSTANT-CHECKED theorems of Assurance

Each theorem checks recorded values against the bound of its requirement by `decide`
(kernel evaluation, no `native_decide`). Every value cites its source and was read by
Formal-Lean-B (assurance fork) on 2026-11-15:

* R10M = Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md` (MEASURED, T10m keys);
* FROZEN = openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/frozen.rs` (CODE);
* CX = Moriarty 3dc9001a9 `tests/quint-regressions/COUNTEREXAMPLES.md` (record);
* D10 = Moriarty `d10-heights` 3b979bd5c (commit message, MEASURED);
* RUNBOOK = Moriarty `t12-gate` 90f9b4df9 `research/sherlock-tested-design-20261113/t12/RUNBOOK.md`;
* QV = Moriarty `t13-lanes` 958a26919 `qa/gemini-quint/` (5 `verdict.md` files).

Bits are in millibits (101.0110 bits = 101011).
-/

namespace Assurance

/-! ## D3 (SQG-004, SQG-007, SSG-001, RKY-005) -/

def d3BoundMilli : Nat := 100000
def d3GrindBoundMilli : Nat := 101000
/-- R10M:91: weakest terms of the 8 chain keys: leaf 101.0110, app 101.0891, `L` 101.0730,
    `L_Fr` 101.1435, nodes 101.1643; R10M:96: MERGE-WORK, NW, NW2 weakest 101.1643. -/
def d3T10m : List Nat := [101011, 101089, 101073, 101143, 101164, 101164]

/-- SQG-004, SQG-007: on the T10m keys every weakest term is at least 100 bits and at least
    101 bits (so every grindable term passes its bound). -/
theorem sqg004_d3_t10m : d3T10m.all (fun b => d3BoundMilli ≤ b && d3GrindBoundMilli ≤ b) = true := by
  decide

/-! ## Wrap k, rows and VK (DG-001, DG-002, DG-003; shared with slice C WRP) -/

/-- R10M:11, :64: wrap 1,685,845 rows at k = 21 (gate 1,782,573); R10M:11, :65: wrap VK 1,325 bytes. -/
def wrapRowsT10m : Nat := 1685845
def wrapK : Nat := 21
def wrapVkBytesT10m : Nat := 1325

/-- 85 percent of the `2^k - 7` usable rows, rounded down. -/
def rowGate (k : Nat) : Nat := 85 * (2 ^ k - 7) / 100

/-- DG-001: the wrap is keyed at k = 21, not above. -/
theorem dg001_k : wrapK ≤ 21 := by decide

/-- DG-002: the measured rows fit the 85 percent gate at k = 21 and not at 19 or 20, so 21 is
    the smallest k that fits; the gate value equals the recorded 1,782,573. -/
theorem dg002_smallest_k : rowGate 21 = 1782573 ∧ wrapRowsT10m ≤ rowGate 21 ∧
    ¬ wrapRowsT10m ≤ rowGate 20 ∧ ¬ wrapRowsT10m ≤ rowGate 19 := by decide

/-- DG-003: the wrap VK is at most 50,000 bytes. -/
theorem dg003_vk : wrapVkBytesT10m ≤ 50000 := by decide

/-! ## Frozen sets (SQG-003, SQG-004, SQG-005, SQG-013, DG-005) -/

/-- A frozen set as the code states it. -/
structure Set' where
  logBlowup : Nat
  wStack : Nat
  foldingPow : Nat
  muPow : Nat
  queryPow : Nat
  logupPow : Nat
  queries : List Nat
  maxLogHeight : Nat
  deriving DecidableEq

/-- FROZEN:180 to :197 (set `L`). -/
def codeL : Set' := ⟨3, 48, 0, 6, 20, 18, [98, 83, 82], 20⟩
/-- FROZEN:130 to :147 (set `internal`; `max_log_height` 19 at :143). -/
def codeInternal : Set' := ⟨3, 512, 0, 9, 21, 18, [97, 82, 81], 19⟩
/-- FROZEN:81 to :98 (app: query PoW 21 at :90) and :105 to :122 (leaf_agg: query PoW 21 at
    :114, LogUp PoW 17 at :115). -/
def codeAppQueryPow : Nat := 21
def codeLeafAggQueryPow : Nat := 21
def codeLeafAggLogupPow : Nat := 17
/-- FROZEN:166 `MERGE_WORK_COLUMNS = 5400`; FROZEN:157 to :161 the set is `..INTERNAL`. -/
def codeMergeWorkColumns : Nat := 5400
/-- R10M:96: the MERGE-WORK key (`merge_work`) has 5,284 columns (MEASURED, T10m). -/
def mergeWorkColumnsT10m : Nat := 5284
/-- D10 (3b979bd5c): `w_stack` 18 on `L_cached`, 26 without the cache. -/
def d10WStackCached : Nat := 18
def d10WStackNoCache : Nat := 26

/-- SQG-003: the `L` set in code is log blowup 3, queries 98/83/82, folding PoW 0, `mu` PoW 6,
    query PoW 20, LogUp PoW 18. -/
theorem sqg003_L_set : codeL.logBlowup = 3 ∧ codeL.queries = [98, 83, 82] ∧
    codeL.foldingPow = 0 ∧ codeL.muPow = 6 ∧ codeL.queryPow = 20 ∧ codeL.logupPow = 18 := by
  decide

/-- SQG-004: query PoW 21 for the app, leaf-aggregation and internal sets; LogUp PoW 17 for the
    leaf-aggregation set. -/
theorem sqg004_pow : codeAppQueryPow = 21 ∧ codeLeafAggQueryPow = 21 ∧
    codeInternal.queryPow = 21 ∧ codeLeafAggLogupPow = 17 := by decide

/-- SQG-005, DG-005: the `L` bound `w_stack` <= 48 with `mu` PoW 6 holds for the D10 values;
    the internal set keeps 512; `L` height at most `2^20`. -/
theorem sqg005_wstack : d10WStackCached ≤ codeL.wStack ∧ d10WStackNoCache ≤ codeL.wStack ∧
    codeL.wStack = 48 ∧ codeL.muPow = 6 ∧ codeInternal.wStack = 512 ∧
    codeL.maxLogHeight = 20 := by decide

/-- SQG-013: `merge_work` has the internal parameters and query vector 97/82/81 and 5,400
    columns; the measured T10m MERGE-WORK key (5,284) is inside the bound. -/
theorem sqg013_merge_work : codeInternal.queries = [97, 82, 81] ∧ codeMergeWorkColumns = 5400 ∧
    mergeWorkColumnsT10m ≤ codeMergeWorkColumns := by decide

/-! ## Encodings (FPR-001, FPR-005, DG-009, ZK-006, ZK-011) -/

/-- FPR-001: limbs of 16 bits: `u32` 2, `u64` 4, `u128` 8; `digest` 8 words; `fr` 9 limbs of
    30 bits cover the 255-bit field. -/
theorem fpr001_widths : 2 * 16 = 32 ∧ 4 * 16 = 64 ∧ 8 * 16 = 128 ∧ 255 ≤ 9 * 30 := by decide

/-- FPR-005: `stmt_tag` 1 + payload 32 + `key_root` 8 + `reg_root` 8 + `shape_d` 8 + `log_d` 8 +
    `log_n` 1 + `keys_d` 8 = 74 words (digests 8 words by FPR-001). -/
theorem fpr005_74_words : 1 + 32 + 8 + 8 + 8 + 8 + 1 + 8 = 74 := by decide

/-- DG-009: a `u64` height (4 limbs of 16 bits) holds `2^64 - 1` and not `2^64`. -/
theorem dg009_height : 2 ^ 64 - 1 < 2 ^ (4 * 16) ∧ ¬ (2 ^ 64 < 2 ^ (4 * 16)) := by decide

/-! ## The T12 gate on the record of 2026-11-15 (SSG-003, SSG-004, SQG-016, SSG-006) -/

/-- CX: 1 row CONFIRMED with no fix (GP2-C2.1), 8 NOT ATTEMPTED; RUNBOOK:77, :87: 2 UNVERIFIED
    steps; no gate check run; G5 not mapped. -/
def gate20261115 : St := { init false false false 1 2 1 8 false with manifestD := 1 }

theorem t12_not_passed_20261115 :
    (stepRefusals gate20261115 (8 * 16)).map Refusal.code =
      ["checks-incomplete", "step-unverified", "confirmed-defect", "g5-old-params"] := by decide

/-- SSG-004: on the same record, after every check passes, every step is verified, the defect is
    fixed and G5 maps the current sets, the record lists the 8 NOT ATTEMPTED rows as open. -/
theorem ssg004_eight_open :
    (run { gate20261115 with g5Current := true } ([7 * 16, 7 * 16 + 1, 7 * 16 + 2, 7 * 16 + 3,
      7 * 16 + 4, 7 * 16 + 5, 15 * 16, 15 * 16, 14 * 16, 8 * 16])).recordOpen = 8 := by decide

/-- LPC-016, LPC-004: QV holds 5 lead verdicts for 15 lanes that SSG-005's status names; no row
    can be counted DONE. -/
def lanes20261115 : St := { init false false false 1 0 0 0 true with
  changes := 15, changeRows := 15, verdicts := 5 }
theorem lpc016_not_done_20261115 :
    (stepRefusals lanes20261115 (13 * 16)).map Refusal.code = ["no-lead-verdict"] := by decide

end Assurance
