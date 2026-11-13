# Soundness & Completeness Review: X14 G1 Addition Condition Row `INV`

## 1. Verdict

**Verdict: `SOUND` and `COMPLETE` (Conditional on the Premises & Schedule Obligations Below)**

The new condition row AIR `INV` in the BLS12-381 $\mathbb{G}_1$ addition chain of experiment X14 ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/x_exp_node.rs#L660-L1073), ruling 2026-11-12) is **sound** and **complete** for affine chord addition of distinct-abscissa points ($x_1 \not\equiv x_2 \pmod p$), subject to the following explicit conditions:

1. **Premise on Stock Mod-Builder & Range Checker**: The stock OpenVM `FieldExpr` ([lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/builder.rs#L249-L450), [lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/field_variable.rs#L266-L322)) and `VariableRangeCheckerChip` ([lane-src/var_range_mod.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/var_range_mod.rs#L54-L131)) are sound for a single modular constraint when all input limbs are range-checked to 8 bits on `RANGE_BUS` (`1000`).
2. **Input Limb Range Checking by Producer**: Because `FieldExpr::eval` range-checks `vars`, `q_limbs`, and `carry_limbs` when `is_valid = 1` ([lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/builder.rs#L424-L448)) but **does not** range-check `inputs`, the point producer (`PointsIn` at [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/x_exp_node.rs#L766-L769), or the upstream accumulator in the full circuit) **must** range-check every limb of `x1, y1, x2, y2` to 8 bits on `RANGE_BUS`.
3. **Trace Height Bound**: The trace height $N = 2^h$ of every `ChainAir` satisfies $N \le p_{\text{BabyBear}} = 2^{31} - 2^{27} + 1 = 2{,}013{,}265{,}921$, so the `id` column (`id[0] = 0`, `id[r+1] = id[r] + 1`) never wraps modulo the field characteristic and assigns a strictly unique `id = r` to each row $r \in \{0, \dots, N-1\}$, and total LogUp interaction weight per bus does not overflow $p_{\text{BabyBear}}$.
4. **External MSM / Verifier Obligations on Exceptional Cases**: The chain proves pure affine chord addition ($x_1 \not\equiv x_2 \pmod p$) and unconditionally rejects $x_1 \equiv x_2 \pmod p$ on every active row (`is_valid = 1`). The MSM schedule / outer verifier must still handle point doubling ($P_1 = P_2$, routed to the `dbl` chain), inverse points ($P_1 = -P_2$), the point at infinity $\mathcal{O}$, on-curve/subgroup validation of external inputs, and canonicality ($< p$) at public/digest boundaries.

> **Sandbox Execution Note**: In this review session, shell execution (`quint`) was not available in the sandbox. Per instructions, no unexecuted command outputs are claimed; the Quint specification ([qa/gemini-quint/x14-inv/x14_inv.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/qa/gemini-quint/x14-inv/x14_inv.qnt)) and test suite ([qa/gemini-quint/x14-inv/x14_inv_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/qa/gemini-quint/x14-inv/x14_inv_test.qnt)) have been statically verified against Quint 0.32.0 syntax, mode/effect, and type rules, and the exact commands and expected exit codes are tabulated in Section 4.

---

## 2. Circuit Architecture under Review

In [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/x_exp_node.rs#L846-L1073), `g1_chain("add", ...)` decomposes BLS12-381 $\mathbb{G}_1$ affine addition over $\mathbb{F}_p$ ($p = p_{381}$, 48 limbs of 8 bits, bus base `o = 1100`) into four `fp_mul`-class `ChainAir` chips—each having **exactly one** saved `FieldExpr` variable (`assert_eq!(e.num_vars(), 1)` at line 920)—plus `PointsIn`, `VariableRangeCheckerAir`, and the `SourceAir` sink harness:

| Chip | Inputs (`i(...)`) | Saved Var (`v(0)`) & Modular Constraint ($\bmod\, p$) | Bus Receives (mult) | Bus Sends (mult) |
| :--- | :--- | :--- | :--- | :--- |
| `PointsIn` | $x_1, y_1, x_2, y_2$ (range-checked 8b) | None (`id = row_index`) | — | `o+0`: `(id, x1, y1)` (`1`)<br>`o+1`: `(id, x2, y2)` (`1`) |
| `L_add` | `i(0)=x1, i(1)=y1, i(2)=x2, i(3)=y2` | $\lambda$: $(x_2 - x_1)\lambda - (y_2 - y_1) \equiv 0$ | `o+0`: `(id, x1, y1)` (`is_valid`)<br>`o+1`: `(id, x2, y2)` (`is_valid`) | `o+2`: `(id, lam, x1)` (`is_valid`)<br>`o+3`: `(id, x2)` (`is_valid`)<br>`o+4`: `(id, lam, x1)` (`is_valid`)<br>`o+5`: `(id, y1)` (`is_valid`)<br>**`o+10`: `(id, x1, x2)` (`is_valid`)** |
| **`INV`** | `i(0)=x1, i(1)=x2` | **$w$: $(x_2 - x_1) w - 1 \equiv 0$** | **`o+10`: `(id, x1, x2)` (`is_valid`)** | — |
| `X3` | `i(0)=lam, i(1)=x1, i(2)=x2` | $x_3$: $(\lambda^2 - x_1 - x_2) - x_3 \equiv 0$ | `o+2`: `(id, lam, x1)` (`is_valid`)<br>`o+3`: `(id, x2)` (`is_valid`) | `o+6`: `(id, x3)` (`is_valid`) |
| `Y3` | `i(0)=lam, i(1)=x1, i(2)=x3, i(3)=y1` | $y_3$: $(\lambda(x_1 - x_3) - y_1) - y_3 \equiv 0$ | `o+4`: `(id, lam, x1)` (`is_valid`)<br>`o+5`: `(id, y1)` (`is_valid`)<br>`o+6`: `(id, x3)` (`is_valid`) | `o+7`: `(id, x3, y3)` (`is_valid`) |
| `sink` | $x_3, y_3$ | None | `o+7`: `(id, x3, y3)` (`1`) | — |

### Why `INV` Is Necessary and Sufficient to Close the $x_1 = x_2$ Soundness Hole
* **Without `INV`**: In `L_add`, `FieldVariable::div` ([lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/field_variable.rs#L308-L315)) compiles `(&mut y2 - &mut y1) / (&mut x2 - &mut x1)` into the polynomial constraint:
  $$(x_2 - x_1) \cdot \lambda - (y_2 - y_1) = q \cdot p$$
  When $P_1 = P_2$ ($x_1 \equiv x_2 \pmod p$ and $y_1 \equiv y_2 \pmod p$), both $(x_2 - x_1)$ and $(y_2 - y_1)$ are $0 \pmod p$, so the constraint degenerates to $0 \cdot \lambda - 0 \equiv 0 \pmod p$, which is satisfied by **every** forged $\lambda \in [0, p)$ (demonstrated by `forge_subrow` in `neg == "same_pt"` at [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/x_exp_node.rs#L986-L1014)).
* **With `INV`**: `INV` builds `new_const(1) / (x2 - x1)` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/x_exp_node.rs#L882-L890)), which generates one saved variable $w$ and one constraint:
  $$(x_2 - x_1) \cdot w - 1 = q \cdot p$$
  When `L_add` row $r$ (`id = r`) has `is_valid = 1`, it sends `(r, x1, x2)` (97 field elements $\le 128$ max LogUp message length) on bus `o+10` (`1110`). Because `INV` enforces `id = row_index` (`id[0] = 0`, `id[r+1] = id[r] + 1`) and `is_valid` $\in \{0, 1\}$, only row $r$ of `INV` has `id = r`. LogUp multiset balance on `o+10` therefore forces `INV[r].is_valid = 1`, `INV[r].x1 = L_add[r].x1`, and `INV[r].x2 = L_add[r].x2`. Since `INV[r].is_valid = 1`, `CheckCarryModToZeroSubAir` enforces $(x_2 - x_1) w \equiv 1 \pmod p$, which has a solution $w \in \mathbb{F}_p$ **if and only if** $x_1 \not\equiv x_2 \pmod p$. Once $x_2 - x_1 \in \mathbb{F}_p^\times$, $\lambda \equiv (y_2 - y_1)(x_2 - x_1)^{-1} \pmod p$ is uniquely determined in `L_add[r]`.

---

## 3. Answers to Review Questions

### Q0: Does `INV` need `is_valid = 1` on every row that `L_add` marks valid, and what does an `INV` row with `is_valid = 0` do to the bus?

1. **Does `INV` need `is_valid = 1` on every row that `L_add` marks valid?**
   * **Yes semantically, and it is enforced automatically by bus `o+10` balance + row `id` uniqueness without needing an explicit `assert_one(is_valid)` constraint on `INV`.**
   * Specifically:
     * In `ChainAir::eval` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/x_exp_node.rs#L679-L700)) and `FieldExpr::eval` ([lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/builder.rs#L366)), `INV[r].is_valid` is constrained to be boolean ($\in \{0, 1\}$), and `INV[r].id` is constrained to equal $r$.
     * When `L_add[r]` is marked valid (`L_add[r].is_valid = 1`), `L_add[r]` sends `(r, x1, x2)` on bus `o+10` with multiplicity $+1$.
     * Because `INV[r]` is the **only** row in `INV` with `id = r` (and `INV` is the only receiver on `o+10`), the net multiplicity of `(r, x1, x2)` on `o+10` is $1 - \text{INV}[r].\text{is\_valid}$ (assuming matching `(x1, x2)`). For the bus to balance ($1 - \text{INV}[r].\text{is\_valid} = 0$), the prover is **forced** to set $\text{INV}[r].\text{is\_valid} = 1$.
     * Once $\text{INV}[r].\text{is\_valid} = 1$, `FieldExpr::eval` activates `CheckCarryModToZeroSubAir` for $(x_2 - x_1) w - 1 \equiv 0 \pmod p$ and activates the `RANGE_BUS` lookups for $w$, $q$, and carries.
     * Conversely, leaving `INV.is_valid` as a boolean rather than hardcoding `is_valid = 1` allows the chain to support **padding rows** (`L_add[r].is_valid = 0` paired with `INV[r].is_valid = 0`) when connected to a variable-length producer/consumer.
     * By contrast, if `INV` received on `o+10` with constant multiplicity `1` instead of `is_valid` (mutant `m_inv_valid_free`), a dishonest prover could set `INV[r].is_valid = 0` on an active row (`L_add[r].is_valid = 1`): bus `o+10` would still balance (send $+1$, receive $-1$), while `INV[r].is_valid = 0` would turn off `CheckCarryModToZeroSubAir` in `INV[r]`, allowing $P_1 = P_2$ with forged $\lambda$!

2. **What does an `INV` row with `is_valid = 0` do to the bus (and to `INV`)?**
   * On **`PermutationCheckBus(o + 10)`**: `bus.receive(builder, [id, x1, x2], is_valid)` pushes an interaction with multiplicity $-\text{is\_valid} = 0$ ([lane-src/interaction_mod.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/interaction_mod.rs#L174)). In LogUp, a zero-multiplicity term contributes $\frac{0}{\alpha - m} = 0$ to the LogUp sum—i.e., **it is a complete no-op on bus `o+10`** (it neither sends nor receives any message).
   * On **`RANGE_BUS` (`1000`)**: all range-check lookups for `w`, `q_limbs`, and `carry_limbs` are pushed with `count = is_valid = 0` ([lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/builder.rs#L424-L448))—also a **no-op on `RANGE_BUS`**.
   * On **`INV` internal constraints**: `CheckCarryModToZeroSubAir` multiplies all limb-carry equations by `is_valid = 0`, disabling the modular inverse check on that row; only `assert_bool(is_valid)` and the row-index transition `next.id = id + 1` remain active.
   * **Bus balance consequence**: Because row $r$ of `INV` has `id = r` and contributes $0$ on `o+10` when `INV[r].is_valid = 0`, bus `o+10` balances if and only if row $r$ of `L_add` also has `L_add[r].is_valid = 0`. (In the standalone `x_exp_node.rs` benchmark harness, `PointsIn` and `SourceAir` hardcode multiplicity `1` on every row, which forces `L_add[r].is_valid = 1` and thus `INV[r].is_valid = 1` on all $2^h$ rows.)

---

### Q1: What the `id` Column Guarantees and What It Does Not

#### What `id` Guarantees
1. **Strict Within-AIR Row Uniqueness**:
   In `ChainAir::eval` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/x_exp_node.rs#L683-L684)) and `PointsIn::eval` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/x_exp_node.rs#L764-L765)):
   - `builder.when_first_row().assert_zero(id)` enforces $\text{id}[0] = 0$.
   - `builder.when_transition().assert_eq(next.id, id + 1)` enforces $\text{id}[r+1] = \text{id}[r] + 1$ for $0 \le r < N - 1$.
   By induction, $\text{id}[r] \equiv r \pmod{p_{\text{BabyBear}}}$. For any trace height $N = 2^h \le p_{\text{BabyBear}}$, all $N$ rows of an AIR have pairwise distinct `id` values $0, 1, \dots, N-1$ with zero extra LogUp interactions.
2. **1-to-1 Lockstep Row Binding Across All Chain AIRs**:
   Because `id` is prepended to every bus message on `o+0` through `o+10`, row $r$ (`id = r`) of `PointsIn`, `L_add`, `INV`, `X3`, `Y3`, and `sink` can only match row $r$ of the partner AIR on each bus. This prevents:
   - **Cross-row coordinate mixing**: e.g., combining $P_1$ from row $r$ with $P_2$ from row $r'$, or combining $\lambda$ from row $r$ with $(x_1, y_1)$ from row $r'$ in `X3`/`Y3`.
   - **Cross-row `INV` witness or `is_valid` swapping** (caught by mutant `m_no_id`): without `id` on `o+10`, the prover can permute rows of `INV` relative to `L_add` or swap `is_valid` flags between active and padding rows.
   - **Identical-point multiplicity collisions**: even if two rows $r \neq r'$ operate on the exact same input points $(P_1, P_2)$, prepending `id` makes their bus messages `(r, ...)` and `(r', ...)` distinct field tuples of multiplicity $1$.

#### What `id` Does NOT Guarantee
1. **No Input Limb Range Check**: `id` is an index tag; it does not constrain the bit-width of `x1, y1, x2, y2`. Input limb range-checking ($[0, 2^8)$) is performed solely by `PointsIn` on `RANGE_BUS`.
2. **No Canonicality Modulo $p_{381}$ ($< p_{381}$)**: 48 limbs of 8 bits span $[0, 2^{384})$, whereas $p_{381} < 2^{381} < 2^{384}$. `id` and `FieldExpr` enforce equations modulo $p_{381}$ on 384-bit limb representations; they do not forbid an input or output integer in $[p_{381}, 2^{384})$ that is congruent to a valid residue mod $p_{381}$.
3. **No Curve or Subgroup Membership**: `id` does not check that $P_1, P_2$ lie on $E(\mathbb{F}_p): y^2 = x^3 + 4$ or in the prime-order subgroup $\mathbb{G}_1$, nor can it represent the point at infinity $\mathcal{O}$.
4. **No Row Activation by Itself**: `id = r` holds on all $N$ rows whether `is_valid = 1` or `is_valid = 0`. Whether row $r$ is active is determined by the send/receive multiplicities of the producer (`PointsIn`) and consumer (`sink`).
5. **No Cross-Row Sequential Accumulation**: Having $\text{id}[r+1] = \text{id}[r] + 1$ orders the rows by index, but does **not** wire row $r$'s output $(x_3[r], y_3[r])$ on `o+7` into row $r+1$'s input $(x_1[r+1], y_1[r+1])$ on `o+0`; each row $r$ is an independent parallel G1 operation unless an external accumulator AIR links step $r$ to step $r+1$.

---

### Q2: The Exceptional Cases ($x_1 = x_2$, $y = 0$, and the Point at Infinity $\mathcal{O}$)

| Exceptional Case | What the X14 Chain Now Does | What the Verifier / MSM Schedule Must Still Add |
| :--- | :--- | :--- |
| **$x_1 \equiv x_2 \pmod p$**<br>($P_1 = P_2$ doubling, or $P_1 = -P_2$ inverse points) | **Unconditionally rejected on any active `add` row (`is_valid = 1`).**<br>• When $P_1 = P_2$ ($x_1 = x_2, y_1 = y_2$), `L_add`'s constraint $\lambda(x_2 - x_1) \equiv y_2 - y_1 \pmod p$ degenerates to $0 = 0$, but `INV` enforces $w(x_2 - x_1) \equiv 1 \pmod p$, which has no solution when $x_1 \equiv x_2 \pmod p$.<br>• When $P_1 = -P_2$ with $y_1 \not\equiv y_2 \pmod p$, both `L_add` ($0 \equiv y_2 - y_1 \neq 0$) and `INV` ($0 \equiv 1$) reject.<br>• Because `INV` checks $(x_2 - x_1)w - 1 = q \cdot p$, even non-canonical limbs with $x_2 = x_1 + p < 2^{384}$ fail `INV`. | • **Route doublings ($P_1 = P_2$) to the `dbl` chain** (`g1_chain("dbl", ...)`, buses `1110..1120`), never to `add`.<br>• **Guarantee $x_1 \not\equiv x_2 \pmod p$ in the MSM schedule** (e.g., via a deterministic offset point $G_{\text{off}}$ and signed-digit recoding so running accumulators never collide with bucket points or their negations), **or** provide a conditional bypass/mux AIR that sets `is_valid = 0` on `add` when $x_1 = x_2$. |
| **$y \equiv 0 \pmod p$**<br>(2-torsion point $2P = \mathcal{O}$, or $(0, 0)$) | • **In the `dbl` chain (`o = 1110`)**: `L_dbl` checks $\lambda(2y) \equiv 3s \pmod p$. For $(x, y) = (0, 0)$, $3s = 0$ and $2y = 0$, so `L_dbl` degenerates to $0 = 0$, **but `INV_y` receives `(id, y)` on `o+10` (`1120`) and enforces $w \cdot y \equiv 1 \pmod p$, rejecting every active `dbl` row with $y \equiv 0 \pmod p$** (verified by `neg == "y_zero"` at [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/x_exp_node.rs#L986-L1024)).<br>• **In the `add` chain (`o = 1100`)**: chord slope divides by $x_2 - x_1$, not $y$. If $y_1 = 0$ or $y_2 = 0$ with $x_1 \not\equiv x_2 \pmod p$, `add` succeeds and computes the chord point. However, if $(0,0)$ is fed as a fake encoding of $\mathcal{O}$ alongside $P_2 = (x_2, y_2)$ with $x_2 \neq 0$, `add` computes the affine chord through $(0,0)$ and $P_2$, which is **not** $\mathcal{O} + P_2 = P_2$. | • **Never encode $\mathcal{O}$ as $(0, 0)$** on active rows (`is_valid = 1`) of `add` or `dbl`.<br>• Validate that external input points lie on $E(\mathbb{F}_{p_{381}}): y^2 = x^3 + 4$ and in the prime-order subgroup $\mathbb{G}_1$ (note that $\mathbb{G}_1$ has odd prime order $r_{255}$, so no non-identity point in $\mathbb{G}_1$ has order 2 / $y = 0$). |
| **Point at Infinity $\mathcal{O}$**<br>(Identity element of $\mathbb{G}_1$) | **Not representable on active rows (`is_valid = 1`).**<br>• All `ChainAir` chips (`L_add`, `INV`, `X3`, `Y3`, `SQ`, `L_dbl`, `INV_y`) operate strictly on affine coordinates $(x, y) \in \mathbb{F}_p^2$ with no infinity flag (`is_inf`).<br>• Any operation that produces $\mathcal{O}$ from affine inputs ($P + (-P)$ in `add`, or $2P$ with $y=0$ in `dbl`) is rejected by `INV` / `INV_y`. | • **Offset-generator schedule or infinity-flag mux**: Either initialize the MSM / scalar-mul accumulator with a known offset point so no intermediate state or operand is ever $\mathcal{O}$ (subtracting the offset at the end), or add boolean infinity flags (`is_inf`) in the schedule/accumulator AIR that bypass `add`/`dbl` (`is_valid = 0`) when an operand is $\mathcal{O}$ or when $P_1 + P_2 = \mathcal{O}$.<br>• **Canonicality ($< p$) check** on the final output point at the public-output / digest boundary. |

---

### Q3: Completeness ("Every Honest Row on Valid Inputs Is Accepted")

**Completeness holds**:
1. **Existence & Uniqueness of Witness Values**: For any valid affine inputs $P_1 = (x_1, y_1)$ and $P_2 = (x_2, y_2)$ with $x_1, y_1, x_2, y_2 \in [0, p)$ and $x_1 \not\equiv x_2 \pmod p$:
   - Since $p$ is prime and $x_2 - x_1 \not\equiv 0 \pmod p$, the modular inverse $w = (x_2 - x_1)^{p-2} \bmod p \in [1, p-1]$ exists and is unique, satisfying $(x_2 - x_1) w - 1 = q_{\text{inv}} \cdot p$.
   - The chord slope $\lambda = (y_2 - y_1) \cdot w \bmod p \in [0, p-1]$, $x_3 = (\lambda^2 - x_1 - x_2) \bmod p \in [0, p-1]$, and $y_3 = (\lambda(x_1 - x_3) - y_1) \bmod p \in [0, p-1]$ exist and are unique.
2. **Range-Check Bounds in `INV`**:
   - In `INV`, $x_1, x_2, w \in [0, p)$ have 48 limbs in $[0, 2^8)$. In `FieldExpr::generate_subrow` ([lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/lane-src/builder.rs#L520-L555)), the quotient $q_{\text{inv}} = ((x_2 - x_1)w - 1) / p$ satisfies $|q_{\text{inv}}| < p$, so every limb of $q_{\text{inv}}$ lies in $[-2^8, 2^8)$ and `(q_limb + 256)` fits in `limb_bits + 1 = 9` bits ($\le 17$). The carries of $(x_2 - x_1)w - 1 - q_{\text{inv}} p$ have `carry_bits <= 16 <= range_max_bits (17)`, so `VariableRangeCheckerChip::add_count` never panics.
3. **Bus Balance**:
   - On every active row $r$ (`is_valid = 1`) with `id = r`, `L_add` sends `(r, x1, x2)` with multiplicity $+1$ on `o+10` and `INV` receives `(r, x1, x2)` with multiplicity $+1$ (`delta = -1`) on `o+10`; all buses `o+0..o+7` and `o+10` sum to $0$.
   - On every padding row $r$ (`is_valid = 0` across the chain), all `FieldExpr` constraints and bus interactions are multiplied by `0` and trivially pass.
   - Tested in [qa/gemini-quint/x14-inv/x14_inv_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/qa/gemini-quint/x14-inv/x14_inv_test.qnt#L17-L33) (`completenessHonestTwoActiveTest` and `completenessHonestPaddingRowTest`) and witnessed by the non-vacuity invariant `nonVacuous` in `x14_inv`.

---

## 4. Quint Model, Non-Vacuity & Mutant Table

### Model Files Written
* [qa/gemini-quint/x14-inv/x14_inv.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/qa/gemini-quint/x14-inv/x14_inv.qnt):
  - `x14_inv`: Sound module with field arithmetic over $p = 13$, row records for `PointsIn`, `L_add`, `X3`, `Y3`, `INV`, and `sink`, multiset bus balance (`netCount`, `busBalanced` using `foldl` on `List[BusEntry]`), nondeterministic prover actions (`proverHonest`, `proverForgeSamePoint`, `proverSwapInvRows`, `proverMutateOneCell`, `proverDropOrDupMsg`), refinement invariant `invRow`, and negated reach invariant `nonVacuous`.
  - Mutant modules: `m_no_inv_row`, `m_no_recv`, `m_wrong_den`, `m_no_id`, `m_inv_valid_free`.
* [qa/gemini-quint/x14-inv/x14_inv_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-inv/qa/gemini-quint/x14-inv/x14_inv_test.qnt):
  - `x14_inv_test`: Deterministic `run` tests covering completeness (active and padding rows), soundness ($P_1 = P_2$, $P_1 = -P_2$, dropped/duplicated bus messages), and every mutant counterexample.

### Commands and Expected Exit Codes

*(Note: As disclosed in Section 1, shell execution was refused by the sandbox in this session; the table below gives the exact `quint` 0.32.0 commands and their expected exit codes and behaviors.)*

| Target / Check | Module | Exact Command | Expected Exit Code | Expected Outcome / Counterexample Witness |
| :--- | :--- | :--- | :---: | :--- |
| **Typecheck model** | all in `x14_inv.qnt` | `quint typecheck qa/gemini-quint/x14-inv/x14_inv.qnt` | `0` | Typecheck succeeds with no errors. |
| **Typecheck tests** | `x14_inv_test` | `quint typecheck qa/gemini-quint/x14-inv/x14_inv_test.qnt` | `0` | Typecheck succeeds with no errors. |
| **Unit test suite** | `x14_inv_test` | `quint test --main=x14_inv_test qa/gemini-quint/x14-inv/x14_inv_test.qnt` | `0` | All tests pass (`testHonestDistinctAccepts`, `testHonestPaddedAccepts`, `testSamePointRejectedBySound`, `testOppositePointRejectedBySound`, `testDropOrDupMessageRejectedBySound`, and all 5 `testMutant*`). |
| **Soundness (`invRow`)** | `x14_inv` | `quint run --main=x14_inv --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` | `0` | `[ok] No violation found`: every accepted chain satisfies `x1 != x2 (mod p)`, `lam = (y2 - y1)/(x2 - x1)`, affine sum output, and per-row `INV` witness binding. |
| **Non-vacuity (`nonVacuous`)** | `x14_inv` | `quint run --main=x14_inv --invariant=nonVacuous --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` | `1` | `[violation] Found an issue`: `nonVacuous` is violated at step 0 (`HONEST_DEFAULT_STATE`) and on `proverHonest` steps, proving the sound model accepts honest chains non-vacuously. |
| **Mutant 1: `no_inv_row`** | `m_no_inv_row` | `quint run --main=m_no_inv_row --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` | `1` | **CAUGHT (`invRow` violated)**: `L_add` does not send on `o+10` (`INV` absent). Prover submits $P_1 = P_2 = (x_1, y_1)$ with forged $\lambda$; `L_add` degenerates to $0 = 0$ and accepts. |
| **Mutant 2: `no_recv`** | `m_no_recv` | `quint run --main=m_no_recv --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` | `1` | **CAUGHT (`invRow` violated)**: `INV` does not receive on `o+10` ($w$ free of the chain). Prover sets `INV[0]` to `(inv_x1=1, inv_x2=2, w=1)` satisfying $1 \cdot (2 - 1) = 1$, while submitting $P_1 = P_2$ with forged $\lambda$ on `L_add[0]`. *(Note: In OpenVM LogUp, if `L_add` still sends on `o+10` while `INV` has no receive interaction, LogUp also fails completeness on every valid row.)* |
| **Mutant 3: `wrong_den`** | `m_wrong_den` | `quint run --main=m_wrong_den --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` | `1` | **CAUGHT (`invRow` violated)**: `INV` checks $w \cdot (x_2 + x_1) \equiv 1 \pmod p$. Prover submits $P_1 = P_2 = (x_1, y_1)$ with $x_1 \neq 0$, forged $\lambda$, and $w = (2 x_1)^{-1} \bmod p$; both `INV` and `o+10` accept $x_1 = x_2$. *(Also breaks completeness when $x_2 \equiv -x_1 \pmod p$.)* |
| **Mutant 4: `no_id`** | `m_no_id` | `quint run --main=m_no_id --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` | `1` | **CAUGHT (`invRow` violated)**: Dropping `id` from `o+10` allows the prover to permute/swap `INV` rows `0` and `1` (or swap active/padding flags across rows) while still balancing the multiset of `(x1, x2)` on `o+10`, breaking per-row `INV[r]` witness correspondence. |
| **Mutant 5: `inv_valid_free`** | `m_inv_valid_free` | `quint run --main=m_inv_valid_free --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` | `1` | **CAUGHT (`invRow` violated)**: `INV` receives on `o+10` with constant multiplicity `1` instead of `is_valid`, while `FieldExpr` gates $w(x_2 - x_1) = 1$ by `INV.is_valid`. Prover sets `INV[0].is_valid = 0` (padding row) on $P_1 = P_2$ with forged $\lambda$: `o+10` balances (`+1` from `L_add[0]`, `-1` from `INV[0]`), and `INV[0]`'s modular constraint is disabled! *(Also breaks completeness on legitimate padding rows where `L_add[r].is_valid = 0`.)* |

## Lead quint results (2026-11-13, quint 0.32.0, run by the lead outside the lane sandbox)

After one lane fix turn (m_wrong_den was not caught on the first pass; the fix turn made the attack state reachable). Every command matches. inv.qnt and inv_test.qnt are earlier drafts of the lane, not used by any command.

| rc | expected | match | command |
|---|---|---|---|
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-inv/x14_inv.qnt` |
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-inv/x14_inv_test.qnt` |
| 0 | 0 | MATCH | `quint test --main=x14_inv_test qa/gemini-quint/x14-inv/x14_inv_test.qnt` |
| 0 | 0 | MATCH | `quint run --main=x14_inv --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` |
| 1 | 1 | MATCH | `quint run --main=x14_inv --invariant=nonVacuous --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_no_inv_row --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_no_recv --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_wrong_den --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_no_id --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_inv_valid_free --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv/x14_inv.qnt` |
