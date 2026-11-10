# Soundness and Completeness Review: Sherlock Leaf as a Chip (`chipleaf`)

- **Design under test**: [`research/sherlock-processor-analogy-20261110/study.md`](../../research/sherlock-processor-analogy-20261110/study.md)
- **Context**: [`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md`](../../openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md) (`GP-1..GP-4`), [`models/sherlock/quint/leafSlots.qnt`](../../models/sherlock/quint/leafSlots.qnt)
- **Quint model**: [`qa/gemini-quint/chipleaf/chipleaf.qnt`](chipleaf.qnt)
- **Quint test suite**: [`qa/gemini-quint/chipleaf/chipleaf_test.qnt`](chipleaf_test.qnt)

---

## 1. Verdicts for the Design as Written

### **SOUND: NO (as written in `study.md`); YES (when strengthened with `C-BUS-1` .. `C-CORE-1`)**

The design as written in `study.md` is **UNSOUND**. While the central architectural premise—joining a small-register `CORE` to functional chip tables via a single halo2 `lookup_any` set-inclusion bus without multiplicities (`study.md` §1.2, §1.4, §2.7.2)—**can** be made sound without a multiset/LogUp/shuffle argument, `study.md` omits critical constraints needed to make stateful memory (`MEM` + `PAGE`) and linked frames a well-defined partial function of the bus key:

1. **Self-lookup without provider/request row separation (`study.md` §1.2, §1.4, §2.7.2)**: §1.2 stacks `CORE` and all chips in the same shared advice columns, and §1.4 specifies *"one `lookup_any` of the column set on itself, gated by a request selector"*. Without gating the table (RHS) side of `lookup_any` by a provider-only selector disjoint from `CORE` request rows, every `CORE` request tuple trivially matches **itself** in the shared columns (`cex_forged_bus_tuple`).
2. **`PAGE` and `MEM` share the `CELL` bus tag with no address-domain partition (`study.md` §1.3, §2.3, §2.4)**: Both `PAGE` and `MEM` serve `(CELL, addr, a, b, c)` (§1.3 line 69, §2.3 line 158, §2.4 line 177). `PAGE` opens rows against `pre_d.mem_root` (a depth-20 Merkle tree, §2.3 line 171), where unallocated addresses `addr >= base` (`pre_d.len`) hold empty/default leaves `(0, 0, 0)`. Because §2.3 does not constrain `index < base` on `PAGE` `CELL` rows:
   - After `CORE` appends a non-zero value `v` at `addr = base` in Step 0 (`len` becomes `base + 1`), Step 1's `LOAD base` (`base < len`) can be served by a `PAGE` row `(CELL, base, 0, 0, 0)` opened against `pre_d.mem_root` instead of `MEM` row 0 (`cex_stale_memory_read`).
   - Conversely, if Step 0 appends `0` at `addr = base`, the `APPEND` request `(CELL, base, 0, 0, 0)` can be answered by `PAGE`, leaving `MEM` row 0 at `addr = base` completely unconstrained to hold any forged value `77` for subsequent `LOAD`s and `post_d.mem_root` (`cex_wrong_chip_answer`).
3. **Unrequested `MEM` rows, non-causal `a < len` checks, and `MEM` row replay (`study.md` §1.3, §1.4, §2.4)**: Because set inclusion (`Requests ⊆ Providers`) is one-directional and has no multiplicities:
   - Active `MEM` rows at `i >= len_post - base` need no matching `APPEND` request and can pollute `post_d.mem_root` (`cex_unrequested_chip_row`).
   - If `len` advances by more than 1 per `APPEND` (or if `a < len` is checked against end-of-step/end-of-leaf `len` rather than the sub-step's current `len_t`), `LOAD` can read an unrequested `MEM` row that was never written (`cex_unwritten_cell_read`).
   - If two `APPEND`/`PUSH` micro-ops emit the same address `addr` and value (because `len` is not strictly chained across every individual `APPEND` sub-step), a single `MEM` row satisfies **both** `APPEND` requests under set inclusion, collapsing two writes/pushes into one (`cex_row_replay_across_steps`).
   - If `MEM` does not enforce `addr_i == base + i` with a fixed row-index column, two active `MEM` rows can share one `addr` with different values (`cex_duplicate_mem_addr`).
4. **Mutually inconsistent linked-stack protocol between §1.3 and §2.4 (`study.md` §1.3, §2.4, §2.7.3)**: §1.3 caches `top` in a register and defines `PUSH` as `APPEND` of `top` with link `fp` and `POP` as `LOAD fp` (where `fp` points to the caller frame *below* `top`). However, §2.4 (lines 181, 185) and §2.7.3 state that every rule that rewrites the top frame appends to `MEM` *"with the same link"* and that `POP` *"loads `fp` and sets `fp` to the stored link"*. When a top-frame rewrite appends `(new_top, link = fp)` at `len` and updates `fp := len`, the subsequent `POP` (`LOAD fp`) loads the **rewritten top frame** instead of the caller frame below it, and orphans any previously pushed frame (`cex_stack_top_rewrite_corrupt`).
5. **Mid-step leaf termination when a chip fills (`study.md` §1.5, §2.1, §2.7.7)**: §1.5 and §2.7.7 state that a leaf ends when `S` steps are done or when a chip is full. Each step spans `H_core = 8` rows with multiple micro-ops (§2.1). Without a step-atomic `enable_j` constraint requiring all micro-ops of an enabled step to complete within chip capacity, a leaf can stop mid-step after `APPEND` increments `len` but before `MOVE` updates `pc`/`acc` (`cex_mid_step_leaf_end`).

---

### **COMPLETE: NO (as written in `study.md`)**

The design as written in `study.md` is **INCOMPLETE** for three independent reasons:

1. **Contradictory `mem_root` Merkle geometry between `PAGE` (§2.3) and `MEM` (§2.4)**:
   - `study.md` §2.3 (lines 163, 170–171) requires every `PAGE` `CELL` row to open a depth-20 Merkle path against `pre_d.mem_root` (`20 x 25 = 500` `HASH` rows per cell).
   - `study.md` §2.4 (lines 195–196) budgets `MEM`'s `mem_root` update for 6 to 12 appended tuples per step at only *"about 8 permutations per step, 200 rows"* (~1 permutation per tuple, i.e., a flat hash-chain accumulator).
   - A flat hash-chain `post_d.mem_root` produced by `MEM` in leaf $L$ cannot be opened by a depth-20 Merkle path in `PAGE` in leaf $L+1$. Conversely, updating a depth-20 Merkle tree in `MEM` requires opening the insertion frontier against `pre_d.mem_root` and recomputing 20 levels (which exceeds `N_hash` as budgeted in §4.1).
2. **Per-step chip capacity exhaustion and unprovable valid steps (`study.md` §1.5, §2.2, §2.3, §6 E0)**:
   - §1.5 acknowledges: *"A step that needs more of one chip than a whole leaf has cannot be proven; the heights are chosen above the per-step maximum of every rule (E0)"*, and §2.2 notes that whether the 108 Sherlock rules decompose into a bounded sequence of the 8 micro-ops of §1.3 is unproven (`E0(c)`). In our model (`completenessSingleStepExceedsMem`), any rule whose single-step appends exceed `N_MEM` has no accepting witness.
3. **Unresolved `GP-1..GP-4` requirements (`study.md` §2.3, §3, §5; `general-purpose.md`)**:
   - `PROG` canonicality is removed from the leaf circuit and deferred to an unspecified admission claim (`study.md` §2.3 line 167, §2.7.4 `[OPEN: its form]`).
   - Only `Halted` u128 publication is wired today (`general-purpose.md` GP-1 line 18), and cross-run composition (`GP-2`), foreign proofs (`GP-3`), and DAG/tree PCD (`GP-4`) remain unsupported because the node only verifies one fixed leaf key in a linear chain (`study.md` §5 line 326).

---

## 2. Answer to the Central Question: When Is Set Inclusion Without Multiplicities Sound?

In halo2 (`MP:plonk/lookup.rs:40-66`), `lookup_any` proves **set inclusion** ($\text{Requests} \subseteq \text{Providers}$), not multiset equality.

For stateless chips (`DEC`, `ALU`, `HASH`), set inclusion is sound as soon as every active provider row is constrained to satisfy a **deterministic function** $f(\text{chip}, \text{key}) = (a, b, c)$ and the RHS of `lookup_any` contains only active provider rows (`C-BUS-1`, `C-ALU-1`, `C-HASH-1`). Replaying a stateless row across steps simply reuses a true equation $f(k) = v$.

For **`MEM`, `PAGE`, and linked frames/tasks**, `CORE` issues **both** writes (`APPEND`/`PUSH`) and reads (`LOAD`/`POP`) as lookup requests `(CELL, addr, a, b, c)` into the union of `PAGE` and `MEM`. Normally, a lookup request `APPEND (CELL, addr, x) ∈ Providers` only proves that `(CELL, addr, x)` appears *somewhere* in the provider table—it does not by itself prevent:
- `PAGE` from answering the `APPEND` or serving a stale pre-leaf value on `LOAD`,
- two `MEM` rows from sharing the same `addr` with different values,
- an active `MEM` row from existing without any `APPEND` request, or
- two `APPEND` requests from replaying the same `MEM` row.

**However, set inclusion WITHOUT multiplicities becomes completely sound for `MEM`, `PAGE`, and linked frames if and only if the circuit enforces the following bijection and domain-separation constraints (`acceptsHardened` in `chipleaf.qnt`):**

1. **`C-BUS-1` (Provider-Request Disjointness & Chip-Tag Binding)**:
   - The RHS of `lookup_any` is gated by `q_prov = sum_{c} q_active_c` (disjoint from `q_req` in `CORE`), where each chip region $c$ fixes `bus.chip` to its allowed tag (`DEC`, `PROG | CELL`, `CELL`, `ALU`, `HASH`), plus one fixed inactive row `NO_REQ = ("NONE", 0, 0, 0, 0)`.
2. **`C-PAGE-1` (`PAGE` vs `MEM` Domain Separation on `CELL`)**:
   - Every active `PAGE` row with `tag == "CELL"` must satisfy **`0 <= idx < pre.len`** (`idx < base`, checked via `ALU`/`RANGE`) and open against `pre.memRoot`; every active `PAGE` row with `tag == "PROG"` must open against `pre.progRoot`.
3. **`C-MEM-1` (Fixed Contiguous `MEM` Address Column & Active Prefix)**:
   - `MEM` has $N_{\text{mem}}$ rows indexed by a fixed column `rowIdx = 0, 1, ..., N_MEM - 1` with **`addr_i == base + i`** (where `base == pre.len` is copy-constrained from `slot[0].pre.len`).
   - Row $i$ is active on the bus (and folded into `post.memRoot`) **if and only if `i < post.len - pre.len`**, and `0 <= post.len - pre.len <= N_MEM`.
4. **`C-MEM-2` (Sub-step `len` Bijection with `APPEND`/`PUSH`)**:
   - Across all micro-op sub-slots $t$ in the leaf, `len` is unchanged (`postRegs.len == preRegs.len`) unless the enabled micro-op is `APPEND` or `PUSH`, in which case it emits a bus request with **`req.chip == "CELL"` and `req.key == preRegs.len`** and sets **`postRegs.len == preRegs.len + 1`**.
5. **`C-MEM-3` (Sub-step Causal Read Bound on `LOAD`/`POP`)**:
   - Every enabled `LOAD` or `POP` micro-op at sub-slot $t$ checks **`0 <= req.key < preRegs.len`** against that sub-slot's **current** `preRegs.len` (not the step-end or leaf-end `len`).
6. **`C-STACK-1` (Linked Frame/Task Stack Discipline)**:
   - Fix the stack invariant to §1.3's register-cached top frame (`topVal` in register, `fp` = address in `MEM`/`PAGE` of the caller frame *below* `top`):
     - `PUSH`: emits `(CELL, pre.len, KIND_FRAME, pre.topVal, pre.fp)` and sets `post.fp == pre.len`, `post.len == pre.len + 1`.
     - `POP`: emits `(CELL, pre.fp, KIND_FRAME, post.topVal, post.fp)` with `0 <= pre.fp < pre.len`.
     - `REWRITE_TOP`: updates `topVal` in `CORE` registers **only** (`post.fp == pre.fp`, `post.len == pre.len`, no `MEM` append). *(Alternatively, if `REWRITE_TOP` appends to `MEM` at `fp`, `top` must cache `below_fp` and `POP` must load `top.below_fp` rather than `fp`.)*

### Why `C-BUS-1` .. `C-STACK-1` Make Set Inclusion Sound (Proof Sketch)

Let $K = \text{post.len} - \text{base}$. By `C-MEM-2` and induction over the enabled micro-op sub-slots, there are exactly $K$ enabled `APPEND`/`PUSH` micro-ops in the leaf, and the $k$-th such micro-op ($0 \le k < K$) emits a `CELL` request with key $\text{addr} = \text{base} + k$.
- By `C-BUS-1` and `C-PAGE-1`, no `PAGE` row has `chip == "CELL"` and `key >= base`.
- By `C-MEM-1`, the only provider row in the entire circuit with `chip == "CELL"` and `key == base + k` is **`MEM` row $k$** (for $0 \le k < N_{\text{mem}}$).
- Therefore, set inclusion ($\text{Requests} \subseteq \text{Providers}$) applied to the $k$-th `APPEND`/`PUSH` request `(CELL, base + k, a_k, b_k, c_k)` **forces** $K \le N_{\text{mem}}$ and **uniquely fixes** `MEM` row $k$'s tuple to `(CELL, base + k, a_k, b_k, c_k)`.
- Because the $K$ `APPEND` keys $\text{base} + 0 < \text{base} + 1 < \dots < \text{base} + K - 1$ are strictly increasing, no two `APPEND`s can ever share a `MEM` row, and no active `MEM` row $k < K$ can be unrequested.
- Finally, any `LOAD` or `POP` at sub-slot $t$ (after $k_t$ appends, where $\text{preRegs.len} = \text{base} + k_t$) satisfies $0 \le \text{key} < \text{base} + k_t$ by `C-MEM-3`:
  - If $\text{key} < \text{base}$, the unique provider is `PAGE(CELL)`, authenticated against `pre_d.mem_root`.
  - If $\text{base} \le \text{key} < \text{base} + k_t$, the unique provider is `MEM` row $k = \text{key} - \text{base} < k_t$, whose value was already uniquely fixed by the $k$-th `APPEND`/`PUSH` at an earlier sub-slot $< t$.

---

## 3. Numbered Findings, Counterexample Witnesses, and Required Fixes

Every finding below is mechanized in [`qa/gemini-quint/chipleaf/chipleaf.qnt`](chipleaf.qnt) and verified in [`qa/gemini-quint/chipleaf/chipleaf_test.qnt`](chipleaf_test.qnt). For each counterexample witness `cex_*`:
- `acceptsAsWritten(cex_*) == true` (accepted by the design as written in `study.md`),
- `isSoundWitness(cex_*) == false` (diverges from the reference semantics `refPost` / `refRules`), and
- `acceptsHardened(cex_*) == false` (refused once the design change is applied).

| # | Finding | `study.md` Section | Counterexample Witness (`chipleaf.qnt`) | Mutant Module | Required Design Change |
|---|---|---|---|---|---|
| **F1** | **Forged bus tuple via ungated self-lookup**: `lookup_any` of the 5 shared bus columns on themselves gated only by a request selector lets every `CORE` request row match itself (or an untagged row) without any chip constraint holding. | §1.2 (ll. 43–45), §1.4 (ll. 81–85), §2.7.2 | `cex_forged_bus_tuple` | `chipleaf_gap_forged_bus` | **`C-BUS-1`**: Gate the RHS of `lookup_any` by a disjoint provider selector `q_prov = sum_c q_active_c` (`q_req * q_prov == 0`), and bind `bus.chip` on each chip region $c$ to its fixed tag. |
| **F2** | **Unrequested `MEM` row polluting `post_d.mem_root`**: Set inclusion does not prevent active `MEM` rows at indices $i \ge \text{len}_{\text{post}} - \text{base}$ that no `APPEND` requested; folding them into `post_d.mem_root` corrupts committed memory. | §1.4 (ll. 87–92), §2.4 (ll. 177–196) | `cex_unrequested_chip_row` | `chipleaf_gap_unrequested_mem` | **`C-MEM-1`**: Constrain `MEM` row $i$'s active selector `m_i == 1 iff i < post.len - pre.len` (a prefix of length `post.len - pre.len`), and fold only active rows into `post_d.mem_root`. |
| **F3** | **Request answered by the wrong chip (`PAGE` hijacks `MEM` `APPEND`)**: Both `PAGE` and `MEM` serve tag `CELL`. When `CORE` appends `0` at `addr = base`, a `PAGE` row opening the unallocated empty leaf `(CELL, base, 0, 0, 0)` in `pre_d.mem_root` satisfies the `APPEND`, leaving `MEM` row 0 free to hold a forged value `77` read by later `LOAD`s. | §1.3 (ll. 69–70), §2.3 (ll. 158, 163), §2.4 (ll. 177–185) | `cex_wrong_chip_answer` | `chipleaf_gap_wrong_chip` | **`C-PAGE-1`**: Enforce `0 <= idx < pre.len` (`idx < base`) on every active `PAGE` row with `tag == "CELL"` (or use distinct bus tags `OLD_CELL` vs `NEW_CELL` selected by `addr < base`). |
| **F4** | **Stale memory read via `PAGE`/`MEM` overlap**: After Step 0 appends `10` at `addr = base` (`len = base + 1`), Step 1's `LOAD base` (`base < len`) matches a `PAGE` row `(CELL, base, 0, 0, 0)` opened against `pre_d.mem_root`, reading the stale pre-leaf value `0` instead of `10`. | §2.3 (ll. 158–168), §2.4 (ll. 177–187) | `cex_stale_memory_read` | `chipleaf_gap_stale_read` | **`C-PAGE-1`**: Enforce `0 <= idx < pre.len` on all `PAGE` `CELL` rows so addresses `>= base` can only be served by `MEM`. |
| **F5** | **Read of never-written cell**: If `len` advances by more than 1 per `APPEND` (or if `LOAD a` checks `a < len` against step-end/leaf-end `len` instead of current sub-step `len_t`), `LOAD` reads an unrequested `MEM` row at `base + 1` holding prover-chosen garbage `88`. | §1.3 (ll. 69–70), §2.4 (ll. 184–187) | `cex_unwritten_cell_read` | `chipleaf_gap_unwritten_read` | **`C-MEM-2` & `C-MEM-3`**: Enforce `postRegs.len == preRegs.len + 1` with `req.key == preRegs.len` on `APPEND`/`PUSH` (`postRegs.len == preRegs.len` otherwise), and check `0 <= req.key < preRegs.len` at the `LOAD`/`POP` sub-slot. |
| **F6** | **Wrong `ALU` result and forged overflow/comparison flag**: `ALU` packs the output flag into `op` (`key`) in `(ALU, op, x, y, z)`. Without 128-bit `RANGE` limb checks and flag/output functional constraints on every active `ALU` row, an `ALU` row serves wrong `(z, flag)`. | §1.3 (l. 72), §2.5 (ll. 200–208) | `cex_wrong_alu_result` | `chipleaf_gap_wrong_alu` | **`C-ALU-1`**: On every active `ALU` row, enforce 128-bit `RANGE` checks on `x, y, z`, `z == evalAluVal(opKind, x, y)`, and `flagBit == evalAluFlag(opKind, x, y)`. |
| **F7** | **Wrong next `pc` / unbound `FETCH` and control transition**: `CORE` updates `pc` to `0` instead of the fetched instruction's `targetPc = 2` (or fetches from `key != pre.pc`). | §1.3 (ll. 68–76), §2.1 (ll. 128–132), §2.2 (ll. 138–149) | `cex_wrong_next_pc` | `chipleaf_gap_wrong_pc` | **`C-CORE-1`**: Bind `decClass == computeDecClass(pre)`, `FETCH` `req.key == pre.pc`, and `post.pc == insn.targetPc` (plus register frame conditions) per decoded control word. |
| **F8** | **Chip-table overflow (`MEM` capacity exceeded)**: A leaf advances `len` beyond `base + N_MEM` while `PAGE` answers the overflowed `APPEND` request, silently dropping writes beyond `N_MEM`. | §1.5 (ll. 100–107), §2.4 (l. 178), §2.7.7 | `cex_chip_table_overflow` | `chipleaf_gap_overflow` | **`C-MEM-1` & `C-PAGE-1`**: Enforce `0 <= post.len - pre.len <= N_MEM` and `idx < base` on `PAGE(CELL)` so any `APPEND` at `addr >= base + N_MEM` has no provider row and fails `lookup_any`. |
| **F9** | **Replay of a `MEM` row across steps**: Because set inclusion has no multiplicities, two `PUSH`/`APPEND` steps that emit the same `(CELL, base, KIND_FRAME, 10, 0)` tuple (when `len` fails to increment) both match the single `MEM` row 0, collapsing two stack pushes into one cell. | §1.4 (ll. 87–92), §2.4 (ll. 184–187) | `cex_row_replay_across_steps` | `chipleaf_gap_row_replay` | **`C-MEM-2`**: Strictly chain `len` across every individual `APPEND`/`PUSH` sub-slot (`req.key == preRegs.len` and `postRegs.len == preRegs.len + 1`) so no two `APPEND`s can emit the same key. |
| **F10** | **Leaf ends mid-step when a chip fills**: A step with 2 micro-ops (`APPEND` then `MOVE`) executes `APPEND` (incrementing `len` and updating `memRoot`) and then disables `MOVE` mid-step, committing a half-executed step in `post_d`. | §1.3 (ll. 64–76), §1.5 (ll. 101–104), §2.1 (ll. 135–136), §2.7.7 | `cex_mid_step_leaf_end` | `chipleaf_gap_mid_step` | **`C-CORE-1` (`stepAtomicMicroOps`)**: Make `enable_j` atomic at the **step-slot** boundary (`uopEn == sl.en` for all micro-ops in slot $j$). If a step would overflow a chip, disable the entire step slot $j$. |
| **F11** | **Duplicate `MEM` rows for one address**: Two active `MEM` rows (row 0 and row 1) both set `addr = base`, holding values `10` and `55`. Step 0's `APPEND` matches row 0 (`10`) while Step 1's `LOAD base` matches row 1 (`55`), breaking write-once determinism. | §1.4 (ll. 88–92), §2.4 (ll. 177–178, 186) | `cex_duplicate_mem_addr` | `chipleaf_gap_dup_mem_addr` | **`C-MEM-1` (`memAddressFixed`)**: Fix a constant column `rowIdx = 0, ..., N_MEM - 1`, copy-constrain `base == slot[0].pre.len`, and enforce `addr_i == base + rowIdx_i` on every `MEM` row. |
| **F12** | **Linked-stack corruption on top-frame rewrite (`§1.3` vs `§2.4`)**: Following §2.4 (*"a rewrite of the top appends with the same link; pop loads `fp` and sets `fp` to the stored link"*), rewriting `top` from `10` to `25` appends `(25, link = 0)` at `addr = 2` and sets `fp = 2`. The next `POP` (`LOAD fp`) loads `25` (the rewritten top frame) instead of the caller frame `100` at `addr = 0`. | §1.3 (ll. 56–57, 71), §2.4 (ll. 181, 184–185), §2.7.3 | `cex_stack_top_rewrite_corrupt` | `chipleaf_gap_stack_rewrite` | **`C-STACK-1`**: Either (a) keep `top` register-cached with `fp` = caller frame below `top` and perform top-frame rewrites in registers **without** appending to `MEM`, or (b) store `below_fp` in `top` and have `POP` load `top.below_fp` rather than `fp`. |
| **F13** | **Forged frame link / cell-kind confusion on `PUSH`/`POP`**: `PUSH` writes a forged `linkAddr = 1` (the address of a `KIND_DATA` cell with payload `50`) instead of `pre.fp = 0` (`KIND_FRAME` with payload `100`). `POP` follows the forged link and loads the data cell as a frame. | §1.3 (l. 71), §2.4 (ll. 176–185) | `cex_forged_frame_link` | `chipleaf_gap_forged_link` | **`C-STACK-1`**: Constrain `PUSH` to emit `(CELL, pre.len, KIND_FRAME, pre.topVal, pre.fp)` and `POP` to emit `(CELL, pre.fp, KIND_FRAME, post.topVal, post.fp)`. |
| **F14** | **Completeness gap — single-step chip demand vs fixed capacity & `mem_root` geometry mismatch**: A single rule appending 6–12 tuples (§2.4 l. 195) exceeds `N_MEM = 2`, and `MEM`'s 1-permutation-per-cell `mem_root` update (§2.4 l. 196) cannot produce the depth-20 Merkle tree required by `PAGE` (§2.3 l. 171). | §1.5 (ll. 104–106), §2.3 (l. 171), §2.4 (ll. 195–196) | `completenessSingleStepExceedsMem` | N/A (completeness) | Size every fixed chip table above the worst-case single-step demand across all 108 rules (`N_c >= max_r demand(r, c)`), and unify `mem_root` as an append-only Merkle tree with authenticated frontier updates in `MEM`. |

---

## 4. Invariants and Commands for the Lead to Run

### 4.1 Typecheck and Unit/Counterexample Test Suite

```bash
quint typecheck qa/gemini-quint/chipleaf/chipleaf.qnt
quint typecheck qa/gemini-quint/chipleaf/chipleaf_test.qnt
quint test qa/gemini-quint/chipleaf/chipleaf_test.qnt
```

### 4.2 Soundness Invariants on the Hardened Design (Expected: **PASS**)

All soundness invariants (`invSound`, `invTamperRefused`, `invStepsRange`, `invPrefix`, `invRules`, `invPost`, `invTerminalLast`, `invOutcome`, `invMemCap`) must **HOLD** on `chipleafHardened`, `chipleafS2`, and `chipleafS4`:

```bash
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleafHardened --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleafHardened --invariant invTamperRefused
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleafS2 --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleafS4 --invariant invSound
```

### 4.3 Non-Vacuity / Reachability Invariants on `chipleafHardened` (Expected: **FAIL / Witness Found**)

Each `reach*` invariant is negated so that `quint run` finds an accepting honest witness of that execution shape:

```bash
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleafHardened --invariant reachShort
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleafHardened --invariant reachTerminalShort
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleafHardened --invariant reachFull
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleafHardened --invariant reachPushPop
```

### 4.4 Unsoundness of the Design as Written & Per-Gap Mutants (Expected: **FAIL / Counterexample Found**)

Running `invSound` on `chipleafAsWritten` and on each of the 13 per-gap mutant modules must **FAIL** with the corresponding adversarial witness:

```bash
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleafAsWritten --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_forged_bus --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_unrequested_mem --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_wrong_chip --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_stale_read --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_unwritten_read --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_wrong_alu --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_wrong_pc --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_overflow --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_row_replay --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_mid_step --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_dup_mem_addr --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_stack_rewrite --invariant invSound
quint run qa/gemini-quint/chipleaf/chipleaf.qnt --main chipleaf_gap_forged_link --invariant invSound
```
