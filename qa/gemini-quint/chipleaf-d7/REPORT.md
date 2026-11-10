# Soundness Review of Design Point D7: The Cached Frame Protocol (`chipleaf-d7`)

- **Design under test**: [`openspec/changes/sherlock-vm-consensus-20261107/design.md`](../../../openspec/changes/sherlock-vm-consensus-20261107/design.md) §2.7 & §2.7.8 (`D1..D8`), [`research/sherlock-processor-analogy-20261110/study.md`](../../../research/sherlock-processor-analogy-20261110/study.md)
- **Prior model & findings (`F12`, `F13`)**: [`qa/gemini-quint/chipleaf/chipleaf.qnt`](../chipleaf/chipleaf.qnt), [`qa/gemini-quint/chipleaf/REPORT.md`](../chipleaf/REPORT.md)
- **Quint model**: [`qa/gemini-quint/chipleaf-d7/cached.qnt`](cached.qnt)
- **Quint test suite**: [`qa/gemini-quint/chipleaf-d7/cached_test.qnt`](cached_test.qnt)

---

## 1. Verdict

### **SOUND: YES (for the Cached Frame Protocol with D1 to D8)**

The **cached frame protocol** for design point **D7**, combined with constraints **D1 to D8** of `design.md` §2.7.8, is **SOUND** across chained leaves under a single halo2 `lookup_any` set-inclusion bus with no multiplicities.

Specifically, over any sequence of chained leaves (`leaf2.pre == leaf1.post`), every accepted execution (`acceptsCached(w)` in [`cached.qnt`](cached.qnt)) satisfies the frame-stack refinement invariant **`invFrameStackRefinement`**:
1. The sequence of `POP` results emitted by the circuit across the chained leaves equals the sequence of `POP` results of the abstract reference stack (`chainPopLog(w) == refFinal.popLog`).
2. The final frame depth equals the length of the reference stack (`w.leaf2.post.depth == refDepth(refFinal)`).
3. The final `top` register equals the top of the reference stack, or `0` when the stack is empty (`w.leaf2.post.top == refTop(refFinal)`).

---

## 2. Why Set Inclusion Without Multiplicities Is Sound Across Chained Leaves

In the cached protocol:
- **Registers**: `CORE` holds `top` (the value of the top frame), `fp` (the address in the memory log of the caller frame *below* `top`), `depth` (the number of frames), `len` (the memory log length), and `memRoot`. At `depth == 0`, the registers hold the sentinel `(top = 0, fp = NIL)` with `NIL = -1`.
- **`PUSH v`**: appends `(pre.top, pre.fp)` at address `pre.len` via bus request `(TAG_CELL, pre.len, pre.top, pre.fp, 0)`, and updates `fp := pre.len`, `len := pre.len + 1`, `top := v`, `depth := pre.depth + 1`. At `depth == 0`, this spills the sentinel `(0, NIL)` to address `base + k`.
- **`POP` (`depth >= 1`)**: produces `pre.top` as the popped result, issues bus request `(TAG_CELL, pre.fp, a, b, 0)` with `0 <= pre.fp < pre.len`, and updates `top := a`, `fp := b`, `depth := pre.depth - 1`, `len := pre.len`. At `depth == 1`, it loads the spilled sentinel `(0, NIL)` back into `(top, fp)`.
- **`REWRITE v` (`depth >= 1`)**: updates `top := v` in `CORE` registers only, with no bus request and with `fp`, `depth`, `len`, and `memRoot` unchanged.
- **Leaf boundary (`pre_d` / `post_d`)**: carries `top`, `fp`, `depth`, `len`, and `memRoot` (along with `pc`, `acc`, `halted`) from `leaf1.post` to `leaf2.pre`.

Under **D1 to D8**:
1. **Intra-leaf bijection (`D1`, `D3`, `D4`, `D6`, `D8`)**: Each enabled `PUSH` in a leaf increments `len` by 1 and requests `(TAG_CELL, pre.len, pre.top, pre.fp, 0)`. By **D6** (`addr_k == base + k` with `base == pre.len`) and **D2** (`PAGE` serves `TAG_CELL` only for `0 <= idx < base`), the $k$-th `PUSH` in a leaf can match **only** `MEM` row $k$. By **D3**, `post.memRoot` hashes exactly `MEM` rows `0 .. post.len - base - 1`.
2. **Cross-leaf continuity (`D2`, `D3`, `D7`)**: When `PUSH`es happen in Leaf 1 and `POP`s happen in Leaf 2:
   - The topmost pushed value $v_n$ crosses the leaf boundary directly in the statement register `top` (`leaf2.pre.top == leaf1.post.top`) without ever being written to Leaf 1's `MEM`.
   - The $n - 1$ deeper frames and the depth-0 sentinel `(0, NIL)` were spilled to Leaf 1's `MEM` at addresses `< leaf1.post.len == leaf2.pre.len` (`base_2`) and committed into `leaf1.post.memRoot == leaf2.pre.memRoot`.
   - In Leaf 2, every `POP` of a frame pushed in Leaf 1 queries `key = pre.fp < base_2`. By **D6**, no `MEM` row in Leaf 2 has `addr < base_2`, so the request can be served **only** by an authenticated `PAGE` row opened against `leaf2.pre.memRoot`.
   - Although set inclusion (`lookup_any`) has no multiplicities, `POP` cannot replay the same `PAGE` row unless the stored link `b` forms a cycle, which is impossible because every honest `PUSH` stores `pre.fp < pre.len` and sets `post.fp = pre.len` (strictly decreasing links terminating at `NIL = -1`).

### Honest Chained-Leaf Witnesses in [`cached.qnt`](cached.qnt)

| Witness Definition | Leaf 1 Steps (`S = 4`) | Leaf 2 Steps (`S = 4`) | POP Sequence | Final `(depth, top, fp)` | What It Exercises |
|---|---|---|---|---|---|
| `hw_push3_pop3` | `PUSH 10`, `PUSH 20`, `PUSH 30`, `LIT 99` | `POP`, `POP`, `POP`, `HALT` | `[30, 20, 10]` | `(0, 0, NIL)` | 3 distinct frames pushed in Leaf 1, popped in Leaf 2 down to sentinel `(0, NIL)` |
| `hw_push3_rewrite_pop3` | `PUSH 100`, `PUSH 200`, `REWRITE 250`, `PUSH 300` | `POP`, `POP`, `POP`, `HALT` | `[300, 250, 100]` | `(0, 0, NIL)` | 3 frames pushed + middle frame rewritten in-place in Leaf 1, all 3 popped in Leaf 2 |
| `hw_push4_rewrite_leaf2_pop3` | `PUSH 11`, `PUSH 22`, `PUSH 33`, `PUSH 44` | `REWRITE 45`, `POP`, `POP`, `POP` | `[45, 33, 22]` | `(1, 11, 0)` | 4 frames pushed in Leaf 1, top rewritten in Leaf 2, 3 popped leaving non-empty stack `[11]` |
| `hw_intra_and_inter_leaf_pop` | `PUSH 10`, `PUSH 20`, `PUSH 30`, `REWRITE 35` | `PUSH 40`, `POP` (`MEM`), `POP` (`PAGE`), `POP` (`PAGE`) | `[40, 35, 20]` | `(1, 10, 0)` | Both intra-leaf `POP` (served by Leaf 2's `MEM`) and cross-leaf `POP` (served by Leaf 2's `PAGE`) |

---

## 3. Numbered Findings, Mutants, Witnesses, and Closing Constraints

### 3.1 Does the Cached Protocol Need Constraints Beyond the First Four Mutants?

**Yes.** Beyond the four core mutants (`rewrite_appends`, `link_from_bus`, `pop_no_reload`, `top_dropped`), the cached protocol requires **four additional load-bearing constraints** (Findings **F5** to **F8** below):
1. **On the depth-0 sentinel (`F5`, `sentinel_unconstrained`)**: Because `PUSH` at `depth == 0` unconditionally spills `(pre.top, pre.fp)` to `MEM[pre.len]` and `POP` at `depth == 1` unconditionally loads that entry back into `(post.top, post.fp)`, genesis (`depth == 0`) **must** constrain `top == 0` and `fp == NIL` (with `NIL` outside valid addresses `[0, len)`). Otherwise, a non-zero initial `top = 777` at `depth == 0` is spilled by the first `PUSH` in Leaf 1 and reloaded by the last `POP` in Leaf 2, leaving `post.top == 777 != 0` at `depth == 0`.
2. **On `depth` guards, step transitions, and statement chaining (`F6`, `depth_unconstrained`)**: `POP` and `REWRITE` must require `pre.depth >= 1`, `PUSH` must increment `depth` by 1, `POP` must decrement `depth` by 1, other ops must preserve `depth`, and `depth` must be carried in `pre_d`/`post_d`. Otherwise, `post.depth` diverges from the true stack height (or `REWRITE` at `depth == 0` overwrites the sentinel `top` before the first `PUSH`).
3. **On `POP` reloading `fp := b` from the loaded entry (`F7`, `pop_no_fp_reload`)**: Because `lookup_any` is set inclusion without multiplicities, a single valid `PAGE` row `(TAG_CELL, 2, 20, 1, 0)` can satisfy arbitrarily many `POP` requests if `fp` stays at `2`. `POP` must constrain both `cellReq.key == pre.fp` and `post.fp == cellReq.b` so consecutive `POP`s walk the pointer chain `2 -> 1 -> 0 -> NIL`.
4. **On `POP` reading an entry served by `PAGE` across leaves (`F8`, `pop_page_unauthenticated`)**: Every cross-leaf `POP` in Leaf 2 reads an address `0 <= fp < leaf2.pre.len` (`base`), which by **D2** and **D6** is served exclusively by `PAGE`. `PAGE` `CELL` rows must open a valid Merkle path against `leaf2.pre.memRoot` (chained to `leaf1.post.memRoot`, which by **D3** folds Leaf 1's active `MEM` rows) and satisfy `0 <= idx < base`. Otherwise, `PAGE` can serve a forged caller frame `(888, 1)` on `POP`.

### 3.2 Complete Findings Table (`F1` to `F8`)

For every finding below, `acceptsWithMutant("<mutant>", cex_*) == true`, `acceptsCached(cex_*) == false`, and `refinesRefStack(cex_*) == false`:

| # | Finding | Mutant Name / Module | Witness (`cached.qnt`) | Reference vs. Witness Behavior | Constraint That Closes It |
|---|---|---|---|---|---|
| **F1** | **`REWRITE` appends `(v, fp)` at `len` and updates `fp := len` (prior F12)**: In Program 1, `REWRITE 250` appends `(250, 1)` at `addr = 2` and sets `fp := 2`; `PUSH 300` then spills `(250, 2)` at `addr = 3`. In Leaf 2, `POP` #2 loads `PAGE[2] = (250, 1)` (the duplicate rewritten frame) instead of `(100, 0)` at `addr = 1`. | `rewrite_appends` (`cached_rewrite_appends`) | `cex_rewrite_appends` | Ref POPs: `[300, 250, 100]`, final `top = 0`<br>Witness POPs: `[300, 250, 250]`, final `top = 100` | **`D7-REWRITE`**: On `OP_REWRITE`, enforce `cellReq == PAD_TUPLE`, `post.top == imm`, `post.fp == pre.fp`, `post.len == pre.len`, `post.memRoot == pre.memRoot`. |
| **F2** | **`PUSH` stores link from a free bus field instead of `pre.fp` (prior F13)**: On the 3rd push (`PUSH 30` at `addr = 2`), the prover sets `cellReq.b = 2` (a self-loop link) instead of `pre.fp = 1`, writing `(20, 2)` to `MEM[2]`. In Leaf 2, `POP` #1, #2, and #3 loop on `PAGE[2]`, never reaching frame `10` at `addr = 1`. | `link_from_bus` (`cached_link_from_bus`) | `cex_link_from_bus` | Ref POPs: `[30, 20, 10]`, final `top = 0`<br>Witness POPs: `[30, 20, 20]`, final `top = 20` | **`D7-PUSH-LINK`**: On `OP_PUSH`, copy-constrain `cellReq.b == pre.fp` (and `cellReq.a == pre.top`, `cellReq.key == pre.len`, `post.fp == pre.len`). |
| **F3** | **`POP` does not reload `top` from the loaded entry**: In Leaf 2, each `POP` loads `(a, b)` from `PAGE[fp]` and updates `fp := b`, but keeps `post.top == pre.top` (`30`), emitting `30` on all three `POP`s. | `pop_no_reload` (`cached_pop_no_reload`) | `cex_pop_no_reload` | Ref POPs: `[30, 20, 10]`, final `top = 0`<br>Witness POPs: `[30, 30, 30]`, final `top = 30` | **`D7-POP-TOP`**: On `OP_POP`, constrain `post.top == cellReq.a` where `cellReq = (TAG_CELL, pre.fp, a, b, 0)`. |
| **F4** | **Leaf statement drops `top` across the leaf boundary**: In Leaf 1, `PUSH 30` places `30` in `leaf1.post.top` (never in `MEM`). When `top` is omitted from `pre_d`/`post_d`, Leaf 2 starts with prover-chosen `pre.top = 999`, so `POP` #1 in Leaf 2 returns `999`. | `top_dropped` (`cached_top_dropped`) | `cex_top_dropped` | Ref POPs: `[30, 20, 10]`, final `top = 0`<br>Witness POPs: `[999, 20, 10]`, final `top = 0` | **`D7-STMT-TOP`**: Include `top` in `pre_d`/`post_d` so the leaf chain enforces `leaf2.pre.top == leaf1.post.top`. |
| **F5** | **Depth-0 sentinel `(top = 0, fp = NIL)` unconstrained at genesis**: Genesis starts at `depth = 0` with `top = 777`. `PUSH 10` in Leaf 1 spills `(777, NIL)` to `MEM[0]`; `POP` #3 in Leaf 2 loads `PAGE[0] = (777, NIL)`, leaving `post.top = 777` at `depth = 0`. | `sentinel_unconstrained` (`cached_sentinel_unconstrained`) | `cex_sentinel_unconstrained` | Ref final `(depth, top) = (0, 0)`<br>Witness final `(depth, top) = (0, 777)` | **`D7-SENTINEL`**: At genesis (`depth == 0`), enforce `top == 0` and `fp == NIL` (`NIL = -1` outside `[0, len)`). |
| **F6** | **`depth` register unconstrained on `POP` / across steps**: `POP` in Leaf 2 fails to decrement `depth` by 1 (`post.depth == pre.depth`), leaving `leaf2.post.depth = 3` when the stack is empty (`0`). | `depth_unconstrained` (`cached_depth_unconstrained`) | `cex_depth_unconstrained` | Ref final `depth = 0`<br>Witness final `depth = 3` | **`D7-DEPTH`**: Enforce `pre.depth >= 1` on `POP`/`REWRITE`, `post.depth == pre.depth + 1` on `PUSH`, `post.depth == pre.depth - 1` on `POP`, `post.depth == pre.depth` on other ops, and carry `depth` in `pre_d`/`post_d`. |
| **F7** | **`POP` does not reload `fp := b` from the loaded entry**: In Leaf 2, `POP` #1 updates `top := 20` from `PAGE[2] = (20, 1)` but keeps `fp = 2`. Because set inclusion has no multiplicities, `POP` #2 and `POP` #3 replay `PAGE[2] = (20, 1)`, returning `[30, 20, 20]`. | `pop_no_fp_reload` (`cached_pop_no_fp_reload`) | `cex_pop_no_fp_reload` | Ref POPs: `[30, 20, 10]`, final `top = 0`<br>Witness POPs: `[30, 20, 20]`, final `top = 20` | **`D7-POP-FP`**: On `OP_POP`, constrain `cellReq.key == pre.fp`, `0 <= pre.fp < pre.len`, and `post.fp == cellReq.b`. |
| **F8** | **Cross-leaf `POP` reads an unauthenticated `PAGE` entry**: In Leaf 2, `POP` #1 reads `fp = 2 < base` from `PAGE`. Without authenticating `PAGE` `CELL` rows against `leaf2.pre.memRoot == leaf1.post.memRoot` (with `0 <= idx < base`, `D2`/`D3`), `PAGE` serves `(888, 1)` at `idx = 2`, so `POP` #2 returns `888`. | `pop_page_unauthenticated` (`cached_pop_page_unauthenticated`) | `cex_pop_page_unauthenticated` | Ref POPs: `[30, 20, 10]`<br>Witness POPs: `[30, 888, 10]` | **`D2` + `D3` + `PAGE-AUTH`**: Authenticate every active `PAGE` `CELL` row against `pre.memRoot` (`== prevLeaf.post.memRoot`) and enforce `0 <= idx < pre.len`. |

---

## 4. Cost Comparison: Cached vs. Uncached Frame Protocol

Let us compare the two candidate protocols for **D7**:
- **Cached protocol**: `top` lives in `CORE` registers; `fp` points to the memory entry *below* `top`; `PUSH v` appends `(top, fp)` at `len` and sets `fp := len, top := v`; `POP` returns `top`, loads `mem[fp] = (a, b)` and sets `top := a, fp := b`; `REWRITE v` sets `top := v` in registers only.
- **Uncached protocol**: no `top` register; `fp` points to the *top* entry itself in memory; `PUSH v` appends `(v, fp)` at `len` and sets `fp := len`; `POP` loads `mem[fp] = (v, link)`, returns `v`, and sets `fp := link`; `REWRITE v` loads `mem[fp] = (old_v, link)` to recover `link`, appends `(v, link)` at `len`, and sets `fp := len`.

| Operation / Metric | Cached Protocol | Uncached Protocol | Difference (Uncached vs. Cached) |
|---|---|---|---|
| **`PUSH v` — `MEM` appends** | **1** (`(top, fp)` at `len`) | **1** (`(v, fp)` at `len`) | Same (1 `MEM` append + 1 `mem_root` update) |
| **`PUSH v` — `MEM` / `PAGE` reads** | **0** | **0** | Same (0 reads) |
| **`POP` — `MEM` appends** | **0** | **0** | Same (0 appends) |
| **`POP` — `MEM` / `PAGE` reads** | **1** (loads `mem[fp]` below top) | **1** (loads `mem[fp]` at top) | Same (1 read) |
| **`REWRITE v` — `MEM` appends** | **0** (register-only `top := v`) | **1** (appends `(v, link)` at `len`) | **Uncached adds +1 `MEM` append** (+ ~200–750 `HASH` rows for `mem_root`) |
| **`REWRITE v` — `MEM` / `PAGE` reads** | **0** (register-only `top := v`) | **1** (loads `mem[fp]` to get `link`) | **Uncached adds +1 `MEM`/`PAGE` read** (+500 `HASH` rows if via depth-20 `PAGE`) |
| **Inspect `top` frame tag in `DEC` dispatch (`study.md` §2.2)** | **0 reads** (already in `top` register) | **1 read** (must load `mem[fp]` when `depth >= 1`) | **Uncached adds +1 read** on steps dispatching on the top frame tag |
| **Registers added to leaf statement (`pre_d` / `post_d`)** | **`top`, `fp`, `depth`, `len`, `memRoot`** (+1 abstract `top` reg; in full Sherlock `study.md` §1.3: 13 frame lanes / ~3–4 packed words for `top`, 5 words with `task`) | **`fp`, `depth`, `len`, `memRoot`** (no `top` register) | **Cached adds `top`** (+1 abstract reg; ~3–4 packed words hashed once at the 2 leaf boundaries) |

---

## 5. Recommendation for Design Point D7

**Recommendation: Adopt the CACHED frame protocol for D7.**

1. **`REWRITE` is free in the cached protocol and expensive in the uncached protocol**:
   In Sherlock's CEK and helper machine (`study.md` §2.4), rules frequently rewrite the top continuation frame or active helper task in place (e.g., advancing from left to right operand evaluation, stepping through `Cases` / `Bind` / `Match` arms, and advancing helper cursors).
   - Under **Cached**, `REWRITE` costs **0 bus requests, 0 `MEM` appends, 0 `MEM`/`PAGE` reads, and 0 `HASH` permutations**, and does not consume `MEM` capacity `N_MEM`.
   - Under **Uncached**, every `REWRITE` costs **1 `MEM`/`PAGE` read + 1 `MEM` append**. Each append costs ~200–750 `HASH` rows to update `mem_root` (`study.md` §4.1), and if the top frame was pushed in a prior leaf (`fp < base`), the read to fetch `link` costs an additional ~500 `HASH` rows for a depth-20 `PAGE` Merkle opening (`study.md` §2.3).
2. **In-circuit dispatch (`study.md` §2.2) needs the top frame tag**:
   `study.md` §2.2 computes the dispatch `class` from registers, including the top frame tag and helper task state. Keeping `top` in `CORE` registers makes dispatch a pure register operation; removing `top` forces an extra memory read before decoding.
3. **Statement digest overhead is paid once per leaf, not once per step**:
   Carrying `top` in `pre_d`/`post_d` adds 1 abstract register (~3–4 packed words for the 13-lane frame in `study.md` §1.3), costing at most ~2 Poseidon permutations (~50 `HASH` rows) **per 16-step leaf**, which is 4x–25x cheaper than even a **single** `REWRITE` under the uncached protocol.
4. **Uniform hardware datapath**:
   With the depth-0 sentinel `(top = 0, fp = NIL)` enforced at genesis (`F5`), `PUSH` and `POP` need no special-case muxing for `depth == 0` vs. `depth == 1`: `PUSH` always spills `(top, fp)` and `POP` (`depth >= 1`) always reloads `(top, fp)` from `mem[fp]`.

---

## 6. Exact `quint` Commands for the Lead to Run

### 6.1 Typecheck and Unit / Mutant Test Suite (Expected: **PASS**)

```bash
quint typecheck qa/gemini-quint/chipleaf-d7/cached.qnt
quint typecheck qa/gemini-quint/chipleaf-d7/cached_test.qnt
quint test --main cached_test qa/gemini-quint/chipleaf-d7/cached_test.qnt
```

### 6.2 Soundness Invariants on `cachedSound` (Expected: **PASS / `[ok]`**)

```bash
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cachedSound --invariant invFrameStackRefinement
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cachedSound --invariant invSound
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cachedSound --invariant invTamperRefused
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cachedSound --invariant invPopSeq
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cachedSound --invariant invFinalDepth
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cachedSound --invariant invFinalTop
```

### 6.3 Non-Vacuity / Reachability Witnesses on `cachedSound` (Expected: **FAIL / Witness Found**)

```bash
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cachedSound --invariant reachPushInLeaf1PopInLeaf2
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cachedSound --invariant reachRewriteAndPopAcrossLeaves
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cachedSound --invariant reachNonEmptyFinalStack
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cachedSound --invariant reachIntraAndInterLeafPop
```

### 6.4 Required and Additional Mutants (Expected: **FAIL / Counterexample Found on Each**)

```bash
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cached_rewrite_appends --invariant invFrameStackRefinement
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cached_link_from_bus --invariant invFrameStackRefinement
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cached_pop_no_reload --invariant invFrameStackRefinement
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cached_top_dropped --invariant invFrameStackRefinement
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cached_sentinel_unconstrained --invariant invFrameStackRefinement
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cached_depth_unconstrained --invariant invFrameStackRefinement
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cached_pop_no_fp_reload --invariant invFrameStackRefinement
quint run qa/gemini-quint/chipleaf-d7/cached.qnt --main cached_pop_page_unauthenticated --invariant invFrameStackRefinement
```

---

## 7. Lead runs (added by the lead, 2026-11-10)

Lane: `argon-run`, `gemini-4-argon-high`, conversation `c6462ddc-d111-4f6a-a457-758a4b995303`,
3 turns, `DONE`. Task text: `TASK.md` in this folder.

One repair before the model ran: the parameter `item` of `putCell` (`cached.qnt` lines 189-190)
is a built-in name (QNT101) and was renamed to `cellVal`. No other change.

With `quint` 0.32.0, `--max-samples 20000 --max-steps 1 --seed 20261110`:

- `quint typecheck cached.qnt` and `cached_test.qnt`: exit 0.
- `quint test cached_test.qnt --main cached_test`: exit 0, 13 passing.
- `cachedSound`: `invFrameStackRefinement`, `invSound`, `invTamperRefused`, `invPopSeq`,
  `invFinalDepth`, `invFinalTop`: exit 0 (no violation). The four negated `reach*` witnesses:
  exit 1 (each shape reached).
- All 8 mutant modules on `invFrameStackRefinement`: exit 1, violation (caught).

Limit: the sampler picks among hand-built witnesses (4 programs, 8 counterexamples), so the passes
say nothing beyond that list. Evidence and reconciliation with the lead's model:
`research/sherlock-processor-analogy-20261110/d7-frames.md` section 6.
