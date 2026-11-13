# Soundness and Completeness Review: X14 BLS12-381 $G_1$ Chain `Y3` Row Kind

## 1. Verdict

**SOUND/COMPLETE** (with explicit cross-chip preconditions **C1–C4** below).

Specifically:
1. **Local & Bus-Coupled Soundness of `Y3`:** Within `ChainAir` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L237-L498)), the `Y3` STARK AIR row kind (`ChainOp::Y3`, [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L254)) is **algebraically and bus-interaction sound** relative to its three upstream `PermutationCheckBus` receptions (`o+4`, `o+5`, `o+6`) and one downstream emission (`o+7`), **provided** the four system-level preconditions **C1–C4** hold:
   - **C1 (Distinct Active `id`s per Chain Bus Offset $o$):** Across all active rows (`is_valid == 1`) emitted into a given chain bus offset $o \in \{1100, 1110\}$, the `id` witness values must be pairwise distinct (or every active row sharing an `id` must carry identical $(x_1, y_1, x_2, y_2)$ inputs). If two active rows share the same `id` with distinct points, `Y3` is vulnerable to a **cross-row coordinate-swap forgery** on buses `o+5` and `o+6` (Finding **F1**, Question 1).
   - **C2 (Non-Exceptional Affine Curve Inputs in `L`):** For `g1_add` (`o = 1100`), active inputs on bus `o+0` and `o+1` must satisfy $x_1 \not\equiv x_2 \pmod p$. In particular, if $P_1 = P_2$ ($x_1 = x_2$ and $y_1 = y_2$) is fed to `g1_add`, `L_add`'s polynomial $(x_2 - x_1)\lambda - (y_2 - y_1) \equiv 0 \pmod p$ degenerates to $0 \cdot \lambda - 0 \equiv 0 \pmod p$, leaving $\lambda$ **completely unconstrained** in $[0, p)$ while `Y3` faithfully computes $y_3 = \lambda(x_1 - x_3) - y_1 \pmod p$ for the forged $\lambda$ (Finding **F2**, Question 2). For `g1_double` (`o = 1110`), active inputs must lie on $E(\mathbb{F}_p)$ (establishing $y_1 \not\equiv 0 \pmod p$); an off-curve input $(0, 0)$ causes $(2y_1)\lambda - 3x_1^2 \equiv 0 \pmod p$ in `L_dbl` to degenerate to $0 \cdot \lambda - 0 \equiv 0 \pmod p$ (Finding **F3**, Question 2).
   - **C3 (Upstream Limb Range-Check on `o+0`):** The upstream chip sending $(id, x_1, y_1)$ on bus `o+0` must range-check every limb of $x_1$ and $y_1$ to $[0, 2^8)$ (via `RANGE_BUS = 1000`). While $\lambda$ and $x_3$ are already range-checked to $[0, 2^8)$ by `L` and `X3` as their respective `Var(0)` outputs, `L` does **not** range-check its `Input(0)` ($x_1$) or `Input(1)` ($y_1$). If an unchecked producer such as test harness `SourceAir` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L118-L216)) supplies un-range-checked BabyBear elements for $x_1$ or $y_1$, linear combinations in `CheckCarryModToZeroSubAir` can wrap modulo the BabyBear prime $p_{\text{BB}} = 2013265921$ (Finding **F4**, Question 3).
   - **C4 (Balanced Downstream Consumer on Bus `o+7`):** The downstream consumer on bus `o+7` must receive `(id, x3, y3)` with multiplicity gated by a boolean flag (`count_weight = 1`), matching `Y3`'s emission.
2. **Completeness of `Y3`:** Under **C1–C4**, for every honest active row (`is_valid = 1`) on valid non-exceptional affine points ($x_1 \not\equiv x_2 \pmod p$ for `g1_add`, $y_1 \not\equiv 0 \pmod p$ for `g1_double`, $P_i \neq \mathcal{O}$) and every inactive padding row (`is_valid = 0`), there exists a canonical witness in $[0, p)$ with limbs in $[0, 2^8)$ that satisfies all `Y3` local polynomial constraints, range checks, and bus multiset balances (`o+4`, `o+5`, `o+6`, `o+7`) (Question 4).

---

## 2. Circuit Architecture and As-Built Rust Trace Layout

### 2.1 Decomposition of $G_1$ Group Operations Across `L`, `X3`, and `Y3`

In [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L397-L448), `g1_chain(start_bus, is_double)` constructs three `ChainAir` instances over the BLS12-381 base field modulus $p$:

$$
p = \text{0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab}
$$

 represented in `LIMBS = 48` limbs of `LIMB_BITS = 8` bits each ($48 \times 8 = 384$ bits):

1. **Chip 1 (`L`, `ChainOp::LAdd` or `ChainOp::LDbl`):**
   - For `g1_add` (`start_bus = 1100`, `is_double = false`):
     - Inputs: `x1 = Input(0)`, `y1 = Input(1)`, `x2 = Input(2)`, `y2 = Input(3)`.
     - Saved witness: `lambda = (y2 - y1) / (x2 - x1)` via `FieldVariable::div` ([lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs#L211-L237)), allocated as `Var(0)` and constraining:
       $$\text{is\_valid} \cdot \bigl((x_2 - x_1)\cdot \lambda - (y_2 - y_1) - q_L \cdot p\bigr) = 0.$$
   - For `g1_double` (`start_bus = 1110`, `is_double = true`):
     - Inputs: `x1 = Input(0)`, `y1 = Input(1)`.
     - Saved witness: `lambda = (3 * x1^2) / (2 * y1)` via `FieldVariable::div`, allocated as `Var(0)` and constraining:
       $$\text{is\_valid} \cdot \bigl((2 y_1)\cdot \lambda - 3 x_1^2 - q_L \cdot p\bigr) = 0.$$
2. **Chip 2 (`X3`, `ChainOp::X3`):**
   - Inputs: `lambda = Input(0)`, `x1 = Input(1)`, `x2 = Input(2)` (where `x2 = x1.clone()` in `g1_double`, allocating `Input(2)` and constraining `Input(2) == Input(1)` at [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L351-L355)).
   - Saved witness: `x3 = (lambda * lambda - (x1 + x2)).save()` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L422-L423)), allocated as `Var(0)` and constraining:
     $$\text{is\_valid} \cdot \bigl(\lambda^2 - (x_1 + x_2) - x_3 - q_{X3} \cdot p\bigr) = 0.$$
3. **Chip 3 (`Y3`, `ChainOp::Y3` — Under Review):**
   - Constructed at [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L428-L440):
     ```rust
     let expr = {
         let mut builder = ExprBuilder::new_bls12_381();
         let lambda = builder.new_input(); // Input(0)
         let x1 = builder.new_input();     // Input(1)
         let x3 = builder.new_input();     // Input(2)
         let y1 = builder.new_input();     // Input(3)
         let mut dx = x1 - x3;
         let mut l_dx = lambda * dx;
         let mut y3 = (l_dx - y1).save();  // Var(0)
         builder.build(vec![y3])
     };
     ```
   - `dx = x1 - x3` and `l_dx = lambda * dx` are not saved beforehand, nor do their symbolic limb degree/overflow trackers exceed the auto-save thresholds in `FieldVariable::sub` or `FieldVariable::mul` ([lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs#L113-L209): `x1 - x3` has `max_Wait = 2`, `max_abs = 2^8`; `lambda * dx` has `max_terms = 2 * 48 = 96 <= 1 << (29 - 16) = 8192` and `max_abs = 48 * 2^8 * 2^8 = 3,145,728 <= 1 << (29 - 8) = 2,097,152` — wait: let's check whether `lambda * dx` triggers auto-save in `mul`!).

### 2.2 Detailed Trace Column Layout of `Y3` (`ChainAir` + `FieldExpr`)

Let us verify the exact auto-save arithmetic in `FieldVariable::sub` and `FieldVariable::mul` ([lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs#L140-L209)):
- For inputs `lambda`, `x1`, `x3`, `y1`:
  - `limb_bits = 8`, `num_limbs = 48`, `max_overflow_bits = 29` ([lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs#L53)).
  - Each `new_input()` has `expr_limbs = 48`, `max_coeff = 1`, `max_abs = (1 << 8) - 1 = 255` ([lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs#L69-L83)).
- In `dx = x1 - x3` (`FieldVariable::sub`, [lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs#L140-L167)):
  - `sum_abs = 255 + 255 = 510`, `limit_abs = 1 << 29 = 536,870,912`. Since `510 <= limit_abs`, `x1` and `x3` are not saved.
  - Resulting `dx` has `expr_limbs = 48`, `max_coeff = 2`, `max_abs = 510`.
- In `l_dx = lambda * dx` (`FieldVariable::mul`, [lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs#L176-L209)):
  - `max_terms = 1 * 2 * 48 = 96 <= 1 << (29 - 2 * 8) = 1 << 13 = 8192`.
  - `prod_abs = 48 * 255 * 510 = 6,242,400`.
  - `limit_abs = 1 << 29 = 536,870,912`. Since `6,242,400 <= 536,870,912`, neither `lambda` nor `dx` is auto-saved!
  - Resulting `l_dx` has `expr_limbs = 48 + 48 - 1 = 95`, `max_coeff = 96`, `max_abs = 6,242,400`.
- In `l_dx - y1` (`FieldVariable::sub`):
  - `sum_abs = 6,242,400 + 255 = 6,242,655 <= 536,870,912`.
  - Resulting `l_dx - y1` has `expr_limbs = 95`, `max_coeff = 97`, `max_abs = 6,242,655`.
- In `y3 = (l_dx - y1).save()` ([lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs#L53-L68)):
  - Allocates `Var(0)` ($y_3$, 48 limbs) and records the single constraint:
    $$\text{expr} = (\lambda \cdot (x_1 - x_3) - y_1) - y_3 \equiv 0 \pmod p.$$
  - In `ExprBuilder::build` ([lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs#L102-L163)):
    - `num_inputs = 4` (`lambda`, `x1`, `x3`, `y1` $\implies 4 \times 48 = 192$ columns).
    - `num_vars = 1` (`y3` $\implies 1 \times 48 = 48$ columns).
    - `q_limbs = [95 + 1 - 48] = [48]` (48 quotient limb columns).
    - `carry_limbs = [max(95, 48 + 48 - 1)] = [95]` (95 carry limb columns).
    - Plus `ChainAir`'s 2 prefix columns (`id` at col 0, `is_valid` at col 1, [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L298-L303)).

| Column Slice | Width | Symbol / Role in `Y3` (`ChainAir`) | Range-Checked in `Y3`? |
| :--- | :--- | :--- | :--- |
| `col[0]` | 1 | `id`: Row / request identifier | **No** (unconstrained field element) |
| `col[1]` | 1 | `is_valid`: Row validity selector (`assert_bool`) | **Yes** ($\text{is\_valid} \in \{0, 1\}$) |
| `col[2..50]` | 48 | `Input(0)` = $\lambda$ (received on bus `o+4`) | **No** in `Y3` (range-checked in `L` as `Var(0)`) |
| `col[50..98]` | 48 | `Input(1)` = $x_1$ (received on bus `o+4`) | **No** in `Y3` or `L` (depends on `o+0` producer) |
| `col[98..146]` | 48 | `Input(2)` = $x_3$ (received on bus `o+6`) | **No** in `Y3` (range-checked in `X3` as `Var(0)`) |
| `col[146..194]` | 48 | `Input(3)` = $y_1$ (received on bus `o+5`) | **No** in `Y3` or `L` (depends on `o+0` producer) |
| `col[194..242]` | 48 | `Var(0)` = $y_3$ (emitted on bus `o+7`) | **Yes** (`RANGE_BUS = 1000`, 8-bit limbs) |
| `col[242..290]` | 48 | Quotient $q$ limbs for $\lambda(x_1 - x_3) - y_1 - y_3 = q \cdot p$ | **Yes** (`RANGE_BUS = 1000`, offset $+128$) |
| `col[290..385]` | 95 | Carry $c_i$ limbs (`CheckCarryModToZeroSubAir`) | **Yes** (`RANGE_BUS = 1000`, 16-bit signed) |
| **Total Width** | **385** | `ChainAir::width()` for `ChainOp::Y3` | — |

### 2.3 Bus Interaction Topology (`o = 1100` for `g1_add`, `o = 1110` for `g1_double`)

In [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L312-L393), `ChainAir::eval` wires eight `PermutationCheckBus` indices `o+0..o+7`:

| Bus Index | `g1_add` | `g1_double` | Sender (`bus.send`) | Receiver (`bus.receive`) | Tuple Payload | Msg Len (Cols) | Mult |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `o+0` | `1100` | `1110` | Upstream producer | `L` (`LAdd` / `LDbl`) | `(id, x1, y1)` | $1 + 48 + 48 = 97$ | `is_valid` |
| `o+1` | `1101` | *(unused)* | Upstream producer | `L` (`LAdd` only) | `(id, x2, y2)` | $1 + 48 + 48 = 97$ | `is_valid` |
| `o+2` | `1102` | *(unused)* | `L` (`LAdd` only) | `X3` (`!is_double`) | `(id, x2)` | $1 + 48 = 49$ | `is_valid` |
| `o+3` | `1103` | `1113` | `L` (`LAdd` / `LDbl`) | `X3` | `(id, lambda, x1)` | $1 + 48 + 48 = 97$ | `is_valid` |
| `o+4` | `1104` | `1114` | `L` (`LAdd` / `LDbl`) | **`Y3`** | `(id, lambda, x1)` | $1 + 48 + 48 = 97$ | `is_valid` |
| `o+5` | `1105` | `1115` | `L` (`LAdd` / `LDbl`) | **`Y3`** | `(id, y1)` | $1 + 48 = 49$ | `is_valid` |
| `o+6` | `1106` | `1116` | `X3` | **`Y3`** | `(id, x3)` | $1 + 48 = 49$ | `is_valid` |
| `o+7` | `1107` | `1117` | **`Y3`** | Downstream sink | `(id, x3, y3)` | $1 + 48 + 48 = 97$ | `is_valid` |

Wait: let us check the interaction parameters in [lane-src/interaction_mod.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/interaction_mod.rs#L129-L176):
- Every bus message has length $49 \le 128$ or $97 \le 128$, satisfying `max_message_length = 128` in `SymbolicInteraction` ([lane-src/interaction_mod.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/interaction_mod.rs#L140)).
- Every send/receive passes `is_valid` (`AB::Expr` of degree 1) as `count` with `count_weight = 1`.
- Furthermore, in `Y3`, the same column slice `x3_limbs` (`Input(2)`, `col[98..146]`) is passed both to `bus.receive` on `o+6` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L368-L372)) and to `bus.send` on `o+7` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L387-L391)). Thus `Y3` cannot receive one $x_3$ on `o+6` and emit a different $x_3'$ on `o+7` within the same row.

---

## 3. Formal Refinement Specification (`qa/gemini-quint/x14-y3/y3.qnt`)

### 3.1 Miniature Curve Model Over $\mathbb{F}_{13}$

In [y3.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3.qnt) and [y3_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt), we instantiate the full 4-stage pipeline (`L`, `X3`, `Y3`, `Sink`) over the prime field $\mathbb{F}_{13}$ ($p = 13$) and the short Weierstrass curve:

$$
E(\mathbb{F}_{13}) : y^2 \equiv x^3 + 4 \pmod{13}
$$

which has the exact same curve equation $y^2 = x^3 + 4$ as BLS12-381 $G_1$ and preserves all algebraic identities, bus couplings, and exceptional cases:
- **Addition witness 0 (`g1_add`):** $P_1 = (0, 2) \in E(\mathbb{F}_{13})$ ($2^2 = 4 \equiv 0^3 + 4$) and $P_2 = (2, 5) \in E(\mathbb{F}_{13})$ ($5^2 = 25 \equiv 12 \equiv 2^3 + 4$).
  - $\lambda = (5 - 2) \cdot (2 - 0)^{-1} = 3 \cdot 7 = 21 \equiv 8 \pmod{13}$.
  - $x_3 = 8^2 - 0 - 2 = 62 \equiv 10 \pmod{13}$.
  - $y_3 = 8 \cdot (0 - 10) - 2 = 8 \cdot 3 - 2 = 22 \equiv 9 \pmod{13}$, and $(10, 9) \in E(\mathbb{F}_{13})$ ($9^2 = 81 \equiv 3 \equiv 10^3 + 4 \pmod{13}$).
- **Addition witness 1 (`g1_add`):** $P_3 = (4, 4) \in E(\mathbb{F}_{13})$ and $P_4 = (7, 3) \in E(\mathbb{F}_{13})$.
  - $\lambda = (3 - 4) \cdot (7 - 4)^{-1} = 12 \cdot 9 = 108 \equiv 4 \pmod{13}$.
  - $x_3 = 4^2 - 4 - 7 = 5 \pmod{13}$.
  - $y_3 = 4 \cdot (4 - 5) - 4 = 4 \cdot 12 - 4 = 44 \equiv 5 \pmod{13}$, and $(5, 5) \in E(\mathbb{F}_{13})$.
- **Doubling witness 0 (`g1_double`):** $P_2 = (2, 5) \in E(\mathbb{F}_{13})$.
  - $\lambda = (3 \cdot 2^2) \cdot (2 \cdot 5)^{-1} = 12 \cdot 10^{-1} = 12 \cdot 4 = 48 \equiv 9 \pmod{13}$.
  - $x_3 = 9^2 - 2 \cdot 2 = 77 \equiv 12 \pmod{13}$.
  - $y_3 = 9 \cdot (2 - 12) - 5 = 9 \cdot 3 - 5 = 22 \equiv 9 \pmod{13}$, and $(12, 9) \in E(\mathbb{F}_{13})$.

### 3.2 Refinement Invariant `invRow` and Non-Vacuity Invariant `invNonVacuous`

In [y3.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3.qnt#L316-L361), the refinement invariant `invRow` states that whenever `verifierAccepts(tr, M_ID)` and `distinctActiveIds(tr)` hold:
1. Every active `Y3` row (`yr.is_valid == 1`) matches a unique active upstream `(lr, xr)` pair (`lr.id == yr.id == xr.id`) and satisfies:
   $$
   \begin{aligned}
   \text{yr.lam} &= \text{lr.lam}, \quad \text{yr.x1} = \text{lr.x1}, \quad \text{yr.y1} = \text{lr.y1}, \\
   \text{yr.x3} &= \text{xr.x3} = (\text{lr.lam}^2 - \text{lr.x1} - \text{lr.x2}) \bmod p, \\
   \text{yr.y3} &= \text{affineY3}(\text{lr.lam}, \text{lr.x1}, \text{xr.x3}, \text{lr.y1}) = \bigl(\text{lr.lam} \cdot (\text{lr.x1} - \text{xr.x3}) - \text{lr.y1}\bigr) \bmod p.
   \end{aligned}
   $$
2. Every active downstream sink row (`sr.is_valid == 1`) on bus `o+7` matches an active upstream `(lr, xr)` pair of the same `id` with:
   $$
   \text{sr.out\_x} = \text{xr.x3}, \qquad \text{sr.out\_y} = \bigl(\text{lr.lam} \cdot (\text{lr.x1} - \text{xr.x3}) - \text{lr.y1}\bigr) \bmod p.
   $$
3. Total active sink receptions equal total active `Y3` rows: $\sum \text{sr.is\_valid} = \sum \text{yr.is\_valid}$.

---

## 4. Summary of Findings

| Finding ID | Severity | Location in `lane-src/` | Summary of Finding | Quint Witness / Test in `y3_test.qnt` | Required Closing Condition / Fix |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **F1** | **Medium (Precondition C1)** | [x_exp_node.rs:299-303](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L299-L303), [x_exp_node.rs:358-392](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L358-L392) | `id` (`col[0]`) is neither range-checked nor constrained to be unique across active rows in `ChainAir`. If two active rows share the same `id`, `Y3` can swap their $y_1$ messages on bus `o+5` and/or their $x_3$ messages on bus `o+6`, forging off-curve outputs that pass all STARK and LogUp checks. | `duplicateIdSwapAttackTest`, `duplicateIdSwapX3AttackTest` | Upstream producer on `o+0` must assign strictly distinct `id` values per active row (e.g., monotonic row index), or `Y3` must receive $(id, \lambda, x_1, y_1)$ in a single bus tuple (`1 + 3*48 = 145` exceeds `max_message_length = 128`, so distinct `id` is required). |
| **F2** | **High (Precondition C2 — `g1_add`)** | [x_exp_node.rs:403-408](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L403-L408), [field_variable.rs:211-237](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs#L211-L237) | In `L_add`, `FieldVariable::div(y2 - y1, x2 - x1)` enforces $(x_2 - x_1)\lambda - (y_2 - y_1) \equiv 0 \pmod p$ without checking $x_2 - x_1 \not\equiv 0 \pmod p$. When $P_1 = P_2$ ($x_1 = x_2, y_1 = y_2$), $0 \cdot \lambda \equiv 0 \pmod p$ holds for **every** $\lambda \in \mathbb{F}_p$, allowing the prover to forge $p$ distinct outputs $(x_3, y_3)$ via `X3` and `Y3`. | `exceptionalSamePointLambdaFreeTest` | Upstream caller of `g1_add` must guarantee $x_1 \not\equiv x_2 \pmod p$, or `L_add` must allocate an inverse witness $\text{inv\_dx}$ and constrain $\text{is\_valid} \cdot ((x_2 - x_1)\cdot \text{inv\_dx} - 1) \equiv 0 \pmod p$. |
| **F3** | **Medium (Precondition C2 — `g1_double`)** | [x_exp_node.rs:410-416](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L410-L416), [field_variable.rs:211-237](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs#L211-L237) | In `L_dbl`, `FieldVariable::div(3*x1^2, 2*y1)` enforces $(2y_1)\lambda - 3x_1^2 \equiv 0 \pmod p$. For any on-curve point on $y^2 = x^3 + 4$ with $y_1 = 0$, $x_1 \neq 0$ makes the constraint unsatisfiable; however, if an off-curve point $(0, 0)$ is fed on `o+0`, $0 \cdot \lambda \equiv 0 \pmod p$ leaves $\lambda$ completely free. | `exceptionalDoubleZeroPointLambdaFreeTest`, `exceptionalInversePointUnsatisfiableTest` | Upstream caller of `g1_double` must only feed on-curve points $P_1 \in E(\mathbb{F}_p) \setminus \{\mathcal{O}\}$ with $y_1 \not\equiv 0 \pmod p$, or `L_dbl` must invert $2y_1$. |
| **F4** | **High (Precondition C3 — `o+0` Range Checks)** | [builder.rs:248-308](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs#L248-L308), [x_exp_node.rs:171-216](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L171-L216) | `FieldExpr::eval` range-checks `vars`, `q_limbs`, and `carry_limbs`, **never** `inputs`. In `Y3`, $\lambda$ (`Input(0)`) and $x_3$ (`Input(2)`) are already range-checked in `L` and `X3` as `Var(0)`, **but** $x_1$ (`Input(1)`) and $y_1$ (`Input(3)`) are `Input(0)` and `Input(1)` in `L`, so `L` does not range-check them either. If the `o+0` producer (such as `SourceAir`) omits 8-bit limb range checks on $x_1, y_1$, limbs up to $p_{\text{BB}} - 1$ overflow `CheckCarryModToZeroSubAir` modulo $p_{\text{BB}} = 2013265921$. | Analyzed in Section 7 (`CheckCarryModToZeroSubAir` BabyBear overflow bound) | Any upstream producer sending on `o+0` (and `o+1`) must range-check all 48 limbs of $x_1, y_1$ (and $x_2, y_2$) to $[0, 2^8)$ on `RANGE_BUS = 1000`. |
| **F5** | **Informational (Bus Length Limit)** | [interaction_mod.rs:140](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/interaction_mod.rs#L140), [x_exp_node.rs:344-391](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L344-L391) | Why `L` splits `(id, lambda, x1)` on `o+4` (97 cols) and `(id, y1)` on `o+5` (49 cols): combining `(id, lambda, x1, y1)` into one bus message would require $1 + 3 \times 48 = 145 > 128 = \text{max\_message\_length}$ columns. Splitting onto `o+4` and `o+5` respects the 128-column limit, making `id` uniqueness (**F1**) essential to bind `o+4`, `o+5`, and `o+6` together. | `completenessAddTest`, `duplicateIdSwapAttackTest` | Keep `o+4`, `o+5`, `o+6` split as built, and enforce `id` uniqueness in the upstream dispatcher. |
| **F6** | **Verified Sound (All 5 Mutants Killed)** | [x_exp_node.rs:365-391](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L365-L391), [x_exp_node.rs:428-440](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L428-L440) | Dropping `o+6` receive (`m_no_recv_x3`), emitting `x1` instead of `x3` on `o+7` (`m_out_wrong_x`), dropping `o+5` receive (`m_no_recv_y1`), flipping `x1 - x3` to `x3 - x1` (`m_sign_flip`), or sending on `o+7` with multiplicity `1` (`m_valid_free`) each immediately breaks `invRow` with an explicit attack trace. | `mutantNoRecvX3CaughtTest`, `mutantOutWrongXCaughtTest`, `mutantNoRecvY1CaughtTest`, `mutantSignFlipCaughtTest`, `mutantValidFreeCaughtTest` | None (as-built `Y3` constraints and bus bindings are minimal and necessary). |

---

## 5. Detailed Analysis of Question (1): `id` Column Guarantees and Duplicate-`id` Forgery

> **Question (1):** What is `id` (`local[0]`), and is it constrained to be unique or sequential across rows? Does `Y3` guarantee that `(lambda, x1)` from `o+4`, `y1` from `o+5`, and `x3` from `o+6` belong to the same logical operation if rows are permuted or if two active rows share the same `id`?

### 5.1 What `ChainAir` Constrains on `id` (`local[0]`)

In [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L298-L393):
- `let id = local[0].clone();` ([line 299](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L299)) extracts column 0 of the trace row.
- `let is_valid = local[1].clone(); builder.assert_bool(is_valid.clone());` ([lines 300–301](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L300-L301)) constrains `is_valid` $\in \{0, 1\}$.
- **There are zero local constraints on `id` inside `ChainAir`:**
  1. `id` is **not** range-checked (it is an arbitrary BabyBear field element $\in \mathbb{F}_{p_{\text{BB}}}$).
  2. `id` has **no** transition constraint (e.g., `next.id - local.id == 1` is never asserted, and `ChainAir` is purely single-row plus LogUp bus interactions).
  3. `id` is **not** locally constrained to be distinct across active rows.
- Instead, within each active row of `Y3`, the **same single trace cell** `id = local[0]` is prepended to all four bus tuples:
  - Bus `o+4` receive: `(id, lambda, x1)` with multiplicity `is_valid` ([lines 360–364](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L360-L364)).
  - Bus `o+5` receive: `(id, y1)` with multiplicity `is_valid` ([lines 365–367](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L365-L367)).
  - Bus `o+6` receive: `(id, x3)` with multiplicity `is_valid` ([lines 368–372](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L368-L372)).
  - Bus `o+7` send: `(id, x3, y3)` with multiplicity `is_valid` ([lines 387–391](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L387-L391)).

### 5.2 Case A: Arbitrary Row Permutations Under Distinct Active `id`s (Sound)

Suppose the upstream producer emits $k$ active operations with pairwise distinct identifiers $\text{id}_1, \dots, \text{id}_k$.
Even if the prover arbitrarily permutes the row order of `L`, `X3`, and `Y3` independently:
- For each active row $r$ of `Y3` with identifier $\text{id}^* = \text{local}_r[0]$:
  - Bus `o+4` multiset equality forces $(\text{id}^*, \lambda_r, x_{1,r})$ to equal the unique tuple sent by `L` with identifier $\text{id}^*$.
  - Bus `o+5` multiset equality forces $(\text{id}^*, y_{1,r})$ to equal the unique tuple sent by `L` with identifier $\text{id}^*$.
  - Bus `o+6` multiset equality forces $(\text{id}^*, x_{3,r})$ to equal the unique tuple sent by `X3` with identifier $\text{id}^*$.
- Because $\text{id}^*$ is unique among active rows in `L` and `X3`, `Y3` row $r$ is **uniquely bound** to the exact $(\lambda, x_1, y_1, x_3)$ of logical operation $\text{id}^*$, regardless of how rows are ordered in `L`, `X3`, or `Y3`.

### 5.3 Case B: Two Active Rows Share the Same `id` (Unsound — Cross-Row Mix-and-Match Forgery)

If two active rows $r = 0$ and $r = 1$ share the **same** identifier $\text{id}_0 = \text{id}_1 = \text{id}^*$, the LogUp multisets on `o+4`, `o+5`, and `o+6` are checked **independently** per bus:
- Bus `o+4` multiset: $\{(\text{id}^*, \lambda^{(0)}, x_1^{(0)}), (\text{id}^*, \lambda^{(1)}, x_1^{(1)})\}$.
- Bus `o+5` multiset: $\{(\text{id}^*, y_1^{(0)}), (\text{id}^*, y_1^{(1)})\}$.
- Bus `o+6` multiset: $\{(\text{id}^*, x_3^{(0)}), (\text{id}^*, x_3^{(1)})\}$.

Because `o+4`, `o+5`, and `o+6` are separate `PermutationCheckBus` instances, an adversarial prover can **permute the matching on `o+5` and/or `o+6` relative to `o+4`**:
1. **Attack 1A (`y1`-Swap on Bus `o+5`, `duplicateIdSwapAttackTest` in `y3_test.qnt`):**
   - Row 0 (`id = 1`): $P_1 = (0, 2), P_2 = (2, 5) \implies \lambda^{(0)} = 8, x_1^{(0)} = 0, y_1^{(0)} = 2, x_3^{(0)} = 10$ (honest $y_3^{(0)} = 9$).
   - Row 1 (`id = 1`): $P_3 = (4, 4), P_4 = (7, 3) \implies \lambda^{(1)} = 4, x_1^{(1)} = 4, y_1^{(1)} = 4, x_3^{(1)} = 5$ (honest $y_3^{(1)} = 5$).
   - In `Y3`, the prover swaps $y_1^{(0)} \leftrightarrow y_1^{(1)}$ between rows 0 and 1 while keeping $(\lambda, x_1)$ and $x_3$ unswapped:
     - `Y3` row 0 receives $(1, 8, 0)$ on `o+4`, $(1, \mathbf{4})$ on `o+5`, $(1, 10)$ on `o+6`, and computes:
       $$y_3^{(0,\text{forged})} = 8 \cdot (0 - 10) - \mathbf{4} = 24 - 4 = 20 \equiv \mathbf{7} \not\equiv 9 \pmod{13}.$$
     - `Y3` row 1 receives $(1, 4, 4)$ on `o+4`, $(1, \mathbf{2})$ on `o+5`, $(1, 5)$ on `o+6`, and computes:
       $$y_3^{(1,\text{forged})} = 4 \cdot (4 - 5) - \mathbf{2} = 9 - 2 = \mathbf{7} \not\equiv 5 \pmod{13}.$$
   - Neither forged output $(10, 7)$ nor $(5, 7)$ lies on $E(\mathbb{F}_{13})$ ($7^2 = 49 \equiv 10 \pmod{13}$, whereas $10^3 + 4 \equiv 3$ and $5^3 + 4 \equiv 12$), yet **every STARK polynomial constraint and every bus multiset (`o+2..o+7`) is 100% satisfied**!
2. **Attack 1B (`x3`-Swap on Bus `o+6`, `duplicateIdSwapX3AttackTest` in `y3_test.qnt`):**
   - Swapping $x_3^{(0)} = 10 \leftrightarrow x_3^{(1)} = 5$ on bus `o+6` under `id = 1` causes `Y3` row 0 to compute:
     $$y_3^{(0,\text{forged})} = 8 \cdot (0 - \mathbf{5}) - 2 = 8 \cdot 8 - 2 = 62 \equiv \mathbf{10} \pmod{13}$$
     and `Y3` row 1 to compute:
     $$y_3^{(1,\text{forged})} = 4 \cdot (4 - \mathbf{10}) - 4 = 4 \cdot 7 - 4 = 24 \equiv \mathbf{11} \pmod{13},$$
     emitting forged off-curve points $(5, 10)$ and $(10, 11)$ on `o+7` that are accepted by the verifier.

| Row / Chip | `id` | `is_valid` | `o+4`: `(id, lam, x1)` | `o+5`: `(id, y1)` | `o+6`: `(id, x3)` | `Y3` Computed `y3` | Honest `(x3, y3)` | Emitted `o+7`: `(id, x3, y3)` | On $E(\mathbb{F}_{13})$? |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Honest Row 0 | `1` | `1` | `(1, 8, 0)` | `(1, 2)` | `(1, 10)` | `9` | `(10, 9)` | `(1, 10, 9)` | Yes ($9^2 \equiv 3 \equiv 10^3+4$) |
| Honest Row 1 | `2` | `1` | `(2, 4, 4)` | `(2, 4)` | `(2, 5)` | `5` | `(5, 5)` | `(2, 5, 5)` | Yes ($5^2 \equiv 12 \equiv 5^3+4$) |
| **Attack 1A Row 0 (`y1` swap)** | **`1`** | `1` | `(1, 8, 0)` | **`(1, 4)`** | `(1, 10)` | **`7`** | `(10, 9)` | **`(1, 10, 7)`** | **No** ($7^2 \equiv 10 \not\equiv 3$) |
| **Attack 1A Row 1 (`y1` swap)** | **`1`** | `1` | `(1, 4, 4)` | **`(1, 2)`** | `(1, 5)` | **`7`** | `(5, 5)` | **`(1, 5, 7)`** | **No** ($7^2 \equiv 10 \not\equiv 12$) |
| **Attack 1B Row 0 (`x3` swap)** | **`1`** | `1` | `(1, 8, 0)` | `(1, 2)` | **`(1, 5)`** | **`10`** | `(10, 9)` | **`(1, 5, 10)`** | **No** ($10^2 \equiv 9 \not\equiv 12$) |
| **Attack 1B Row 1 (`x3` swap)** | **`1`** | `1` | `(1, 4, 4)` | `(1, 4)` | **`(1, 10)`** | **`11`** | `(5, 5)` | **`(1, 10, 11)`** | **No** ($11^2 \equiv 4 \not\equiv 3$) |

---

## 6. Detailed Analysis of Question (2): Exceptional Curve Inputs and Division-by-Zero

> **Question (2):** Does `Y3` itself have any division or exceptional cases, or are all exceptional cases ($x_1 = x_2$ in `g1_add`, $y_1 = 0$ in `g1_double`, point at infinity $\mathcal{O}$) upstream in `L`? What happens in the full chain (`L`, `X3`, `Y3`) if an exceptional input is fed?

### 6.1 `Y3` Has Zero Divisions of Its Own

In [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L428-L440), `Y3` evaluates the purely polynomial expression:

$$
y_3 \equiv \lambda \cdot (x_1 - x_3) - y_1 \pmod p.
$$

`Y3` performs **no field inversion or division**: for any quadruple $(\lambda, x_1, x_3, y_1) \in [0, p)^4$, there is a unique canonical $y_3 \in [0, p)$ satisfying `Y3`'s constraint. All divisions occur upstream in `L` via `FieldVariable::div` ([lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs#L211-L237)), which compiles `a / b` into the polynomial constraint:

$$
\text{is\_valid} \cdot \bigl(b \cdot \lambda - a - q \cdot p\bigr) = 0
$$

**without** asserting that the denominator $b$ is invertible ($b \not\equiv 0 \pmod p$).

### 6.2 Exhaustive Case Analysis of Exceptional Inputs in `L` $\to$ `X3` $\to$ `Y3`

1. **Case 2A (`g1_add` with Inverse Points $P_1 = -P_2$, i.e. $x_1 \equiv x_2 \pmod p$ and $y_1 \not\equiv y_2 \pmod p$):**
   - In `L_add`, the denominator is $b = x_2 - x_1 \equiv 0 \pmod p$ and the numerator is $a = y_2 - y_1 \not\equiv 0 \pmod p$.
   - The constraint in `L_add` becomes:
     $$\text{is\_valid} \cdot \bigl(0 \cdot \lambda - (y_2 - y_1)\bigr) \equiv 0 \pmod p.$$
   - For an active row (`is_valid = 1`), this requires $-(y_2 - y_1) \equiv 0 \pmod p$, which is **unsatisfiable** for every $\lambda \in \mathbb{F}_p$.
   - **Effect:** Completeness failure for $P_1 + (-P_1) = \mathcal{O}$ (no valid active witness exists; verified by `exceptionalInversePointUnsatisfiableTest` in [y3_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L220-L231)).
2. **Case 2B (`g1_add` with Identical Points $P_1 = P_2$, i.e. $x_1 \equiv x_2 \pmod p$ and $y_1 \equiv y_2 \pmod p$ — Critical Soundness Hazard):**
   - In `L_add`, both the denominator $b = x_2 - x_1 \equiv 0 \pmod p$ and the numerator $a = y_2 - y_1 \equiv 0 \pmod p$ vanish!
   - The constraint in `L_add` degenerates to:
     $$\text{is\_valid} \cdot (0 \cdot \lambda - 0) \equiv 0 \pmod p$$
     which holds for **every** $\lambda \in [0, p)$!
   - Downstream, `X3` deterministically computes $x_3 = \lambda^2 - 2x_1 \bmod p$ and `Y3` deterministically computes $y_3 = \lambda(x_1 - x_3) - y_1 \bmod p$ for the prover's arbitrarily chosen $\lambda$.
   - **Concrete $\mathbb{F}_{13}$ Trace (`exceptionalSamePointLambdaFreeTest` in [y3_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L196-L218)):**
     When $P_1 = P_2 = (2, 5) \in E(\mathbb{F}_{13})$ is fed to `g1_add`, the true group sum is $2 P_2 = (12, 9)$ (requiring tangent slope $\lambda = 9$). However, `g1_add` accepts **all 13 choices** of $\lambda \in \mathbb{F}_{13}$:

| Prover's Chosen $\lambda \in \mathbb{F}_{13}$ | `L_add` Check $(2-2)\lambda == (5-5)$ | `X3` Output $x_3 = \lambda^2 - 4 \bmod 13$ | `Y3` Output $y_3 = \lambda(2 - x_3) - 5 \bmod 13$ | Emitted Point $(x_3, y_3)$ on `o+7` | On Curve $E(\mathbb{F}_{13})$? | Equals True $2P_2 = (12, 9)$? |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| $\lambda = 0$ | $0 == 0$ (Pass) | $9$ | $8$ | $(9, 8)$ | **No** ($8^2 \equiv 12 \not\equiv 5$) | **No** |
| $\lambda = 1$ | $0 == 0$ (Pass) | $10$ | $0$ | $(10, 0)$ | **No** ($0^2 \equiv 0 \not\equiv 3$) | **No** |
| $\lambda = 2$ | $0 == 0$ (Pass) | $0$ | $12$ | $(0, 12)$ | **No** ($12^2 \equiv 1 \not\equiv 4$) | **No** |
| $\lambda = 3$ | $0 == 0$ (Pass) | $5$ | $12$ | $(5, 12)$ | **Yes** ($12^2 \equiv 1 \not\equiv 12$ — wait: $5^3+4 \equiv 12$, so **No**!) | **No** |
| $\lambda = 9$ (Tangent) | $0 == 0$ (Pass) | $12$ | $9$ | $(12, 9)$ | **Yes** ($9^2 \equiv 3 \equiv 12^3+4$) | **Yes** |

3. **Case 2C (`g1_double` with $y_1 \equiv 0 \pmod p$):**
   - In `L_dbl`, the constraint is $\text{is\_valid} \cdot \bigl((2y_1)\lambda - 3x_1^2\bigr) \equiv 0 \pmod p$.
   - For any on-curve point $(x_1, 0)$ on $y^2 = x^3 + 4$, $0 = x_1^3 + 4 \implies x_1^3 \equiv -4 \not\equiv 0 \pmod p$, so $x_1 \not\equiv 0 \pmod p$ and $3x_1^2 \not\equiv 0 \pmod p$ (for $p > 3$). Thus $(2 \cdot 0)\lambda \equiv 3x_1^2 \pmod p$ has **no solution** $\lambda \in \mathbb{F}_p$ (unsatisfiable for `is_valid = 1`).
   - However, if an **off-curve** point $(x_1, y_1) = (0, 0)$ is fed to `g1_double`, both $2y_1 = 0$ and $3x_1^2 = 0$ vanish, so $0 \cdot \lambda \equiv 0 \pmod p$ again leaves $\lambda \in \mathbb{F}_p$ **completely unconstrained** (`exceptionalDoubleZeroPointLambdaFreeTest` in [y3_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L233-L246)).
4. **Case 2D (Point at Infinity $\mathcal{O}$):**
   - `ChainAir` uses affine coordinates $(x, y) \in \mathbb{F}_p^2$ with no infinity flag bit; $\mathcal{O}$ cannot be represented as an active row.

---

## 7. Detailed Analysis of Question (3): Input Limb Range Checks in `Y3`

> **Question (3):** Does `Y3` range-check its four inputs (`lambda`, `x1`, `x3`, `y1`), or only its saved variable `y3` (`Var(0)`), quotient limbs, and carry limbs? Why is that safe (or under what condition could un-range-checked inputs break soundness)?

### 7.1 What `FieldExpr::eval` Range-Checks in `Y3`, `X3`, and `L`

Inspection of `FieldExpr::eval` in [lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs#L248-L308) shows that `FieldExpr` emits `RANGE_BUS = 1000` lookups for:
1. **Saved variables (`vars`):** Each limb of `Var(i)` is range-checked to `limb_bits = 8` bits ($[0, 255]$) via `range_bus.lookup(v, 8, is_valid)` ([lines 275–281](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs#L275-L281)).
2. **Quotient limbs (`q_limbs`):** Each limb of $q_i$ is offset by $+128$ (`1 << (limb_bits - 1)`) and range-checked to 8 bits ($q_{i,k} \in [-128, 127]$) ([lines 282–290](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs#L282-L290)).
3. **Carry limbs (`carry_limbs`):** Range-checked inside `CheckCarryModToZeroSubAir` (`overflow_bits` signed decomposition) ([lines 292–307](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs#L292-L307)).
4. **Inputs (`inputs`):** `FieldExpr::eval` emits **zero** range checks on `Input(0)..Input(num_inputs - 1)`!

Let us trace each of `Y3`'s four inputs (`lambda`, `x1`, `x3`, `y1`) upstream across the buses:

| `Y3` Input | Bus Received By `Y3` | Upstream Chip Sending Tuple | Role in Upstream Chip | Range-Checked in Upstream Chip? |
| :--- | :--- | :--- | :--- | :--- |
| `Input(0)` = $\lambda$ | `o+4` (`(id, lambda, x1)`) | `L` (`LAdd` / `LDbl`) | `Var(0)` in `L` | **Yes** (`L` range-checks its `Var(0)` to $[0, 2^8)$) |
| `Input(1)` = $x_1$ | `o+4` (`(id, lambda, x1)`) | `L` (forwarded from `o+0`) | `Input(0)` in `L` | **No** in `L` (depends on `o+0` producer!) |
| `Input(2)` = $x_3$ | `o+6` (`(id, x3)`) | `X3` | `Var(0)` in `X3` | **Yes** (`X3` range-checks its `Var(0)` to $[0, 2^8)$) |
| `Input(3)` = $y_1$ | `o+5` (`(id, y1)`) | `L` (forwarded from `o+0`) | `Input(1)` in `L` | **No** in `L` (depends on `o+0` producer!) |

### 7.2 Quantitative BabyBear Overflow Analysis When `o+0` Inputs Are Not Range-Checked

Why does `CheckCarryModToZeroSubAir` require all input limbs to be bounded in $[0, 2^8)$?
In `Y3`, `CheckCarryModToZeroSubAir` verifies the limb-wise carry equation in the native STARK field (BabyBear, $p_{\text{BB}} = 2013265921 = 15 \cdot 2^{27} + 1 < 2^{31}$):

$$
c_i \cdot 2^8 \equiv \text{expr}_i - q_i \cdot p_i + c_{i-1} \pmod{p_{\text{BB}}}
$$

where for `Y3`:

$$
\text{expr}_i = \sum_{j+k=i} \lambda_j \cdot (x_{1,k} - x_{3,k}) - y_{1,i} - y_{3,i}.
$$

- **When $x_{1,k}, y_{1,i} \in [0, 255]$ (Precondition C3 holds):**
  - Since $\lambda_j, x_{3,k}, y_{3,i} \in [0, 255]$ and $q_j \in [-128, 127], p_k \in [0, 255]$:
    - $|\text{expr}_i| \le 48 \times 255 \times 255 + 255 = 3,121,455$.
    - $|(q \cdot p)_i| \le 48 \times 128 \times 255 = 1,566,720$.
    - $|c_i| < 2^{15} = 32,768$.
    - Total magnitude of the carry relation is at most:
      $$|\text{expr}_i - (q \cdot p)_i + c_{i-1} - c_i \cdot 2^8| \le 3,121,455 + 1,566,720 + 32,768 + 8,388,608 = 13,109,551 \ll \frac{p_{\text{BB}}}{2} \approx 1.006 \times 10^9.$$
  - Because $13,109,551 < p_{\text{BB}} / 2$, equality modulo $p_{\text{BB}}$ implies **exact integer equality over $\mathbb{Z}$**, which upon multiplying by $2^{8i}$ and summing over $i = 0 \dots 94$ proves $\lambda(x_1 - x_3) - y_1 - y_3 = q \cdot p$ over $\mathbb{Z}$.
- **When $x_{1,k}$ or $y_{1,i}$ is NOT range-checked on `o+0` (e.g., `SourceAir` at [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L118-L216)):**
  - An adversarial prover can set limb $y_{1,0}' = y_{1,0} + p_{\text{BB}}$ in $\mathbb{Z}$ (which is identical in $\mathbb{F}_{p_{\text{BB}}}$, or even simpler: set two limbs $y_{1,0}, y_{1,1} \in [0, p_{\text{BB}})$ such that $y_{1,0} + c_0 - 256 c_0 \equiv \dots \pmod{p_{\text{BB}}}$ wraps around $p_{\text{BB}} = 2013265921$).
  - Specifically, choosing $y_{1,0} \in [256, p_{\text{BB}} - 1]$ allows $\text{expr}_0 - (q \cdot p)_0 - c_0 \cdot 2^8$ to equal a non-zero multiple $k \cdot p_{\text{BB}}$ over $\mathbb{Z}$ while vanishing in $\mathbb{F}_{p_{\text{BB}}}$! Since $p_{\text{BB}} = 2013265921$ is not a multiple of $p_{\text{BLS}}$, wrap-around modulo $p_{\text{BB}}$ injects an error of $k \cdot p_{\text{BB}} \cdot 2^{8i} \not\equiv 0 \pmod{p_{\text{BLS}}}$ into the multi-precision equation!
  - **Conclusion on Question (3):** `Y3` does not need to range-check `lambda` or `x3` (already range-checked in `L` and `X3`), and does not need to range-check `x1` or `y1` **if and only if** the upstream chip sending on `o+0` range-checks all 48 limbs of $x_1$ and $y_1$ to $[0, 2^8)$.

---

## 8. Detailed Analysis of Question (4): Completeness of `Y3`

> **Question (4):** For every honest active row (`is_valid = 1`) on valid non-exceptional affine points and every inactive padding row (`is_valid = 0`), does `ChainAir::generate_trace_rows` produce a witness that satisfies all `Y3` constraints and bus balances?

### 8.1 Active Row Completeness (`is_valid = 1`)

In `ChainAir::generate_trace_rows` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L261-L294)):
1. For each active row $r < n$ with valid non-exceptional inputs $(x_1, y_1, x_2, y_2) \in [0, p)^4$:
   - In `L`, $x_2 - x_1 \not\equiv 0 \pmod p$ (for `g1_add`) or $2y_1 \not\equiv 0 \pmod p$ (for `g1_double`), so `FieldExpr::generate_row_slice` computes the unique canonical $\lambda \in [0, p)$ with limbs in $[0, 255]$.
   - In `X3`, `generate_row_slice` computes the unique canonical $x_3 = (\lambda^2 - x_1 - x_2) \bmod p \in [0, p)$ with limbs in $[0, 255]$.
   - In `Y3`, `generate_row_slice` ([lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs#L171-L245)) evaluates:
     $$y_3 = \bigl(\lambda \cdot (x_1 - x_3) - y_1\bigr) \bmod p \in [0, p)$$
     and decomposes $y_3$ into 48 canonical 8-bit limbs in $[0, 255]$.
   - The exact integer polynomial $\lambda(x_1 - x_3) - y_1 - y_3$ is divisible by $p$ with quotient:
     $$|q| = \frac{|\lambda(x_1 - x_3) - y_1 - y_3|}{p} < \frac{p^2 + 2p}{p} = p + 2 < 2^{384}$$
     whose 48 signed limbs $q_{0,k} \in [-128, 127]$ and 95 carries $c_{0,i}$ stay strictly within the 8-bit quotient and 16-bit carry range bounds.
   - All four bus interactions of `Y3` (`o+4`, `o+5`, `o+6`, `o+7`) fire with multiplicity `is_valid = 1`, matching `L`, `X3`, and the downstream sink.

### 8.2 Inactive Padding Row Completeness (`is_valid = 0`)

When the number of active rows $n$ is not a power of two, `generate_trace_rows` pads the trace to $N = 2^{\lceil \log_2 \max(n, 2) \rceil}$ rows ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L266-L269)).
For every padding row $r \in [n, N)$:
- All columns (`id`, `is_valid`, inputs, `y3`, quotient limbs, carry limbs) remain `0`, except `FieldExpr::generate_row_slice` is only called for $r < n$ — wait: for $r \ge n$, the row is left as all zeros (`BabyBear::ZERO`).
- Let us verify that an all-zero padding row (`is_valid = 0`) is accepted by `Y3::eval`:
  - `assert_bool(is_valid)` holds since $0 \in \{0, 1\}$.
  - `FieldExpr::eval` passes `is_valid = 0` to `CheckCarryModToZeroSubAir` and to every `range_bus.lookup(..., is_valid)` ([lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs#L275-L307)), gating both the polynomial carry constraints and the range-bus multiplicities by `is_valid = 0`.
  - Every `PermutationCheckBus` call (`o+4`, `o+5`, `o+6`, `o+7`) receives `is_valid = 0` as its multiplicity, contributing zero weight to the LogUp sum.
- Verified in `completenessAddTest`, `completenessDblTest`, and `paddingRowInactiveCompletenessTest` in [y3_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L11-L72).

---

## 9. Mutation Testing and Verification Commands

### 9.1 The Five Required Mutants in `qa/gemini-quint/x14-y3/y3.qnt`

All five required mutants are implemented as top-level modules in [y3.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3.qnt#L569-L592) and tested both via `quint test` ([y3_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt)) and `quint run`:

| Module Name | Mutant ID | Mutation Applied to `Y3` | Honest Witness Accepted & Refines? | Attack Witness Breaking `invRow` | Expected `invRow` Result |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `y3` / `Y3` / `sound` | `M_ID = 0` | None (as-built `ChainOp::Y3` specification) | **Yes** (`honestTrace(0, 1, 0)`) | None (all 182 single-cell tampers + 5 mutant attacks rejected or refine) | **`[ok]`** (exit `0`) |
| `m_no_recv_x3` | `M_ID = 1` | Drop `Y3`'s `bus.receive` on bus `o+6` (`(id, x3)` from `X3`) | **Yes** (`verifierAccepts(honestSt, 1) && invRowProperty(honestSt)`) | `attackTraceNoRecvX3(0)`: set `y0.x3 = 0 != 10`, recompute `y0.y3 = 11 != 9`, emit `(1, 0, 11)` on `o+7` | **`[violation]`** (exit `1`) |
| `m_out_wrong_x` | `M_ID = 2` | Send `(id, x1, y3)` instead of `(id, x3, y3)` on bus `o+7` | Changes honest bus `o+7` layout (`out_x = x1 = 0 != 10`) | `attackTraceOutWrongX`: sink receives `out_x = y0.x1 = 0` instead of `x0.x3 = 10` on bus `o+7` | **`[violation]`** (exit `1`) |
| `m_no_recv_y1` | `M_ID = 3` | Drop `Y3`'s `bus.receive` on bus `o+5` (`(id, y1)` from `L`) | **Yes** (`verifierAccepts(honestSt, 3) && invRowProperty(honestSt)`) | `attackTraceNoRecvY1(0)`: set `y0.y1 = 0 != 2`, recompute `y0.y3 = 11 != 9`, emit `(1, 10, 11)` on `o+7` | **`[violation]`** (exit `1`) |
| `m_sign_flip` | `M_ID = 4` | Flip sign in `Y3` polynomial to `y3 = lambda * (x3 - x1) - y1` | Changes honest polynomial (`y3 = 0 != 9` on $P_1+P_2$) | `attackTraceSignFlip`: `y0.y3 = 8*(10 - 0) - 2 = 0 != 9 (mod 13)`, off-curve $(10, 0)$ emitted on `o+7` | **`[violation]`** (exit `1`) |
| `m_valid_free` | `M_ID = 5` | Send `(id, x3, y3)` on bus `o+7` with multiplicity `1` instead of `is_valid` | **Yes** on 2-active-row trace (`verifierAccepts(honestSt, 5) && invRowProperty(honestSt)`) | `attackTraceValidFree(0)`: inactive padding row 1 (`is_valid = 0`, `y3 = 0`) sent with mult `1` on `o+7` and consumed by active sink (`s1.is_valid = 1`) | **`[violation]`** (exit `1`) |

### 9.2 Exact `quint` Verification Commands and Expected Exit Codes

Run the following commands from the repository root (`/home/hoskinson/.cache/gemini-lanes/x14-y3`):

| Step | Exact Command | Purpose | Expected Output / Exit Code |
| :--- | :--- | :--- | :--- |
| **1. Typecheck Spec** | `quint typecheck qa/gemini-quint/x14-y3/y3.qnt` | Verify types and effect annotations of `y3.qnt` | `[ok]` — **Exit `0`** |
| **2. Typecheck Tests** | `quint typecheck qa/gemini-quint/x14-y3/y3_test.qnt` | Verify types and effect annotations of `y3_test.qnt` | `[ok]` — **Exit `0`** |
| **3. Unit & Witness Tests** | `quint test qa/gemini-quint/x14-y3/y3_test.qnt --main=y3_test` | Run all 15 deterministic unit, witness, non-vacuity, and exhaustive tamper tests | `15 passing` — **Exit `0`** |
| **4. Soundness (`y3`)** | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=y3 --invariant=invRow --max-samples=500 --max-steps=10 --seed=42` | Verify `invRow` holds on sound `y3` across 500 random samples | `[ok] No violation found` — **Exit `0`** |
| **5. Non-Vacuity (`y3`)** | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=y3 --invariant=invNonVacuous --max-samples=200 --max-steps=10 --seed=42` | Confirm negated reach invariant `invNonVacuous` is violated by accepted honest trace | `[violation]` — **Exit `1`** |
| **6. Non-Vacuity (`add`)** | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=y3 --invariant=invNoAcceptedAdd --max-samples=200 --max-steps=10 --seed=42` | Confirm accepted `g1_add` trace is reached | `[violation]` — **Exit `1`** |
| **7. Non-Vacuity (`dbl`)** | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=y3 --invariant=invNoAcceptedDbl --max-samples=200 --max-steps=10 --seed=42` | Confirm accepted `g1_double` trace is reached | `[violation]` — **Exit `1`** |
| **8. Non-Vacuity (`pad`)** | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=y3 --invariant=invNoAcceptedPadding --max-samples=200 --max-steps=10 --seed=42` | Confirm accepted trace with inactive padding row (`is_valid = 0`) is reached | `[violation]` — **Exit `1`** |
| **9. Mutant 1 (`m_no_recv_x3`)** | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=m_no_recv_x3 --invariant=invRow --max-samples=200 --max-steps=10 --seed=42` | Kill `m_no_recv_x3` via `x3` forgery in `Y3` | `[violation]` — **Exit `1`** |
| **10. Mutant 2 (`m_out_wrong_x`)** | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=m_out_wrong_x --invariant=invRow --max-samples=200 --max-steps=10 --seed=42` | Kill `m_out_wrong_x` via wrong `out_x = x1` on `o+7` | `[violation]` — **Exit `1`** |
| **11. Mutant 3 (`m_no_recv_y1`)** | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=m_no_recv_y1 --invariant=invRow --max-samples=200 --max-steps=10 --seed=42` | Kill `m_no_recv_y1` via `y1` forgery in `Y3` | `[violation]` — **Exit `1`** |
| **12. Mutant 4 (`m_sign_flip`)** | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=m_sign_flip --invariant=invRow --max-samples=200 --max-steps=10 --seed=42` | Kill `m_sign_flip` via flipped $(x_3 - x_1)$ sign in `Y3` | `[violation]` — **Exit `1`** |
| **13. Mutant 5 (`m_valid_free`)** | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=m_valid_free --invariant=invRow --max-samples=200 --max-steps=10 --seed=42` | Kill `m_valid_free` via forged inactive padding row on `o+7` | `[violation]` — **Exit `1`** |

---

## 10. Detailed Walkthrough of Unit and Witness Tests (`qa/gemini-quint/x14-y3/y3_test.qnt`)

To ensure deterministic, zero-flakiness verification of every algebraic and bus property, [y3_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt) provides 15 self-contained `run ...Test` definitions:

1. **`completenessAddTest` ([y3_test.qnt:14-31](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L14-L31)):**
   - Verifies that $P_1 = (0, 2), P_2 = (2, 5), P_1 + P_2 = (10, 9)$ and $P_3 = (4, 4), P_4 = (7, 3), P_3 + P_4 = (5, 5)$ all satisfy `onCurve` on $E(\mathbb{F}_{13}) : y^2 = x^3 + 4 \pmod{13}$.
   - Verifies that `honestTrace(0, 1, 0)` is accepted by `verifierAccepts(st, 0)` and satisfies `invRowProperty(st)`.
2. **`completenessDblTest` ([y3_test.qnt:36-51](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L36-L51)):**
   - Verifies that $2 \cdot (2, 5) = (12, 9)$ ($\lambda = 9$) and $2 \cdot (4, 4) = (2, 8)$ ($\lambda = 6$) lie on $E(\mathbb{F}_{13})$.
   - Verifies that `honestTrace(1, 1, 0)` is accepted by `verifierAccepts(st, 0)` and satisfies `invRowProperty(st)`.
3. **`paddingRowInactiveCompletenessTest` ([y3_test.qnt:55-68](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L55-L68)):**
   - Verifies that both `g1_add` and `g1_double` traces with an inactive padding row (`active1 = 0`, `y1_row.is_valid = 0`) are accepted by `verifierAccepts(..., 0)` and satisfy `invRowProperty`.
4. **`honestUnderMutantsNonVacuityTest` ([y3_test.qnt:77-92](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L77-L92)):**
   - Confirms that the canonical honest 2-active-row witness `honestTrace(0, 1, 0)` is accepted and refines under `m_no_recv_x3` (`mId = 1`), `m_no_recv_y1` (`mId = 3`), and `m_valid_free` (`mId = 5`), proving that those three mutants are caught by genuine adversarial attacks rather than vacuous honest rejection.
   - Confirms that `m_out_wrong_x` (`mId = 2`) and `m_sign_flip` (`mId = 4`) modify the honest emission/polynomial layout itself.
5. **`mutantNoRecvX3CaughtTest` ([y3_test.qnt:97-108](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L97-L108)):**
   - Verifies that `attackTraceNoRecvX3(0)` (`y0.x3 = 0 != 10`, `y0.y3 = 11 != 9`) is accepted by `m_no_recv_x3`, rejected by sound `y3` (`bus6Balanced(st, 0) == false`), and violates `invRowProperty`.
6. **`mutantOutWrongXCaughtTest` ([y3_test.qnt:113-124](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L113-L124)):**
   - Verifies that `attackTraceOutWrongX` (`s0.out_x = 0 != 10`) is accepted by `m_out_wrong_x`, rejected by sound `y3` (`bus7Balanced(st, 0) == false`), and violates `invRowProperty`.
7. **`mutantNoRecvY1CaughtTest` ([y3_test.qnt:129-140](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L129-L140)):**
   - Verifies that `attackTraceNoRecvY1(0)` (`y0.y1 = 0 != 2`, `y0.y3 = 11 != 9`) is accepted by `m_no_recv_y1`, rejected by sound `y3` (`bus5Balanced(st, 0) == false`), and violates `invRowProperty`.
8. **`mutantSignFlipCaughtTest` ([y3_test.qnt:145-156](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L145-L156)):**
   - Verifies that `attackTraceSignFlip` (`y0.y3 = 0 != 9`) is accepted by `m_sign_flip`, rejected by sound `y3` (`y3RowValid(st.y0, 0) == false`), and violates `invRowProperty`.
9. **`mutantValidFreeCaughtTest` ([y3_test.qnt:162-173](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L162-L173)):**
   - Verifies that `attackTraceValidFree(0)` (inactive row 1 with `is_valid = 0, y3 = 0` consumed by active `s1.is_valid = 1`) is accepted by `m_valid_free`, rejected by sound `y3` (`bus7Balanced(st, 0) == false`), and violates `invRowProperty`.
10. **`duplicateIdSwapAttackTest` ([y3_test.qnt:179-194](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L179-L194)):**
    - Demonstrates Finding **F1** (Attack 1A): when rows 0 and 1 share `id = 1`, swapping $y_1$ on bus `o+5` yields off-curve outputs $(10, 7)$ and $(5, 7)$ that are accepted by `verifierAccepts(st, 0)`.
11. **`duplicateIdSwapX3AttackTest` ([y3_test.qnt:198-211](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L198-L211)):**
    - Demonstrates Finding **F1** (Attack 1B): when rows 0 and 1 share `id = 1`, swapping $x_3$ on bus `o+6` yields off-curve outputs $(5, 10)$ and $(10, 11)$ that are accepted by `verifierAccepts(st, 0)`.
12. **`exceptionalSamePointLambdaFreeTest` ([y3_test.qnt:216-235](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L216-L235)):**
    - Demonstrates Finding **F2**: feeding $P_1 = P_2 = (2, 5)$ into `g1_add` allows all 13 values of $\lambda \in \mathbb{F}_{13}$ to pass `verifierAccepts(..., 0)`.
13. **`exceptionalInversePointUnsatisfiableTest` ([y3_test.qnt:240-249](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L240-L249)):**
    - Demonstrates that feeding $P_1 = (2, 5)$ and $P_2 = (2, 8) = -P_1$ into an active `L_add` row has no solution $\lambda \in \mathbb{F}_{13}$.
14. **`exceptionalDoubleZeroPointLambdaFreeTest` ([y3_test.qnt:254-266](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L254-L266)):**
    - Demonstrates Finding **F3**: in `L_dbl`, $y_1 = 0$ with any $x_1 \in \{1 \dots 12\}$ has no solution $\lambda \in \mathbb{F}_{13}$, whereas off-curve $(0, 0)$ leaves $\lambda$ unconstrained across all 13 values in $\mathbb{F}_{13}$.
15. **`exhaustiveSoundSingleCellTamperTest` ([y3_test.qnt:271-273](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt#L271-L273)):**
    - Evaluates `allSingleCellTampersSound` across all $7 \times 13 \times 2 = 182$ single-cell tamper configurations on `Y3` row 0, proving that every trace accepted by sound `y3` satisfies `invRowProperty`.

---

## 11. Line-by-Line Audit of `ChainAir` and `FieldExpr` for `ChainOp::Y3`

For completeness of the audit record, below is a line-by-line verification of every Rust statement executed by `ChainOp::Y3` during trace generation and AIR constraint evaluation:

### 11.1 `g1_chain` Construction of `Y3` ([lane-src/x_exp_node.rs:428-440](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L428-L440))

1. **Line 429 (`let mut builder = ExprBuilder::new_bls12_381();`):**
   - Initializes `ExprBuilder` with the 48-limb little-endian base-256 decomposition of $p_{\text{BLS12-381}}$, `limb_bits = 8`, `num_limbs = 48`, `max_overflow_bits = 29` ([lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs#L46-L61)).
2. **Lines 430–433 (`lambda`, `x1`, `x3`, `y1` input declarations):**
   - `let lambda = builder.new_input();` $\to$ `Input(0)` (`col[2..50]`).
   - `let x1 = builder.new_input();` $\to$ `Input(1)` (`col[50..98]`).
   - `let x3 = builder.new_input();` $\to$ `Input(2)` (`col[98..146]`).
   - `let y1 = builder.new_input();` $\to$ `Input(3)` (`col[146..194]`).
   - Order of inputs matches `ChainAir::eval` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L358-L372)), which slices:
     - `inputs[0 * 48 .. 1 * 48]` as `lambda_limbs`
     - `inputs[1 * 48 .. 2 * 48]` as `x1_limbs`
     - `inputs[2 * 48 .. 3 * 48]` as `x3_limbs`
     - `inputs[3 * 48 .. 4 * 48]` as `y1_limbs`
3. **Line 434 (`let mut dx = x1 - x3;`):**
   - Calls `FieldVariable::sub` ([lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs#L140-L167)), building `SymbolicExpr::Sub(Input(1), Input(2))` and `ComputeExpr::Sub(Input(1), Input(2))`.
4. **Line 435 (`let mut l_dx = lambda * dx;`):**
   - Calls `FieldVariable::mul` ([lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs#L176-L209)), building `SymbolicExpr::Mul(Input(0), Sub(Input(1), Input(2)))` and `ComputeExpr::Mul(Input(0), Sub(Input(1), Input(2)))`.
5. **Line 436 (`let mut y3 = (l_dx - y1).save();`):**
   - Calls `FieldVariable::sub` followed by `FieldVariable::save` ([lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs#L53-L68)), allocating `Var(0)` (`col[194..242]`) and recording the constraint:
     $$\text{SymbolicExpr::Sub}\bigl(\text{Sub}(\text{Mul}(\text{Input}(0), \text{Sub}(\text{Input}(1), \text{Input}(2))), \text{Input}(3)), \text{Var}(0)\bigr) \equiv 0 \pmod p.$$
6. **Line 437 (`builder.build(vec![y3])`):**
   - Marks `Var(0)` ($y_3$) at output index `0`, so `ChainAir::eval` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L377-L379)) extracts `out_limbs = vars[0..48]` ($y_3$) and sends `(id, x3_limbs, out_limbs)` = `(id, x3, y3)` on bus `o+7` ([lines 387–391](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L387-L391)).

### 11.2 Summary Checklist of Security Properties Verified

| Security Property | Status in `Y3` | Evidence |
| :--- | :--- | :--- |
| Booleanity of `is_valid` (`col[1]`) | **Enforced** | `builder.assert_bool(is_valid.clone())` at [x_exp_node.rs:301](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs#L301) |
| Inactive row silencing (`is_valid == 0`) | **Enforced** | All bus sends/receives and `FieldExpr::eval` gated by `is_valid`; verified by `m_valid_free` and `paddingRowInactiveCompletenessTest` |
| Exact polynomial $\lambda(x_1 - x_3) - y_1 - y_3 \equiv 0 \pmod p$ | **Enforced** | `FieldExpr::eval` + `CheckCarryModToZeroSubAir`; verified by `m_sign_flip` and `exhaustiveSoundSingleCellTamperTest` |
| Output $y_3$ limb range check ($[0, 2^8)$) | **Enforced** | `FieldExpr::eval` range-checks `Var(0)` on `RANGE_BUS = 1000` at [builder.rs:275-281](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs#L275-L281) |
| Input $\lambda$ and $x_3$ limb range checks ($[0, 2^8)$) | **Enforced Upstream** | Range-checked as `Var(0)` in `L` and `X3` respectively; bound to `Y3` via buses `o+4` and `o+6` |
| Input $x_1$ and $y_1$ limb range checks ($[0, 2^8)$) | **Conditional (C3)** | Requires upstream producer on bus `o+0` to range-check $x_1, y_1$ limbs (Finding **F4**) |
| Binding of $(\lambda, x_1)$, $y_1$, and $x_3$ to same row | **Conditional (C1)** | Enforced by shared `id = local[0]` on `o+4`, `o+5`, `o+6` iff active `id`s are distinct (Finding **F1**) |
| Non-zero slope denominator in `L` | **Conditional (C2)** | Requires $x_1 \not\equiv x_2 \pmod p$ in `g1_add` and $y_1 \not\equiv 0 \pmod p$ in `g1_double` (Findings **F2**, **F3**) |
| Consistency of $x_3$ received on `o+6` and sent on `o+7` | **Enforced** | Both `o+6` receive and `o+7` send slice the identical columns `inputs[96..144]` (`col[98..146]`); verified by `m_no_recv_x3` and `m_out_wrong_x` |

---

## 12. Formal Abstraction Mapping Between Rust Implementation and Quint Specification

The table below maps every construct in [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/x_exp_node.rs), [lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/builder.rs), [lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/field_variable.rs), and [lane-src/interaction_mod.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/lane-src/interaction_mod.rs) to its formal counterpart in [qa/gemini-quint/x14-y3/y3.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3.qnt) and [qa/gemini-quint/x14-y3/y3_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-y3/qa/gemini-quint/x14-y3/y3_test.qnt):

| Rust Implementation Construct (`lane-src/`) | Quint Specification Construct (`y3.qnt`) | Soundness Justification of Abstraction |
| :--- | :--- | :--- |
| BLS12-381 base field $\mathbb{F}_p$ ($p \approx 2^{381}$) | Prime field $\mathbb{F}_{13}$ (`P = 13`, `modP`, `fAdd`, `fSub`, `fMul`, `fInv`) | $\mathbb{F}_{13}$ is a prime field with $\gcd(6, 13) = 1$, preserving all field identities, degree-2/3 curve equations, and division-by-zero edge cases. |
| Short Weierstrass curve $E(\mathbb{F}_p): y^2 = x^3 + 4$ | `onCurve(x, y): y^2 == x^3 + 4 (mod 13)` | Identical Weierstrass coefficients ($a = 0, b = 4$); contains non-trivial affine addition pairs $(0,2)+(2,5)=(10,9)$, $(4,4)+(7,3)=(5,5)$, and doubling $2\cdot(2,5)=(12,9)$. |
| `ChainAir` row prefix `id = local[0]`, `is_valid = local[1]` (`assert_bool`) | `r.id`, `r.is_valid` (`inField(r.id) and isBit(r.is_valid)`) | Preserves unconstrained field status of `id` and boolean constraint $\text{is\_valid} \in \{0, 1\}$. |
| `ChainOp::LAdd` (`(x2 - x1) * lambda - (y2 - y1) = q * p`) | `lRowValid(0, r)`: `(r.is_valid == 0) or ((x2 - x1) * lam == y2 - y1 mod 13)` | Exact polynomial division relation (not an explicit inverse), faithfully reproducing both $P_1 = -P_2$ unsatisfiability and $P_1 = P_2$ unconstrained-$\lambda$ behavior. |
| `ChainOp::LDbl` (`(2 * y1) * lambda - 3 * x1^2 = q * p`) | `lRowValid(1, r)`: `(r.is_valid == 0) or ((2 * y1) * lam == 3 * x1^2 mod 13)` | Exact polynomial division relation for tangent slope. |
| `ChainOp::X3` (`x3 = lambda^2 - (x1 + x2) mod p`) | `x3RowValid(r)`: `(r.is_valid == 0) or (r.x3 == affineX3(r.lam, r.x1, r.x2))` | Exact affine $x_3$ polynomial constraint gated by `is_valid`. |
| `ChainOp::Y3` (`y3 = lambda * (x1 - x3) - y1 mod p`) | `y3RowValid(r, mId)`: `(r.is_valid == 0) or (r.y3 == targetY3)` | Exact affine $y_3$ polynomial constraint gated by `is_valid` (and mutated to `flippedY3` under `mId == 4`). |
| `PermutationCheckBus` `o+2` (`(id, x2)` from `L` to `X3`) | `bus2Balanced(st)` (`multisetEq2` for `opMode == 0`, `x2 == x1` for `opMode == 1`) | Exact multiset equality of active tuples weighted by `is_valid`. |
| `PermutationCheckBus` `o+3` (`(id, lambda, x1)` from `L` to `X3`) | `bus3Balanced(st)` (`multisetEq3`) | Exact multiset equality of active 3-tuples weighted by `is_valid`. |
| `PermutationCheckBus` `o+4` (`(id, lambda, x1)` from `L` to `Y3`) | `bus4Balanced(st)` (`multisetEq3`) | Exact multiset equality of active 3-tuples weighted by `is_valid`. |
| `PermutationCheckBus` `o+5` (`(id, y1)` from `L` to `Y3`) | `bus5Balanced(st, mId)` (`multisetEq2`; dropped when `mId == 3`) | Exact multiset equality of active 2-tuples weighted by `is_valid`. |
| `PermutationCheckBus` `o+6` (`(id, x3)` from `X3` to `Y3`) | `bus6Balanced(st, mId)` (`multisetEq2`; dropped when `mId == 1`) | Exact multiset equality of active 2-tuples weighted by `is_valid`. |
| `PermutationCheckBus` `o+7` (`(id, x3, y3)` from `Y3` to Sink) | `bus7Balanced(st, mId)` (`multisetEq3`; mutated under `mId in {2, 5}`) | Exact multiset equality of active 3-tuples weighted by `is_valid` (`1` under `m_valid_free`, `x1` under `m_out_wrong_x`). |

---

## Lead results (2026-11-13 03:19, quint 0.32.0, run by the lead outside the lane sandbox)

The lane could not run commands. The lead ran every quint command of this report in the lane worktree `~/.cache/gemini-lanes/x14-y3`; expected exit code 0 for typecheck, test and the sound invariant, 1 for non-vacuity and mutants.

| rc | expected | result | command |
|---:|---:|---|---|
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-y3/y3.qnt` |
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-y3/y3_test.qnt` |
| 0 | 0 | MATCH | `quint test qa/gemini-quint/x14-y3/y3_test.qnt --main=y3_test` |
| 0 | 0 | MATCH | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=y3 --invariant=invRow --max-samples=500 --max-steps=10 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=y3 --invariant=invNonVacuous --max-samples=200 --max-steps=10 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=y3 --invariant=invNoAcceptedAdd --max-samples=200 --max-steps=10 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=y3 --invariant=invNoAcceptedDbl --max-samples=200 --max-steps=10 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=y3 --invariant=invNoAcceptedPadding --max-samples=200 --max-steps=10 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=m_no_recv_x3 --invariant=invRow --max-samples=200 --max-steps=10 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=m_out_wrong_x --invariant=invRow --max-samples=200 --max-steps=10 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=m_no_recv_y1 --invariant=invRow --max-samples=200 --max-steps=10 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=m_sign_flip --invariant=invRow --max-samples=200 --max-steps=10 --seed=42` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-y3/y3.qnt --main=m_valid_free --invariant=invRow --max-samples=200 --max-steps=10 --seed=42` |
