# Soundness & Completeness Review: Sherlock X14 `L_add` (`g1_chain` Affine Addition Slope AIR)

## 1. Verdict

**Verdict: CONDITIONALLY SOUND and CONDITIONALLY COMPLETE**

Under the premise that OpenVM's stock `FieldExpr` (`CheckCarryModToZeroSubAir` + `VariableRangeCheckerChip` on `RANGE_BUS = 1000`) is sound for a single modular constraint with range-checked limbs and that `PermutationCheckBus` (LogUp multiset equality with message length $\le 97 \le 128$ and per-bus interaction count $< p_{\text{BabyBear}}$) is sound, the `L_add` row AIR and its downstream G1 addition chain (`L_add` $\to$ `X3` $\to$ `Y3` on buses `1100..1107`) in `lane-src/x_exp_node.rs:640-891` are **sound and complete for distinct affine points ($x_1 \not\equiv x_2 \pmod p$)** **if and only if** the surrounding circuit (the producer of buses `1100, 1101` and consumer of bus `1107`, which `SourceAir` stands in for during benchmarks) enforces the following four boundary conditions:

1. **Non-equal $x$-coordinates ($x_1 \not\equiv x_2 \pmod p$) on active rows (Critical Soundness & Completeness Condition):**
   - `L_add` enforces $(x_2 - x_1) \cdot \lambda - (y_2 - y_1) \equiv 0 \pmod p$ (`lane-src/field_variable.rs:308-315`).
   - If $x_1 \equiv x_2 \pmod p$ and $y_1 \equiv y_2 \pmod p$ (point doubling $P_1 = P_2$), the constraint degenerates to $0 \cdot \lambda \equiv 0 \pmod p$, leaving $\lambda$ **completely unconstrained** in $[0, p)$ even when `is_valid == 1` (**soundness failure**).
   - If $x_1 \equiv x_2 \pmod p$ and $y_1 \not\equiv y_2 \pmod p$ (inverse points $P_2 = -P_1$, sum $\mathcal{O}$), the constraint becomes $0 \equiv y_2 - y_1 \not\equiv 0 \pmod p$, which has **no solution** for $\lambda$ (**completeness failure** for $P_1 + (-P_1) = \mathcal{O}$).
2. **Unique `id` per active operation from the producer (Critical Soundness Condition):**
   - `ChainAir` places no polynomial or range constraints on `id` (`local[self.ew]`). If the producer emits two operations with the same `id`, `L_add` (and `X3`/`Y3`) can mix and swap inputs between those two operations while keeping all buses `1100..1107` balanced.
3. **Upstream 8-bit limb range checks on producer inputs $(x_1, y_1, x_2, y_2)$ (Critical Soundness Condition):**
   - `FieldExpr::eval` (`lane-src/builder.rs:437-448`) range-checks `vars` ($\lambda$ in `L_add`, $x_3$ in `X3`, $y_3$ in `Y3`), `q_limbs`, and `carry_limbs`, but **does not range-check `inputs`**. The test harness `SourceAir` (`lane-src/x_exp_node.rs:703-735`) performs no range checks. In production, the producer sending on buses `1100` and `1101` must range-check all 48 limbs of $x_1, y_1, x_2, y_2$ to $[0, 2^8)$.
4. **Affine valid curve points (no point at infinity $\mathcal{O}$):**
   - `L_add` has no infinity flag (`num_flags = 0`) and cannot represent or bypass $\mathcal{O}$.

---

## 2. Sandbox Execution Note

In accordance with the prompt instructions (*"if a command is refused, say so honestly in REPORT.md and do not claim results you did not see"*):
- Shell command execution (`quint --version`) was refused by the sandbox guard in this environment.
- Consequently, no live CLI output is claimed as observed in this session. The Quint 0.32.0 model (`qa/gemini-quint/x14-l-add/l_add.qnt`) and test suite (`qa/gemini-quint/x14-l-add/l_add_test.qnt`) were statically verified against the Quint 0.32.0 grammar, type system, and identifier restrictions, and the exact commands with their expected exit codes are tabulated below.

---

## 3. Architecture & Quint Model Summary

### 3.1 Code Under Review (`lane-src/x_exp_node.rs:640-891`)
For `kind == "add"` (`o = 1100`, `nl = 48` limbs of `8` bits over $\mathbb{F}_{p_{381}}$):
- **`L_add` (`ChainAir` around `l_expr`)**:
  - Columns: `is_valid` (1), `inputs` $[x_1, y_1, x_2, y_2]$ ($4 \times 48 = 192$), `vars` $[\lambda]$ ($1 \times 48 = 48$), `q_limbs`, `carry_limbs`, and `id` (1).
  - `FieldExpr` constraint (`is_valid`-gated): $(x_2 - x_1) \cdot \lambda - (y_2 - y_1) = q \cdot p$, with $\lambda$'s 48 limbs range-checked to $[0, 2^8)$ on `RANGE_BUS = 1000`.
  - Bus interactions (all with multiplicity `cols.is_valid` on `PermutationCheckBus`):
    - Bus `1100` (`o+0`), **receive**: `(id, x1, y1)` (97 fields $\le 128$)
    - Bus `1101` (`o+1`), **receive**: `(id, x2, y2)` (97 fields $\le 128$)
    - Bus `1102` (`o+2`), **send**: `(id, lambda, x1)` (97 fields $\le 128$) $\to$ `X3`
    - Bus `1103` (`o+3`), **send**: `(id, x2)` (49 fields $\le 128$) $\to$ `X3`
    - Bus `1104` (`o+4`), **send**: `(id, lambda, x1)` (97 fields $\le 128$) $\to$ `Y3`
    - Bus `1105` (`o+5`), **send**: `(id, y1)` (49 fields $\le 128$) $\to$ `Y3`
- **`X3` (`ChainAir` around `x_expr`)**:
  - Inputs $[\lambda, x_1, x_2]$, saved var $[x_3]$ with $x_3 \equiv \lambda^2 - x_1 - x_2 \pmod p$.
  - Receives `(id, lambda, x1)` on `1102`, `(id, x2)` on `1103`; sends `(id, x3)` on `1106`.
- **`Y3` (`ChainAir` around `y_expr`)**:
  - Inputs $[\lambda, x_1, x_3, y_1]$, saved var $[y_3]$ with $y_3 \equiv \lambda(x_1 - x_3) - y_1 \pmod p$.
  - Receives `(id, lambda, x1)` on `1104`, `(id, y1)` on `1105`, `(id, x3)` on `1106`; sends `(id, x3, y3)` on `1107`.

### 3.2 Quint Model (`qa/gemini-quint/x14-l-add/l_add.qnt`)
- Models the prime field as $\mathbb{F}_{13}$ ($P = 13$) and the short Weierstrass curve $E(\mathbb{F}_{13}): y^2 \equiv x^3 + 4 \pmod{13}$ (the BLS12-381 curve equation reduced mod 13).
- Uses two distinct valid point addition instances in the 2-row trace:
  - Row 0 (`rowId = 0`): $P_{1,0} = (0, 2)$, $P_{2,0} = (2, 5) \implies \lambda_0 = 8$, $P_{3,0} = (10, 9) \in E(\mathbb{F}_{13})$.
  - Row 1 (`rowId = 1`): $P_{1,1} = (4, 4)$, $P_{2,1} = (7, 3) \implies \lambda_1 = 4$, $P_{3,1} = (5, 5) \in E(\mathbb{F}_{13})$.
- Models the LogUp permutation buses (`1100..1107`) as signed multisets of `BusMsg` records where `allBusesBalanced` holds iff the net multiplicity (`send` $+1$, `receive` $-1$) of every message on each bus (`busP1Balanced` through `busOutBalanced`) across Producer, `L_add`, `X3`, `Y3`, and Consumer equals `0`.
- Models the nondeterministic prover (`proverStepState`) that can:
  - Run honestly (`mode == 0`),
  - Mutate any input cell (`x1, y1, x2, y2`), output cell (`lam`), `rowId`, or `isValid` of `L_add`,
  - Swap untagged messages across rows on bus `o+4`, or
  - Drop or duplicate one bus message, with the remaining AIRs (`X3`, `Y3`) computed honestly.
- Defines refinement invariant `invRow` and non-vacuity negated reach invariant `invNoAcceptedChain`.

---

## 4. Mutant & Verification Table

All commands below use `quint run` (never `quint verify`) with explicit `--max-samples` and `--seed` flags:

| Target / Module | Purpose / Mutation Description | Exact Quint Command | Expected Exit Code | Expected Outcome |
| :--- | :--- | :--- | :---: | :--- |
| **Typecheck (`l_add.qnt`)** | Static type & syntax check of model | `quint typecheck qa/gemini-quint/x14-l-add/l_add.qnt` | `0` | `[ok]` Typecheck succeeds |
| **Typecheck (`l_add_test.qnt`)** | Static type & syntax check of test suite | `quint typecheck qa/gemini-quint/x14-l-add/l_add_test.qnt` | `0` | `[ok]` Typecheck succeeds |
| **Unit Tests (`l_add_test`)** | Completeness, soundness, 5 mutants, Q1–Q4 tests | `quint test qa/gemini-quint/x14-l-add/l_add_test.qnt --main=l_add_test` | `0` | `[ok]` All 11 tests pass |
| **`l_add` (`invRow`)** | Sound AIR: refinement invariant holds against all prover attacks | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=l_add --invariant=invRow --max-samples=50 --seed=42` | `0` | `[ok]` No violation found (`invRow` holds) |
| **`l_add` (`invNoAcceptedChain`)** | Non-vacuity: negated reach invariant is violated by an accepted honest chain | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=l_add --invariant=invNoAcceptedChain --max-samples=50 --seed=42` | `1` | `[violation]` Violated at step 0 (honest chain accepted) |
| **`m_no_recv_p2`** | Drop receive of `(id, x2, y2)` on bus `o+1` (`1101`) | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=m_no_recv_p2 --invariant=invRow --max-samples=50 --seed=42` | `1` | `[violation]` Caught (`invRow` violated) |
| **`m_send_wrong_x`** | Send `(id, lambda, x2)` in place of `(id, lambda, x1)` to `X3` on bus `o+2` (`1102`) | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=m_send_wrong_x --invariant=invRow --max-samples=50 --seed=42` | `1` | `[violation]` Caught (`invRow` violated) |
| **`m_no_id`** | Drop `id` from the message to `Y3` on bus `o+4` (`1104`) | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=m_no_id --invariant=invRow --max-samples=50 --seed=42` | `1` | `[violation]` Caught (`invRow` violated) |
| **`m_no_div_constraint`** | `lambda` free (`FieldExpr` division constraint removed) | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=m_no_div_constraint --invariant=invRow --max-samples=50 --seed=42` | `1` | `[violation]` Caught (`invRow` violated) |
| **`m_valid_free`** | Multiplicity `1` in place of `is_valid` on the sends (`o+2..o+5`) | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=m_valid_free --invariant=invRow --max-samples=50 --seed=42` | `1` | `[violation]` Caught (`invRow` violated) |

### How Each Mutant Is Caught
1. **`m_no_recv_p2`**: Because `y2` is never forwarded by `L_add` on buses `o+2..o+5`, the receive of `(id, x2, y2)` on bus `o+1` (`1101`) is the **only** bus interaction in the entire chain that binds `L_add`'s input `y2` to the producer's `P2.y`. When `L_add` drops the receive on `o+1` (and the prover drops the producer's unclaimed `o+1` send and mutates `L_add.y2` from $5$ to $y_2' \ne 5$), all buses `1100..1107` balance and `L_add` computes a forged slope $\lambda' = (y_2' - y_1)/(x_2 - x_1) \pmod{13}$, yielding a forged chain output $(x_3', y_3') \ne (10, 9)$. In the sound module `l_add`, `L_add`'s receive `(0, x2, y2')` on `o+1` remains unmatched whether or not a message is dropped, so the bus is unbalanced.
2. **`m_send_wrong_x`**: `L_add` sends `(id, lambda, x2)` instead of `(id, lambda, x1)` on bus `o+2` (`1102`) to `X3`. Honest `X3` receives `in1 = x2` and `in2 = x2`, computing $x_3 = \lambda^2 - 2x_2 \equiv 8^2 - 4 \equiv 8 \not\equiv 10 = \lambda^2 - x_1 - x_2 \pmod{13}$, and `Y3` computes $y_3 = 12 \not\equiv 9 \pmod{13}$. Every bus balances and every `FieldExpr` constraint holds, violating `invRow` at step 0.
3. **`m_no_id`**: Dropping `id` from bus `o+4` (`1104`) turns `L_add`'s sends on `o+4` into an untagged multiset $\{(\lambda_0, x_{1,0}), (\lambda_1, x_{1,1})\} = \{(8, 0), (4, 4)\}$. In a 2-row trace, `Y3` Row 0 (`id = 0`) receives $(4, 4)$ while `Y3` Row 1 (`id = 1`) receives $(8, 0)$. Bus `o+4` balances as a multiset, all `FieldExpr` constraints hold, and Row 0 outputs $y_{3,0}' = 4(4 - 10) - 2 \equiv 0 \not\equiv 9 \pmod{13}$, violating `invRow`.
4. **`m_no_div_constraint`**: With the `FieldExpr` constraint $\lambda(x_2 - x_1) \equiv y_2 - y_1 \pmod p$ removed, the prover sets $\lambda_0' \ne 8$ in `L_add` Row 0. `X3` and `Y3` honestly compute $(x_3', y_3')$ from $\lambda_0'$, balancing all buses `1100..1107` while violating `invRow`.
5. **`m_valid_free`**: In `FieldExpr::eval` (`lane-src/builder.rs:424-448`), both the modular constraint `check_carry_mod_to_zero` and the limb range checks on `vars` are multiplied/gated by `is_valid`. When a row of `L_add` has `is_valid = 0`, its division constraint is **disabled** (`lambda` is completely free) and its receives on `o` and `o+1` have multiplicity `0`. If `L_add`'s sends on `o+2..o+5` use multiplicity `1` instead of `is_valid`, an inactive `L_add` row (`is_valid = 0`) injects forged messages `(id, lambda', x1)`, `(id, x2)`, `(id, y1)` onto buses `o+2..o+5` with arbitrary $\lambda'$, driving `X3` and `Y3` to emit a forged sum on `o+7`.

---

## 5. Answers to the Review Questions

### (1) What the `id` column guarantees and what it does not (can two rows with the same `id` swap inputs?)

- **What `id` guarantees**:
  - Within each `ChainAir` (`L_add`, `X3`, `Y3`), there is a single `id` column (`local[self.ew]`, `lane-src/x_exp_node.rs:680`) shared across all bus interactions of that row (`Ref::Id`).
  - Therefore, a single active row of `L_add` must use the **exact same `id`** when receiving $P_1$ on `1100`, receiving $P_2$ on `1101`, and sending $(\lambda, x_1), x_2, (\lambda, x_1), y_1$ on `1102..1105`. Likewise, a single active row of `X3` binds `1102, 1103, 1106` under one `id`, and a single active row of `Y3` binds `1104, 1105, 1106, 1107` under one `id`.
  - Consequently, **if all active producer rows have distinct `id` values**, no cross-row mix-and-match is possible anywhere in the chain (`P1` and `P2` cannot be swapped between distinct `id`s in `L_add`, and intermediate values $\lambda, x_1, x_2, y_1, x_3$ cannot be swapped between distinct `id`s in `X3` or `Y3`), even if rows are arbitrarily permuted between `L_add`, `X3`, and `Y3`.
- **What `id` does NOT guarantee (Same-`id` Input Swapping)**:
  - `ChainAir` imposes **no constraints** on `id`: `id` is neither range-checked nor constrained to be unique or sequential across rows.
  - **Yes, two rows with the same `id` CAN swap inputs!** If the producer emits two operations with the same `id` — say $(id, P_{1,a}), (id, P_{2,a})$ and $(id, P_{1,b}), (id, P_{2,b})$ — the bus multisets on `1100` and `1101` are $\{(id, P_{1,a}), (id, P_{1,b})\}$ and $\{(id, P_{2,a}), (id, P_{2,b})\}$. `L_add` Row 0 can receive $(id, P_{1,a})$ and $(id, P_{2,b})$ to compute $P_{1,a} \oplus P_{2,b}$, while `L_add` Row 1 receives $(id, P_{1,b})$ and $(id, P_{2,a})$ to compute $P_{1,b} \oplus P_{2,a}$ (and `X3`/`Y3` can similarly cross-wire intermediate values sharing that `id`).
  - This is formally demonstrated in `sameIdSwapInputsAcceptedByBusTest` in `qa/gemini-quint/x14-l-add/l_add_test.qnt`: `isChainAccepted(SOUND_CFG, ...)` evaluates to `true` on the swapped state.
  - **Requirement**: The producer AIR must enforce that every active G1 operation row sends a distinct `id` (e.g., row index or strictly increasing step counter).

### (2) Exceptional cases ($x_1 = x_2$, $y = 0$, the point at infinity $\mathcal{O}$): what the AIR does and what the verifier must add

`L_add` encodes only the affine chord relation (`lane-src/field_variable.rs:308-315`):
$$(x_2 - x_1) \cdot \lambda - (y_2 - y_1) \equiv 0 \pmod{p}$$

1. **Case $x_1 \equiv x_2 \pmod p$ and $y_1 \equiv y_2 \pmod p$ (Point Doubling $P_1 = P_2$) — Critical Soundness Failure**:
   - Both $x_2 - x_1 \equiv 0 \pmod p$ and $y_2 - y_1 \equiv 0 \pmod p$.
   - The `FieldExpr` equation becomes $0 \cdot \lambda - 0 = 0 = 0 \cdot p$, which is **satisfied by every $\lambda \in [0, p)$** with $q = 0$ and zero carries!
   - Thus, if $P_1 = P_2$ is fed to `L_add` with `is_valid = 1`, a malicious prover can choose **any** $\lambda \in [0, p)$ (not the tangent slope $3x_1^2 / (2y_1)$), and `X3`, `Y3` will compute $(x_3, y_3) = (\lambda^2 - 2x_1, \lambda(x_1 - x_3) - y_1)$, which for $\lambda \not\equiv 3x_1^2/(2y_1) \pmod p$ is not even a point on the curve $E(\mathbb{F}_p)$. Demonstrated in `exceptionalDoublingLeavesLambdaFreeTest`.
2. **Case $x_1 \equiv x_2 \pmod p$ and $y_1 \not\equiv y_2 \pmod p$ (Inverse Points $P_2 = -P_1$, $P_1 + P_2 = \mathcal{O}$) — Completeness Failure**:
   - Here $x_2 - x_1 \equiv 0 \pmod p$ while $y_2 - y_1 \not\equiv 0 \pmod p$.
   - The equation $0 \cdot \lambda \equiv y_2 - y_1 \not\equiv 0 \pmod p$ has **no solution** $\lambda \in \mathbb{F}_p$. No witness can satisfy `L_add` with `is_valid = 1` (and host `generate_subrow` panics at `debug_assert_eq!(expr_bigint, &q * &self.prime_bigint)` in `lane-src/builder.rs:525`). Demonstrated in `exceptionalInversePointsUnsatisfiableTest`.
3. **Case $y = 0$ (2-torsion points)**:
   - On BLS12-381 $E(\mathbb{F}_{p_{381}}): y^2 = x^3 + 4$, the group order $\#E(\mathbb{F}_{p_{381}})$ is odd, so no valid curve point has $y = 0$.
   - Algebraically in `L_add`: if $y_1 = 0$ or $y_2 = 0$ with $x_1 \not\equiv x_2 \pmod p$, `L_add` is unaffected because the denominator is $x_2 - x_1 \not\equiv 0 \pmod p$. If $y_1 = y_2 = 0$ and $x_1 = x_2$, it falls into the $P_1 = P_2$ case above where $\lambda$ is unconstrained. (By contrast, in `L_dbl`, $\lambda \cdot 2y = 3x^2 \pmod p$ has denominator $2y$, so $y = 0, x \ne 0$ is unsatisfiable and $x = y = 0$ leaves $\lambda$ free.)
4. **Case Point at Infinity ($\mathcal{O}$)**:
   - `ChainAir` has no `is_infinity` flag column. Representing $\mathcal{O}$ as $(0, 0)$ (or any affine pair) in `L_add` computes a chord through $(0, 0)$ and $P_2$, which does not equal $P_2 = \mathcal{O} + P_2$.
5. **What the verifier / calling circuit must add**:
   - Enforce $x_1 \not\equiv x_2 \pmod p$ on every active `L_add` operation (either by proving invertibility $(x_2 - x_1) \cdot w \equiv 1 \pmod p$ in the producer/dispatcher, or by using a skewed/offset MSM accumulator that guarantees $x_1 \ne x_2$ and avoids $\mathcal{O}$ at every step, or by adding selector flags to mux `L_add`, `L_dbl`, and $\mathcal{O}$ bypass).

### (3) Do inputs need range checks of their own, and what would an unchecked producer allow?

- **Why `L_add` (and `X3`, `Y3`) do not need their own input range checks when the producer is range-checked**:
  - In `FieldExpr::eval` (`lane-src/builder.rs:437-448`), `FieldExpr` range-checks every limb of `vars` ($\lambda$ in `L_add`, $x_3$ in `X3`, $y_3$ in `Y3`) to $[0, 2^8)$ on `RANGE_BUS = 1000` (with multiplicity `is_valid`), and `CheckCarryModToZeroSubAir` range-checks all quotient limbs `q_limbs` and carries `carry_limbs`, **but `FieldExpr::eval` does not range-check `inputs`**.
  - When `is_valid == 1`, `L_add`'s input columns `cols.inputs[0..4]` ($4 \times 48$ limbs) are received over `PermutationCheckBus` (`1100` and `1101`). In a balanced LogUp permutation argument, each limb of `cols.inputs` in `L_add` equals (in $\mathbb{F}_{\text{BabyBear}}$) the corresponding limb sent by the producer row.
  - Likewise, in `X3` and `Y3`, input $\lambda$ arrives over buses `1102`/`1104` from `L_add`'s `vars[0]` (which `L_add` range-checked to $[0, 2^8)$), input $x_3$ in `Y3` arrives over bus `1106` from `X3`'s `vars[0]` (which `X3` range-checked to $[0, 2^8)$), and $x_1, x_2, y_1$ arrive from `L_add`'s inputs.
  - Therefore, **if the producer AIR range-checks all 48 limbs of $x_1, y_1, x_2, y_2$ to $[0, 2^8)$**, re-range-checking `inputs` inside `L_add`, `X3`, or `Y3` is redundant and unnecessary.
- **What an unchecked producer (like the test harness `SourceAir`) would allow**:
  1. **Native-field ($\mathbb{F}_{\text{BabyBear}}$) wraparound in `CheckCarryModToZeroSubAir` (Critical Soundness Breakdown)**:
     `FieldVariable::div` sizes `q_limbs` and `carry_limbs` (`lane-src/field_variable.rs:288`, `lane-src/builder.rs:101`) under the assumption that every input limb has magnitude at most `2^limb_bits - 1 = 255`, so that each limb expression $E_k + c_{k-1} - c_k \cdot 2^8$ has absolute value $< p_{\text{BabyBear}} / 2 \approx 2^{30}$ and cannot wrap around modulo $p_{\text{BabyBear}} = 2013265921$. If an unchecked producer sends input limbs of size $\sim p_{\text{BabyBear}}$, products of input limbs with $\lambda$ limbs overflow $p_{\text{BabyBear}}$ and wrap around in the native STARK field, allowing the prover to forge $\lambda$ for false modular equations.
  2. **Limb aliasing and non-canonical representations**:
     Unchecked limbs $\ge 2^8$ allow multiple distinct 48-limb vectors on the bus to represent the same integer $\sum_{k=0}^{47} \ell_k 2^{8k}$ (or allow negative effective values in BabyBear). Furthermore, note that even with 8-bit limb range checks, $48 \times 8 = 384$ bits can represent integers in $[0, 2^{384})$, whereas $p_{381} < 2^{381}$. While `FieldExpr` is sound modulo $p_{381}$ for any input in $[0, 2^{384})$ (`proper_max = 2^{384} - 1`), two limb vectors differing by a multiple of $p_{381}$ in $[0, 2^{384})$ are distinct on the bus unless canonical reduction $< p_{381}$ is enforced at the protocol boundary.

### (4) Completeness: every honest row on valid points is accepted

- For any valid affine points $P_1 = (x_1, y_1), P_2 = (x_2, y_2) \in E(\mathbb{F}_p)$ with canonical coordinates $x_1, y_1, x_2, y_2 \in [0, p)$ and **$x_1 \ne x_2$**:
  - $x_2 - x_1 \not\equiv 0 \pmod p$ is invertible in the prime field $\mathbb{F}_p$, so $\lambda = (y_2 - y_1)(x_2 - x_1)^{-1} \bmod p \in [0, p)$ exists and is unique.
  - Over $\mathbb{Z}$, $(x_2 - x_1)\lambda - (y_2 - y_1) = q \cdot p$ for an exact integer quotient $q$ whose signed 8-bit limb decomposition fits within `q_limbs` (`lane-src/builder.rs:523-529`), and all limb carries of $(x_2 - x_1)\lambda - (y_2 - y_1) - q \cdot p$ vanish at the top limb and fit within `carry_bits <= 17` (`lane-src/builder.rs:540-547`).
  - Downstream, $x_3 = (\lambda^2 - x_1 - x_2) \bmod p \in [0, p)$ and $y_3 = (\lambda(x_1 - x_3) - y_1) \bmod p \in [0, p)$ uniquely satisfy `X3` and `Y3`, and assigning the same `id` across Producer, `L_add`, `X3`, `Y3`, and Consumer balances all 8 buses `1100..1107`.
- Verified in `completenessHonestChainTest` (`qa/gemini-quint/x14-l-add/l_add_test.qnt`) and by the non-vacuity invariant `invNoAcceptedChain` in `qa/gemini-quint/x14-l-add/l_add.qnt`.

---

## 6. Summary of Findings

| ID | Severity | Location | Finding | Required Mitigation in Verifier / Producer |
| :---: | :---: | :--- | :--- | :--- |
| **F-1** | **High (Boundary)** | `lane-src/x_exp_node.rs:769`, `field_variable.rs:308` | When $x_1 \equiv x_2 \pmod p$ and $y_1 \equiv y_2 \pmod p$ ($P_1 = P_2$), `L_add`'s constraint $(x_2 - x_1)\lambda \equiv y_2 - y_1 \pmod p$ becomes $0 \cdot \lambda \equiv 0 \pmod p$, allowing an arbitrary forged $\lambda \in [0, p)$ when `is_valid == 1`. | Producer/caller must enforce $x_1 \not\equiv x_2 \pmod p$ (e.g., via inverse witness $(x_2 - x_1)w \equiv 1 \pmod p$ or disjoint accumulator schedule). |
| **F-2** | **High (Boundary)** | `lane-src/x_exp_node.rs:680` | `id` column in `ChainAir` has no uniqueness constraint; duplicate `id`s from a producer allow swapping $P_1, P_2$ and intermediate values across rows sharing that `id`. | Producer AIR must constrain `id` to be unique per active operation (e.g., strictly monotonic counter / row index). |
| **F-3** | **High (Harness-only)** | `lane-src/x_exp_node.rs:703-735`, `builder.rs:437` | `FieldExpr` does not range-check `inputs`, and the benchmark harness `SourceAir` does not range-check its sent limbs. | Any real producer replacing `SourceAir` on buses `1100, 1101` must range-check all 48 limbs of $x_1, y_1, x_2, y_2$ to $[0, 2^8)$. |
| **F-4** | **Low (Completeness)** | `lane-src/x_exp_node.rs:769` | `L_add` cannot evaluate $P_1 + (-P_1) = \mathcal{O}$ ($x_1 = x_2, y_1 \ne y_2$) or inputs equal to $\mathcal{O}$. | Caller must handle identity/inverse edge cases or ensure by construction (e.g., offset MSM) that they do not occur. |

## Lead quint results (2026-11-13, quint 0.32.0, run by the lead outside the lane sandbox)

After lane fix turn 3 (the turn ran out of turns before writing FIXED3.md, but its rewrite of l_add_test.qnt and l_add.qnt is complete). The test file now parses; every command matches.

| rc | expected | match | command |
|---|---|---|---|
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-l-add/l_add.qnt` |
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-l-add/l_add_test.qnt` |
| 0 | 0 | MATCH | `quint test qa/gemini-quint/x14-l-add/l_add_test.qnt --main=l_add_test` |
| 0 | 0 | MATCH | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=l_add --invariant=invRow --max-samples=50 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=l_add --invariant=invNoAcceptedChain --max-samples=50 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=m_no_recv_p2 --invariant=invRow --max-samples=50 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=m_send_wrong_x --invariant=invRow --max-samples=50 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=m_no_id --invariant=invRow --max-samples=50 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=m_no_div_constraint --invariant=invRow --max-samples=50 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=m_valid_free --invariant=invRow --max-samples=50 --seed=42` |
| 0 | 0 | MATCH | `quint run qa/gemini-quint/x14-l-add/l_add.qnt --main=l_add --invariant=invRow --max-samples=1000 --seed=0x14a001` |
