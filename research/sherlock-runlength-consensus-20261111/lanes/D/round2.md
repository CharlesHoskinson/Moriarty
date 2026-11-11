# Round 2 Cross-Review & Final Position — Lane D (Prover Throughput, Hardware Limits & Benchmarking Lead)

Study: `sherlock-runlength-consensus-20261111` (Round 2 Cross-Review)
Repository state inspected read-only: `/home/hoskinson/Projects/Moriarty` at commit `c57351dd43cec8cb9f2db55d8350c0388c1f65ab` (`moriarty-authoring-beta-0.1.0-beta.1-1569-gc57351dd4`, 2026-11-10).
Sibling Round 1 reports reviewed in full: `lanes/{A,B,C,E}/round1.md` (no `round2.md` file was listed or read).

---

## Section 1: What Lane D Now Accepts (and Updates to Lane D's Round 1)

### 1.1 Cross-Lane 4-of-5 Convergence on Tail-Buffered Dense `M3` (`Q2(d)`)
Lanes A (`M3-Dense`, `lanes/A/round1.md:421-462`), B (`Carry-Buffered Arena`, `lanes/B/round1.md:190-235`), D (`Tail-Buffered Dense M3`, `lanes/D/round1.md:116-142`), and E (`M3-Tail64`, `lanes/E/round1.md:210-255`) independently converged on the exact same structural fix to `D3` (`crates/mori-chip/src/m3.rs:50-55,132,209`):
- Advance `len` strictly by actual logical appends `k_used` (`0` on non-allocating steps/leaves; **never** pad `len` to `base + 8` on leaf close).
- Carry the `0..7` records of the active open 8-cell segment `q = len >> 3` across leaf boundaries in `pre_d`/`post_d`.
- Flush a 100%-dense 8-cell segment into the top Merkle tree **if and only if** `floor(len_post / 8) > floor(len_pre / 8)`.
- Result: zero address burn on pure-compute loops, zero dead padding gaps, and **100% partition-independent physical commitments `(mem_root, len, tail_d)`** at every step height `h`.

### 1.2 Specific Technical Improvements Accepted from Sibling Lanes

1. **Accepted from Lane E (`Exploit E-12`, `lanes/E/round1.md:115-128`) — Retraction of Carbon's `D-1` Cons-Linked Slices:**
   - In Round 1, Lanes A (`lanes/A/round1.md:461`), C (`lanes/C/round1.md:187`), and D (`lanes/D/round1.md:124`) paired widened/dense `M3` with Carbon's `D-1` cons-linked slices (`openspec/changes/sherlock-vm-consensus-20261107/dissent.md:201`).
   - Direct source verification of `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime/owner_adt_runtime.rs:1430-1454` (`walk_path_from` calling `pre.field_at(&cur, &fidx)` at `fields.start.0 + fidx` in **one single step**), `owner_adt_runtime.rs:675-679` (`BuildCell`), and `crates/mori-core/src/sherlock/helper_machine/families/environment.rs:212-216` (`Append.cell` indexing `src.start + *src_cursor` in **one step**) confirms Lane E's `Exploit E-12`: under backward-linked cons slices, random-access projection of field `0` on an `m`-field constructor requires `m` sequential `PAGE` dereferences inside **1 step**. Whenever `m > N_page` (`N_page = 4..8`), the fixed-height leaf circuit becomes unsatisfiable (`GP-1` completeness failure), while changing projection to an `O(m)`-step loop violates `SVM-001`/`SVM-011` step-for-step equivalence with `mori-core`.
   - **Lane D update:** We retract Carbon's `D-1` cons-linked slices and preserve `O(1)` contiguous `start + k` indexing directly over dense `M3-Tail64` (see next item and §2.4).

2. **Accepted from Lane B (`lanes/B/round1.md:205-232`) & Lane E (`lanes/E/round1.md:215-220`) — Moving `FRAMES` and `TASKS` Out of `MEM` into `D7` Poseidon Hash-Chain Stacks (`fp_hash`, `tp_hash`):**
   - Keeping top-of-stack `top` and `task` in core registers (`design.md:100` `D7`, `0` hash rows on `RewriteTop`) while replacing `MEM`-backed `fp`/`tp` pointer chains with rolling Poseidon commitments `fp_hash = H(DOM_FRM, frame_words..., prev_fp_hash)` and `tp_hash = H(DOM_TSK, task_words..., prev_tp_hash)` (with `frame_count` and `task_count` enforced `== 0 <-> hash == 0`) yields three decisive wins:
     - **(a) Preserves contiguous `start + k` in 1 unified dense `MEM` arena:** Verified in `environment.rs:201-284,399-416`, `adt_constructor.rs:135-156`, `adt_build.rs:16-61`, and `owner_runtime.rs:3515-3565`: `Append` (`append_copy* -> append_tail`) allocates *only* `value_slots` (`1`/step), and `Build` (`XAdtHelperBuildField*`) allocates *only* `data_fields` (`1`/step) until `GBuildLast`, which pushes `data_field[m-1]` at `len` *before* `RawDataRow::Constructor` at `len + 1` (`owner_runtime.rs:3547-3559`). With `FRAMES`/`TASKS` in hash chains and zero `D3` leaf-close padding, **zero unrelated allocations ever interleave inside a `FIELDS` or `SLOTS` slice**!
     - **(b) Cuts stack `Pop` cost `16x` in leaf rows:** A stack `Pop` below `top` verifies 1 Poseidon chain preimage (`3–4` perms = `210–280` rows `ESTIMATE`) instead of a cold depth-64 `PAGE` Merkle walk (`4,620` rows `ESTIMATE`).
     - **(c) Boosts `S > 16` leaf amortization (`§3.5`):** Removing stack traffic from `MEM` drops `MEM` write frequency and cold `PAGE` reads, shrinking marginal rows/step from `~1,800` to `~1,460–1,735` (`ESTIMATE`) so `S = 256` fits comfortably in `k_leaf = 19` (`<= 524,282` rows) and `S = 512` fits in `k_leaf = 20` (`<= 1,048,570` rows).

3. **Accepted from Lane A (`lanes/A/round1.md:427-446`) & Lane E (`lanes/E/round1.md:221-226`) — 1-Word `tail_d` + Fixed `W = 15` Window (Replacing Lane D R1's 7-Word `tail_d[0..6]` Register Carry):**
   - Lane D Round 1 (`lanes/D/round1.md:119,132`) carried `7` record digests `tail_d[0..6]` directly in `pre_d`/`post_d` (`+3.5` perms/side + `~560` barrel-shift mux rows). Carrying a **single** depth-3 `Tree8` digest `tail_d` in `pre_d`/`post_d` and assigning a fixed 15-row `MEM` window `addr_k = 8 * seg + k` (`k = 0..14`, `off = len_pre & 7` carried-in cells + `m <= 8` new appends, `off + m <= 15`) keeps `D6` (`addr = base + k` with fixed `k`) verbatim, eliminates the 7-word state-digest expansion, and avoids barrel-shift mux gates.

4. **Accepted from Lane B (`lanes/B/round1.md:120-135`) — Exact Halo2 Usable-Row Formula (`2^k - 6`):**
   - Verified in `tools/profile/vendor/midnight-proofs-0.8.3/src/dev/cost_model.rs:370-398`: `nb_unusable_rows = cs.blinding_factors() + 1 = 6` at max degree `6`, so exact `k_leaf` usable ceilings are `8,186` (`k=13`), `16,378` (`k=14`), `32,762` (`k=15`), `65,530` (`k=16`), `131,066` (`k=17`), `262,138` (`k=18`), `524,282` (`k=19`), and `1,048,570` (`k=20`).

5. **Accepted from Lanes A & C (`lanes/A/round1.md:350-361`, `lanes/C/round1.md:108-125`) — Mirror Spec Text for `specs/sherlock-general-purpose/spec.md:7-15`:**
   - Added to §3.1 alongside `general-purpose.md:9,14,16` and `requirements.md:56`.

6. **Accepted from Lane E (`lanes/E/round1.md:35-128,258-284`) & Lane B (`lanes/B/round1.md:266-275`) — Soundness Hardening & Two-Mode Q3 Continuation Split:**
   - **Exploit `E-1` (`architecture.md:100`):** Self-joining `JOIN(D_i, D_i)` in a `GP-4` diamond DAG doubles `n_claims` (`2^k`), wrapping `u64` in 64 merges and `mod r` in 255 merges. Every additive node counter (`h_hi`, `n_claims`, `cum_steps`) MUST enforce an explicit `< 2^64` range check at every merge.
   - **Exploit `E-5` (`Aggregation.lean:159-214`, `aggregation.qnt:12-15`):** Verified in `clean/fork/Clean/Sherlock/Aggregation.lean:201,211-214`: `digest_inj` + `leaf_sound` + `Run.halt_unique` requires `post_d` at step `h` to be unique across leaf partitions, which `M3-Tail64` restores literally.
   - **Exploit `E-10` (`crates/mori-chip/src/alu.rs:6-9,111-125`) & `E-11` (`crates/mori-chip/src/rf.rs:34`):** Verified in `alu.rs:111-125` (`OP_LT` relies on inductive `StepInv` `x, y < 2^128`) and `rf.rs:34` (`NDIG = 29` omits `V0..V2`). Scratch registers `v0..v2` MUST be constrained to `0` at leaf boundaries (or digested), `M3-Tail64`'s `old_walk` MUST verify `old_leaf == EMPTY_SEG` on every flush, and every continuation epoch `k+1` MUST verify epoch `k` in-circuit back to `h_lo = 0` (`Init`) so `StepInv` holds inductively.
   - **Refinement to Lane D R1's `Q3` Continuation Design (`E-2, E-6, E-8, E-9`):** Lane D Round 1 (`lanes/D/round1.md:158-186`) used fuel-exhaustion `Suspended(chk_d)` + `WithFuel(in_0, fuel_a + fuel_b)` for cross-run continuations. As Lane E (`E-2`, `E-6`, `E-8`) and Lane C (`lanes/C/round1.md:228`) show, in `/1` (`SVM-009`) `initial_fuel` is observable in-machine (`owner_runtime.rs:1059-1068`), so splitting `fuel_0` into `fuel_a + fuel_b` alters `initial_fuel` and risks fuel-refresh or split-dependent state if misused. Because `M3-Tail64` makes `canon_state_d(h)` 100% partition-independent, **transparent `/1` cross-session pause/resume** should use Lane E/B's domain-separated **`SHK_CONT_V1` claim over `canon_state_d(h)`** threading `orig_input_d` untouched (`0` extra leaf rows, `0` `UNFOLD` overhead), while **address-free `ckpt_d` + lazy 1-level `UNFOLD`** (Lanes B/C/D) is retained specifically for **`SRC-021` genesis `val_d` lazy loading** and future **profile `/2` semantic `Await`/`Resume`** across transactions (§3.3).

7. **Accepted from Lanes A, B, C, E on `Q4` Prior Art & `Q5` Setup Breakdown:**
   - **Q4 additions:** Exact counter limits from Lane A (`Ziren` `(u16, u26)` -> `~2^39.7` insns; `Pico 3.1` `(u16, u25)` -> `2^39` cycles; `Airbender` `(u19, u19)` -> `2^36` cycles; `Binius64` `ValueIndex(u32)` -> `2^30` words; `TwistShout.pdf` §2.9.3 `O(|M|)`/shard barrier; `Nebula` `ePrint 2024/1605`); live verification of `sp1/dev` combining 48-bit intra-chunk septic-curve timestamps (`columns.rs`) with inter-chunk Poseidon2 `prev_merkle_root`/`merkle_root` (`public_values.rs`, Lane C); and 2024–2026 disclosed zkVM soundness bugs from Lanes B & E (`CVE-2026-62333`, `GHSA-g6qg-9j8p-7j87`, `GHSA-fh29-29h9-qm9h`, `CVE-2025-46723`, `GHSA-c873-wfhp-wx5m`, Stwo `claimed_sum`, Jolt `termination`/`memory_layout`, Cairo relocation aliasing).
   - **Q5 fixed-tail clarification (`runs.jsonl:87,648,649`):** Separating **warm cached `resident_pk` prove+verify tail** (`~42.3 s` node PK load in `A16-node-step` + `~45 s` `ESTIMATE` cached decider PK load + `70.35 s` decider prove + `2.86 s` `SRC-010` verify = **`~160 s` / `2.7 min` `ESTIMATE`**) from **full `G-C` QA gate tail** (`211.07 s` uncached `A16-fold3-decider` setup + `70.35 s` decider + `2.86 s` `A5` verify + `163.94 s` `A6` `SRC-020` ledger WASM `validate_call` = **`448.2 s` / `7.5 min` measured**) reconciles Lanes A/B/C/E (`~2.5–3.5 min`) and Lane D R1 (`~7.5 min`) identically.

---

## Section 2: What Lane D Rejects and Why

### 2.1 Reject Lane C's `Q2(d)` (`Wide M3` `TOP = 64` + `act[0]` Empty-Leaf Bypass Without Tail Buffering)
Lane C (`lanes/C/round1.md:17-20,165-195`) widens `TOP` to `64` and skips the `M3` walk when `act[0] == 0`, **but retains `len_post = base + 8` whenever a leaf appends `1..7` cells**. We reject this design on three concrete grounds:
1. **Violates `SRC-023` physical invariance and falsifies `Aggregation.lean:201-214` / `aggregation.qnt:12-15` (`Exploit E-5`):** A 2-step trace appending 1 cell per step produces `len = 16`, cell addresses `(8, 9)` under `[2]` vs `len = 24`, cell addresses `(8, 16)` under `[1, 1]`. In `clean/fork/Clean/Sherlock/Aggregation.lean:201,211-214`, `digest_inj` + `leaf_sound` + `Run.halt_unique` requires a single canonical `digest(s)` at step `h`; padding `1..7`-append leaves breaks `Premises.det` on `post_d` and blocks arbitrary `GP-4` parallel segment splitting unless host provers pre-agree on exact leaf cut boundaries.
2. **Leaves `SI-12` (`start + k`) broken and relies on broken `D-1` linked slices (`Exploit E-12`):** Any `Build` or `Append` slice crossing a `1..7`-append leaf close contains dead padding holes; Lane C's reliance on Carbon's `D-1` linked slices fails `owner_adt_runtime.rs:1430-1454` (`1`-step `field_at` on arity `m > N_page`).
3. **Wastes `TOP = 64` (`2^67` cells) row budget:** With dense `M3-Tail64`, `TOP = 61` (`8 * 2^61 = 2^64` cells) already gives a full 64-bit cell domain; `TOP = 64` adds `+420` `M3` rows and `+210` rows per `PAGE` read solely to absorb Lane C's worst-case `8x` padding waste on 1-step leaves.

### 2.2 Reject Carbon's `D-1` Cons-Linked Slice Encoding (Lanes A, C, and D-R1)
As proved in §1.2(1) from `owner_adt_runtime.rs:675-679,1430-1454` and `environment.rs:212-216`, replacing contiguous slices with cons-linked lists breaks 1-step `O(1)` field projection for constructor arity `m > N_page` (`Exploit E-12`), and is unnecessary once `M3-Tail64` removes leaf-close padding and `FRAMES`/`TASKS` move to Poseidon hash stacks.

### 2.3 Reject Lane B's Sub-64-Bit `B = 32` (`2^35`-Cell) Cap in Favor of `TOP = 61` (`2^64` Cells)
Lane B (`lanes/B/round1.md:195-210`) caps `mem_root` at `B = 32` (`2^35` cells) instead of `TOP = 61` (`2^64` cells) to save `(61 - 32) * 70 * (2 + N_page) = 20,300` leaf rows (`42,179` vs `62,479` rows at `N_page = 8`). We reject `B = 32` for two reasons:
1. **Zero `k_leaf`, Node, or Decider savings at `S = 16`:** Under Lane B's own `2^k - 6` formula (`cost_model.rs:370-398`), `32,762 < 42,179 < 62,479 <= 65,530`: **both `B = 32` and `TOP = 61` compile to the exact same `k_leaf = 16` (`65,536`) domain!** Because Halo2 proving is `88.4%` domain-proportional (`c0-first-profile.md:51`) and `E2` proved `node_rows` depends only on `k_leaf` and column counts (`e2-e4.md:25-35`), `B = 32` saves **`0` node rows**, **`0` decider rows**, and `< 0.4 s` (`< 0.6%`) per `68 s` step cycle at `S = 16`.
2. **Avoids a second sub-64-bit constant in `GP-001`:** `TOP = 61` (`len < 2^64`) aligns memory addresses with `SRC-004` (`h < 2^64`) so every counter in the system uses one uniform 64-bit non-wraparound invariant. *(Even at `S = 512`, `TOP = 61` still fits inside `k_leaf = 20` at `~860k–920k <= 1,048,570` rows `ESTIMATE`.)*

### 2.4 Reject Lane E's 4-Arena Split (`DATA, FIELDS, ENV, SLOTS` + `W_flush = 2`) as Mandatory Over Domain-Tagged 1-Arena `M3-Tail64` (`W_flush = 1`)
Lane E (`lanes/E/round1.md:215-248`) splits persistent memory into **4 separate depth-61 Merkle trees (`DATA, FIELDS, ENV, SLOTS`)**, carrying `12` arena words `(root_a, len_a, tail_d_a)` in `state_d` and provisioning **`W_flush = 2` depth-61 shared-path walks (`19,180` memory-write rows `ESTIMATE`)** because `adt_build.rs:40-50` appends `1` `FIELDS` cell and `1` `DATA` cell in the same step (`GBuildLast`). While sound, 4 separate Merkle trees are **strictly unnecessary**:
1. **1-Arena `M3-Tail64` + Hash-Stack `FRAMES`/`TASKS` ALREADY Guarantees Gap-Free `SI-12` Slices:** Our audit of all allocation sites in `crates/mori-core/src/sherlock/helper_machine/` (`environment.rs:201-284,399-416`, `adt_constructor.rs:135-156`, `adt_build.rs:16-61`, `owner_runtime.rs:2780-3017,3515-3565`) proves that `mori-core` **never** appends `SLOTS` or `FIELDS` interleaved with expression evaluation: `Append` copies the completed prefix + new value in tight consecutive `append_copy`/`append_tail` steps (allocating *only* `value_slots`), and `Build` copies `src` in tight consecutive `XAdtHelperBuildField` steps (allocating *only* `data_fields`, with `GBuildLast` pushing `data_field[m-1]` at `len` *before* `Constructor` at `len + 1`, and `Cons`/`Branch` pushing `[field_0, field_1, node]` at `[len, len+1, len+2]`). Thus every `SliceC(start, len)` is 100% contiguous (`start + k`) in 1 dense `MEM` arena under the monotone address refinement already modeled in `c6_phys_state.rs` (keeping logical `AllocCounters` in registers for `SVM-010` cap checks).
2. **Eliminates Simultaneous-Flush Livelock With `W_flush = 1` (Saving `~5,000` Leaf Rows at `S = 16` and `2x–3x` Walk Rows at `S = 256`):** Simultaneous-flush livelock in `GBuildLast` was an **artifact of splitting `DATA` and `FIELDS` into separate tail buffers** (`R_DATA = 7` and `R_FIELDS = 7` both hitting `8`!). In 1-arena `M3-Tail64`, `off in 0..7` plus `<= 3` appends in any single step (`Cons`/`Branch` = 3, `GBuildLast` = 2) gives `off + k_step <= 10 < 16`, crossing **at most 1** multiple of 8 (`W_flush = 1` = `13,860–14,350` M3 rows vs `19,180` in 4-arena). At `S = 256` (`k_leaf = 19`), 1-arena pools `B = 16` segment flushes dynamically across all heap kinds instead of over-provisioning worst-case walks across 4 separate trees. Tagging records as `H(kind_tag, a, b, c)` (`kind_tag in {1=DATA, 2=FIELD, 3=ENV, 4=SLOT}`, `0=PAD`) prevents cross-kind confusion at **0 row cost**. *(4-arena `M3-Tail64` remains a valid fallback only if Task 4.18 insists on 0-based per-arena physical indices rather than `c6_phys` monotone addresses.)*

### 2.5 Reject Specific `Q3` Flaws in Lanes A and C
1. **Reject Lane A's claim (`lanes/A/round1.md:495-499`) that GP-2 run-chaining must either expose $K$ checkpoint claims on-chain or wrap the guest in `prog_W`:** Adding one internal cut-elimination rule in `N` over domain-separated `SHK_CONT_V1` claims (agreed by Lanes B, C, D, and E) discharges all intermediate continuation claims inside recursion and emits **one** root `SHK_RUN` proof for the unmodified `prog_d` and `orig_input_d`, leaking zero intermediate digests on-chain (`C6` preserved).
2. **Reject Lane C's embedding of `quantum_steps` into `/1` `input_d` (`lanes/C/round1.md:319-337`) for same-key `/1` continuations:** Making `input_d` depend on `Q_0` changes `claim_d` across checkpoint schedules unless stripped, and `/1` (`SVM-009`) forbids `Suspended` outcomes. Under `SHK_CONT_V1` over deterministic `canon_state_d(h)` (§3.3), `orig_input_d` stays untouched (`SRC-021`) and any epoch split `[0, h_1), [h_1, h_2), ..., [h_{m-1}, N)` collapses to `[0, N)`.

### 2.6 Reject Concrete `Q4` & `Q5` Errors in Sibling Lanes
1. **Reject Lane C's `2L - 1` Binary-Tree Node Proof Count (`lanes/C/round1.md:501`):**
   - Lane C claimed `GP-4` binary tree folding of `L` leaves requires `L` leaf lifts + `L - 1` merges = **`2L - 1` `k = 20` proofs** (`87.6 s/leaf` at `P = 3`, slower than linear IVC).
   - **Refutation (`research/sherlock-gp-architecture-20261110/architecture.md:75`, `research/sherlock-speed-replan-20261108/tree.md:44`):** Run lift `R` verifies **one or two `U` proofs** (`U + U -> SEG`)! Therefore `L` leaves need **`ceil(L / 2)` `R`-lifts** + **`ceil(L / 2) - 1` `N`-merges** + `1` closing proof `C` = **`2 * ceil(L / 2) ≈ L` `k = 20` proofs** (half of `2L - 1`!). At `T1(b)`'s measured `43.83 s/node` (`P = 3`, `47.73 GB` RSS, `runs.jsonl:618`), binary tree proving achieves **`~45.3 s/leaf = 2.83 s/step` (`0.353 Hz`, `+46%` faster than serial, NOT `1.35x` slower)**!
2. **Reject Zero-Contention Single-Host Pipelining Assumptions (Lanes A `lanes/A/round1.md:510`, C `lanes/C/round1.md:500`, E `lanes/E/round1.md:343`) and Nominal-Only `273 GB/s` Citations:**
   - Measured unified LPDDR5X bandwidth is **`244 GB/s` R+W** (`122 GB/s` copy, `gpu.md:18`, `design.md:327`) vs `273 GB/s` nominal (`prover.md:15`). Because a 20-thread `k = 20` node is memory-bus bound (`d = 0.884`, `9.77/20` active cores, `c0-first-profile.md:51-59`), concurrent leaf proving on the same bus slowed the node from `61–65 s` to **`90.2 s` (`1.43x`)** in `E4` (`e2-e4.md:19`, `runs.jsonl:711`) and `1.69–1.74x` in `T1(b)` (`runs.jsonl:618`). At `S = 16`, sequential `leaf (~5.5 s) -> node (~63 s) = ~68.5 s` avoids contention and beats oversubscribed pipelining (`90.2 s`); pipelining pays off materially only at **`S >= 128`** with explicit `X925`/`A725` core pinning and a `1.25x–1.35x` bus-contention factor budgeted.
3. **Reject Lane C's Q4 `u64` `eid` Claim for `zkWasm` (`lanes/C/round1.md:420,452`) and Fix Lane E's `2.23 billion years` Unit Slip (`lanes/E/round1.md:351`):**
   - Live inspection of `https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/crates/zkwasm/src/circuits/mtable/allocator.rs` confirms `alloc_u32_state_cell` (`AllocatedU32Cell`), capping `zkWasm` continuations at **`2^32 - 1` steps** (`U64_CELLS = 1` is the Wasm `i64` value column `"mc9. value"`).
   - Arithmetic correction to `lanes/E/round1.md:351`: `2^60 * 61 s / (3.15576e7 s/yr) =` **`2.229 x 10^12` years (`2.23 trillion years`)**, not `2.23 billion years` (`1,000x` unit typo).

---

## Section 3: Lane D's Final Self-Contained Position on Q1 to Q5

### 3.1 Final Position on Q1: Exact `GP-1` & `GP-001` Wording, `2^64`, and Honest Meaning of "Unbounded"

#### 1. What `2^64` Actually Is (Why There Is No `2^64` Circuit)
- **Fixed small circuit domains:** Every Sherlock circuit has a constant power-of-two domain independent of run length: chip leaf `U` at `k_leaf = 13..16` for `S = 16` (`8,192..65,536` rows, `design.md:105,129`; up to `K_LEAF_MAX = 20` = `1,048,576` rows for larger `S`, `requirements.md:39`), recursive node `SHKIVC06` / `N` at `k = 20` (`823,466` rows over `E8`, `design.md:130`), and `RunDecider` at `k = 20` (`929,166` ordinary rows / `1,032,467` table rows, `review.md:213`). A degree-`2^64` KZG polynomial would require `590.3 Exabytes` per column (`ESTIMATE`) and never exists.
- **`2^64` is a ~24–32-row arithmetic non-wraparound guard (`SRC-004`):** In `crates/mori-recursion/src/ivc.rs:79,363-376` and `clean/fork/Clean/Sherlock/Aggregation.lean:1196-1213` (`src004_height_bridge`, `unchecked_prev_wraps_to_genesis`), `prev_h`, `steps`, and `h = prev_h + steps` are range-checked `< 2^64` so `h` cannot wrap modulo the 255-bit BLS12-381 scalar prime `r` back to `0` (`is_genesis`). All `SRC-004` + `SRC-005` checks combined cost `73` node rows (`652,364 -> 652,437`, `qa/sherlock/decisions/speed-levers.json:31`).
- **Physical impossibility of `2^64` steps:** Proving `2^64` steps takes `2.41 x 10^12` years (`~175x` age of the universe) at `0.24 Hz` (`ESTIMATE`), and even unproved sequential execution at `1 step/clock` on a `4.004 GHz` Cortex-X925 takes `146.0` years (`ESTIMATE`, §3.5.4).

#### 2. Honest Engineering Meaning of "Unbounded Run Length and Memory"
1. **Key & Verifier Universality:** One fixed key set and one deployed contract verify any program on any input for any outcome without re-keying, new ceremonies, or capacity ladders (`O(1)` circuit domains, `6,432 B` final proof, `O(1)` on-chain verification).
2. **Zero Sub-64-Bit Architectural Bottlenecks (`< 2^64` Counter Guard Only):** Step counters (`h < 2^64`, `SRC-004`), memory frontiers (`len < 2^64`, `M3-Tail64` `TOP = 61`), and `GP-4` DAG claim counters (`n_claims < 2^64`) share a uniform 64-bit non-wraparound width unreachable by physical hardware; non-allocating steps burn `0` memory cells; and cross-session / multi-run continuations compose via `SHK_CONT_V1` / `GP-2` + `GP-4`.
3. **Linear `O(n_steps)` Prover Work + Explicit Tested Bounds (`SAS-001`):** Prover time and `SRC-019` bundle bytes grow linearly `O(n_steps)` with `O(1)` streaming prover RSS (`~17.4–20 GB` sequential), bounded in practice only by wall-clock budget and host storage, with actual tested step/cell targets recorded in `SAS-001` (`requirements.md:65`).

#### 3. Exact Drop-In Replacement Text (Zero Narrowing of `GP-1`..`GP-4`)

**A. `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md` (`GP-1` intro `:9`, run-length bullet `:14`, and acceptance `:16`; lines `:10-13` and `GP-2..GP-4` `:20-58` unchanged):**
```markdown
One fixed key set proves the execution of every program of the Sherlock language, on every input, for any finite run length and every outcome. A new program or a longer run needs no new key, no new contract and no new ceremony.

- Run length and memory scale linearly without re-keying or fixed-capacity circuits. Fixed-size leaf proofs (`S` steps) fold recursively into one constant-size proof (`O(n_steps)` prover work, `O(1)` circuit size, final proof size and on-chain verification); no circuit is sized to the total run length. Neither the leaf nor the memory commitment imposes a step, segment or cell ceiling narrower than the 64-bit non-wraparound counter guard (`< 2^64`, enforced in-circuit by `SRC-004` solely to prevent modular field overflow, far beyond physical reach); non-allocating steps consume zero memory addresses, practical run length on a host is bounded only by prover wall-clock time and host storage, and computations also checkpoint and compose across runs via GP-2 and GP-4.

Acceptance: one key set and one deployed verifier contract accept proofs of at least three programs that the key set was not built from, including one multi-leaf, multi-segment program whose live state and allocations exceed one leaf's carriers (meeting the routine `>= 512`-step unseen gate and the `>= 4,096`-step multi-segment soak target), one run with input, and one run of each outcome kind; counter non-wraparound up to `2^64 - 1` is verified by boundary negative tests (`SRC-004`) and formal proof (`SRC-017`).
```

**B. `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:56` (`GP-001` EARS Row):**
- **Requirement cell:**
```markdown
GP-1 universal VM. One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for any finite number of executed steps within the 64-bit non-wraparound counter guard (`1 <= n_steps < 2^64`, `SRC-004`) and across authenticated continuation runs (`GP-002`, `GP-004`), and for every outcome (`Halted(v)` for every value type, as a digest of a canonical value encoding; `Rejected(code)`; `Exhausted(kind)`) by folding fixed-size leaf proofs linearly in the executed step count into one constant-size proof; no circuit SHALL scale its domain with total run length, and neither the leaf nor the memory commitment SHALL impose a step, segment or cell ceiling narrower than `< 2^64` (non-allocating steps consume zero memory addresses, and state larger than one leaf's carriers runs against committed memory); the public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants; capacity SHALL NOT depend on the key; a new program or longer run SHALL need no new key, contract or ceremony (`SAS-001` records empirically tested bounds separately from the 64-bit counter guard). When a supported run exceeds one leaf's step, storage or hashing capacity, the prover SHALL continue through authenticated memory, recursive folding, and/or authenticated continuations under the same key set, bounded in circuit only by the 64-bit counter guard (`SRC-004`) and in practice only by prover time and host storage.
```
- **Acceptance check cell:**
```markdown
One key set and one deployed verifier contract accept proofs of at least three programs the key set was not built from, including one multi-leaf, multi-segment program exceeding one leaf's carriers (`293–320`-step seeded `G-C` suite, `>= 512`-step unseen multi-segment program in `< 2 h`, and `>= 4,096`-step / `>= 512`-cell multi-segment & continuation soak in `< 6 h`), one run with input, and one run of each outcome kind, while `SRC-004` raw-witness boundary tests and `SRC-017` Lean/Quint proofs verify non-wraparound up to `2^64 - 1` (row A33).
```

**C. `openspec/changes/sherlock-vm-consensus-20261107/specs/sherlock-general-purpose/spec.md:7-15`:**
```markdown
### Requirement: GP-001 Universal VM
One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for any finite number of executed steps within the 64-bit non-wraparound counter guard (`1 <= n_steps < 2^64`, `SRC-004`) and across authenticated continuation runs (`GP-002`, `GP-004`), and for every outcome (`Halted(v)` for every value type as a digest of a canonical value encoding, `Rejected(code)`, `Exhausted(kind)`) by folding fixed-size leaf proofs linearly in the executed step count into one constant-size proof. No circuit SHALL scale its domain with total run length, and neither the leaf nor the memory commitment SHALL impose a step, segment or cell ceiling narrower than `< 2^64`. The public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants. Capacity SHALL NOT depend on the key: state larger than one leaf's carriers runs against committed memory without burning addresses on non-allocating steps. A new program or longer run SHALL need no new key, contract or ceremony (`SAS-001` records tested step bounds).

When a supported run exceeds one leaf's step, storage or hashing capacity, the prover SHALL continue through authenticated memory, recursive folding, and/or authenticated continuations under the same key set, bounded in circuit only by the 64-bit counter guard (`SRC-004`) and in practice only by prover time and host storage.

#### Scenario: Programs the keys were not built from
- **WHEN** one key set and one deployed verifier contract receive proofs of at least three programs the key set was not built from, including a multi-leaf, multi-segment program exceeding one leaf's carriers (`>= 512` steps in routine qualification and `>= 4,096` steps / `>= 512` cells in endurance soak), one run with input, and one run of each outcome kind.
- **THEN** every proof is accepted with no new key and no new contract, and raw-witness negative tests refuse step or address wraparound at `2^64` (row A33).
```

---

### 3.2 Final Position on Q2: Domain-Tagged 1-Arena `M3-Tail64` (`TOP = 61`, `W = 15`, `W_flush = 1`) + `D7` Poseidon Hash-Chain Stacks

#### 1. Specification of Recommended Design (`Q2(d)`)
1. **Stacks (`FRAMES`, `TASKS`) in `D7` Poseidon Hash Chains (`fp_hash`, `tp_hash`):**
   - `top` and `task` remain in core registers (`0` hash rows on `RewriteTop`, `design.md:100` `D7`).
   - `PushFrame` updates `fp_hash' = Poseidon(DOM_FRM, frame_lanes..., fp_hash)` and `frame_count += 1`; `PopFrame` witnesses `(below_frame_lanes, prev_fp_hash)`, asserts `Poseidon(...) == fp_hash`, sets `top := below_frame`, `fp_hash := prev_fp_hash`, `frame_count -= 1`, and enforces `frame_count == 0 <-> fp_hash == 0` (`tp_hash` / `task_count` identical).
2. **Persistent Append-Only Heap (`DATA, FIELDS, ENV, SLOTS`) in Domain-Tagged 1-Arena `M3-Tail64` (`TOP = 61`, `2^64` Dense Cells):**
   - Exact dense cell frontier `len = 8 * seg + off` (`len < 2^64`, `seg = len >> 3 < 2^61`, `off = len & 7 in 0..7`; genesis reserves segment `0` so `len_0 = 8, seg = 1, off = 0, mem_root = empty_top(61), tail_d = EMPTY_SEG_ROOT`).
   - Every record hashes `rec = H(kind_tag, a, b, c)` with `kind_tag in {1=DATA, 2=FIELD, 3=ENV, 4=SLOT}` (`0=PAD`, preserving `D3`/`D8`).
   - **State digest (`32` words = `16` Poseidon perms = `1,120` rows/side):** `state_d = Poseidon(DOM_STATE_V1, w_0..w_26, fp_hash, tp_hash, len, mem_root, tail_d)`, with scratch registers `v0..v2` constrained `== 0` at leaf boundaries (`Exploit E-11` closed).
   - **Leaf `MEM` window (`W = 15` rows at `addr_k = 8 * seg + k`, `k = 0..14` at `S = 16`):** Carries `off` existing open-segment cells (`pinned to pre.tail_d`) + `m` new appends (`0 <= m <= 8`, `total = off + m <= 15`).
     - If `total < 8` (`flush == 0`): `post.mem_root == pre.mem_root`, `post.tail_d = seg_root(rec[0..total-1], R_PAD...)`, `post.len = 8*seg + total` (**`0` segments burned on 0-append leaves!**).
     - If `8 <= total <= 15` (`flush == 1`): `old_walk` proves slot `seg` equals `EMPTY_SEG` in `pre.mem_root` (`Exploit E-11` closed), `new_walk` writes `seg_root(rec[0..7])` (all 8 real written cells, **zero padding cells ever enter `mem_root`**) at `seg < 2^61`, `post.tail_d = seg_root(rec[8..total-1], R_PAD...)`, and `post.len = 8*seg + total < 2^64`.
   - **Reads (`D2` & `D6` preserved):** `PAGE` opens `addr < 8 * seg` (`64` perms = `4,480–4,620` rows); `MEM` serves hot tail reads `8 * seg <= addr < running_len` in `0` Merkle walks.
   - **Native `SI-12..SI-14` (`D-1` & `Exploit E-12` closed):** Because `FRAMES`/`TASKS` are in hash chains and `Append`/`Build` never interleave other allocations (`environment.rs:201-284`, `adt_build.rs:16-61`, `owner_runtime.rs:3547-3559`), every `FIELDS` and `SLOTS` `SliceC(start, len)` is 100% gap-free contiguous (`start + k`) for arbitrary arity `m`, and any single step appends `<= 3` cells (`off + 3 <= 10 < 16` -> `W_flush = 1` is 100% livelock-free!).

#### 2. Quantitative Comparison Matrix (`Q2(a)`–`Q2(d)`, `S = 16` Baseline, `ESTIMATE` vs Measured `E8`)
*(Usable rows at degree 6: `k=14 -> 16,378`, `k=15 -> 32,762`, `k=16 -> 65,530`, `k=20 -> 1,048,570`, `cost_model.rs:370-398`; `E8` measured anchor: `6,511` leaf rows, `823,466` node rows, `929,166` / `1,032,467` decider rows, `design.md:127-135`, `review.md:213`.)*

| Criterion | (a) Keep `D3` (`TOP=17`) + `GP-2` Chain | (b) Widen `D3` Only (`TOP=61`, `2^64` cells; or `TOP=29`) | (c1–c4) Skip-0 Only / Unbatched 8-Walk / Dyn-Depth / Height-Keyed | (d) **1-Arena `M3-Tail64` (`TOP=61`, `W=15, W_flush=1`) + Hash Stacks** *(Recommended; 4-Arena Fallback Noted)* |
|---|---|---|---|---|
| **Max steps & cells / run** | `< 2^17` leaves (`131,071..2,097,136` steps); `< 2^20` cells | `< 2^61` leaves (`< 2^29` at `TOP=29`); `< 2^64` cells | `c1`: `< 2^64` 0-alloc, `< 2^61` 1-alloc; `c2..c4`: `< 2^64` | **`2^64 - 1` steps** (every partition); **`2^64 - 1` dense cells** (`< 2^64` uniform guard) |
| **0-append / 1-append waste** | Burns `8` addrs/leaf | Burns `8` addrs/leaf | `c1` burns `7` dead cells on `1`-append leaf (`E-5` fails!) | **`0` wasted cells ever (`len` = exact cell count)** |
| **M3 + `state_d` HASH rows (`ESTIMATE`)** | `6,090` (`87` perms, measured) | `TOP=61`: `12,250` (`175`p)<br>`TOP=29`: `7,770` (`111`p) | `c1`: `12,320`; `c2`: **`71,680`** (fails `k<=16`!); `c3`: `12,670+mux`; `c4`: `12,670` | **`14,350` rows** (`205`p: `30` rec + `21` seg/tail + `122` walk + `32` `state_d`) *(4-arena `W_flush=2`: `~20,300`)* |
| **Cold `PAGE` read / Stack `Pop`** | `1,540` rows (`PAGE` & `Pop`) | `4,620` rows (`TOP=61`) / `2,380` (`TOP=29`) | `4,620–4,830` rows | **`4,620` rows** (`PAGE`), **`0` rows** (hot tail), **`210–280` rows** (stack `Push`/`Pop`!) |
| **Full `S=16` leaf rows & `k_leaf` (`ESTIMATE`)** | `~20k–26k` (`k = 15`) | `TOP=61`: `~38k–48k` (`k = 16`)<br>`TOP=29`: `~24k–30k` (`k = 15`) | `c1/c3/c4`: `~40k–58k` (`k = 16`)<br>`c2`: `~95k–110k` (**`k = 17` FAILS**) | **`~38,000–48,000` rows (`k_leaf = 16 <= 65,530`, `~5.5–6.5 s` prove)** *(4-arena: `~45k–56k`, `k = 16`)* |
| **Node rows (`k = 20` <= `950k`, `ESTIMATE`)** | `823,474` (`+8`) | `823,478` (`+12` at `k=16`) | `823,478–836,000` (`c2` fails leaf gate) | **`823,478 – 829,940`** (`+0` adv/eq/lookup cols, `0..1` fixed code col; `> 120k` headroom!) |
| **Decider rows (`k = 20` <= `1M`)** | `929,166` / `1,032,467` (`+0`) | `929,166` / `1,032,467` (`+0`) | `929,166` / `1,032,467` (`+0`) | **`929,166` ord / `1,032,467` tbl (`+0` rows; `16,109` margin untouched)** |
| **`SI-12..SI-14` (`D-1` & `E-12`)** | **BROKEN** (`>8` slice & gaps) | **BROKEN** (`>8` slice & gaps) | `c1/c4`: **BROKEN** | **SOLVED NATIVELY** (`start + k` contiguous for any arity `m`; `O(1)` 1-step `field_at`) |
| **`SRC-023` & Lean/Quint `det` (`E-5`)** | **FALSIFIED** (provability & `post_d` vary) | **FALSIFIED** (`post_d` varies w/ split) | `c1`: **FALSIFIED** on `1..7` allocs | **100% PARTITION-INDEPENDENT** (`post_d(h)` unique across all splits) |
| **`GP-2..GP-4` & `S > 16` scaling** | Breaks `GP-1`; caps `S` | Caps `S` at `C=8`; needs quotient | `c2` blows `k_leaf`; `c4` sparse | **Optimal for `GP-2..4` parallel `SEG` trees & scales `B` flushes cleanly to `S = 256..512`** |

---

### 3.3 Final Position on Q3: Two-Layer Continuation & Aggregation Protocol Under C6

1. **Intra-Run Parallel `SEG` Recursion (Default for Any Single `/1` Run `< 2^64` Steps):**
   - Under `M3-Tail64`, `canon_state_d(h)` (`state_d` at step `h`) is unique and partition-independent (`SRC-023`). A single run folds `L` leaves via linear IVC or `GP-4` binary tree (`U + U -> R -> SEG`, `SEG + SEG -> N -> SEG` checking `L.post_d == R.pre_d` and `0 <= L.h_lo < L.h_hi == R.h_lo < R.h_hi < 2^64`), sealing once to `[prog_d, input_d, outcome_d, n_steps]` (`SRC-006`) with **zero cross-run state serialization**.

2. **Mode A — Same-Key `/1` Cross-Session Checkpointed Continuations (`SHK_CONT_V1`, Closing `E-1, E-2, E-6, E-8, E-9, E-10`):**
   - When a long `/1` run is paused/checkpointed across separate proving epochs `k = 0 .. M - 1`:
     - **Canonical State Digest (`canon_state_d`):** `Poseidon(DOM_STATE_V1, w_0..w_26, fp_hash, tp_hash, len, mem_root, tail_d)` — 100% partition-independent (`SRC-023`), committing exact `initial_fuel`, `remaining_work`, `consumed_work`, `stack_limit`, `AllocCounters`, stacks, and heap commitments (`v0..v2 == 0`).
     - **Domain-Separated Continuation Claim (`SHK_CONT_V1 != SHK_RUN`, `Exploit E-8` closed):** Non-terminal epoch `k` emits an internal prefix claim (refused by `RunDecider` and `SherlockClaims.openClaim`):
       $$\text{cont\_claim}_k = \text{Poseidon}\bigl(\text{"SHK\_CONT\_V1"}, 1, \text{vk\_id}, \text{prog\_d}, \text{orig\_input\_d}, 0, h_{k+1}, \text{post\_canon\_d}_k, \text{deps\_d}_k\bigr)$$
     - **What `input_d` Is Across the Chain (`Exploit E-2` & `E-6` closed):** Every epoch carries the **unmodified genesis `orig_input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)`** (`SRC-021`). Epoch `0` proves `h_lo = 0` (`Init(prog_d, orig_input_d)`). Epoch `k + 1` verifies `cont_claim_k` **in-circuit** (preserving inductive `StepInv` for `ALU` `OP_LT`, closing `Exploit E-10`!), enforces `prev.post_canon_d == cur.pre_canon_d` and `0 == prev.h_lo < prev.h_hi == cur.h_lo < cur.h_hi < 2^64` (preventing fuel refresh `E-2` and state cycles `E-9`), and collapses `[0, h_{k+1}) + [h_{k+1}, h_{k+2}) -> [0, h_{k+2})`.
     - **Single On-Chain Proof & C6 Privacy:** Only terminal epoch `M - 1` (`Halted`, `Rejected`, `Exhausted`, `deps_d == EMPTY`) passes `RunDecider`, which opens `outcome_d` (`sherlock-value/1`), erases all `canon_state_d` and `SHK_CONT_V1` digests, and publishes **one** `6,432 B` proof of `[prog_d, orig_input_d, outcome_d, N]` — byte-for-byte indistinguishable on-chain (`claim_d` and `N`) from an unsplit single-session proof.

3. **Mode B — Semantic Multi-Transaction Suspension in Profile `/2` & `SRC-021` Lazy Input (`sherlock-ckpt/1` + `UNFOLD`):**
   - Where arenas legitimately reset across distinct transactions (`/2` `Await`/`Resume`, `SVM-009`) or at `SRC-021` genesis for non-Unit `val_d`: bind address-free `ckpt_d = H(SHK_CKPT_V1, origin_in_d, epoch, cum_steps, ctl_canon_d, counters_d, env_vd, frames_vd, tasks_vd)` (using creation-time `vd` from `SVM-012` for `O(1)` suspend without heap scan) and lazily materialize touched nodes on demand via 1-level `UNFOLD` (`~210–280` rows/node `ESTIMATE`, memoized by `vd -> local_addr` so shared DAGs never blow up exponentially).

---

### 3.4 Final Position on Q4: Reconciled Production zkVM Prior Art, Prover `Hz`, and Real-World CVEs

| System | Per-Shard / Segment Ceiling | Total Run & Address Ceiling | Cross-Segment Memory Mechanism | On-Chain Statement (Length/RAM Exposed?) | Published Prover Speed (`Hz` / `kHz` / `MHz`) |
|---|---|---|---|---|---|
| **RISC Zero v3.x** | `2^po2` cyc (`po2 in 13..24`, default `2^20`) | Unbounded (`u64` host cyc); `4 GiB` (`2^22 x 1 KiB` pages) | Intra-seg BabyBear trace -> **depth-22 Poseidon2 Merkle page tree** | `(imageId, journalDigest)`; `Halted(0)` zeros `postState` (**0** cyc/seg/RAM) | `10–45 kHz` CPU; **`0.81–1.21 MHz`** 1x 4090 |
| **SP1 v6 Hypercube** | `MAX_SHARD_SIZE = 2^24`, chip `<= 2^22` rows (Jagged PCS) | `u48` chunk `clk` (`24b+24b` -> `2^48` ticks / `2^45` insns); **48-bit RAM (`256 TiB`)** | Intra-chunk **$\mathbb{F}_{p^7}$ septic-curve global sum** + inter-chunk **Poseidon2 `merkle_root`** (`sp1/dev`) | `[vkey, pvDigest]` (v5) / 5 scalars (v6) (**0** cyc/shard/RAM) | Tens of kHz CPU; `1–3+ MHz` 1x GPU; `<12s` ETH (16x 5090) |
| **OpenVM v2.0/2.1** | `timestamp_max_bits <= 29` (`~2^22` rows, `15 GiB` cut) | Unbounded segments; `29`-bit RV32 (`512 MiB`, `CHUNK=4/8`) / RV64 v2.1 | Intra-seg `BEGKN92` LogUp -> **depth-29/30 Poseidon2 `MemoryMerkleAir`** | `(publicValues, appExeCommit, appVmCommit)` via Halo2 (**0** `final_root`/len) | **`19.4 MHz`** CoreMark (1x 5090); `964 MHz` (64x 5090) |
| **Ziren v1.2/v2.0** | Chip `< 2^22`, shard `clk < 2^26` | v1.2: `(u16, u26)` `~2^39.7` insns; v2.0: `< 2^26` shards; `4 GiB` | KoalaBear LogUp-GKR + WHIR -> **$\mathbb{F}_{p^7}$ septic-curve global bus** | `[vkey, pvDigest, VK_ROOT]` (**0** shard/clk/RAM) | **`12.0 MHz`** warm (1x 5090); `75.8 MHz` (8x 5090) |
| **Pico v3.1** | `~2^20..2^22` rows | `(u16, u25)` -> **`2^39` cycles**; `4 GiB` unified | Cross-chunk offline LogUp (`InitializeFinalize` + `ReadWrite`) | `(riscvVKey, publicValuesDigest)` (**0** len/RAM) | Multi-MHz (1x 5090); `<8.73s` p99 ETH (4x 5090) |
| **Jolt (RV64IMAC)** | Monolithic streaming (`DEFAULT_MAX_TRACE_LENGTH = 2^24`) | Power-of-2 `T` (`< 2 GB` streaming target); `2^32` dwords | **Twist & Shout** one-hot increment sumcheck + Dory + BlindFold | Transcript absorbs `(inputs, outputs, panic, MemoryLayout, T)` | **`> 500 kHz`–`1 MHz`** on 32c CPU (zero recursion!) |
| **Cairo / Stwo** | `N * B^2 <= 2^30` (`<= 2^28` M31 rows) | Unbounded via **Cairo Bootloader** PIEs; `2^29` M31 addrs/proof | Continuous **write-once relocated** `seg:off -> [1..M]` + public pages | `FactRegistry` stores `keccak256(progHash, outHash)` (**0** step count) | Multi-MHz M31 CPU/GPU Circle STARK |
| **zkWasm** | Halo2 KZG `k = 18..22` (`57k..1.04M` steps/slice) | **`2^32 - 1` steps** (`AllocatedU32StateCell` `eid`); `4 GiB` Wasm | **`img_col == post_img_col` KZG G1 commitment equality** + depth-32 SMT | 1 hashed BN254 scalar (`vk_chain`, init/out G1, pub I/O) | **`5–20 kHz`** (1x 4090, **amortizing `65k–1M` steps/slice!**) |
| **Miden v0.36/1.0rc** | Monolithic **`MAX_CYCLES = 2^29`** | `2^29` cyc/VM proof; Tx->Batch->Block kernels; `[0, 2^32)` + depth-64 SMT | Sorted `MemoryChiplet` (`2x16b` $\Delta\text{clk}$) + Hash Chiplet Merkle paths | 40-felt `ExecutionClaim` (**0** cyc/RAM) | **`152–270 kHz`** (64c EPYC); `170 kHz` (16c M4 Max) |
| **Valida 1.0** | `<= 2^26` BabyBear rows (`2^20` mem ops default) | `< p - 4` segments; **`960 MiB`** (`[0, 0x3C000000)`) | Stack/FP-only LogUp -> **cross-segment `PersistentMemBus`** | Raw proof exposes per-seg `(pc, fp, log_deg)`; recursive hides | Hundreds of kHz CPU |
| **Nexus (v1/2 -> v3)** | v1/2: `K=1..16` IVC; v3: single Stwo `<= 2^28..29` | v1/2: unbounded IVC; v3: single Stwo chunk (`seq.rs`, no cont. yet) | v1/2: height-27 Poseidon Merkle; v3: Harvard LogUp (`< 2^31 - 1`) | v1/2: 1 scalar `u.x` (hides `n` & root); v3: `log_size` + I/O | **v1/2 Curve IVC: `200–813 Hz`** -> v3 Stwo: `10–16+ kHz` |
| **Airbender / ZisK / Lambda / Binius64 / Lattice** | Airbender `~2^22`; Binius64 binary tower; Neo/SuperNeo CCS | Airbender: **`2^36` cyc** (`2x19b` ts); Binius64: **`2^30` words (`8 GiB`)** | Airbender: 2-pass mem subtree; Twist-IVC: `O(\|M\|)`/shard (`2025/105`); Nebula: `O(\|M\|)` once (`2024/1605`) | Wrap/root erases internal shards | Multi-MHz (Airbender GPU); laptop-scale streaming (Mangrove/Neo) |

**Key Prior-Art & CVE Lessons for Sherlock:**
1. **Why boundary Merkle (`M3-Tail64`) is mandatory on `midnight-circuits 7.3.0`:** Zero user challenge phases (`verifier_gadget.rs:16-17`) and unit rotations `-1, 0, +1` (`verifier_gadget.rs:609`) rule out in-leaf LogUp/Nebula challenges, while Sharded Twist costs `O(|M|)` per shard (`TwistShout.pdf` §2.9.3) — the exact reason even SP1 v6.1 (`sp1/dev`) re-introduced inter-chunk Poseidon2 Merkle roots (`merkle_root`) alongside septic sums.
2. **Real-world 2024–2026 CVEs directly validate our defenses:** Missing counter/limb range checks caused RISC Zero `GHSA-g6qg-9j8p-7j87` and OpenVM `CVE-2025-46723` (validating `SRC-004` + `E-1`); missing Merkle boundary guards caused OpenVM `CVE-2026-62333` (validating fixed `TOP = 61` + `old_leaf == EMPTY_SEG`); missing `is_complete`/`termination` checks caused SP1 `GHSA-c873-wfhp-wx5m` and Jolt truncation bugs (validating `SRC-006` + `SHK_CONT_V1` domain separation); and post-run `segment:offset` aliasing in Cairo (`security.rs`) validates domain-tagged `M3-Tail64`.
3. **Why Sherlock runs at `~0.23 Hz` (`S = 16`) while `zkWasm` (also Halo2 KZG!) hits `5–20 kHz`:** Both pay heavy 255-bit curve MSM+NTT per circuit, plus Sherlock folds over a single pairing curve (`507,091` non-native collapse rows in `SHKIVC06`, `design.md:128`) — **but `zkWasm` amortizes each proof over `57,056–1,040,096` steps whereas Sherlock currently amortizes `63 s` over `S = 16` steps!**

---

### 3.5 Final Position on Q5 [LEAD]: Hardware Envelope, Prover Throughput, `S` Amortization, Scaling to `2^64`, and Reconciled Tested Targets

#### 1. Verified Host Hardware & Resource Envelope
- **Platform:** `ASUS ProArt P14 H7407BA`, Linux `7.3.0-41-omarchy` `aarch64`, **20 cores / 20 threads** (`10x Cortex-X925 @ 4.004 GHz` + `10x Cortex-A725 @ 2.808 GHz`, `25 MiB` L2 + `24 MiB` L3), **`120.28 GiB`** (`129.15 GB`) unified LPDDR5X (**`244 GB/s` measured R+W** copy rate of `273 GB/s` nominal, `gpu.md:15,18`, `prover.md:15`, `design.md:327`), `~3,090 GiB` free NVMe (`runs.jsonl:655`).
- **Integrated GPU:** NVIDIA `RTX Spark N1X` (`GB20B`, `sm_121`, `6,144` CUDA cores, CUDA `13.4`); shares unified LPDDR5X (`cudaMalloc` counts directly in process `VmRSS` and cgroup `MemoryMax`, `gpu.md:16`); prover kernels unbuilt (`PARKED` `L4`, `speed-levers.json:112-125`).
- **Enforced Cgroup Caps (`SRC-016`, `requirements.md:41`, `budgets.json:235-236`):** **`48 GiB`/process**, **`100 GiB` owner aggregate** (`60 GiB` default concurrent operational cap / `72 GiB` `heavy.slice`, `c0-first-profile.md:57`), `20 GiB` free reserve, `< 16 GB` / `< 120 s` leaf cap (`SRC-014`).

#### 2. Measured Baseline Service Times & Contention (`qa/sherlock/runs.jsonl`)
- **`k = 13` M2 leaf (20t):** **`2.55 s`** prove, **`0.64 GB`** warm RSS, `3,824 B` proof (`prover.md:11`, `runs.jsonl:638`).
- **`k = 20` stand-in leaf (`E4`, `742,327` rows, 20t):** **`66.16 s`** solo (`69.11 s` concurrent), **`11.14 GB`** RSS, `3,824 B` proof (`e2-e4.md:7-19`, `runs.jsonl:709,711`).
- **`k = 16` full-VM `M3-Tail64` chip leaf (`S = 16`, `~38k–48k <= 65,530` rows):** **`ESTIMATE 5.0–6.5 s` solo**, **`1.2–2.2 GB` RSS** (scaled via `d = 0.884`).
- **`k = 20` IVC Node `SHKIVC06` (20t solo):** **`59.62–64.95 s`** (`61.07 s` in `A16-node-step`, `16.58 GB` cgroup peak, `6,432 B` proof, `runs.jsonl:617,648`); over `823,478–829,940` chip-keyed rows: **`ESTIMATE 63.0–67.0 s` solo**, **`17.0–18.0 GB` RSS**.
- **Measured Unified-Memory Contention:**
  - *`E4` 20t Node + 20t Leaf (`runs.jsonl:711`)*: Node slowed `1.43x` to **`90.2 s`** (`17.55 GB` RSS).
  - *`T1(b)` `P = 3 x 6t` Concurrent Nodes (`runs.jsonl:616,618`, `c0-first-profile.md:57`)*: `75.77 s` (1x6t) slowed `f3 = 1.690–1.736x` to **`128.09–131.50 s`/node** (**`43.83 s/node` net**, **`47.73 GB` aggregate RSS**), because `2^23` extended scalar FFTs (`56.6%` of wall) + `blst` MSM (`25.6%`) saturate the single `244 GB/s` LPDDR5X bus (`U = 9.77/20` cores).
- **`k = 20` `RunDecider` (`929,166` rows):** **`70.35–77.88 s`**, **`17.35 GB`** RSS, `6,432 B` proof (`runs.jsonl:617,649`).
- **Fixed Per-Run Non-Step Tail:**
  - *Warm cached prove+verify (`resident_pk` node+decider load `~85 s` `ESTIMATE` from `42.25 s` `A16-node-step` + `70.35 s` decider + `2.86 s` `A5` `SRC-010` verify)*: **`~160 s` (`2.7 min`, `ESTIMATE`)**.
  - *Full `G-C` QA gate (`211.07 s` `A16-fold3-decider` setup + `70.35 s` decider + `2.86 s` `SRC-010` + `163.94 s` `A6` `SRC-020` `validate_call`)*: **`448.2 s` (`7.5 min`, measured sum)**.

#### 3. Throughput Across Folding Modes at `S = 16` (`ESTIMATE` Central `5.5 s` Leaf + `63.0 s` Node = `68.5 s` / 16 Steps)

| Folding Mode (`S = 16`) | Time / 16-Step Leaf (`ESTIMATE`) | Effective `s/step` (`ESTIMATE`) | Effective `Hz` (`ESTIMATE`) | Peak Host RSS | Verdict on This Single Host |
|---|---:|---:|---:|---:|---|
| **Sequential 1-proc (20t, leaf -> node)** | `5.5 + 63.0 = 68.5 s` (`65–72 s`) | **`4.28 s/step`** (`4.06–4.50 s`) | **`0.234 Hz`** (`0.22–0.25 Hz`) | **`17.4–18.0 GB`** | **Best efficiency at `S = 16`** (`92%` node-dominated, zero bus contention) |
| **Core-Pinned Pipeline (`18t` node + `2t` leaf)** | `max(63.0 * 1.05, ~11 s) = ~66.2 s` | **`4.14 s/step`** | **`0.242 Hz`** (`+3.4%`) | `~19.5 GB` | Marginal at `S = 16`; unpinned `20t+20t` regresses to **`90.2 s` (`0.177 Hz`)** (`E4`) |
| **`GP-4` Binary Tree (`P = 3 x 6t`, `~L` `k=20` proofs)** | `43.83 s` (`R`/`N`) + `~1.8 s` leaf = **`45.6 s`** | **`2.85 s/step`** (`2.7–3.1 s`) | **`0.351 Hz`** (`+50%` wall) | **`47.73 GB`** (`<= 60 GiB`) | **Fastest 1-host `S = 16` mode** (`ceil(L/2)` `R` + `ceil(L/2)-1` `N` = `~L` `k=20` proofs) |
| **Multi-Host `GP-4` Tree (`W >= L/2` hosts)** | Depth `1 + ceil(log2(L/2))` x `63 s` | `O((log2 L)/(16 L))` | **`~6.5 Hz`** (`4,096` steps, `W=128`) | `17.4 GB` / host | Scales horizontally across independent memory buses |

#### 4. Concrete Run-Length Scaling (`16` to `2^64` Steps, `S = 16` on This Host) & Proof Why `2^64` Is Physically Impossible
*(Shows `[Warm Prove+Verify / Full G-C Gate w/ validate_call]` for Sequential 20t (`68.5 s`/leaf) and `P = 3` Tree (`45.6 s`/leaf); `SRC-019` GP bundle = `10,256 B * M + 6,432 B`.)*

| Run Length (`N` Steps) | Leaves `M` (`S=16`) | Proofs (`2M+1`) | Sequential 20t Wall (`ESTIMATE`) | `P = 3` Host Tree Wall (`ESTIMATE`) | Peak Prover RSS | `SRC-019` Bundle Disk (`ESTIMATE`) | Streamed Witness (`ESTIMATE`) | Physical & Practical Status |
|---|---:|---:|---|---|---|---|---|---|
| **`16`** (`2^4`) | `1` | `3` | **`3.8 min` / `8.6 min`** | `3.8 min` / `8.6 min` | **`17.4 GB`** | `16.3 KiB` | `4–32 KiB` | Trivial `G-B` smoke (`< 20 min`) |
| **`256`** (`2^8`) *(`list123` = `303`)* | `16` *(19)* | `33` *(39)* | **`20.9 min` / `25.7 min`** *(`29.2 min` @ 303)* | **`14.8 min` / `19.6 min`** *(`21.9 min` @ 303)* | `17.4` / `47.7 GB` | `166.5 KiB` | `64–512 KiB` | Routine `G-C` (`4x` under `< 2 h` cap) |
| **`4,096`** (`2^12`) | `256` | `513` | **`4.92 h` (`4.99 h` w/ gate)** | **`3.29 h` (`3.36 h` w/ gate)** | `17.4` / `47.7 GB` | **`2.51 MiB`** | `1–8 MiB` | **Recommended `M5` endurance & continuation soak** (`< 35 min` at `S=256`!) |
| **`65,536`** (`2^16`) | `4,096` | `8,193` | **`3.25 days` (`78.0 h`)** | **`2.16 days` (`51.9 h`)** | `17.4 GB` (+`<50 MB` tree) | `40.1 MiB` | `16–128 MiB` | Weekend CPU run at `S=16` (**`~3.6 h` at `S=256`**) |
| **`2,097,152`** (`2^21`, old `D3` cap) | `131,072` | `262,145` | **`103.9 days` (`3.4 mo`)** | **`69.2 days`** | `17.4 GB` (+`~128 MB`) | `1.25 GiB` | `0.5–4 GiB` | Old `D3` ceiling (`~5.3 days` at `S=256`) |
| **`16,777,216`** (`2^24`) | `1,048,576` | `2,097,153` | **`2.28 years` (`831 d`)** | **`1.52 years`** | `17.4 GB` (+`~1 GB`) | `10.02 GiB` | `4–32 GiB` | Multi-host cluster + `S >= 256` regime |
| **`4,294,967,296`** (`2^32`) | `2.684 x 10^8` | `5.369 x 10^8` | **`582.5 years`** | **`387.8 years`** | **HOST OOM** if `2^32` live cells (`>256 GB`) | **`2.50 TiB`** | `1–8 TiB` | Exceeds human lifetime & host RAM on KZG CPU |
| **`18,446,744,073,709,551,616`** (`2^64`) | `2^60` (`1.153e18`) | `2.306 x 10^18` | **`2.502 x 10^12` years (`2.50 trillion yr`)** | **`1.666 x 10^12` years** | **Impossible** (`> 1 ZB` RAM) | **`11.82 Zettabytes`** | `4.7–37 ZB` | **Physically impossible** (`181x` age of cosmos; `146 yr` unproved at `4.004 GHz` 1-cyc/step; `~6.3 x 10^21 J` = `10x` global annual energy at `80 W`) |

#### 5. Primary Throughput Lever: Scaling `S` (`Steps / Leaf`) Over Domain-Tagged 1-Arena `M3-Tail64` + Hash Stacks
Because `E2` proved `k_leaf = 13 -> 20` adds only `+4` node rows/bit (`+28` rows total, `e2-e4.md:25-35`) and `E4` proved a `k_leaf = 20` circuit runs in `66.16 s` at `11.14 GB` RSS (`runs.jsonl:709`), while moving `FRAMES`/`TASKS` to Poseidon hash stacks reduces marginal leaf cost to **`~1,460–1,735` rows/step (`ESTIMATE`)** (`B = S / 16` pooled segment flushes in 1-arena `M3-Tail64`), raising `S` breaks the `63 s` node bottleneck **on 1 host without GPU or new proof primitives**:

| `S` (Steps / Leaf) | Pooled Flushes `B` (Max Appends `8B`) | `ESTIMATE` Leaf Rows (`1`-Arena + Hash Stacks) | `k_leaf` (`<= 2^k - 6`) | `ESTIMATE` Solo `t_leaf` (20t) | Sequential `s/step` (`Hz`) (`ESTIMATE`) | Pinned Pipeline (`X925` Node + `A725` Leaf, `1.3x` Contention, `~28 GB` RSS) | `303`-Step `list123` Warm Wall (`ESTIMATE`) | `4,096`-Step Soak Warm Wall (`ESTIMATE`) |
|---:|---:|---:|---:|---:|---|---|---|---|
| **`16`** *(M4 baseline)* | `1` (`8` cells) | `~38k–46k` | `16` (`65,530`) | `~5.5 s` | `4.28 s` (**`0.23 Hz`**) | `0.24 Hz` (run serial) | **`24.3 min`** (`19` leaves) | **`4.92 h`** (`256` leaves) |
| **`64`** | `4` (`32` cells) | `~105k–120k` | `17` (`131,066`) | `~11.0 s` | `1.16 s` (**`0.86 Hz`**) | `0.98 Hz` | **`8.9 min`** (`5` leaves) | **`1.43 h`** (`64` leaves) |
| **`128`** | `8` (`64` cells) | `~198k–230k` | `18` (`262,138`) | `~20.0 s` | `0.65 s` (**`1.54 Hz`**) | **`1.85 Hz`** | **`5.4 min`** (`3` leaves) | **`47.0 min`** (`32` leaves) |
| **`256`** *(Recommended pre-freeze)* | `16` (`128` cells) | `~385k–450k` | `19` (`524,282`) | `~36.0 s` | `0.39 s` (**`2.59 Hz`**) | **`3.12 Hz`** | **`6.0 min`** (`2` leaves) | **`29.1 min`** (`16` leaves) |
| **`512`** | `32` (`256` cells) | `~760k–895k` | `20` (`1,048,570`) | `~67.0 s` | `0.25 s` (**`3.94 Hz`**) | **`4.95 Hz`** | **`4.9 min`** (`1` leaf!) | **`20.0 min`** (`8` leaves) |

*(Adding optional `sm_121` `mori-gpu` at `ESTIMATE 1.6x–2.6x` node speedup over `S = 256..512` reaches `ESTIMATE ~8–12 Hz` = `~700k–1M` steps/day on this single machine.)*

#### 6. Reconciled Four-Tier Tested Targets for the Specification (`GP-001`, `SRC-011`, `SRC-015`, `SRC-023`, `SAS-001`)

| Qualification Tier | Concrete Workload & Requirements | Wall-Clock Cap (`ESTIMATE` Expected at `S = 16`) | Enforced RSS Cap (`SRC-016`) | Specific Guarantees Verified |
|---|---|---|---|---|
| **Tier 1: Smoke Gate `G-B`** (`SRC-015(b)`, row `A18`) | `entry 7;` (`6` steps, partial leaf) + 2-leaf link (`20–32` steps) | **`<= 20 min` (`1,200 s`)** *(`~8.6–9.8 min` w/ `validate_call`)* | `<= 24 GiB`/proc, `<= 48 GiB` agg | `Init` (`SRC-021`), partial-leaf slot disable (`SRC-014`), `prev_h > 0` link, decider + `SRC-010` + `SRC-020` |
| **Tier 2: Routine Release Gate `G-C`** (`SRC-011`, `SRC-015(c)`, rows `A19, A33`) | `fact(3)` (`293` steps) + `list123` (`303` steps) (each `< 2 h`, *`~28 min`*) + **unseen post-freeze `P3` of `512–1,024` steps** allocating `>= 128` cells across `>= 16` segment flushes incl. a **constructor of arity `>= 10` (`> C = 8`) spanning a leaf close** + a zero-append compute loop | **`<= 2 h` (`7,200 s`) per job** *(`~42–75 min` for `512–1,024`-step `P3`)* | `<= 48 GiB`/proc, `<= 100 GiB` agg (`60 GiB` concurrent slice) | Multi-segment `M3-Tail64` flushes, cold `PAGE` reads, `0`-burn spin loop, and `Exploit E-12` (`m > 8` contiguous `Build` across leaf close) within a `< 2 h` CI budget |
| **Tier 3: `M5` / `SAS-001` Endurance & Continuation Soak** (rows `A33b, A34c`) | **`>= 4,096` owner steps (`>= 256` leaves at `S = 16`)** allocating **`>= 512` heap cells (`>= 64` flushed segments)**, proved **(i)** as 1 uninterrupted run and **(ii)** as a **2-epoch `SHK_CONT_V1` continuation (`2,048 + 2,048` steps)** | **`<= 6 h` (`21,600 s`)** *(`~4.9 h` serial / `~3.3 h` `P=3` at `S=16`; `< 35 min` at `S=256`)* | `<= 24 GiB` single-proc; **`RSS(leaf 256) - RSS(leaf 1) <= 1 GiB`** | Flat `O(1)` streaming RSS (zero memory leak over 256 folds), `2.5 MiB` `SRC-019` bundle verification, and bit-identical `[prog_d, orig_input_d, outcome_d, 4096]` + `claim_d` across continuation splits |
| **Tier 4: `SRC-023` 3-Partition Replay & `2^64` Boundary Suite** (row `A38`, `SRC-004`) | **(a)** `>= 64`-step constructor-allocating program proved under `[16, 16, ...]`, `[1, 1, ...]`, and `[7, 9, ...]` asserting identical `post_canon_d`, `outcome_d`, `claim_d`, `n_steps`; **(b)** `MockProver` boundary suite at `seg = 0`, `seg = 2^17` (proving `D3` wall is gone), `seg = 2^61 - 1` (`len = 2^64 - 1` pass, `2^64` fail), `h = 2^64 - 1` pass vs `h = 2^64` / `p - 1` fail, and `E-1` DAG self-join `n_claims >= 2^64` refusal | `(a) <= 90 min` KZG; `(b) < 5 min` `MockProver` + Lean/Quint | `<= 24 GiB` | Direct empirical & formal proof of `SRC-023`, `Exploit E-5` closure, `D3` removal, and `2^64` non-wraparound without claiming impossible `2^64`-step runs |

---

### 3.6 Evidence List (Verified Paths, Commits, and URLs)
- **Repository (`c57351dd43cec8cb9f2db55d8350c0388c1f65ab`):**
  - `research/sherlock-runlength-consensus-20261111/{BRIEF.md:1-79, lanes/A/round1.md:1-547, lanes/B/round1.md:1-357, lanes/C/round1.md:1-542, lanes/D/round1.md:1-390, lanes/E/round1.md:1-373}`
  - `openspec/changes/sherlock-vm-consensus-20261107/{general-purpose.md:1-58, requirements.md:1-66, design.md:85-225,281-342, dissent.md:150-232, tasks.md:1-144, specs/sherlock-general-purpose/spec.md:1-25}`
  - `crates/mori-core/src/sherlock/helper_machine/{families/environment.rs:201-284,399-416, families/adt_constructor.rs:135-156, families/adt_build.rs:16-61, shared/owner_runtime.rs:2780-3035,3515-3580, shared/owner_runtime/owner_adt_runtime.rs:640-735,1272-1454,1580-1612}`
  - `crates/mori-chip/src/{m3.rs:50-55,104-144,209, poseidon.rs:52-61,113-115, range.rs:30-35,92-129, alu.rs:6-9,23-26,111-125, rf.rs:32-68,94-100}`
  - `crates/mori-recursion/src/{ivc.rs:24-26,77-79,359-376, run_decider.rs:48-86,190-329}`
  - `clean/fork/Clean/Sherlock/Aggregation.lean:157-220,1191-1214` & `models/sherlock/quint/aggregation.qnt:1-45`
  - `tools/profile/vendor/midnight-proofs-0.8.3/src/dev/cost_model.rs:370-398`
  - `qa/sherlock/{runs.jsonl:77-87,598-618,638-655,708-711, budgets.json:225-375, decisions/speed-levers.json:31,70-125}`
  - `wiki-llm/recursion-perf/profiles/c0-first-profile.md:19-65`
  - `research/sherlock-recursion-design-review-20261110/experiments/{e2-e4.md:1-85, e4-solo.json, e4-concurrent.json}`
  - `research/sherlock-processor-analogy-20261110/{study.md, d7-frames.md, experiments-2.md, experiments-3.md, experiments-4.md}`
  - `research/sherlock-c6-outcome-ledger9-review-20261110/review.md:1-259`
  - `research/sherlock-gp-architecture-20261110/architecture.md:68-185`
  - `research/sherlock-speed-replan-20261108/{prover.md:1-25, tree.md:35-65, gpu.md:1-28}`
- **External Primary Sources & Advisories:**
  - RISC Zero: `https://raw.githubusercontent.com/risc0/risc0/main/risc0/binfmt/src/{image.rs,lib.rs,exit_code.rs}`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/circuit/rv32im/src/lib.rs`, `https://raw.githubusercontent.com/risc0/risc0-ethereum/main/contracts/src/IRiscZeroVerifier.sol`, `https://github.com/risc0/risc0/security/advisories/GHSA-g6qg-9j8p-7j87`
  - SP1: `https://raw.githubusercontent.com/succinctlabs/sp1/dev/crates/primitives/src/consts.rs`, `https://raw.githubusercontent.com/succinctlabs/sp1/dev/crates/hypercube/src/air/public_values.rs`, `https://raw.githubusercontent.com/succinctlabs/sp1/dev/crates/core/machine/src/memory/consistency/columns.rs`, `https://docs.succinct.xyz/docs/sp1/hypercube/shards`, `https://github.com/succinctlabs/sp1/security/advisories/GHSA-c873-wfhp-wx5m`
  - OpenVM: `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/system/memory/mod.rs`, `https://osv.dev/vulnerability/CVE-2026-62333`, `https://github.com/openvm-org/openvm/security/advisories/{GHSA-fh29-29h9-qm9h,GHSA-jf2r-x3j4-23m7,GHSA-j9m2-fxc5-fr82}`
  - Ziren / Pico / Airbender: `https://eprint.iacr.org/2026/2330`, `https://blog.brevis.network/2026/11/05/pico-3-1-128-bit-provable-security-still-real-time-on-4-gpus-now-fully-open-source/`, `https://raw.githubusercontent.com/matter-labs/zksync-airbender/main/docs/subarguments_used.md`
  - Jolt / Twist & Shout / Nebula / Lattice / Binius64: `https://people.cs.georgetown.edu/jthaler/TwistShout.pdf` (`https://eprint.iacr.org/2025/105`), `https://blog.zksecurity.xyz/posts/jolt-findings/`, `https://eprint.iacr.org/2024/1605`, `https://eprint.iacr.org/2025/294`, `https://eprint.iacr.org/2026/242`, `https://github.com/binius-zk/binius64/blob/main/crates/core/src/constraint_system/value_index.rs`
  - Cairo / Stwo / zkWasm / Miden / Valida / Nexus / Lambda VM: `https://www.axiom.xyz/blog/stwo-bug`, `https://raw.githubusercontent.com/lambdaclass/cairo-vm/main/vm/src/vm/security.rs`, `https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/crates/zkwasm/src/circuits/mtable/allocator.rs`, `https://raw.githubusercontent.com/0xMiden/miden-vm/next/processor/src/execution_options.rs`, `https://arxiv.org/html/2505.08114v1`, `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/specification/zkvm-spec-3.0.pdf`, `https://blog.alignedlayer.com/lambda-vm-continuations/`

---

### 3.7 What Was Not Checked
1. **No circuit compilation, `cost_model` runs, `MockProver` execution, Quint/Lean checks, or heavy KZG proving:** All builds, tests, and proving remain prohibited (`BRIEF.md:57,75-77`). Row counts and timings for 1-arena `M3-Tail64` (`W = 15`), `D7` Poseidon hash-chain stacks, `S = 64..512` scaling, and `SHK_CONT_V1` node rules are analytical `ESTIMATE`s grounded in `poseidon.rs`, `m3.rs`, `cost_model.rs`, and measured `E2/E4/E8/A16/C0` receipts.
2. **Full 108-rule universal chip wiring in `crates/mori-chip` (Tasks 4.16–4.19):** Empirical early-close frequency ($\bar{s} \le S$) across real Sherlock workloads when `N_alu`, `N_page`, or `B` flushes saturate before slot `S`, plus mechanical verification that `v0..v2 == 0` holds across all 108 rules, awaits chip implementation.
3. **Simultaneous `prove` + `prove` contention between a fast hand-written `k = 18..19` chip leaf on `A725` cores and a `k = 20` node on `X925` cores, and unbuilt `sm_121` `mori-gpu` kernels:** `E4` (`e2-e4.md:19`) overlapped `model_pick` rather than `prove + prove`, and `mori-gpu` has no built MSM/NTT kernels yet.
4. **Orthogonal open items (`D-2` `decl_schema_d` canonical normal form in `SRC-022`, `Exhausted(LogicalCap)` kind `7` vs `SVM-009` in `dissent.md:226`, `midnight-proofs` `0.8.2` vs `0.8.3` ledger reunification in Task `2.17`, and live Midnight Preview `LedgerParameters` in Task `6.8`):** Not re-tested here.
5. **Other lanes' Round 2 outputs (`lanes/{A,B,C,E}/round2.md`):** Never listed or read.
