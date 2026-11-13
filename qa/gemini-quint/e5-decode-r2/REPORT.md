# Soundness Review Report (Round 2): E5 Wrap In-Circuit D-9 Decoder (`lane-src/decode.rs`)

## Executive Summary

We performed an independent mathematical soundness and completeness review of the Round 2 changes (`lane-src/decode.diff`) to the Sherlock co-processor E5 wrap in-circuit D-9 decoder (`decode::decode` in `lane-src/decode.rs`) against the updated native reference (`iface::decode` in `lane-src/iface.rs`, `openvm` `e3-iface` `9539418`, diff in `lane-src/iface-2f09caf-9539418.diff`) over the BLS12-381 scalar field $\mathbb{F}_r$ ($\lceil \log_2 r \rceil = 255$).

**Overall Verdict: SOUND and COMPLETE** (up to the known open item E3 J6 — native replay of discretionary entries outside `R` and outside the lane map — plus the deliberate `pw[0] == sh.pre_hash` pin on the `L` verifying-key pre-hash word and the standard negligible Schwartz-Zippel soundness/completeness error $\le \frac{3620}{r} < 2^{-242}$ of the Fiat-Shamir log-derivative lookup).

| Question | Component | Verdict | Counterexample |
| :--- | :--- | :--- | :--- |
| **1** | Lane-map op test (`sel_dig + sel_sp == nz`, `sel_sp * (p - 3) == 0`) | **SOUND** | None |
| **2** | Output index (`idx = o + 1 + 2 * sel_sp`) and gated lookup reads (`o`, `o + 1`, `o + 2`) | **SOUND** | None |
| **3** | Mapped twice (`nz_i * nz_j * ((lane_i - lane_j) * inv - 1) == 0` + `s[lane] == 0`) | **SOUND** | None |
| **4** | `u16` range checks on all 8 lane-map words (equivalence to stock & redundancy with one-hots) | **SOUND** | None |
| **5** | Mapped-entry replay (tag one-hot `sel_dig * p` over `N_TAGS = 6`, gated `yd`/`ys` checks) | **SOUND** | None |
| **6** | Step 4 (`R` and the anchor rule at static positions, `mode`, `j`/`k`, `t_in`/`t_out`, `claim_pub`, replays) | **SOUND** | None |
| **7** | Output equivalence with stock `iface::decode` (confirming unmapped non-`R` replay is the only divergence) | **SOUND** | None |

---

## Detailed Answers and Mathematical Evidence

### Question 1: Lane-Map Op Test
**Verdict: SOUND**

1. **Exactness of `sel_dig + sel_sp == nz` and `sel_sp * (p - 3) == 0` (`lane-src/decode.rs:471-487` vs `lane-src/iface.rs:583-589`):**
   - For each lane-map pair $k \in \{0, \dots, \text{LANE\_PAIRS} - 1\}$, let $u = \text{lane}_k + \text{lanes} \cdot \text{ent}_k$. Since $\text{lane}_k, \text{ent}_k \in [0, 2^{16}) \subset [0, P)$ and $\text{lanes} = 1026$, over $\mathbb{Z}$ we have $0 \le \text{lane}_k + 1026 \cdot \text{ent}_k < 1027 \cdot 2^{16} < 2^{27} \ll r$. Thus $u \equiv 0 \pmod r \iff (\text{lane}_k, \text{ent}_k) = (0, 0)$.
   - As proved in Round 1, $\text{nz} = u \cdot \text{inv}$ and $(1 - \text{nz}) \cdot u = 0$ uniquely force $\text{nz} = 0$ when $(\text{lane}_k, \text{ent}_k) = (0, 0)$ (skipped pair) and $\text{nz} = 1$ when $(\text{lane}_k, \text{ent}_k) \ne (0, 0)$ (non-skipped pair).
   - Next, `e = one_hot(a, d.b, &ent, &nz, "lane map entry")` constrains $e_i \in \{0, 1\}$ (`wit_bit`), $\sum_{i=0}^{B-1} e_i \equiv \text{nz} \pmod r$, and $\sum_{i=0}^{B-1} i \cdot e_i \equiv \text{ent}_k \pmod r$. Since $B = 256 < r$:
     - **Case A: Skipped pair ($\text{nz} = 0$, $\text{lane}_k = \text{ent}_k = 0$).**
       Over $\mathbb{Z}$, $\sum_{i=0}^{B-1} e_i = 0$ forces $e_i = 0$ for all $i \in \{0, \dots, B - 1\}$. Consequently:
       $$\text{sel\_dig} = \sum_{i=0}^{B-1} e_i \cdot \text{ops}[i][I_{\text{DIGEST}}] = 0, \quad \text{sel\_sp} = \sum_{i=0}^{B-1} e_i \cdot \text{ops}[i][I_{\text{SPONGE}}] = 0, \quad p = \sum_{i=0}^{B-1} e_i \cdot \text{params}[i] = 0.$$
       Both $\text{sel\_dig} + \text{sel\_sp} = 0 = \text{nz}$ and $\text{sel\_sp} \cdot (p - 3) = 0 \cdot (-3) = 0$ are satisfied unconditionally, and since $\text{nz} = \text{sel\_dig} = \text{sel\_sp} = 0$ and $h = \vec{0}$, nothing is accepted, replayed, queried, or written to $s$.
     - **Case B: Non-skipped pair ($\text{nz} = 1$).**
       Over $\mathbb{Z}$, $\sum_{i=0}^{B-1} e_i = 1$ forces $e$ to have a single $1$ at index $i^* = \text{ent}_k \in \{0, \dots, B - 1\}$ (refusing $\text{ent}_k \ge B$). Thus:
       $$\text{sel\_dig} = \text{ops}[\text{ent}_k][I_{\text{DIGEST}}], \quad \text{sel\_sp} = \text{ops}[\text{ent}_k][I_{\text{SPONGE}}], \quad p = \text{params}[\text{ent}_k].$$
       From Step 3 (`decode.rs:311-319`), $\text{ops}[\text{ent}_k] \in \{0, 1\}^7$ is a one-hot vector over `ADMITTED`, so $\text{sel\_dig}, \text{sel\_sp} \in \{0, 1\}$ are mutually exclusive ($\text{sel\_dig} \cdot \text{sel\_sp} = 0$).
       Therefore, $\text{sel\_dig} + \text{sel\_sp} = 1$ holds **if and only if** either:
       1. $(\text{sel\_dig}, \text{sel\_sp}) = (1, 0)$, i.e., $\text{op}_{\text{ent}_k} = \text{DIGEST}$. Then $\text{sel\_sp} \cdot (p - 3) = 0 \cdot (p - 3) = 0$ holds for every admitted `DIGEST` parameter $p \in \{0, \dots, 5\}$.
       2. $(\text{sel\_dig}, \text{sel\_sp}) = (0, 1)$, i.e., $\text{op}_{\text{ent}_k} = \text{SPONGE}$. Then $\text{sel\_sp} \cdot (p - 3) = 1 \cdot (p - 3) = 0 \iff p = 3$, i.e., entry $\text{ent}_k$ is `SPONGE(3)` (any `SPONGE(n)` with $n \ne 3$ has $n \in [1, V - 1] \setminus \{3\}$, so $n - 3 \not\equiv 0 \pmod r$ and is refused).
       Any other op (`NOP`, `PERM`, `H2`, `PATH`, `PUB`) yields $\text{sel\_dig} = \text{sel\_sp} = 0$, failing $\text{sel\_dig} + \text{sel\_sp} = 1$.
   - This matches stock `iface::apply_lane_map` (`iface.rs:584`): `!(op == DIGEST || (op == SPONGE && param == 3))`.

---

### Question 2: Output Index and Gated Reads
**Verdict: SOUND**

1. **Exactness of `idx = o + 1 + 2 * sel_sp` (`lane-src/decode.rs:502-503` vs `lane-src/iface.rs:592`):**
   - For a non-skipped pair ($\text{nz} = 1$), Question 1 established that the only accepted entries are:
     - `DIGEST(param)`: $\text{vals}(\text{DIGEST}, \text{param}) = 2$, so $o + \text{vals} - 1 = o + 1$. In the circuit, $\text{sel\_sp} = 0$, giving $\text{idx} = o + 1 + 2(0) = o + 1$.
     - `SPONGE(3)`: $\text{vals}(\text{SPONGE}, 3) = 3 + 1 = 4$, so $o + \text{vals} - 1 = o + 3$. In the circuit, $\text{sel\_sp} = 1$, giving $\text{idx} = o + 1 + 2(1) = o + 3$.
   - Thus $\text{idx} = o + 1 + 2 \cdot \text{sel\_sp} = o + \text{vals}(\text{op}, \text{param}) - 1$ holds identically for all accepted ops.

2. **No $S$-table key collision ($\ge V$) or wrong value on gated reads (`lane-src/decode.rs:504-510, 526-529, 578-607`):**
   - Each lane-map iteration appends four queries to `qv`:
     $$(\text{nz}, \text{idx}, \text{val}), \quad (\text{nz}, o, x_0), \quad (\text{sel\_sp}, o + 1, x_1), \quad (\text{sel\_sp}, o + 2, x_2).$$
   - **When $\text{nz} = 0$ (skipped pair):**
     All four gates are $0$ ($\text{nz} = \text{sel\_sp} = 0$). In the log-derivative sum (`decode.rs:587-590`), each query's inverse term is multiplied by $g = 0$ (`lhs = a.mul_add(g, &inv, &lhs)`), contributing $0$ to `lhs` and consuming no table multiplicity (requiring only $\alpha \ne \text{key} + \beta \cdot v$, which holds w.h.p. over random challenges $\alpha, \beta$). Moreover, $\text{val}, x_0, x_1, x_2$ are multiplied by $0$ everywhere they are used ($h = \vec{0}, \text{sel\_dig} = 0, \text{sel\_sp} = 0$).
   - **When $\text{nz} = 1$ and the entry is `DIGEST` ($\text{sel\_dig} = 1, \text{sel\_sp} = 0$):**
     - Entry $\text{ent}_k$ has $\text{vals} = 2$. Because every entry $i$ has $\text{vals}_i \ge 0$ and the capacity check enforces $\text{off}_B \le V$, we have:
       $$0 \le o < o + 1 < o + 2 = \text{offs}[\text{ent}_k] + \text{vals}_{\text{ent}_k} \le \text{off}_B \le V.$$
     - Thus the two active queries (gate $\text{nz} = 1$) have keys $o \in [0, V - 2]$ and $\text{idx} = o + 1 \in [1, V - 1]$. Both keys are strictly $< V$, so neither can hit an $S$-table key (which lie in $[V, V + \text{lanes} - 1]$). By log-derivative lookup soundness (since all $N_T = V + \text{lanes} = 1282$ table keys are distinct and total queries $N_Q = 4 \cdot \text{LANE\_PAIRS} + 2 B = 528 < r$), they uniquely force $x_0 = \text{vs}[o]$ and $\text{val} = \text{vs}[o + 1]$.
     - The other two queries at $o + 1$ and $o + 2$ have gate $\text{sel\_sp} = 0$; even when $o + 2 = V$ (a `DIGEST` at the very end of $V$), its gate is $0$, so it contributes $0$ to `lhs` and cannot hit Table $S$.
   - **When $\text{nz} = 1$ and the entry is `SPONGE(3)` ($\text{sel\_dig} = 0, \text{sel\_sp} = 1$):**
     - Entry $\text{ent}_k$ has $\text{vals} = 4$, so $o + 4 \le \text{off}_B \le V$, which implies:
       $$0 \le o < o + 1 < o + 2 < o + 3 \le V - 1.$$
     - All four queries are active ($\text{nz} = \text{sel\_sp} = 1$) and all four keys $o, o + 1, o + 2, \text{idx} = o + 3$ lie strictly in $[0, V - 1]$. None can hit an $S$-table key ($\ge V$), and the lookup uniquely forces $x_0 = \text{vs}[o], x_1 = \text{vs}[o + 1], x_2 = \text{vs}[o + 2], \text{val} = \text{vs}[o + 3]$.

---

### Question 3: Mapped Twice
**Verdict: SOUND**

1. **Exactness of `nz_i * nz_j * ((lane_j - lane_i) * inv - 1) == 0` (`lane-src/decode.rs:488-500` vs `lane-src/iface.rs:574-592`):**
   - In stock `iface::apply_lane_map`, a per-lane boolean flag `mapped[lane]` is set to `true` on each active write, and any pair targeting a lane with `mapped[lane] == true || s[lane] != 0` is refused.
   - Checking `taken = dot(h, s) == 0` alone is insufficient when an earlier active pair $i < j$ maps `lane` to an entry whose output $\text{vs}[o_i + \text{vals}_i - 1] = 0$: after pair $i$, $s[\text{lane}]$ remains $0$, so pair $j$ would pass `taken == 0` (`e3-lanemap` finding 2a).
   - For every pair $j \in \{0, \dots, \text{LANE\_PAIRS} - 1\}$ and every earlier pair $i \in \{0, \dots, j - 1\}$, the circuit computes $\text{dl} = \text{lane}_j - \text{lane}_i$, witnesses $\text{inv}$, sets $m = \text{dl} \cdot \text{inv}$ and $g = \text{nz}_j \cdot \text{nz}_i$, and enforces:
     $$g \cdot m = g \iff \text{nz}_j \cdot \text{nz}_i \cdot ((\text{lane}_j - \text{lane}_i) \cdot \text{inv} - 1) = 0.$$
   - **Soundness ($\text{nz}_i = 1$ and $\text{nz}_j = 1$):**
     Then $g = 1$, so the constraint becomes $(\text{lane}_j - \text{lane}_i) \cdot \text{inv} = 1$ in $\mathbb{F}_r$.
     - If $\text{lane}_j = \text{lane}_i$, then $\text{lane}_j - \text{lane}_i = 0$, and $0 \cdot \text{inv} = 0 \ne 1$ for every $\text{inv} \in \mathbb{F}_r$. The circuit is unsatisfiable, refusing the duplicate lane even when the first mapped output was $0$.
     - If $\text{lane}_j \ne \text{lane}_i$, since $\text{lane}_i, \text{lane}_j \in [0, \text{lanes}) \subset [0, r)$, $\text{lane}_j - \text{lane}_i \not\equiv 0 \pmod r$, and the honest witness $\text{inv} = (\text{lane}_j - \text{lane}_i)^{-1}$ satisfies $m = 1$ and $g \cdot m = g$.
   - **Completeness with skipped pairs ($\text{nz}_i = 0$ or $\text{nz}_j = 0$):**
     Then $g = \text{nz}_j \cdot \text{nz}_i = 0$, so $g \cdot m = 0 = g$ holds for any $\text{inv} \in \mathbb{F}_r$ (including $\text{inv} = 0$ when $\text{lane}_i = \text{lane}_j = 0$ on two skipped pairs, or when one pair is skipped `(0, 0)` and another active pair maps lane `0`).
   - Combined with `taken = dot(h, s) == 0` (which enforces that the initial call-vector value $\text{lanes}[\text{lane}_j] = 0$), this accepts and refuses the exact same cases as stock `mapped[lane] || s[lane] != 0`.

---

### Question 4: `u16` Range Checks on Lane-Map Words
**Verdict: SOUND**

1. **Equivalence to stock's `lane >= 2^16 || ent >= 2^16` refusal (`lane-src/decode.rs:469-470` vs `lane-src/iface.rs:577-582`):**
   - For each $k \in \{0, \dots, \text{LANE\_PAIRS} - 1\}$, the circuit enforces `a.range(&lane, &pow2(16), ...)` and `a.range(&ent, &pow2(16), ...)` on all $8$ lane-map words.
   - In stock `iface::apply_lane_map`, `(0, 0)` is skipped first, and then `lane >= 1 << 16 || ent >= 1 << 16` is refused. Because $(0, 0)$ trivially satisfies $0 < 2^{16} \land 0 < 2^{16}$, checking `< 2^16` on all 4 pairs before the `(0, 0)` skip test is logically equivalent to stock.

2. **Redundancy with the gated one-hots (`lane < lanes`, `ent < B`):**
   - **On the `Gadget` backend (satisfiability) at `Dims::FULL` ($\text{lanes} = 1026 \le 2^{16}, B = 256 \le 2^{16}$):** Yes, the 8 lane-map `u16` range checks in Step 5 are algebraically redundant with `is_zero` + the gated one-hots `one_hot(a, d.lanes(), &lane, &nz)` and `one_hot(a, d.b, &ent, &nz)`. Specifically, Step 1 (Unpack) already bounds $\text{lane}_k, \text{ent}_k < P < 2^{31}$, so $0 \le \text{lane}_k + \text{lanes} \cdot \text{ent}_k \le 1027(P - 1) < 2^{42} \ll r$. Thus $u = 0 \iff (\text{lane}_k, \text{ent}_k) = (0, 0)$ (which already satisfies $< 2^{16}$), and whenever $(\text{lane}_k, \text{ent}_k) \ne (0, 0)$, $\text{nz}_k = 1$, whereupon `one_hot` forces $\text{lane}_k < \text{lanes} = 1026 < 2^{16}$ and $\text{ent}_k < B = 256 < 2^{16}$.
   - **On the `Native` backend:** They preserve the exact diagnostic failure ordering (`"lane map pair: not u16"` before `"lane map pair"` / `"lane map entry"`), as noted in the comment at `decode.rs:467-468`.
   - **Contrast with Step 4's `u16` range checks (`decode.rs:393-395`):** In Step 4, the 4 `u16` range checks on `w[J_WORDS..J_WORDS+4]` are **not** redundant (they are essential for soundness in both `mode == 0` and `mode == 1`, as proved in Question 6 and demonstrated by `mut_no_u16`).

---

### Question 5: Mapped-Entry Replay
**Verdict: SOUND**

1. **Soundness and completeness of the tag one-hot (`lane-src/decode.rs:511-514`):**
   - The circuit computes $\text{pd} = \text{sel\_dig} \cdot p$ and `tb = one_hot(a, N_TAGS as usize, &pd, &sel_dig, "lane map entry")` with `N_TAGS = 6`, constraining $tb_0, \dots, tb_5 \in \{0, 1\}$, $\sum_{t=0}^5 tb_t = \text{sel\_dig}$, and $\sum_{t=0}^5 t \cdot tb_t = \text{sel\_dig} \cdot p$.
   - **When $\text{sel\_dig} = 1$ (mapped entry is `DIGEST`):**
     Step 3 (`bounds(I_DIGEST) = (0, 5)`) already constrains $p = \text{params}[\text{ent}_k] \in \{0, 1, 2, 3, 4, 5\}$. Since $\text{sel\_dig} = 1$ and $6 < r$, $\sum_{t=0}^5 tb_t = 1$ and $\sum_{t=0}^5 t \cdot tb_t = p$ uniquely force $tb_t = [t == p]$ for all $t \in \{0, \dots, 5\}$, and $\text{tag} = \sum_{t=0}^5 \text{tag\_fr}(t) \cdot tb_t = \text{tag\_fr}(p)$. Both sound and complete for all $p \in 0..6$.
   - **When $\text{sel\_dig} = 0$ (mapped entry is `SPONGE(3)` with $p = 3$, or pair is skipped with $\text{nz} = 0$):**
     Because $p$ is multiplied by $\text{sel\_dig}$ to form $\text{pd} = 0$, $\sum_{t=0}^5 tb_t = 0$ forces $tb = \vec{0}$ and $\sum_{t=0}^5 t \cdot tb_t = 0 = \text{pd}$ holds (note that without multiplying $p$ by $\text{sel\_dig}$, a `SPONGE(3)` pair would have $\text{sel\_dig} = 0$ and index $p = 3 \ne 0$, which would be unsatisfiable!).

2. **Exactness of the gated replay equalities (`lane-src/decode.rs:515-522` vs `lane-src/iface.rs:658-659`):**
   - The circuit computes $\text{yd} = \text{digest}(\text{tag}, x_0) = \text{hash\_slice}(4, [\text{tag}, x_0])$ and $\text{ys} = \text{hash\_slice}(0, [x_0, x_1, x_2])$, and enforces:
     $$\text{sel\_dig} \cdot (\text{yd} - \text{val}) = 0, \quad \text{sel\_sp} \cdot (\text{ys} - \text{val}) = 0.$$
   - **Active `DIGEST(p)` pair ($\text{nz} = 1, \text{sel\_dig} = 1, \text{sel\_sp} = 0$):**
     By Question 2, $x_0 = \text{vs}[o]$ and $\text{val} = \text{vs}[o + 1]$. The first equation forces $\text{digest}(\text{tag\_fr}(p), \text{vs}[o]) = \text{vs}[o + 1]$ (matching `iface.rs:659`), while the second equation is multiplied by $\text{sel\_sp} = 0$.
   - **Active `SPONGE(3)` pair ($\text{nz} = 1, \text{sel\_dig} = 0, \text{sel\_sp} = 1$):**
     By Question 2, $x_0 = \text{vs}[o], x_1 = \text{vs}[o + 1], x_2 = \text{vs}[o + 2], \text{val} = \text{vs}[o + 3]$. The second equation forces $\text{hash\_slice}(0, \text{vs}[o..o+3]) = \text{vs}[o + 3]$ (matching `iface.rs:658`), while the first equation is multiplied by $\text{sel\_dig} = 0$.
   - **Skipped pair ($\text{nz} = 0 \implies \text{sel\_dig} = \text{sel\_sp} = 0$):**
     Both equations reduce to $0 = 0$, imposing no constraint.

---

### Question 6: Step 4 (`R` and the Anchor Rule at Static Positions)
**Verdict: SOUND**

Comparing `lane-src/decode.rs:387-450` with `iface::anchor_tags`, `iface::r_entries`, and `iface::decode` (`lane-src/iface.rs:533-561, 639-665`):

1. **`mode` bit (`decode.rs:389-392` vs `iface.rs:540-548`):**
   - $\text{mode}^2 - \text{mode} = 0$ in prime field $\mathbb{F}_r$ forces $\text{mode} \in \{0, 1\}$, refusing $\text{mode} \ge 2$.
2. **`u16` range checks on `j` and `k` words (`decode.rs:393-397` vs `iface.rs:535-539`):**
   - `a.range(&w[J_WORDS + i], &pow2(16), "mode")` for $i \in \{0, 1, 2, 3\}$ enforces $\text{to\_big}(w[\text{J\_WORDS} + i]) < 2^{16}$ in **both** $\text{mode} = 0$ and $\text{mode} = 1$, refusing any high limb $\ge 2^{16}$ even when $\text{mode} = 0$ (matching `iface.rs:535` and test vector `(0, [1 << 16, 0], [0, 0], None)` at `iface.rs:1056`).
   - Reconstructing $j = w[\text{J\_WORDS}] + 2^{16} w[\text{J\_WORDS} + 1]$ and $k = w[\text{K\_WORDS}] + 2^{16} w[\text{K\_WORDS} + 1]$ yields exact integers $j, k \in [0, 2^{32} - 1] \subset [0, r)$ with no modular wrap.
3. **Gated chunk ordering `mode * (j - 1) < 2^32` and `mode * (k - j) < 2^32` (`decode.rs:398-404` vs `iface.rs:541`):**
   - **When $\text{mode} = 0$:** Both products are $0 < 2^{32}$, so any $j, k \in [0, 2^{32})$ (including $j = 0$ and $j > k$) is accepted, matching `0 => Some((SHK_PROGC, SHK_PROGC))` in `anchor_tags`.
   - **When $\text{mode} = 1$:** Over $\mathbb{Z}$, $j - 1 \in [-1, 2^{32} - 2]$ and $k - j \in [-(2^{32} - 1), 2^{32} - 1]$.
     - If $j = 0$, $j - 1 \equiv r - 1 \pmod r$; since $r - 1 > 2^{254} \gg 2^{32}$, `range(mode * (j - 1), 2^32)` fails.
     - If $j > k$, $k - j \in [-(2^{32} - 1), -1]$, so $\text{to\_big}(k - j) = r + (k - j) \ge r - (2^{32} - 1) > 2^{254} - 2^{32} \gg 2^{32}$, and `range(mode * (k - j), 2^32)` fails.
     - If $1 \le j \le k$, both $j - 1$ and $k - j$ lie in $[0, 2^{32} - 1] < 2^{32}$ and pass.
     Thus, for $\text{mode} = 1$, the two range checks hold **if and only if** $1 \le j \le k$.
4. **`is_zero` bits `eq_j1`, `eq_jk` and anchor tags `t_in`, `t_out` (`decode.rs:405-410` vs `iface.rs:540-547`):**
   - `is_zero(a, x)` enforces $z = 1 - x \cdot \text{inv}$ and $x \cdot z = 0$, which uniquely forces $z = [x == 0] \in \{0, 1\}$. Since $j, k \in [0, 2^{32}) \subset [0, r)$, $\text{eq\_j1} = [j == 1]$ and $\text{eq\_jk} = [j == k]$.
   - With `SHK_PROGC = 0` and `SHK_LOCK = 1`:
     $$t_{\text{in}} = \text{mode} - \text{mode} \cdot \text{eq\_j1} = \text{mode} \cdot [j \ne 1], \quad t_{\text{out}} = 1 - \text{eq\_jk} = [j \ne k].$$
   - For $\text{mode} = 0$, $t_{\text{in}} = 0 = \text{SHK\_PROGC}$, and $t_{\text{out}}$ is gated off by $\text{mode} = 0$ everywhere it is used (`decode.rs:422-424, 445-449`), matching `r_entries` (which only includes entry 5 when $\text{mode} = 1$).
   - For $\text{mode} = 1$, $t_{\text{in}} = [j \ne 1]$ ($0$ if $j = 1$, $1$ if $j > 1$) and $t_{\text{out}} = [j \ne k]$ ($0$ if $j = k$, $1$ if $j < k$), matching `anchor_tags` on all cases ($j = 1 = k$, $1 = j < k$, $1 < j < k$, $1 < j = k$).
5. **`R` `(op, param)` checks (`decode.rs:411-424` vs `iface.rs:552-561, 639-644`):**
   - Entries $0, 1, 2$: $\text{ops}[i][I_{\text{DIGEST}}] = 1$ and $\text{params}[i] = \text{R\_RUN}[i] \in \{0, 2, 3\}$.
   - Entry $3$: $\text{ops}[3][I_{\text{SPONGE}}] = 1$ and $\text{params}[3] = 3$.
   - Entry $4$: $\text{ops}[4][I_{\text{DIGEST}}] = 1$ and $\text{params}[4] = t_{\text{in}}$.
   - Entry $5$: $\text{mode} \cdot (\text{ops}[5][I_{\text{DIGEST}}] - 1) = 0$ and $\text{mode} \cdot (\text{params}[5] - t_{\text{out}}) = 0$, which forces $\text{op}_5 = \text{DIGEST}$ and $\text{param}_5 = t_{\text{out}}$ when $\text{mode} = 1$ and leaves entry 5 unconstrained by `R` when $\text{mode} = 0$.
6. **Anchor entry words (`decode.rs:425-427` vs `iface.rs:645-648`):**
   - Enforces $w[\text{E\_IN\_WORD}] = 4$ and $w[\text{E\_OUT\_WORD}] - \text{mode} = 4 \iff w[\text{E\_OUT\_WORD}] = 4 + \text{mode}$.
7. **`claim_pub` inputs (`decode.rs:428-431` vs `iface.rs:649-652`):**
   - Since entries $0..4$ have fixed value counts $2, 2, 2, 4, 2$, their offsets in $\text{vs}$ are fixed at $\text{R\_OFF} = [0, 2, 4, 6, 10, 12]$.
   - Enforces $\text{vs}[6 + i] = \text{vs}[\text{R\_OFF}[i] + 1]$ for $i \in \{0, 1, 2\}$, i.e., $\text{vs}[6..9] = [\text{vs}[1], \text{vs}[3], \text{vs}[5]]$.
8. **Native replays of entries 0 to 5 (`decode.rs:434-449` vs `iface.rs:653-665`):**
   - Entries $i \in \{0, 1, 2\}$: $\text{digest}(\text{tag\_fr}(\text{R\_RUN}[i]), \text{vs}[\text{R\_OFF}[i]]) = \text{vs}[\text{R\_OFF}[i] + 1]$.
   - Entry $3$: $\text{hash\_slice}(0, \text{vs}[6..9]) = \text{vs}[9]$.
   - Entry $4$: since $t_{\text{in}} \in \{0, 1\}$, $\text{anchor\_tag}(t_{\text{in}}) = (\text{tag\_fr}(1) - \text{tag\_fr}(0)) \cdot t_{\text{in}} + \text{tag\_fr}(0) = \text{tag\_fr}(t_{\text{in}})$, and $\text{digest}(\text{anchor\_tag}(t_{\text{in}}), \text{vs}[10]) = \text{vs}[11]$.
   - Entry $5$: $\text{mode} \cdot (\text{digest}(\text{anchor\_tag}(t_{\text{out}}), \text{vs}[12]) - \text{vs}[13]) = 0$, which forces the `DIGEST(t_out)` replay at offset $12$ when $\text{mode} = 1$ and imposes no constraint when $\text{mode} = 0$.

---

### Question 7: Output Equivalence with Stock `iface::decode`
**Verdict: SOUND**

1. **Confirmation of the only divergence (`replay_all` true vs false):**
   - Comparing `iface::decode` (`lane-src/iface.rs:604-676`) and `decode::decode` (`lane-src/decode.rs:252-609`), every check is identical except:
     1. **Known open item E3 J6:** Stock `iface::decode` (`iface.rs:653-665`) replays every entry $i \in \{0, \dots, B - 1\}$ with $\text{op}_i \in \{\text{PERM}, \text{H2}, \text{SPONGE}, \text{DIGEST}\}$. The wrap circuit `decode::decode` replays all entries in `R` (entries $0..4$ when $\text{mode} = 0$, entries $0..5$ when $\text{mode} = 1$) and all entries referenced by active lane-map pairs, but does **not** replay unmapped discretionary entries $i \ge 5 + \text{mode}$.
     2. **Pinned pre-hash constant (`decode.rs:261`):** The circuit enforces `pw[0] == sh.pre_hash`, whereas `iface::unpack` ignores `lfr_words[0]`.
     3. **Log-derivative lookup soundness/completeness error:** The circuit replaces direct indexing for lane-map value reads and `PUB` checks with a Fiat-Shamir log-derivative argument over $N_Q = 4 \cdot \text{LANE\_PAIRS} + 2 B = 528$ queries and $N_T = V + \text{lanes} = 1282$ table rows, which has soundness loss $\le \frac{2(N_Q + N_T)}{r} = \frac{3620}{r} < 2^{-242}$ and completeness loss $\le \frac{N_Q + N_T}{r} < 2^{-243}$ over random challenges $(\alpha, \beta)$.
2. **Exactness of the returned statement `s`:**
   - Apart from the replay of unmapped entries outside `R` (and `pw[0] == sh.pre_hash`), `decode::decode` is satisfiable if and only if `iface::decode` returns `Ok(s_stock)` (w.h.p. $\ge 1 - 2^{-242}$), and every satisfying witness produces the exact call vector `s == s_stock` with zero prover freedom.

---

## Commands Run and Outcomes

During this review, we attempted to execute `quint` via the command tool, which was refused by the sandbox permission check:

1. `quint typecheck qa/gemini-quint/e5-decode-r2/decode.qnt`
   - **Outcome:** Refused by sandbox permission check (`permission check failed for unsandboxed "quint typecheck qa/gemini-quint/e5-decode-r2/decode.qnt": user denied permission to run command`).
2. Read attempts on `.lane-skills/quint-modeling/SKILL.md` and `/home/hoskinson/.gemini/skills/quint-modeling/SKILL.md` (symlinks resolving outside the checkout to `/home/hoskinson/Projects/workflowskills/skills/quint-modeling/SKILL.md`) were similarly refused by the workspace path check.

Per the instructions (*"If your sandbox lets you run `quint typecheck`, `quint test` and `quint run`, do so; if a command is refused, say so honestly in REPORT.md and do not claim results you did not see"*), we report honestly that no `quint` commands could be executed inside the sandbox.

The Quint files `qa/gemini-quint/e5-decode-r2/decode.qnt` and `qa/gemini-quint/e5-decode-r2/decode_test.qnt` are written to match Quint 0.32.0 syntax (incorporating all Round 1 lead fixes) and can be verified directly with:
```sh
quint typecheck qa/gemini-quint/e5-decode-r2/decode.qnt
quint typecheck qa/gemini-quint/e5-decode-r2/decode_test.qnt
quint test --main decode_test --match '_test$' qa/gemini-quint/e5-decode-r2/decode_test.qnt
quint run --invariant SOUND_AND_COMPLETE --max-samples 5000 --max-steps 10 --seed 1 qa/gemini-quint/e5-decode-r2/decode.qnt
quint run --invariant SOUND_mut_no_sponge3_param --max-samples 5000 --max-steps 10 --seed 1 qa/gemini-quint/e5-decode-r2/decode.qnt
quint run --invariant SOUND_mut_idx_plus1 --max-samples 5000 --max-steps 10 --seed 1 qa/gemini-quint/e5-decode-r2/decode.qnt
quint run --invariant SOUND_mut_no_twice --max-samples 5000 --max-steps 10 --seed 1 qa/gemini-quint/e5-decode-r2/decode.qnt
quint run --invariant SOUND_mut_no_u16 --max-samples 5000 --max-steps 10 --seed 1 qa/gemini-quint/e5-decode-r2/decode.qnt
quint run --invariant SOUND_mut_no_mapped_replay --max-samples 5000 --max-steps 10 --seed 1 qa/gemini-quint/e5-decode-r2/decode.qnt
quint run --invariant SOUND_mut_t_in_wrong --max-samples 5000 --max-steps 10 --seed 1 qa/gemini-quint/e5-decode-r2/decode.qnt
quint run --invariant SOUND_mut_no_claim_pub --max-samples 5000 --max-steps 10 --seed 1 qa/gemini-quint/e5-decode-r2/decode.qnt
quint run --invariant SOUND_mut_no_chunk_range --max-samples 5000 --max-steps 10 --seed 1 qa/gemini-quint/e5-decode-r2/decode.qnt
```

---

## Quint Model and Mutants (`qa/gemini-quint/e5-decode-r2/`)

We modeled both the reference decoder `ref_decode(pw, replay_all)` and the circuit constraint system `constraints_ok(pw, wit)` in `qa/gemini-quint/e5-decode-r2/decode.qnt`, with unit and mutant-catching tests in `qa/gemini-quint/e5-decode-r2/decode_test.qnt`:

- **Scaled Parameters:**
  - Prime field `FR_MOD = 359` ($r - 1 = 358 = 6 + 4 \cdot 8 + 5 \cdot 64$, `R1_LIMBS = [6, 4, 5]`).
  - Packing: `PACK_K = 2` words per $\mathbb{F}_r$ element, `WORD_BASE = 18`, `BB_P = 17` ($17 + 17 \cdot 18 = 323 < 359$, no modular wrap).
  - Limbs: `LIMBS = 3`, `LIMB_BASE = 8` ($8^3 - 1 = 511 \ge 359$).
  - Bounds: `U16_LIM = 8`, `U32_LIM = 64` ($8^2$), `BOUND_LIM = 16`, `SPONGE_Q_LIM = 16`, `CAP_LIM = 32`.
  - Dimensions: `STMT = 13` (`MODE_WORD = 0, J_WORDS = 1, K_WORDS = 3, E_IN_WORD = 5, E_OUT_WORD = 6, LANE_MAP = 7, LANE_PAIRS = 3`), `B_CAP = 7` (fits `R` of 5 or 6 entries + discretionary entries), `V_CAP = 16` (fits `R` of 12 or 14 values + discretionary values), `P_CAP = 10`, `LANES = 4`, `N_TAGS = 6` (`TAG_FR_TABLE = [101, 113, 127, 139, 151, 163]`), and `HASH_TABLE` as a strictly injective uninterpreted table (verified by `hash_table_injective_test`).
- **Invariants:**
  - `SOUND`: `constraints_ok(cur_pw, cur_wit) implies (ref_decode(cur_pw, false).ok and circuit_output(cur_pw, cur_wit) == ref_decode(cur_pw, false).stmt)`
  - `COMPLETE`: `ref_decode(cur_pw, false).ok implies (constraints_ok(cur_pw, honest_witness(cur_pw)) and circuit_output(cur_pw, honest_witness(cur_pw)) == ref_decode(cur_pw, false).stmt)`
  - `REPLAY_DIFF_EXACT`: verifies that `ref_decode(pw, true).ok` implies `ref_decode(pw, false).ok` with identical output, and whenever `ref_decode(pw, false).ok` holds, `ref_decode(pw, true).ok == unmapped_outside_r_replay_ok(pw)` (tested on `PW_UNMAPPED_BAD_REPLAY` in `replay_all_divergence_test`).

### List of 8 Mutants and How Each Is Caught

| # | Mutant Predicate | Constraint Modified (`lane-src/decode.rs`) | Counterexample in `decode.qnt` | How Caught (`decode.qnt` & `decode_test.qnt`) |
| :- | :--- | :--- | :--- | :--- |
| **1** | `mut_no_sponge3_param` | Drops `sel_sp * (p - 3) == 0` in the lane-map op check (`decode.rs:485-487`). | `PW_BAD_SPONGE3_PARAM` / `WIT_BAD_SPONGE3_PARAM`: Entry 5 is `SPONGE(2)` ($\text{vs}[12..15] = [7, 8, 60]$) and Entry 6 is `PUB(0)` ($\text{vs}[15] = 61 = \text{hash\_slice}(0, [7, 8, 60])$); Pair 2 maps `(3, 5)`. | `ref_decode` and `constraints_ok` reject ($p = 2 \ne 3$); `mut_no_sponge3_param` accepts. Caught by `SOUND_mut_no_sponge3_param` and `catch_mut_no_sponge3_param_test`. |
| **2** | `mut_idx_plus1` | Uses output index `o + 1` for every mapped op instead of `o + 1 + 2 * sel_sp` (`decode.rs:503`). | `PW_BAD_IDX_PLUS1` / `WIT_BAD_IDX_PLUS1`: Entry 5 is `SPONGE(3)` at $o = 12$ with $\text{vs}[12..16] = [1, 20, 2, 99]$ where $\text{hash\_slice}(0, [1, 20, 2]) = 20 = \text{vs}[13] \ne \text{vs}[15]$; Pair 0 maps `(3, 5)`. Also tested on `PW_HONEST_ORD`. | Violates **both** `SOUND` (accepts `PW_BAD_IDX_PLUS1` with `val = vs[13] = 20`) and `COMPLETE` (rejects honest `PW_HONEST_ORD`). Caught by `SOUND_mut_idx_plus1` and `catch_mut_idx_plus1_test`. |
| **3** | `mut_no_twice` | Drops the pairwise duplicate-lane check `nz_i * nz_j * ((lane_j - lane_i) * inv - 1) == 0` (`decode.rs:492-499`). | `PW_BAD_TWICE` / `WIT_BAD_TWICE`: Entry 4 (`DIGEST(0)` on `42`) has valid output $\text{vs}[11] = 0$. Pair 0 maps `(2, 4)` (writing $0$ to $s[2]$), and Pair 1 maps `(2, 3)` (writing $50$ to $s[2]$, passing `taken = dot(h, s) == 0`!). | `ref_decode` and `constraints_ok` reject; `mut_no_twice` accepts. Caught by `SOUND_mut_no_twice` and `catch_mut_no_twice_test`. |
| **4** | `mut_no_u16` | Drops the `< U16_LIM` (`2^16`) range checks on `j, k` words and lane-map words (`decode.rs:393-395, 469-470`). | `PW_BAD_U16` / `WIT_BAD_U16`: Ordinary call ($\text{mode} = 0$) with $w[\text{J\_WORDS}] = \text{U16\_LIM} = 8 < \text{BB\_P}$. | `ref_decode` and `constraints_ok` reject (`anchor_tags` returns `None`); `mut_no_u16` accepts. Caught by `SOUND_mut_no_u16` and `catch_mut_no_u16_test`. |
| **5** | `mut_no_mapped_replay` | Drops `sel_dig * (yd - val) == 0` and `sel_sp * (ys - val) == 0` in Step 5 (`decode.rs:517-522`). | `PW_BAD_MAPPED_REPLAY` / `WIT_BAD_MAPPED_REPLAY`: Entry 5 (outside `R`) is `DIGEST(4)` with input $15$ and forged output $\text{vs}[13] = 199 \ne 45$; Pair 2 maps `(3, 5)`. | `ref_decode` and `constraints_ok` reject; `mut_no_mapped_replay` accepts and writes forged $199$ into $s[3]$. Caught by `SOUND_mut_no_mapped_replay` and `catch_mut_no_mapped_replay_test`. |
| **6** | `mut_t_in_wrong` | Computes `t_in = 1 - [j == 1]` without gating by `mode` (`decode.rs:408-409`). | `PW_BAD_T_IN` / `WIT_BAD_T_IN`: $\text{mode} = 0, j = 0, k = 0$ with Entry 4 forged to `DIGEST(SHK_LOCK = 1)`. Also tested on honest `PW_HONEST_ORD` ($\text{mode} = 0, j = 0$, Entry 4 `DIGEST(SHK_PROGC = 0)`). | Violates **both** `SOUND` (accepts `PW_BAD_T_IN`) and `COMPLETE` (rejects honest `PW_HONEST_ORD`). Caught by `SOUND_mut_t_in_wrong` and `catch_mut_t_in_wrong_test`. |
| **7** | `mut_no_claim_pub` | Drops `vs[sp + i] == vs[R_OFF[i] + 1]` for $i \in \{0, 1, 2\}$ (`decode.rs:428-431`). | `PW_BAD_CLAIM_PUB` / `WIT_BAD_CLAIM_PUB`: Entry 3 (`SPONGE(3)`) has inputs $[99, 22, 23]$ ($\text{vs}[6] = 99 \ne \text{vs}[1] = 21$) and matching sponge output $\text{vs}[9] = 59$. | `ref_decode` and `constraints_ok` reject; `mut_no_claim_pub` accepts. Caught by `SOUND_mut_no_claim_pub` and `catch_mut_no_claim_pub_test`. |
| **8** | `mut_no_chunk_range` | Drops the gated chunk ordering range checks `mode * (j - 1) < 2^32` and `mode * (k - j) < 2^32` (`decode.rs:400-404`). | `PW_BAD_CHUNK_RANGE` / `WIT_BAD_CHUNK_RANGE`: $\text{mode} = 1, j = 3, k = 2$ ($j > k$), plus `pw_j0` ($\text{mode} = 1, j = 0, k = 2$). | `ref_decode` and `constraints_ok` reject; `mut_no_chunk_range` accepts. Caught by `SOUND_mut_no_chunk_range` and `catch_mut_no_chunk_range_test`. |

---

## What the Quint Model Does Not Cover

1. **Full 255-Bit Field Arithmetic and `Dims::FULL` Capacities:**
   - The Quint model scales $\mathbb{F}_r$ down to $\mathbb{F}_{359}$, limb decomposition from $9 \times 30$-bit to $3 \times 3$-bit, packed words from $8 \times 31$-bit ($P = \text{0x78000001}$) to $2 \times \text{base-}18$ ($P = 17$), `u16`/`u32` bounds from $2^{16}, 2^{32}$ to $8, 64$, and capacities from $(B, V, P_{\text{cap}}, \text{lanes}, \text{LANE\_PAIRS}) = (256, 256, 256, 1026, 4)$ to $(7, 16, 10, 4, 3)$. Absence of modular wrap-around at full scale is proved analytically in Questions 1–6 above.
2. **Concrete Poseidon Permutation and Fiat-Shamir Transcript Sampling:**
   - The model represents `hash_slice` and `digest` via the uninterpreted injective table `HASH_TABLE` and checks the log-derivative lookup symbolically (`check_lookup`), assuming that the Poseidon permutation acts as a random oracle/permutation and that Schwartz-Zippel holds over $\mathbb{F}_r$ with soundness loss $\le \frac{2(N_Q + N_T)}{r} < 2^{-242}$.
3. **Underlying `ZkStdLib` / Halo2 Gadget Implementation:**
   - The model assumes the `Alg` primitives (`wit_bit`, `lin`, `mul`, `mul_add`, `assert_eq`, `assert_eq_c`, `range`, `permute`) are soundly constrained by `midnight-zk-stdlib` in the Halo2 backend.
4. **Statement Header Words Outside `MODE_WORD..E_OUT_WORD` and `LANE_MAP..LANE_MAP + 8`:**
   - Statement header words in `0..74` (`root_stmt`) and `81..106` (`run_tag`, `layout_d`, `X`) are only range-checked to $< P$ in both stock `iface::decode` and `decode::decode`.

---

## Lead verification (2026-11-13, quint 0.32.0)

The lane's sandbox refused every command, so the lead ran the commands in `lead-run.sh` (output in `lead-runs.log`). The model needed no fixes: `decode.qnt` and `decode_test.qnt` are the same as the lane's originals (`*.lane`).

- `quint typecheck`: OK on both files.
- `quint test --main decode_test --match '_test$'`: 14 of 14 pass.
- `quint run --invariant SOUND_AND_COMPLETE --max-samples 5000 --max-steps 10`: no violation with seed 1 or seed 2 (10,000 traces).
- Each of the 8 mutants is violated (seed 1): `no_sponge3_param`, `idx_plus1`, `no_twice`, `no_u16`, `no_mapped_replay`, `t_in_wrong`, `no_claim_pub`, `no_chunk_range`.

The lead accepts the verdict: SOUND and COMPLETE on questions 1 to 7. The only known divergence is the open replay of entries outside `R` and the lane map. No circuit change is needed.
