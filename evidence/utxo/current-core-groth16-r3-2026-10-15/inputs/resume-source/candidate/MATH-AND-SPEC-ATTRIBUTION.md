# Constraint arguments and specification attribution (source author's derivation)

Author: Claude Opus 5.5 (`claude-opus-5-5`), compiler source author. These are hand arguments about the authored source. Nothing was compiled, executed or machine-checked, and no independent reviewer has checked them yet.

Notation: `r` is the scalar modulus. `compile` refuses fields under `MIN_MODULUS_BITS = 198`, so `r ≥ 2^197`; BLS12-381 has a 255-bit `r`. "Bit-sum" means `Σ 2^i b_i` over witnesses each constrained `b·b = b`.

## 1. Representation lemmas

**L1 (UInt wire).** Every non-constant `UIntW` comes from `alloc_uint`: `lo` and `hi` are bit-sums of 64 bits each. Each satisfying assignment therefore has `lo, hi ∈ [0, 2^64)` as integers, and `value = lo + 2^64·hi ∈ [0, 2^128)`. Constants are canonical limbs. Every gadget output that is not a constant is again an `alloc_uint`, which preserves the invariant.

**L2 (Bool wire).** A `BoolW` is a booleanity-constrained witness, a constant, `1 - x`, a product `x·y` of Booleans, `x + y - xy`, `1 - x - y + 2xy`, or the `eq_uint` flag. Each is 0 or 1 whenever its inputs are. The `eq_uint` flag is forced by L5.

**L3 (integer equations).** A field equation between integer expressions whose difference has absolute value below `r` is an integer equation. Bounds used: Add `< 2^129`, Sub `< 2^129`, Mul right side `< 2^128 + 2^193`, Lt `< 2^130`, wide sum `< 2^144`. All are below `2^195 < r`.

## 2. Gadget correctness (satisfiable iff Core succeeds, with a unique output)

- **Add.** `a + b = c` with c range-proven. By L3, `c = a + b` over the integers, and `c < 2^128` holds iff `checked_add` succeeds. The output is unique.
- **Sub.** `b + c = a`, c range-proven. So `c = a - b ≥ 0` iff `checked_sub` succeeds. Unique.
- **Mul.** With limbs `a0, a1, b0, b1 < 2^64`: `p11 = a1·b1 = 0` (integer, since `< 2^128`), and `c = a0b0 + 2^64(a0b1 + a1b0)` (integer by L3). Then `c = a·b` and `c < 2^128`. Conversely, `a·b < 2^128` forces `a1 = 0` or `b1 = 0`, so the system is satisfiable exactly when `checked_mul` succeeds. A naive `a·b = c (mod r)` check with a 128-bit range on c accepts `a = 2^128 - 1`, `b = ⌈r/a⌉`, `c = a·b mod r` (AUTHOR-EXPECTATIONS A11; constants re-checked by integer arithmetic in this lane). This gadget does not.
- **Lt.** `f(2d + 1) = d - t` with `d = a - b`, f Boolean, t range-proven. `f = 0` gives `t = d`, so `a ≥ b`; `f = 1` gives `t = -d - 1`, so `a < b`. With the wrong f, t would have to be congruent to an integer in `[-2^129, -1]`, i.e. `t ≥ r - 2^129 > 2^128`, which L1 excludes. f and t are unique.
- **Eq (UInt).** `d·inv = 1 - f`, `d·f = 0`. `d ≠ 0` (mod r) forces `f = 0` and `inv = d⁻¹`; `d = 0` forces `f = 1` with inv free. `d ≡ 0` iff `a = b`, since `|d| < 2^128 < r`. f is unique. inv is unique except when `a = b`, where it does not affect any output.
- **Bool ops.** `x·y`, `x + y - xy` and `1 - x - y + 2xy` are And, Or and Eq on {0,1}; Not is `1 - x`.
- **Wide sum.** `Σ terms = lo + 2^64·mid + 2^128·top` with 64/64/16-bit bit-sums. At most 1,024 terms (`MAX_STATEMENTS`) below 2^128 sum below 2^138, so the limbs are unique (L3).

Consequence used by the tests: each gadget's witnesses are a deterministic function of its inputs whenever the system is satisfiable, up to the free `inv` when `a = b`. Internal wires are computed only from claimed inputs and pre-state. So if the honest synthesis of a claim is unsatisfied, no assignment satisfies that claim, apart from the free inv, which cannot change any output. This is why `tamper.rs` mutates claims and runs honest synthesis.

## 3. Statement-level correspondence (success relation)

For an admitted `/1` program `P`, an action `A` and a plan with represented cells `C` and registry duties `D`, the relation holds for a public statement `(h, x, s, s', e, cap?, acc?)` exactly when these conditions hold:

1. `h` equals the compiled header constants;
2. `x`, `s` and `s'` are well-typed canonical encodings, and absent duties have amount 0;
3. Core `simulate(P, A, scenario(s, x))` succeeds, where `scenario` has exactly the represented rows, with post snapshot `s'` restricted to `C ∪ D` and effect list `e`;
4. with a cap, `W ≤ cap`;
5. with accounting, `acc` equals the exact sums over `e`.

Argument sketch:

- **Direction ⇐.** Run the gadgets on Core's values; each is satisfiable by §2.
- **Direction ⇒.** Induct over statements. The environment, snapshot cells, duty cells and effect list after statement k equal Core's after step k (runtime.rs:702-912). Each statement's success conditions are exactly the gadget conditions on its reachable nodes plus its statement check. This holds because Core evaluates every reachable node (no short-circuit, runtime.rs:451-453) and charges each once per statement (runtime.rs:399-428).
- **Bounds.** Scenario-level refusals (runtime.rs:528-667) cannot occur. Bounds are checked at compile time over `C ∪ D`, with every registry duty counted as present. Types are fixed by slots. Every touched cell is represented, and identities are registry constants.
- **Work.** A successful action charges exactly `W = Σ(1 + |reach(root)|)` (stage/cost.rs:1-8). Metered success is therefore unmetered success plus `W ≤ cap` (policy.rs:37-53).

**Premise (target obligation).** Rows outside `C ∪ D` are not in the statement. Core never changes them, but they count toward the 65,536 trace bound. The relation's claim is about the represented snapshot only.

## 4. Specification attribution

| Requirement | Source | Implemented in |
|---|---|---|
| Profile routing and `/1` admission | parser.rs:5-161, 352-377; runtime.rs:120-322; stage/profile.rs:10-23 | frontend.rs |
| `/2` admission | stage/mod.rs:71-88 | frontend.rs, staged.rs |
| Program identity = SHA-256(`intent::canonical`) | intent.rs:69-88; export.rs:22; stage/digest.rs:20-22 | identity.rs |
| Raw source identity separate | export.rs:27; source_sponsor.rs:153 | identity.rs |
| Strict evaluation, per-statement memo, charge per node | runtime.rs:392-484 | lower.rs |
| Checked UInt arithmetic | runtime.rs:455-469 | gadgets.rs |
| Comparisons, Bool ops | runtime.rs:447-450, 470-477 | gadgets.rs |
| Duty reads incl. present-zero | runtime.rs:430-441 | lower.rs |
| Positional Requires/Ensures | runtime.rs:721-734 | lower.rs |
| Next rebinding | runtime.rs:735-746 | lower.rs |
| Emit | runtime.rs:747-754 | lower.rs, statement.rs |
| Transfer/Fee, debit before credit, self-transfer | runtime.rs:668-700, 755-786 | lower.rs |
| Liability accrual, present-zero creation | runtime.rs:787-836 | lower.rs |
| Discharge | runtime.rs:837-860 | lower.rs |
| Source duty registry, conflicts | runtime.rs:325-372, 497-527 | plan.rs |
| Scenario bounds and order | runtime.rs:528-667 | plan.rs, statement.rs |
| Logical work W | runtime.rs:408-428, 710; logical_cost.rs:33-55; stage/cost.rs:1-68 | plan.rs |
| Gross debit and fees (owner-intent arithmetic) | intent.rs:286-331 | statement.rs, circuit.rs (accounting) |
| Projection order and absent `(false, 0)` | projection.rs:46-65, 338-375 | statement.rs |
| Atomic Emit refusal (target) | profile.rs:11-13 | adapter.rs |
| Staged segments, costs and NotAdvanced | stage/cost.rs:70-162; stage/admit.rs:83-214; runtime.rs:934-1021 | staged.rs |
| R7 boundary predicates | stage/decode.rs:87-227 | STAGED-LOWERING-PLAN.md (future) |

## 5. Library facts relied on (UNVERIFIED-LIB: recalled, not inspected)

These are ark-relations `=0.4.0` and ark-ff `=0.4.2` facts. Nothing was compiled, so the risk that one is wrong is real:

- `LinearCombination(pub Vec<(F, Variable)>)`; `Variable: Ord`.
- `ConstraintSystemRef::{new_input_variable, new_witness_variable, enforce_constraint, num_constraints, is_in_setup_mode, set_mode, set_optimization_goal, finalize, to_matrices, is_satisfied}`.
- In setup mode the witness closures are not evaluated.
- `ConstraintMatrices{num_instance_variables, num_witness_variables, num_constraints, a, b, c}`.
- `PrimeField::{MODULUS, MODULUS_BIT_SIZE, into_bigint}`; `BigInteger::to_bytes_le`; `From<u128>` for field elements.
- ark-groth16 `=0.4.0` setup synthesizes under `OptimizationGoal::Constraints` and finalizes. If that is wrong, `r1cs_sha256` would not describe the matrices that setup uses.
- `Groth16<E>: SNARK` with `type Error = SynthesisError`; `VerifyingKey.gamma_abc_g1`.
