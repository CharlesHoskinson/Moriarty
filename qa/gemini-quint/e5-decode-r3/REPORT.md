# Soundness Review Report (Round 3): E5 Wrap In-Circuit D-9 Decoder (`lane-src/decode.rs`)

## Executive Summary

We performed an independent mathematical soundness and completeness review of the Round 3 changes (`lane-src/decode.diff`) to the Sherlock co-processor E5 wrap in-circuit D-9 decoder (`decode::decode` in `lane-src/decode.rs`) against the stock native reference (`iface::decode` in `lane-src/iface.rs`) over the BLS12-381 scalar field $\mathbb{F}_r$ ($\lceil \log_2 r \rceil = 255$).

Round 3 replaces the partial native replays of Round 2 (`R` entries and lane-mapped entries) with **Step 7**: a unified tape of $P = 256$ permutation slots that natively replays **every** logged `PERM`, `H2`, `SPONGE`, and `DIGEST` entry (`PATH` is not replayed by stock), tied to the entry table by a log-derivative lookup over packed 40-bit entry tuples with fixed multiplicities $\text{perms}_i$ and reading logged values through the shared $(V, S)$ log-derivative lookup. This closes open item E3 J6 from Round 2.

**Overall Verdict: SOUND and COMPLETE** (up to the deliberate `pw[0] == sh.pre_hash` pin on the `L` verifying-key pre-hash word and the standard negligible Schwartz-Zippel soundness/completeness error $\le \frac{7660}{r} < 2^{-241}$ of the Fiat-Shamir log-derivative lookups).

| Question | Component | Verdict | Counterexample |
| :---: | :--- | :---: | :--- |
| **1** | Slot placement (fixed multiplicities $\text{perms}_i$, monotone $\text{act}$, $\sum \text{act} = \text{np}$, inactive $\text{pk}_t = 0$, $0 \le t - \text{poff} < \text{perms}$, packing injectivity) | **SOUND** | None |
| **2** | Sponge chaining ($\text{prev} = \text{out}_{t-1}$ when $\neg\text{first}$, capacity $n$ on block 0, second input gated by $\text{has1} = 1 - \text{last} \cdot \text{par}$, all $n \in 1..V-1$) | **SOUND** | None |
| **3** | `PERM`, `H2`, `DIGEST` inputs and output checks (`perm`, `sponge_u(0, [a, b])`, `sponge_u(4, [tag_fr(param), x])`) | **SOUND** | None |
| **4** | Reads (all gated indices $< \text{off}_n \le V$, no $S$-table collision, ungated reads multiplied by their gate everywhere) | **SOUND** | None |
| **5** | `PATH` and inactive slots (free of replay constraints and unable to alter any verdict or subsequent sponge state) | **SOUND** | None |
| **6** | Fiat-Shamir transcript order (every $\text{mv}, \text{ms}$, query value $v$, and slot tuple $\text{pk}_t$ observed before $\alpha, \beta$) | **SOUND** | None |
| **7** | Full output equivalence with stock `iface::decode` (satisfiable iff stock returns `Ok(s)`, with identical statement `s`) | **SOUND** | None |

---

## Detailed Answers and Mathematical Evidence

### Question 1: Slot Placement and Tuple Packing
**Verdict: SOUND**

1. **Active slot prefix $\text{act}_t = [t < \text{np}]$ (`lane-src/decode.rs:382-383, 547-557, 653-654`):**
   - Step 3 enforces $\text{cap}_p = P - \text{np} \in [0, 2^{17})$ (`decode.rs:382-383`). Since each entry $i \in \{0, \dots, B - 1\}$ has $\text{perms}_i \in [0, \max(V/2, \text{PATH\_MAX})] = [0, 128]$, over $\mathbb{Z}$ we have $0 \le \text{np} = \sum_{i=0}^{B-1} \text{perms}_i \le P = 256 \ll r$.
   - For each slot $t \in \{0, \dots, P - 1\}$, $\text{act}_t \in \{0, 1\}$ (`wit_bit`), $\text{act}_{t-1} \cdot \text{act}_t = \text{act}_t$ for $t \ge 1$ (`decode.rs:550-554`), and $\sum_{t=0}^{P-1} \text{act}_t \equiv \text{np} \pmod r$ (`decode.rs:653-654`).
   - Because $\text{act}_t = 1 \implies \text{act}_{t-1} = 1$, the bit vector $(\text{act}_0, \dots, \text{act}_{P-1})$ is non-increasing ($1, \dots, 1, 0, \dots, 0$), and since $P = 256 < r$, the sum over $\mathbb{F}_r$ equals the integer count of $1$s. Thus $\text{act}_t = [t < \text{np}]$ is uniquely forced for all $t \in \{0, \dots, P - 1\}$.
   - On every inactive slot ($t \ge \text{np}$, so $1 - \text{act}_t = 1$), the circuit enforces $(1 - \text{act}_t) \cdot \text{pk}_t = 0$ (`decode.rs:574-577`), forcing $\text{pk}_t = 0$.

2. **Injectivity of `tup` on claimable rows and witness tuples (`lane-src/decode.rs:188-191, 518-530, 560-573`):**
   - The packing map is:
     $$\text{tup}(b_0, b_1, b_2, b_3, b_4, \text{param}, o, \text{poff}, \text{prm}) = \sum_{k=0}^4 2^k b_k + 2^5 \text{param} + 2^{13} o + 2^{22} \text{poff} + 2^{31} \text{prm}.$$
   - On every slot $t$, the witness fields are constrained to $b_k \in \{0, 1\}$, $\text{param} \in [0, 2^8)$, and $o, \text{poff}, \text{prm} \in [0, 2^9)$ (`decode.rs:561-572`). Since the bit intervals $[0, 5)$, $[5, 13)$, $[13, 22)$, $[22, 31)$, and $[31, 40)$ are pairwise disjoint and $2^{40} < 2^{254} < r$, `tup` is a **bijection** from $\{0, 1\}^5 \times [0, 2^8) \times [0, 2^9)^3$ into $[0, 2^{40}) \subset \mathbb{F}_r$. In particular, for any slot $t$, $\text{pk}_t = 0 \iff b_0 = \dots = b_4 = \text{param}_t = o_t = \text{poff}_t = \text{prm}_t = 0$.
   - For any entry $i \in \{0, \dots, B - 1\}$ with $\text{perms}_i > 0$, $\text{op}_i \in \{\text{PERM}, \text{H2}, \text{SPONGE}, \text{PATH}, \text{DIGEST}\}$. By Step 3 (`decode.rs:122-130, 331-383`):
     - $b_0, \dots, b_4 \in \{0, 1\}$ (`c_perm`, `c_h2`, `c_sp`, `c_dig`, `par`);
     - $\text{param}_i \in \{0\} \cup [1, V - 1] \cup [1, 32] \cup [0, 5] = [0, 255] \subset [0, 2^8)$;
     - $o_i = \text{offs}[i] \le \text{off}_B \le V = 256 < 2^9$;
     - $\text{poff}_i = \text{poffs}[i] < \text{np} \le P = 256 < 2^9$;
     - $\text{perms}_i \le \text{np} \le P = 256 < 2^9$.
     Therefore, every entry $i$ with $\text{perms}_i > 0$ has its fields strictly inside $\{0, 1\}^5 \times [0, 2^8) \times [0, 2^9)^3$, so `tup` is **strictly injective** on all claimable ($\text{perms}_i > 0$) rows (moreover, distinct entries $i < j$ with $\text{perms}_i, \text{perms}_j > 0$ have $\text{poff}_i < \text{poff}_j$, so their $\text{row}_i \ne \text{row}_j$ are always distinct).

3. **Inability to claim a $\text{perms}_i = 0$ row (`PUB`, `NOP`) or an overflowing `PUB` row (`lane-src/decode.rs:724-742`):**
   - In the slot-to-entry log-derivative lookup (`decode.rs:724-742`):
     $$\sum_{t=0}^{P-1} \frac{\text{act}_t}{\alpha - \text{pk}_t} = \sum_{i=0}^{B-1} \frac{\text{perms}_i}{\alpha - \text{row}_i},$$
     where every denominator $\alpha - \text{pk}_t$ and $\alpha - \text{row}_i$ is inverted and checked nonzero (`inv1`).
   - For any `NOP` or `PUB` entry $i$, Step 3 sets $\text{perms}_i = 0$ (`decode.rs:367-370`, since $c_1 = c_2 = c_{I_{\text{DIGEST}}} = c_{I_{\text{SPONGE}}} = c_{I_{\text{PATH}}} = 0$). Consequently, the term $\text{perms}_i \cdot (\alpha - \text{row}_i)^{-1}$ is multiplied by $\text{perms}_i = 0$ in `r2` (`decode.rs:736-739`) and **vanishes identically**!
   - Even though a `PUB(param)` entry can have $\text{param} \in [256, 1025]$ (whose top 2 bits overlap the low 2 bits of $o_i$ in $\text{row}_i$), its multiplicity in `r2` is $0$ (and $\alpha \ne \text{row}_i$ holds w.h.p. $\ge 1 - B/r$ since $\text{row}_i$ is fixed by `pw` before $\alpha$ is sampled). Thus no active slot can ever claim a `NOP` or `PUB` row, nor can an overflowing `PUB` parameter collide with a claimed row on the RHS. (Independently, any active slot has $0 \le m \le \text{prm}_t - 1$, forcing $\text{prm}_t \ge 1$.)

4. **Unique placement at $t = \text{poff}_i + m$ (`lane-src/decode.rs:579-585`):**
   - By Schwartz-Zippel (over random challenge $\alpha$ sampled after all $\text{pk}_t$ are observed, with soundness loss $\le \frac{P + B}{r} = \frac{512}{r} < 2^{-245}$), the rational-function equality $l_2(\alpha) = r_2(\alpha)$ implies that the multiset of active slot tuples $\{\text{pk}_t : 0 \le t < \text{np}\}$ equals the multiset joining $\text{row}_i$ with multiplicity $\text{perms}_i$ over all entries $i$ with $\text{perms}_i > 0$ (since $\sum_{\text{perms}_i > 0} \text{perms}_i = \text{np} < r$).
   - By the injectivity of `tup`, every active slot $t \in \{0, \dots, \text{np} - 1\}$ has $(\text{bits}_t, \text{param}_t, o_t, \text{poff}_t, \text{prm}_t) = (\text{bits}_{i(t)}, \text{param}_{i(t)}, o_{i(t)}, \text{poff}_{i(t)}, \text{perms}_{i(t)})$ for a unique entry $i(t)$ with $\text{perms}_{i(t)} > 0$.
   - On every active slot $t \in [0, \text{np})$ ($\text{act}_t = 1$), the circuit enforces (`decode.rs:579-585`):
     $$d_1 = t - \text{poff}_t \in [0, 2^9), \quad d_2 = \text{prm}_t - 1 - (t - \text{poff}_t) \in [0, 2^9).$$
     Since $t, \text{poff}_t, \text{prm}_t \in [0, 2^9)$, over $\mathbb{Z}$ we have $t - \text{poff}_t \in (-2^9, 2^9)$ and $\text{prm}_t - 1 - (t - \text{poff}_t) \in (-2^{10}, 2^{10})$. A negative integer in $(-2^{10}, 0)$ wraps modulo $r$ to $[r - 2^{10}, r - 1] \gg 2^9$, failing `range(..., 2^9)`. Therefore, $d_1, d_2 \in [0, 2^9)$ holds **if and only if** over $\mathbb{Z}$:
     $$0 \le t - \text{poff}_{i(t)} \le \text{perms}_{i(t)} - 1 \iff \text{poff}_{i(t)} \le t < \text{poff}_{i(t)} + \text{perms}_{i(t)}.$$
   - Because the intervals $I_i = [\text{poff}_i, \text{poff}_i + \text{perms}_i)$ for entries with $\text{perms}_i > 0$ form a **disjoint partition** of $[0, \text{np})$, each $t \in [0, \text{np})$ belongs to a unique interval $I_i$. Hence slot $t$ is forced to hold $\text{row}_i$ for that unique entry $i$, with block index $m = t - \text{poff}_i \in \{0, \dots, \text{perms}_i - 1\}$. Every block $m$ of every entry $i$ with $\text{perms}_i > 0$ is replayed at **exactly one** slot, at position $t = \text{poff}_i + m$.

---

### Question 2: Sponge Chaining
**Verdict: SOUND**

Comparing `lane-src/decode.rs:358-366, 586-650` against `iface::sponge_u(0, xs)` (`lane-src/iface.rs:192-206`):

1. **Block count $\text{perms}$ and parity $\text{par}$ (`decode.rs:358-366`):**
   - For a `SPONGE(n)` entry ($n = \text{param} \in [1, V - 1] = [1, 255]$), Step 3 constrains $e \in \{0, 1\}$, $q \in [0, 2^{16})$, and $2q - n - e \equiv 0 \pmod r$. Since $2q - n - e \in [-(V + 1), 2^{17}) \subset (-r, r)$, this holds over $\mathbb{Z}$ ($2q = n + e$), which uniquely forces $e = n \bmod 2$ (`par`) and $q = \lceil n / 2 \rceil \ge 1$ (`perms`).
2. **Flags $\text{first}$, $\text{last}$, and $\text{has1}$ (`decode.rs:586-593`):**
   - By Question 1, `SPONGE(n)` occupies contiguous slots $t = \text{poff} + m$ for $m \in \{0, \dots, q - 1\}$.
   - `is_zero` forces $\text{first} = [m == 0] \in \{0, 1\}$, $\text{nf} = 1 - \text{first} = [m > 0]$, and $\text{last} = [q - 1 - m == 0] = [m == q - 1] \in \{0, 1\}$.
   - $\text{has1} = 1 - \text{last} \cdot \text{par} \in \{0, 1\}$ equals $0$ **if and only if** $m = q - 1$ and $n \equiv 1 \pmod 2$ (the final block of an odd-length sponge, where chunk $m$ has length $1$); for every other block ($m < q - 1$, or $m = q - 1$ when $n$ is even), chunk $m$ has length $2$ and $\text{has1} = 1$.
3. **Permutation inputs and carried state across blocks (`decode.rs:620-637, 650`):**
   - On a `SPONGE` slot ($c_{\text{sp}} = 1$, $c_{\text{perm}} = c_{\text{h2}} = c_{\text{dig}} = 0$), the circuit computes:
     $$\text{in}_0 = y_0 + (1 - \text{first}) \cdot \text{prev}_0, \quad \text{in}_1 = \text{has1} \cdot y_1 + (1 - \text{first}) \cdot \text{prev}_1, \quad \text{in}_2 = \text{first} \cdot n + (1 - \text{first}) \cdot \text{prev}_2,$$
     where $y_0 = \text{vs}[o + 2m]$ (since $g_0 = 1$) and, whenever $\text{has1} = 1$, $y_1 = \text{vs}[o + 2m + 1]$ (since $g_1 = \text{sp}_1 = 1$).
   - **Block $m = 0$ ($\text{first} = 1, 1 - \text{first} = 0$):**
     $\text{prev}$ is multiplied by $1 - \text{first} = 0$ in all three coordinates, so any state left over from slot $\text{poff} - 1$ is completely cleared. The input is:
     $$(\text{in}_0, \text{in}_1, \text{in}_2) = (\text{vs}[o], \text{has1} \cdot \text{vs}[o + 1], n).$$
     Since $u = 0$ has $\text{cap\_fixed}(0, n) = n + 0 \cdot 2^{66} = n$, this matches the initial state $[0, 0, n]$ of `sponge_u(0, xs)` after absorbing block $0$ ($\text{xs}[0]$ and, if $n > 1$, $\text{xs}[1]$), and $\text{out}^{(0)} = \text{perm}(\text{in}_0, \text{in}_1, \text{in}_2)$.
   - **Blocks $m = 1, \dots, q - 1$ ($\text{first} = 0, 1 - \text{first} = 1$):**
     Because the slots $\text{poff}, \text{poff} + 1, \dots, \text{poff} + q - 1$ are contiguous, $\text{prev}$ at slot $\text{poff} + m$ is $\text{out}^{(m-1)}$, the permutation output of block $m - 1$. The input is:
     $$(\text{in}_0, \text{in}_1, \text{in}_2) = (\text{out}^{(m-1)}_0 + \text{vs}[o + 2m], \; \text{out}^{(m-1)}_1 + \text{has1} \cdot \text{vs}[o + 2m + 1], \; \text{out}^{(m-1)}_2),$$
     which matches the additive absorption of chunk $m$ in `sponge_u(0, xs)` (`iface.rs:198-204`).
4. **Output check (`decode.rs:593, 599, 605-606, 649`):**
   - For $m < q - 1$, $\text{last} = 0 \implies \text{spl} = c_{\text{sp}} \cdot \text{last} = 0$, so no output equality is asserted on intermediate blocks.
   - On the final block $m = q - 1$, $\text{last} = 1 \implies \text{spl} = 1$, $g_3 = \text{spl} = 1$, and $i_3 = o + n$, so $y_3 = \text{vs}[o + n]$. The check `chk(a, &spl, &out[0], &y[3])` enforces $\text{out}^{(q-1)}_0 = \text{vs}[o + n]$, which is identical to `sponge_u(0, &v[..param]) == v[param]` (`iface.rs:658`) for all $n \in \{1, \dots, V - 1\}$ (both odd and even).

---

### Question 3: `PERM`, `H2`, and `DIGEST`
**Verdict: SOUND**

For each of `PERM`, `H2`, and `DIGEST`, $\text{perms}_i = 1$, so by Question 1 the entry occupies a single active slot $t = \text{poff}_i$ with $m = t - \text{poff}_i = 0$ ($\text{first} = 1, \text{nf} = 0, \text{rem} = 0, \text{last} = 1$):

1. **`PERM` ($c_{\text{perm}} = 1, c_{\text{h2}} = c_{\text{sp}} = c_{\text{dig}} = 0$, $\text{param} = 0$):**
   - Read gates: $g_0 = g_1 = g_2 = g_3 = g_4 = g_5 = 1$, with indices $i_0 = o, i_1 = o + 1, i_2 = o + 2, i_3 = o + 3, i_4 = o + 4, i_5 = o + 5$, forcing $y_k = \text{vs}[o + k]$ for $k \in \{0, \dots, 5\}$.
   - Permutation input (`decode.rs:620-636`): $\text{in} = (y_0, y_1, y_2) = (\text{vs}[o], \text{vs}[o + 1], \text{vs}[o + 2])$.
   - Output check (`decode.rs:644-646`): enforces $\text{out}_k = y_{3 + k} = \text{vs}[o + 3 + k]$ for $k \in \{0, 1, 2\}$, while `chk` for $c_{\text{h2}}, c_{\text{dig}}, \text{spl}$ are gated off by $0$. Matches `perm([v[0], v[1], v[2]]) == [v[3], v[4], v[5]]` (`iface.rs:656`).
2. **`H2` ($c_{\text{h2}} = 1, c_{\text{perm}} = c_{\text{sp}} = c_{\text{dig}} = 0$, $\text{param} = 0$):**
   - Read gates: $g_0 = g_1 = g_2 = 1$ and $g_3 = g_4 = g_5 = 0$, with indices $i_0 = o, i_1 = o + 1, i_2 = o + 2$, forcing $y_0 = \text{vs}[o], y_1 = \text{vs}[o + 1], y_2 = \text{vs}[o + 2]$.
   - Permutation input: $\text{in} = (y_0, y_1, \text{cap}_{0,2}) = (\text{vs}[o], \text{vs}[o + 1], \text{cap\_fixed}(0, 2))$.
   - Output check (`decode.rs:647`): enforces $\text{out}_0 = y_2 = \text{vs}[o + 2]$. Since `h2(a, b) = sponge_u(0, [a, b]) = perm([a, b, cap_fixed(0, 2)])[0]`, this matches `h2(v[0], v[1]) == v[2]` (`iface.rs:657`).
3. **`DIGEST` ($c_{\text{dig}} = 1, c_{\text{perm}} = c_{\text{h2}} = c_{\text{sp}} = 0$, $\text{param} \in \{0, \dots, 5\}$):**
   - Read gates: $g_0 = g_1 = 1$ and $g_2 = g_3 = g_4 = g_5 = 0$, with indices $i_0 = o, i_1 = o + 1$, forcing $y_0 = \text{vs}[o]$ and $y_1 = \text{vs}[o + 1]$.
   - Tag selection (`decode.rs:616-619`): $\text{pd} = c_{\text{dig}} \cdot \text{param} = \text{param} \in \{0, \dots, 5\}$. `one_hot(a, 6, &pd, c_dig)` forces $\text{tb}_k = [k == \text{param}]$ for $k \in \{0, \dots, 5\}$, giving $\text{tag} = \sum_{k=0}^5 \text{tag\_fr}(k) \cdot \text{tb}_k = \text{tag\_fr}(\text{param})$ (when $c_{\text{dig}} = 0$, $\text{pd} = 0$ and gate $c_{\text{dig}} = 0$ forces $\text{tb} = \vec{0}$ and $\text{tag} = 0$).
   - Permutation input: $\text{in} = (\text{tag}, y_0, \text{cap}_{4,2}) = (\text{tag\_fr}(\text{param}), \text{vs}[o], \text{cap\_fixed}(4, 2))$.
   - Output check (`decode.rs:648`): enforces $\text{out}_0 = y_1 = \text{vs}[o + 1]$. Since `digest(param, x) = sponge_u(4, [tag_fr(param), x]) = perm([tag_fr(param), x, cap_fixed(4, 2)])[0]`, this matches `digest(param, v[0]) == v[1]` (`iface.rs:659`).
