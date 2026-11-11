# Sherlock Run Length and Memory Ceiling — Round 2 Cross-Review and Final Position (Lane B: Chip Leaf Memory Design and Cost-Fit Lead)

Date: 2026-11-11
Study: `sherlock-runlength-consensus-20261111` (`research/sherlock-runlength-consensus-20261111/BRIEF.md:57-79`)
Inputs Reviewed: `lanes/{A,B,C,D,E}/round1.md` and primary repository / external sources.

---

## Executive Summary of Lane B Round 2 Shifts and Final Stance

1. **Upgrade Arena Depth from `B = 32` (`2^35` cells) to `TOP = 61` (`2^64` dense cells per arena)**: Because Halo2 KZG proving cost depends on domain power $2^{k_{\text{leaf}}}$ (`midnight-proofs-0.8.3/src/dev/cost_model.rs:370-381`), both `B = 32` (`~44k` rows) and `TOP = 61` (`58,050..62,670` rows at `N_page_cell = 4..5`, `ESTIMATE`) fit inside **`k_leaf = 16` (`<= 65,530` usable rows)** — or `k_leaf = 17` (`76,530..79,330` rows at `N_page_cell = 8`, adding exact **`+4` node rows** and **`+0` decider rows**, `e2-rows.jsonl:1-7`). Adopting `TOP = 61` eliminates every sub-`2^64` constant from `GP-1` and `GP-001`.
2. **Converge on 4 Dense Arenas (`DATA, FIELDS, ENV, SLOTS`) + 2 Hash-Chain Stacks (`FRAMES, TASKS`) + `W_flush = 2` Shared Dual Walks + `open_r[0..3][0..6]` Carry (`M3-Tail64`)**: Auditing `owner_runtime.rs:2990,3023,3028,4701-4734`, `owner_adt_runtime.rs:1344-1359,1430-1454,1585-1623`, and `adt_build.rs:37-55` confirms both (a) why 4 separate dense arenas (already in `design.md:249-260`) preserve `SI-12..SI-14` and `SI-21..SI-23` natively without Carbon `D-1` linked slices, and (b) why Lane B Round 1's `W_flush = 1` hits a simultaneous-flush livelock when `Cons`/`Branch` or `XAdtHelperBuildField (GBuildLast)` crosses `0 mod 8` in `DATA` and `FIELDS` on the same step. Setting **`W_flush = 2`** eliminates all livelocks (`GP-1` complete) while register-carrying `open_r[0..3][0..6]` (`28` words) saves up to `2,240` rows over scalar `tail_d[0..3]`.
3. **Reject `Wide M3 + act[0]` (Lane C) and Carbon `D-1` Linked Slices (Lanes A, C, D)**: `Wide M3 + act[0]` still pads `1..7`-append leaves to `+8`, breaking `Aggregation.lean:1162` (`statement_independent_of_S`) and `SI-12`. Carbon `D-1` linked slices break 1-step random-access projection `field_at(cur, fidx)` (`owner_adt_runtime.rs:1430-1454`, `c6_match_helper_step.rs:307-336`).
4. **Two-Tier Q3 Continuations with Red-Team Soundness Hardening**: Under `M3-Tail64`, `post_d(h)` is a deterministic function of `(prog_d, input_d, h)`. **Tier 1** folds epochs via `SEG + SEG -> SEG` or `SHK_CONT_V1` over `post_d(h)` (`0` extra leaf rows, `0` `UNFOLD` rules); **Tier 2** adds address-free `ckpt_d` (`SHK_CKPT_V1`) + lazy 1-level `UNFOLD` for dead-heap compaction, `/2` async suspension, and `SRC-021` non-scan `val_d` input loading — always verifying predecessors in-circuit back to `h_lo = 0` (`E-10`) and enforcing `< 2^64` on `n_claims` (`E-1`).
5. **Realistic Throughput & Tested Gates**: Keep `S = 16` (`k_leaf = 16..17`, `ESTIMATE ~4.25 s/step` serial, `~2.95 s/step` 3-way `GP-4` tree via 2-leaf `R` lift) for M4 bring-up, benchmark bounded `S = 64..128` (`k_leaf = 18..19`, `ESTIMATE ~0.83–1.31 s/step`) before key freeze, retain `SRC-011` (`<= 320` steps `< 2 h`), and add a `>= 4,096`-step / `>= 512`-cell `GP-001` soak (`< 6 h`), `>= 1,024`-step 2-epoch continuation test (`< 2 h`), and `2^64 - 1` / dual-flush boundary tests.

---

## Section 1: What Lane B Now Accepts (and Updates to Lane B Round 1)

### 1.1 Upgrade Top-Tree Depth from `B = 32` (`2^35` Cells) to `TOP = 61` (`2^64` Dense Cells per Arena) — Accepting Lanes A, D, and E

In Round 1 (`lanes/B/round1.md:17-20,176-182`), Lane B chose `B = 32` (`2^35` cells) to save leaf rows while already proving in Table 2.4.2 (`lanes/B/round1.md:178`) that `B = 61` (`2^64` cells) fits in `k = 16`. Lanes A (`lanes/A/round1.md:201-230`), D (`lanes/D/round1.md:113-147`), and E (`lanes/E/round1.md:214-248`) point out two decisive facts that we **accept**:

1. **Zero KZG domain jump inside `k_leaf = 16` (and `+4` node rows if `k_leaf = 17`)**: In `tools/profile/vendor/midnight-proofs-0.8.3/src/plonk/circuit.rs:2344-2389` and `src/dev/cost_model.rs:370-381`, prover FFT/MSM work depends solely on $2^{k_{\text{leaf}}}$, not active row utilization within the same $k_{\text{leaf}}$. As recomputed in §3.2.4 below, our complete 4-arena `M3-Tail64` leaf (`S = 16`, `W_flush = 2`, `TOP = 61`) uses `ESTIMATE 58,050` rows at `N_page_cell = 4` and `ESTIMATE 62,670` rows at `N_page_cell = 5` — both **strictly below the `65,530` usable-row ceiling of `k_leaf = 16`** (`65,536 - 6`). Even if Milestone M4 (`SRC-013`, `requirements.md:38`) provisions `N_page_cell = 8` plus in-leaf `Init` (`ESTIMATE 76,530..79,330` rows $\implies$ `k_leaf = 17`, well below `K_LEAF_MAX = 20` in `SRC-014`, `requirements.md:39`), `research/sherlock-recursion-design-review-20261110/experiments/e2-rows.jsonl:1-7` proves `k_leaf = 16 -> 17` adds **exact `+4` node rows** (`~3 s` leaf prove delta vs a `65 s` node) and **`+0` decider rows**.
2. **Elimination of any sub-`2^64` ceiling in `GP-1` / `GP-001`**: With `TOP = 61` (`2^61` 8-cell segments = **`2^64` dense cells per arena**), every arena length counter `len_a` has the exact same `< 2^64` non-wraparound bound as step height `h < 2^64` (`SRC-004`). Because no step appends more than 2 cells to any single arena (`owner_runtime.rs:2780-2802`), `GP-1` and `GP-001` need **no separate sub-`2^64` memory ceiling** (`BRIEF.md:8`).

### 1.2 Adopt 4 Separate Dense Arenas (`DATA, FIELDS, ENV, SLOTS`) + 2 Hash-Chain Stacks (`FRAMES, TASKS`) — Converging with Lane E and `design.md`

In Round 1 (`lanes/B/round1.md:205-231`), Lane B moved `FRAMES` and `TASKS` into cached-top Poseidon hash chains (`design.md:265`), but packed `DATA, FIELDS, ENV, SLOTS` into 1 unified dense log. Lane E (`lanes/E/round1.md:174-184,218-235`, `Exploit E-12`) independently converged on hash-chain stacks + tail-buffered dense segments, while demonstrating from `mori-core` why **4 separate dense arenas** are superior. Direct verification confirms:

1. **Already specified in `Init` (`design.md:249-260,265-266`)**: Fields `26..29` and `34..37` of `models/sherlock/quint/init.qnt` (`design.md:249-260`) **already** carry four distinct `(len_a, root_a)` pairs (`len_data, len_fields, len_env, len_slots` and `root_data, root_fields, root_env, root_slots`), with `fp = tp = NIL = EMPTY_STACK` (`design.md:265`).
2. **`mori-core` rules depend on per-arena dense indices (`SI-12`, `SI-21..SI-23`)**:
   - `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:2990,3023,3028,4701-4734`: `owner_validate_plan` enforces `new_data_id == active.data`, and `SI-12` (`:4701-4734`) checks that `0 .. active.data_fields` is an **exact contiguous partition** of constructor field slices with zero non-field entries in between.
   - `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime/owner_adt_runtime.rs:1344-1359,1585,1620,1623`: During multi-argument constructor evaluation, `ConstructorFieldsF` holds a partially accumulated `completed: SliceC` in `value_slots` (`start .. start + len`) while recursive evaluation of the next argument allocates arbitrary `DATA`, `FIELDS`, and `ENV` cells before appending to `value_slots` (`families/environment.rs:212-265`). In a 1-arena unified log, those intermediate `DATA`/`ENV` allocations land between `value_slots` copy operations; in 4 separate dense arenas (`DATA`, `FIELDS`, `ENV`, `SLOTS`), `SLOTS` stays 100% contiguous (`addr = start + k`) across arbitrary sub-evaluations and leaf closes with **zero rule changes**.

### 1.3 Fix Lane B Round 1's `W_flush = 1` Simultaneous-Flush Livelock via `W_flush = 2` Shared Dual Walks — Accepting Lane E (`E-12`)

Lane E (`lanes/E/round1.md:182,231-235`) flagged that a single atomic step can append to two arenas simultaneously. Direct inspection of `crates/mori-core/src/sherlock/helper_machine/` confirms the exact per-step allocation bound across all 108 rules:
- `crates/mori-core/src/sherlock/helper_machine/families/adt_build.rs:37-55` (`XAdtHelperBuildField` when `GBuildLast` holds): appends `1` final field cell to `FIELDS` **and** `1` constructor record to `DATA` in the **same** step (`1 DATA + 1 FIELDS`).
- `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:2780-2802,3507-3544` (`derived_delta` & `AllocDataWrite::{Leaf, Cons, Branch}`): `Leaf` appends `1 DATA + 1 FIELDS`; `Cons` and `Branch` append **`1 DATA + 2 FIELDS`** (`3` cells total across `2` arenas) in **one** step.
- Every other allocating rule (`BuildCell`, `AppendCell`, `AppendTail`, `EnvPutInsert`, `BindInsert`, `ScalarConstruct`) appends to **at most 1 arena** (`1` cell).

**Why `W_flush = 1` livelocks and `W_flush = 2` is 100% complete**:
- Suppose at leaf entry (`slot 0`), `pre.len_data mod 8 == 7` and `pre.len_fields mod 8 in {6, 7}`. Executing one `Cons`/`Branch` (`+1 DATA, +2 FIELDS`) or `XAdtHelperBuildField (GBuildLast)` (`+1 DATA, +1 FIELDS`) step pushes **both** `DATA` and `FIELDS` across a multiple of 8 simultaneously! With `W_flush = 1` shared walk, even an `S = 1` leaf cannot prove that step — violating `GP-1` completeness.
- With **`W_flush = 2` shared dual walks** per leaf (`flush_slot[0..1]`, each tagged with `arena_id in {0..3}`), any single step touches at most `2` arenas and appends at most `2` cells to one arena (`off + 2 <= 9`, at most `1` flush per touched arena). Therefore **every reachable single step requires at most `2` flushes** (`S = 1` never blocks), and an `S = 16` leaf simply closes early before slot $j \ge 1$ only in the rare case where slots $0 \dots j$ accumulate a third 8-cell boundary crossing (`<= 8` total cell appends/leaf anyway).

### 1.4 Refine Register-Carried Tail Buffer from `8` to `7` Digests per Arena (`open_r[0..3][0..6]`) — Accepting Lane D

Lane D (`lanes/D/round1.md:120-122`) observed that an un-flushed segment has `off_a = len_a mod 8 in {0..7}`, so at most **7** partial cells (`indices 0..6`) can exist before the 8th cell triggers an immediate flush to `root_a` (resetting `off_a = 0`). Replacing `open_r[0..7]` with **`open_r[0..3][0..6]` (`4 × 7 = 28` field words total)** saves `4` words per state digest (`280` Poseidon rows) and eliminates the unreferenced 8th slot.

### 1.5 Accept Formal `Aggregation.lean` / `aggregation.qnt` Determinism Proof Alignment — Accepting Lane E (`E-5`, with Exact Citation Correction)

We verified `clean/fork/Clean/Sherlock/Aggregation.lean:176-214,1136-1164` and `models/sherlock/quint/aggregation.qnt:89-105`. Under 4-Arena `M3-Tail64` + 2 Hash-Chain Stacks, `post_d(h)` is a **strictly deterministic function of `(prog_d, input_d, h)`**, directly satisfying `d(p, i) = { prog: p, idx: i }` (`aggregation.qnt:89`), `result_unique` (`Aggregation.lean:1136-1150`), and `statement_independent_of_S` (`Aggregation.lean:1154-1164`, proving `root₁.post = root₂.post`) with zero equivalence-relation refactoring. *(Precision correction to Lane E `lanes/E/round1.md:145`: `det` is an explicit hypothesis of `result_unique` at `Aggregation.lean:1137` and `statement_independent_of_S` at `:1155`, rather than a struct field `Premises.det` inside `structure Premises` at `:176-214`.)*

### 1.6 Accept Two-Tier Q3 Continuation Framing (Lanes C & E) and Red-Team Soundness Hardening (Lane E `E-1, E-7, E-8, E-9, E-10, E-11`)

1. **Two-Tier Q3 (`lanes/C/round1.md:274-305`, `lanes/E/round1.md:254-308`)**: Because `M3-Tail64` already makes `post_d(h)` 100% partition-independent (`SRC-023`), **non-compacting multi-session continuations (Tier 1)** do not need canonical relocation or `UNFOLD` rules at all — they fold directly as `SEG + SEG -> SEG` (`architecture.md:84`) or via `SHK_CONT_V1` binding `post_d(h)` at **`0` extra leaf rows**! Lane B/C's address-free `ckpt_d` + lazy 1-level `UNFOLD` becomes **Tier 2**, serving (a) live-heap GC compaction if an arena ever approaches `2^64` or host disk limits, (b) profile `/2` async `Await`/`Resume`, and (c) closing `SRC-021`'s `[OPEN]` non-scan `val_d` input relation (`requirements.md:46`, `lanes/C/round1.md:339-351`).
2. **Mandatory Red-Team Soundness Guards (`lanes/E/round1.md:133-185`)**:
   - **`E-1` (`research/sherlock-gp-architecture-20261110/architecture.md:100`)**: In `SET = (set_root, n_claims)`, `n_claims` counts claim occurrences across DAG merges. Without `< 2^64` range checks on `SET + SET -> SET`, a 64-step (or 255-step) diamond DAG wraps `n_claims = 0 (mod r)`. Every `SET` join MUST range-check `n_claims_out = n_claims_L + n_claims_R < 2^64`, and every `SEG + SEG -> SEG` merge MUST enforce `0 <= h_lo < h_mid < h_hi < 2^64`.
   - **`E-7`**: Domain-separate `DOM_MEM_REC(a)`, `DOM_MEM_SEG(a)`, `DOM_MEM_NODE(a)` by arena index $a \in \{0..3\}$ (`EMPTY(a)` distinct per `design.md:266`).
   - **`E-10` & `E-11` (`crates/mori-chip/src/alu.rs:6-9,111-125`, `rf.rs:34`, `qa/gemini-quint/chipleaf-final/verdict.md:48`)**: Verified in `alu.rs:111-125`: `OP_LT` checks `d = y - 1 - x` (`128`-bit) assuming `x, y < 2^128` by induction from `Init` (`StepInv`, `SVM-002`). Therefore **no continuation epoch (`ResumeInit` or `SHK_CONT_V1`) may ever be sealed standalone without in-circuit verification of its predecessor proof chain back to `h_lo = 0` (`Init`)**, and scratch `v0..v2` (`design.md:267`) MUST be zeroed at `Init` and on disabled slots with strict write-before-read in every rule.

### 1.7 Accept Prior-Art / CVE Additions (Lanes A & E) and Post-Bringup `S`-Amortization Insight (Lane D)

- **Prior art & CVEs**: We incorporate Brevis Pico v3.1 (`has_initial` fresh-zero Merkle write elision), ZKsync Airbender (`2^36` cycles), Lambda VM (*Prove-and-Retire* linear memory), LatticeFold+/Neo/SuperNeo/Symphony, Binius64/Ligerito/Bolt (`lanes/A/round1.md:315-425`), and the 2024–2026 zkVM vulnerability corpus (`GHSA-g6qg-9j8p-7j87`, `GHSA-fh29-29h9-qm9h`, `CVE-2025-46723`, `CVE-2026-62333`, `GHSA-c873-wfhp-wx5m`, `stwo-cairo` Fiat-Shamir bug, Cairo relocation aliasing; `lanes/E/round1.md:312-338`).
- **Leaf step-count `S` (`lanes/D/round1.md:309-340`)**: We accept Lane D's diagnosis (`e2-e4.md:17-22`) that oversubscribed 20t+20t concurrent leaf+node proving (`90.2 s` node) loses to sequential 20t execution (`65 s` node + `~3–6 s` `k = 16..17` leaf), and that raising `S` from `16` to bounded `S = 64..128` (`k_leaf = 18..19`, requiring an amendment to `SRC-014` `S_MAX = 16`) is a `3.2x–5.3x` CPU throughput lever to benchmark before mainnet key freeze (while rejecting unbounded `S = 512` in §2.6).

---

## Section 2: What Lane B Rejects and Why

### 2.1 Reject Lane C's `Wide M3 (TOP = 64) + act[0]` Empty-Leaf Bypass (`lanes/C/round1.md:12,189-199`)

Lane C proposes keeping M3's 8-cell leaf segment (`TOP = 64`) and gating `len_post = base + 8 * act[0]` so zero-append leaves (`act[0] = 0`) advance `len` by `0`. **Rejected on three concrete grounds**:
1. **Allocating leaves with `1..7` appends (`act[0] = 1`) still burn `8 - used` dead padding cells**: Consider a 16-step run that appends 1 cell per step. Under partition `[16]` (two 8-append leaves or one leaf if `C = 16`), `len` advances cleanly; under partition `[5, 11]` (leaf 1 closes after 5 appends, leaf 2 runs 8 + 3), leaf 1 sets `len_post = base + 8` (injecting 3 dead zero cells at addresses `5..7`).
2. **Falsifies `Aggregation.lean` and `aggregation.qnt` without full proof-tree rewrite**: Because `post_d` at step 5 differs depending on whether a leaf closed at step 5 (`len = 8`) or stayed open (`len = 5`), `Wide M3 + act[0]` **violates `statement_independent_of_S`** (`root₁.post = root₂.post` at `clean/fork/Clean/Sherlock/Aggregation.lean:1162`) and `d(p, i)` (`models/sherlock/quint/aggregation.qnt:89`).
3. **Leaves `SI-12` broken AND wastes `+3` tree levels (`TOP = 64` vs `61`)**: Any slice crossing a partial leaf close still gets zero padding injected inside `start .. start + len - 1` (`owner_runtime.rs:4701-4734`), forcing Lane C to also demand Carbon `D-1` linked slices (§2.2), while `TOP = 64` (`2^67` cells) pays `+3` extra Merkle levels (`+1,960..2,800` rows/leaf) purely to compensate for `7/8` padding waste. `M3-Tail64` (`TOP = 61`) achieves `2^64` **true** cells with **zero** padding waste and **deterministic `post_d(h)`**.

### 2.2 Reject Carbon `D-1` Cons-Linked Slices Proposed by Lanes A, C, and D (`lanes/A/round1.md:461`, `lanes/C/round1.md:200-202`, `lanes/D/round1.md:124`)

Lanes A, C, and D propose rewriting `DATA_FIELDS` and `VALUE_SLOTS` from contiguous `(start, len)` arrays (`addr = start + k`) to cons-linked list cells (`(val, next_ptr)`). **Rejected — breaks 1-step random-access projection and `GP-1` completeness**:
1. **Verified $O(1)$ random-access field projection in `mori-core` and `mori-backend`**:
   - `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime/owner_adt_runtime.rs:1430-1454` (`walk_path_from`) executes `let child = pre.field_at(&cur, &fidx)?;` (`fields.start.0 + fidx`) for arbitrary field index `fidx`.
   - `crates/mori-backend/src/sherlock_machine/c6_match_helper_step.rs:307-336` (`BHelperMatchField`) computes `pos = slice_start + M_FIELD_IDX` (`:307`) and reads `data_fields[pos]` in **one atomic owner step**.
2. **Write-once backward linking forces $O(m)$ pointer chasing per projection**: Because Sherlock memory is strictly write-once (`design.md:82`), `XAdtHelperBuildField` (`adt_build.rs:37-55`) appends fields $0, 1, \dots, m - 1$ sequentially and can only link `field[k] -> field[k - 1]` (or `field[0] <- ... <- field[m-1]`). Projecting `fidx = 0` (the first field!) from the slice head `field[m - 1]` in **one** step requires **$m$ sequential `PAGE` Merkle openings**. If a program defines an ADT constructor with arity $m > N_{\text{page\_cell}}$ (e.g., a 9-field record when `N_page_cell = 8`), **not even an `S = 1` leaf can prove `BHelperMatchField`** (`GP-1` completeness failure!). Conversely, splitting `BHelperMatchField` into $m$ pointer-chasing microsteps changes the reference step count `n_steps`, violating `SVM-001` (`requirements.md:9`) and `SRC-006`.
3. **Completely unnecessary under 4-Arena `M3-Tail64`**: Four dense arenas (`DATA, FIELDS, ENV, SLOTS`) preserve exact `start + k` indexing across all 108 rules with **zero** rule changes and **1** `PAGE` (or `0`-path hot-tail `MEM`) lookup per field projection.

### 2.3 Reject Scalar `tail_d[0..3]` Digest Carry (Lanes A & E) in Favor of Register-Carried `open_r[0..3][0..6]` — and Correct Lane E's `19,180`-Row Undercount (`lanes/A/round1.md:203-216`, `lanes/E/round1.md:224-246`)

Lanes A (`W = 15`, 1 arena) and E (`4` arenas) compress each arena's un-flushed partial segment into a single `Tree8` digest `tail_d[a]` inside `pre_d`/`post_d`, whereas Lane B (and Lane D for 1 arena) carries the un-flushed **record digests `open_r[a][0..6]`** directly as state scalars. Exact Poseidon-permutation accounting proves **`open_r[0..3][0..6]` is strictly cheaper and simpler**, and exposes an undercount in Lane E's `19,180` M3 row subtotal:
1. **What Lane E omitted (`lanes/E/round1.md:245`)**: Lane E summed `244` perms (`2` depth-61 walks = `17,080` rows) + `16` perms (`8` new records = `1,120` rows) + `14` perms (`2` `Tree8` = `980` rows) = `19,180` rows. However, if `pre_d`/`post_d` stores only compressed `tail_d[0..3]`, then:
   - For **every** arena $a \in \{0..3\}$ that appends $\ge 1$ cell **without** flushing (or flushes with spillover `post.off_a > 0`), the leaf MUST **open** `pre.tail_d[a]` (`7` `Tree8` perms to authenticate `pre.r[0..6]`) and **re-compute** `post.tail_d[a]` (`7` `Tree8` perms)! When a 16-step leaf touches all 4 arenas (`DATA`, `FIELDS`, `ENV`, `SLOTS`), scalar `tail_d[0..3]` requires **`4 × 7 (open) + 4 × 7 (close) = 56` extra `Tree8` perms (`3,920` rows)**!
   - By contrast, carrying `open_r[0..3][0..6]` (`28` words vs `4` `tail_d` words = `+24` words) adds only **`+12` perms/side × `2` = `24` perms (`1,680` rows)** to `pre_d`/`post_d` and requires **`0` `Tree8` perms** on un-flushed arenas (`14` `Tree8` perms total ONLY for the `W_flush = 2` flushed subtrees) — a net saving of **`2,240` rows/leaf** (`3,920 - 1,680`)!
2. **Hot-tail reads without Merkle walks**: Lane E's `19,180` rows also omitted opening cell tuples `(a, b, c, d)` from the un-flushed open segment `seg_0` written by an immediately preceding leaf. In `M3-Tail64`, provisioning an `8`-row hot-tail witness cache (`8 × 2 = 16` perms = `1,120` rows checked against `pre.open_r[a][idx]`) lets `MEM` answer **any** read to an un-flushed tail cell across all 4 arenas at **`0` Merkle tree walks**.

### 2.4 Reject Lane A's `Halted(Left(ms_v))` (`lanes/A/round1.md:473`) and Lane D's `WithFuel(in_0, fuel_a + fuel_b)` (`lanes/D/round1.md:175,185`) Q3 Continuation Encodings

1. **Why `Halted(Left(ms_v))` is unsound (`Exploit E-8`, `E-10`)**: Emitting an incomplete continuation chunk as `Halted(Left(ms_v))` under the standard `SHK_RUN` tag allows a prover to stop any program after 1 step and submit `Halted(Left(ms_v))` to `acceptRun` (`SRC-007`), spoofing terminal completion with wrong `n_steps` and `outcome_d`. Worse, starting chunk $k+1$ from an unverified `Right(ms_v)` input allows arbitrary out-of-range registers to enter `ALU` `OP_LT` (`alu.rs:111-125`), forging comparisons in 1 step (`Exploit E-10`).
2. **Why `WithFuel(in_0, fuel_a + fuel_b)` is defective (`Exploit E-6`)**: Splitting `fuel` into per-chunk `SHK_IN` values changes `initial_fuel` (`design.md:229`, field 6) vs `consumed_work` (`field 8`), allows chunk 0 (`fuel_a < total_fuel`) to terminate validly as `Exhausted(Fuel)` and settle early on `acceptRun`, and requires `N` to open and re-hash `SHK_IN`. Threading `orig_input_d` unchanged inside a distinct `SHK_CONT_V1` relation tag avoids both flaws.

### 2.5 Reject Lane C's SP1 v6 Guest-RAM Merkle Attribution (`lanes/C/round1.md:415,436,471`)

Lane C (`lanes/C/round1.md:415,436,471`) asserted that SP1 v6 Hypercube *"reintroduced Poseidon2 Merkle roots (`prev_merkle_root`, `merkle_root`) across `2^24`-entry trace chunks"* for guest RAM. **Factually incorrect**: as verified across `succinctlabs/sp1` (`crates/core/machine/src/memory/{global.rs,local.rs}`, `CIRCUITS.md`, `crates/prover/src/verify.rs`), SP1 v6 Hypercube uses **zero Merkle trees for guest RAM**. Cross-shard RAM consistency is enforced purely by **elliptic-curve multiset sums (`SepticDigest`) over the degree-7 extension curve on KoalaBear** (`MemoryLocalChip` + `MemoryGlobalInit/Finalize` telescoping to $\mathcal{O}$ in recursion); `vk_merkle` (`vkRoot`) in SP1 v6 commits to the **admitted recursion verifier-key whitelist**, not guest memory.

### 2.6 Reject Lane C's `2L - 1` Binary-Tree Proof Count (`lanes/C/round1.md:501`) and Lane D's Unbounded `S = 512` (`k = 20`) Sizing (`lanes/D/round1.md:316-324`)

1. **Binary tree proof count (`lanes/C/round1.md:501`)**: Lane C modeled binary tree aggregation of `L` leaves as requiring `2L - 1` `k = 20` node proofs (`1` 1-leaf lift per leaf + `L - 1` binary merges), concluding `3`-way concurrency (`88.6 s/leaf`) is **slower** than serial IVC (`65.5 s/leaf`). However, `research/sherlock-gp-architecture-20261110/architecture.md:75` explicitly defines the run lift `R` (`k = 20`) to verify **1 or 2 `U` leaf proofs** (`U + U -> SEG`)! Pairing leaves in `R` requires $\lceil L/2 \rceil$ `R` proofs + $(\lceil L/2 \rceil - 1)$ `N` proofs = **`2 * ceil(L/2) - 1 ≈ L` `k = 20` proofs** — the **exact same `k = 20` proof count as linear IVC**, so 3-way concurrency (`43.8 s` amortized/proof, `runs.jsonl:618`) achieves **`~47–49 s/leaf` (`1.4x` speedup, not `1.35x` slowdown)**.
2. **`S = 512` (`k = 20`) static chip sizing (`lanes/D/round1.md:316-324`)**: Lane D sized an `S = 512` leaf (`~928,390` rows, `k = 20`) using **mean** E0 event rates (`0.25` cold `PAGE` reads/step = `128` reads, `32` flushes). Because Halo2 chip region heights are fixed circuit constants (`study.md:101-106`), any 512-step window exceeding `128` cold `PAGE` reads or `32` flushes forces an **early leaf close** (e.g., at 200 steps) while still paying the full `66.2 s` `k = 20` leaf proving cost! Conversely, sizing `S = 512` with burst headroom (`192` cold `PAGE` reads = `887,040` rows + `64` flushes = `578,000` rows + `121k` `PROG` + core) requires **`ESTIMATE ~1.63M` rows (`k = 21 > K_LEAF_MAX = 20`)** and penalizes short runs (`entry 7;`, 6 steps) by `+60 s`. Furthermore, `SRC-014` (`requirements.md:39`) currently mandates `S <= S_MAX = 16`, and `design.md:123` records an `[OPEN]` `+16,256` node-row jump from `S = 1` to `S = 16`. Therefore, **`S = 16` (`k_leaf = 16..17`) must remain the M4 bring-up baseline**, and **`S = 64..128` (`k_leaf = 18..19`, `ESTIMATE ~189k–373k` rows)** should be tested as a dedicated speed-lever experiment before mainnet key freeze.

---

## Section 3: Lane B's Self-Contained Final Position on Q1 to Q5

### 3.1 Q1: What `2^64` Is, Honest Meaning of "Unbounded", and Exact `GP-1` / `GP-001` Replacement Text

#### 3.1.1 Why `2^64` Is a Counter Non-Wraparound Guard — Never a Circuit Size
In `crates/mori-recursion/src/ivc.rs:77-79,363-376,497-501` and `requirements.md:29` (`SRC-004`), `2^64` is strictly an **integer range bound on the accumulated step counter `h`**, checked in 4 constraints per node step so `h = prev_h + steps` cannot wrap modulo the BLS12-381 scalar prime $r \approx 2^{255}$ back to `0` and spoof `is_genesis = 1` (`clean/fork/Clean/Sherlock/Aggregation.lean:1211-1213`). **No circuit ever has $2^{64}$ rows**: the universal chip leaf has $2^{16}..2^{17}$ rows (`65,536..131,072`) and the recursive node and decider have $2^{20} = 1{,}048{,}576$ rows (`e2-e4.md:28`). Proving work scales linearly $\Theta(N)$ with executed steps $N$, so proving $2^{64}$ steps is physically impossible (`BRIEF.md:6-7`).

#### 3.1.2 Honest Three-Part Meaning of "Unbounded" (Preserving All of `GP-1..GP-4`)
1. **Key-independent universality (zero capacity baked into keys)**: The fixed verifying-key set contains **no constant** capping program size, stack depth, environment size, heap size, or step count. Growing a program or workload never triggers a new key, ceremony, or contract (`general-purpose.md:9-15`).
2. **Linear-work intra-run folding ($O(N)$ prover, $O(1)$ circuit & on-chain proof)**: Each leaf proves $1 \le \text{steps} \le S$ steps against commitment-authenticated program and memory state (`prog_root`, hash-chain stacks, and 4 dense Merkle arenas), folded in chains or balanced trees (`GP-4`). Arithmetic counter widths (`h < 2^64` in `SRC-004` and `len_a < 2^64` in `M3-Tail64`) exist solely to prevent field wraparound and exceed any physically executable run.
3. **Cross-run continuation composition (`GP-2`)**: Long, suspended, or GC-compacted executions also compose across runs via authenticated continuation claims (`SHK_CONT_V1`) into **one** constant-size on-chain proof with identical `[prog_d, input_d, outcome_d, n_steps]` (`SRC-006`, `SRC-023`).

#### 3.1.3 Exact Drop-In Replacement for `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:9-17` (`GP-1`)
*(Preserves lines 7–13 structure verbatim, replaces the misleading line 14 bullet, updates line 16 acceptance, and leaves `GP-2..GP-4` untouched.)*

```markdown
One fixed key set proves the execution of every program of the Sherlock language, on every input, for any physically realizable finite run length and every outcome. A new program or larger run needs no new key, no new contract and no new ceremony.

- Programs and inputs are data. The public statement carries `prog_d` (program identity) and `input_d` (run input). Neither is a circuit constant.
- Every outcome is a provable statement: `Halted(v)` for every value type (the result as a digest of a canonical value encoding), `Rejected(code)` and `Exhausted(kind)`.
- Capacity does not depend on the key. A program whose state does not fit one leaf's carriers runs against committed memory: state lives behind commitments, and each step proves its reads and writes against them. A fixed ladder of capacity keys is not universal.
- Run length and memory scale by folding fixed-size leaf proofs (`O(N)` prover work, constant circuit and final proof sizes), never by growing a circuit; long or resumable executions also compose across runs through GP-2 continuation claims. The `2^64` bounds on step height (`SRC-004`) and committed-memory counters are field-integer non-wraparound guards, not circuit sizes, and exceed any physically executable run.

Acceptance: one key set and one deployed verifier contract accept proofs of at least three programs that the key set was not built from, including one program larger than one leaf's carriers, one multi-leaf run exceeding single-leaf step and memory budgets (`>= 4,096` steps and `>= 512` allocated cells), one multi-epoch continuation chain folded into a single proof, one run with input, and one run of each outcome kind.
```

#### 3.1.4 Exact Drop-In Replacement for `GP-001` in `requirements.md:56` and `specs/sherlock-general-purpose/spec.md:7-15`

**(a) `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:56`:**
```markdown
| GP-001 | U | GP-1 universal VM. One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every finite run proved by folding fixed-size leaf proofs within the `2^64` field-integer non-wraparound guards on step height (`SRC-004`) and committed-memory counters (and across continuation epochs under GP-002), and for every outcome (`Halted(v)` for every value type, as a digest of a canonical value encoding; `Rejected(code)`; `Exhausted(kind)`); the public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants; capacity and circuit sizes SHALL NOT depend on run length, memory size or the program (state larger than one leaf's carriers runs against committed memory); a new program or larger workload SHALL need no new key, contract or ceremony. When a supported run exceeds one leaf's step, storage or hashing capacity, or spans continuation checkpoints, the prover SHALL continue through authenticated memory, recursive segment folding and/or GP-002 continuation claims under the same key set, producing one constant-size proof satisfying SRC-006 and SRC-023. | One key set and one deployed verifier contract accept proofs of at least three programs the key set was not built from, including one program larger than one leaf's carriers, one multi-leaf run (`>= 4,096` steps and `>= 512` allocated cells), one 2-epoch continuation chain folded into one proof with unsplit `claim_d` and `n_steps` parity, one run with input, and one run of each outcome kind (row A33). | `general-purpose.md` GP-1; SRC-017; C6 review 2026-11-10; runlength consensus 2026-11-11 |
```

**(b) `openspec/changes/sherlock-vm-consensus-20261107/specs/sherlock-general-purpose/spec.md:7-15`:**
```markdown
### Requirement: GP-001 Universal VM
One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every finite run proved by folding fixed-size leaf proofs within the `2^64` field-integer non-wraparound guards on step height (`SRC-004`) and committed-memory counters (and across continuation epochs under GP-002), and for every outcome (`Halted(v)` for every value type as a digest of a canonical value encoding, `Rejected(code)`, `Exhausted(kind)`). The public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants. Capacity and circuit sizes SHALL NOT depend on the key, program size, memory size or run length: a program whose state or trace exceeds one leaf's carriers runs against committed memory and recursive proof composition. A new program or larger workload SHALL need no new key, contract or ceremony.

When a supported run exceeds one leaf's step, storage or hashing capacity, or spans continuation checkpoints, the prover SHALL continue through authenticated memory, recursive segment folding and/or GP-002 continuation claims under the same key set, producing one constant-size proof whose published statement satisfies SRC-006 and SRC-023.

#### Scenario: Programs and lengths the keys were not built from
- **WHEN** one key set and one deployed verifier contract receive proofs of at least three programs the key set was not built from, including one program larger than one leaf's carriers, one multi-leaf run (`>= 4,096` steps and `>= 512` allocated cells), one 2-epoch continuation chain folded into one proof, one run with input, and one run of each outcome kind.
- **THEN** every proof is accepted with no new key and no new contract, and the continuation proof produces the same `claim_d` and `n_steps` as an unsplit run of the same `(prog_d, input_d)` (row A33).
```

---

### 3.2 Q2: Chip Leaf Memory Design and Exact Cost-Fit Model (`M3-Tail64`)

#### 3.2.1 Verified Baseline Cost Laws (`mori-chip`, `midnight-proofs 0.8.3`, `e2-e4.md`)
- **Usable `rows_count` per `k` (`circuit.rs:2346-2376`, `cost_model.rs:370-381`)**: `blinding_factors = 5`, `nb_unusable_rows = 6` $\implies$ `k = 13: <= 8,186`; `k = 14: <= 16,378`; `k = 15: <= 32,762`; **`k = 16: <= 65,530`**; **`k = 17: <= 131,066`**; `k = 20: <= 1,048,570`.
- **E8 Spike (`rfcore.rs:1119-1243`, `m3.rs:50-55`, `poseidon.rs:56-59`)**: `6,511` rows (`221` Narrow core + `192` ALU/RANGE + `8` SegMem + `87 × 70 = 6,090` Poseidon rows), `12` advice, `13` perm, `20` fixed, `4` lookups, `max_deg = 6`.
- **Node & Decider (`design.md:134`, `experiments-2.md:125,223-230`, `e2-rows.jsonl:1-7`)**: Baseline node over E8 leaf is `823,466` rows (`k = 20`); `+1` in `k_leaf` adds **exact `+4` node rows** and **`+0` decider rows** (`decider = 929,166` circuit rows / `1,032,467` table rows, `k = 20`). `WalkConfig` (`m3.rs:104-144`) bit-checks `idx < 2^TOP` and `ALU`/`RANGE` (`alu.rs:1-9`, `range.rs:33-35`) check 128-bit limbs with **`+0` extra advice columns and `+0` lookups** for any `TOP <= 125`.

#### 3.2.2 Comparison of Options `(a)`, `(b)`, `(c)`, and Final Synthesis `(d) M3-Tail64`

| Option | Single-Run Ceiling | Full Leaf Rows at `S = 16` (`k_leaf`) (`ESTIMATE`) | Node / Decider Rows (`k = 20`) (`ESTIMATE`) | `SI-12..14` & `SI-23` | `SRC-023` & `Aggregation.lean` `det` | Verdict |
|---|---|---|---|---|---|---|
| **(a) Keep D3 (`TOP=17`) + GP-2 Cont.** | `131,071` leaves; `< 2^20` padded cells | `~31,291..33,500` (`k = 15..16`) | `~823,474` / `929,166` | **Broken** (holes) | **Broken** (`post_d` partition-dep.) | **Reject**: live sets `> 2^20` unprovable; pure compute capped at `131k..2.1M` steps |
| **(b) Naive Widen D3 (`TOP = 24..64`)** | Spike crosses `k=13->14` at **`TOP=29`** (`8,191 > 8,186`). `TOP=61`: `2^61` leaves | `TOP=32`: `41,791` (`k=16`); `TOP=61`: `62,091` (`k=16`); `TOP=64`: `64,191..82,951` (`k=16..17`) | `823,478..823,482` / `929,166` | **Broken** (leaf-close holes) | **Broken** (`post_d` partition-dep.) | **Reject alone**: widening `TOP` is cheap in `N`/`D`, but `len = base + 8` still injects holes |
| **(c.1) `Wide M3 + act[0]`** (Lane C) | Unbounded pure compute; `2^64..2^67` padded cells | `~64,191..82,951` (`k = 16..17`) | `~829,940` / `929,166` | **Broken** on `1..7`-append leaves | **Broken** (`[16]` vs `[5,11]` differ) | **Reject** (§2.1): fails `Aggregation.lean:1162` & `SI-12` |
| **(c.2–c.4) `M2` / Dirty Pages / Dynamic MMR** | Up to `2^64` cells | `55k..100k+` (`k = 16..17`) | `836k..843k` / `929,166` | Fixed if dense `len` | Deterministic if dense | **Subsumed / Risky**: dynamic depth caused `CVE-2026-62333`; `M3-Tail64` is simpler & smaller |
| **(c.5) LogUp / Twist-Shout / Curve Multiset** | Up to `2^64` | Blocked (`MC:verifier_gadget.rs:16-17`) or needs `M/C` finalization leaves | N/A on stock stack | Supports dense RAM | Needs global sweep | **Reject on pinned stack**: no challenge phases; curve sweep doubles `65s` node folds |
| **(d) `M3-Tail64` (4 Dense Arenas `TOP=61`, `W_flush=2`, `open_r[4][7]` + 2 Hash-Stacks)** | **`h < 2^64` steps**; **`2^64` dense cells/arena**; **`0` cells for stacks/compute** | **`58,050..62,670` (`k=16`, `N_page=4..5`)**; **`76,530..79,330` (`k=17`, `N_page=8`)** | **`~829,940..836,400`** / **`929,166`** | **100% Native** (`start + k` exact) | **100% Exact** (`post_d(h)` pure fn of `h`) | **ADOPT**: complete, zero holes, zero rule rewrites, fits `k_leaf = 16..17` |

#### 3.2.3 Architecture of Recommended Design `(d) M3-Tail64`
1. **Two Cached-Top Poseidon Hash-Chain Stacks (`FRAMES`, `TASKS`) (`design.md:239-242,265`)**:
   - `top_frame` (13 lanes), `fp` (`frame_h`), `top_task` (7 lanes), and `tp` (`task_h`) live in scalar carriers (`NIL = EMPTY_STACK`).
   - `SETTOP` rewrites registers (`0` hashes, `0` cells). `PUSH` hashes `(DOM_FRAME/DOM_TASK, old_top..., fp)` (`4..7` perms); `POP` witnesses parent lanes and verifies the hash (`4..7` perms vs `66` perms for a depth-64 Merkle path!). Stack traffic consumes **0 arena addresses**.
2. **Four Dense Tail-Buffered Merkle Arenas (`a in {DATA=0, FIELDS=1, ENV=2, SLOTS=3}`, `C = 8`, `TOP = 61` $\implies 2^{64}$ Cells Each)**:
   - Each arena maintains `(root_a, len_a, open_r[a][0..6])` in `pre_d` and `post_d`, with `len_a < 2^64` (`seg_a = floor(len_a / 8) < 2^61`, `off_a = len_a mod 8 in 0..7`, `open_r[a][k] = 0` for `k >= off_a`, and domain-separated `EMPTY(a)` roots per `E-7`).
   - **Strict dense advance**: `post.len_a = pre.len_a + used_a` (`0 <= sum_a used_a <= 8`). **Leaf close NEVER pads `len_a`.**
   - **`W_flush = 2` shared dual walks (`GP-1` livelock-free)**: Whenever `off_a + used_a >= 8` for an arena $a$, one of the `2` shared flush units combines `pre.open_r[a][0..off_a-1]` with the `8 - off_a` completing record digests (`7` `Tree8` perms $\to \text{seg\_root}$), runs the exact M3 dual walk at `idx = seg_a < 2^61` (`EMPTY_SEG(a) -> pre.root_a` and `seg_root -> post.root_a`, `122` perms), and shifts any spillover `(off_a + used_a - 8)` digests into `post.open_r[a][0..]`. Unused flush units run a gated `0 -> 0` dummy walk (`post.root_a = pre.root_a`). Because no step touches $> 2$ arenas (`adt_build.rs:37-55`, `owner_runtime.rs:2780-2802`), `W_flush = 2` never blocks at slot 0.
   - **Disjoint `D2` Read Routing (Hot-Tail `MEM` + Cold `PAGE`)**: An `8`-slot hot-tail witness table authenticates up to `8` pre-existing un-flushed cells against `pre.open_r[a][k]` (`16` perms) alongside the `8` newly appended cells in `MEM`. Reads to `(a, addr)` with `addr >= seg_a * 8` hit `MEM` in **`0` Merkle paths**; cold reads with `addr < seg_a * 8` hit `PAGE` against `pre.root_a` (`floor(addr / 8) < seg_a`), preserving `D2` disjointness (`quint-check.md:332-338`).

#### 3.2.4 Exact Block-by-Block Row Budget for `M3-Tail64` (`S = 16`, `TOP = 61`, `ESTIMATE`)

| Leaf Block (`12` advice, `4` lookups, `max_deg = 6`) | Permutations (`70` rows/perm) | Exact / `ESTIMATE` Rows | Basis & Source |
|---|---:|---:|---|
| **1. Core (`17` slot boundaries, ~96 words) + `ALU/RANGE` (`N_alu = 24`) + `16`-row `MEM` + mux** | `0` | **`1,350`** (`ESTIMATE`) | `rfcore.rs:1204-1208`, `range.rs:33-35`, `mem.rs:123` |
| **2. Boundary `pre_d` + `post_d` (~96 words: 64 VM scalars + 4 `root_a` + 28 `open_r`)** | `48 × 2 = 96` | **`6,720`** (`ESTIMATE`) | `design.md:222-263` + `28` carry words |
| **3. Hot-tail `pre` cell openings (`8` cells × `2` perms) + new `8` cell records (`8 × 2` perms)** | `16 + 16 = 32` | **`2,240`** (`ESTIMATE`) | `m3.rs:50-51` (`2` perms per 4-lane cell record) |
| **4. `W_flush = 2` `Tree8` subtrees (`2 × 7` perms) + depth-`61` dual walks (`2 × 122` perms)** | `14 + 244 = 258` | **`18,060`** (`ESTIMATE`) | `m3.rs:52-55` scaled to `W_flush = 2, TOP = 61` |
| **5. `SVM-012` `vd` creation (`<= 8` allocs) + Hash-Stack `PUSH`/`POP` (`4` ops × `7` perms)** | `24 + 28 = 52` | **`3,640`** (`ESTIMATE`) | `review.md:135-136`, `design.md:239-242` |
| **6. `PROG` `PAGE` Merkle openings (`6` distinct rows × `18` perms at depth `16`)** | `108` | **`7,560`** (`ESTIMATE`) | `experiments-3.md:205` (`E0` max `<= 5` distinct/16 steps) |
| **Subtotal before Cold Arena `PAGE` Reads** | **`546` perms** | **`39,570`** (`ESTIMATE`) | |
| **7a. Cold Arena `PAGE` Reads: `N_page_cell = 4` (`4 × 66` perms at `TOP = 61`)** | `+264` (`810` tot) | **`58,050` (`k_leaf = 16`)** (`ESTIMATE`) | `7,480` rows below `65,530` `k=16` cap (`E0` mean ~4/leaf) |
| **7b. Cold Arena `PAGE` Reads: `N_page_cell = 5` (`5 × 66` perms at `TOP = 61`)** | `+330` (`876` tot) | **`62,670` (`k_leaf = 16`)** (`ESTIMATE`) | `2,860` rows below `65,530` `k=16` cap |
| **7c. High-Burst Option: `N_page_cell = 8` (+ optional `~2,800` in-leaf `Init` rows)** | `+528 (+40)` | **`76,530..79,330` (`k_leaf = 17`)** (`ESTIMATE`) | `> 51,700` rows below `131,066` `k=17` cap (`K_LEAF_MAX=20`) |
| **Node (`k = 20`) / Decider (`k = 20`) Impact (`+1..2` fixed cols, `k_leaf = 16..17`)** | — | **Node `~829,940..836,400`** / **Decider `929,166`** (`ESTIMATE`) | `experiments-2.md:125`, `e2-rows.jsonl:1-7` (`>113k` under `950k` gate) |

---

### 3.3 Q3: Two-Tier Continuations, `input_d` Threading, GP-2 Single-Proof Aggregation, and C6 Privacy

1. **Tier 1 — Zero-Overhead Deterministic Continuations (`post_d(h)` via `SEG` or `SHK_CONT_V1`)**:
   - Because `M3-Tail64` makes `post_d(h)` a deterministic function of `(prog_d, input_d, h)` (`§1.5`), any suspended prover session that retains its external arena store simply resumes from `pre_d = post_d(h_k)` and folds either directly as `SEG + SEG -> SEG` (`0 <= h_lo < h_mid < h_hi < 2^64`) or as a domain-separated internal claim `H("SHK_CONT_V1", prog_d, orig_input_d, h_lo, h_hi, pre_d, post_d, deps_d)` at **`0` extra leaf rows and `0` new VM rules**.
2. **Tier 2 — Address-Free Canonical Checkpoint `ckpt_d` (`SHK_CKPT_V1`) + Lazy 1-Level `UNFOLD` (Compaction, `/2` Async, and `SRC-021` Input)**:
   - For dead-heap GC compaction across multi-TB runs, `/2` async suspension, and `SRC-021`'s `[OPEN]` non-scan `val_d` input mechanism (`requirements.md:46`), maintain incremental address-free canonical digests at cell creation (`vd` per `SVM-012`, `env_cd`, `slice_cd`, `frames_cd`, `tasks_cd`) so `ckpt_d = H("SHK_CKPT_V1", prog_d, control_cd, init_fuel, rem_fuel, consumed_work, helper_work, stack_limit, alloc_d, frames_cd, tasks_cd, helper_cd, result_cd, deps_d)` contains **zero physical addresses** (`alloc_d` preserves exact `CapKind` exhaustion semantics, `SVM-010`).
   - On resume (and at `h_pre = 0` for `val_d` under `SRC-021`), `ResumeInit` binds `ckpt_d` in $O(1)$ rows without scanning memory, and authenticates compacted/input cells lazily on first dereference via 1-level `UNFOLD` checking `vd` / `env_cd`.
3. **Mandatory Soundness & C6 Settlement Rules (Closing `E-1, E-2, E-6, E-8, E-9, E-10`)**:
   - **`orig_input_d` invariant**: Every epoch carries the **unmodified genesis `orig_input_d = H("SHK_IN", val_d_0, fuel_lo_0, fuel_hi_0, stack_limit_0)`**; fuel and `alloc.*` come strictly from the authenticated predecessor state (`no fuel reset`, `no WithFuel mutation`).
   - **In-circuit ancestry back to `h_lo = 0` (`E-10`)**: A `SHK_CONT_V1` claim is **never** accepted standalone on-chain (`tag != SHK_RUN`) and **never** starts an unverified `StepInv` chain; `N` / `C` (`architecture.md:84`) discharges `(h_0=0 -> h_1) + ... + (h_{K-1} -> h_K, terminal)` privately in-circuit before emitting `SHK_RUN`, with `< 2^64` range checks on both `h` and `SET.n_claims` (`E-1`).
   - **C6 outcome-only preservation (`SRC-006`, `SRC-007`, `SRC-023`)**: Only the final terminal proof exposes `[prog_d, orig_input_d, outcome_d, n_steps]`, producing the **exact same `claim_d` and `n_steps`** as an unsplit run while keeping all intermediate `post_d` / `ckpt_d` digests private.

---

### 3.4 Q4: Synthesized Prior-Art Matrix and 2024–2026 zkVM Soundness Lessons

#### 3.4.1 Cross-zkVM Architecture Matrix
| System | Segment / Shard Cut | Run & RAM Ceiling | Cross-Shard Memory Mechanism | On-Chain Statement (Partition-Indep.?) |
|---|---|---|---|---|
| **RISC Zero v3.x** (`https://dev.risczero.com/proof-system/`) | `po2` $\in [13,24]$ (default $2^{20}$) | Unbounded segments; `4 GiB` (`2^32` B, 1 KiB pages) | Depth-22 Poseidon2 Binary Merkle `PageTable` in `SystemState` | `(seal, imageId, journalDigest)` $\to$ `Halted` zeroes `postStateDigest` (**Yes**) |
| **SP1 v6 Hypercube** (`https://docs.succinct.xyz/docs/sp1/hypercube/`) | Jagged PCS budget (`~2^22`) | **48-bit clk** (`2^48`); **48-bit RV64 addr** | **Zero RAM Merkle**: `MemoryLocal` + **KoalaBear $\mathbb{F}_{p^7}$ `SepticDigest` curve sum** (`global.rs`) | `[programVKey, sha256(publicValues)]` (`ISP1Verifier.sol`) (**Yes**) |
| **OpenVM v2.1** (`https://github.com/openvm-org/openvm`) | Metered trace/RAM/`2^29` clk | Unbounded segments; `512 MiB` (`29b`, `CHUNK=8`) | Offline `MemoryBridge` $\to$ **depth-29 Poseidon2 Binary `MemoryMerkleAir`** | `(publicValues, proofData, appExeCommit, appVmCommit)` (**Yes**) |
| **Brevis Pico v3.1** (`https://docs.brevis.network/`) | Dynamic shape (`~2^20..2^22`) | Unbounded chunks; 32-bit RV32IM RAM | Timestamped rw-bus $\to$ lazy paged Poseidon2 Merkle (`has_initial` skips fresh-zero pre-reads, matching `M3` `EMPTY_SEG`!) | `(riscvVKey, publicValuesDigest, proof)` (**Yes**) |
| **Ziren v2.0 (zkMIPS)** (`https://eprint.iacr.org/2026/2330`) | `clk < 2^26` / chip `< 2^22` | `32b shard × 2^26 clk`; 32b RAM + regs `0..35` | KoalaBear LogUp + Jagged WHIR $\to$ **Septic-Curve `global_cumulative_sum`** | `[zkm_vkey_hash, committed_values_digest, vk_root]` (**Yes**) |
| **Valida** (`https://arxiv.org/html/2505.08114v1`) | Default $2^{20}$ rows/segment | BabyBear clk; `960 MiB` (`[0,(p-1)/2)`) | Stack/frame-only (`fp`, no GPRs); `PersistentSend/Receive` bus | Wrapped: `(vk, output, pc_init, fp_init, did_stop)` (**Yes** when wrapped) |
| **Cairo / Stwo** (`https://eprint.iacr.org/2024/278`) | M31 domain ($< 2^{31}$ rows) | Unbounded via Bootloader; `< 2^31` addrs/proof | Relocated write-once `segment:offset -> [1..M]` LogUp + L1 `MemoryPageFactRegistry` | `factHash = keccak256(programHash, outputHash)` (**Yes** at FactRegistry) |
| **zkWasm** (`https://github.com/DelphinusLab/zkWasm`) | Halo2 $2^k$ ($k \in [18,22]$) | Unbounded slices; `31.2 MiB` KZG + `4 GiB` SMT | `mtable` $\to$ `img_col <-> post_img_col` KZG commitment link + depth-32 SMT | `(init_img_kzg, post_img_or_io)` (slice count hidden: **Yes**) |
| **Miden VM** (`https://docs.miden.xyz/reference/miden-vm/design/chiplets/`) | **`MAX_CYCLES = 2^29`** / proof | Unbounded across recursive Tx/Batch/Block kernels | `MemoryChiplet` (`2×16b` clk delta) + depth-64 `MerkleStore` SMT | 40-felt `ExecutionClaim` (`prog, kernel, in, out`) (**Yes**) |
| **Jolt v0.2+ / Nexus 3.0** (`https://eprint.iacr.org/2025/105`, `https://github.com/nexus-xyz/nexus-zkvm`) | Monolithic `2^k` / M31 `< 2^30` | Single run (`max_trace_length` / `< 2^30`) | Jolt: **Twist & Shout** sum-check (`BlindFold`); Nexus 3.0: Stwo LogUp (`ram_init_final`) | Transcript absorbs `trace_length`/`log_size` (**No** without wrapper) |

*(2025–2026 Folding / Binary-Field Notes)*: **Nebula** (`https://eprint.iacr.org/2024/1605`) folds commitment-carrying offline memory in IVC; **LatticeFold+ / Neo / SuperNeo / Symphony** (`https://eprint.iacr.org/2025/247`, `2025/294`, `2026/242`, `2025/1905`) and **Binius64 / Ligerito** (`https://eprint.iacr.org/2025/938`, `2025/697`) eliminate large-prime embedding overhead, relevant only to post-Midnight-0.8.3 architectures.

#### 3.4.2 2024–2026 Soundness Advisories and Direct Mitigations in Sherlock `M3-Tail64`
- **OpenVM `CVE-2026-62333`** (`https://osv.dev/vulnerability/CVE-2026-62333`, dynamic Merkle subtree forgery at `parent_height = 0`) $\implies$ **Mitigated**: `M3-Tail64` uses strictly **fixed depth `TOP = 61`** with zero dynamic-height expansion.
- **RISC Zero `CVE-2025-46723` / SP1 `GHSA-g6qg-9j8p-7j87` / Jolt `GHSA-c873-wfhp-wx5m` / `stwo-cairo` Fiat-Shamir bug** (`https://nvd.nist.gov/vuln/detail/CVE-2025-46723`, `https://github.com/succinctlabs/sp1/security/advisories/GHSA-g6qg-9j8p-7j87`, `https://github.com/a16z/jolt/security/advisories/GHSA-c873-wfhp-wx5m`, `https://blog.zksecurity.xyz/posts/stwo-fiat-shamir/`) $\implies$ **Mitigated**: `WalkConfig` + 128-bit `RANGE` enforce canonical bounds (`E-1, E-3`), `v0..v2` are zeroed/write-before-read (`E-11`), and all public statement lanes bind to the Halo2 verifier transcript (`SRC-001`, `SRC-006`).
- **Cairo Segment Relocation Aliasing** (`https://blog.zksecurity.xyz/posts/cairo-memory-relocation/`) $\implies$ **Mitigated**: `M3-Tail64` assigns exact dense addresses in-step with **zero post-facto relocation**.

---

### 3.5 Q5: Practical Run Length on This Hardware and Concrete Tested Specification Targets

#### 3.5.1 Host Envelope and Proving Regimes (`prover.md:15-17`, `runs.jsonl:617,618,709`, `architecture.md:75`)
- **Host**: 20-core `aarch64` (`10× Cortex-X925 + 10× Cortex-A725`), `120.3 GiB` unified LPDDR5X (`48 GiB`/proc, `60 GiB` default operational / `100 GiB` nominal aggregate cgroup caps, `SRC-016`; GPU `sm_121` parked).
- **Measured component baselines**: `k = 20` node solo (`20t`): **`61.1–67.0 s`**, `16.6–17.6 GB` RSS; 3-way concurrent (`3 × 6t`): **`43.8 s` amortized/proof**, `47.7 GB` aggregate RSS; `k = 20` decider: **`70.4–77.9 s`**, `17.4 GB` RSS; fixed run setup + decider + `SRC-010` + `SRC-020` tail $T_{\text{fixed}} \approx$ **`ESTIMATE 160–180 s`**.
- **Three throughput regimes (`ESTIMATE`)**:
  1. **M4 Baseline — `S = 16` (`k_leaf = 16..17`, `~3–6 s` leaf), Sequential `20t`**: `~68–71 s/leaf` $\implies$ **`~4.25–4.44 s/step`** (`S = 16` full) or `~5.1 s/step` ($\bar{s} \approx 13.5$); peak RSS **`~17.6–19 GB`**.
  2. **M5 Tree — `S = 16`, 3-Way Concurrent `GP-4` (`R` 2-leaf lift + `N` binary merge $\implies \approx L$ `k = 20` proofs)**: `~47–49 s/leaf` $\implies$ **`~2.95–3.10 s/step`**; aggregate RSS **`~48–52 GB <= 60 GiB`**.
  3. **Post-Bringup `S`-Lever Experiment — `S = 64` (`k_leaf = 18`, `~189k` rows, `~17 s` leaf) or `S = 128` (`k_leaf = 19`, `~373k` rows, `~34 s` leaf), Sequential `20t`** *(requires amending `SRC-014` `S_MAX = 16`)*: `~82 s / 64 =` **`~1.28 s/step`** (`S = 64`) or `~99 s / 128 =` **`~0.77–0.85 s/step`** (`S = 128`); peak RSS **`~18 GB`**.

#### 3.5.2 Practical Run-Length Scaling Table on This Host (`ESTIMATE`)

| Workload / Step Count $N$ | Leaves (`S = 16`) | Regime 1 (`S=16` Serial `20t`) (`ESTIMATE`) | Regime 2 (`S=16` `3×6t` Tree) (`ESTIMATE`) | Regime 3 (`S=128` Serial `20t`, Post-M4) (`ESTIMATE`) | Peak Aggregate RSS (`ESTIMATE`) |
|---|---:|---:|---:|---:|---:|
| **`entry 7;` (`6` steps, Gate `G-B`)** | `1` | **`~4.0 min`** (`< 20 min` cap) | `~4.0 min` | `~3.6 min` | `~18 GB` |
| **`fact(3)` / `list123` (`293–303` steps, Gate `G-C`)** | `19..23` | **`~25–30 min`** (`< 2 h` cap) | **`~18–22 min`** | **`~7–8 min`** | `18 GB` / `50 GB` |
| **`GP-001` Endurance Soak (`4,096` steps)** | `256..300` | **`~5.1–5.9 h`** (`< 6 h` cap) | **`~3.4–4.0 h`** | **`~58–68 min`** | `18 GB` / `50 GB` |
| **1-Day Single-Host Run (`~20,000..29,000` steps)** | `1,250..1,800` | `~24 h` (`~20k` steps) | `~24 h` (`~29k` steps) | `~24 h` (`~100k` steps) | `18 GB` / `50 GB` |
| **Old D3 Ceiling (`2^20 = 1,048,576` steps)** | `65,536` | `~53 days` | `~36 days` | `~10.3 days` | `18 GB` / `50 GB` |
| **`2^64` Counter Bound (`1.84 × 10^19` steps)** | `1.15 × 10^18` | **`~2.48 × 10^12` years** | `~1.73 × 10^12` years | `~4.8 × 10^11` years | Physically impossible |

#### 3.5.3 Four Concrete Tested Targets to Name in the Specification
1. **Gate `G-B` (`SRC-015(b)`, Smoke & Multi-Leaf Seam)**: `entry 7;` (`6` steps, 1 leaf) **plus** one `32`-step (`>= 2`-leaf) test proved end-to-end (leaves + fold + decider + `SRC-010` + `SRC-020`) in **`< 20 minutes`**, `<= 48 GiB`/proc.
2. **Gate `G-C` (`SRC-011` / `SRC-015(c)`, Mandatory Release Gate — Keep Unchanged)**: `fact(3)` (`293` steps) and `list123` (`303` steps, `<= 320` steps) proved end-to-end in **`< 2 hours`**, `<= 48 GiB`/proc, `<= 100 GiB` (`60 GiB` default operational) aggregate RSS.
3. **Milestone M5 `GP-001` & `GP-002` Endurance Soak (Rows `A33b`, `A34c`)**:
   - **`A33b` (`GP-001`)**: One recursive + heap-allocating program of **`>= 4,096` owner steps (`>= 256` leaves at `S = 16`) and `>= 512` allocated arena cells** (`>= 64` 8-cell top-tree flushes + cold `PAGE` reads + hash-stack spills) proved end-to-end in **`< 6 hours`**.
   - **`A34c` (`GP-002` Continuation)**: One **`>= 1,024`-step** run proved both unsplit and as **2 chained `>= 512`-step continuation epochs** in **`< 2 hours`**, asserting bit-identical `[prog_d, input_d, outcome_d, n_steps]` and `claim_d` replay refusal on the second submission.
4. **Partition-Independence & High-Boundary Soundness Suite (`SRC-023` Row `A38` + `SRC-004` / `M3-Tail64` Unit Gates)**:
   - **`A38`**: One `>= 64`-step allocating program proved under **3 partitions** (`S = 16`, `S = 1`, mixed early closes) in **`< 90 minutes`**, asserting identical `post_d(64)`, `outcome_d`, `n_steps`, and `claim_d`.
   - **Fast `MockProver` boundary suite (`< 5 min`)**: Positive/negative leaf and node boundary tests seeded at `h_pre = 2^64 - 17` (valid `+16`) vs `2^64 - 15` (refused wrap), `len_a = 2^20 + 5` (exceeding old D3 `2^20` ceiling), `len_a = 2^64 - 8` vs `2^64 - 7` (`TOP = 61` overflow refused), `SET.n_claims` `2^64 - 1` wrap refused (`E-1`), and simultaneous `DATA + FIELDS` `0 mod 8` dual flush (`W_flush = 2` positive).

---

## What Was Not Checked

In compliance with the study rules (`BRIEF.md:42,57-77`), the following items were **not** executed or empirically measured:
1. **No builds, tests, `cost_model` runs, `quint`, `lake`, or proof runs were executed**: All row counts for `M3-Tail64`, `Node`, and `S = 64..128` beyond committed repo receipts (`6,511` E8 leaf, `741,708..741,736` E2 node, `823,466` chip-key node, `929,166` decider) are algebraic `ESTIMATE` projections from `crates/mori-chip/src/{rfcore.rs,m3.rs,poseidon.rs,range.rs,alu.rs}` and the E2/E4/E5/E7/E8 fits.
2. **Trace simulation of effective $\bar{s}$ on `M3-Tail64` (`W_flush = 2`, `N_page_cell = 4..8`)**: Exact early-close rates across all 108 rules on `fact(3)`, `list123`, and `>= 4,096`-step workloads were not simulated.
3. **Open `design.md` items (`[OPEN]` `+16,256` `S=1->16` node jump at `design.md:123`; `ENTRY`/`ARG` rules at `design.md:216`; `decl_schema_d` normal form in `SRC-022`; GP-2..GP-4 `R, F, N, C` row counts in `architecture.md:214-224`; ledger `9.1.0.1` `0.8.2` vs `0.8.3` byte parity)**: Not empirically run here.
4. **Other lanes' Round 2 outputs (`lanes/{A,C,D,E}/round2.md`)**: Never listed or read.
