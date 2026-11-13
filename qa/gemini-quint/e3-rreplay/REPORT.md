# E3 J7 Soundness & Completeness Review: Reservation `R` Native Replay (`gen_r_check`)

## Executive Summary

This report presents an independent soundness and completeness review of `gen_r_check` in [lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L165-L292), together with its underlying tape/AIR semantics in [tape.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/tape.rs), [machine.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/machine.rs), [fr.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/fr.rs), [poseidon.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/poseidon.rs), and the native reference/decoder in [iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/iface.rs), accompanied by a reduced-scale formal model in [rreplay.qnt](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/qa/gemini-quint/e3-rreplay/rreplay.qnt) and test suite in [rreplay_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/qa/gemini-quint/e3-rreplay/rreplay_test.qnt).

| Question | Topic | Verdict | Summary |
| :--- | :--- | :--- | :--- |
| **Q1** | Zero tests `is_zero(a)`, `tag_in`, `tag_out`, `mode` bit | **SOUND** | Over prime field $\mathbb{F}_p$, $a w = 1 - z \land a z = 0$ uniquely forces $z = [a = 0] \in \{0, 1\}$ for every prover choice of $(w, z) \in \mathbb{F}_p^2$; `mode * (mode - 1) = 0` (and `CSwapAir`) forces $\text{mode} \in \{0, 1\}$; `tag_in` and `tag_out` are uniquely forced under limb-wise equality. |
| **Q2** | Entry checks `0..5`, `e_in`, `e_out`, and `R_OFF` alignment | **SOUND** | Entries `0..3` are fixed to `DIGEST(0)`, `DIGEST(2)`, `DIGEST(3)`, `SPONGE(3)`; entry `4` to `DIGEST(tag_in)`; entry `5` to `DIGEST(tag_out)` iff $\text{mode} = 1$; $e_{\text{in}} = 4$, $e_{\text{out}} = 4 + \text{mode}$; `R_OFF = [0, 2, 4, 6, 10, 12]` matches cumulative `vals(op, param)`. |
| **Q3** | Native replay (`DIGEST`, `SPONGE(3)`, and entry-5 `cswap`) | **SOUND** | Each `DIGEST` replay matches `digest(tag, x) = Perm([tag_fr(tag), x, cap(4, 2)])[0]`; `SPONGE(3)` matches `sponge_u(0, [o0, o1, o2])` in state layout `[rate0, rate1, cap=3]` and absorption order; `cswap(mode, y, o[0]) == y` enforces entry 5 iff $\text{mode} = 1$. |
| **Q4** | Value limb entry (`fr_val`) vs. reader `decode` (`dec`) | **SOUND (with `decode`) / UNSOUND (tape standalone)** | The tape alone accepts non-canonical limbs $\in [2^{30}, p)$ and 9-limb integers $\ge r$, reducing them modulo $r$. However, `decode` ([iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/iface.rs#L391-L404)) refuses both (`x >= 2^30` and `big >= r`). On all inputs accepted by both, `fr_val` and `dec` are identical. |
| **Q5** | Completeness across ordinary calls and all chunk regimes | **SOUND & COMPLETE** | Honest witnesses (`l_vals` advice) satisfy every assertion for ordinary calls ($\text{mode} = 0$) and chunks with $j = 1 < k$, $1 < j < k$, $1 < j = k$, and $j = k = 1$. |
| **Q6** | Missing range/order checks on $j, k$ (`16`-bit limbs, $1 \le j \le k$) | **UNSOUND (without range/order check)** | Neither `gen_r_check` nor `decode` checks $j_{\text{lo}}, j_{\text{hi}}, k_{\text{lo}}, k_{\text{hi}} < 2^{16}$ or $1 \le j \le k$. This permits $j = 0$, $j > k$, and limb aliasing $\ge 2^{16}$ where `u32w` ($lo + 2^{16} hi$) in `anchor_tags` disagrees with limb-wise equality in `gen_r_check`. |

---

## Detailed Answers and Evidence per Question

### Question 1: Zero Tests, `tag_in`, `tag_out`, and `mode` Bit

**Verdict: SOUND** (with `[j = 1]` defined as $j_{\text{lo}} = 1 \land j_{\text{hi}} = 0$ and `[j = k]` as limb-wise equality $j_{\text{lo}} = k_{\text{lo}} \land j_{\text{hi}} = k_{\text{hi}}$; see Question 6 for non-16-bit limb aliasing).

1. **Field semantics**:
   - All public words in `l_pub` and advice inputs allocated via `g.bb_in(...)` ([lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L94-L98)) are `ValKind::Bb` rows of `InputAir` ([tape.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/tape.rs#L576-L606)), which constrains coefficient 0 to be a canonical BabyBear field element $v_0 \in [0, p)$ ($p = 2^{31} - 2^{27} + 1$, prime) and coefficients $1..3$ to $0$.
   - `g.add`, `g.sub`, and `g.mul` emit `BbAluProgAir` rows ([tape.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/tape.rs#L701-L727)) performing arithmetic in $\mathbb{F}_p$, and `g.assert_eq` emits `AssertEqAir` rows ([machine.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/machine.rs#L580-L586)) enforcing equality on `MEM_BUS`.

2. **Is `mode` forced to be a bit?**
   - In [lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L177-L180):
     ```rust
     let mode = l_pub[MODE_WORD];
     let m1 = g.sub(mode, one);
     let mm = g.mul(mode, m1);
     g.assert_eq(mm, zero);
     ```
   - This enforces $\text{mode} \cdot (\text{mode} - 1) \equiv 0 \pmod p$. Since $\mathbb{F}_p$ is a prime field (an integral domain), the only roots are $\text{mode} \in \{0, 1\}$.
   - In addition, line 290 passes `mode` as the selector bit to `g.tape.cswap(mode, y, o[0])`, where `CSwapAir` ([machine.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/machine.rs#L908-L909)) independently enforces `builder.assert_bool(bit)` in $\mathbb{F}_r$.

3. **Does `is_zero(a)` with advice `(w, z)` force $z = [a = 0]$ for every prover choice?**
   - In [lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L182-L196), `is_zero` allocates free advice $(w, z) \in \mathbb{F}_p^2$ and enforces:
     $$a \cdot w \equiv 1 - z \pmod p \quad\text{and}\quad a \cdot z \equiv 0 \pmod p.$$
   - **Case $a \equiv 0 \pmod p$**: The first equation reduces to $0 = 1 - z \pmod p$, forcing $z = 1$ for every $w \in \mathbb{F}_p$. The second equation is $0 \cdot 1 = 0$, which holds.
   - **Case $a \not\equiv 0 \pmod p$**: Since $p$ is prime, $a \in \mathbb{F}_p^\times$ has a multiplicative inverse $a^{-1} \bmod p$. Multiplying $a \cdot z \equiv 0 \pmod p$ by $a^{-1}$ forces $z = 0$. Substituting $z = 0$ into $a \cdot w \equiv 1 - z \pmod p$ yields $a \cdot w \equiv 1 \pmod p$, uniquely forcing $w = a^{-1} \bmod p$.
   - Hence for **every** $a \in \mathbb{F}_p$ and **every** prover choice of $(w, z) \in \mathbb{F}_p^2$, the constraints force $z = [a = 0] \in \{0, 1\}$.

4. **Are `tag_in = mode (1 - [j = 1])` and `tag_out = mode (1 - [j = k])` forced?**
   - In [lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L199-L217):
     - $z_{1a} = [j_{\text{lo}} - 1 = 0] = [j_{\text{lo}} = 1] \in \{0, 1\}$ and $z_{1b} = [j_{\text{hi}} = 0] \in \{0, 1\}$.
     - $z_{j1} = z_{1a} \cdot z_{1b} = [j_{\text{lo}} = 1 \land j_{\text{hi}} = 0] \in \{0, 1\}$.
     - $z_{ka} = [j_{\text{lo}} - k_{\text{lo}} = 0] = [j_{\text{lo}} = k_{\text{lo}}] \in \{0, 1\}$ and $z_{kb} = [j_{\text{hi}} - k_{\text{hi}} = 0] = [j_{\text{hi}} = k_{\text{hi}}] \in \{0, 1\}$.
     - $z_{jk} = z_{ka} \cdot z_{kb} = [j_{\text{lo}} = k_{\text{lo}} \land j_{\text{hi}} = k_{\text{hi}}] \in \{0, 1\}$.
     - Therefore $\text{tag\_in} = \text{mode} \cdot (1 - z_{j1}) \in \{0, 1\}$ and $\text{tag\_out} = \text{mode} \cdot (1 - z_{jk}) \in \{0, 1\}$ are uniquely forced to `SHK_PROGC` ($0$) or `SHK_LOCK` ($1$).

---

### Question 2: Entry Checks and `R_OFF` Value Offset Alignment

**Verdict: SOUND**

1. **Entry checks (`0..5`) and anchor entry words (`e_in`, `e_out`)**:
   - In [lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L219-L243), with `ent(i) = (l_pub[STMT + 2*i], l_pub[STMT + 2*i + 1])` (`STMT = 114`):
     - **Entries `0..2`**: Loop over `R_RUN = [SHK_PROGC (0), SHK_INPUT (2), SHK_OUTCOME (3)]` asserts `ent(0) = (DIGEST, SHK_PROGC)`, `ent(1) = (DIGEST, SHK_INPUT)`, `ent(2) = (DIGEST, SHK_OUTCOME)`.
     - **Entry `3`**: Asserts `ent(3) = (SPONGE, 3)`.
     - **Entry `4` (`ANCHOR_IN = 4`)**: Asserts `ent(4) = (DIGEST, tag_in)`.
     - **Entry `5` (`ANCHOR_IN + 1 = 5`)**: Asserts $\text{mode} \cdot (\text{op}_5 - \text{DIGEST}) = 0$ and $\text{mode} \cdot (\text{param}_5 - \text{tag\_out}) = 0$ in $\mathbb{F}_p$. Since $\text{mode} \in \{0, 1\}$, when $\text{mode} = 1$ this forces $\text{ent}(5) = (\text{DIGEST}, \text{tag\_out})$; when $\text{mode} = 0$ both products are $0$ and entry 5 is unconstrained by `gen_r_check` (allowing `NOP` or the first discretionary entry).
     - **`e_in` and `e_out`**: Lines 240–243 assert `l_pub[E_IN_WORD] == 4` (`E_IN_WORD = 79`) and `l_pub[E_OUT_WORD] == 4 + mode` (`E_OUT_WORD = 80`).

2. **Value offsets `R_OFF` vs. `vals(op, param)`**:
   - By [iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/iface.rs#L128-L140), `vals(DIGEST, _) = 2` and `vals(SPONGE, 3) = 3 + 1 = 4`.
   - Cumulative value offsets for entries `0..5` are:
     - Entry 0 (`DIGEST`): offset $0$, uses values $0, 1$ (count $2 \implies$ next offset $2$).
     - Entry 1 (`DIGEST`): offset $2$, uses values $2, 3$ (count $2 \implies$ next offset $4$).
     - Entry 2 (`DIGEST`): offset $4$, uses values $4, 5$ (count $2 \implies$ next offset $6$).
     - Entry 3 (`SPONGE(3)`): offset $6$, uses values $6, 7, 8, 9$ (count $4 \implies$ next offset $10$).
     - Entry 4 (`DIGEST`): offset $10$, uses values $10, 11$ (count $2 \implies$ next offset $12$).
     - Entry 5 (`DIGEST` when $\text{mode} = 1$): offset $12$, uses values $12, 13$ (count $2$).
   - This matches `R_OFF: [usize; 6] = [0, 2, 4, 6, 10, 12]` ([iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/iface.rs#L78)) at every entry.
   - Lines 245–249 of [lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L245-L249) assert `limb(R_OFF[3] + i, t) == limb(R_OFF[i] + 1, t)` for all $i \in \{0, 1, 2\}$ and $t \in [0, 9)$, forcing the three `SPONGE(3)` input values (slots $6, 7, 8$) to equal the three run `DIGEST` output values (slots $1, 3, 5$) limb by limb.

---

### Question 3: Native Replay Equivalence (`DIGEST`, `SPONGE(3)`, and `cswap`)

**Verdict: SOUND**

1. **`DIGEST` replay vs. native `digest(tag, d2)`**:
   - In [iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/iface.rs#L224-L226), `digest(tag, d2) = sponge_u(4, &[tag_fr(tag), d2])`.
   - For a 2-word input at use code $u = 4$, `sponge_u` ([iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/iface.rs#L192-L206)) initializes state $s = [0, 0, 2 + 4 \cdot 2^{66}] = [0, 0, \text{cap\_digest}()]$, absorbs `tag_fr(tag)` into $s[0]$ and `d2` into $s[1]$, applies `perm([tag_fr(tag), d2, cap_digest()])`, and outputs $s[0]$.
   - In [lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L262-L291):
     - For entries `0..2`, `o = g.tape.perm([tk, x, cap])` and `g.assert_eq(o[0], y)` directly enforce $y = \text{Perm}([\text{tag\_fr}(t), x, \text{cap\_digest}()])[0]$.
     - For entries `4` and `5`, `g.tape.cswap(tag_bit, t_progc, t_lock)` ([machine.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/machine.rs#L905-L916)) computes $l = a + \text{bit} \cdot (b - a)$, which selects `t_progc = tag_fr(0)` when $\text{tag\_bit} = 0$ and `t_lock = tag_fr(1)` when $\text{tag\_bit} = 1$.

2. **`SPONGE(3)` replay vs. native `sponge_u(0, [o0, o1, o2])`**:
   - In [iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/iface.rs#L192-L206), `sponge_u(0, &[o0, o1, o2])`:
     1. Initializes state $s = [0, 0, 3 + 0 \cdot 2^{66}] = [0, 0, 3]$.
     2. Chunk 1 (`[o0, o1]`): adds $o_0$ to $s[0]$ and $o_1$ to $s[1]$, giving $[o_0, o_1, 3]$, and permutes: $s \leftarrow \text{perm}([o_0, o_1, 3])$.
     3. Chunk 2 (`[o2]`): adds $o_2$ to $s[0]$ (leaving $s[1], s[2]$ unchanged), giving $[s[0] + o_2, s[1], s[2]]$, and permutes: $s \leftarrow \text{perm}([s[0] + o_2, s[1], s[2]])$.
     4. Outputs $s[0]$.
   - In [lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L274-L279):
     ```rust
     let k3 = g.tape.fr_const(Fr::from_u64(3));
     let s = g.tape.perm([outs[0], outs[1], k3]);
     let s0 = g.tape.fr_add(s[0], outs[2]);
     let s = g.tape.perm([s0, s[1], s[2]]);
     let claim = fr_val(g, R_OFF[3] + 3);
     g.assert_eq(s[0], claim);
     ```
   - Because lines 245–249 already constrain the limbs of value slots $6, 7, 8$ to equal the limbs of slots $1, 3, 5$ (`outs[0..3]`), the state layout, capacity constant $3$, absorption order, and output cell $s[0]$ are identical to `sponge_u(0, [o0, o1, o2])`.

3. **Entry-5 check gated by `cswap(mode, y, o[0])`**:
   - In [lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L290-L291):
     `let (l, _) = g.tape.cswap(mode, y, o[0]); g.assert_eq(l, y);`
   - By `CSwapAir` ([machine.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/machine.rs#L910-L913)), $l = y + \text{mode} \cdot (o[0] - y)$.
   - When $\text{mode} = 0$, $l = y$, so `assert_eq(l, y)` is $y = y$ (no constraint on value slots $12, 13$).
   - When $\text{mode} = 1$, $l = o[0]$, so `assert_eq(l, y)` enforces $o[0] = y = \text{digest}(\text{tag\_out}, x)$.

---

### Question 4: Value Entry (`fr_val`) and Non-Canonical Limbs vs. `decode`

**Verdict: SOUND when composed with `decode` (Plan D-9) / UNSOUND if the tape is used without `decode`**

1. **What `fr_val` computes in the tape**:
   - In [lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L251-L261), `fr_val(g, j)` packs each BabyBear limb $l_t \in [0, p)$ into $\mathbb{F}_r$ as a 1-word `Pack` (`PackAir` in [machine.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/machine.rs#L282-L305)) and performs Horner's method with `LIMB_BITS = 30` doublings via `fr_add(acc, acc)`:
     $$\text{fr\_val}(j) = \left(\sum_{t=0}^{8} l_t \cdot 2^{30 t}\right) \bmod r \in \mathbb{F}_r.$$

2. **What a prover can do in the tape alone with non-canonical limbs**:
   - **Limb $\ge 2^{30}$ ($2^{30} \le l_t < p = 2^{31} - 2^{27} + 1$)**: Bit 30 of $l_t$ carries into the weight $2^{30(t+1)}$ of $l_{t+1}$. A prover can supply distinct public limb vectors that evaluate to the same $\mathbb{F}_r$ element (for example, $l = [2^{30}, 0, 0, \dots]$ and $l' = [0, 1, 0, \dots]$ both evaluate to $2^{30} \in \mathbb{F}_r$).
   - **9-limb integer $\ge r$**: Even with all $l_t \in [0, 2^{30})$, the 9-limb integer $B = \sum_{t=0}^{8} l_t 2^{30 t}$ ranges up to $2^{270} - 1 > r \approx 2^{255}$. A prover can represent $x + m \cdot r$ (for $m \ge 1$) in 30-bit limbs, which `fr_val` reduces modulo $r$ to $x \in \mathbb{F}_r$.

3. **Which of these `decode` refuses, and whether any discrepancy can pass both**:
   - In [iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/iface.rs#L544-L553), `decode` calls `dec(&w[at..at + LIMBS])` ([iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/iface.rs#L391-L404)) on **every** lane $i \in [0, d.\text{lanes}())$ and **every** value slot $j \in [0, d.v)$:
     ```rust
     pub fn dec(l: &[u64]) -> Option<Fr> {
         assert_eq!(l.len(), LIMBS);
         let mut big = BigUint::from(0u32);
         for (k, &x) in l.iter().enumerate() {
             if x >= 1 << LIMB_BITS {
                 return None;
             }
             big += BigUint::from(x) << (LIMB_BITS as usize * k);
         }
         if big >= Fr::order() {
             return None;
         }
         Some(fr_from_big(&big))
     }
     ```
   - `decode` **refuses both** non-canonical forms:
     1. Any limb $x \ge 2^{30}$ is refused at `if x >= 1 << LIMB_BITS { return None; }`.
     2. Any 9-limb integer $\text{big} \ge r$ is refused at `if big >= Fr::order() { return None; }`.
   - Whenever both the tape and `decode` accept, $0 \le l_t < 2^{30}$ for all $t$ and $0 \le \sum_{t=0}^{8} l_t 2^{30 t} < r$, on which $\text{fr\_val}(j) = \text{dec}(l)$ and the base-$2^{30}$ representation is bijective. **Nothing is accepted by both the tape and `decode` with a different meaning than the native statement.**
   - In [rreplay_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/qa/gemini-quint/e3-rreplay/rreplay_test.qnt#L106-L139) (`q4_value_limbs_and_decoderTest`), both non-canonical cases are demonstrated at reduced scale ($\text{LIMB\_BASE} = 4, R_{\text{ORDER}} = 13$):
     - `[4, 0]` (limb $4 \ge 4$) is accepted by `gen_r_check_ok` as $4 \in \mathbb{F}_{13}$ and refused by `dec_limbs`.
     - `[2, 3]` (both limbs $< 4$, integer $2 + 4 \cdot 3 = 14 \ge 13$, $14 \equiv 1 \pmod{13}$) is accepted by `gen_r_check_ok` as $1 \in \mathbb{F}_{13}$ and refused by `dec_limbs`.

---

### Question 5: Completeness Across All Call and Chunk Regimes

**Verdict: SOUND & COMPLETE**

- In [lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L182-L213), the honest advice generator computes `av` in $[0, p)$ matching the BabyBear value of $a$ (`j_lo - 1`, `j_hi`, `j_lo - k_lo`, `j_hi - k_hi`):
  - When $\text{av} = 0$, it supplies $(w, z) = (0, 1)$, satisfying $0 \cdot 0 = 1 - 1$ and $0 \cdot 1 = 0$.
  - When $\text{av} \ne 0$, it supplies $(w, z) = (\text{av}^{-1} \bmod p, 0)$, satisfying $\text{av} \cdot \text{av}^{-1} = 1 - 0$ and $\text{av} \cdot 0 = 0$.
- Across all five honest regimes (verified in `q5_completeness_all_regimesTest` in [rreplay_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/qa/gemini-quint/e3-rreplay/rreplay_test.qnt#L145-L169)):
  1. **Ordinary call ($\text{mode} = 0, j = 0, k = 0$)**: $e_{\text{in}} = e_{\text{out}} = 4$, $\text{tag\_in} = \text{tag\_out} = 0$ (`SHK_PROGC`), entry 4 is `DIGEST(SHK_PROGC)`, entry 5 is masked out by $\text{mode} = 0$ and `cswap(0, y, o[0]) == y`.
  2. **First chunk ($\text{mode} = 1, j = 1 < k$)**: $z_{j1} = 1, z_{jk} = 0 \implies (\text{tag\_in}, \text{tag\_out}) = (\text{SHK\_PROGC}, \text{SHK\_LOCK})$, $e_{\text{in}} = 4, e_{\text{out}} = 5$.
  3. **Interior chunk ($\text{mode} = 1, 1 < j < k$)**: $z_{j1} = 0, z_{jk} = 0 \implies (\text{tag\_in}, \text{tag\_out}) = (\text{SHK\_LOCK}, \text{SHK\_LOCK})$, $e_{\text{in}} = 4, e_{\text{out}} = 5$.
  4. **Final chunk of multi-chunk call ($\text{mode} = 1, 1 < j = k$)**: $z_{j1} = 0, z_{jk} = 1 \implies (\text{tag\_in}, \text{tag\_out}) = (\text{SHK\_LOCK}, \text{SHK\_PROGC})$, $e_{\text{in}} = 4, e_{\text{out}} = 5$.
  5. **Single-chunk call ($\text{mode} = 1, j = k = 1$)**: $z_{j1} = 1, z_{jk} = 1 \implies (\text{tag\_in}, \text{tag\_out}) = (\text{SHK\_PROGC}, \text{SHK\_PROGC})$, $e_{\text{in}} = 4, e_{\text{out}} = 5$.

---

### Question 6: Missing Range and Order Checks on $j$ and $k$

**Verdict: UNSOUND (missing 16-bit limb range checks and $1 \le j \le k$ order check)**

Neither `gen_r_check` ([lverify_excerpt.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/lverify_excerpt.rs#L199-L217)) nor `decode` / `anchor_tags` ([iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/iface.rs#L510-L534)) constrains $j_{\text{lo}}, j_{\text{hi}}, k_{\text{lo}}, k_{\text{hi}} < 2^{16}$ or $1 \le j \le k$. Each word is only constrained by `InputAir` to lie in $[0, p)$.

#### Concrete Consequences and Counterexamples

1. **Invalid chunk index $j = 0$ when $\text{mode} = 1$**:
   - **Counterexample**: $\text{mode} = 1$, $(j_{\text{lo}}, j_{\text{hi}}) = (0, 0)$, $(k_{\text{lo}}, k_{\text{hi}}) = (1, 0)$ (or $(0, 0)$).
   - **Effect**: Both `gen_r_check` and `decode` accept! When $j = 0, k = 1$, the circuit assigns $(\text{tag\_in}, \text{tag\_out}) = (\text{SHK\_LOCK}, \text{SHK\_LOCK})$, treating chunk $0$ as an interior chunk of a 1-chunk call. Even worse, when $j = 0, k = 0$, the circuit assigns $(\text{tag\_in}, \text{tag\_out}) = (\text{SHK\_LOCK}, \text{SHK\_PROGC})$, allowing a 0-chunk call to terminate with `SHK_PROGC` at $a_{\text{out}}$ while starting from `SHK_LOCK` at $a_{\text{in}}$.

2. **Out-of-bounds chunk index $j > k$ when $\text{mode} = 1$**:
   - **Counterexample**: $\text{mode} = 1$, $(j_{\text{lo}}, j_{\text{hi}}) = (2, 0)$, $(k_{\text{lo}}, k_{\text{hi}}) = (1, 0)$ ($j = 2 > k = 1$).
   - **Effect**: Both `gen_r_check` and `decode` accept with $(\text{tag\_in}, \text{tag\_out}) = (\text{SHK\_LOCK}, \text{SHK\_LOCK})$, allowing chunks beyond the declared chunk count $k$.

3. **Non-16-bit limb $\ge 2^{16}$ aliasing $j = k$ between `gen_r_check` and `iface::anchor_tags` (`u32w`)**:
   - Look at `anchor_tags` in [iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/iface.rs#L510-L522):
     ```rust
     let u32w = |at: usize| head[at] + (head[at + 1] << 16);
     let (j, k) = (u32w(J_WORDS), u32w(K_WORDS));
     ```
     Whereas `gen_r_check` tests `[j = k]` limb-wise: $j_{\text{lo}} = k_{\text{lo}} \land j_{\text{hi}} = k_{\text{hi}}$.
   - **Counterexample (full scale)**: $\text{mode} = 1$, $(j_{\text{lo}}, j_{\text{hi}}) = (2^{16}, 0) = (65536, 0)$, $(k_{\text{lo}}, k_{\text{hi}}) = (0, 1)$.
     - In `anchor_tags` (`u32w`): $j = 65536 + (0 \ll 16) = 65536$ and $k = 0 + (1 \ll 16) = 65536$, so $j = k$ and `anchor_tags` computes $\text{tag\_out} = \text{SHK\_PROGC}$ ($0$).
     - In `gen_r_check`: $j_{\text{lo}} = 65536 \ne 0 = k_{\text{lo}}$ and $j_{\text{hi}} = 0 \ne 1 = k_{\text{hi}}$, so $z_{jk} = 0$ and `gen_r_check` forces $\text{tag\_out} = \text{SHK\_LOCK}$ ($1$)!
     - Any downstream verifier/composer that reconstructs $j$ and $k$ as $lo + 2^{16} hi$ believes $j = k$ (the final chunk), while the circuit actually proved $\text{tag\_out} = \text{SHK\_LOCK}$ ($j \ne k$).
   - **Counterexample (reduced scale in `q6_missing_jk_range_consequencesTest`)**: With `JK_BASE = 4`, $(j_{\text{lo}}, j_{\text{hi}}) = (4, 0)$ and $(k_{\text{lo}}, k_{\text{hi}}) = (0, 1)$ both give $lo + 4 \cdot hi = 4$, where `gen_r_check_ok` forces $\text{tag\_out} = \text{SHK\_LOCK}$ ($1$) while `native_anchor_tags_combined` expects $\text{tag\_out} = \text{SHK\_PROGC}$ ($0$).

4. **Non-16-bit limb $\ge 2^{16}$ aliasing $j = 1 \pmod{2^{32}}$ (or $j = k \pmod{2^{32}}$)**:
   - **Counterexample (full scale)**: $\text{mode} = 1$, $(j_{\text{lo}}, j_{\text{hi}}) = (1, 2^{16}) = (1, 65536)$.
   - **Effect**: Truncated to `u32`, $j = 1 + (65536 \ll 16) \equiv 1 \pmod{2^{32}}$, aliasing chunk $j = 1$, while `gen_r_check` sees $j_{\text{hi}} = 65536 \ne 0$ and forces $\text{tag\_in} = \text{SHK\_LOCK}$ ($1$) instead of $\text{SHK\_PROGC}$ ($0$).

#### Minimal Constraints to Close Each

1. **16-bit limb range checks on $j_{\text{lo}}, j_{\text{hi}}, k_{\text{lo}}, k_{\text{hi}}$**:
   - Send $j_{\text{lo}}, j_{\text{hi}}, k_{\text{lo}}, k_{\text{hi}}$ to the existing 16-bit range bus `RANGE_BUS` (`Range16Air` in [machine.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/machine.rs#L1292)), and check `head[at] < (1 << 16)` in `decode` / `anchor_tags`.
   - This makes $(lo, hi) \mapsto lo + 2^{16} hi \in [0, 2^{32})$ a bijection, closing all limb aliasing between $(lo, hi)$ and `u32`.
2. **Order constraints $1 \le j \le k$ when $\text{mode} = 1$ (and optionally $j = k = 0$ when $\text{mode} = 0$)**:
   - **$j \ge 1$ when $\text{mode} = 1$**: Since $j_{\text{lo}}, j_{\text{hi}} \in [0, 2^{16})$, $j = 0 \iff j_{\text{lo}} = 0 \land j_{\text{hi}} = 0$. Using $z_{1b} = \text{is\_zero}(j_{\text{hi}})$ (already computed on line 207) and $z_{j0} = \text{is\_zero}(j_{\text{lo}})$, assert $\text{mode} \cdot (z_{j0} \cdot z_{1b}) = 0$.
   - **$j \le k$ when $\text{mode} = 1$**: Supply two 16-bit range-checked advice limbs $d_{\text{lo}}, d_{\text{hi}} \in [0, 2^{16})$ (on `RANGE_BUS`) and a borrow bit $b \in \{0, 1\}$ such that when $\text{mode} = 1$:
     $$k_{\text{lo}} - j_{\text{lo}} + b \cdot 2^{16} = d_{\text{lo}} \quad\text{and}\quad k_{\text{hi}} - j_{\text{hi}} - b = d_{\text{hi}}.$$
     Then $k - j = d_{\text{lo}} + 2^{16} d_{\text{hi}} \in [0, 2^{32})$, which holds if and only if $j \le k$.
   - In [rreplay.qnt](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/qa/gemini-quint/e3-rreplay/rreplay.qnt#L449-L464), `jk_range_ok` and `inv_jk_range_strengthened` model this strengthened predicate and show that it eliminates all four counterexample classes above.

---

## Mutation Testing Analysis

Six mutants are modeled in [rreplay.qnt](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/qa/gemini-quint/e3-rreplay/rreplay.qnt#L470-L725) and tested in [rreplay_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/qa/gemini-quint/e3-rreplay/rreplay_test.qnt#L219-L315). Each modifies one constraint of `gen_r_check_ok` and is caught by a concrete witness that fails on the mutant while passing on the real constraints:

| Mutant Name | Modified Constraint | Violated Property | Concrete Witness Catching the Mutant (`rreplay_test.qnt`) |
| :--- | :--- | :--- | :--- |
| `mut_no_az` | Drops $a \cdot z = 0$ from `is_zero(a)`, keeping only $a \cdot w = 1 - z$ | `SOUND` | `mut1_no_az_caughtTest`: For $\text{mode} = 1, j = 2, k = 3$ ($a_1 = j_{\text{lo}} - 1 = 1 \ne 0$), dishonest advice $z_{1a} = (w = 0, z = 1)$ satisfies $1 \cdot 0 = 1 - 1$, forging $z_{j1} = 1$ and $\text{tag\_in} = \text{SHK\_PROGC}$ ($0$) instead of $\text{SHK\_LOCK}$ ($1$). Accepted by `mut_no_az`, rejected by `gen_r_check_ok` and `native_r_rule_holds`. |
| `mut_mode_not_bit` | Drops $\text{mode} \cdot (\text{mode} - 1) = 0$ and the boolean check on `mode` | `SOUND` | `mut2_mode_not_bit_caughtTest`: Dishonest prover sets $\text{mode} = 2, e_{\text{out}} = 6$ with $j = k = 1$ (so $1 - z_{j1} = 1 - z_{jk} = 0$, keeping $\text{tag\_in} = \text{tag\_out} = 0$). Accepted by `mut_mode_not_bit`, rejected by `gen_r_check_ok` and `native_r_rule_holds`. |
| `mut_tag_out_unmasked` | Leaves $\text{tag\_out} = 1 - z_{jk}$ and entry-5 op/param checks unmasked by `mode` | `COMPLETE` (and `SOUND` on `tag_out`) | `mut3_tag_out_unmasked_caughtTest`: Honest ordinary call ($\text{mode} = 0, j = 0, k = 0$) with `NOP` at entry 5 is accepted by `gen_r_check_ok` and `native_r_rule_holds`, but falsely rejected by `mut_tag_out_unmasked` (which forces entry 5 to be `DIGEST(1)`). |
| `mut_sponge_swapped` | Absorbs `(outs[1], outs[0])` in swapped order `perm([y1, y0, 3])` in `SPONGE(3)` | `SOUND` & `COMPLETE` | `mut4_sponge_swapped_caughtTest`: With $d = (1, 2, 3)$, run outputs are $(y_0, y_1, y_2) = (8, 1, 12) \in \mathbb{F}_{13}^3$. Honest `claim` is $\text{sponge\_u}(0, [8, 1, 12]) = 5$, whereas swapped absorption produces $11 \ne 5$. Witness with slot $9 = 11$ is accepted by `mut_sponge_swapped` and rejected by `gen_r_check_ok`. |
| `mut_off_shift` | Uses shifted offsets `[0, 2, 4, 6, 9, 11]` (treating `SPONGE(3)` as 3 values instead of 4) | `SOUND` & `COMPLETE` | `mut5_off_shift_caughtTest`: Witness placing entry 4's output at slot 10 ($\text{digest}(0, \text{claim}) = \text{digest}(0, 5) = 12$) and zeroing slot 11 is accepted by `mut_off_shift`, violates `native_r_rule_holds`, and is rejected by `gen_r_check_ok`. |
| `mut_no_cswap` | Omits the final `cswap(mode, y, o[0])` on entry 5 (asserting `y5 == y5`) | `SOUND` | `mut6_no_cswap_caughtTest`: In chunk mode ($\text{mode} = 1, j = 1, k = 2$), forging entry 5's digest output at value slot 13 to $(y_5 + 1) \bmod 13$ is accepted by `mut_no_cswap`, violates `native_r_rule_holds`, and is rejected by `gen_r_check_ok`. |

---

## Commands Run and Outcomes

In accordance with the review instructions, below is the exact log of commands executed and their outcomes:

1. `ls -la lane-src` -> **Succeeded (exit code 0)**: listed `fr.rs`, `iface.rs`, `lverify_excerpt.rs`, `machine.rs`, `poseidon.rs`, `tape.rs`.
2. `ls -la` -> **Succeeded (exit code 0)**: listed `.git`, `.lane-skills`, `lane-src`, `qa`.
3. `ls -la .lane-skills` -> **Succeeded (exit code 0)**: showed `.lane-skills/quint-lang` and `.lane-skills/quint-modeling` are symlinks pointing outside the workspace (`/home/hoskinson/Projects/workflowskills/skills/...`).
4. `cat .lane-skills/quint-modeling/SKILL.md .lane-skills/quint-lang/SKILL.md` and `head -n 200 .lane-skills/quint-lang/SKILL.md` -> **Refused by sandbox guard** (target paths resolve outside the workspace checkout).
5. `quint --version` and `quint typecheck qa/gemini-quint/e3-rreplay/rreplay.qnt` -> **Refused by sandbox guard** (`Tool execution was canceled`). Consequently, no `quint` CLI output was observed inside this sandbox session, and none is claimed.

To run typechecking, unit/mutant tests, and randomized invariant exploration outside the sandbox guard, run:

```bash
quint typecheck qa/gemini-quint/e3-rreplay/rreplay.qnt
quint typecheck qa/gemini-quint/e3-rreplay/rreplay_test.qnt
quint test qa/gemini-quint/e3-rreplay/rreplay_test.qnt
quint run qa/gemini-quint/e3-rreplay/rreplay.qnt --invariant=inv_sound_and_complete --max-samples=1000 --seed=0x42
quint run qa/gemini-quint/e3-rreplay/rreplay.qnt --invariant=inv_jk_range_strengthened --max-samples=1000 --seed=0x42
```

---

## What the Reduced Model Does Not Cover

1. **Field sizes and limb dimensions**: The model scales BabyBear ($p = 2^{31} - 2^{27} + 1$) down to $P = 17$, the BLS12-381 scalar field ($r \approx 2^{255}$) down to $R_{\text{ORDER}} = 13$, value limbs from $9 \times 30$-bit limbs to $2 \times 2$-bit limbs ($\text{LIMB\_BASE} = 4$), and $j, k$ limbs from $2 \times 16$-bit limbs to $2 \times 2$-bit limbs ($\text{JK\_BASE} = 4$). While $P = 17$ and $R_{\text{ORDER}} = 13 < 4^2 = 16$ preserve prime-field invertibility, non-canonical limb overflow ($\ge \text{LIMB\_BASE}$), and non-canonical integer overflow ($\ge R_{\text{ORDER}}$), arithmetic overflow boundaries specific to 30-bit/31-bit/255-bit widths are abstracted.
2. **Cryptographic Poseidon permutation rounds**: Instead of the 68-round width-3 Midnight Poseidon permutation over BLS12-381 $\mathbb{F}_r$ ([poseidon.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/poseidon.rs)), `perm` is modeled as a 1-round bijective permutation over $\mathbb{F}_{13}^3$ using the S-box $u \mapsto u^5 \bmod 13$ ($\gcd(5, 12) = 1$) and an invertible MDS matrix.
3. **Low-level AIR bus interactions**: `MEM_BUS`, `ProgramTableAir`, `Range16Air`, `InputAir`'s 4-cell canonical decomposition (`eval_canonical`), and `PublicAir` carrier selectors ([machine.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/machine.rs)) are abstracted into functional semantics of the tape operations (`kbb`, `bb_in`, `add`, `sub`, `mul`, `assert_eq`, `pack`, `fr_add`, `perm`, `cswap`).
4. **Discretionary log entries and onion commitment**: The model covers the reservation `R` (entries `0..5` and value slots `0..13`) enforced by `gen_r_check`, and does not model discretionary entries (`6..B`), the `PATH` sibling witness in the outer wrap, the lane map (`LANE_MAP`), or the BabyBear Poseidon2 log onion (`p2` / `onion` in [iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-rreplay/lane-src/iface.rs#L242-L284)).

---

## Lead verification (2026-11-13, quint 0.32.0, Rust backend)

The lane could not run commands (headless sandbox) and could not read the skills: `.lane-skills/` held symlinks to a path outside the checkout (a lead setup defect; fixed with real copies for round 2). The lead ran:

| Command | Outcome |
|---|---|
| `quint typecheck rreplay.qnt` | ok |
| `quint typecheck rreplay_test.qnt` | ok |
| `quint test rreplay_test.qnt --main=rreplay_test --seed=0x1 --max-samples=200` | 13 passing (q1 x2, q2, q3, q4, q5, q6, mut1 to mut6) |
| `quint run rreplay.qnt --main=rreplay --init=init --step=step --invariant=inv_sound_and_complete --max-samples=2000 --max-steps=3 --seed=0x42` | `[ok] No violation found` |
| `quint run ... --invariant=inv_jk_range_strengthened --max-samples=2000 --max-steps=3 --seed=0x42` | `[ok] No violation found` |
| `quint run ... --invariant=inv_jk_unchecked_expect_violation --max-samples=2000 --max-steps=3 --seed=0x42` (lead addition: the Q6 property on the old check) | `[violation] Found an issue`: `mode 1`, `j = (2, 1)`, `k = (2, 0)` (j = 6 > k = 2 at base 4) accepted by the f725b15 check, as Q6 says |

Mutants: each `mut*_caughtTest` asserts that the mutant predicate accepts the witness, the native rule refuses it, and `gen_r_check_ok` refuses it (mut3 is a completeness mutant: it refuses an honest ordinary call). All 6 pass, so all 6 mutants are caught.

Lead action on Q6: the j, k rule is built in openvm `e3-iface` 95ed47b (`gen_r_check`: 4 limbs through `LowBits(x, 16) == x`; for `mode` 1, `is_zero(j_lo + j_hi)` refused and an advice borrow bit with both differences 16-bit; `anchor_tags` refuses the same words). Round 2 of this lane reviews the built rule.
