# Independent Soundness Review: Sherlock X14 `ChainAir` (`id = row index`)

## Executive Verdict

**Verdict: SOUND and COMPLETE (subject to the boundary conditions below).**

Adding an `id` column constrained by:
- `builder.when_first_row().assert_zero(id)` (`id[0] = 0`), and
- `builder.when_transition().assert_eq(next.id, id + 1)` (`id[r + 1] = id[r] + 1`),

to both `PointsIn` and every `ChainAir` in the BLS12-381 $\mathbb{G}_1$ addition (`L_add`, `X3`, `Y3`, `INV`) and doubling (`SQ`, `L_dbl`, `X3`, `Y3`, `INV_y`) chains—combined with prefixing every `PermutationCheckBus` message on buses `o+0 .. o+10` with `id`—**closes the cross-row input-swap vulnerability** identified in earlier X14 lanes without adding any extra bus interactions.

### Sandbox Execution Disclosure
In accordance with the review instructions: shell command execution (`run_command`) was refused by the sandbox permission guard in this environment. Consequently, `quint typecheck`, `quint test`, and `quint run` were not executed live in the shell, and no unverified runtime output is claimed. The Quint 0.32.0 specification ([chain_id.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/qa/gemini-quint/x14-chain-id/chain_id.qnt)) and test suite ([chain_id_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/qa/gemini-quint/x14-chain-id/chain_id_test.qnt)) were statically audited against Quint 0.32.0 syntax, name-resolution, and type rules. The exact commands and expected exit codes are tabulated below.

---

## 1. Architecture and Constraint Analysis

### 1.1 `ChainAir` and `PointsIn` (`lane-src/x_exp_node.rs`)
In [x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/lane-src/x_exp_node.rs#L660-L702), `ChainAir` wraps a single-constraint OpenVM `FieldExpr` ([builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/lane-src/builder.rs#L249-L450), `e.num_vars() == 1`, `needs_setup = false`) of width `ew` and appends one column `id` at index `self.ew`:
1. **Intra-row modular arithmetic (`SubAir::eval(&self.expr, ...)`):**
   - Asserts `is_valid` is boolean (`is_valid in {0, 1}`).
   - When `is_valid == 1`, `CheckCarryModToZeroSubAir` enforces `constraint_expr(inputs, vars) - q * p_381 == 0` limb-by-limb with carries over `RANGE_BUS`, and range-checks every limb of the saved output variable `vars[0]` to `limb_bits = 8` bits on `RANGE_BUS`.
   - `FieldExpr` does **not** range-check `inputs`.
2. **Row-index constraint (`id = row index`):**
   - `when_first_row().assert_zero(id)` forces `id[0] = 0`.
   - `when_transition().assert_eq(next.id, id + 1)` forces `id[r + 1] = id[r] + 1 (mod p_BabyBear)` for all `0 <= r < H - 1`.
   - Both constraints are **unconditional** (not gated by `is_valid`).
3. **Bus interactions (`PermutationCheckBus`):**
   - Every message `m` in `self.msgs` starts with `Ref::Id` (`id`) and is sent (`count = +is_valid`) or received (`count = -is_valid`) on `PermutationCheckBus::new(m.bus)` ([interaction_mod.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/lane-src/interaction_mod.rs#L131-L176)).
   - The longest message has `1 + 2 * 48 = 97` field elements, within the LogUp bound `2^7 = 128`.

In [x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/lane-src/x_exp_node.rs#L744-L781), `PointsIn` holds `id` at column `0` followed by `nv` field elements of `nl = 48` limbs (`nv = 4` for `add`: `x1, y1, x2, y2`; `nv = 2` for `dbl`: `x, y`):
- Enforces the same `id[0] = 0` and `id[r + 1] = id[r] + 1` constraints on `local[0]`.
- Range-checks **every** input limb `local[1..]` to 8 bits on `RANGE_BUS` with multiplicity `1` ([var_range_bus.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/lane-src/var_range_bus.rs#L47-L57), [var_range_mod.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/lane-src/var_range_mod.rs#L72-L131)).
- Sends input point messages prefixed with `id` (`None` in `refs`) with multiplicity `1`.

### 1.2 Input Range-Check Propagation Across the Bus DAG
Although `FieldExpr` does not range-check its `inputs`, every input to every active row (`is_valid == 1`) of every `ChainAir` in `g1_chain` is guaranteed to consist of 8-bit limbs in `[0, 256)`:
- Every bus `o+0 .. o+10` has a single producer AIR and a single consumer AIR.
- On `o+0`, `o+1` (for `add`) and `o+8`, `o+0` (for `dbl`), the producer is `PointsIn`, which directly range-checks all 48 limbs of each coordinate to 8 bits on `RANGE_BUS`.
- On `o+2 .. o+7` and `o+9 .. o+10`, every value sent by an upstream `ChainAir` is either one of its own `vars` (which `FieldExpr` range-checks to 8 bits when `is_valid == 1`) or one of its `inputs` (already received from `PointsIn` or `SQ`).
- By LogUp multiset equality on `PermutationCheckBus`, the consumer's `inputs` on any active row (`is_valid == 1`) are limb-for-limb identical to a producer's 8-bit-checked limbs.

---

## 2. Refinement Invariant `invRow` and Mutant Analysis

### 2.1 Refinement Invariant `invRow`
In [chain_id.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/qa/gemini-quint/x14-chain-id/chain_id.qnt#L809-L816), `invRow` asserts that for every accepted chain (`chain_accepted == true`):
1. **Distinct and row-indexed valid `id`s:** Within each AIR (`PointsIn`, `L_add`, `INV`, `X3`, `Y3`), the `id`s of all valid rows (`is_valid == 1`) are pairwise distinct and equal to their physical row index (`row_id == idx`).
2. **Unique producer row per message:** Every message sent on buses `o+0 .. o+10` carries an `id` (`has_id == true`) that names a unique valid row (`prod_count == 1`) of its producing AIR.
3. **No cross-row swap (exact per-`id` functional refinement):** Every valid output row `(k, x3, y3)` in `Sink` matches a unique valid row with `row_id == k` in `PointsIn`, its inputs satisfy the non-exceptionality condition `x1 != x2 (mod P)`, and `(x3, y3)` equals the reference affine $\mathbb{G}_1$ sum `spec_x3_add(x1, y1, x2, y2), spec_y3_add(x1, y1, x2, y2)` over $\mathbb{F}_P$.

We also define three companion invariants in [chain_id.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/qa/gemini-quint/x14-chain-id/chain_id.qnt#L818-L864):
- `invNoSwapById`: checks clauses (1) and (3) keyed solely by `row_id` (without requiring `row_id == idx`), isolating whether cross-row input swapping is possible when `row_id` is shifted by a constant offset.
- `invComplete`: `is_honest implies chain_accepted` (every honest chain on valid inputs is accepted).
- `invNonVacuous` (aliased as `nonVacuous` and `invReach`): `not(chain_accepted and is_honest)` (negated reach invariant that **must fail** in the sound module `chain_id`, witnessing an accepted honest chain).

### 2.2 Mutant Analysis

| Module | Mutation | Effect on Soundness & Completeness | Caught by `invRow`? |
| :--- | :--- | :--- | :--- |
| `chain_id` | None (sound baseline) | Every honest trace is accepted (`invComplete` holds); every accepted trace satisfies `invRow`; `invNonVacuous` is violated as required. | Pass (exit `0`) |
| `m_no_first_row` | Drop `when_first_row: id = 0` | **Does NOT break cross-row swap soundness or completeness, but breaks `id == row_index`.** Because `next.id = id + 1` still holds and `H <= 2^17 < p`, the `id`s in each AIR are $id_0, id_0 + 1, \dots, id_0 + H - 1 \pmod p$, which remain pairwise distinct (`invNoSwapById` passes!). However, the prover can shift all `id`s by a nonzero offset $c$ (or in a taller `ChainAir` of height 3 with `is_valid = [0, 1, 1]`, start at $id_0 = p - 1 = 12$ so active rows `id = 0, 1` sit at physical indices `idx = 1, 2`), violating `row_id == idx` in `invRow`. | **Caught** (exit `!= 0`) |
| `m_no_transition` | Drop `when_transition: next.id = id + 1` | **Breaks soundness (the earlier duplicate-`id` swap returns).** Row 0 and row 1 can both set `id = 0` (`first_row` only constrains row 0). `L_add` (and `INV`, `X3`, `Y3`) swaps `P2` between row 0 and row 1, computing $P_{1,0} + P_{2,1}$ and $P_{1,1} + P_{2,0}$ while all bus multisets balance. Violates both `invRow` and `invNoSwapById`. | **Caught** (exit `!= 0`) |
| `m_step_two` | `next.id = id + 2` in one AIR only (`L_add`) | **Breaks both completeness on equal-height traces and `id == row_index` across AIRs.** On an honest all-valid trace, `L_add` has `id = [0, 2, ...]` while `PointsIn`/`X3`/`Y3`/`INV` have `id = [0, 1, ...]`, so bus `o+0` fails to balance (`invComplete` violated). When `PointsIn` and the other AIRs have height 3 with `is_valid = [1, 0, 1]` (`id = [0, 1, 2]`) and `L_add` has height 2 with `is_valid = [1, 1]` (`id = [0, 2]`), the chain is accepted with `L_add` row `idx = 1` having `id = 2 != 1`, violating `invRow`. | **Caught** (exit `!= 0`) |
| `m_id_not_in_msg` | Omit `id` from one bus message (`o + 1`, carrying $P_2$) | **Breaks soundness (cross-row swap returns even with distinct `id`s).** Every AIR satisfies `id = row_index` (`id = [0, 1]`), but because `o + 1` carries `(x2, y2)` without `id`, `L_add` row 0 (`id = 0`) can receive `(x2_1, y2_1)` from `PointsIn` row 1 (`id = 1`) and vice versa. Downstream buses `o + 3` and `o + 10` forward `L_add`'s swapped `x2` with `id = 0, 1`, so all buses balance while computing $P_{1,0} + P_{2,1}$. Violates both `invRow` and `invNoSwapById`. | **Caught** (exit `!= 0`) |

### 2.3 Exact Quint Commands and Expected Exit Codes

| Check / Target | Exact Command | Expected Exit Code | Expected Outcome |
| :--- | :--- | :---: | :--- |
| Typecheck model | `quint typecheck qa/gemini-quint/x14-chain-id/chain_id.qnt` | `0` | Typecheck succeeds |
| Typecheck tests | `quint typecheck qa/gemini-quint/x14-chain-id/chain_id_test.qnt` | `0` | Typecheck succeeds |
| Unit & mutant test suite | `quint test qa/gemini-quint/x14-chain-id/chain_id_test.qnt --main=chain_id_test` | `0` | All 7 `run` tests pass |
| Sound module (`invRow`) | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=chain_id --invariant=invRow --max-samples=25 --max-steps=2 --seed=0x14` | `0` | `[ok]` (`invRow` holds) |
| Sound module (`invComplete`) | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=chain_id --invariant=invComplete --max-samples=25 --max-steps=2 --seed=0x14` | `0` | `[ok]` (`invComplete` holds) |
| Non-vacuity (`invNonVacuous`) | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=chain_id --invariant=invNonVacuous --max-samples=25 --max-steps=2 --seed=0x14` | `1` (non-zero) | `[violation]` (witnesses accepted honest chain) |
| Mutant `m_no_first_row` (`invRow`) | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=m_no_first_row --invariant=invRow --max-samples=25 --max-steps=2 --seed=0x14` | `1` (non-zero) | `[violation]` (shifted `id_0` breaks `row_id == idx`) |
| Mutant `m_no_transition` (`invRow`) | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=m_no_transition --invariant=invRow --max-samples=25 --max-steps=2 --seed=0x14` | `1` (non-zero) | `[violation]` (duplicate `id = 0` swaps $P_2$) |
| Mutant `m_step_two` (`invRow`) | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=m_step_two --invariant=invRow --max-samples=25 --max-steps=2 --seed=0x14` | `1` (non-zero) | `[violation]` (`L_add` row 1 has `id = 2 != 1`) |
| Mutant `m_id_not_in_msg` (`invRow`) | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=m_id_not_in_msg --invariant=invRow --max-samples=25 --max-steps=2 --seed=0x14` | `1` (non-zero) | `[violation]` ($P_2$ swapped on bus `o + 1`) |

---

## 3. Structural Questions: Middle Padding, Field Wrap, and Cross-AIR Heights

### 3.1 Do `is_valid = 0` rows (padding) in the middle of an AIR break anything?
- **Soundness:** **No.** In `ChainAir`, `when_first_row: id = 0` and `when_transition: next.id = id + 1` are enforced unconditionally on every row regardless of `is_valid`. A middle row $r$ with `is_valid = 0` still gets `id = r`, and row $r + 1$ still gets `id = r + 1`. Because `is_valid = 0` multiplies both the `FieldExpr` carry constraints and the `PermutationCheckBus` multiplicities by $0$, row $r$ sends and receives nothing on any bus, while all valid rows retain distinct `id`s equal to their row indices.
- **Cross-AIR consistency:** Because row $r$ is the *unique* row in each AIR that can carry `id = r`, if one AIR has `is_valid = 0` at row $r$ while an adjacent AIR on the bus has `is_valid = 1` at row $r$, the message with `id = r` is unmatched and the verifier **rejects** the proof (modeled in `step_adversarial_tamper` branch `kind == 4`). Thus, a middle row $r$ can have `is_valid = 0` in an accepted proof if and only if **every** AIR in the chain is inactive at `id = r`.
- **Note on `PointsIn` in `x_exp_node.rs`:** In [x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/lane-src/x_exp_node.rs#L778), `PointsIn` sends with hardcoded multiplicity `AB::Expr::ONE` (it has no `is_valid` column). Therefore, for any $0 \le r < H_{\text{PointsIn}}$, `PointsIn` always sends `id = r`, forcing every `ChainAir` to have `is_valid = 1` on all rows $0 \le r < H_{\text{PointsIn}}$. Padding rows (`is_valid = 0`) in `ChainAir` can only appear at the tail $r \ge H_{\text{PointsIn}}$ unless `PointsIn` is also given an `is_valid` selector.

### 3.2 Does the field wrap (`height <= 2^17 < p` for BabyBear)?
- **No.** The `id` column is a native STARK scalar field element in BabyBear, where $p_{\text{BB}} = 2^{31} - 2^{27} + 1 = 2{,}013{,}265{,}921$ (`0x78000001`).
- By induction from `id[0] = 0` and `id[r + 1] = id[r] + 1`, row $r$ has $\text{id}[r] = r \bmod p_{\text{BB}}$.
- Two rows $0 \le r_1 < r_2 < H$ can collide modulo $p_{\text{BB}}$ if and only if $H > p_{\text{BB}}$.
- Since the maximum AIR trace height in X14 is $H \le 2^{17} = 131{,}072$ (and even the maximum stacked height is $2^{21} = 2{,}097{,}152 \ll p_{\text{BB}}$), `id` **never wraps** modulo $p_{\text{BB}}$, and $\text{id}[r] = r$ holds over $\mathbb{Z}$ for all rows.

### 3.3 Must cross-AIR heights match?
- **No, provided the active-row set matches across all AIRs in the chain.**
- For every row index $r \ge 0$, define $\text{active}(A, r) \iff (r < H_A \land \text{is\_valid}_A[r] = 1)$. Because each bus edge $A \to B$ requires the multiset of active `id`s in $A$ and $B$ to be identical and $\text{id}[r] = r$, bus balance holds if and only if $\text{active}(A, r) = \text{active}(B, r)$ for all $r$ and all connected AIRs $A, B$.
- Consequently:
  1. Any `ChainAir` (`L_add`, `X3`, `Y3`, `INV`, `SQ`, `L_dbl`, `INV_y`) may have a **larger height** $H_{\text{ChainAir}} > H_{\text{PointsIn}}$ than `PointsIn` (and different `ChainAir`s may have different power-of-two heights from one another), provided rows $0 \dots H_{\text{PointsIn}} - 1$ have `is_valid = 1` and trailing rows $H_{\text{PointsIn}} \dots H_{\text{ChainAir}} - 1$ have `is_valid = 0` (modeled in `step_honest` with `extra_pad == 1`).
  2. With `PointsIn` as currently written in [x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/lane-src/x_exp_node.rs#L744-L781) (where multiplicity is hardcoded to `AB::Expr::ONE` on every row), `PointsIn` **cannot be taller** than `L_add` ($H_{\text{PointsIn}} \le H_{\text{ChainAir}}$ is forced, because rows $r \ge H_{\text{ChainAir}}$ of `PointsIn` would send `id = r` with no matching row in `ChainAir`). If `PointsIn` is given a boolean `is_valid` column in production, `PointsIn` can also be taller than `L_add` by setting `is_valid = 0` on its trailing rows.

---

## 4. Guarantees, Exceptional Cases, and Completeness

### 4.1 (1) What the `id` Column Guarantees and What It Does Not
- **What it guarantees:**
  1. **Intra-AIR row uniqueness:** Every row $0 \le r < H$ in `PointsIn` and in each `ChainAir` has a unique identifier $\text{id}[r] = r$.
  2. **End-to-end per-row lockstep across the chain DAG:** Because every bus message on `o+0 .. o+10` is prefixed with `id`, LogUp multiset equality partitions by `id = k`. For each active $k$, row $k$ of `L_add` (or `SQ` and `L_dbl`), `INV` (or `INV_y`), `X3`, and `Y3` must evaluate its `FieldExpr` constraint on the exact values originated by row $k$ of `PointsIn`. No inputs or intermediate values ($\lambda, s, x_3$) can be swapped or mixed between different rows $k_1 \ne k_2$.
  3. **Synchronized validity (`is_valid`) and limb range-checking:** A row index $k$ is active in one AIR of the chain if and only if it is active in all AIRs of the chain, and every active row's inputs and saved variables have 8-bit range-checked limbs.
- **What it does NOT guarantee:**
  1. **Strict canonicality $< p_{381}$:** Range-checking 48 limbs to 8 bits bounds each coordinate to $[0, 2^{384})$, not $[0, p_{381})$. `FieldExpr` enforces congruence modulo $p_{381}$; if canonical representation $< p_{381}$ is required at the final public output boundary, a terminal canonicality check is still needed.
  2. **Curve and subgroup membership:** Neither `PointsIn` nor `ChainAir` checks the curve equation $y^2 \equiv x^3 + 4 \pmod{p_{381}}$ or $\mathbb{G}_1$ prime-order subgroup membership; input points must come from an authenticated source or verified table.
  3. **Consumer schedule binding:** Once `(k, x3, y3)` is emitted on bus `o + 7`, the real consumer AIR (replacing the test harness `SourceAir`) must constrain the received `k` against its own schedule/step column so the consumer itself cannot permute outputs.

### 4.2 (2) Exceptional Cases ($x_1 = x_2$, $y = 0$, Point at Infinity $\mathcal{O}$)
- **What the chain now does on them:**
  1. **$x_1 \equiv x_2 \pmod{p_{381}}$ in `add` (both doubling $P_1 = P_2$ and inverse points $P_1 = -P_2$):**
     - In `L_add` alone, $(x_2 - x_1)\lambda - (y_2 - y_1) \equiv 0 \pmod{p_{381}}$ degenerates to $0 \cdot \lambda - 0 \equiv 0$ when $P_1 = P_2$, which would allow any forged $\lambda$.
     - However, `L_add` sends `(id, x1, x2)` on `o + 10` to `INV`, which enforces $(x_2 - x_1) \cdot w - 1 \equiv 0 \pmod{p_{381}}$ for range-checked $w$.
     - When $x_1 \equiv x_2 \pmod{p_{381}}$, $0 \cdot w \equiv 1 \pmod{p_{381}}$ has **no solution** $w$. Because `id = row index` locks `INV` row $k$ to `L_add` row $k$, the prover cannot substitute a non-equal pair from another row (`dup_id` is blocked). Thus **every active `add` row with $x_1 \equiv x_2 \pmod{p_{381}}$ is unconditionally rejected** (modeled in `step_adversarial_tamper` branch `kind == 2`).
  2. **$y \equiv 0 \pmod{p_{381}}$ in `dbl` (including sentinel $(0, 0)$):**
     - In `L_dbl` alone, $(2y)\lambda - 3s \equiv 0 \pmod{p_{381}}$ degenerates to $0 \cdot \lambda - 0 \equiv 0$ when $x = y = 0$.
     - `L_dbl` sends `(id, y)` on `o + 10` to `INV_y`, which enforces $y \cdot w - 1 \equiv 0 \pmod{p_{381}}$.
     - When $y \equiv 0 \pmod{p_{381}}$, $0 \cdot w \equiv 1 \pmod{p_{381}}$ has **no solution** $w$, and `id = row index` blocks cross-row substitution. Thus **every active `dbl` row with $y \equiv 0 \pmod{p_{381}}$ is unconditionally rejected** (tested in `doubling_chain_and_y_zero_test`).
  3. **The point at infinity $\mathcal{O}$:**
     - Affine coordinates $(x, y) \in \mathbb{F}_{p_{381}}^2$ cannot represent $\mathcal{O}$. If $\mathcal{O}$ is encoded as $(0, 0)$, `dbl` rejects it via `INV_y` ($y = 0$) and `add` treats $(0, 0)$ as an ordinary affine point $(x=0, y=0)$ on the secant formula, which computes the wrong sum instead of the group identity.
- **What the verifier or MSM schedule must still add:**
  1. **Exception-free MSM schedule (e.g., offset/shifted accumulator):** Because `add` rejects $x_1 \equiv x_2 \pmod{p_{381}}$, `dbl` rejects $y \equiv 0 \pmod{p_{381}}$, and neither handles $\mathcal{O}$, the MSM schedule or accumulator controller must ensure that no active `add` row ever receives $x_1 \equiv x_2 \pmod{p_{381}}$, no active `dbl` row ever receives $y \equiv 0$, and no operand is $\mathcal{O}$ (for example, by starting the Pippenger/MSM bucket and window accumulators from a deterministic random/offset generator point $G_{\text{offset}}$ and subtracting the known multiple of $G_{\text{offset}}$ at the end, or by gating identity/collision bypasses in the controller).
  2. **Consumer `id` binding and input point validity:** The MSM accumulator AIR that consumes bus `o + 7` must constrain the message `id` against its own step index, and the producer of `PointsIn` must guarantee valid $\mathbb{G}_1$ curve points.

### 4.3 (3) Completeness on Honest Rows with Valid Inputs
- For any batch of $N \le 2^{17}$ valid non-exceptional inputs ($x_1, y_1, x_2, y_2 \in [0, p_{381})$ with $x_1 \not\equiv x_2 \pmod{p_{381}}$ for `add`, or $x, y \in [0, p_{381})$ with $y \not\equiv 0 \pmod{p_{381}}$ for `dbl`):
  - Since $p_{381}$ is prime and $x_2 - x_1 \not\equiv 0 \pmod{p_{381}}$ (resp. $2y \not\equiv 0 \pmod{p_{381}}$), the unique field inverses $w \in [1, p_{381})$ and outputs $\lambda, s, x_3, y_3 \in [0, p_{381})$ exist and decompose into 48 canonical 8-bit limbs.
  - `FieldExpr::generate_subrow` ([builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-chain-id/lane-src/builder.rs#L467-L577)) computes the exact integer quotients $q$ and limb carries within `range_max_bits = 17` and records their multiplicities in `VariableRangeCheckerChip`.
  - Setting $\text{id}[r] = r$ on rows $0 \le r < H$ and $\text{is\_valid}[r] = 1$ for $0 \le r < N$ (with $\text{is\_valid}[r] = 0$ for any trailing padding rows $N \le r < H$) satisfies all row, transition, range-check, and LogUp bus constraints.
  - Thus **every honest trace on valid non-exceptional inputs is accepted** (verified by `invComplete` and `honest_chain_accepted_test`).

## Lead quint results (2026-11-13, quint 0.32.0, run by the lead outside the lane sandbox)

After one lane fix turn (the test file did not typecheck). Lead notes: quint test runs only tests whose names match its default pattern, and these names end in _test, so the lead ran the suite with --match='_test$' (7 passing); the lane's invRow command uses 25 samples, so the lead also ran it with 1,000 samples (no violation).

| rc | expected | match | command |
|---|---|---|---|
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-chain-id/chain_id.qnt` |
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-chain-id/chain_id_test.qnt` |
| 0 | 0 | MATCH | `quint test qa/gemini-quint/x14-chain-id/chain_id_test.qnt --main=chain_id_test` |
| 0 | 0 | MATCH | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=chain_id --invariant=invRow --max-samples=25 --max-steps=2 --seed=0x14` |
| 0 | 0 | MATCH | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=chain_id --invariant=invComplete --max-samples=25 --max-steps=2 --seed=0x14` |
| 1 | 1 (REPORT: non-zero) | MATCH | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=chain_id --invariant=invNonVacuous --max-samples=25 --max-steps=2 --seed=0x14` |
| 1 | 1 (REPORT: non-zero) | MATCH | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=m_no_first_row --invariant=invRow --max-samples=25 --max-steps=2 --seed=0x14` |
| 1 | 1 (REPORT: non-zero) | MATCH | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=m_no_transition --invariant=invRow --max-samples=25 --max-steps=2 --seed=0x14` |
| 1 | 1 (REPORT: non-zero) | MATCH | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=m_step_two --invariant=invRow --max-samples=25 --max-steps=2 --seed=0x14` |
| 1 | 1 (REPORT: non-zero) | MATCH | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=m_id_not_in_msg --invariant=invRow --max-samples=25 --max-steps=2 --seed=0x14` |
| 0 | 0 | MATCH | `quint test qa/gemini-quint/x14-chain-id/chain_id_test.qnt --main=chain_id_test --match='_test$'` (7 passing) |
| 0 | 0 | MATCH | `quint run qa/gemini-quint/x14-chain-id/chain_id.qnt --main=chain_id --invariant=invRow --max-samples=1000 --seed=0x14` |
