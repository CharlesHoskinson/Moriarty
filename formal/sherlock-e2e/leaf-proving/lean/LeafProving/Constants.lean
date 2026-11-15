import LeafProving.Spec

/-!
# CONSTANT-CHECKED theorems of Leaf Proving

Each theorem checks a recorded value against its bound by `decide`. Sources (read by
Formal-Lean-B on 2026-11-15): CODE = openvm `rekey-chips` dcf1b4c (in
`~/.cache/m7-wrap/src/openvm-e3`, `git show dcf1b4c:<path>`); LIST =
`~/.cache/rekey-t11/LIST.md` sha256 6392209810…81db; RESULTS = Moriarty 3dc9001a9
`research/sherlock-t10m-20261114/RESULTS.md`.
-/

namespace LeafProving

/-- CODE `crates/sherlock-leaf/src/rsl/trace.rs:49` `MAX_APP_AIRS = 8`. -/
def codeMaxAppAirs : Nat := 8
/-- CODE `rsl/trace.rs:122` to `:138` (`names`): 5 base AIRs (WORD, ALU, PUBLIC, RANGE, SHA_IN)
    plus 3 for the SHA-256 kit or 3 for the Keccak-256 kit. -/
def codeBaseAirs : Nat := 5
def codeKitAirs : Nat := 3

/-- LFP-003: the code bound equals the spec bound. -/
theorem lfp003_bound_code : codeMaxAppAirs = maxAppAirs := by decide

/-- RSL-002, LFP-003: every shape the chip set builds (no hash, one hash kit) is inside the
    6-or-9 bucket and the 8-AIR bound; both kits give 11 AIRs, over the bound (refused). -/
theorem rsl002_code_shapes :
    inBucket codeBaseAirs = true ∧ inBucket (codeBaseAirs + codeKitAirs) = true ∧
    codeBaseAirs + codeKitAirs ≤ maxAppAirs ∧ maxAppAirs < codeBaseAirs + 2 * codeKitAirs := by
  decide

/-- RSL-002 against LFP-003: the bound of LFP-003 admits 6 and 7 app AIRs, which are outside
    the bucket of RSL-002 (finding F-B-LFP-1: no requirement refuses them; the code never
    builds them). -/
theorem lfp003_admits_outside_bucket :
    (6 ≤ maxAppAirs ∧ inBucket 6 = false) ∧ (7 ≤ maxAppAirs ∧ inBucket 7 = false) := by decide

/-- LIST row 6f773b2: MEASURED odin r10 probe, `n1_pre_hash_equals_universal_n1` true for
    a05, t02, s01 (6 AIRs with FP-1) and d04, z02 (9 AIRs). LIST row 5d77cfc: a05 at 10 AIRs
    gave another pre-hash. The model's pre-hash predicate agrees on all three counts. -/
theorem rsl001_probe_counts :
    inBucket (6 - 1) = true ∧ inBucket (9 - 1) = true ∧ inBucket (10 - 1) = false := by decide

/-- CODE `crates/sherlock-lfr/src/frozen.rs:83` (set "app") and `:98` `max_columns: 30000`. -/
def codeAppMaxColumns : Nat := 30000
/-- LIST.md:30: MEASURED (odin r5) max R_SL app columns 16,346 (d04 withdraw). -/
def measuredMaxAppCols : Nat := 16346

/-- LFP-004: the code bound is the spec bound and the largest measured R_SL app is inside. -/
theorem lfp004_columns : codeAppMaxColumns = maxAppCols ∧ measuredMaxAppCols ≤ maxAppCols := by
  decide

/-- LFP-004 scenario: an app key of 30,001 columns is refused at admission. -/
theorem lfp004_refuses_30001 :
    (stepRefusals (init 5 false false 30001 false true 1 2 2 3 3 4 4 0 1) 0).map Refusal.code =
      ["column-bound"] := by decide

/-- CODE `crates/sherlock-leaf/src/leaf.rs:34` `HEIGHTS: [usize; 7]` and `:635` `HEIGHTS_V`
    (chip set 2): seven heights in AIR order. -/
def codeHeightsLen : Nat := 7

/-- LFP-005: a fixed vector of seven heights exists for chip sets 0 to 2 (none is named for
    chip set 3: spec status GAP). -/
theorem lfp005_heights : codeHeightsLen = 7 := by decide

/-- CODE `crates/sherlock-leaf/src/core.rs:46` `pv::PROG_D = 8` (doc `:42`). -/
def codeProgDSlot : Nat := 8
/-- CODE `crates/sherlock-leaf/src/rsl/air.rs:223` `FP1_N = 54`. -/
def codeFp1Words : Nat := 54

/-- LFP-006: the slot `PROG_D` lies inside the 54 FP-1 public values. -/
theorem lfp006_slot_in_fp1 : codeProgDSlot < codeFp1Words := by decide

/-- RESULTS:11, :64: MEASURED wrap at k = 21 (1,685,845 rows). -/
def measuredWrapK : Nat := 21

/-- CPR-001: no BLS12-381 circuit at k >= 22; the wrap takes k = 19, 20 or 21. -/
theorem cpr001_wrap_k : measuredWrapK ≤ 21 ∧ 19 ≤ measuredWrapK := by decide

end LeafProving
