import Bridge.Spec

/-!
# CONSTANT-CHECKED theorems of Bridge

`decide` over recorded values (kernel evaluation, no `native_decide`). Labels:
MEASURED = a run record; CODE = a constant or an asserted test value in the code at the cited
commit (read by Formal-Lean-B on 2026-11-15 with `git show <commit>:<path>` in the openvm clone
`~/.cache/m7-wrap/src/openvm-e3`); MODEL = a Quint constant; DERIVED = computed from other
values, not measured; LIST = `~/.cache/rekey-t11/LIST.md` (sha256 6392209810…81db, 10:31).
`rekey-int` 155d321 is the integration branch that merges fp3 a2eeece (1dd9a7e) and t12-slot15
4ea3d05 (8cb13b6).
-/

namespace Bridge

/-! ## `L` key bounds (LTR-003) -/

/-- MEASURED: Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:11` and `:49`:
    `L` 1,995 of 2,000 columns, `StmtAir` 99 of 100 interactions (T10m keys). -/
def lColsT10m : Nat := 1995
def lInterT10m : Nat := 99
/-- CODE: openvm t10mm 5b7ada8 `crates/sherlock-lfr/src/frozen.rs:195`, `:197` (`L` set). -/
def codeLMaxColsT10m : Nat := 2000
def codeLMaxInter : Nat := 100
/-- MEASURED: LIST line 44: FP-3 `EffectAir` has 77 columns. -/
def effectAirCols : Nat := 77
/-- DERIVED: LIST line 47: `L` after FP-3 = 1,995 - 1 + 77 (the bare `Call` AIR goes, 1 column). -/
def lColsFp3Derived : Nat := lColsT10m - 1 + effectAirCols
/-- CODE: openvm fp3 a2eeece and rekey-int 155d321 `crates/sherlock-lfr/src/frozen.rs:202`:
    `L` max_columns 2,200 (LEAD PICK FP3-LCOLS, for T15). -/
def codeLMaxColsFp3 : Nat := 2200

/-- LTR-003 on the T10m keys: the spec bound and the code bound hold (5 columns headroom). -/
theorem ltr003_t10m : lColsT10m ≤ lMaxCols ∧ lInterT10m ≤ lMaxInter ∧
    codeLMaxColsT10m = lMaxCols ∧ codeLMaxInter = lMaxInter ∧ lMaxCols - lColsT10m = 5 := by
  decide

/-- LTR-003 against CLB-002 (FINDING F-BR-1): the `L` width that FP-3 needs (DERIVED 2,071) is
    above the spec bound 2,000, so the oracle refuses the `L` key that CLB-002 requires; the
    committed code bound (2,200) accepts it. -/
theorem ltr003_fp3_conflict : lColsFp3Derived = 2071 ∧ lMaxCols < lColsFp3Derived ∧
    lColsFp3Derived ≤ codeLMaxColsFp3 ∧ codeLMaxColsFp3 ≠ lMaxCols := by decide

/-- The FP-3 configuration of the T25 rekey: the bridge keygen refuses it under LTR-003. -/
def fp3Keygen : St := init lColsFp3Derived lInterT10m 15 15 1 2 2 true false true true true true true
theorem ltr003_fp3_refused : (stepRefusals fp3Keygen 0).map Refusal.code = ["l-key-bounds"] := by
  decide

/-! ## Interface (CLB-001, LW-001, LW-003, LW-018, FLG-003, FLG-004) -/

/-- CODE: openvm t10mm 5b7ada8 `crates/sherlock-lfr/src/iface.rs:42` `LANE_PAIRS = 5`; the test
    `layout_full_is_12166` asserts 12,166 words, call vector at 2,932 and `2B + 9V = 2,816`
    (`iface.rs:1457` to `:1459`), unchanged on rekey-int 155d321 (`iface.rs:1585` to `:1588`). -/
def codeLanePairs : Nat := 5
def codeIfaceWords : Nat := 12166
def codeCallAt : Nat := 2932
def codeWpv : Nat := 2816
/-- FLG-003: `B = V = P = 256`, `E = 0`. LW-003: `L_TX = 1,024`. -/
def capB : Nat := 256
def capV : Nat := 256
def capP : Nat := 256
def capE : Nat := 0
def lTx : Nat := 1024

theorem clb001_interface : codeLanePairs = 5 ∧ codeIfaceWords = 12166 := by decide

/-- FLG-004: `W_pv = 2B + 9V = 2,816`, equal to the value the code asserts. -/
theorem flg004_wpv : 2 * capB + 9 * capV = 2816 ∧ codeWpv = 2 * capB + 9 * capV := by decide

/-- LW-003, LW-001: the call vector has `2 + L_TX` = 1,026 words. -/
theorem lw003_call_vector : 2 + lTx = 1026 := by decide

/-- LW-018: `L_Fr` publishes the `L` key pre-hash and the `L` words packed 8 per Fr word:
    1 + ceil(12,166 / 8) = 1,522 (the value the code asserts at 5b7ada8 `iface.rs:1461`). -/
theorem lw018_lfr_statement : 1 + (codeIfaceWords + 7) / 8 = 1522 := by decide

/-- FLG-006: the largest reservation (4 entries, 8 values, 4 permutations) fits the capacity;
    FLG-005: `SPONGE(n)` admits `1 <= n <= V - 1` and `PUB(i)` admits `i < 2 + L_TX`; `ECMUL`
    is not admitted while `E = 0`. -/
theorem flg_capacity : 4 ≤ capB ∧ 8 ≤ capV ∧ 4 ≤ capP ∧ capV - 1 = 255 ∧ 2 + lTx = 1026 ∧
    capE = 0 := by decide

/-! ## Anchor, lock, chunks (CLB-004, CLB-005, CLB-007, CLB-008, ANC, HDM-001) -/

/-- CODE: openvm t12-slot15 4ea3d05 and rekey-int 155d321 `crates/sherlock-lfr/src/iface.rs:112`
    `ANCHOR_SLOT: u64 = 15`. -/
def codeAnchorSlot : Nat := 15
/-- CODE: openvm t10q-ntfix e3b407d `crates/sherlock-lfr/src/iface.rs:277` `U_LOCK = 8`,
    `:281` `LOCK_LEN = 6` (the six operands of CLB-005, ANC-002, ANC-017). -/
def codeULock : Nat := 8
def codeLockLenNtfix : Nat := 6
/-- CODE: openvm fp3 a2eeece / rekey-int 155d321 `iface.rs:290` `LOCK_LEN = 7`: FP-3 adds the
    operand `link` = `pack(C_(j+1))` (doc at `iface.rs:288` to `:289`, `lock_x` at `:316`). -/
def codeLockLenFp3 : Nat := 7
/-- The operands that CLB-005, ANC-002 and ANC-017 name: c, pack(run_tag), k, d_in, d_out, j. -/
def specLockOperands : List String := ["c", "pack(run_tag)", "k", "d_in", "d_out", "j"]
/-- MODEL: Moriarty t20-batched 5dc1be2ee `models/sherlock/quint/t20-batched/chunkCap.qnt:26`
    `K_MAX = 2` (no Rust constant, CLB-007 status). -/
def modelKMax : Nat := 2

theorem clb004_anchor_slot : codeAnchorSlot = anchorSlot := by decide

/-- HDM-001: the lock operand sponge uses use code 8, the code's `U_LOCK`; its fixed-length
    capacity is `l + 8 * 2^66` with `l` the operand count. -/
theorem hdm001_lock_code : codeULock = 8 ∧ specLockOperands.length = codeLockLenNtfix := by decide

/-- CLB-005 against the FP-3 code (FINDING F-BR-2): the merged lock sponge absorbs 7 operands,
    the requirement text names 6, so the capacities `cap(8, 6)` and `cap(8, 7)` differ. -/
theorem clb005_fp3_operands : specLockOperands.length ≠ codeLockLenFp3 ∧
    codeLockLenNtfix + codeULock * 2 ^ 66 ≠ codeLockLenFp3 + codeULock * 2 ^ 66 := by decide

/-- CLB-007: the model cap equals the spec cap. -/
theorem clb007_kmax : modelKMax = kMax := by decide

/-- HDM-001, HDM-004: the transcript capacities `2^64 + u 2^66` for `L` (u = 3) and `L_Fr`
    (u = 7) are distinct from each other and from every fixed-length capacity `l + u 2^66` with
    `l < 2^64` of the other use codes. -/
theorem hdm004_capacities : 2 ^ 64 + 3 * 2 ^ 66 ≠ 2 ^ 64 + 7 * 2 ^ 66 ∧
    (2 ^ 64 + 3 * 2 ^ 66) % 2 ^ 66 = 2 ^ 64 ∧ (2 ^ 64 + 7 * 2 ^ 66) / 2 ^ 66 = 7 := by decide

/-! ## `L_Fr` parameters and the wrap (LW-013, LW-016, LW-019) -/

/-- CODE: openvm t10mm 5b7ada8 `crates/sherlock-lfr/src/frozen.rs:203` to `:214` (`LFR` set):
    log blowup 3, l_skip 2, n_stack 18, w_stack 48, query PoW 20, queries 98/83/82. -/
def lfrParams : List Nat := [3, 2, 18, 48, 20, 98, 83, 82]
theorem lw019_lfr_params : lfrParams = [3, 2, 18, 48, 20, 98, 83, 82] ∧ 48 ≤ 48 := by decide

/-- MEASURED: RESULTS.md:11 (T10m keys): wrap 1,685,845 rows at k = 21, VK 1,325 bytes;
    RESULTS.md:91: `L_Fr` query term 101.1435 bits (millibits below). -/
def wrapRowsT10m : Nat := 1685845
def wrapK : Nat := 21
def wrapVkBytesT10m : Nat := 1325
def lfrQueryMillibits : Nat := 101143

theorem lw013_wrap_bounds : wrapK ≤ 21 ∧ wrapRowsT10m ≤ 2 ^ wrapK ∧ wrapVkBytesT10m ≤ 50000 ∧
    101000 ≤ lfrQueryMillibits := by decide

end Bridge
