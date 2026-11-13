# Soundness & Completeness Review: X14 `SQ` AIR (BLS12-381 G1 Doubling Chain)

## Verdict

- **SOUND (Conditional):** `SQ` (`ChainAir` instantiated with `sq_expr` and `sq_msgs` in [x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/x_exp_node.rs#L782-L827)) is **sound** within the BLS12-381 $G_1$ affine doubling chain (`L_dbl`, `X3`, `Y3`), **subject to four external/chain conditions**:
  1. **Input limb range check by the producer on bus $o+8$ (`1118`):** `FieldExpr::eval` ([builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/builder.rs#L437-L448)) range-checks only saved variables (`vars`, i.e., `s`), quotient limbs (`q_limbs`), and carry limbs (`carry_limbs`), **never** `inputs` (`x`). The producer sending `(id, x)` on bus $o+8$ must range-check each of the 48 limbs of `x` to $[0, 2^8)$ (note: the benchmark harness `SourceAir` in [x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/x_exp_node.rs#L704-L735) does not).
  2. **Pairwise-distinct operation `id`s from the producer:** `ChainAir` does not range-check or constrain `id` to be unique across active rows. If two active doubling operations share the same `id`, `SQ` and `L_dbl` can swap $(s, x)$ across the two points while all LogUp bus multisets balance (proven by `DUPLICATE_ID_SWAP_TRACE` in [sq.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq.qnt#L516-L521) and [x14_sq.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/x14_sq.qnt#L516-L521)).
  3. **Non-zero $y \not\equiv 0 \pmod p$ (and exclusion of $(0, 0)$) in `L_dbl`:** `SQ` itself has no division, so $s \equiv x^2 \pmod p$ is well-defined for all $x \in \mathbb{F}_p$. However, downstream `L_dbl` encodes $\lambda = 3s / (2y)$ as $(2y) \cdot \lambda - 3s \equiv 0 \pmod p$ ([field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/field_variable.rs#L308-L315)). At $(x, y) = (0, 0)$, `SQ` outputs $s = 0$, reducing `L_dbl`'s constraint to $0 \cdot \lambda - 0 \equiv 0 \pmod p$, which accepts **any** $\lambda \in \mathbb{F}_p$ and lets the prover forge $(x_3, y_3) = (\lambda^2, -\lambda^3) \pmod p$ (proven by `zeroPointForgeryTrace` in [sq.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq.qnt#L525-L538)).
  4. **LogUp total interaction bound:** Total interactions per bus across all traces must not overflow the BabyBear characteristic $p_{\text{BB}} = 2013265921$ ([interaction_mod.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/interaction_mod.rs#L126-L133)).
- **COMPLETE (Conditional on $y \not\equiv 0 \pmod p$):** Every honest row of `SQ` on canonical 8-bit limb inputs $x \in [0, p)$ (and every honest doubling chain row on affine points $(x, y)$ with $y \not\equiv 0 \pmod p$) satisfies all `FieldExpr` polynomial constraints, range checks on bus `1000`, and `PermutationCheckBus` multiset balances on buses $o, o+2, \dots, o+9$. Inactive padding rows in `SQ` with `is_valid = 0` and zeroed cells are also accepted.

---

## Execution Note

Direct shell execution was not used in this review; per instructions, no runtime execution outputs are claimed as observed. The exact commands and expected exit codes for `quint 0.32.0` are provided in the table below.

---

## Specification of `SQ` Under Review

In [x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/x_exp_node.rs#L660-L698) and [x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/x_exp_node.rs#L782-L830), `SQ` is a `ChainAir` with bus base $o = 1110$:

- **Columns (`width = ew + 1`):**
  - `local[0]`: `is_valid` (constrained boolean `is_valid * (is_valid - 1) == 0` in [builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/builder.rs#L366)).
  - `local[1 .. 49]`: `inputs[0] = x` (48 limbs of 8 bits).
  - `local[49 .. 97]`: `vars[0] = s` (48 limbs of 8 bits, range-checked on `RANGE_BUS = 1000` with multiplicity `is_valid`).
  - Quotient and carry limbs `q_limbs[0]`, `carry_limbs[0]` for `CheckCarryModToZeroSubAir`, range-checked on `RANGE_BUS = 1000` with multiplicity `is_valid`.
  - `local[ew]`: `id` (1 BabyBear element).
- **Modular Constraint (`sq_expr`):**
  - `x.square().save()` creates one saved variable `s = Var(0)` with symbolic constraint:
    $$\text{when } \text{is\_valid} = 1: \quad x \cdot x - s \equiv 0 \pmod p$$
- **Bus Interactions (`sq_msgs`, via `PermutationCheckBus`):**
  - **Receive on bus $o+8 = 1118$:** message `[id, x[0..48]]` (49 field elements $\le 128$), multiplicity `is_valid` (pushes count $-\text{is\_valid}$).
  - **Send on bus $o+9 = 1119$:** message `[id, s[0..48], x[0..48]]` (97 field elements $\le 128$), multiplicity `is_valid` (pushes count $+\text{is\_valid}$).
- **Downstream Chain Wiring ($o = 1110$):**
  - `L_dbl`: receives `(id, s, x)` on $o+9$ and `(id, y)` on $o$; constrains $(2y) \cdot \lambda \equiv 3s \pmod p$; sends `(id, lam, x)` on $o+2$, `(id, x)` on $o+3$, `(id, lam, x)` on $o+4$, and `(id, y)` on $o+5$.
  - `X3`: receives `(id, lam, x1)` on $o+2$ and `(id, x2)` on $o+3$; constrains $x_3 \equiv \lambda^2 - x_1 - x_2 \pmod p$; sends `(id, x3)` on $o+6$.
  - `Y3`: receives `(id, lam, x1)` on $o+4$, `(id, y1)` on $o+5$, and `(id, x3)` on $o+6$; constrains $y_3 \equiv \lambda(x_1 - x_3) - y_1 \pmod p$; sends `(id, x3, y3)` on $o+7$ to the consumer.

---

## Quint Model & Mutant Results

The Quint model is provided in [sq.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq.qnt) (and [x14_sq.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/x14_sq.qnt)) with unit tests in [sq_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq_test.qnt) (and [x14_sq_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/x14_sq_test.qnt)). It instantiates the doubling chain over $\mathbb{F}_{17}$ ($P = 17$) on $E: y^2 \equiv x^3 + 4 \pmod{17}$ with two active points:
- $P_1 = (6, 4)$ (`rowId = 1`), where $s_1 = 6^2 \bmod 17 = 2$, $\lambda_1 = (3 \cdot 2) / (2 \cdot 4) \bmod 17 = 5$, and $2P_1 = (13, 12)$.
- $P_2 = (9, 6)$ (`rowId = 2`), where $s_2 = 9^2 \bmod 17 = 13$, $\lambda_2 = (3 \cdot 13) / (2 \cdot 6) \bmod 17 = 16$, and $2P_2 = (0, 2)$.

### Commands & Expected Exit Codes

| Target / Check | Module / Test | Exact Quint Command | Expected Exit Code | Expected Outcome |
| :--- | :--- | :--- | :---: | :--- |
| Typecheck model | `sq.qnt` / `x14_sq.qnt` | `quint typecheck qa/gemini-quint/x14-sq/sq.qnt` | `0` | Types & effects valid |
| Typecheck tests | `sq_test.qnt` / `x14_sq_test.qnt` | `quint typecheck qa/gemini-quint/x14-sq/sq_test.qnt` | `0` | Types & effects valid |
| Unit test suite | `sq_test` | `quint test --main=sq_test --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq_test.qnt` | `0` | All 7 `run` tests pass |
| Soundness (`invRow`) | `sq` / `x14_sq` | `quint run --main=sq --invariant=invRow --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` | `0` | `[ok]` (`invRow` holds on all samples) |
| Non-vacuity (`invNonVacuous`) | `sq` / `x14_sq` | `quint run --main=sq --invariant=invNonVacuous --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` | `1` | `[violation]` at step 0 (`HONEST_TRACE` is accepted) |
| Mutant 1: `no_recv` | `m_no_recv` | `quint run --main=m_no_recv --invariant=invRow --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` | `1` | `[violation]` (`invRow` falsified by `EXPLOIT_NO_RECV`) |
| Mutant 2: `send_other_x` | `m_send_other_x` | `quint run --main=m_send_other_x --invariant=invRow --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` | `1` | `[violation]` (`invRow` falsified by `EXPLOIT_SEND_OTHER_X`) |
| Mutant 3: `no_id` | `m_no_id` | `quint run --main=m_no_id --invariant=invRow --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` | `1` | `[violation]` (`invRow` falsified by `EXPLOIT_NO_ID`) |
| Mutant 4: `wrong_square` | `m_wrong_square` | `quint run --main=m_wrong_square --invariant=invRow --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` | `1` | `[violation]` (`invRow` falsified by `EXPLOIT_WRONG_SQUARE`) |
| Mutant 5: `valid_free` | `m_valid_free` | `quint run --main=m_valid_free --invariant=invRow --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` | `1` | `[violation]` (`invRow` falsified by `EXPLOIT_VALID_FREE`) |

### How Each Mutant Violates `invRow`

1. **`m_no_recv` (drop receive of `(id, x)` on bus $o+8$):**
   - `SQ` no longer receives `(id, x)` on bus $o+8$. On `EXPLOIT_NO_RECV` ([sq.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq.qnt#L516-L525)), row 0 (`rowId = 1`, true source point $P_1 = (6, 4)$) sets `xIn = 9 != 6`, `sOut = 9^2 mod 17 = 13`. Downstream `L_dbl`, `X3`, `Y3` compute $\lambda = 7, (x_3, y_3) = (14, 12) \ne (13, 12)$. All AIR constraints and buses balance, violating both clauses of `invRow`.
2. **`m_send_other_x` (send `(id, s, s)` in place of `(id, s, x)` on bus $o+9$):**
   - `SQ` receives `(1, 6)` on $o+8$, computes `sOut = 2`, and sends `(1, 2, 2)` on $o+9$. On `EXPLOIT_SEND_OTHER_X` ([sq.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq.qnt#L527-L542)), `L_dbl` receives `sIn = 2, xIn = 2` (instead of `6`) and forwards `x = 2` on buses $o+2, o+3, o+4$ to `X3` and `Y3`. `X3` computes $x_3 = (5^2 - 2 - 2) \bmod 17 = 4 \ne 13$. All AIR constraints and buses balance, violating `invRow`.
3. **`m_no_id` (drop `id` from the send on bus $o+9$):**
   - `SQ` sends `[s, x]` without `id` on $o+9$. On `EXPLOIT_NO_ID` ([sq.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq.qnt#L544-L559)), `SQ` row 0 (`rowId = 1`, $x_1 = 6, s_1 = 2$) and row 1 (`rowId = 2`, $x_2 = 9, s_2 = 13$) emit $\{[2, 6], [13, 9]\}$ on $o+9$. `L_dbl` row 0 (`rowId = 1`, $y_1 = 4$) consumes $[13, 9]$ and `L_dbl` row 1 (`rowId = 2`, $y_2 = 6$) consumes $[2, 6]$, producing $(x_3, y_3) = (14, 12) \ne (13, 12)$ for `rowId = 1` and $(1, 5) \ne (0, 2)$ for `rowId = 2` while all buses balance.
4. **`m_wrong_square` (constraint $s \equiv x^2 + x \pmod p$):**
   - On `EXPLOIT_WRONG_SQUARE` ([sq.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq.qnt#L561-L576)), `SQ` row 0 sets `sOut = (6^2 + 6) mod 17 = 8 != 2`. `L_dbl`, `X3`, `Y3` honestly propagate $s = 8$ to produce $\lambda = 3$ and $(x_3, y_3) = (14, 6) \ne (13, 12)$. All mutated AIR constraints and buses balance, violating `invRow`.
5. **`m_valid_free` (multiplicity `1` in place of `is_valid` on the $o+9$ send):**
   - When `is_valid = 0` on an `SQ` row, `FieldExpr::eval` ([builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/builder.rs#L424-L448)) multiplies the modular constraint and range-check multiplicities by `is_valid = 0`, disabling $s \equiv x^2 \pmod p$. On `EXPLOIT_VALID_FREE` ([sq.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq.qnt#L578-L587)), row 0 sets `isValid = 0` and forged `sOut = 8 != 2` (with the $o+8$ message dropped so $o+8$ balances against $-\text{isValid} = 0$). Because `m_valid_free` sends on $o+9$ with multiplicity `1`, the unconstrained row still injects `(1, 8, 6)` onto $o+9$, yielding $(x_3, y_3) = (14, 6) \ne (13, 12)$ while all constraints and buses pass.

---

## Answers to Review Questions

### (1) What the `id` column guarantees and what it does not (can two rows with the same `id` swap inputs?)

- **What `id` guarantees:** Every bus message in the doubling chain ($o, o+2, \dots, o+9$) is prefixed with `id` ([x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/x_exp_node.rs#L820-L830)). Because `PermutationCheckBus` enforces multiset equality on each bus, **if** the producer assigns pairwise-distinct `id`s ($id_i \ne id_j$ for $i \ne j$) to all active operations, each active `id` has multiplicity $1$ on every bus. This uniquely binds `(id, x)` from the producer on $o+8$ to the matching `(id, s, x)` on $o+9$, `(id, y)` on $o$, `(id, lam, x)` on $o+2, o+4$, `(id, x)` on $o+3$, `(id, y)` on $o+5$, `(id, x3)` on $o+6$, and `(id, x3, y3)` on $o+7$, preventing cross-row mix-and-match attacks across distinct operations.
- **What `id` does NOT guarantee:**
  1. `ChainAir::eval` ([x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/x_exp_node.rs#L680)) places **zero local constraints** on `id = local[self.ew]`: `id` is not range-checked, not constrained to equal the row index, and not constrained to be unique across rows.
  2. **Yes, two rows with the same `id` CAN swap inputs:** If two active rows share `rowId = id` with points $(x_1, y_1)$ and $(x_2, y_2)$, `SQ` sends the multiset $\{(id, s_1, x_1), (id, s_2, x_2)\}$ on bus $o+9$ while the producer sends $\{(id, y_1), (id, y_2)\}$ on bus $o$. In `L_dbl`, the row receiving $(id, y_1)$ on bus $o$ can receive $(id, s_2, x_2)$ on bus $o+9$, and the row receiving $(id, y_2)$ can receive $(id, s_1, x_1)$. Both bus multisets balance identically, causing the chain to compute the hybrid pseudo-doublings of $(x_2, y_1)$ and $(x_1, y_2)$. This is formally demonstrated by `DUPLICATE_ID_SWAP_TRACE` and `duplicateIdSwapVulnerabilityTest` in [sq.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq.qnt#L516-L521) and [sq_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq_test.qnt#L84-L88).
  3. **Requirement:** The producer/consumer AIR (e.g., MSM chip) must enforce that all active requests use distinct `id` values (for instance, via a monotonic step counter or row-index lookup).

### (2) Exceptional cases ($x_1 = x_2$, $y = 0$, the point at infinity): what the AIR does and what the verifier must add

- **Behavior of `SQ` alone:** `SQ` only enforces $x^2 - s \equiv 0 \pmod p$. Because squaring has no denominator, `SQ` is total over $\mathbb{F}_p$: it accepts any canonical $x \in [0, p)$ with $s = x^2 \bmod p$, regardless of whether $y = 0$ or $(x, y)$ represents $\mathcal{O}$.
- **Behavior of the chain (`L_dbl` and `L_add`) on exceptional cases:**
  In [field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/field_variable.rs#L308-L315), `FieldVariable::div(self, other)` constrains `new_var` ($\lambda$) via:
  $$\text{other} \cdot \lambda - \text{self} \equiv 0 \pmod p$$
  - **$y = 0$ with $x \not\equiv 0 \pmod p$ in `L_dbl` (2-torsion point $(x, 0)$):** `SQ` forces $s = x^2 \not\equiv 0 \pmod p$. `L_dbl`'s constraint becomes $(2 \cdot 0) \cdot \lambda - 3s \equiv -3s \not\equiv 0 \pmod p$, which has **no solution** $\lambda \in \mathbb{F}_p$ for `is_valid = 1` (trace generation fails `debug_assert_eq!` in [builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/builder.rs#L525), and no valid proof exists).
  - **$(x, y) = (0, 0)$ in `L_dbl` (conventional affine representation of $\mathcal{O}$):** `SQ` forces $s = 0^2 = 0$. `L_dbl`'s constraint becomes $(2 \cdot 0) \cdot \lambda - 3 \cdot 0 \equiv 0 \pmod p$, which is **trivially satisfied for every $\lambda \in [0, p)$**! A dishonest prover can choose any $\lambda \in \mathbb{F}_p$ in `L_dbl`, and `X3`, `Y3` will output $(x_3, y_3) = (\lambda^2 \bmod p, -\lambda^3 \bmod p)$, forging a non-zero output from $(0, 0)$ (verified in `zeroPointUnconstrainedLambdaTest` in [sq_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq_test.qnt#L90-L99)).
  - **$x_1 = x_2$ in `X3` vs `L_add`:** In the doubling chain (`kind == "dbl"`), `X3` always receives $x_1 = x$ on $o+2$ and $x_2 = x$ on $o+3$, so $x_1 = x_2$ is the normal doubling path. In the addition chain (`kind == "add"`, which does not use `SQ`), `L_add` constrains $(x_2 - x_1) \cdot \lambda - (y_2 - y_1) \equiv 0 \pmod p$: if $x_1 = x_2$ and $y_1 \not\equiv y_2$ ($P + (-P) = \mathcal{O}$), the constraint is unsatisfiable; if $x_1 = x_2$ and $y_1 = y_2$ ($P + P$), $0 \cdot \lambda - 0 \equiv 0 \pmod p$ leaves $\lambda$ completely unconstrained.
- **What the verifier / circuit must add:**
  1. For `g1_double`, enforce $y \not\equiv 0 \pmod p$ on every active row (either by witnessing $y^{-1}$ with $y \cdot y^{-1} \equiv 1 \pmod p$, or by guaranteeing that all active inputs are valid affine curve points on $y^2 = x^3 + 4$ over $\mathbb{F}_{p_{381}}$ and handling $\mathcal{O}$ via a dedicated infinity flag before/outside the affine chain).
  2. For `g1_add`, enforce $x_1 \not\equiv x_2 \pmod p$ via an inverse witness $(x_2 - x_1)^{-1}$, or add boolean flags to branch between distinct addition, doubling ($P = Q$), inverse cancellation ($P = -Q \to \mathcal{O}$), and $\mathcal{O}$ bypass.

### (3) Whether inputs need range checks of their own, and what an unchecked producer would allow

- **Do inputs need range checks of their own in `SQ` / `ChainAir`?**
  - In `FieldExpr::eval` ([builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/builder.rs#L437-L448)), range checks on `RANGE_BUS` (`1000`) are emitted **only** for `vars` (saved output variables), `q_limbs`, and `carry_limbs`—**never for `inputs`**.
  - Whenever an input arrives over `PermutationCheckBus` from an upstream `ChainAir` saved variable (`Var(j)`—specifically `s` from `SQ` on $o+9$, `lam` from `L_dbl` on $o+2, o+4$, and `x3` from `X3` on $o+6$), the upstream `ChainAir` already range-checked all 48 limbs of that `Var` to $[0, 2^8)$ when `is_valid = 1`. Because `PermutationCheckBus` enforces exact limb-by-limb equality in $\mathbb{F}_{\text{BabyBear}}$, the receiving AIR does **not** need to re-range-check those limbs.
  - **However, the primary coordinate inputs $x$ (sent by the producer on $o+8$ to `SQ`, then forwarded as `In` by `SQ` on $o+9$ and by `L_dbl` on $o+2, o+3, o+4$) and $y$ (sent by the producer on $o$ to `L_dbl` and forwarded on $o+5$) are `In(...)` columns in every `ChainAir` and are NEVER range-checked anywhere in `[SQ, L_dbl, X3, Y3]`!**
- **What an unchecked producer would allow:**
  1. **Native BabyBear overflow in `CheckCarryModToZeroSubAir` (complete soundness breakdown):** `ExprBuilder` sizes `max_carry_bits` and `q_limbs` assuming every input limb satisfies $0 \le x_i < 2^8$ so that limb convolutions $x_i x_j$ and carry propagation stay strictly below the 31-bit BabyBear prime $p_{\text{BB}} = 2013265921$ ([builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/builder.rs#L94-L103)). If the producer (like the test harness `SourceAir` in [x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/x_exp_node.rs#L704-L735)) does not range-check the 48 limbs of $x$ and $y$, a malicious prover can supply limbs of size $\sim 2^{30}$ in $\mathbb{F}_{\text{BabyBear}}$ whose products wrap around modulo $p_{\text{BB}}$ inside the carry equations, forging $s \not\equiv x^2 \pmod{p_{381}}$.
  2. **Above-modulus representations in $[p_{381}, 2^{384})$:** Even with 8-bit limb range checks, 48 limbs of 8 bits span $[0, 2^{384})$, whereas $p_{381} < 2^{381}$. `FieldExpr` only enforces congruence $\bmod p_{381}$ and 8-bit limb bounds ($< 2^{384}$), so values in $[p_{381}, 2^{384})$ are accepted unless canonicality ($< p_{381}$) is enforced at the boundary.

### (4) Completeness: every honest row on valid points is accepted

- For any valid affine point $(x, y) \in E(\mathbb{F}_p)$ with $y \not\equiv 0 \pmod p$ and canonical limbs $x, y \in [0, p)$:
  - In `SQ`, `generate_subrow` ([builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/lane-src/builder.rs#L467-L577)) computes $s = x^2 \bmod p \in [0, p)$, integer quotient $q = (x^2 - s) / p$, and limb carries of $x^2 - s - q \cdot p = 0$. Because $0 \le x, s < p < 2^{384}$, $q$ and all carries lie strictly within the static bounds `q_limbs` and `carry_bits <= 17`, passing all `RANGE_BUS` checks.
  - Because $y \not\equiv 0 \pmod p$ and $p > 2$ is prime, $2y$ is invertible $\bmod p$, so `L_dbl`, `X3`, and `Y3` deterministically compute canonical $\lambda, x_3, y_3 \in [0, p)$ satisfying their `CheckCarryModToZeroSubAir` constraints and range checks.
  - With `is_valid = 1` and matching `id` across `SourceAir`, `SQ`, `L_dbl`, `X3`, `Y3`, every message sent on buses $o, o+2, \dots, o+9$ has net multiplicity $+1 - 1 = 0$. Any unused padding rows in `SQ`, `L_dbl`, `X3`, `Y3` with `is_valid = 0` and zeroed limbs satisfy all polynomial constraints and contribute multiplicity $0$ to all buses (`completenessHonestAcceptedTest` in [sq_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-sq/qa/gemini-quint/x14-sq/sq_test.qnt#L25-L31)).

---

## Lead results (2026-11-13 03:19, quint 0.32.0, run by the lead outside the lane sandbox)

The lane could not run commands. The lead ran every quint command of this report in the lane worktree `~/.cache/gemini-lanes/x14-sq`; expected exit code 0 for typecheck, test and the sound invariant, 1 for non-vacuity and mutants.

| rc | expected | result | command |
|---:|---:|---|---|
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-sq/sq.qnt` |
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-sq/sq_test.qnt` |
| 0 | 0 | MATCH | `quint test --main=sq_test --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq_test.qnt` |
| 0 | 0 | MATCH | `quint run --main=sq --invariant=invRow --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` |
| 1 | 1 | MATCH | `quint run --main=sq --invariant=invNonVacuous --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_no_recv --invariant=invRow --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_send_other_x --invariant=invRow --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_no_id --invariant=invRow --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_wrong_square --invariant=invRow --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_valid_free --invariant=invRow --max-samples=100 --seed=0x14ea qa/gemini-quint/x14-sq/sq.qnt` |
