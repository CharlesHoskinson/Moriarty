# astra-1-12: witness depth attack and draft audit

## Question (astra-1)
Check the same-VK idea from `astra-1/PROPOSAL.md` section 5: per 4-bit MSM window, compute all intermediate prefix sums with an associative group scan, then batch-normalize and batch-invert the lambda witnesses for the original gate order; four doublings remain between windows; tables are independent per base. Does it preserve every intermediate assigned point and `completeness_error_if` behaviour if original adjacency is tested explicitly? What about eager foreign BigUint and GLV decomposition? Give blockers and path:line evidence, no timing guesses. Correct earlier witness-depth and cost-model wording as needed.

Path abbreviations: `C` = midnight-circuits-7.2.4/src, `P` = midnight-proofs-0.8.3/src (both under `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`). `R` = recl9/src.

## Answer
1. **Intermediate points: yes, the group values are preserved.** Each intermediate point cell is the group sum or double of its two input points, computed by the library on values (`C/ecc/foreign/weierstrass_chip.rs:1144` `p+q`, then `assign_point_unchecked`, then `assert_add`). The affine point is unique, so any evaluation order that computes the same group elements yields the same assigned coordinates. An affine point is canonical; the limb encoding is canonical as long as it comes from `assign` of the group value (section 3 for the caveat).
2. **Completeness: yes, if the predicate is re-evaluated on the original adjacent pairs.** The original predicate is evaluated on `(acc_before_add, addend)` for every add (`:1900`, `completeness_error_if` defined at `:1012`), and in the table build on `(acc_k, p)` (`:1846-1849`). A scan computes the same prefix values, so the same pairs can be tested. The test must use the pairs in original order, including the accumulator after the four doublings and not before them.
3. **The scan does not need to be a generic prefix scan over the whole MSM.** Two facts make a much shallower schedule possible:
   - the addends (`multi_select` outputs, `:1886-1889` (approximate)) depend only on scalar bits and precomputed tables, not on the accumulator;
   - the accumulator recurrence is `acc_i = 16·acc_{i-1} + T_i`, where `T_i` is the window total.
   So the per-window adds can be filled in parallel after `acc_i^{start}` is known: `acc_i^{start} + P_{i,k}` for each prefix `P_{i,k}` of the addends in window `i`. The only sequential chain is the 4 doublings and 1 add per window.
4. **Depth correction.** My earlier `32·(4+2B)` (astra-1-2-followup and astra-1-7 section 4) is the depth of the circuit as written (`:1875-1909`), not a lower bound. The algebraic depth under this schedule is about `5W` group operations, with `W = 32` windows for GLV-halved 128-bit scalars: 160 operations, plus `O(log B)` for in-window prefix sums and 14 sequential adds per base for the tables (parallel across bases). This is a structural bound from the code, not a timing. The earlier "10^3–10^4" range should be read as the as-written depth only.
5. **Lambda batch inversion: yes, with one exact rule.** Each lambda depends only on its own input pair. Its denominator is `(q.x − p.x)` for an add, and `2·p.y` for a double. The code's special cases must be copied exactly:
   - add: `if p.0 == q.0 { ONE }` (`C/ecc/foreign/weierstrass_chip.rs:1266-1280`). Zero denominators are excluded from the batch and assigned ONE;
   - add with identity input: `ONE` (same block);
   - double: identity gives `ONE`; otherwise `(2·p.y).invert().unwrap()` (`:1182-1190`). **This `unwrap` panics if `y = 0`** (a 2-torsion point). Whether such a point reaches a double is not established here: doubles act on accumulators, not on decoded proof points, but this needs a check (see blockers).
6. **GLV decomposition and BigUint limbs are value-only and parallel.** GLV runs in a closure on each scalar value (`:1973-1977`, `glv_split` at `:1958`; algorithm `C/utils/util.rs:110-`), independent of the accumulator. Limb decomposition runs per assignment (`C/field/foreign/field_chip.rs:430-434`, `C/biguint/biguint_gadget.rs:745-751`). Both can be computed for all bases and scalars in parallel. Exact equality needs the same algorithm (GLV) and the same canonical limb convention.

## Hardened-copy caveat (applies to every citation above)
- The Sherlock parent is built against the Fenrir-hardened circuits copy, not the stock registry 7.2.4: `R/Cargo.toml:36` `[patch.crates-io] midnight-circuits = { path = "../verifier-patch/midnight-circuits" }` (stock pin at `R/Cargo.toml:27`).
- Diff against stock (`diff -rq` over `src/`): only `src/ecc/foreign/weierstrass_chip.rs` differs. Three hunks:
  1. `mul_by_u128` (`verifier-patch/midnight-circuits/src/ecc/foreign/weierstrass_chip.rs:1652-1686`): `guard_x = C::COFACTOR > 1` (true for BLS G1, `:1675`); normalizes `p.x` if not well-formed.
  2. `mul_by_u128` loop (`:1718-1723`): `incomplete_assert_different_x(acc, tmp)` before every incomplete add when `guard_x`. This adds constraints and witness inversions on every add in the cofactor and `l_times_r` ladders.
  3. `msm_randomness` (`:1750-1770`): the shift `r` is assigned with the checked `assign` (`:1766`), not `assign_without_subgroup_check`. For BLS G1 this runs the cofactor chain (`mul_by_constant` by the cofactor, 125 doublings plus adds) on the random shift.
- Consequences: (a) the cofactor chain is now on the MSM path via `r` (`neg_alpha = -(2^WS-1)·r` at `msm_randomness`, then the table precompute depends on `neg_alpha`). It does not depend on the transcript, so it can overlap the sponge. (b) The stock-path claim "no subgroup check on the shift" is not the hardened VK. (c) Hardened line numbers after `:1686` are shifted: `msm_randomness` stock `:1726` → `:1750` (+24); `windowed_msm` stock `:1771` → `:1803` (+32); `nb_iterations` stock `:1875` → `:1907` (+32); `r_correction` stock `:1909` → `:1941` (+32). Lines before `:1652` (`incomplete_add` `:1138`, `assert_double` `:1174`, `assert_add` `:1257`, the inversion unwraps at `:1189` and `:1276`) are unchanged. Native-chip citations (`C/field/native/native_chip.rs`) are unchanged in the hardened copy.
- Re-derive every weierstrass citation against `verifier-patch/midnight-circuits` before any implementation or audit claim.

## Blockers and pathlines
- **B1 (exceptional cases in parallel).** The original sequence checks every adjacent pair. A parallel fill must compute the same predicates on the same pairs. Any pair where `acc == ±addend` (`:1900`, `:1846-1849`) is an error in both schedules. Identity intermediates are also errors (`incomplete_add` asserts `p.is_id == r.is_id`, `:1148`). A parallel run must not "repair" them.
- **B2 (double-unwrap panic).** `C/ecc/foreign/weierstrass_chip.rs:1182-1190` `(2·p.y).invert().unwrap()`. A parallel batch must map zero to a defined error. Changing a panic into `Error` is a failure-mode change; document it. Reachability from proof bytes is not established.
- **B3 (cell identity, not value, drives shortcuts).** `AssignedField` equality compares limb cells (`C/field/foreign/field_chip.rs:100-106`), and `mul` shortcuts (`:899-926`) use it. So the schedule must keep the same fixed `zero`/`one` cells, or the shortcut structure changes. This is structural, not value-based.
- **B4 (foreign-field representatives).** Group values are unique, but arithmetic witnesses inside `assert_add`/`assert_double` (quotients and remainders for foreign-field `mul`/`sub`, limb carries) may use any representative within bounds (`C/field/foreign/field_chip.rs:61-66`: limbs can wrap around, lower bounds may be negative). Exact-cell equality therefore needs the same representative choice. Group-value equality does not guarantee it. This is the real exact-cell blocker, not the scan.
- **B5 (random point).** `l_times_r = mul_by_u128(B, r)` depends on the OsRng point `r` (`:1832`, `:994`). Same as `astra-1-7`: exact-cell equality requires an injected point.
- **B6 (order-dependent state).** `tag_cnt` (`:1517-1519`, `:1835-1836`) and `msm_randomness` (`:1726-1751`) are RefCell state. A schedule that changes the order of table loads or randomness draws changes tags and therefore the VK. Keep the call sequence of structural operations fixed (as `astra-1-7` section 2).
- **B8 (recipe extraction boundary).** A recipe cannot be read from `Assignment`: `assign_advice` receives a closure whose value is already evaluated (`P/src/plonk/prover.rs:845-872`, `:871`), so no opcode or input DAG is visible. Extraction needs typed instrumentation (a `RecipeValue` in place of `F` in the chip and field operations at `weierstrass_chip.rs:1138-1157`, `:1174-`, `:1257-`, `field_chip.rs:899-926`) or a specialized witness generator. The scan schedule above is a schedule over such a recipe, not over the current closures.
- **B7 (identity-result case in the sum).** `r_correction` and the final add (`:1909`) combine `acc` with `-l_times_r`. A zero-sum output sets `is_id`; the circuit handles it through `is_id` assignments, which must be reproduced.

## Eager BigUint and GLV (exactness)
- GLV: `glv_scalar_decomposition` (`C/utils/util.rs:110-`) uses BigInt extended Euclid with per-call Vecs. Deterministic, so it can run in parallel over scalars. Exact equality needs a port of the same algorithm and output conventions; a different valid decomposition gives different cells that still verify.
- BigUint limbs: `big_to_limbs` (`C/field/foreign/util.rs:130-140`), per assignment. Deterministic. Canonical limbs from `assign` mean the same cells regardless of who computes them.
- Neither feeds the accumulator except through values, so both are movable without changing the chain.

## Corrections to earlier wording
- `astra-1-2-followup-witness.md` section C and `astra-1-7.md` section 4: the `32·(4+2B)` depth is the depth of the circuit as written. The algebraic depth under this schedule is about `5W` (160 for W=32) plus `O(log B)`. "10^3–10^4" is not a bound.
- `astra-1-2-followup-witness.md` section A: "lower bound on structure only" should read: the cost model executes no value closures (`P/dev/cost_model.rs:526-560`), so its time is structure-only. It is neither a lower bound nor an upper bound on witness-closure time. Its 5.687 s is also not a per-pass figure; the number of structural passes is not recorded.
- `astra-1-7.md` section 4 critical path: the transcript sponge chain is unchanged; the MSM part shrinks from `32·(4+2B)` to about `5W`.

## Test for the scan (no build run here)
1. For a fixed random point and fixed scalars, compute the sequential reference (original order) and the scan schedule; compare every intermediate affine point (including is_id) and every lambda.
2. Adversarial adjacency: inputs with `acc = ±addend`, identity inputs, and equal x with different y. The scan must raise `CompletenessFailure` exactly when the sequential reference does.
3. Double with `y = 0` input (2-torsion): confirm whether it reaches `:1182-1190`; if it does, map to an error.
4. Limb equality: compare canonical limb cells of every intermediate point between the two schedules.

## Raw numbers table

| Item | Value | Status | Source |
|---|---|---|---|
| Windows W (GLV-halved 128-bit scalars, WS = 4) | 32 | CODE-derived | `:1875`, `:850` |
| As-written depth | 32·(4+2B) | CODE-derived; B not recorded | `:1875-1909` |
| Schedule depth | ~5W + O(log B) (~160 for W = 32) | derived from structure; no timing | this file |
| Table depth per base | 14 sequential adds, parallel across bases | CODE | `:1845-1856` |
| Add completeness check | `av == addv || av == -addv` | CODE | `:1900` (`:1012` def) |
| Add lambda special case | `if p.0 == q.0 { ONE }` | CODE | `:1266-1280` |
| Double lambda unwrap | `(2·p.y).invert().unwrap()` | CODE | `:1182-1190` |
| Cell-based equality | limb cells | CODE | `C/field/foreign/field_chip.rs:100-106` |

## Caveats
- No timing estimates are given. The depth figures are counts of group operations from the code's structure.
- Citations use stock `midnight-circuits-7.2.4`. The Sherlock parent uses the hardened copy (section above), which changes `msm_randomness` and `mul_by_u128`.
- The scan schedule is a proposal; I did not implement or run it. The equality claims are for group values and canonical limbs; exact cells for arithmetic witnesses (B4) remain open.
- Reachability of the double-unwrap panic (B2) from proof bytes is not established.
- No builds, proofs or repo changes were made.
