# Independent Soundness Review: `INV_y` (X14 BLS12-381 G1 Doubling Chain)

## Verdict

**VERDICT: SOUND/COMPLETE — SOUND and COMPLETE (with conditions stated below)**

Under the premise that the stock OpenVM `FieldExpr` (`CheckCarryModToZeroSubAir` + `VariableRangeCheckerChip` on `RANGE_BUS = 1000`) is sound for a single modular constraint when its inputs and saved variables are range-checked, and that the LogUp `PermutationCheckBus` multiset equality check is sound, the `INV_y` condition row (`o = 1110`, bus `o + 10 = 1120`) together with `ChainAir`'s `id = row_index` constraint makes `y = 0 (mod p)` unprovable in `L_dbl` and uniquely locks the doubling slope $\lambda \equiv 3x^2 / (2y) \pmod p$ and affine doubling output $(x_3, y_3)$ for every valid row of `L_dbl` that is balanced on the bus.

### Conditions Required for Soundness and Completeness

1. **Input Limb Range Checking (`PointsIn` / Producer):** `FieldExpr::eval` ([lane-src/builder.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/builder.rs#L437-L448)) range-checks `vars`, `q_limbs`, and `carry_limbs` over `RANGE_BUS`, but does **not** range-check `inputs`. In `g1_chain("dbl", ...)`, `PointsIn` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/x_exp_node.rs#L766-L769)) range-checks every limb of $(x, y)$ to 8 bits, which propagates through bus `o+8` (`SQ`), `o+0` (`L_dbl`), and `o+10` (`INV_y`). Any real MSM producer replacing `PointsIn` must likewise range-check $(x, y)$ limbs to 8 bits.
2. **Active Consumer / Producer Binding (`is_valid = 1`):** Each `ChainAir` sends and receives with multiplicity `cols.is_valid` (boolean). In `g1_chain`, `PointsIn` sends with multiplicity `1` and `SourceAir` receives with multiplicity `1`, forcing `is_valid = 1` on every row $r \in \{0, \dots, 2^h - 1\}$. If an MSM schedule allows padding rows (`is_valid = 0`), the consumer must only consume rows with `is_valid = 1`.
3. **Trace Height Bound ($N < p_{\text{BabyBear}}$):** The `id = row_index` constraint (`id[0] = 0`, `id[r+1] = id[r] + 1`) guarantees distinct row IDs across a trace of height $N = 2^h$ provided $N < p_{\text{BabyBear}} = 2^{31} - 2^{27} + 1$ (satisfied for all STARK heights $h \le 21$) and that the LogUp total interaction weight does not overflow $p_{\text{BabyBear}}$.
4. **Bus Offset Separation When Instantiating `add` and `dbl` Together (Finding F-1):** In [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/x_exp_node.rs#L924), `let o = if add { 1100u16 } else { 1110 };` assigns bus `o + 10 = 1110` to `add`'s denominator bus (`L_add -> INV`) and bus `o + 0 = 1110` to `dbl`'s $y$-input bus (`PointsIn -> L_dbl`). Currently `g1_chain` instantiates `"add"` or `"dbl"` in separate circuits, and their message lengths on bus `1110` differ (`1 + 96 = 97` fields in `add` vs `1 + 48 = 49` fields in `dbl`). When both chains are combined into a single STARK circuit, `dbl`'s base bus offset `o` should be shifted to at least `1120` so `add` (`1100..=1110`) and `dbl` (`1120..=1130`) use disjoint bus indices.

### Sandbox Execution Note

In this review lane's sandbox, `read_file` on `.lane-skills/quint-modeling/SKILL.md` and `.lane-skills/quint-lang/SKILL.md` was refused because `.lane-skills/` symlinks outside the workspace checkout, and shell execution (`run_command`) for `quint typecheck`, `quint test`, and `quint run` was refused by the sandbox permission policy. Per instructions, no unexecuted command outputs are claimed below; the Quint 0.32.0 model ([inv_y.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/qa/gemini-quint/x14-inv-y/inv_y.qnt)) and test suite ([inv_y_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/qa/gemini-quint/x14-inv-y/inv_y_test.qnt)) were statically audited against Quint 0.32.0 grammar and type rules, and the exact commands and expected exit codes are listed in the Mutant Table.

---

## Architecture of the AIR Under Review (`INV_y` in `g1_chain("dbl")`)

In [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/x_exp_node.rs#L846-L1073), each BLS12-381 $G_1$ affine doubling operation over $\mathbb{F}_p$ (48 limbs of 8 bits, bus base `o = 1110`) is decomposed into five `ChainAir` chips (`SQ`, `L_dbl`, `INV_y`, `X3`, `Y3`), each having `num_vars() == 1` (one `FieldExpr` modular constraint per row, `fp_mul` class) plus one `id` column (`width = ew + 1`):

| Chip | Inputs | Saved Variable & `FieldExpr` Constraint ($\bmod p$) | `id` Constraint | Bus Sends (mult = `is_valid`) | Bus Receives (mult = `is_valid`) |
|---|---|---|---|---|---|
| `PointsIn` | — (cols: `id, x, y`) | Range-checks all $x, y$ limbs to 8 bits on `RANGE_BUS` (`1000`) | `id[0]=0`, `id'=id+1` | `1118`: `(id, x)` (mult `1`)<br>`1110`: `(id, y)` (mult `1`) | — |
| `SQ` | `i(0) = x` | `v(0) = s`: $x^2 - s \equiv 0$ | `id[0]=0`, `id'=id+1` | `1119`: `(id, s, x)` | `1118`: `(id, x)` |
| `L_dbl` | `i(0) = s`<br>`i(1) = x`<br>`i(2) = y` | `v(0) = lam`: $(2y)\cdot \text{lam} - 3s \equiv 0$ | `id[0]=0`, `id'=id+1` | `1112`: `(id, lam, x)`<br>`1113`: `(id, x)`<br>`1114`: `(id, lam, x)`<br>`1115`: `(id, y)`<br>`1120`: `(id, y)` | `1119`: `(id, s, x)`<br>`1110`: `(id, y)` |
| **`INV_y`** | `i(0) = y` | `v(0) = w`: $y \cdot w - 1 \equiv 0$ (`c1 / y`) | `id[0]=0`, `id'=id+1` | — | `1120`: `(id, y)` |
| `X3` | `i(0) = lam`<br>`i(1) = x1`<br>`i(2) = x2` | `v(0) = x3`: $\text{lam}^2 - x_1 - x_2 - x_3 \equiv 0$ | `id[0]=0`, `id'=id+1` | `1116`: `(id, x3)` | `1112`: `(id, lam, x1)`<br>`1113`: `(id, x2)` |
| `Y3` | `i(0) = lam`<br>`i(1) = x1`<br>`i(2) = x3`<br>`i(3) = y1` | `v(0) = y3`: $\text{lam}(x_1 - x_3) - y_1 - y_3 \equiv 0$ | `id[0]=0`, `id'=id+1` | `1117`: `(id, x3, y3)` | `1114`: `(id, lam, x1)`<br>`1115`: `(id, y1)`<br>`1116`: `(id, x3)` |
| `SourceAir` | — (cols: `id, x3, y3`) | Test harness sink | — | — | `1117`: `(id, x3, y3)` (mult `1`) |

### Why `L_dbl` Degenerates Without `INV_y`

In [lane-src/field_variable.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/field_variable.rs#L265-L322), `FieldVariable::div(&mut self, other: &mut FieldVariable)` encodes division `self / other` by introducing `new_var` and imposing the polynomial constraint:
$$\text{other} \cdot \text{new\_var} - \text{self} \equiv 0 \pmod p.$$
For `L_dbl` (`s.int_mul(3) / y.int_mul(2)`), this constraint is:
$$(2y) \cdot \lambda - 3s \equiv 0 \pmod p.$$
- If $y \equiv 0 \pmod p$ and $x \not\equiv 0 \pmod p$, `SQ` forces $s \equiv x^2 \not\equiv 0 \pmod p$, so $(2 \cdot 0)\lambda - 3s \equiv -3x^2 \not\equiv 0 \pmod p$ (since $\gcd(3, p) = 1$), which already has no solution in `L_dbl`.
- However, if $(x, y) \equiv (0, 0) \pmod p$, `SQ` produces $s = 0$, and `L_dbl`'s constraint becomes:
  $$0 \cdot \lambda - 0 \equiv 0 \pmod p,$$
  which is satisfied by **every** $\lambda \in \mathbb{F}_p$. A dishonest prover can therefore choose any forged $\lambda \in \mathbb{F}_p$ and output $(x_3, y_3) = (\lambda^2 \bmod p,\; -\lambda^3 \bmod p)$ as the purported doubling of $(0, 0)$ (precisely the `"dbl:y_zero"` negative test in [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/x_exp_node.rs#L991-L1020)).

### How `INV_y` Eliminates the Degeneracy

1. Every active row $r$ of `L_dbl` (`is_valid == 1`) sends `(id = r, y)` on `PermutationCheckBus(1120)` (`o + 10`).
2. `INV_y` receives `(id, y)` on bus `1120` with multiplicity `INV_y.is_valid`.
3. Because `ChainAir` constrains `id = row_index` (`id[0] = 0`, `id[r+1] = id[r] + 1`) and `is_valid` is boolean $\{0, 1\}$, row $r$ of `INV_y` is the unique row in `INV_y` with `id == r`.
4. To balance `(r, y)` on bus `1120`, row $r$ of `INV_y` must have `INV_y[r].is_valid == 1` and `INV_y[r].y == L_dbl[r].y`.
5. Since `INV_y[r].is_valid == 1`, `INV_y`'s `FieldExpr` constraint (`c1 / y`) enforces:
   $$y \cdot w - 1 \equiv 0 \pmod p,$$
   which has a solution $w \in \mathbb{F}_p$ if and only if $y \not\equiv 0 \pmod p$.

---

## Mutant Table & Exact `quint` Verification Commands

The Quint model in [inv_y.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/qa/gemini-quint/x14-inv-y/inv_y.qnt) models the 2-row `g1_chain("dbl")` over $\mathbb{F}_{13}$ ($y^2 = x^3 + 4 \pmod{13}$), with honest points $P_0 = (2, 5)$ and $P_1 = (4, 4)$ and a nondeterministic prover that can submit $(x, y) = (0, 0)$ with any forged $\lambda$ and $w$, mutate any single cell (`input`, `output`, `id`, `is_valid`) of any row across all 7 AIRs, swap row witnesses in `INV_y`, or drop/duplicate any single bus message.

| Target / Module | Description | Exact Command | Expected Exit Code | Expected Outcome |
|---|---|---|---|---|
| Typecheck model | Typecheck `inv_y.qnt` | `quint typecheck qa/gemini-quint/x14-inv-y/inv_y.qnt` | `0` | Typecheck succeeds |
| Typecheck tests | Typecheck `inv_y_test.qnt` | `quint typecheck qa/gemini-quint/x14-inv-y/inv_y_test.qnt` | `0` | Typecheck succeeds |
| Unit test suite | Run all 10 unit/property tests | `quint test --main=inv_y_test qa/gemini-quint/x14-inv-y/inv_y_test.qnt` | `0` | All 10 `*Test` runs pass |
| `inv_y` (soundness) | Sound module holds `invRow` | `quint run --main=inv_y --invariant=invRow --max-samples=500 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` | `0` | `[ok]` No violation found |
| `inv_y` (non-vacuity) | Negated reach invariant violated on accepted honest chain | `quint run --main=inv_y --invariant=invNonVacuous --max-samples=500 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` | `1` | `[violation]` Witness: accepted honest chain |
| `m_no_inv_row` | `L_dbl` does not send on `o+10` (`INV_y` absent) | `quint run --main=m_no_inv_row --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` | `1` | `[violation]` Accepts $(0,0)$ with forged $\lambda$ |
| `m_no_recv` | `INV_y` does not receive on `o+10` | `quint run --main=m_no_recv --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` | `1` | `[violation]` Accepts $(0,0)$ with decoupled `INV_y` ($y=1,w=1$) since `INV_y` does not receive on `o+10` |
| `m_wrong_den` | `INV_y` constraint $w \cdot (y + 1) = 1$ | `quint run --main=m_wrong_den --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` | `1` | `[violation]` Accepts $(0,0)$ with $w = 1$ since $1 \cdot (0+1) = 1$ |
| `m_no_id` | Drop `id` from `o+10` message | `quint run --main=m_no_id --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` | `1` | `[violation]` Accepts swapped `INV_y` rows (`invy0_y = 4 != ldbl0_y = 5`) |
| `m_inv_valid_free` | `INV_y` receives on `o+10` with mult `1` instead of `is_valid` | `quint run --main=m_inv_valid_free --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` | `1` | `[violation]` Accepts $(0,0)$ with `inv_valid = 0` |

---

## Answers to Review Questions

### 0. Does $(0, 0)$ Reach the Chain When `PointsIn` Sends Unchecked Curve Points, and Does It Matter?

- **Does $(0, 0)$ lie on the curve $E: y^2 = x^3 + 4$?**
  No. Substituting $(x, y) = (0, 0)$ gives $0^2 = 0 \not\equiv 0^3 + 4 = 4 \pmod p$ (both for $p = 13$ and for the BLS12-381 base field prime $p$). In fact, on BLS12-381 $E(\mathbb{F}_p)$, the curve order $\#E(\mathbb{F}_p) = h_1 \cdot r$ is **odd** (both the cofactor $h_1 = \text{0x396c8c005555e1568c00aaab0000aaab}$ and the scalar field prime $r = \text{0x73eda753...00000001}$ are odd, and $-4$ is not a cube in $\mathbb{F}_p$), so $E(\mathbb{F}_p)$ has **no** $\mathbb{F}_p$-rational 2-torsion points ($y = 0$) at all. Moreover, for any off-curve point $(x, 0)$ with $x \not\equiv 0 \pmod p$, `L_dbl` alone rejects it because $2(0)\lambda = 3x^2 \not\equiv 0 \pmod p$ has no solution.
- **Does $(0, 0)$ reach the chain from `PointsIn`?**
  **Yes.** `PointsIn::eval` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/x_exp_node.rs#L758-L780)) only checks `id = row_index` and range-checks each limb of $x$ and $y$ to $[0, 2^8)$ on `RANGE_BUS`. It does not check the curve equation $y^2 \equiv x^3 + 4 \pmod p$. Since $(0, 0)$ has all-zero limbs ($0 \in [0, 2^8)$), `PointsIn` accepts $(0, 0)$ without complaint and sends `(id, 0)` on `o+8` (`SQ`) and `o+0` (`L_dbl`).
- **Does it matter?**
  **Yes, for two reasons:**
  1. In the standalone X14 chain with `PointsIn` (and in any architecture where intermediate or base point tables contain zeroed slots or unvalidated witnesses), $(0, 0)$ is the **only** input in $\mathbb{F}_p^2$ where `SQ` ($s = 0$) and `L_dbl` ($2y\lambda = 3s$) simultaneously degenerate to $0 = 0$. Without `INV_y`, a dishonest prover can feed $(0, 0)$ and fabricate any output point $(\lambda^2, -\lambda^3) \bmod p$ via a chosen $\lambda$.
  2. With `INV_y`, the G1 doubling chain is **algebraically self-contained**: its soundness as a relation $R_{\text{dbl}}((x, y), (x_3, y_3)) \iff y \not\equiv 0 \pmod p \land (x_3, y_3) = \text{ affine\_double}(x, y)$ holds unconditionally over $\mathbb{F}_p^2$ without relying on external curve-membership invariants or the odd-order property of $b = 4$.

### 1. What the `id` Column Guarantees and What It Does Not

- **What `id` guarantees:**
  1. **Row-index uniqueness (`id = r`):** In `ChainAir` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/x_exp_node.rs#L681-L684)) and `PointsIn` ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/x_exp_node.rs#L764-L765)), `when_first_row().assert_zero(id)` and `when_transition().assert_eq(next.id, id + 1)` force row $r$ to have $id \equiv r \pmod{p_{\text{BB}}}$. Because $N = 2^h < p_{\text{BB}}$, every row in a trace has a distinct `id`. This blocks the `"dup_id"` attack ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/x_exp_node.rs#L999)), where a prover sets `id = 0` in multiple rows to alias messages across rows.
  2. **Cross-AIR row synchronization across split messages:** Because the LogUp message length bound is $2^7 = 128$ BabyBear fields and each $\mathbb{F}_p$ element occupies 48 limbs, each AIR's inputs and outputs are split across multiple buses (`o+0, o+2..o+10`). Prefixing every message with `id` and restricting multiplicities to `is_valid` $\in \{0, 1\}$ establishes a strict per-row bijection across `PointsIn`, `SQ`, `L_dbl`, `INV_y`, `X3`, `Y3`, and `Sink`, preventing cross-row coordinate splicing (e.g., combining $s_0, x_0$ from row 0 with $y_1$ from row 1 in `L_dbl`, or $\lambda_0$ from row 0 with $x_{3,1}$ from row 1 in `Y3`).
  3. **Row-level witness binding on `o+10`:** Including `id` in `(id, y)` on bus `o+10` guarantees that row $r$ of `INV_y` (`id = r`) specifically witnesses the inverse $w_r = y_r^{-1} \bmod p$ of row $r$ of `L_dbl`.
  4. **Nuance on `o+10` (multiset vs. per-row binding):** Because `INV_y` takes only one field element $y$ and $0$ has no multiplicative inverse in $\mathbb{F}_p$ in *any* valid row of `INV_y`, even if `id` is omitted from `o+10` (`m_no_id`), no valid row of `INV_y` can ever receive `[0]` on `o+10`; thus multiset balance on `o+10` still prevents $y = 0$ in `L_dbl`. However, omitting `id` on `o+10` allows `INV_y` to permute its rows relative to `L_dbl` (caught by `m_no_id` on `swappedInvIdTrace`, where `INV_y[0]` inverts $y_1$ and `INV_y[1]` inverts $y_0$).

- **What `id` does NOT guarantee:**
  1. **Sequential chaining across rows ($r \to r+1$):** `id = r` only links the *same* row index $r$ across the parallel sub-row AIRs (`PointsIn[r] <-> SQ[r] <-> L_dbl[r] <-> INV_y[r] <-> X3[r] <-> Y3[r] <-> Sink[r]`). It does not feed the output `(x3, y3)` of row $r$ as the input `(x, y)` of row $r + 1$; an MSM accumulator AIR must enforce cross-step transitions.
  2. **Curve or subgroup membership:** `id` does not check that input points satisfy $y^2 \equiv x^3 + 4 \pmod p$ or lie in $G_1[r]$.
  3. **Cross-chain bus isolation:** `id` does not disambiguate messages if two different chains accidentally share a `PermutationCheckBus` index (see Finding F-1 on bus `1110`).

### 2. Exceptional Cases ($x_1 = x_2$, $y = 0$, Point at Infinity $\mathcal{O}$)

| Exceptional Case | What the X14 Chain Now Does | What the Verifier or MSM Schedule Must Still Add |
|---|---|---|
| **$x_1 = x_2$ in `g1_add`** ($P_1 = \pm P_2$, vertical or tangent chord) | `L_add` sends `(id, x1, x2)` on `o+10` (`1110`), and `INV` enforces $w \cdot (x_2 - x_1) \equiv 1 \pmod p$ ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/x_exp_node.rs#L882-L890)). Any valid `L_add` row with $x_1 \equiv x_2 \pmod p$ is **unsatisfiable / rejected** (verified by `"add:same_pt"`). | The MSM schedule must ensure `g1_add` is never invoked on $P_1 = P_2$ (use `g1_double` or a signed-digit/offset-point schedule) or $P_1 = -P_2$ (avoid producing $\mathcal{O}$ mid-window, or route through an explicit identity selector). |
| **$y = 0$ in `g1_double`** (2-torsion tangent or forged $(0, 0)$) | `L_dbl` sends `(id, y)` on `o+10` (`1120`), and `INV_y` enforces $w \cdot y \equiv 1 \pmod p$ ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/x_exp_node.rs#L891-L897)). Any valid `L_dbl` row with $y \equiv 0 \pmod p$ is **unsatisfiable / rejected** (verified by `"dbl:y_zero"`). | On BLS12-381 $E(\mathbb{F}_p)$, no valid curve point has $y = 0$ because $\#E(\mathbb{F}_p)$ is odd. For padding/unused rows in a fixed-height table, the MSM schedule must either set `is_valid = 0` on all chain AIRs for unused rows or populate unused rows with a dummy valid point (e.g. generator $G_1$) rather than $(0, 0)$. |
| **Point at infinity $\mathcal{O}$** | Affine coordinates $(x, y) \in \mathbb{F}_p^2$ cannot represent $\mathcal{O}$; the common convention of encoding $\mathcal{O}$ as $(0, 0)$ is **rejected** by `INV_y` in `g1_double` (and would compute bogus affine slopes in `g1_add`). | The MSM schedule / verifier must either: (a) use an offset-point accumulator initialized to an independent random/fixed generator $R$ so intermediate accumulator states never hit $\mathcal{O}$ or $x_1 = x_2$, subtracting the known offset at the end, or (b) maintain an `is_infinity` boolean flag in the accumulator chip that gates (`is_valid = 0` or multiplexes) the result when an operand is $\mathcal{O}$. |

### 3. Completeness: Every Honest Row on Valid Inputs Is Accepted

For every input $(x, y) \in [0, p)^2$ with $y \not\equiv 0 \pmod p$ (which includes **every** affine curve point $(x, y) \in E(\mathbb{F}_p)$ since $y^2 = x^3 + 4$ has no solution with $y \equiv 0 \pmod p$ over $\mathbb{F}_{13}$ or BLS12-381 $\mathbb{F}_p$):
1. Since $p > 3$ is prime and $y \not\equiv 0 \pmod p$, both $y$ and $2y$ are invertible modulo $p$.
2. The honest trace generator ([lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-inv-y/lane-src/x_exp_node.rs#L996-L1046)) computes canonical witnesses in $[0, p)$:
   - `SQ`: $s = x^2 \bmod p$
   - `L_dbl`: $\lambda = 3s \cdot (2y)^{-1} \bmod p$
   - `INV_y`: $w = y^{-1} \bmod p$
   - `X3`: $x_3 = (\lambda^2 - 2x) \bmod p$
   - `Y3`: $y_3 = (\lambda(x - x_3) - y) \bmod p$
3. Each witness decomposes uniquely into 48 limbs in $[0, 2^8)$, satisfying all `FieldExpr` quotient and carry range bounds on `RANGE_BUS` (`rb = 17` bits), with `is_valid = 1` and `id = r` on row $r$, and all 10 permutation buses (`1110, 1112..1120`) balance with zero residual multiplicity.
4. In our Quint model, `honestCompletenessTest` verifies completeness on $P_0 = (2, 5)$ and $P_1 = (4, 4)$ (and `invNonVacuous` confirms reachability of the accepted honest chain).

## Lead quint results (2026-11-13, quint 0.32.0, run by the lead outside the lane sandbox)

After one lane fix turn and one lead edit: in inv_y.qnt, stepZeroYWrongDen now also sets the honest second row to invy1_w = 8 = 1/(4+1) mod 13, because under the wrong_den mutant an honest prover computes w = 1/(y+1) on every row (before the edit, the honest second row failed the mutated constraint and hid the attack). The lane's original file is kept outside the repository at `~/.cache/x-exp/inv_y.qnt.lane-orig`. Every command matches.

| rc | expected | match | command |
|---|---|---|---|
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-inv-y/inv_y.qnt` |
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-inv-y/inv_y_test.qnt` |
| 0 | 0 | MATCH | `quint test --main=inv_y_test qa/gemini-quint/x14-inv-y/inv_y_test.qnt` |
| 0 | 0 | MATCH | `quint run --main=inv_y --invariant=invRow --max-samples=500 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` |
| 1 | 1 | MATCH | `quint run --main=inv_y --invariant=invNonVacuous --max-samples=500 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_no_inv_row --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_no_recv --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_wrong_den --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_no_id --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` |
| 1 | 1 | MATCH | `quint run --main=m_inv_valid_free --invariant=invRow --max-samples=200 --seed=42 qa/gemini-quint/x14-inv-y/inv_y.qnt` |
