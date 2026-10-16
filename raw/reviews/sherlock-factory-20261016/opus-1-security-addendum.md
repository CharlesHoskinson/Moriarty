# Addendum A1: corrections to the Opus V1 security-planning proposal

**Status.** This addendum only corrects my earlier proposal. That proposal stays as the historical record. Where the two conflict, this addendum governs.

Nothing here adds a gate, a runtime command or an authorization, or changes any bound. These remain as they were:
- native authority 0;
- C4;
- the 90s/30s caps, counters and terminal failures;
- every exact native and resource bound.

**Sources cited.** All hashes are as you supplied them; I did not recompute them.
- aux/2 design `88887f1d…`, with the same content in manifest `63182dc1…`
- Quint test file `66f96bbb…`
- Astra A1 report `6a349a6f…`

---

## 1. MUL carries (S3.1 and the S3 tests)

**Retracted:**
- In S3.1, "every carry or borrow to {0, 1}".
- In the S3 rejection row, "carry = 2" used as a general test.

**Source:**
- **aux/2 §4.3:**
  - `ADD.c[1..17)` is Bool.
  - `SUB.bo[1..17)` is Bool.
  - `MUL.c[1..32)` is `unsigned(12)`.
  - `MUL.hSum` is `unsigned(12)`.
  - `MUL.hZero` is Bool.
- **aux/2 §10:**
  - The honest bound is `c_max ≤ 4080`, attained at 4079 (M-6, `c_16`).
  - The admitted range is `[0,4095]`.
  - It requires G4-FIELD `p > 2^20`.
- **Quint `arithmeticNormal8Test`:** freezes the carry chain `…3824,4079,3825…` and `highSum: 4079`.

**Corrected S3.1.** WHEN a checked UInt128 operation is lowered, the fragment SHALL constrain:
- every byte limb (operands, `r`, `h`) to [0, 255];
- ADD carries `c_1..c_16` and SUB/Lt borrows `bo_1..bo_16` to {0, 1};
- MUL carries `c_1..c_31` to 12-bit unsigned, with `c_0 = c_32 = 0` fixed;
- `MUL.hSum` to 12 bits and `MUL.hZero` to {0, 1};
- the fuel values `z`, `zp` and `bw` to {0, 1}.

No witness with an out-of-range value in any of these SHALL satisfy the fragment. The fragment SHALL NOT constrain MUL carries to {0, 1}, because the honest max×max witness (`c_16 = 4079`) must be accepted.

**S3.3, addition.** The certificate SHALL cite G4-FIELD `p > 2^20`. Its basis is aux/2 §10, where the MUL-k range of `E` lies in `(−2^20, 2^20)`. G4-FIELD remains an undischarged premise (aux/2 §15).

**Corrected S3.4** (this conflicts with aux/2 §5 and §8, so it is corrected here too):
- The charge equals `en_chg` and lies in {0, 1}.
- A charged step gives `f' = f − 1` exactly, with `bw_16 = 0`.
- A080, A082 and A084 give `f' = f`.
- At `f = 0`, precedence is Done (A080), then Rejected (A082), then Exhausted (A084). A charged success never occurs at `f = 0`.

The original "IF charge > fuel_in THEN exhaustion" is retracted because it omits terminal precedence. That precedence is shown by aux/2 F-5a and F-7 and by Quint `fuelNormal5Test` and `fuelNormal8Test`.

**Corrected S3 tests:**
- **Completeness control:** M-6 / `arithmeticNormal8Test` (carry 4079, hSum 4079, Err10).
- **Rejection tests:**
  - an ADD or SUB carry/borrow of 2;
  - a MUL carry ≥ 2^12, or a field-wrapped carry (N6);
  - `h` hidden in `c_32` (N7);
  - `hZero = 1` with `h ≠ 0` (N8).

## 2. Clean, Quint and K are required developer QA (C-5, S2 [F], §3.1)

**Retracted:**
- C-5: "optional developer-QA model".
- S2: "[F] optional developer QA".

**Corrected C-5.**
- Quint, K and Lean/Clean are **required**, iterative developer-correctness QA, run alongside the handwritten circuits.
- No Sherlock correspondence claim (S4, or S4.6 below) SHALL be made without them.
- They SHALL NOT become a dependency of the public compiler, runtime or prover. The product contract's "Lean is not a dependency" still holds for the shipped toolchain.
- The [F] label keeps the meaning "developer QA only" and drops "optional".

**Frozen aux/2 bytes.**
- aux/2 §13 labels N1–N16 "optional", and IO-8 says they are "not demanded".
- That frozen text is unchanged; any byte change would void its review.
- For current interpretation, the N1–N16 checks and the §3 specimens fall within the required QA scope.
- Running them still happens under the existing lock and counters. This addendum grants no run.

## 3. C-1: what the 18 tests cover

**Retracted:**
- C-1: "No adversarial or overflow model test ran".
- The S3 table's "add adversarial … for these", as applied to max+1 and 0−1.

**Source.** The Quint test file `66f96bbb…` has 8 arithmetic and 10 fuel `run` tests. Each one is `observe…(input).expect(observed == …(expected) and modelInv)` with an honest witness. They include:
- `arithmeticNormal2Test`: max+1 gives Err10, with 16 carries.
- `arithmeticNormal3Test`: 0−1 gives Err10, with `low` = 255 in all 16 bytes.
- `arithmeticNormal8Test`: max×max gives Err10.
- **Fuel precedence:** `fuelNormal5Test` (A080), `fuelNormal8Test` (A082) and `fuelNormal6Test` (A084), each at zero fuel.

**Corrected C-1.**
- **Reported pass.** The Root receipt reports that 18 named honest-witness tests passed once. Zero failed or ignored was inferred, not printed.
- **Covered:** semantic error-branch coverage for Add overflow, Sub underflow and Mul overflow, plus fuel-regime precedence. Class [F].
- **Not observed:** any hostile or mutated witness. No test supplies a witness other than the model's own.
- **Still open:**
  - K33, the sampled run and phase 2 remain unrun.
  - Trace adjudication is pending.
- R-3 stands, scoped to hostile-witness evidence.

**Data note, not a gate.**
- The comment on `fuelNormal9Test` cites "A108", which does not appear in the aux/2 §9 table of 39 rules.
- `FuelInput` carries no rule tag, so this pass does not show rule-tag binding (IO-1).
- I record the label as unreconciled.

## 4. Staged source row (§1.2)

**Retracted:** "unmerged" in the "Core continuations" row.

**Source.** Astra A1 inspected HEAD `9bcbca52` and found:
- `commands/simulate.rs` dispatches staged start/resume.
- `mori-core/src/stage/profile.rs` isolates `/1` and `/2`.
- `mori-backend/src/stage/mod.rs` produces N+1 segments.
- In `commands/compile.rs`, `/2` selects `compile_staged`.
- `/4` cannot convert to `/3`, and no proving, signed or native consumer accepts it.
- "All ten stage gates remain unperformed according to the source."
- The manifest test consumer calls `/1` directly.

**Corrected row.**
- The staged source is present and merged at `9bcbca52`. This is a source-inspected [L] observation, with no fresh run.
- The 58/78/86/88 counts stay as historical captures in a separate field.
- Native and target status is unqualified.
- No staged manifest journey exists.

**Anchors now supplied** (Astra's line observations at `9bcbca52`; re-anchoring SEC-01–21 is still deliverable 1):
- `compile_reported` is at `mori-backend/src/source_projection.rs:31` and is called from `commands/compile.rs`.
- The adapter label `source-static-scalar-projection-only` is at `mori-core/src/sherlock/source_adapter.rs:135`.
- The `cfg(test)` draft frontend is at `mori-core/src/lib.rs:10–26`.
- `todo!` is at `lsp.rs:9` and `mcp.rs:9`.
- The second `mori` executable is at `packages/moriarty-beta/package.json:9`.
- The C4 meaning comes from aux/2 §0.

**S1.1 consequence.** First check whether the existing label already states that the projection did not produce the artifact. Add a field only if it does not.

## 5. S11.3: M5 scope

**Retracted:** the whole of S11.3 as written. It applied k > 19 refusal to every keygen API, including `generate_call_keys`, and added an unpinned-parameter refusal.

**Source.** The Astra M5 row says:
- "64 KB transaction, fraud k≤19 and replay-budget qualification. Static ZKIR size is not transaction size."
- The backend sets `circuit_k` and `fraud_max_k` to None, with `cost_measured=false`.

**Corrected S11.3.**
- WHEN M5 qualification is claimed, the record SHALL show three things:
  - a measured k ≤ 19 for each fraud-game circuit;
  - a measured size ≤ 64 KB for each encoded transaction;
  - the replay-budget qualification.
- IF a fraud-game circuit exceeds k = 19, or an encoded transaction exceeds 64 KB, THEN M5 SHALL NOT be claimed for it.
- Static ZKIR byte or instruction counts SHALL NOT stand in for transaction size.
- `None` together with `cost_measured=false` SHALL be reported as unmeasured.

**Scope.**
- k ≤ 19 applies to fraud-game circuits only.
- Every other circuit and keygen route, including `generate_call_keys` and the Sherlock step circuits, keeps its existing exact bounds unchanged.
- S5.4's pinned-SRS condition remains, but only as a condition on independence claims.

## 6. Sherlock on Midnight stagenet is required

**Retracted:**
- The §2 ordering sentence that joins S3/S4 to stagenet "only if you require" it. It also cited U-5 by mistake.
- D-2.
- S5's "recommended pipeline", read as satisfying Sherlock's target requirement.
- §1.3 "No recursion", read as dismissing the recursion route.

**Corrected critical path.**
- **Lane A** (S1 → S2 → S5 → S6, on the ledger signed-v2 path) may deliver an earlier, independent U2 slice on stagenet (P-A and P-B).
- **Lane B** (S3 → S4 → S4.6, Sherlock end-to-end on stagenet) is critical.
- **Join.** V1 requires both lanes, plus the remaining U exits. Lane A evidence SHALL NOT substitute for S4.6 or for V1.
- Lane B still waits on these existing constraints:
  - C4, admitted A102 compilation;
  - native admission;
  - the phase-1 trace postgate (§7);
  - stagenet facts from the operator.

**New S4.6.**
- WHEN Sherlock end-to-end validation is claimed, a Sherlock program from the production frontend SHALL traverse compile, prove, independent verify (S5.4), deploy, call and authoritative readback on Midnight stagenet.
- It SHALL do so under the S4.1 statement, with readback reconciled as in S5.2.
- IF any stage used the backend transition relation or the ledger signed-v2 relation in place of the Sherlock step circuit, THEN the record SHALL NOT count as Sherlock end-to-end evidence (S1.1, R-1).

**S8 correction.**
- The owned interface is the ledger-based recursion route. No VerifyProof instruction is assumed necessary.
- New prerequisite: inspect and qualify that route against S8.1–S8.4.
- U-5 now reads "uninspected, to be inspected", not "unavailable". My earlier omission reflected what I was given, not a finding about the route.
- In §1.3, "No recursion" means no qualified recursion evidence yet.

## 7. Foreman, the trace postgate and phase 2

**Retracted:**
- C-6: "regex bug in the query, not in the model".
- D-1's framing that the phase-2 hold is an open decision.

**Corrected C-6 and D-1.**
- **Phase-1 trace postgate.** C-4 removes Foreman and its Pel plan only. It does not remove the existing phase-1 trace postgate and does not authorize phase 2.
- **Current state.** Phase 2 is NEVER_STARTED and adjudication is PENDING (Astra A1).
- **Whitespace mismatch.** This is ordinary data-reader diagnosis:
  - find the reader's comparison defect;
  - repair it;
  - re-read the existing captured trace within the existing command and counter limits.
- **Model involvement.** Whether the model is implicated is an outcome of that re-read, not an assumption.
- **No new layers.** There is no review-of-review and no added adjudication layer.

---

**Everything else in the original proposal stands.** That includes the three-path warning, the §3.2 positive-control rule and the mutation matrix.
