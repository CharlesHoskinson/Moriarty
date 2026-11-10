# Independent Circuit Audit: Sherlock Self-Recursive IVC Node `SHKIVC06`

**Target:** `SHKIVC06` in [`crates/mori-recursion/src/ivc.rs`](../../../crates/mori-recursion/src/ivc.rs), [`crates/mori-recursion/tests/ivc_node_mock.rs`](../../../crates/mori-recursion/tests/ivc_node_mock.rs), [`crates/mori-recursion/tests/ivc_cost.rs`](../../../crates/mori-recursion/tests/ivc_cost.rs), Task 2.3 of [`openspec/changes/sherlock-vm-consensus-20261107/tasks.md`](../../../openspec/changes/sherlock-vm-consensus-20261107/tasks.md#L40), and [`clean/fork/Clean/Sherlock/Aggregation.lean`](../../../clean/fork/Clean/Sherlock/Aggregation.lean) (`AccAlgebra.nodeFold`, `genesis_collapse_both`, `non_genesis_keeps_both`, `src004_height_bridge`, `lin_refines_chain`).  
**Baseline Comparison:** [`qa/gemini-quint/ivc/ivc_circuit.qnt`](../ivc/ivc_circuit.qnt) and [`qa/gemini-quint/ivc/REPORT.md`](../ivc/REPORT.md) (`SHKIVC05` audit).  
**Deliverables:**
- [`qa/gemini-quint/ivc06/ivc06.qnt`](./ivc06.qnt) — Quint specification of `SHKIVC06` constraints and Lean reference semantics.
- [`qa/gemini-quint/ivc06/ivc06_test.qnt`](./ivc06_test.qnt) — 10-test verification suite covering completeness, Lean bridge lemmas, all 5 adversarial witness classes, `nodeFold` equivalence, and 5 seeded circuit mutants.

---

## 1. Overall Verdicts: SOUND and COMPLETE

### 1.1 SOUND Verdict: **SOUND** (for `SherlockIvcRelation::circuit` paired with final accumulator verification)

Every witness `(inst, w)` accepted by `SherlockIvcRelation::circuit` (`crates/mori-recursion/src/ivc.rs:456-565`) and the final accumulator pairing check (`SherlockIvcRelation::check_acc` at `ivc.rs:350-356` / `RunDeciderRelation::circuit` at `crates/mori-recursion/src/run_decider.rs:206, 276`) satisfies the reference semantics (`AccAlgebra.nodeFold`, `src004_height_bridge`, and `lin_refines_chain` in `clean/fork/Clean/Sherlock/Aggregation.lean:1201-1502`):

1. **Height and Step Range Contract (`SRC-004`, `constrain_heights` at `ivc.rs:363-376, 497` $\leftrightarrow$ `src004_height_bridge` at `Aggregation.lean:1201-1206`):**
   - `std.assert_lower_than_fixed(l, prev_h, &h_bound)` (`ivc.rs:370`, with `h_bound = 1 << 64` from `H_BITS = 64` at `ivc.rs:79, 369`) constrains the canonical representative of `prev_h` in $\mathbb{F}_r$ to $[0, 2^{64})$. By `lower_than_fixed_correct` (`Aggregation.lean:1511-1525`), this rejects any negative/wrapped field element such as `prev_h = -2` (`r - 2`, `ivc_node_mock.rs:458`).
   - `let steps_m1 = std.add_constant(l, steps, -F::ONE)` (`ivc.rs:371`) followed by `std.assert_lower_than_fixed(l, &steps_m1, &BigUint::from(S_MAX))` (`ivc.rs:372`, with `S_MAX = 16` at `ivc.rs:77`) constrains $(\text{steps} - 1) \bmod r \in [0, 16)$, which holds in $\mathbb{F}_r$ iff $\text{steps} \in [1, 16]$. In particular, $\text{steps} = 0$ maps to $\text{steps\_m1} = r - 1 \ge 2^{64} > 16$ (refused), and $\text{steps} = 17$ maps to $\text{steps\_m1} = 16 \not< 16$ (refused). Because $S_{\max} = 16 < 2^{64}$, this single check discharges all three step hypotheses (`hsr : steps < 2^64`, `hs1 : 1 <= steps`, `hsS : steps <= S`) of `src004_height_bridge` (`Aggregation.lean:1201-1203`).
   - `let h = std.add(l, prev_h, steps)` (`ivc.rs:373`) followed by `std.assert_lower_than_fixed(l, &h, &h_bound)` (`ivc.rs:374`) constrains $h = (\text{prev\_h} + \text{steps}) \bmod r < 2^{64}$. Because $\text{prev\_h} < 2^{64}$, $\text{steps} < 2^{64}$, and $2^{65} \le r$ (BLS12-381 scalar field has 255 bits), `field_sum_exact` (`Aggregation.lean:1185-1187`) and `src004_height_bridge` (`Aggregation.lean:1201-1206`) guarantee that no modular wrap occurs: $h = \text{prev\_h} + \text{steps} \in [1, 2^{64})$, $\text{prev\_h} < h$, and $1 \le h - \text{prev\_h} \le S_{\max}$.
2. **Genesis Detection and Prefix-Chain Refinement (`ivc.rs:498-504, 522-538` $\leftrightarrow$ `lin_refines_chain` at `Aggregation.lean:1454-1485`):**
   - `let is_genesis = std.is_zero(l, &prev_h)` (`ivc.rs:498`) and `let not_genesis = std.not(l, &is_genesis)` (`ivc.rs:499`) constrain `is_genesis` $\in \{0,1\}$ with $\text{is\_genesis} = 1 \iff \text{prev\_h} = 0$.
   - Because every accepted node proof publishes $h = \text{prev\_h} + \text{steps} \ge 1$ (`LinCert.h_ne_zero`, `Aggregation.lean:1413-1419`), an accepted predecessor statement never has height $0$ (`lin_certified_prev_not_genesis`, `Aggregation.lean:1425-1427`), and because `constrain_heights` forbids modular wrap-around to $0$, `is_genesis == 1` holds only at true genesis (`prev_h = 0`).
   - `let chosen = std.select(l, &is_genesis, &init_d, &prev_d); std.assert_equal(l, &chosen, &prev_d)` (`ivc.rs:500-501`) enforces `prev_d == init_d` when `prev_h == 0`, matching `prev.h = 0 -> prev.cur = prev.init` in `linNode` (`Aggregation.lean:1381-1385`).
   - At non-genesis (`prev_h != 0`, `not_genesis = 1`), `prev_pi` (`ivc.rs:522-529`) binds `[self_vk_repr, prog_d, init_d, prev_d, prev_h, prev_acc_pi...]` into `verifier.prepare` (`ivc.rs:531-537`), and `leaf_inst` (`ivc.rs:549`) binds `[binding, prog_d, prev_d, cur_d, rule, prev_h, steps]` into `verify_child_incircuit_with` (`ivc.rs:550-559`). By `lin_refines_chain` (`Aggregation.lean:1454-1485`) and `lin_sound` (`Aggregation.lean:1488-1491`), every accepted linear node statement `(prog_d, init_d, cur_d, h)` refines the F21 segment `(prog_d, init_d, cur_d, 0, h)` certified by `chainOf` of its leaves.
3. **Genesis Neutrality and Collapsed-Carry Constraints (`SRC-005` Realization (a), `ivc.rs:383-421, 517-564` $\leftrightarrow$ `genesis_collapse_both` & `non_genesis_keeps_both` at `Aggregation.lean:1260-1310`):**
   - At genesis (`is_genesis = 1`, `not_genesis = 0`), `proof_acc` is scaled by `0` (`ivc.rs:538`) and collapsed (`ivc.rs:546`) to `trivial_acc`, while `constrain_genesis_neutral` (`ivc.rs:383-400, 520`) forces every public-input lane of `prev_acc` to equal `trivial_acc` (`is_genesis * (lane - neutral) == 0`). Thus both `proof_acc` and `prev_acc` equal `trivial_acc` entering `verifier.accumulate` (`ivc.rs:562`), closing the `CEX-3` genesis carry defect-cancellation / transcript-grinding vector from `SHKIVC05` and realizing `genesis_collapse_both` (`Aggregation.lean:1260-1278`).
   - Off genesis (`is_genesis = 0`, `not_genesis = 1`), `constrain_collapsed_carry` (`ivc.rs:409-421, 521`) forces both variable-base scalar lanes of `prev_acc` (`carry_scalar_lanes()` at `ivc.rs:239-248`) to `F::ONE`, ensuring `collapse(prev_acc) == prev_acc` and realizing `non_genesis_keeps_both` (`Aggregation.lean:1290-1310`).

---

### 1.2 COMPLETE Verdict: **COMPLETE**

Every honest genesis step (`prev_h = 0`, `prev_d = init_d`, `prev_acc = trivial_acc()`, `1 <= steps <= S_MAX = 16`, valid leaf proof) and every honest non-genesis step (`1 <= prev_h < 2^64 - steps`, `1 <= steps <= S_MAX = 16`, valid predecessor node proof with its collapsed output accumulator `prev_acc`, valid leaf proof) is accepted by both `SherlockIvcRelation::next_acc` (`ivc.rs:277-300`) and `SherlockIvcRelation::circuit` (`ivc.rs:456-565`), and its folded accumulator passes `SherlockIvcRelation::check_acc` (`ivc.rs:350-356`):

1. **Honest Heights Pass `constrain_heights` (`ivc.rs:363-376`):**
   For any integer $0 \le \text{prev\_h} < 2^{64}$ and $1 \le \text{steps} \le 16$ with $\text{prev\_h} + \text{steps} < 2^{64}$, `prev_h < 2^64` holds, $\text{steps} - 1 \in \{0, \dots, 15\} < 16$ holds, and $h = \text{prev\_h} + \text{steps} < 2^{64}$ holds. Verified by `ivc_node_genesis_mock` (`steps = 2`, `ivc_node_mock.rs:230-246`), `ivc_node_genesis_steps_bound` (`steps = S_MAX = 16`, `ivc_node_mock.rs:254-255`), and `ivc_node_step1_negatives` (`prev_h = 2, steps = 3 -> h = 5`, `ivc_node_mock.rs:406-418`).
2. **Honest Genesis Carry Passes `constrain_genesis_neutral` and `constrain_collapsed_carry` (`ivc.rs:200-210, 383-421`):**
   At genesis, the honest witness supplies `rel.trivial_acc()` (`ivc.rs:200-210`), which has public encoding `neutral` (so `x - t == 0` on every lane in `constrain_genesis_neutral`, `ivc.rs:397`) and variable-base scalars `F::ONE` on both `lhs` and `rhs` (so `constrain_collapsed_carry` holds, `ivc.rs:419`).
3. **Honest Non-Genesis Carry Passes `constrain_collapsed_carry` (`ivc.rs:345, 563`):**
   Every predecessor node step ends with `next.collapse(l, bls, scalar)` in-circuit (`ivc.rs:563`) and `acc.collapse()` off-circuit (`ivc.rs:345`), which produces an accumulator with 1 variable base and scalar `F::ONE` per side (`is_collapsed_carry == true`, `ivc.rs:252-261`). At non-genesis (`is_genesis = 0`), `constrain_genesis_neutral` multiplies `x - t` by `g = 0` (`ivc.rs:397`), which evaluates to `0` for any `prev_acc`.
4. **In-Circuit and Off-Circuit Fold Parity (`ivc.rs:316-346` vs `ivc.rs:531-564`):**
   At genesis (`prev[3] == F::ZERO`), `next_acc_raw` (`ivc.rs:317-318`) uses `self.trivial_acc()` for `proof_acc`, while `circuit` (`ivc.rs:538, 546`) scales `proof_acc` by `not_genesis = 0` and collapses it to the identical encoding `trivial_acc()`. At non-genesis, both prepare `prev_proof` and call `.collapse()` (`ivc.rs:333` and `ivc.rs:546`), leave `leaf_acc` uncollapsed (`ivc.rs:341-342` and `ivc.rs:557-558`), fold `[proof_acc, prev_acc, leaf_acc]`, and collapse the result (`ivc.rs:344-345` and `ivc.rs:562-563`).

---

### 1.3 Per-Constraint Verdict Summary

| Constraint / Function | Rust Source (`file:line`) | Lean Reference (`Aggregation.lean`) | Verdict | Justification |
|---|---|---|---|---|
| **C1. Relation tag `MAGIC = b"SHKIVC06"`** | `ivc.rs:64, 572, 581` | — | **SOUND & COMPLETE** | Domain-separates `SHKIVC06` serialized relations from `SHKIVC05` and earlier keys. |
| **C2. `constrain_heights` (`prev_h < 2^64`, `steps - 1 < S_MAX`, `h = prev_h + steps < 2^64`)** | `ivc.rs:77-79, 363-376, 497` | `src004_height_bridge` (`:1201-1206`), `lower_than_fixed_correct` (`:1511-1525`), `linNode_field_height` (`:1495-1502`) | **SOUND & COMPLETE** | Enforces $0 \le \text{prev\_h} < 2^{64}$, $1 \le \text{steps} \le 16$, and $h = \text{prev\_h} + \text{steps} < 2^{64}$ with no modular wrap (`h >= 1`). |
| **C3. `is_genesis`, `not_genesis`, and `init_d == prev_d` selector** | `ivc.rs:498-501` | `linNode` (`:1381-1385`), `lin_certified_prev_not_genesis` (`:1425-1427`), `lin_refines_chain` (`:1454-1485`) | **SOUND & COMPLETE** | With C2 preventing height wrap to $0$, `is_genesis = is_zero(prev_h)` fires exclusively at step 0 and forces `prev_d == init_d`. |
| **C4. `constrain_genesis_neutral` (`is_genesis * (lane - neutral) == 0`)** | `ivc.rs:231-234, 383-400, 518-520` | `genesis_collapse_both` (`:1260-1278`) | **SOUND & COMPLETE** | Realizes `SRC-005` (a): at genesis, every public lane of `prev_acc` is constrained equal to `trivial_acc()`. |
| **C5. `constrain_collapsed_carry` (`lanes[idx] == F::ONE` for `idx = [P, 2P+1]`)** | `ivc.rs:239-261, 409-421, 521` | `AccAlgebra.nodeFold` (`:1248-1252`), `non_genesis_keeps_both` (`:1290-1310`) | **SOUND & COMPLETE** | Forces `lhs` and `rhs` variable-base scalars of `prev_acc` to `1` at every height so `collapse(prev_acc) == prev_acc`. |
| **C6. `proof_acc` `scale_by_bit(&not_genesis)` + `proof_acc.collapse()`** | `ivc.rs:531-546` | `collapse_scale_false` (`:1238`), `scale_true` (`:1234`) | **SOUND & COMPLETE** | Collapses zero-scaled `proof_acc` at genesis to `trivial_acc()` and preserves `collapse(proof_acc)` off genesis. |
| **C7. Leaf verification & final fold (`[proof_acc, prev_acc, leaf_acc]`)** | `ivc.rs:549-564` | `AccAlgebra.nodeFold` (`:1248-1252`), `lin_step_link` (`:1439-1443`) | **SOUND & COMPLETE** | Links leaf `[binding, prog_d, prev_d, cur_d, rule, prev_h, steps]`, folds and collapses `[proof_acc, prev_acc, leaf_acc]`, and publishes all lanes. |

---

## 2. Does the Collapsed-Carry Constraint Make the Circuit Equal `nodeFold`?

**Answer: YES — on every witness accepted by `SherlockIvcRelation::circuit`, the combination of `AssignedAccumulator::assign(..., 1, 1, &[], &self.self_base_names(), ...)` (`ivc.rs:507-516`), `constrain_genesis_neutral` (`ivc.rs:383-400, 520`), and `constrain_collapsed_carry` (`ivc.rs:409-421, 521`) makes the circuit's accumulator fold identical to Lean's `AccAlgebra.nodeFold` (`Aggregation.lean:1248-1252`), while avoiding a second ~239,577-row in-circuit variable-base MSM (`tasks.md:40`).**

### 2.1 Algebraic Proof by Cases (`notGenesis = false` and `notGenesis = true`)

In `clean/fork/Clean/Sherlock/Aggregation.lean:1248-1252`, `AccAlgebra.nodeFold` is defined as:
$$\text{nodeFold}(\text{notGenesis}, p, c, l) = \text{collapse}\!\left(\text{accumulate}\!\left([\text{collapse}(\text{scale}(\text{notGenesis}, p)),\; \text{collapse}(\text{scale}(\text{notGenesis}, c)),\; l]\right)\right).$$

In `crates/mori-recursion/src/ivc.rs:538, 546, 562-563`, the circuit computes:
$$\text{circuitFold}(\text{notGenesis}, p, c, l) = \text{collapse}\!\left(\text{accumulate}\!\left([\text{collapse}(\text{scale}(\text{notGenesis}, p)),\; c,\; l]\right)\right).$$

Because the first element $\text{collapse}(\text{scale}(\text{notGenesis}, p))$, the third element $l$, and the outer `collapse(accumulate(...))` are identical in both expressions, `circuitFold` equals `nodeFold` on a witness $(\text{notGenesis}, p, c, l)$ if and only if the second input to `accumulate` has the exact same structural encoding:
$$c = \text{collapse}(\text{scale}(\text{notGenesis}, c)).$$

Let us check both values of the boolean bit $\text{notGenesis} = 1 - \text{is\_genesis}$ (`ivc.rs:498-499`):

1. **Case 1: Genesis (`is_genesis = 1`, `notGenesis = false`):**
   - By `AccAlgebra.collapse_scale_false` (`Aggregation.lean:1238`), the right-hand side is $\text{collapse}(\text{scale}(\text{false}, c)) = \text{trivial\_acc}$.
   - In the circuit, `constrain_genesis_neutral` (`ivc.rs:383-400, 520`) enforces $1 \cdot (\text{prev\_acc\_pi}[i] - \text{neutral}[i]) = 0$ for every lane $i$ of `prev_acc_pi = verifier.as_public_input(l, &prev_acc)` (`ivc.rs:517`), where `neutral = as_public_input(&self.trivial_acc())` (`ivc.rs:518-519`).
   - Because `prev_acc` is assigned with fixed shape `(1, 1, &[], &self.self_base_names())` (`ivc.rs:507-516`), its public-input lanes completely determine its `lhs` point (`C::default()`), `lhs` scalar (`F::ONE`), `rhs` point (`C::default()`), `rhs` scalar (`F::ONE`), and `self_base_names` fixed-base scalars (all `F::ZERO`).
   - Therefore, on every witness accepted at genesis, $c = \text{trivial\_acc} = \text{collapse}(\text{scale}(\text{false}, c))$, so:
     $$\text{circuitFold}(0, p, c, l) = \text{collapse}(\text{accumulate}([\text{trivial\_acc}, \text{trivial\_acc}, l])) = \text{nodeFold}(\text{false}, p, c, l),$$
     matching `genesis_collapse_both` (`Aggregation.lean:1260-1263`).
   - *Domain restriction note:* In Lean, `nodeFold false p c l` is a total function of `c` that maps even a non-neutral `c` to `collapse(accumulate([trivial, trivial, l]))` (`genesis_witness_independent`, `Aggregation.lean:1281-1285`), whereas the circuit implements `SRC-005` Realization (a) and **refuses** any witness with $c \neq \text{trivial\_acc}$ at genesis. On the domain of witnesses accepted by the circuit, the two functions are identical.

2. **Case 2: Non-Genesis (`is_genesis = 0`, `notGenesis = true`):**
   - By `AccAlgebra.scale_true` (`Aggregation.lean:1234`), the right-hand side is $\text{collapse}(\text{scale}(\text{true}, c)) = \text{collapse}(c)$.
   - In `midnight-circuits`, `AssignedAccumulator::assign(l, bls, scalar, 1, 1, &[], &self.self_base_names(), ...)` (`ivc.rs:507-516`) assigns `c` with 1 variable base $B_{\text{lhs}}$ and 1 variable scalar $s_{\text{lhs}}$ on `lhs` (and empty `lhs` fixed-base scalars), and 1 variable base $B_{\text{rhs}}$, 1 variable scalar $s_{\text{rhs}}$, and `self_base_names` fixed-base scalars on `rhs`.
   - On such an accumulator, `collapse(c)` replaces each side's variable-base MSM with the single evaluated point $[s_{\text{lhs}} \cdot B_{\text{lhs}}]$ (resp. $[s_{\text{rhs}} \cdot B_{\text{rhs}}]$) and the single scalar $[1]$, leaving the `self_base_names` fixed-base scalars untouched.
   - `constrain_collapsed_carry` (`ivc.rs:409-421, 521`) asserts `prev_acc_pi[P] == F::ONE` ($s_{\text{lhs}} = 1$) and `prev_acc_pi[2P + 1] == F::ONE` ($s_{\text{rhs}} = 1$), where `carry_scalar_lanes()` returns `[pts / 2, pts + 1]` (`ivc.rs:239-248`).
   - Because $1 \cdot B_{\text{lhs}} = B_{\text{lhs}}$ and $1 \cdot B_{\text{rhs}} = B_{\text{rhs}}$ in the curve group, $s_{\text{lhs}} = 1 \land s_{\text{rhs}} = 1$ implies $\text{collapse}(c) = c$ in both group value and exact public-input encoding.
   - Therefore, on every witness accepted off genesis, $c = \text{collapse}(c) = \text{collapse}(\text{scale}(\text{true}, c))$, so:
     $$\text{circuitFold}(1, p, c, l) = \text{collapse}(\text{accumulate}([\text{collapse}(p), \text{collapse}(c), l])) = \text{nodeFold}(\text{true}, p, c, l),$$
     matching `non_genesis_keeps_both` (`Aggregation.lean:1290-1294`).

3. **Why `constrain_collapsed_carry` is Necessary (`T7` in `ivc06_test.qnt`, `tests/ivc_node_mock.rs:420-443`):**
   - Suppose `constrain_collapsed_carry` (`ivc.rs:521`) were omitted. Let `acc0` be a valid collapsed accumulator with non-identity base $B$ and scalar $1$, and let `carry` re-encode each side as $(\frac{1}{2} B, \text{scalar } 2)$.
   - Then $\text{collapse}(\text{carry}) = (2 \cdot \frac{1}{2} B, 1) = (B, 1) = \text{acc0}$, so `nodeFold(true, p, carry, l)` hashes `(B, 1)` into the Poseidon challenge of `accumulate` and outputs the same folded accumulator as for `acc0`.
   - However, `circuitFold(1, p, carry, l)` passes `carry` uncollapsed into `accumulate`, hashing $(\frac{1}{2} B, 2) \neq (B, 1)$ into the Poseidon transcript challenge (`ivc.rs:540-544`), so $\text{circuitFold}(1, p, \text{carry}, l) \neq \text{nodeFold}(\text{true}, p, \text{carry}, l)$!
   - `constrain_collapsed_carry` (`ivc.rs:409-421, 521`) rejects `carry` because its scalar lanes equal $2 \neq 1$.

---

## 3. Adversarial Witnesses & Counterexample Analysis

### 3.1 Adversarial Witnesses Against `SHKIVC06` (All Refused)

All five required adversarial witness classes are modeled in [`qa/gemini-quint/ivc06/ivc06.qnt`](./ivc06.qnt) and verified in [`qa/gemini-quint/ivc06/ivc06_test.qnt`](./ivc06_test.qnt):

1. **Adversarial Witness W-1 — Height Wrap (`attackHeightWrap` in `ivc06.qnt:357-382`, `adversarialHeightWrapRefusedTest` `T4` in `ivc06_test.qnt:112-160`):**
   - **Variant W-1a (Field wrap to $h = 0$ mid-chain, `tests/ivc_node_mock.rs:456-464` `"step1 wrap h=0"`):**
     Witness supplies `prev_h = -2` (`P - 2 = 255` in `ivc06.qnt`) and `steps = 2`, so `prev_h != 0` (`is_genesis = 0`), `1 <= steps <= S_MAX`, and `h = (prev_h + steps) mod P = 0` (which would allow the *next* step to set `is_genesis = 1` and drop the predecessor proof, cf. `unchecked_prev_wraps_to_genesis` at `Aggregation.lean:1211-1213`).
     *Refusal:* `std.assert_lower_than_fixed(l, prev_h, &h_bound)` (`ivc.rs:370`) rejects `prev_h = r - 2 >= 2^64` (`circuitOk == false`).
   - **Variant W-1b (Boundary overflow $h = 2^{64}$, `tests/ivc_node_mock.rs:445-454` `"step1 h=2^64"`):**
     Witness supplies `prev_h = 2^64 - 1` (`HMAX - 1 = 63` in `ivc06.qnt`, passing `prev_h < 2^64`) and `steps = 1` (passing `steps - 1 < S_MAX`), yielding `h = 2^64` (`HMAX = 64`), matching `weaker_pair_insufficient` (`Aggregation.lean:1179-1181`).
     *Refusal:* `std.assert_lower_than_fixed(l, &h, &h_bound)` (`ivc.rs:374`) rejects `h = 2^64` (`circuitOk == false`).
   - **Variant W-1c (Unreduced predecessor height `prev_h = P`, `CEX-1` from `qa/gemini-quint/ivc/REPORT.md`):**
     Witness at true step height `truePrevH = P` supplies `prevH = P`.
     *Refusal:* Rejected by `assertLowerThanFixed(rawPrevH, HMAX)` (`ivc06.qnt:173`, `circuitOk == false`).

2. **Adversarial Witness W-2 — Steps `0` and `17` (`S_MAX + 1`) (`attackSteps0And17` in `ivc06.qnt:385-395`, `adversarialSteps0And17RefusedTest` `T5` in `ivc06_test.qnt:163-178`):**
   - Matches `ivc_node_genesis_steps_bound` (`tests/ivc_node_mock.rs:248-260`).
   - For `leaf_steps = 0`, `steps_m1 = std.add_constant(l, steps, -F::ONE)` (`ivc.rs:371`) evaluates to $r - 1$ (`256` in `ivc06.qnt`), which fails `std.assert_lower_than_fixed(l, &steps_m1, &BigUint::from(S_MAX))` (`ivc.rs:372`, since $r - 1 \ge 16$).
   - For `leaf_steps = 17` (`S_MAX + 1`), `steps_m1` evaluates to $16$, which fails `assert_lower_than_fixed(..., 16)` (`ivc.rs:372`, since $16 \not< 16$).
   - Both are refused in-circuit (`circuitOk == false`) at genesis and mid-chain.

3. **Adversarial Witness W-3 — Non-Neutral Carried Accumulator at Genesis (`attackNonNeutralGenesisCarry` in `ivc06.qnt:399-428`, `adversarialNonNeutralGenesisCarryRefusedTest` `T6` in `ivc06_test.qnt:182-222`):**
   - Matches `ivc_node_genesis_non_neutral_carry` (`tests/ivc_node_mock.rs:262-290`).
   - At `prev_h = 0` (`is_genesis = 1`), the witness supplies a well-formed collapsed accumulator `carry` (`lhsScalar = 1, rhsScalar = 1`) whose variable bases are the curve generator $g$ (`lhsBase = 1, rhsBase = 1`) and whose first `self_base_names` scalar is `5` (`selfScalar = 5`), including the `CEX-3` variant where `carry.defect = -r * badLeaf.defect (mod P)` attempts to cancel an invalid genesis leaf.
   - *Refusal:* In-circuit, `constrain_genesis_neutral` (`ivc.rs:383-400, 520`) evaluates `d = is_genesis * (lane - neutral)` (`ivc.rs:397`), which is non-zero on the generator point lanes and the fixed-base scalar lane, failing `std.assert_zero(l, &d)` (`ivc.rs:398`, `circuitOk == false`). Off-circuit, `SherlockIvcRelation::next_acc` (`ivc.rs:282-284`) checks `w.prev.h == 0 && !self.is_neutral(&w.prev_acc)` and returns `Err(...)` (`hostOk == false`).

4. **Adversarial Witness W-4 — Non-Collapsed Carried Accumulator Mid-Chain (`attackNonCollapsedCarryMidChain` in `ivc06.qnt:431-438`, `adversarialNonCollapsedCarryMidChainRefusedTest` `T7` in `ivc06_test.qnt:229-251`):**
   - Matches `"step1 non-collapsed carry"` in `ivc_node_step1_negatives` (`tests/ivc_node_mock.rs:420-443`).
   - At `prev_h = 2, steps = 3`, the witness takes the honest collapsed step-0 accumulator `acc0` and re-encodes each side's MSM as `(base * 2^{-1}, scalar 2)` (`reencodeCarryScalar2` in `ivc06.qnt:310-316`).
   - Although `collapse(carry) == acc0`, `carry` has variable-base scalars `2 != 1` at `carry_scalar_lanes()` (`ivc.rs:239-248`).
   - *Refusal:* In-circuit, `constrain_collapsed_carry` (`ivc.rs:409-421, 521`) fails `std.assert_equal_to_fixed(l, x, F::ONE)` on both scalar lanes (`ivc.rs:419`, `circuitOk == false`). Off-circuit, `SherlockIvcRelation::next_acc` (`ivc.rs:285-289`) checks `!self.is_collapsed_carry(&w.prev_acc)` and returns `Err(...)` (`hostOk == false`).

5. **Adversarial Witness W-5 — Invalid Leaf (`attackInvalidLeaf` in `ivc06.qnt:444-452`, `adversarialInvalidLeafRefusedByPairingTest` `T8` in `ivc06_test.qnt:257-291`):**
   - Matches `ivc_node_genesis_invalid_leaf_pairing` (`tests/ivc_node_mock.rs:292-328`) and `ivc_node_step1_negatives` (`tests/ivc_node_mock.rs:358-395`).
   - A leaf proof generated for another post-state is presented against `leaf_inst = [binding, prog_d, prev_d, cur_d, rule, prev_h, steps]` (`ivc.rs:549`).
   - Because `verify_child_incircuit_with` (`ivc.rs:550-559`, `crates/mori-recursion/src/inner.rs:162-212`) performs deferred-pairing Plonk verification (`verifier.prepare` + `resolve_fixed_bases`), the node's in-circuit constraints are satisfied (`circuitOk == true`), and `leaf_acc` carries a non-zero KZG pairing defect (`rawLeafAcc.defect != 0`).
   - *Refusal:* By `genesis_collapse_both` (`Aggregation.lean:1260-1263`) at genesis and `non_genesis_keeps_both` (`Aggregation.lean:1290-1294`) mid-chain, the folded accumulator `next` inherits the non-zero KZG defect (`inst.acc.defect != 0`) and is **refused by the final pairing check** (`SherlockIvcRelation::check_acc` at `ivc.rs:350-356` / `RunDeciderRelation::circuit` at `run_decider.rs:276`, `deciderOk == false`).

---

### 3.2 Gaps Found: Circuit vs. Host Helper Divergences

No circuit-level soundness or completeness gap was found in `SherlockIvcRelation::circuit` (`crates/mori-recursion/src/ivc.rs:456-565`). However, comparing the off-circuit host methods (`next_acc` and `is_collapsed_carry` in `ivc.rs:252-300`) against the in-circuit constraints reveals **two host-level validation gaps** where the host helper accepts inputs that `SherlockIvcRelation::circuit` rejects:

#### Gap H1 — `SherlockIvcRelation::next_acc` omits host-side height/step and genesis-digest checks (`crates/mori-recursion/src/ivc.rs:277-300`)
- **Location:** `crates/mori-recursion/src/ivc.rs:277-300` vs. `crates/mori-recursion/src/ivc.rs:363-376, 497-501`.
- **Description:** `next_acc` checks `w.prev.h == 0 && !self.is_neutral(&w.prev_acc)` (`ivc.rs:282`) and `!self.is_collapsed_carry(&w.prev_acc)` (`ivc.rs:285`), but does **not** check:
  1. `1 <= w.leaf_steps && w.leaf_steps <= S_MAX` (`ivc.rs:371-372`),
  2. `w.prev.h.checked_add(w.leaf_steps).is_some()` (`h < 2^64`, `ivc.rs:373-374`), or
  3. `w.prev.h == 0 => w.prev.cur_d == w.prev.init_d` (`ivc.rs:500-501`).
- **Counterexample Witness (`W-H1`, reproduced in `T4`/`T5` of `ivc06_test.qnt`):**
  Let `w` be a genesis witness with `w.prev.h = 0`, `w.prev.cur_d = w.prev.init_d`, `w.prev_acc = rel.trivial_acc()`, and `w.leaf_steps = 17` (or `w.leaf_steps = 0`, or `w.prev.h = u64::MAX, w.leaf_steps = 1`).
  - `rel.next_acc(None, &w)` (`ivc.rs:277-300`) succeeds (`Ok(acc)`, `hostOk == true`).
  - `SherlockIvcRelation::circuit` (`ivc.rs:497`) rejects `w` in `constrain_heights` (`circuitOk == false`).
- **Impact:** Non-exploitable for proof soundness (the circuit enforces all three checks), but callers of `next_acc` on the host do not get an early `Err(...)` on out-of-range `leaf_steps`, `u64` height overflow, or `prev.cur_d != prev.init_d` at genesis.

#### Gap H2 — `SherlockIvcRelation::is_collapsed_carry` checks flattened public-input length rather than per-side base counts (`crates/mori-recursion/src/ivc.rs:252-261`)
- **Location:** `crates/mori-recursion/src/ivc.rs:252-261` vs. `crates/mori-recursion/src/ivc.rs:426-445, 507-516`.
- **Description:** `is_collapsed_carry(&self, acc: &Accumulator<S>)` checks only that `as_public_input(acc).len() == as_public_input(&self.trivial_acc()).len()` and that lanes `[pts / 2, pts + 1]` equal `F::ONE`. Because `AssignedAccumulator::<S>::as_public_input` concatenates `lhs` bases, `lhs` scalars, `lhs` fixed-base scalars, `rhs` bases, `rhs` scalars, and `rhs` fixed-base scalars without length prefixes or fixed-base key names, an off-circuit `Accumulator<S>` with mismatched per-side base counts can have the same total lane count `n`.
- **Counterexample Witness (`W-H2`):**
  Construct an off-circuit `Accumulator<S>` with `0` `lhs` variable bases, `2` `rhs` variable bases `[C::default(), C::default()]` with scalars `[F::ONE, F::ONE]`, `1` `lhs` fixed-base scalar equal to `F::ONE` (which lands at index `pts / 2 = 5`), and `self.self_base_names().len() - 1` `rhs` fixed-base scalars.
  - Its flattened `as_public_input` vector has length `0 + 0 + 1 + 10 + 2 + (K - 1) = 12 + K = n`, with `lanes[5] == F::ONE` (the `lhs` fixed-base scalar) and `lanes[11] == F::ONE` (the first `rhs` variable-base scalar).
  - `rel.is_collapsed_carry(&acc)` (`ivc.rs:252-261`) returns `true`, even though `acc` has `0` `lhs` bases and `2` `rhs` bases (`collapse(acc) != acc`)!
  - In-circuit, `AssignedAccumulator::<S>::assign(l, bls, scalar, 1, 1, &[], &self.self_base_names(), ...)` (`ivc.rs:507-516`) and `acc_from_lanes` (`ivc.rs:426-445`) enforce `lhs.bases.len() == 1`, `rhs.bases.len() == 1`, empty `lhs` fixed bases, and exact `self_base_names()` keys, so the circuit refuses it.

---

## 4. Quint Verification Commands

The model [`qa/gemini-quint/ivc06/ivc06.qnt`](./ivc06.qnt) and test suite [`qa/gemini-quint/ivc06/ivc06_test.qnt`](./ivc06_test.qnt) avoid the Quint built-in identifiers `fields`, `next`, and `fold`, and are structured for direct execution by the lead:

```bash
# 1. Typecheck the model and test suite
quint typecheck qa/gemini-quint/ivc06/ivc06.qnt
quint typecheck qa/gemini-quint/ivc06/ivc06_test.qnt

# 2. Run the 10 unit, adversarial-witness, nodeFold-equivalence, and mutant tests
quint test qa/gemini-quint/ivc06/ivc06_test.qnt --main ivc06_test --max-samples 100 --seed 0x2a

# 3. Run bounded invariant exploration across all honest and adversarial step actions
quint run qa/gemini-quint/ivc06/ivc06.qnt --main ivc06 --invariant allInvariants --max-steps 20 --max-samples 200 --seed 0x2a
```

## Host run (lead, 2026-11-09)

The lane could not run commands, so the lead ran Quint 0.x (inside heavy.slice) on the lane's model. One rename was needed: the built-in name `variant` became `wvariant` in both files.

- `quint typecheck ivc06.qnt`: pass.
- `quint test ivc06_test.qnt`: 10 of 10 pass (honest genesis and step 1; the Lean `genesis_collapse_both`, `non_genesis_keeps_both`, `src004_height_bridge` and `lin_refines_chain` shapes; height wrap, steps 0 and 17, non-neutral genesis carry, non-collapsed mid-chain carry and invalid leaf refused; collapsed carry makes the circuit equal `nodeFold`; seeded circuit mutants caught).
- `quint run ivc06.qnt --invariant allInvariants --max-samples 20000 --max-steps 12`: no violation (20,000 traces of 13 states).

This is model evidence from an independent lane. It is an input to task 2.3, not circuit evidence.
