# X14 Soundness & Completeness Review: `PointsIn` Identity-Point Flag Path

## 1. Executive Summary & Verdict

- **Soundness Verdict:** **SOUND (with producer & consumer boundary conditions)**
- **Completeness Verdict:** **COMPLETE (with distinct-$x$ condition on active `add` chain rows and dummy-witness trace generation on bypassed/padding chain rows)**
- **Sandbox Execution Disclosure:** In this review environment, shell command execution (`run_command`, including `quint typecheck`, `quint test`, and `quint run`) was refused by the host sandbox policy. Per review instructions, no unobserved CLI outputs are claimed; all Quint 0.32.0 specifications (`x14_identity.qnt`) and tests (`x14_identity_test.qnt`) were statically audited for Quint 0.32.0 type/syntax rules and algebraically verified against the Rust AIR implementation in [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-identity/lane-src/x_exp_node.rs).

---

## 2. Architecture of the Feature Under Review

In [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-identity/lane-src/x_exp_node.rs#L743-L833), `PointsIn` produces input points for the BLS12-381 $\mathbb{G}_1$ addition (`add`, `npts = 2`, `nv = 4`) and doubling (`dbl`, `npts = 1`, `nv = 2`) chains over 48 limbs of 8 bits:

1. **Row Indexing & Prefix Validity:**
   - `id = local[0]` is constrained to equal the row index $r \in \{0, \dots, 2^h - 1\}$ via `when_first_row().assert_zero(id)` and `when_transition().assert_eq(next.id, id + 1)`.
   - `v = local[1]` (`is_valid`) and each input identity flag $f_i = \text{local}[2 + i]$ ($i \in \{0, \dots, \text{npts}-1\}$) are constrained boolean (`assert_bool`).
   - Valid rows form a prefix of the trace: `when_transition().assert_zero(next.v * (1 - v))`.

2. **Canonical Zero-Limb Encoding of $\mathcal{O}$:**
   - For each input point $i$ and each of its coordinate values $k \in \{2i, 2i+1\}$ ($x_i$ and $y_i$), every limb $\ell$ satisfies:
     $$f_i \cdot \ell = 0$$
   - Thus any flagged point ($f_i = 1$) has the unique limb encoding $(x_i = 0, y_i = 0, f_i = 1)$.
   - Every coordinate limb in `local[off..]` is range-checked to 8 bits on `RANGE_BUS` (`1000`) with multiplicity $1$.

3. **Chain Enable $e$ and Bypass Multiplicity $v - e$:**
   - For **addition** (`npts = 2`):
     $$g = (1 - f_0)(1 - f_1), \qquad e = v \cdot g$$
   - For **doubling** (`npts = 1`):
     $$e = v \cdot (1 - f_0)$$
   - `PointsIn` sends the input coordinates to the chain (`o, o+1` for `add`; `o+8, o` for `dbl`) with multiplicity $e \in \{0, 1\}$.
   - Because every `ChainAir` (`L_add`/`L_dbl`, `SQ`, `X3`, `Y3`, `INV`/`INV_y`) also constrains `id = row_index` and sends/receives on internal chain buses with multiplicity `cols.is_valid`, LogUp multiset equality per unique `id` forces:
     $$\text{is\_valid}_{\text{SQ}}[r] = \text{is\_valid}_{L}[r] = \text{is\_valid}_{\text{INV}}[r] = \text{is\_valid}_{X_3}[r] = \text{is\_valid}_{Y_3}[r] = e[r]$$
   - When $v = 1$ and $e = 0$ ($v - e = 1$, at least one input is $\mathcal{O}$), the chain is bypassed for row $r$ and `PointsIn` sends the result directly on the `OUT` bus (`o+7`) with multiplicity $v - e = 1$:
     - **Add (`npts = 2`):** $(\text{id}, \text{sel}_x, \text{sel}_y, f_0 \cdot f_1)$ where limbwise $\text{sel} = P_1 + f_0 (P_2 - P_1)$:
       - $(f_0, f_1) = (1, 0)$ ($\mathcal{O} + P_2$): $\text{sel} = P_2$, $\text{out\_flag} = 0 \implies (x_2, y_2, 0) = P_2$.
       - $(f_0, f_1) = (0, 1)$ ($P_1 + \mathcal{O}$): $\text{sel} = P_1$, $\text{out\_flag} = 0 \implies (x_1, y_1, 0) = P_1$.
       - $(f_0, f_1) = (1, 1)$ ($\mathcal{O} + \mathcal{O}$): $\text{sel} = P_2 = (0, 0)$ (forced zero by $f_1 \cdot \text{limb} = 0$), $\text{out\_flag} = 1 \implies (0, 0, 1) = \mathcal{O}$.
     - **Doubling (`npts = 1`):** $(\text{id}, x_1, y_1, f_0) = (\text{id}, 0, 0, 1) = \mathcal{O}$ (forced zero by $f_0 \cdot \text{limb} = 0$, since $2\mathcal{O} = \mathcal{O}$).
   - When $v = 1$ and $e = 1$ ($v - e = 0$, non-identity inputs), `PointsIn` sends with multiplicity $0$ on `o+7`, while `Y3` sends $(\text{id}, x_3, y_3, 0)$ on `o+7` with multiplicity $e = 1$.

---

## 3. Formal Quint Model & Refinement Invariant `invRow`

The Quint model in [x14_identity.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-identity/qa/gemini-quint/x14-identity/x14_identity.qnt) and test suite in [x14_identity_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-identity/qa/gemini-quint/x14-identity/x14_identity_test.qnt) instantiate the exact bus topology (`o..o+10`) and AIR constraints over the small prime $p = 13$:

- **Curve:** $E(\mathbb{F}_{13}) : y^2 = x^3 + 4 \pmod{13}$, matching the BLS12-381 $\mathbb{G}_1$ short Weierstrass equation $y^2 = x^3 + 4$.
- **Structural fidelity to BLS12-381 $\mathbb{G}_1$:**
  - $\#E(\mathbb{F}_{13}) = 19$ (prime order).
  - $(0, 0) \notin E(\mathbb{F}_{13})$ since $0^2 \neq 0^3 + 4 \pmod{13}$, so every valid affine curve point has non-zero coordinates $(x, y) \neq (0, 0)$.
  - No affine point on $E(\mathbb{F}_{13})$ has $y = 0$ (no 2-torsion since 19 is odd).
  - Generator multiples used in traces: $G_1 = (2, 5)$, $2G_1 = (12, 9)$, $3G_1 = (8, 3)$, $4G_1 = (6, 5)$, plus explicit identity `PT_INF = { x: 0, y: 0, is_inf: true }`.
- **Refinement Invariant `invRow`:**
  For every row $k \in \{0, 1, 2, 3\}$ of any accepted trace (`producerValid and pointsInValid and chainValid and busBalanced`):
  - If `pin[k].v == 1`: the consumer (`SinkAir`) receives messages with `id == k` of total multiplicity $1$, and every active consumer row for `id == k` equals the true group result `g1Add(in1, in2)` (or `g1Dbl(in1)`) with canonical identity flag `expected_f` ($\mathcal{O} + P = P$, $P + \mathcal{O} = P$, $\mathcal{O} + \mathcal{O} = \mathcal{O}$, $2\mathcal{O} = \mathcal{O}$, and the affine chord/tangent sum for non-identity inputs).
  - If `pin[k].v == 0`: the consumer receives total multiplicity $0$ for `id == k`.

---

## 4. Mutant Analysis & Verification Commands

| Module | Mutation | Soundness / Completeness Failure Mechanism | Exact Quint Command | Expected Exit Code |
| :--- | :--- | :--- | :--- | :--- |
| Typecheck | — | Static type and syntax verification of model and test files | `quint typecheck qa/gemini-quint/x14-identity/x14_identity.qnt` | `0` |
| Typecheck (tests) | — | Static type and syntax verification of test suite | `quint typecheck qa/gemini-quint/x14-identity/x14_identity_test.qnt` | `0` |
| Unit / Neg Tests | — | 11 deterministic tests (group law, honest `add`/`dbl`, `inf_lie`, `inf_out`, 5 mutants, bus drop/dup) | `quint test qa/gemini-quint/x14-identity/x14_identity_test.qnt` | `0` |
| `x14_identity` (sound) | None | Holds across all honest and adversarial prover steps | `quint run qa/gemini-quint/x14-identity/x14_identity.qnt --main=x14_identity --invariant=invRow --max-samples=500 --seed=0x14` | `0` |
| `x14_identity` (non-vacuity) | Negated reachability (`invNonVacuous`) | Violated on step 0 by honest 4-row `add` trace ($\mathcal{O}+G_1$, $G_1+\mathcal{O}$, $\mathcal{O}+\mathcal{O}$, $G_1+G_2$) | `quint run qa/gemini-quint/x14-identity/x14_identity.qnt --main=x14_identity --invariant=invNonVacuous --max-samples=500 --seed=0x14` | `1` (violated) |
| `m_no_zero_limbs` | Drop $f_i \cdot \text{limb} = 0$ | Prover sets $f_0 = 1$ on real point $P_1 = G_1 = (2, 5)$ in $G_1 + G_2$; chain is skipped ($e=0$) and circuit outputs $G_2 = (12, 9)$ instead of $G_3 = (8, 3)$ | `quint run qa/gemini-quint/x14-identity/x14_identity.qnt --main=m_no_zero_limbs --invariant=invRow --max-samples=500 --seed=0x14` | `1` (violated) |
| `m_e_ignores_flag` | $e = v$ in place of $v \cdot g$ / $v(1-f_0)$ | On $\mathcal{O} + G_1$ ($P_1 = (0,0), P_2 = (2,5)$), $x_2 - x_1 = 2 \neq 0$ is invertible, so the chain **accepts a wrong value** $(1, 4, 0) \neq (2, 5, 0)$ (while rejecting $\mathcal{O}+\mathcal{O}$ and $2\mathcal{O}$) | `quint run qa/gemini-quint/x14-identity/x14_identity.qnt --main=m_e_ignores_flag --invariant=invRow --max-samples=500 --seed=0x14` | `1` (violated) |
| `m_sel_wrong` | $\text{sel} = P_1$ always | On $\mathcal{O} + G_1$ ($f_0 = 1, f_1 = 0$), `PointsIn` sends $P_1 = (0, 0, 0)$ instead of $P_2 = (2, 5, 0)$ | `quint run qa/gemini-quint/x14-identity/x14_identity.qnt --main=m_sel_wrong --invariant=invRow --max-samples=500 --seed=0x14` | `1` (violated) |
| `m_out_flag_wrong` | Output flag $f_0 + f_1$ in place of $f_0 f_1$ | On $\mathcal{O} + G_1$ and $G_1 + \mathcal{O}$, outputs flag $1$ on a non-identity point (marking $G_1$ as $\mathcal{O}$); on $\mathcal{O} + \mathcal{O}$, outputs non-boolean flag $2$ | `quint run qa/gemini-quint/x14-identity/x14_identity.qnt --main=m_out_flag_wrong --invariant=invRow --max-samples=500 --seed=0x14` | `1` (violated) |
| `m_out_mult_v` | `PointsIn` `OUT` send multiplicity $v$ in place of $v - e$ | On active chain row $G_1 + G_2$ ($v = 1, e = 1$), both `PointsIn` ($\text{mult} = 1$) and `Y3` ($\text{mult} = 1$) send for $\text{id} = 0$; spare consumer row absorbs $(0, G_3, 0)$ while row 0 takes wrong output $(0, G_1, 0)$ | `quint run qa/gemini-quint/x14-identity/x14_identity.qnt --main=m_out_mult_v --invariant=invRow --max-samples=500 --seed=0x14` | `1` (violated) |

### Detailed Answer on `m_e_ignores_flag` ($e = v$ with Zero Limbs)

When $e = v$ (`m_e_ignores_flag`), a flagged point still has zero limbs $(0, 0)$ due to $f_i \cdot \text{limb} = 0$, and $v - e = 0$ disables the `PointsIn` bypass send on `o+7` while sending $(0, 0)$ to the chain with multiplicity $e = 1$. What happens in the chain depends on which inputs are $\mathcal{O}$:

1. **$\mathcal{O} + P_2$ (with $x_2 \neq 0$) and $P_1 + \mathcal{O}$ (with $x_1 \neq 0$) in `add` — THE CHAIN ACCEPTS A WRONG VALUE (Soundness Failure):**
   - Because $(0, 0)$ has $x = 0$, the denominator $x_2 - x_1$ equals $x_2$ (for $\mathcal{O} + P_2$) or $-x_1$ (for $P_1 + \mathcal{O}$).
   - For any curve point $P = (x, y)$ with $x \neq 0$, $x_2 - x_1 \not\equiv 0 \pmod{p}$ is invertible!
   - Therefore, `INV` succeeds with $w = (x_2 - x_1)^{-1} \pmod{p}$, `L_add` succeeds with $\lambda = (y_2 - y_1)(x_2 - x_1)^{-1} = \pm y / x \pmod{p}$, and `X3`/`Y3` compute the affine chord formula through the fake non-curve point $(0, 0)$ and $P$:
     $$x_3 = (y/x)^2 - x \pmod{p}, \qquad y_3 = \begin{cases} -(y/x) x_3 \pmod{p} & (\mathcal{O} + P_2) \\ (y/x)(x - x_3) - y \pmod{p} & (P_1 + \mathcal{O}) \end{cases}$$
   - Over $\mathbb{F}_{13}$ with $P = G_1 = (2, 5)$, $\mathcal{O} + G_1$ gives $w = 2^{-1} \equiv 7$, $\lambda = 5 \cdot 7 \equiv 9$, $x_3 = 9^2 - 2 \equiv 1$, $y_3 = 9(0 - 1) \equiv 4$, so the chain **accepts** and outputs $(1, 4, 0)$ (which is not even on $y^2 = x^3 + 4$) instead of $G_1 = (2, 5, 0)$.

2. **$\mathcal{O} + \mathcal{O}$ in `add` (and $\mathcal{O} + P$ when $x = 0$) — THE CHAIN REJECTS (Completeness Failure):**
   - Here $x_1 = x_2 = 0 \implies x_2 - x_1 = 0$, so `INV` requires $w \cdot 0 \equiv 1 \pmod{p}$, which has no solution.

3. **$2\mathcal{O}$ in `dbl` — REJECTS with `INV_y`, or ACCEPTS A WRONG VALUE under `X14_NO_INV_Y=1`:**
   - With `INV_y` enabled (default): $y_1 = 0$, so `INV_y` requires $w \cdot 0 \equiv 1 \pmod{p}$ and **rejects**.
   - With `X14_NO_INV_Y=1`: `INV_y` is removed, and `L_dbl` checks $\lambda \cdot (2 y_1) \equiv 3 s \pmod{p}$, which degenerates to $\lambda \cdot 0 \equiv 0 \pmod{p}$ when $(x_1, y_1) = (0, 0)$. The prover can choose **any** $\lambda \in \mathbb{F}_p$ and the chain **accepts** the forged point $(\lambda^2, -\lambda^3, 0) \neq \mathcal{O}$.

---

## 5. Answers to the Three Review Questions

### Question (1): Is the design complete for every case of a bucket MSM where only input points (never an accumulator) can be $\mathcal{O}$?

- **For all identity-input cases, YES:**
  - Even though a bucket MSM with non-identity accumulators only needs $P_{\text{acc}} + \mathcal{O} = P_{\text{acc}}$ (or $\mathcal{O} + P_{\text{in}} = P_{\text{in}}$) and $2 P_{\text{acc}}$ with $P_{\text{acc}} \neq \mathcal{O}$, `PointsIn` is complete for all four identity combinations: $\mathcal{O} + P = P$, $P + \mathcal{O} = P$, $\mathcal{O} + \mathcal{O} = \mathcal{O}$, and $2\mathcal{O} = \mathcal{O}$.
  - Because the accumulator is never $\mathcal{O}$, inverse-point additions $P + (-P) = \mathcal{O}$ (where $f_0 = f_1 = 0$, $x_1 = x_2$, $y_1 = -y_2$, which cannot be handled by the affine `add` chain because $x_2 - x_1 = 0$ fails `INV` and `Y3` hardcodes output flag `0`) never occur.
- **Required non-identity condition for `add` completeness ($x_1 \neq x_2$):**
  - When neither input is $\mathcal{O}$ ($f_0 = 0, f_1 = 0 \implies e = 1$), the `add` chain runs `INV` on $x_2 - x_1$ (`w = 1 / (x2 - x1)`). This requires **$x_1 \neq x_2 \pmod{p}$**, which rules out feeding two equal points $P_1 = P_2 \neq \mathcal{O}$ into the `add` chain (`neg == "same_pt"`).
  - Therefore, the bucket MSM schedule must ensure that an active bucket accumulator never has the same $x$-coordinate as the point being added to it (e.g., by initializing buckets/accumulators with a distinct offset point or handling equal-point additions via `dbl`).
- **For `dbl` completeness ($y_1 \neq 0$):**
  - Every non-identity point in $\mathbb{G}_1$ has $y_1 \neq 0$ because $\#\mathbb{G}_1 = r$ is an odd prime (no 2-torsion points exist in $\mathbb{G}_1$), so `INV_y` ($w = 1/y_1$) is always satisfiable for $P_1 \in \mathbb{G}_1 \setminus \{\mathcal{O}\}$.
- **Trace-generation condition on bypassed/padding chain rows ($e = 0$):**
  - As verified in [lane-src/check_carry_to_zero.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-identity/lane-src/check_carry_to_zero.rs#L98-L108), `CheckCarryToZeroSubAir::eval` does **not** gate its carry-propagation equality constraints by `is_valid` (only the `RANGE_BUS` lookups and `ChainAir` bus messages are gated by `is_valid`). Consequently, when $e = 0$ (either $v = 0$ padding or $v = 1$ identity bypass), the prover must populate a dummy satisfying witness in each `ChainAir` row with `is_valid = 0` and discard its range-checker increments (`tmp_rc` in [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-identity/lane-src/x_exp_node.rs#L1091-L1092)).

### Question (2): What must the producer bind (flag and limbs to the transcript encoding), and what breaks if it does not?

For every valid row ($v = 1$) and each input point $i$, the upstream producer (transcript sponge or commitment opening) must bind **both** the 96 coordinate limbs $(x_i, y_i)$ **and** the boolean flag $f_i$ to the authenticated point representation:

1. **If the producer fails to bind $f_i = 1$ when the transcript point is $\mathcal{O} = (0, 0)$ (allowing $f_i = 0$ on zero limbs):**
   - `PointsIn` only enforces $f_i \cdot \text{limb} = 0$, which is trivially satisfied when $f_i = 0$ and $\text{limb} = 0$ ($0 \cdot 0 = 0$).
   - A malicious prover can set $f_0 = 0$ on $P_1 = \mathcal{O} = (0, 0)$ in an addition $\mathcal{O} + P_2$ (with $x_2 \neq 0$). Then $g = 1, e = 1$, and as shown in `m_e_ignores_flag`, the `add` chain executes on $(0, 0)$ and $(x_2, y_2)$ with $x_2 - 0 = x_2$ invertible, **accepting the forged secant output** $((y_2/x_2)^2 - x_2, -(y_2/x_2)x_3, 0) \neq P_2$.
   - And if `X14_NO_INV_Y=1` is enabled on `dbl`, setting $f_0 = 0$ on $\mathcal{O} = (0, 0)$ lets the prover pick any $\lambda \in \mathbb{F}_p$ in `L_dbl` ($0 \cdot \lambda = 0$) and forge $(\lambda^2, -\lambda^3, 0)$ for $2\mathcal{O}$.

2. **If the producer gates limb binding by $(1 - f_i)$ without binding $f_i$ to the transcript's infinity bit:**
   - Suppose the producer only constrains $(1 - f_i)(\text{limb} - \text{transcript\_limb}) = 0$ without constraining $f_i = f_i^{\text{transcript}}$.
   - Then for any real non-identity point $P_i \neq \mathcal{O}$, the prover can set $f_i = 1$ and $\text{limb} = 0$ in `PointsIn`. That satisfies $f_i \cdot \text{limb} = 0$ ($1 \cdot 0 = 0$) and erases $P_i \mapsto \mathcal{O}$, turning $P_1 + P_2$ into $P_2$ (**point-skipping soundness failure**).

3. **If the producer binds $(x_i, y_i)$ unconditionally to the transcript limbs, why $f_i \cdot \text{limb} = 0$ is still essential:**
   - Because $(0, 0) \notin E(\mathbb{F}_p)$ on $y^2 = x^3 + 4$, every valid affine curve point $P_i \neq \mathcal{O}$ has at least one non-zero limb. Unconditional limb binding plus $f_i \cdot \text{limb} = 0$ forces $f_i = 0$ on every $P_i \neq \mathcal{O}$. Without $f_i \cdot \text{limb} = 0$ (`m_no_zero_limbs`), the prover can set $f_i = 1$ while keeping the true non-zero limbs of $P_i$, again skipping $P_i$ (`neg == "inf_lie"`).

4. **Curve membership when `X14_NO_INV_Y=1`:**
   - When `INV_y` is omitted in `dbl` (`X14_NO_INV_Y=1`), the producer must guarantee that every unflagged point ($f_0 = 0$) lies on $E(\mathbb{F}_p)$ (where $y = 0$ has no solution in $\mathbb{G}_1$), because otherwise an off-curve input $(x_1, 0)$ with $x_1 \neq 0$ or $(0, 0)$ causes $2 y_1 = 0$ in `L_dbl`.

### Question (3): Cost Analysis (`PointsIn` Columns and Interactions)

From [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-identity/lane-src/x_exp_node.rs#L760-L832) (`nl = 48` limbs per field element):

| Variant | Metric | Previous (`SourceAir`) | New (`PointsIn`) | Delta | Breakdown |
| :--- | :--- | ---: | ---: | ---: | :--- |
| **Add (`npts = 2`, `nv = 4`)** | Columns | `193` | **`198`** | **`+5`** | `off = 6` (`id, v, f_0, f_1, g, e`) + $4 \times 48 = 192$ limb cols (was `id` + 192 limbs). Auxiliary column $g = (1-f_0)(1-f_1)$ keeps `e = v * g` quadratic (degree 2). |
| **Add (`npts = 2`, `nv = 4`)** | Interactions | `194` | **`195`** | **`+1`** | $4 \times 48 = 192$ limb range checks on `RANGE_BUS` + 2 chain-input sends (`o, o+1`, mult $e$) + **1 bypass send on `o+7`** (mult $v - e$, 98 fields). |
| **Doubling (`npts = 1`, `nv = 2`)** | Columns | `97` | **`100`** | **`+3`** | `off = 4` (`id, v, f_0, e`) + $2 \times 48 = 96$ limb cols (was `id` + 96 limbs). No $g$ column needed since $e = v(1 - f_0)$ is already degree 2. |
| **Doubling (`npts = 1`, `nv = 2`)** | Interactions | `98` | **`99`** | **`+1`** | $2 \times 48 = 96$ limb range checks on `RANGE_BUS` + 2 chain-input sends (`o+8, o`, mult $e$) + **1 bypass send on `o+7`** (mult $v - e$, 98 fields). |

- **Chain AIRs overhead:** `0` additional columns and `0` additional interactions across `L_add`/`L_dbl`, `SQ`, `X3`, `Y3`, `INV`/`INV_y`. `Y3` only appends `Ref::Zero` (`1` constant field element) to its `o+7` send (`97 -> 98` fields, within the LogUp `max_message_length = 128` bound).

---

## 6. Explanation of Measured Negative Tests (`g1neg` in `x_exp_node.rs`)

All three measured cases in [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-identity/lane-src/x_exp_node.rs#L1072-L1197) are modeled in `x14_identity.qnt` and verified by unit tests in `x14_identity_test.qnt`:

1. **Honest $\mathcal{O} + P$, $P + \mathcal{O}$, and $\mathcal{O} + \mathcal{O}$ (`neg == "inf"`, modeled in `honestAddVerifyTest` & `honestDblVerifyTest`):**
   - Rows $r = 0, 1, 2$ have $(f_0, f_1) = (1, 0), (0, 1), (1, 1)$ with zeroed limbs on flagged inputs, $g = 0$, and $e = 0$.
   - Chain rows at $r = 0, 1, 2$ have `is_valid = 0` (holding dummy satisfying witnesses for `CheckCarryToZeroSubAir` whose range-check counts are sent to a throwaway `tmp_rc`), so they emit no messages (`mult = 0`) on buses `o..o+10`.
   - `PointsIn` sends $(0, P_2, 0)$, $(1, P_1, 0)$, and $(2, 0, 0, 1)$ on `o+7` with multiplicity $v - e = 1$, matching `SinkAir` rows $0, 1, 2$. All AIR constraints and LogUp buses verify.

2. **Flag on a real point with nonzero limbs (`neg == "inf_lie"`, modeled in `negFlagOnRealPointFailsOnlyPointsInTest`):**
   - Row $0$ sets $f_0 = 1, f_1 = 0, g = 0, e = 0$, but keeps the real non-zero limbs $(x_1, y_1)$ of $P_1$ in `PointsIn` (`keep(f1) == true` at line 1155).
   - `PointsIn` sends $(0, x_2, y_2, 0)$ on `o+7` with multiplicity $v - e = 1$ (since $\text{sel} = x_1 + 1 \cdot (x_2 - x_1) = x_2$), and `SinkAir` receives $(0, x_2, y_2, 0)$ with multiplicity $1$.
   - Because `PointsIn` and `SinkAir` agree on `o+7`, the chain is off ($e = 0, \text{is\_valid} = 0$), and the 8-bit range checker includes the limbs of $(x_1, y_1)$, **LogUp multiset balance (`logup_ok = true`) and all other AIRs pass**; **only `PointsIn` fails** (`builder.assert_zero(f[0] * l)` fails on the non-zero limbs of $x_1, y_1$).

3. **Consumer takes the chain sum in place of $P_2$ for $\mathcal{O} + P_2$ (`neg == "inf_out"`, modeled in `negConsumerTakesChainSumFailsLogUpTest`):**
   - Row $0$ has $f_0 = 1, f_1 = 0$, zeroed limbs for $P_1$, $g = 0, e = 0$. `PointsIn` satisfies all polynomial constraints and sends $(0, x_2, y_2, 0)$ on `o+7` with multiplicity $v - e = 1$.
   - Chain rows at $r = 0$ have `is_valid = 0`, so `Y3` sends $(0, x_3, y_3, 0)$ with multiplicity $0$.
   - `SinkAir` row $0$ sets its columns to $(0, x_3, y_3, 0)$ with multiplicity $1$ (`line 1186`).
   - Every AIR's local polynomial constraints are satisfied (`constraint_fail_airs = []`), because `SinkAir` has no local coordinate equation and `PointsIn`/chain rows are locally valid. However, **LogUp balance fails (`logup_ok = false`)** on bus `o+7`: $(0, x_2, y_2, 0)$ has net multiplicity $+1$ and $(0, x_3, y_3, 0)$ has net multiplicity $-1$.

---

## 7. Additional Review Note on `SinkAir` (Test Harness vs. Production Consumer)

In [lane-src/x_exp_node.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-identity/lane-src/x_exp_node.rs#L838-L859), `SinkAir` (documented as `/// Test harness only`) receives `(local[0], local[3..], local[2])` on `o+7` with multiplicity `local[1]` (`v`), without calling `builder.assert_bool(local[1])`. As noted in [lane-src/interaction_mod.rs](file:///home/hoskinson/.cache/gemini-lanes/x14-identity/lane-src/interaction_mod.rs#L163) (`PermutationCheckBus::receive`: *"Caller must constrain `enabled` to be boolean"*), the real MSM accumulator / consumer AIR that replaces `SinkAir` in production must constrain its receive multiplicity to $\{0, 1\}$.
