# Soundness & Completeness Review: E3 J8 Log Onion AIR (`OnionAir` in `lane-src/iface_l.rs`)

## Executive Summary

| Question | Topic | Verdict |
| :--- | :--- | :--- |
| **Q1** | Row grammar (`E`, `FA`/`A`/`LAST`, `B0`..`B2`, inactive rows, unique `FIN` on `E` or `B2`) | **SOUND** |
| **Q2** | Permutation absorption (`total = 3 + 9k`, `U` prefix mask, `POS`, `pos + c = total`, capacities) | **SOUND** |
| **Q3** | Value & entry bus binding (`VAL_BUS`, `ENT_BUS`, `LOG_BUS`, `OFF` chain, ordering & uniqueness) | **SOUND** |
| **Q4** | Digest & count chaining (`d` from `E`/`B2`, `h` from last `A`, `TIE_BUS` tie `(log_d, log_n)`, empty log) | **SOUND** |
| **Q5** | Completeness (`OnionAir::trace`, inactive padding rows, height bound) | **COMPLETE** |
| **Q6** | Other attack vectors & BabyBear field wrap-around ($p = 2^{31} - 2^{27} + 1$) | **SOUND** |

---

## Detailed Analysis by Question

### 1. Row Grammar — **SOUND**

**Claim:** The selector constraints in `OnionAir::eval` ([lane-src/iface_l.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L547-L580)) force the trace of height $H = \text{height}()$ to consist of a non-empty active prefix of the exact form
$$E \;\cdot\; \big( \underbrace{\text{FA} \cdot A_{\text{mid}}^*}_{\ge 0\text{ non-LAST}} \cdot A_{\text{LAST}} \cdot B_0 \cdot B_1 \cdot B_2 \big)^n \;\cdot\; \text{inactive}^{H - n_{\text{act}}}$$
(where a 1-block entry has a single $A$ row with both `O_FA = 1` and `O_LAST = 1`, though for all admitted operations $k \ge 1 \implies \text{total} = 3 + 9k \ge 12 > 8 \implies \ge 2$ $A$-blocks), with `O_FIN = 1` on **exclusively** the last active row $r = n_{\text{act}} - 1$, which must be either `E` ($n = 0$) or `B2` ($n \ge 1$).

**Proof & Evidence:**
1. **Booleanity & Mutual Exclusion ([iface_l.rs:547-558](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L547-L558)):**
   - Every selector $c \in \{\text{O\_E}, \text{O\_A}, \text{O\_FA}, \text{O\_B0}, \text{O\_B1}, \text{O\_B2}, \text{O\_LAST}, \text{O\_FIN}\}$ and every mask bit $\text{O\_U}[0..8]$ is constrained by `assert_bool`.
   - `ax = x[O_E] + x[O_A] + x[O_B0] + x[O_B1] + x[O_B2]` is constrained by `assert_bool(ax)`. Since the sum of five $\{0,1\}$ values in $\mathbb{F}_p$ ($p = 2013265921 > 5$) cannot wrap around modulo $p$, at most one of $\{\text{O\_E}, \text{O\_A}, \text{O\_B0}, \text{O\_B1}, \text{O\_B2}\}$ is $1$ on any row.
   - `x[O_FA] * (1 - x[O_A]) = 0` forces $\text{O\_FA} = 1 \implies \text{O\_A} = 1$ (so $\text{O\_FA} = 0$ on all non-$A$ and inactive rows).
2. **Unique `E` Row at Row 0 ([iface_l.rs:560-562](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L560-L562)):**
   - `when_first_row().assert_one(x[O_E])` and `when_transition().assert_zero(y[O_E])` force row $0$ to have $\text{O\_E} = 1$ ($\text{ax} = 1$) and every row $r \in \{1, \dots, H-1\}$ to have $\text{O\_E} = 0$.
3. **Contiguous Active Prefix & Unique `FIN` Row ([iface_l.rs:574-580](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L574-L580)):**
   - Transition constraint `(1 - ax) * ay = 0` forces $\text{ax} = 0 \implies \text{ay} = 0$. Since row $0$ has $\text{ax} = 1$, active rows form a contiguous prefix $0 .. n_{\text{act}} - 1$ ($1 \le n_{\text{act}} \le H$), followed by inactive rows $n_{\text{act}} .. H - 1$.
   - On rows $0 .. H-2$, `x[O_FIN] - ax * (1 - ay) = 0` forces $\text{O\_FIN} = 1$ if and only if $\text{ax} = 1 \land \text{ay} = 0$. On row $H - 1$, `when_last_row().assert_zero(x[O_FIN] - ax)` forces $\text{O\_FIN} = \text{ax}$.
   - In both cases ($n_{\text{act}} < H$ and $n_{\text{act}} = H$), row $r = n_{\text{act}} - 1$ is the **unique** row in the entire trace with $\text{O\_FIN} = 1$.
   - Line 579 (`x[O_FIN] * (x[O_A] + x[O_B0] + x[O_B1]) = 0`) forbids $\text{O\_FIN} = 1$ on any `A`, `B0`, or `B1` row. Because $\text{O\_FIN} = 1 \implies \text{ax} = 1$, the unique `FIN` row must have $\text{O\_E} = 1$ or $\text{O\_B2} = 1$. In particular, no `A`, `B0`, or `B1` row can transition to an inactive row ($\text{ay} = 0$ would force $\text{O\_FIN} = 1$) or sit at the last trace row $H - 1$ ($\text{ax} = 1$ at $H - 1$ forces $\text{O\_FIN} = 1$).
4. **Deterministic Active Transitions ([iface_l.rs:564-573](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L564-L573)):**
   - From `E` or `B2`: `(x[O_E] + x[O_B2]) * ay * (1 - y[O_FA]) = 0`. Either $\text{ay} = 0$ (so $x$ is the `FIN` row) or $\text{ay} = 1$ and $y$ starts a new entry with $\text{y[O\_FA]} = 1$ (and hence $\text{y[O\_A]} = 1$).
   - From `A` with $\text{O\_LAST} = 0$: `x[O_A] * (1 - x[O_LAST]) * (1 - y[O_A]) = 0` and `x[O_A] * (1 - x[O_LAST]) * y[O_FA] = 0` force the next row to be a continuation `A` row ($\text{O\_A} = 1, \text{O\_FA} = 0$).
   - From `A` with $\text{O\_LAST} = 1$: `x[O_A] * x[O_LAST] * (1 - y[O_B0]) = 0` forces the next row to be `B0`.
   - From `B0`: `x[O_B0] * (1 - y[O_B1]) = 0` forces the next row to be `B1`.
   - From `B1`: `x[O_B1] * (1 - y[O_B2]) = 0` forces the next row to be `B2`.
   - Thus a prover **cannot** insert, drop, or reorder block types, nor terminate on an `A`, `B0`, or `B1` row.

---

### 2. Absorption — **SOUND**

**Claim:** On every active row, the 16 permutation input words `x[O_W .. O_W + 16]` (`O_W[0..8]` rate, `O_CAP[0..8]` capacity) equal the native `p2_with_inputs` state ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface.rs#L242-L259)), and `x[O_OUT .. O_OUT + 16]` equals $P(\text{input})$.

**Proof & Evidence:**
1. **Permutation Lookup ([iface_l.rs:601-608](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L601-L608)):**
   - Every active row looks up `(input: x[O_W..O_W+16], output: x[O_OUT..O_OUT+16])` on `Poseidon2PermuteBus` with multiplicity $\text{ax} = 1$. Since `O_CAP = O_W + 8`, `x[O_W..O_W+16]` is `O_W[0..8] ++ O_CAP[0..8]`.
2. **Initial & Chained Capacities ([iface_l.rs:582-599](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L582-L599)):**
   - On `E`: `O_CAP[0] = 1` (`1 + |[]|`), `O_CAP[1..8] = 0`, `O_W[0] = 1 = TAG_EMPTY`, `O_W[1..8] = 0`.
   - On `FA`: `O_CAP[0] = total = 3 + 9 * O_K` (`1 + |enc(e)|`), `O_CAP[1..8] = 0`.
   - On `B0`: `O_CAP[0] = 17` (`1 + |d ++ h|`), `O_CAP[1..8] = 0`.
   - On continuation blocks (`cont_y = y[O_A] - y[O_FA] + y[O_B1] + y[O_B2] = 1`, which holds on and only on continuation `A` rows, `B1`, and `B2`): `y[O_CAP + m] = x[O_OUT + 8 + m]` for $m \in 0..8$. By Q1, the predecessor $x$ is always the immediately preceding block of the same sponge call (`A` before continuation `A`, `B0` before `B1`, `B1` before `B2`).
3. **`A`-Block Words, `U` Pattern, `POS`, and `pos + c = total` ([iface_l.rs:610-635](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L610-L635)):**
   - Entry fields `O_OP, O_PARAM, O_OFF, O_K` are carried unchanged across all rows of an entry via `same = ay * (1 - y[O_FA])` ([iface_l.rs:646-650](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L646-L650)).
   - On `FA`: `O_W[0] = TAG_ENT (4)`, `O_W[1] = O_OP`, `O_W[2] = O_PARAM`, and `O_POS = 0`.
   - On each continuation `A` row (`y[O_A] - y[O_FA] = 1`), `y[O_POS] = x[O_POS] + 8`, so the $b_i$-th `A` row of an entry ($b_i \ge 0$) has $\text{O\_POS} = 8 b_i$.
   - On every `A` row: `O_U[0] = 1` (line 619) and `O_U[j+1] * (1 - O_U[j]) = 0` for $j \in 0..7$ (line 625), forcing `O_U[0..8]` to be $c$ ones followed by $8 - c$ zeros for $c = \sum_{j=0}^7 \text{O\_U}[j] \in \{1, \dots, 8\}$.
   - Unused slots are zero-padded: `a * (1 - u_j) * x[O_W + j] = 0` (line 627).
   - Non-last `A` rows (`a = 1, last = 0`) are full: `a * (1 - last) * (8 - c) = 0` forces $c = 8$ ($\text{O\_U} = [1; 8]$).
   - Last `A` row (`a = 1, last = 1`) satisfies `x[O_POS] + c = total` (line 634), i.e., $8 b_{\text{last}} + c = 3 + 9 k$ with $c \in \{1, \dots, 8\}$. Since $8 b_{\text{last}} + c < 8 H + 8 \ll p$ and $3 + 9 k \le 2307 \ll p$, this holds in $\mathbb{Z}$ and uniquely determines both the block count $nb = b_{\text{last}} + 1 = \lceil (3 + 9k)/8 \rceil$ and final block width $c$.
4. **`B`-Block Words ([iface_l.rs:636-644](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L636-L644)):**
   - `B0`: `O_W = [TAG_STEP, d_0, d_1, d_2, d_3, d_4, d_5, d_6]`.
   - `B1`: `O_W = [d_7, h_0, h_1, h_2, h_3, h_4, h_5, h_6]`.
   - `B2`: `O_W = [h_7, 0, 0, 0, 0, 0, 0, 0]`.

---

### 3. Values (`VAL_BUS` and `ENT_BUS`) — **SOUND**

**Claim:** `ENT_BUS`, `VAL_BUS`, `LOG_BUS`, and the `OFF` chain force `OnionAir` to absorb, in exact table order, all and only the used entries of `LogEntry` and their exact logged limbs from `LogValue`.

**Proof & Evidence:**
1. **`LogEntry` & Strict Monotonicity of `off` ([iface_l.rs:240-318](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L240-L318)):**
   - `Carrier` binds row $i \in 0..B-1$ of `LogEntry` to public values $(\text{op}_i, \text{param}_i)$.
   - One-hot op bits `b[0..9]` with `b[ECMUL] = b[ECMULG] = 0` and transition `b[0] * (1 - bn0) = 0` force used entries ($\text{op}_i \ne \text{NOP}$, $\text{used}_i = 1 - b_0 = 1$) to form a prefix $i \in 0 .. n_{\text{used}} - 1$, followed by `NOP` rows $i \in n_{\text{used}} .. B - 1$.
   - Every non-`NOP` operation has $\text{vals}_i \ge 1$ (`PERM`: 6, `H2`: 3, `SPONGE`: $\text{param}_i + 1 \ge 2$ since $\text{low} = 1$, `PATH`: 3, `PUB`: 1, `DIGEST`: 2), while `NOP` has $\text{vals}_i = 0$.
   - Since $\text{off}_0 = 0$ and $\text{off}_{i+1} = \text{off}_i + \text{vals}_i$, the offsets on used entries are strictly increasing:
     $$0 = \text{off}_0 < \text{off}_1 < \dots < \text{off}_{n_{\text{used}}-1} < \text{off}_{\text{end}} \le V.$$
2. **`ENT_BUS` + `OFF` Chain Fixes Exact Entry Order ([iface_l.rs:662-673](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L662-L673)):**
   - `LogEntry` sends $(\text{op}_i, \text{param}_i, \text{off}_i, \text{vals}_i)$ on `ENT_BUS` with multiplicity $\text{used}_i \in \{0, 1\}$.
   - `OnionAir` receives $(\text{O\_OP}, \text{O\_PARAM}, \text{O\_OFF}, \text{O\_K})$ on `ENT_BUS` with multiplicity $-\text{O\_FA} \in \{0, -1\}$.
   - Multiset equality on `ENT_BUS` forces the $n_{\text{onion}}$ `FA` rows of `OnionAir` to be a permutation of the $n_{\text{used}}$ used entries of `LogEntry` (so $n_{\text{onion}} = n_{\text{used}}$, and no entry can be absorbed twice or dropped).
   - In `OnionAir`, `x[O_E] * y[O_OFF] = 0` forces the first entry's `O_OFF` to be $0$, `same * (y[f] - x[f]) = 0` carries `O_OFF` and `O_K` to the entry's `B2` row, and `b2 * y[O_FA] * (y[O_OFF] - x[O_OFF] - x[O_K]) = 0` forces $\text{O\_OFF}_{idx+1} = \text{O\_OFF}_{idx} + \text{O\_K}_{idx}$.
   - By induction on $idx \in 0 .. n_{\text{used}} - 1$, entry $idx$ in `OnionAir` must have $\text{O\_OFF}_{idx} = \text{off}_{idx}$, which uniquely matches row $idx$ of `LogEntry`. Thus two entries **cannot** swap.
3. **`LogValue`, `LOG_BUS`, and `VAL_BUS` ([iface_l.rs:327-348](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L327-L348), [628-631](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L628-L631)):**
   - In `LogValue`, `u` is boolean and non-increasing (`(1 - u) * xn[V_USED] = 0`), `cnt` starts at `u_0` and increments by `xn[V_USED]`, and `LOG_BUS` equates $\text{cnt}_{V-1}$ with $\text{off}_{\text{end}}$ from `LogEntry`. Thus $u_r = 1 \iff 0 \le r < \text{off}_{\text{end}}$, and on every used row $r$, $\text{cnt}_r - 1 = r$.
   - `LogValue` sends $(9(\text{cnt}_r - 1) + t, \text{limb}_{r,t}) = (9r + t, \text{limb}_{r,t})$ on `VAL_BUS` with multiplicity $u_r \in \{0, 1\}$. Unused value rows have multiplicity $0$ and **cannot** be read.
   - On `OnionAir`, slot $j$ of an `A` row has `VAL_BUS` multiplicity $-m$ where $m = a \cdot u_j - (\text{if } j < 3 \text{ then } fa \text{ else } 0)$.
   - Since $\text{O\_K} \ge 1 \implies \text{total} = 3 + 9 \cdot \text{O\_K} \ge 12 > 8$, every `FA` row has $\text{O\_LAST} = 0 \implies c = 8 \implies u_0 = u_1 = u_2 = 1$, so on `FA` rows $m = 0$ for $j < 3$ and $m = 1$ for $j \in 3..7$. (Even if $\text{O\_K} = 0$ were possible, $\text{total} = 3 \implies c = 3 \implies u_0 = u_1 = u_2 = 1$, so $m \in \{0, 1\}$ always).
   - Across the `A` rows of entry $i$, the active slots with $m = 1$ have $\text{O\_POS} + j - 3 = 8 b_i + j - 3$ ranging consecutively over $0, 1, \dots, 9 \cdot \text{vals}_i - 1$, querying the unique keys $g = 9 \cdot \text{off}_i + (8 b_i + j - 3) \in [9 \cdot \text{off}_i, 9(\text{off}_i + \text{vals}_i) - 1]$. Hence every limb is read from the exact right entry and value slot.

---

### 4. Chaining — **SOUND**

**Claim:** For each entry $i$, `O_D` equals $d_i$ (`D_EMPTY` for $i = 0$, and entry $(i-1)$'s `B2` output for $i \ge 1$), `O_H` on `B0..B2` equals $h_i = \text{P2}(\text{TAG\_ENT}, \text{enc}(e_i))$, and the `TIE_BUS` tie equals $(\text{onion}(\text{log}), n)$ including for the empty log ($n = 0$).

**Proof & Evidence:**
1. **`O_D` Chaining ([iface_l.rs:654-655](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L654-L655)):**
   - `y[O_FA] * (y[O_D + m] - x[O_OUT + m]) = 0` sets `O_D` on each `FA` row to the preceding row's `O_OUT[0..8]`. By Q1, the row before entry $0$'s `FA` is row $0$ (`E`, whose `O_OUT[0..8]` is $d_0 = \text{P2}(\text{TAG\_EMPTY}, [])$), and the row before entry $i$'s `FA` ($i \ge 1$) is entry $(i-1)$'s `B2` row (whose `O_OUT[0..8]` is $d_i$).
   - `same * (y[O_D + m] - x[O_D + m]) = 0` carries `O_D` unchanged across all continuation `A` rows and `B0, B1, B2` of entry $i$.
2. **`O_H` Chaining ([iface_l.rs:656-659](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L656-L659)):**
   - By Q1, `y[O_B0] = 1` occurs if and only if row $x$ is the `LAST` `A` row of the same entry.
   - `y[O_B0] * (y[O_H + m] - x[O_OUT + m]) = 0` sets `O_H[0..8]` on `B0` to the `LAST` `A` row's output $h_i = \text{P2}(\text{TAG\_ENT}, \text{enc}(e_i))$.
   - `(y[O_B1] + y[O_B2]) * (y[O_H + m] - x[O_H + m]) = 0` carries `O_H[0..8]` unchanged to `B1` and `B2`.
3. **`O_CNT` and `TIE_BUS` Tie ([iface_l.rs:561, 576, 675-677](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L675-L677), [319-326](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L319-L326)):**
   - `O_CNT` starts at $0$ on row $0$ and satisfies `y[O_CNT] = x[O_CNT] + x[O_B2]`. Thus on all rows of entry $i$ ($0 \le i < n$), $\text{O\_CNT} = i$, and on entry $i$'s `B2` row, $\text{O\_CNT} + \text{O\_B2} = i + 1$.
   - On the unique `FIN` row:
     - If $n = 0$ (empty log): the `FIN` row is row $0$ (`O_E = 1, O_B2 = 0`), which sends `(O_OUT[0..8], O_CNT + O_B2) = (D_EMPTY, 0)` on `TIE_BUS`.
     - If $n \ge 1$: the `FIN` row is entry $(n-1)$'s `B2` row (`O_B2 = 1`), which sends `(d_n, (n - 1) + 1) = (onion(log), n)` on `TIE_BUS`.
   - In `Stmt`, `Carrier::new(114, 48)` places `LOG_D_WORD..LOG_N_WORD + 1` ($57..66$) in carrier row $r = 1$ at columns $c_0 .. c_0 + 9 = 9..18$, and `Carrier::eval` ([lane-src/carrier.rs:70-140](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/carrier.rs#L70-L140)) forces `ind = a_1 * b_0` to be $1$ on row $1$ and $0$ elsewhere, binding the received `TIE_BUS` tuple directly to public words `words[57..66]`.

---

### 5. Completeness — **COMPLETE**

**Claim:** For every valid run, `OnionAir::trace` ([lane-src/iface_l.rs:412-527](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L412-L527)) satisfies all constraints in `OnionAir::eval`, balances all buses with `IfaceAir::trace`, and respects the fixed height bound `OnionAir::height()` ([lane-src/iface_l.rs:407-409](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L407-L409)).

**Proof & Evidence:**
1. **Height Bound:**
   - For $n \le B$ used entries with $\sum_{i=0}^{n-1} k_i \le V$, the number of active rows generated by `OnionAir::trace` is:
     $$n_{\text{act}} = 1 + \sum_{i=0}^{n-1} \left( \left\lceil \frac{3 + 9 k_i}{8} \right\rceil + 3 \right).$$
   - Using $\lceil (3 + 9 k_i)/8 \rceil \le (10 + 9 k_i)/8 = 1 + (2 + 9 k_i)/8$, summing over $i \in 0..n-1$ yields:
     $$\sum_{i=0}^{n-1} \left\lceil \frac{3 + 9 k_i}{8} \right\rceil \le n + \left\lfloor \frac{2n + 9 \sum k_i}{8} \right\rfloor \le B + \left\lceil \frac{3B + 9V}{8} \right\rceil.$$
   - Therefore, $n_{\text{act}} \le 1 + 3n + B + \lceil (3B + 9V)/8 \rceil \le 1 + \lceil (3B + 9V)/8 \rceil + 4B \le (1 + \lceil (3B + 9V)/8 \rceil + 4B).\text{next\_power\_of\_two}() = \text{height}()$.
   - At `Dims::FULL` ($B = V = 256$), $1 + \lceil 3072 / 8 \rceil + 1024 = 1409 \le 2048 = 2^{11}$.
2. **Inactive Padding Rows ([iface_l.rs:519-524](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L519-L524)):**
   - Each padding row has all columns $0$ except `r[O_CNT] = cnt_end` ($= n$).
   - At the transition from `rows[n_act - 1]` to `rows[n_act]`: if $n = 0$, `rows[0]` has `O_CNT = 0, O_B2 = 0` and `rows[1]` has `O_CNT = 0 = 0 + 0`; if $n \ge 1$, `rows[n_act - 1]` is the last `B2` row with `O_CNT = n - 1, O_B2 = 1` and `rows[n_act]` has `O_CNT = n = (n - 1) + 1`.
   - Between two padding rows, `O_CNT` stays $n$ and `O_B2 = 0`, and all `ay` / `cont_y` / `same`-gated constraints vanish.

---

### 6. Other Attack Vectors & BabyBear Field Wrap-Around — **SOUND**

**Claim:** Neither `POS`, `OFF`, nor `CNT` (nor `total` or the `VAL_BUS` index $g$) can wrap around modulo $p = 2^{31} - 2^{27} + 1 = 2013265921$, and no auxiliary degree of freedom allows forging `(log_d, log_n)`.

**Proof & Evidence:**
1. **No Wrap-Around in `CNT`:**
   - `O_CNT` starts at $0$ and increments by `x[O_B2] in {0, 1}` on each transition. Across $H \le 2048$ rows (and in fact at most $B = 256$ `B2` rows since `FA` rows are bounded by $B$ via `ENT_BUS`), $0 \le \text{O\_CNT} \le B = 256 \ll p$.
2. **No Wrap-Around in `OFF`:**
   - In `LogEntry`, `param` is range-checked by two 11-bit slack decompositions (`E_LO`, `E_HI` in $[0, 2047]$) against `low in {0, 1}` and `high <= 1025`, so `vals_i` is an integer in $[0, V]$ ($V = 256$).
   - Over $B = 256$ rows, $\text{off}_{\text{end}} = \sum_{i=0}^{B-1} \text{vals}_i \in [0, B \cdot V] = [0, 65536] \ll p$.
   - On the last row $B - 1$, `V - off_end` equals a 9-bit sum in $[0, 511]$, which in $\mathbb{F}_p$ forces $0 \le \text{off}_{\text{end}} \le V = 256$.
3. **No Wrap-Around in `POS`, `total`, or `VAL_BUS` Index $g$:**
   - `O_POS` resets to $0$ on each `FA` row and increments by $8$ on continuation `A` rows. Over at most $H = 2048$ rows, $0 \le \text{O\_POS} < 8 H = 16384 \ll p$.
   - On the `LAST` `A` row, $\text{O\_POS} + c = 3 + 9 \cdot \text{O\_K}$ has left-hand side in $[1, 16392]$ and right-hand side in $[12, 2307]$, both far below $p$, so equality in $\mathbb{F}_p$ is equality in $\mathbb{Z}$.
   - On `VAL_BUS`, for every slot with multiplicity $m = 1$, $g = 9 \cdot \text{O\_OFF} + \text{O\_POS} + j - 3 \in [0, 9V - 1] = [0, 2303] \ll p$, so keys never wrap or collide mod $p$.
4. **Non-Negativity of `VAL_BUS` Multiplicity $m$:**
   - Could a dishonest prover make $m = a \cdot u_j - (\text{if } j < 3 \text{ then } fa \text{ else } 0)$ equal $-1$ (so `-m = +1`, injecting a forged value into `VAL_BUS`)?
   - For $m = -1$ to occur, we would need $j < 3$, $\text{fa} = 1$, and $a \cdot u_j = 0$.
   - However, `fa * (1 - a) = 0` forces $a = 1$ whenever $\text{fa} = 1$; and on every `FA` row, since $\text{total} = 3 + 9 \cdot \text{O\_K} \ge 12 > 8$, `last` must be $0$ (otherwise $\text{O\_POS} + c = 0 + c \le 8 < 12$), which forces $c = 8 \implies u_0 = u_1 = \dots = u_7 = 1$. Thus $a \cdot u_j = 1$ for all $j < 3$ on every `FA` row, so $m \in \{0, 1\}$ is strictly invariant.

---

## Quint Formal Model & Mutants

- **Model file:** [onion.qnt](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/qa/gemini-quint/e3-onion/onion.qnt)
- **Test file:** [onion_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/qa/gemini-quint/e3-onion/onion_test.qnt)

### Mutants & How Each Is Caught

We defined 7 mutants in [onion.qnt](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/qa/gemini-quint/e3-onion/onion.qnt), each relaxing one constraint family in `OnionAir`, and paired each with an explicit adversarial counterexample instance (`cex_*`) and invariant (`INV_MUT_*`):

| # | Mutant Predicate | Dropped / Relaxed Constraint | Concrete Counterexample (`cex_*`) & How Caught |
| :- | :--- | :--- | :--- |
| 1 | `mut_no_off_chain` | Omits `x[O_E] * y[O_OFF] = 0` and `b2 * y[O_FA] * (y[O_OFF] - x[O_OFF] - x[O_K]) = 0` ([iface_l.rs:664-666](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L664-L666)) | `cex_no_off_chain`: On a 2-entry log (`PUB` at `off=0, k=1`, `DIGEST` at `off=1, k=2`), absorbs entry 1 before entry 0. `ENT_BUS` and `VAL_BUS` still balance (since `VAL_BUS` uses each entry's carried `O_OFF`), yielding `onion([e_1, e_0]) != onion([e_0, e_1])`. Caught by `system_ok` (`test_mut_no_off_chain`, `INV_MUT_NO_OFF_CHAIN`). |
| 2 | `mut_fa_reads_w0` | Omits `fa * (x[O_W] - TAG_ENT) = 0` ([iface_l.rs:611](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L611)) | `cex_fa_reads_w0`: Because `VAL_BUS` skips $j < 3$ on `FA` rows, setting `w[0] = 99 != TAG_ENT` on an `FA` row still balances `VAL_BUS` and `ENT_BUS` while forging $h_0$ and `log_d`. (Additionally, `test_mut_fa_reads_w0` verifies that if `FA` did not skip $j < 3$ on `VAL_BUS`, honest traces fail `VAL_BUS` balance due to negative keys `off*2 + j - 3`). Caught by `system_ok` (`test_mut_fa_reads_w0`, `INV_MUT_FA_READS_W0`). |
| 3 | `mut_no_last_count` | Omits `a * last * (x[O_POS] + c - total) = 0` ([iface_l.rs:634](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L634)) | `cex_no_last_count`: On two `PUB` entries ($k_0 = k_1 = 1$, `total = 5`), entry 0 sets $c = 3$ on its `LAST` row (`pos = 4`, reading $g = 1, 2, 3$ and stealing entry 1's two limbs), while entry 1 uses a single `A` row (`FA = 1, LAST = 1, pos = 0`) with $c = 3$ (`u = [1, 1, 1, 0]`), which queries **zero** limbs on `VAL_BUS` since $j < 3$ is skipped on `FA` and $u_3 = 0$! `VAL_BUS` and `ENT_BUS` balance 100%, yet both entries' hashes and `log_d` are forged. Caught by `system_ok` (`test_mut_no_last_count`, `INV_MUT_NO_LAST_COUNT`). |
| 4 | `mut_fin_on_b1` | Omits `x[O_B1]` from `x[O_FIN] * (x[O_A] + x[O_B0] + x[O_B1]) = 0` and allows `B1` to transition to inactive ([iface_l.rs:573, 579](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L573-L579)) | `cex_fin_on_b1`: Truncates a 1-entry trace after `B1` (`O_FIN = 1` on `B1`, followed by inactive row). `ENT_BUS` and `VAL_BUS` still balance (neither is used on `B2`), forging both `log_d` (`B1.out` instead of `B2.out`) and `log_n` ($0$ instead of $1$). Caught by `system_ok` (`test_mut_fin_on_b1`, `INV_MUT_FIN_ON_B1`). |
| 5 | `mut_cap_not_chained` | Omits `cont_y * (y[O_CAP + m] - x[O_OUT + 8 + m]) = 0` ([iface_l.rs:594-599](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L594-L599)) | `cex_cap_not_chained`: Sets `O_CAP = [0, 0, 0, 0]` on the second `A` block of entry 0 instead of chaining the first `A` block's capacity output, forging $h_0$ and `log_d` while balancing all buses. Caught by `system_ok` (`test_mut_cap_not_chained`, `INV_MUT_CAP_NOT_CHAINED`). |
| 6 | `mut_cnt_no_b2` | Omits `- x[O_B2]` in `y[O_CNT] - x[O_CNT] - x[O_B2] = 0` and `+ b2` in the `TIE_BUS` count ([iface_l.rs:576, 676](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L576)) | `cex_cnt_no_b2`: Keeps `O_CNT = 0` across a 1-entry trace and sends `log_n = 0 != 1` on `TIE_BUS`. Caught by `system_ok` (`test_mut_cnt_no_b2`, `INV_MUT_CNT_NO_B2`). |
| 7 | `mut_d_not_chained` | Omits `y[O_FA] * (y[O_D + m] - x[O_OUT + m]) = 0` ([iface_l.rs:655](file:///home/hoskinson/.cache/gemini-lanes/e3-onion/lane-src/iface_l.rs#L655)) | `cex_d_not_chained`: Replaces `O_D` on entry 0 with `[0, 0, 0, 0]` instead of `D_EMPTY` from row `E`, forging `log_d` while balancing all buses. Caught by `system_ok` (`test_mut_d_not_chained`, `INV_MUT_D_NOT_CHAINED`). |

---

## Commands Run & Outcomes

*(Pending execution check of `quint typecheck`, `quint test`, and `quint run` in sandbox.)*

---

## What the Model Does Not Cover

1. **Concrete Poseidon2 Cryptographic Permutation:** The Quint model replaces the 16-word BabyBear Poseidon2 permutation (`Poseidon2SubAir` in `recursion_poseidon2.rs`) with an ideal, collision-free 8-word Cantor-pairing bijection `perm8`. Preimage/collision resistance of Poseidon2 itself and the internal round constraints of `Poseidon2SubAir` are outside the scope of `OnionAir`.
2. **Reduced Block & Limb Dimensions:** The Quint model uses block/rate/digest/capacity width `W_BLK = 4` (instead of $8$), `LIMBS = 2` limbs per value (instead of $9$), `STEP_LEN = 9` (instead of $17$), and up to $2$ active entries with $k \in \{1, 2, 3\}$ values (`PUB`, `DIGEST`, `H2`), preserving the exact 3-word `FA` header `[TAG_ENT, op, param]`, multi-block `FA` / middle-`A` / `LAST` transitions, and 3-block `B0, B1, B2` split of `d` and `h`.
3. **Finite-Field Arithmetic mod $p = 2^{31} - 2^{27} + 1$:** Quint `int` models arbitrary-precision integers $\mathbb{Z}$ rather than $\mathbb{F}_p$; absence of modulo-$p$ wrap-around for `POS`, `OFF`, `CNT`, `total`, and `VAL_BUS` indices is established analytically in Question 6 above.
4. **LogUp Soundness & Wrap-Level Checks (Plan D-9 / M7b):** LogUp bus arguments (`ENT_BUS`, `VAL_BUS`, `TIE_BUS`, `LOG_BUS`, `Poseidon2PermuteBus`) are modeled as exact multiset/lookup balances; polynomial IOP soundness of LogUp, limb range ($< 2^{30}$), canonicality (`Canon`, $< r$), and native Fr operation replay (`iface::decode`) belong to the STARK backend and outer wrap circuit.

---

## Lead verification (2026-11-13, quint 0.32.0)

The lane could not run commands (headless sandbox) and could not read the skills: `.lane-skills/` held symlinks to a path outside the checkout (a lead setup defect; real copies for round 2). The lane's original files are in `lane-original/`; the lead made these changes before running anything:

1. Typecheck fixes: `[a, b, ..].forall(..)` on list literals does not typecheck (forall is a set operator); changed to `Set(a, b, ..).forall(..)` in `onion.qnt` and `onion_test.qnt`. In `test_state_machine_sound_and_complete` the last `all { .. }` updated no variable; `step` added to it.
2. The ideal permutation `perm8` (Cantor pairing) overflowed the Rust evaluator on chained digests (`QNT601 Integer overflow`). Replaced by a bijection of Z_M^8, M = 1000003: `out = V in + c (mod M)` with V the Vandermonde matrix on nodes 2 to 9 (invertible mod a prime), so every output word mixes every input word. The counterexample instances are built by the model's own functions, so they follow the new permutation.
3. Lead extension `step_tamper` / `TAMPER_SOUND`: from each honest instance, one random change (one value limb, one tie word, `log_n`, or one of 15 trace cells: pos, cnt, op, param, off, k, the A/FA/LAST/B2/FIN selectors, a block word, a d word, an h word, a U bit) by -2, -1, 1 or 2, with the claimed tie reset to the tampered trace's FIN tie. Invariant: `system_ok` implies the tie equals the native onion of the tampered tables.

The Rust evaluator of `quint test` and `quint run` crashed on this model after fix 2 (`Failed to parse test result from Rust evaluator: {}`), so the runs below use `--backend=typescript`. Note: `quint test` without `--match` selects names ending in `Test`, so it ran no test of this file; the runs use `--match=test_`.

| Command | Outcome |
|---|---|
| `quint typecheck onion.qnt` / `onion_test.qnt` | ok (after fix 1) |
| `quint test onion_test.qnt --main=onion_test --match=test_ --backend=typescript --seed=0x1 --max-samples=200` | 13 passing: 5 completeness (empty log, PUB, DIGEST, H2, two entries), 7 mutant tests, `test_state_machine_sound_and_complete` (200 runs) |
| `quint run onion.qnt --main=onion --backend=typescript --init=init --step=step --invariant=SOUND_AND_COMPLETE --max-samples=500 --max-steps=4 --seed=0x1` | `[ok] No violation found` |
| `quint run ... --step=step_tamper --invariant=TAMPER_SOUND --max-samples=300 --max-steps=1 --seed=0x2a` | `[ok] No violation found` |
| `quint run ... --step=step_tamper --invariant=TAMPER_SOUND --max-samples=3000 --max-steps=1 --seed=0x2b` | `[ok] No violation found` |
| `quint run ... --step=step --invariant=INV_MUT_<m> --max-samples=500 --max-steps=4 --seed=0x1` for the 7 mutants NO_OFF_CHAIN, FA_READS_W0, NO_LAST_COUNT, FIN_ON_B1, CAP_NOT_CHAINED, CNT_NO_B2, D_NOT_CHAINED | `[violation] Found an issue` for each of the 7 (each mutant accepts its counterexample, whose tie differs from the native onion) |

Each mutant test asserts three things: the mutant accepts the counterexample, the counterexample's tie is not the native onion, and the real constraint system refuses it. So all 7 mutants are caught.

Lead assessment: the verdicts SOUND for Q1 to Q4 and Q6 and COMPLETE for Q5 agree with the lead's own reading of `OnionAir::eval` (the OFF chain fixes the order because every non-NOP op has at least 1 value; the FA row cannot be a LAST row as `total >= 12 > 8`). The model checks soundness on the honest instances, 7 hand-built attacks and random one-cell tampering; it is not an exhaustive search over all traces, so the SOUND verdicts rest on the lane's analytic arguments above.
