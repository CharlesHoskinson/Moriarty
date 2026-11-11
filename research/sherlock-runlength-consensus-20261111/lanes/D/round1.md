# Round 1 Position — Lane D (Prover Throughput, Hardware Limits & Benchmarking Lead)

Study: `sherlock-runlength-consensus-20261111`
Repository state inspected read-only: `/home/hoskinson/Projects/Moriarty` at commit `c57351dd43cec8cb9f2db55d8350c0388c1f65ab` (`moriarty-authoring-beta-0.1.0-beta.1-1569-gc57351dd4`, 2026-11-10).

---

## Executive Summary

1. **Q1 (`2^64` vs. run length, exact `GP-1` / `GP-001` text):**
   - `2^64` in `SRC-004` (`openspec/changes/sherlock-vm-consensus-20261107/requirements.md:29`; `crates/mori-recursion/src/ivc.rs:24-26,79,363-376`) is the **integer range-check width of the step counter `h`** (`prev_h < 2^64`, `1 <= steps <= S_MAX`, `h = prev_h + steps < 2^64`), enforced in ~24–32 rows so `h` never wraps modulo the 255-bit BLS12-381 scalar prime `p` back to `0` (which would forge `is_genesis`). No circuit has `2^64` rows, and executing or proving `2^64` sequential steps is physically impossible (`ESTIMATE`: `2.41 x 10^12` years and `11.82 ZB` of bundle proofs at current `0.24 Hz` on this host; `146` years even unproved at `4.004 GHz` at 1 step/clock).
   - **"Unbounded" honestly means:** (i) *key-independence* — the fixed proving/verifying key set contains no circuit-size bound, loop-unrolling bound, or table-size cap; (ii) *wide non-wrapping arithmetic invariants* — per-run height and address counters are wide enough (`64` bits) that no physically realizable run ever hits an architectural ceiling; and (iii) *linear `O(N)` resource scaling + GP-2 continuations* — proving cost grows linearly with executed steps `N`, streaming prover RAM is `O(1)` in circuit work (`~17.4 GB` sequential, `~47.7 GB` 3-way concurrent), long jobs checkpoint/resume via `GP-2`, and `SAS-001` (`requirements.md:65`) records actual tested bounds separately from counter bit-widths.

2. **Q2 (Per-run memory/length ceiling):**
   - **Remove the `D3` `2^17`-leaf ceiling inside the leaf rather than relying on `D3 + continuations` (option a).** Under `D3` (`crates/mori-chip/src/m3.rs:50-55,132,209`), every leaf unconditionally consumes `C = 8` addresses (`lp = base + 8`) even on zero allocations, capping a run at `2^17 - 1 = 131,071` leaves (`131,071` steps at 1 step/leaf, `2,097,136` steps at `S = 16`) and making provability itself depend on prover leaf slicing.
   - **Crucial code finding:** `RangeConfig` (`crates/mori-chip/src/range.rs:30-35`) and `AluConfig` (`crates/mori-chip/src/alu.rs:23-26`) already check `128`-bit integers, and `WalkConfig` (`m3.rs:104-144`) reuses existing shared advice columns `a[2..11]`. Widening the Merkle address space from `D = 20` (`TOP = 17`) to `D = 64` (`TOP = 61`, `2^64` cells) adds **zero columns and zero lookups** to the leaf, **`+4` rows** to the `823,466`-row node (`research/sherlock-recursion-design-review-20261110/experiments/e2-e4.md:25-35`), and **`0` rows** to the `929,166`-row decider.
   - **Recommended design (Option d — Tail-Buffered Dense `M3` at `D = 64` + Carbon `D-1` Linked Slices):** Advance `len` strictly by `k_used` (`0 <= k_used <= 8`, zero phantom allocations on 0-append leaves), carry the `0..7` record digests `tail_d[0..6]` of the open 8-cell segment in state registers, and flush an aligned `C = 8` segment into the depth-`61` top tree **only when `floor(len_post / 8) > floor(len_pre / 8)`** (at most `1` flush per leaf since `7 + 8 = 15 < 16`). This eliminates the `2^17` ceiling, makes `len`, references, and `mem_root` **100% partition-independent (`SRC-023`)**, costs `ESTIMATE` `~13,230` M3 rows (`k_leaf = 15..16`, `< 5%` wall-clock impact at `S = 16`), and pairs with `D-1` cons-linked slices (`dissent.md:201`) to retire contiguous `addr + k` (`SI-12`).

3. **Q3 (Continuations, `input_d`, GP-2 aggregation, and C6):**
   - Even with per-run `D = 64`, **GP-2 continuations remain essential** for checkpoint/restart, bounded-fuel metering, and multi-host proving. Run `k` halts deterministically at `fuel == 0` into a canonical, address-free suspended outcome `Suspended(chk_d)` where `chk_d` is a `sherlock-value/1` tree digest over `(ctl, env, frames, tasks, logical_counters)` — maintained incrementally in `O(1)` registers via `SVM-012` cached value digests and `D7` frame-chain digests without scanning memory. Run `k+1` binds `chk_d` in `input_d_{k+1}`, lazily unfolds only touched records, authenticates run `k`'s sealed claim via `deps_d` (`GP-2`), and folds all `K` chunks through a telescopic `CONT_JOIN` rule in `GP-4` (`{U, R, F, N, C, D}`) into **one on-chain decider proof** of `(prog_d, input_d_0, outcome_d_final)`, leaking zero physical state digests (`C6` preserved).

4. **Q4 (Prior art across 11+ zkVMs):**
   - Production zkVMs split per-shard traces at `2^20..2^22` rows (`po2=20..22` in RISC Zero; `HEIGHT_THRESHOLD = 2^22` in SP1 Hypercube and Ziren 2.0; `<= 2^29` timestamps in OpenVM and Miden), bridge memory via Poseidon2 Merkle page trees (RISC Zero, OpenVM) or septic-curve / LogUp offline multisets (SP1, Ziren, Valida, Pico), and **universally strip intermediate `pc`, memory roots, shard counts, and cycle counts on-chain**. Both RISC Zero (`ExitCode::SessionLimit` disabled) and SP1 (`GAS_TRACE_CHUNK_THRESHOLD != MINIMAL_TRACE_CHUNK_THRESHOLD`) confirm that segment/shard boundaries cannot meter execution — validating Sherlock's `SVM-010` and `SRC-023`.

5. **Q5 (Lead analysis — Prover throughput, hardware limits, `S` scaling, and tested targets):**
   - On this `aarch64` host (`20` cores: `10x Cortex-X925` + `10x Cortex-A725`, `120.28 GiB` unified LPDDR5X at `244 GB/s` measured), `SHKIVC06` (`k = 20`) proves in `59.62–64.95 s` solo (`16.6–17.4 GB` RSS) and `RunDecider` (`k = 20`, `929,166` rows) proves in `70.35–77.88 s` (`qa/sherlock/runs.jsonl:617,648,649`).
   - At `S = 16`, `94%` of step time is node folding: sequential proving achieves **`4.01–4.38 s/step` (`0.229–0.250 Hz`)**. Single-host 3-way concurrency (`T1(b)`) suffers `f3 = 1.69–1.74x` memory-bus contention (`128.1–131.5 s`/node at `47.73 GB` aggregate RSS, `runs.jsonl:618`), giving only `1.39x` speedup (`0.353 Hz`).
   - **Primary throughput lever:** Because `E2` proved `k_leaf` up to `20` adds `+4` node rows/bit (`e2-e4.md:25-35`) and `E4` proved a `k = 20` leaf runs in `66.16 s` at `11.14 GB` RSS (`runs.jsonl:709`), raising `S` in the chip leaf from `16` (`k = 15`) to **`S = 256..512` (`k = 19..20`, with `B = S/16` segment flushes per leaf)** lifts single-host CPU throughput **`11x–21x` to `2.6–5.3 Hz` (`0.19–0.38 s/step` `ESTIMATE`)** without re-architecting the `k = 20` node.
   - **Recommended tested targets in spec:** Tier 1 Smoke `6` & `32` steps (`<= 20 min`, `<= 24 GiB` RSS); Tier 2 Qualification Gate `G-C` `293–320` steps (`<= 2 h`, `<= 48 GiB` process / `100 GiB` aggregate, `ESTIMATE` `~28 min` at `S = 16`); Tier 3 Soak & Continuation Acceptance **`4,096` steps (`2^12`)** (`256` leaves at `S = 16`, including a `2,048 + 2,048` GP-2 continuation split, `<= 6 h` wall at `S = 16` or `< 40 min` at `S >= 128`, `<= 24 GiB` single-process RSS).

---

## Q1. GP-1 & GP-001 Run-Length Semantics, What `2^64` Actually Is, and Exact Replacement Text

### 1.1 What `2^64` Actually Is (and Why There Is No `2^64` Circuit)

The current wording in `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:14` (*"The run length is bounded only by the height field (below 2^64)"*) and `requirements.md:56` (`GP-001`: *"for every run length below 2^64"*) conflates an **in-circuit counter overflow guard** with an **executable/provable run length**:

1. **No circuit has `2^64` rows.** Every circuit in the Sherlock key set has a fixed small power-of-two evaluation domain: the chip leaf uses `k = 13..16` (`8,192..65,536` rows, `design.md:129,206`, up to `K_LEAF_MAX = 20` = `1,048,576` rows in `requirements.md:39`), the self-recursive node `SHKIVC06` uses `k = 20` (`823,466` model rows over `1,048,576` domain slots, `design.md:130`), and the `RunDecider` uses `k = 20` (`929,166` model rows, `design.md:131`). A single degree-`2^64` KZG polynomial over 32-byte BLS12-381 scalars would occupy `2^64 x 32 B = 590.3 Exabytes` per column (`ESTIMATE`) and is mathematically and physically nonsensical.
2. **`2^64` is a ~24–32-row integer range check in `ivc.rs` (`SRC-004`).** In `crates/mori-recursion/src/ivc.rs:79,363-376`, `constrain_heights` calls `std.assert_lower_than_fixed` (`H_BITS = 64`) on `prev_h`, `steps - 1 < S_MAX`, and `h = prev_h + steps`. Without that bound, addition takes place modulo the 255-bit BLS12-381 scalar field prime `p` (`~2^254.86`); a malicious prover could pick `prev_h = p - steps` so `h = 0 (mod p)`, forging `is_genesis = is_zero(prev_h)` mid-chain (`ivc.rs:24-26`, `requirements.md:29`). All `SRC-004` and `SRC-005` checks together added only `73` rows to `SHKIVC06` (`652,364 -> 652,437` rows; `qa/sherlock/profiles/c0-gp-r4-t20/GP-resident_pk-t20-0-off.json:70-80`, `qa/sherlock/decisions/speed-levers.json:31`).
3. **`2^64` sequential steps cannot be physically executed or proved.** As quantified in §Q5.4 below, `2^64` sequential steps takes `2.41 x 10^12` years (`~175x` the age of the universe) at this host's `0.24 Hz` proving rate (`ESTIMATE`), and `146.0` years even unproved on a `4.004 GHz` Cortex-X925 core retiring 1 VM step per clock cycle (`ESTIMATE`).

### 1.2 What "Unbounded" Can Honestly Mean Here

In a recursive zkVM over fixed-size circuits, **"unbounded run length"** has a precise three-part engineering meaning that neither promises physically impossible runs nor permits artificial circuit ceilings like `D3`'s `2^17` leaves:

1. **Key-set universality (zero circuit-baked capacity caps):** One fixed key set proves computations of arbitrary finite step count `N >= 1` and arbitrary working-set size without re-keying, new ceremonies, or capacity ladders (`general-purpose.md:9,13`). Neither the leaf key, node key, decider key, nor verifier contract fixes a loop bound, trace length, or carrier array size.
2. **Inaccessible arithmetic counter ceilings + GP-2 continuation closure:** Within a single run, step counters (`h < 2^64`, `SRC-004`) and committed-memory addresses (`D = 64` bits in §Q2.2) are wide enough that physical time and energy exhaust orders of magnitude before any counter saturates. Across runs, computations of any duration checkpoint and resume through authenticated `GP-2` claims (`§Q3`) and aggregate via `GP-4` (`general-purpose.md:20-50`).
3. **Honest `O(N)` operational scaling (`SAS-001`):** Proving work and GP bundle size (`SRC-019`) grow linearly `O(N)` with executed steps `N`, while streaming circuit-prover RAM stays `O(1)` (`~17.4 GB` for `k = 20`). Practical run length on any machine is therefore governed by wall-clock budget, energy, and host storage for live memory/bundles; `SAS-001` (`requirements.md:65`) records empirically **tested step bounds** separately from 64-bit soundness counter widths.

### 1.3 Exact Drop-In Replacement Text

#### A. Replacement in `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md` (GP-1)

Replace the intro sentence (`:9`), run-length bullet (`:14`), and acceptance paragraph (`:16`) — keeping bullets 1–3 (`:11-13`) and GP-2..GP-4 (`:20-58`) unchanged in substance:

> **GP-1 Universal VM (`general-purpose.md:9` replacement):**
> One fixed key set proves the execution of every program of the Sherlock language, on every input, for any finite run length and every outcome. A new program or a longer run needs no new key, no new contract and no new ceremony.

> **Run-length bullet (`general-purpose.md:14` replacement):**
> - The key set imposes no architectural step or memory ceiling on practical runs. Each run folds fixed-size leaf proofs (`S` steps per leaf) with `O(1)` circuit size and `O(N)` proving work in the executed step count `N`; per-run step counters (`h < 2^64`, `SRC-004`) and committed-memory addresses (`< 2^64`) are bounded only by 64-bit non-wraparound invariants that exceed any physically realizable execution, and multi-run computations extend and checkpoint across runs via `GP-2` continuation claims aggregated under `GP-4`.

> **Acceptance line (`general-purpose.md:16` replacement):**
> Acceptance: one key set and one deployed verifier contract accept proofs of at least three programs that the key set was not built from, including one program whose state exceeds one leaf's carriers and spans multiple committed-memory segments, one multi-leaf run meeting the qualification step target (`>= 293` steps under `SRC-011`) plus a multi-segment soak/continuation run (`>= 4,096` steps), one run with input, and one run of each outcome kind; counter non-wraparound at `2^64 - 1` is verified by boundary negative tests (`SRC-004`) and formal proof (`SRC-017`).

#### B. Replacement in `openspec/changes/sherlock-vm-consensus-20261107/requirements.md` (`GP-001`, `:56`)

| ID | Form | Requirement | Acceptance check | Refines / adopts |
|---|---|---|---|---|
| GP-001 | U | GP-1 universal VM. One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for any finite number of executed steps within the 64-bit non-wraparound counter domain (`1 <= n_steps < 2^64`, `SRC-004`) and across chained continuation runs (`GP-002`, `GP-004`), and for every outcome (`Halted(v)` for every value type, as a digest of a canonical value encoding; `Rejected(code)`; `Exhausted(kind)`); the public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants; neither step capacity nor memory capacity SHALL depend on the key or impose a sub-64-bit per-leaf segment ceiling (state larger than one leaf's carriers runs against committed memory without consuming addresses on non-allocating leaves); a new program or longer run SHALL need no new key, contract or ceremony, while proving work scales linearly with executed steps over fixed-size circuits (`SAS-001` records tested step bounds). When a supported run exceeds one leaf's storage or hashing capacity, the prover SHALL continue through authenticated memory, recursive folding, and/or authenticated `GP-002` continuations under the same key set. | One key set and one deployed verifier contract accept proofs of at least three programs the key set was not built from, including one program larger than one leaf's carriers, one run with input, one run of each outcome kind, qualification runs at `293–320` steps (`SRC-011`), and a multi-segment/continuation soak run at `>= 4,096` steps, while `SRC-004` raw-witness boundary tests and `SRC-017` Lean/Quint proofs verify non-wraparound up to `2^64 - 1` (row A33). | `general-purpose.md` GP-1; SRC-004; SRC-017; SAS-001; C6 review 2026-11-10 |

---

## Q2. Per-Run Memory and Length Ceiling: Quantitative Comparison of Options (a)–(d)

### 2.1 Grounded Cost-Model Facts from `crates/mori-chip` and `E2–E8`

Every row and proving-time `ESTIMATE` below uses measured circuit constants from this repository:
- **Poseidon sponge (`crates/mori-chip/src/poseidon.rs:52-61,113-115`):** Width `3`, rate `2`, `68` rounds + `1` absorb + `1` output = **`70` rows per permutation** (`rows(w) = 70 * ceil(w / 2)`).
- **Current `E8` chip leaf (`design.md:125-134`, `research/sherlock-processor-analogy-20261110/experiments-4.md:89-128`):** `S = 16`, `C = 8` (`c = 3`), `D = 20` (`TOP = 17`): `12` advice cols, `13` perm cols, `20` fixed cols (`17` filled), `4` lookups, max degree `6`; **`6,511` active rows** (`6,090` HASH rows = `87` perms + `421` Core/ALU/MEM rows), `k_leaf = 13` (`8,192` domain, fixed `RANGE` table `4,096` rows in `range.rs:34`).
- **Node & Decider sensitivity to leaf changes (`design.md:130-134`, `e2-e4.md:25-35`):**
  - Over `E8` (`k_leaf = 13`), `ChipNode` / `SHKIVC06` = **`823,466` rows (`k = 20`)** (`126,534` rows below `950,000` gate); `RunDecider` = **`929,166` rows (`k = 20`)**.
  - Adding leaf **rows** without adding columns/lookups adds **`0` node rows** when `k_leaf` is unchanged, **`+4` node rows per `+1` increase in `k_leaf`** (`e2-e4.md:25-35`, one extra squaring of `z^n` per domain bit), and **`0` decider rows**.
  - Adding leaf **columns/lookups** costs `+7,298` node rows/advice col, `+8,189`/equality col, `+6,004` to `+6,469`/filled fixed col, and `+22,547` to `+22,672`/lookup (`design.md:134`, `experiments-2.md:134`, `experiments-3.md:166`).
- **Critical `mori-chip` width fact:** `RangeConfig` (`range.rs:30-35`, `11 x 12`-bit limbs) and `AluConfig` (`alu.rs:23-26`, `2^128` bound) **already check 128-bit values**, and `WalkConfig` (`m3.rs:104-144`) accumulates `acc_{l+1} = acc_l + 2^l * b_l` in the 255-bit field `Fq` using existing advice columns `a[2..11]` and one packed 3-code fixed column (`m3.rs:117-135`). Widening tree depth `D` to `32`, `64`, or `67` adds **zero advice, equality, fixed, or lookup columns**.

### 2.2 Detailed Analysis of Options (a), (b), (c), and (d)

#### Option (a): Keep `D3` (`D = 20`, `C = 8`, `< 2^17` leaves = `131,071` leaves, `< 2^20` cells) + `GP-2` Continuations
- **Mechanism:** Retain `m3.rs:50-55` (`TOP = 17`, genesis reserves segment `0` so `len_0 = 8`, `rf.rs:94-95`). Every leaf unconditionally sets `len_post = base + 8` (`m3.rs:132,209`). Runs exceeding `131,071` leaves split into chained runs via `GP-2` (`§Q3`).
- **Row & prover cost (`ESTIMATE`):** `+0` leaf rows over `E8` (`6,090` M3 rows; `1,540` rows per cold `PAGE` read; full-VM leaf `~16k–25k` rows at `k_leaf = 15`, `~4.0 s` prove; `823,474` node rows; `929,166` decider rows). However, at every continuation boundary (`<= 2.09M` steps), run `k+1` must re-verify/unfold touched state from `chk_d` and fold a `GP-2` `SET` proof (`+2..3` extra `k = 20` proofs = `+130–200 s` per boundary).
- **Fatal defects:**
  1. **Violates `GP-1` / `GP-001` even on zero-memory programs (`dissent.md:228-232`):** A pure register loop (`0` heap allocations) still burns 1 segment per leaf and becomes unprovable in one run after `2,097,136` steps (`S = 16`) — or after just **`131,071` steps** if the prover closes leaves at 1 step/leaf.
  2. **Breaks provability partition-independence (`SRC-023` spirit):** Under `SRC-023` (`requirements.md:48`), two prover partitions of a `500,000`-step run (`S = 16` full leaves = `31,250` leaves vs. `1` step/leaf = `500,000` leaves) have Asymmetric Provability: one succeeds, the other hits an unprovable address exhaustion that `SVM-010` forbids encoding as `Exhausted`.
  3. **Blocks scaling `S` (`§Q5.5`) and breaks `SI-12` (`dissent.md:200`):** `C = 8` caps a leaf at `<= 8` allocations, forcing early leaf closes if `S > 32`, and any contiguous field slice `> 8` cells cannot exist under `C = 8`.

#### Option (b): Widen Segment Index Only (`D = 32` / `TOP = 29`, or `D = 64` / `TOP = 61`, or `D = 67` / `TOP = 64`, keeping unconditional `C = 8` per leaf)
- **Mechanism:** Change `pub const TOP: usize` in `m3.rs:50` from `17` to `29` (`32`-bit cell addresses, `2^29 - 1 = 5.37 x 10^8` leaves), `61` (`64`-bit cell addresses, `2^61 - 1` leaves), or `64` (`67`-bit cell addresses, `2^64 - 1` leaves so even 1-step leaves never exhaust before `h = 2^64`).
- **Exact row & prover cost (`ESTIMATE` via `Rows_M3(C, D) = 70 * (3C + 29 + 2*(D - c))` and `Rows_PAGE(D) = 70 * (D + 2)`):**
  - **`D = 32` (`TOP = 29`, `C = 8`):**
    - M3 HASH rows: `70 * (24 + 29 + 58) = 70 * 111 = 7,770` rows (`+1,680` rows over `D = 20`).
    - Cold `PAGE` read: `70 * 34 = 2,380` rows (`+840` rows/read; `+3,360` rows for 4 reads).
    - Full-VM `S = 16` leaf: `~21k–30k` rows -> stays **`k_leaf = 15`** (`32,768`), **`+0` node rows over `k=15` (`823,474`)**, **`+0` decider rows**, `+0.5–1.0 s` leaf prove (`< 1.5%` wall impact at `66 s`/leaf). Note: `2^29` leaves caps 1-step/leaf runs at `5.37 x 10^8` steps (`< 2^64`).
  - **`D = 64` (`TOP = 61`, `C = 8`) / `D = 67` (`TOP = 64`, `C = 8`):**
    - M3 HASH rows: `70 * 175 = 12,250` rows at `D = 64` (`+6,160` rows); `70 * 181 = 12,670` rows at `D = 67` (`+6,580` rows).
    - Cold `PAGE` read: `4,620` rows (`D = 64`) / `4,830` rows (`D = 67`) (`+3,080..3,290` rows/read; `+12,320..13,160` rows for 4 reads).
    - Full-VM `S = 16` leaf: `~34k–44k` rows -> **`k_leaf = 16`** (`65,536`), adding **`+4` node rows (`823,478` rows, `k = 20`)**, **`+0` decider rows (`929,166`)**, leaf prove `ESTIMATE` `~5.5–7.5 s` (`+2.5 s` on a `66 s` cycle = **`+3.8%` total step time**).
- **Soundness & remaining defects:** Very low soundness risk (`WalkConfig` already proves bit binary-ness and field no-wrap holds since `2^67 << p_BLS`). However, **0-append leaves still burn `8` phantom cells and hash two 61-level spine walks**, `len` and `mem_root` remain **partition-dependent** (forcing complete reliance on C6 address hiding), `SI-12` (`> 8` contiguous cells) stays broken, and `C = 8` still throttles higher `S`.

#### Option (c): Non-Fixed-Segment / Dynamic Memory Designs
1. **(c1) Conditional M3 Segment Allocation (`alloc_en = act_0` gate):** Gate the M3 top-tree walk so leaves with `0` appends keep `len_post = len_pre` and `mem_root_post = mem_root_pre`. Costs `~20` gate rows, `+0` columns. Fixes 0-append loop exhaustion, **but** any leaf with `1..7` appends still pads to `8`, so `len` and `mem_root` remain partition-dependent and `SI-12` stays broken.
2. **(c2) Unaligned Per-Cell Frontier (`M2` dense `len_post = len_pre + k_used`):** Update up to `K_ALLOC = 8` arbitrary consecutive leaves `len_pre .. len_pre + k_used - 1` directly in a depth-`64` binary tree. Makes `len` and `mem_root` **strictly partition-independent**, **but** without aligned segment buffering, an unaligned batch of `8` cells crossing a power-of-two boundary splits into two variable-shared spine walks with complex per-level routing gates (`ESTIMATE` `+18,000–24,000` HASH+mux rows and higher soundness surface in muxed tree paths).
3. **(c3) Growing-Depth Merkle Tree (`depth` in register, `4 -> 64`):** Lift `mem_root' = H(NODE, mem_root, ZERO_ROOT[depth])` only when `len == 2^depth`. While this cuts *host witness* hashing on small runs (`depth = 8` for `fact(3)`), **in Halo2 Plonkish circuits region heights are fixed at keygen**: the circuit must still lay out all `D_max = 64` rows per walk and gate levels `l >= depth` to identity (`+1` control code/column + subtle off-by-one soundness risks at depth transitions), saving **zero Halo2 domain rows**!
4. **(c4) Height-Keyed Address Space (`addr = (h_step << 4) | sub_idx`, `68`-bit sparse key):** Key every allocation by logical step height `h` (`< 2^64`) and per-step ordinal (`< 16`). Eliminates `len` and makes addresses partition-independent (`h` is partition-independent), **but** sparse keys separated by non-allocating steps cannot share `C = 8` aligned leaf subtrees (forcing up to `8` separate depth-`68` walks = `8 x 2 x 68 x 70 = 76,160` rows/leaf `ESTIMATE` unless `K_ALLOC = 1`), and breaks `SI-12` (`addr + k` no longer exists).

#### Option (d) [RECOMMENDED]: Tail-Buffered Dense Aligned `M3` (`D = 64`, `TOP = 61`, `C = 8` per flush) + Carbon `D-1` Linked Slices
- **Core insight (combining the best of `(b)` and `(c2)` while keeping `m3.rs`'s exact aligned geometry):**
  1. **Dense Logical Address Counter (`len < 2^64`):** `len` counts **exact logical cells allocated** (`len_post = len_pre + k_used`, `0 <= k_used <= 8` at `S = 16`, scaling to `8 * B` at higher `S`). Zero-append leaves advance `len` by `0`. Let `q = floor(len / 8)` (`< 2^61`) be the count of *completed* 8-cell segments and `r = len mod 8` (`r in 0..7`) be the fill count of the active tail segment.
  2. **Register-Cached Open Tail Segment (`tail_d[0..6]`):** Mirroring `D7` top-of-frame register caching (`d7-frames.md:45-92`), the machine state carries `7` field registers `tail_d[0..6]` holding the record digests `H(1, a, b, c)` of the `r` cells (`0 <= r <= 7`) in currently open segment `q` (zero-padded for slots `>= r`), while `mem_root` commits to completed segments `0 .. q - 1` in the depth-`61` top tree.
  3. **At Most One Aligned Top-Tree Flush per `8` Appends:** Because `0 <= r <= 7` and `0 <= k_used <= 8`, we have `0 <= r + k_used <= 15 < 16`, so **`floor((r + k_used) / 8)` is strictly `0` or `1`**!
     - **Case `r + k_used < 8` (`flush = 0`):** `mem_root_post = mem_root_pre` (the top-tree walk inputs dummy neutral path bits and its output is gated off); the `k_used` new record digests shift into `tail_d[r .. r + k_used - 1]`.
     - **Case `8 <= r + k_used <= 15` (`flush = 1`):** `tail_d[0..r-1]` concatenated with the first `8 - r` new record digests compress via the exact 7-perm `SegTree` of `m3.rs` into `seg_root`, which is written at aligned segment index `q < 2^61` (`old = 0 -> new = seg_root`) in the depth-`61` `WalkConfig`! The remaining `r + k_used - 8 <= 7` new record digests become `tail_d_post[0 .. r + k_used - 9]`.
  4. **Fast Hot-Tail Reads + `D = 64` Cold Reads:** Any read of `addr >= 8*q` (`r` most recent heap cells) verifies `H(1, a, b, c) == tail_d[addr - 8*q]` in `2` perms (`140` rows, **zero Merkle walk**); cold reads of `addr < 8*q` verify a depth-`64` path (`61` top + `3` seg = `4,620` rows).
  5. **Slices (`SI-12..SI-14`, `dissent.md:198-202` `D-1`):** Adopt Carbon's `D-1` cons-linked slice cells `(TAG_SLICE, elem_ref, prev_link)` (with inline refs for arity `<= 2`). No rule ever computes `addr + k`; `SI-13` (child-before-parent) and `SI-14` (acyclic strict ordering) hold directly on dense `0 < addr_child < addr_parent < len`. *(Bonus: because `len` is now dense without leaf-boundary gaps, even contiguous `addr + k` slices would stay contiguous across leaf closes!)*

### 2.3 Quantitative Comparison Matrix (`ESTIMATE`s vs. Measured `E8`)

| Criterion | (a) Keep `D3` (`D=20`) + `GP-2` | (b) Widen `D3` Only (`D=64..67`) | (c2/c3/c4) Unaligned / Dyn-Depth / Height-Keyed | (d) **Tail-Buffered Dense `M3` (`D=64`) + `D-1`** *(Recommended)* |
|---|---|---|---|---|
| **Max steps / cells per run** | `2,097,136` steps (`131,071` min); `< 2^20` cells | `2^64 - 1` steps (`D=67`); `2^64` cells | `2^64 - 1` steps; `2^64` cells | **`2^64 - 1` steps** (every partition); **`2^64` cells** |
| **0-append leaf address burn** | `8` cells/leaf (fatal on loops) | `8` cells/leaf (wasteful) | `0` cells/leaf | **`0` cells/leaf** |
| **Leaf M3 + state-digest HASH rows (`ESTIMATE`)** | `6,090` (`87` perms, measured) | `12,250–12,670` (`175–181` perms) | `~18,000–76,000` (unaligned/sparse) | **`13,370`** (`16` rec + `7` seg + `122` walk + `38` `pre/post` w/ `7` tail words + `~560` shift mux) |
| **Cold `PAGE` read rows (`ESTIMATE`)** | `1,540` (`22` perms) | `4,620–4,830` (`66–69` perms) | `4,620–4,900` | **`140` rows** (hot tail) / **`4,620` rows** (cold `< 8q`) |
| **Full-VM `S=16` `k_leaf` & leaf time (`ESTIMATE`)** | `k = 15` (`~22k` rows, `~4.0 s`) | `k = 16` (`~38k` rows, `~6.5 s`) | `k = 16..18` (`~45k–110k` rows, `~8–18 s`) | **`k = 16` (`~36k` rows, `~6.0 s`; `+3%` total step wall)** |
| **Added leaf cols / lookups** | `+0` | `+0` | `+1..3` cols / mux codes | **`+0` advice/perm/lookup cols** (`1` packed mux code) |
| **Node rows (`SHKIVC06`, `k=20`)** | `823,474` (`ESTIMATE` at `k=15`) | `823,478` (`+4` rows at `k=16`) | `823,478–845,000` | **`823,478` (`+12` rows vs `E8` `k=13`!)** |
| **Decider rows (`k=20`)** | `929,166` (unchanged) | `929,166` (unchanged) | `929,166` (unchanged) | **`929,166` (unchanged)** |
| **Soundness risk** | Low locally, high at continuation seam | Very low (existing `WalkConfig`) | Medium–High (split-path / depth-lift gates) | **Low** (aligned top-tree walk unchanged + 8-slot barrel shift) |
| **Effect on `SI-12..SI-14` (`D-1`)** | Breaks `SI-12` (`> 8` slice) | Breaks `SI-12` (`> 8` slice) | Height-keyed breaks `SI-12` | **Solves cleanly** (`D-1` linked slices + dense `len` preserves `SI-13/14`) |
| **Effect on `SRC-023` (partition indep.)** | Physical state varies; provability varies! | Physical state varies (`C6` hides it) | Physical state canonical (`c2`) | **`len`, refs, `tail_d`, and `mem_root` are 100% partition-independent!** |
| **Effect on `GP-2..GP-4` & `S > 16`** | Forces continuations at `2M` steps; caps `S` | Frees `GP-2` for true multi-run; caps `S` at `C=8` | Supports `GP-2..4` | **Cleanest `GP-2..4` base; `B` flushes/leaf scales cleanly to `S = 256..512`** |

---

## Q3. Continuations: Canonical State Hand-Off, `input_d`, GP-2 On-Chain Aggregation, and C6 Interaction

Even after Option (d) removes the per-run address ceiling, **continuations across chained runs remain mandatory** for: (1) bounding per-job wall-clock risk (`< 2–6 h` checkpoints on multi-day computations), (2) reactive/interactive multi-transaction workflows, and (3) pipelining/distributing segments when host storage or operational windows are bounded.

### 3.1 Why Raw State Commitments Cannot Cross Run Boundaries

Three adopted rules forbid passing `post_d` or `mem_root` from run `k` to run `k+1`:
1. **`SRC-006` (`requirements.md:31`):** The decider SHALL NOT expose `fin_d`, `init_d`, `pre_d`, `post_d`, `mem_root`, `lanes_d`, `len`, a reference, or any address-dependent value.
2. **`GP-002` (`requirements.md:57`):** A consumed claim SHALL be a sealed claim `(prog_d', input_d', outcome_d')` opened from a terminal state; segment statements and state digests SHALL NOT be accepted.
3. **`SVM-010` & `SRC-023` (`requirements.md:18,48`):** A continuation split must be triggered by a **logical** condition (`fuel == 0` on the deterministic fuel budget committed in `input_d_k`), never by a prover-chosen leaf boundary or physical memory full flag, so `outcome_d_k` is strictly partition-independent.

### 3.2 Canonical, Partition-Independent Full-State Digest (`chk_d`)

Today, `SRC-022` (`requirements.md:47`, `design.md:191`) encodes `Exhausted(Fuel)` as `H(O, 1, 2, 0)`, discarding the machine state. To pass **full machine state** (control, environment, call stack, helper stack, and live heap values — not merely a final value) across runs without leaking addresses or scanning memory:

1. **Extend `sherlock-value/1` with an Address-Free Logical Checkpoint Digest `chk_d`:**
   Define terminal outcome tag `3` (or refine `Exhausted(Fuel)` under continuation mode) as `Suspended(chk_d)` with `outcome_d = H(O, 1, 3, chk_d)`, where:
   $$\text{chk\_d} = H\bigl(\text{dom}(\text{"SHK\_CHK\_V1"}),\; \text{ctl\_tag},\; \text{ctl\_id},\; \text{env\_vd},\; \text{frames\_vd},\; \text{tasks\_vd},\; \text{counters\_d}\bigr)$$
   - `ctl_tag, ctl_id`: canonical `PROG` expression/function indices (already address-free).
   - `env_vd`: canonical cons-chain digest of live lexical bindings `(slot_id, vd(val))`, where each value's `vd(val)` is **already computed and bound at value creation by `SVM-012` (`requirements.md:20`)** and erases addresses, padding, and DAG sharing (`SRC-022`, `SRC-023`).
   - `frames_vd` & `tasks_vd`: rolling Poseidon chain digests maintained incrementally on every `PushFrame`/`PopFrame` (`D7` in `d7-frames.md:45-92` already structures frames as an explicit linked chain!) over canonical `(frame_kind, ret_expr, env_vd, live_slots_vd, parent_frames_vd)`.
   - `counters_d`: canonical digest of logical work counters (`stack_depth`, `helper_work`, `reference_work` — never `len`, `fp`, `tp`, or `n_steps`).
2. **Zero Heap Scan at Halt AND Zero Heap Scan at Resume (`SRC-021` kill criterion respected):**
   - **At suspension (`fuel == 0` in run `k`):** Because `vd` is maintained inductively on values (`SVM-012`), environments, and frame pushes, all six inputs to `chk_d` sit in registers! Computing `chk_d` at `fuel == 0` costs **4 Poseidon permutations (`280` leaf rows `ESTIMATE`)**, independent of heap size! Unreachable garbage cells in run `k`'s arena are automatically dropped (free compacting GC at every continuation boundary!).
   - **At resume (`h_pre == 0` in run `k+1`):** Run `k+1` does **not** eagerly load or scan the heap. Instead, `Init_Resume` sets registers `(ctl, env_vd, frames_vd, tasks_vd, counters)` from the witnessed opening of `chk_d` (`280` rows `ESTIMATE`), starts with an **empty** physical `DATA` arena (`len = 8`), and **lazily unfolds** (`UnfoldValue` / `PopFrameUnfold` microsteps, the exact lazy-unfold mechanism noted in `SRC-021`, `requirements.md:46`) only the frames and subvalues actually dereferenced during run `k+1`! Every unfolded cell verifies its `sherlock-value/1` preimage against the authenticated parent `vd` in-circuit, establishing `StepInv` (`SVM-002`) by construction without trusting host memory images.

### 3.3 Exact `input_d` Structure for Run `k+1`

To bind run `k+1` to the same program, original run input, and exact predecessor state without replay ambiguity:
- **Initial chunk (`k = 0`):** Standard `SRC-021` `input_d_0 = Poseidon(SHK_IN, val_d_0, fuel_lo_0, fuel_hi_0, stack_limit)`.
- **Continuation chunk (`k >= 1`):**
  $$\text{input\_d}_k = \text{Poseidon}\bigl(\text{SHK\_IN\_CONT},\; \text{input\_d}_0,\; \text{chk\_d}_{k-1},\; \text{fuel\_lo}_k,\; \text{fuel\_hi}_k,\; \text{stack\_limit}\bigr)$$
  At `h_pre == 0`, when the entry mode is `Resume`, the first leaf of run `k`:
  1. Verifies the `input_d_k` opening and initializes registers to `chk_d_{k-1}` with `fuel = fuel_k` and `stack_limit` matching `input_d_0`.
  2. Records the required predecessor claim `c_{k-1} = (prog_d, input_d_{k-1}, H(O, 1, 3, chk_d_{k-1}))` into `deps_d` (`architecture.md:84,96,133`), so **run `k` cannot seal unless a verified proof of run `k-1` is supplied in-circuit (`GP-002`)**! This prevents host forgery of arbitrary mid-execution states that violate `StepInv` (`SVM-002`).

### 3.4 Proving the Whole Chain Once On-Chain (`GP-2` + `GP-4` Aggregation) & C6 Interaction

Under the binary PCD architecture `{U, R, F, N, C, D}` (`architecture.md:67-105`), a `K`-chunk continuation (`R_0, R_1, ..., R_{K-1}`) settles in **one on-chain proof** with zero intermediate leakage:
1. Each chunk `R_i` (`0 <= i < K`) proves its leaves independently in parallel (once witness snapshots `S_0..S_{K-1}` are produced by `mori simulate`!), folds its `SEG` tree via `R` and `N`, and seals (`SEG -> SET` for `R_0`; `SEG + SET -> SET` for `R_i` verifying `deps_d`).
2. **Telescopic Continuation Reduction (`CONT_JOIN` in `N`):** Add one statement-level transition rule to `N` (`SET + SET -> SET`): given verified adjacent continuation claims `(prog_d, in_a, H(O, 1, 3, chk))` and `(prog_d, Resume(in_0, chk, fuel_b), out_b)` with matching `stack_limit` and `fuel_a + fuel_b < 2^128`, `N` emits the single telescoped claim `(prog_d, WithFuel(in_0, fuel_a + fuel_b), out_b)`! Across `K` chunks in a balanced binary tree of depth `ceil(log2 K)`, this collapses all `K` chunks to the **exact single C6 claim `(prog_d, input_d_total, outcome_d_final)`** (`n_steps = sum_{i=0}^{K-1} n_steps_i < 2^64`).
3. **C6 Interaction:** Neither `pre_d`, `post_d`, `mem_root`, nor even the intermediate logical `chk_d_1 .. chk_d_{K-1}` or chunk count `K` appear on-chain! The on-chain `acceptRun` / `acceptRoot` call sees the **identical** `[prog_d, input_d_total, outcome_d_final, n_steps]` statement and `claim_d` replay key (`SRC-006`, `SRC-007`, `SRC-023`) whether the prover ran 1 unsplit run of `F` fuel or `K` continuation chunks summing to `F` fuel!

---

## Q4. Prior Art: Run-Length Ceilings, Memory Persistence, On-Chain Statements, and Prover Hz Across Production zkVMs

### 4.1 Cross-System Architecture & Statement Comparison

| System (ISA / Field) | Per-Segment / Shard Limit | Total Run Ceiling | Address Space & Memory Cap | Cross-Segment Memory Mechanism | On-Chain Public Statement (Memory/Length Exposed?) |
|---|---|---|---|---|---|
| **RISC Zero** R0VM 2.0/3.x (RV32IM, BabyBear) | `po2 = 20` default (`1.05M` cyc); `po2 = 21` (24 GB GPU); `po2 = 22` (`4.19M` cyc w/ `low_vram`) | Unbounded via STARK recursion (`u64` kernel cycle counter; `u32` PoVW segment index) | 32-bit (`4 GiB`; `~3 GiB` user `0x0001_0000..0xc000_0000`) | **Poseidon2 binary Merkle page table** (`DEPTH = 22`, `1 KiB` pages; `~1,130` cyc/page fault) | `verify(seal, imageId, journalDigest)` zeros `postStateDigest` on `Halted(0)` — **hides `mem_root`, segments, and cycles** (except optional PoVW `WorkClaim`) |
| **SP1 Hypercube** v6 (RV64IM, KoalaBear + Jagged PCS) | `HEIGHT_THRESHOLD = 2^22` (`4.19M` rows), `MAX_SHARD_SIZE = 2^24`, area `3 x 2^27` cells | Unbounded via 3–4-ary STARK recursion; multi-limb timestamps | **48-bit** (`2^48 - 1` = `256 TiB` logical; `1 TiB` JIT; `24 GiB` host RSS default) | **Offline elliptic-curve multiset sum** (`SepticDigest` via Poseidon2 + `SepticCurve::lift_x` + `MemoryInit/Finalize` shards) | 2 BN254 scalars: `(programVKey, sha256(publicValues) & ((1<<253)-1))` — **hides shards, timestamps, addresses, septic sums** |
| **Jolt** Twist & Shout (RV64IM, BN254 Dory / Akita) | Monolithic (no shards): `DEFAULT_MAX_TRACE_LENGTH = 2^24` (`16.7M` cyc), configurable `T` | Monolithic `T`; **streaming sumcheck roadmap** proves arbitrary `T` in `< 2 GB` RAM without recursion | 64-bit (`XLEN = 64`), `DEFAULT_HEAP_SIZE = 32 MiB` (scales sparsely w/ touched `K`) | **Twist & Shout:** one-hot address vectors + write increments via multilinear sumcheck (zero grand products) | Bytecode + layout commitment, `inputs`, `outputs`, `panic` bit, padded `T` (raw sumcheck reveals `ceil(log2 T)` unless SNARK-wrapped) |
| **Nexus** 1.0/2.0 (Nova/HyperNova) -> **3.0** (RV32IM, Stwo M31) | v1/v2: small IVC step batches; v3.0: M31 circle domain (`<= 2^30` rows), 32-bit timestamps | Unbounded via IVC (v1/v2) or Stwo recursive composition (v3.0) | 32-bit, `MEMORY_TOP = 0x80400000` (`~2.0 GiB`) | v1/v2: incremental Merkle/Spice; v3.0: **LogUp offline memory bus** (`RamInitFinal` + 32-bit `prev_ts < cur_ts`) | ELF commitment, public input/output slices, exit code — **intermediate accumulators/chunks stripped** |
| **Valida** (Custom 32-bit stack/RAM ISA, BabyBear) | `max_trace_height: u32` (`<= 2^27` BabyBear two-adicity, typ. `2^20..2^22`) | Parallel multi-segment continuations (`PriorSegment(T)`) | 32-bit byte RAM (`u32` addr; GPR-less `FP`-relative stack in RAM) | Intra-segment LogUp + cross-segment **multiplicative grand product** (`PersistentMemBus` + `DummyRead`) | `(Program ROM, StaticData, Output tape bytes)` — **segment `PC`, `FP`, and `PersistentMemBus` hidden** |
| **Cairo / Stwo** (`felt252` / M31 Circle STARK) | Circle subgroup `<= 2^30` rows (`MEMORY_ADDRESS_TO_ID_SPLIT = 16`) | Unbounded via **Cairo Bootloader** + `stwo_cairo_verifier` SHARP trees | Contiguous **write-once** relocated cells (`< 2^31 - 1` in Stwo M31) | Intra-run post-execution **segment relocation** (`a_{i+1} - a_i in {0,1}`); inter-run **Poseidon/Pedersen state roots** in Bootloader | `(program_hash, output / output_hash)` (on Starknet L1: bootloader hash + L2 state-diff hash; **Cairo step count hidden**) |
| **zkWasm** (WASM32, Halo2 KZG BN254) | `-k 18..22` (`2^18..2^22` rows; `4` rows/event -> `2^16..2^20` steps/slice) | Unbounded slices (`u32` `eid_cell`) via `continuation-batcher` | WASM32 64 KiB pages (`4 GiB` arch max; tens/hundreds of MiB practical) | **KZG witness column commitments** (`slice[i].in_ctx_comm == slice[i-1].out_ctx_comm`) | Image commitment + KZG/hash of public inputs & final context outputs (**slice count & mid-slice commitments hidden** in rollup mode) |
| **Miden VM** v0.35/0.36 (Goldilocks Plonky3) | Monolithic: `MAX_CYCLES = 2^29` (`536.87M` cyc) | `2^29` cyc per proof; extended via explicit in-VM STARK verifier (`sys::vm`) | Per-context `addr < 2^32` (`d0, d1` 16-bit delta cols in 15-col `Memory` chiplet) | Intra-proof sorted `(ctx, addr, clk)` trace; inter-proof **Poseidon2/RPO Sparse Merkle Tree (`Smt`)** roots on stack | Fixed 40-felt `ExecutionClaim`: `MAST_root(4) \|\| Kernel(4) \|\| StackIn(16) \|\| StackOut(16)` — **trace length & RAM completely hidden** |
| **OpenVM** 1.x–2.1 (Modular RV32/64IM, Plonky3 -> Halo2 root) | Per-segment `timestamp_max_bits <= 29` (typ. `2^20..2^22` instrs) | Unbounded via 6-layer convergent STARK tree (`internal-recursive` fixed VK) | `(address_space, pointer)` w/ `pointer_max_bits <= 29` (`512 MiB–4 GiB`; RV64 in v2.1) | Intra-segment `BEGKN92` offline bus + **`PersistentBoundaryChip` Poseidon2 sparse Merkle tree** across segments | `root` strips `final_pc` & `final_root`, exposing to Halo2 EVM verifier (~316k gas) only `(user_public_values, app_exe_commit, app_vm_commit)` |
| **Ziren 2.0** / zkMIPS (MIPS32r2, KoalaBear Jagged + WHIR) | Chip height `< 2^22`, shard `clk < 2^26`, area budget | Unbounded shards (`(shard, clk)` global ts; multi-billion-cycle Geth blocks) | 32-bit (`4 GiB`; regs mapped to `0..35`) | **Septic-curve offline memory bus** (`global_cumulative_sum`; `MemoryInit/Finalize` in final shards) | BN254 Groth16/Plonk checks `(VK_ROOT_BYTES, program_vk, committed_value_digest)` — **shards, `(shard, clk)`, septic sums hidden** |
| **Pico 3.1** (Brevis, RV32IM, KoalaBear -> Gnark BN254) | `~2^20..2^22`-row chunks | Unbounded via `CONVERT/COMBINE/COMPRESS` STARK tree | 32-bit unified (regs `0..31` + RAM) | Cross-chunk offline memory (`InitializeFinalize` + `ReadWrite` chips) | `(riscvVKey, publicValuesDigest)` — **chunk states & counts hidden** |

### 4.2 Prover Throughput (`Hz` / `kHz` / `MHz`) & 2025–2026 Research Context vs. Sherlock

| System / Protocol | CPU Prover Speed & RAM | Single-GPU Speed | Multi-GPU Cluster Speed | Architectural Reason vs. Sherlock (`~0.24 Hz` at `S=16` -> `~5 Hz` at `S=512`) |
|---|---|---|---|---|
| **RISC Zero v3.0.4** | `~10–45 kHz` (`4.5–9.2 GB` RAM) | **`0.81–1.21 MHz`** (1x RTX 4090, 24 GB) | ETH block `44 s` (R0VM 2.0 cluster) | 31-bit BabyBear FRI leaves (`2^21..22` cyc/proof); curve SNARK runs **once** at root |
| **SP1 Hypercube** | Tens of kHz (`21 GB` RAM) | `~1–3+ MHz` (1x RTX 4090/5090) | `< 12 s` on 99.7% ETH blocks (**16x RTX 5090**) | 31-bit KoalaBear + Jagged PCS (`2^22` rows/shard); BN254 wrap runs **once** at root |
| **OpenVM 2.0 / 2.1** | Top CPU in Fenbushi (`8.2–11 GB` RAM) | **`19.4 MHz`** CoreMark (1x RTX 5090) | **`964 MHz`** (64x 5090); `< 9.7 s` p99 ETH (**4x 5090**) | Custom per-opcode chips (no central CPU table), 31-bit field, 1 Halo2 KZG proof at root |
| **Ziren 2.0** (`2026/2330`) | v1 CPU `~82 GB` RAM, tens of kHz | **`12.0 MHz`** warm (1x RTX 5090) | **`75.8 MHz`** peak (8x 5090); `8.6 s` avg ETH (**4x 5090**) | Per-opcode frames + LogUp-GKR + Jagged WHIR on KoalaBear; 1 BN254 wrap at root |
| **Pico 3.1** (Nov 2026) | `~1.5–2x` SP1 CPU (`20–21 GB` RAM) | Multi-MHz (1x RTX 5090) | `< 8.73 s` p99 ETH at 128-bit sec (**4x 5090**) | Degree-3 KoalaBear AIRs + coprocessors |
| **Jolt** (`Twist & Shout`, `2025/105`) | **`> 500 kHz`** (MacBook), **`> 1.0 MHz`** (**32-core CPU**); `< 2 GB` per `10^6` cyc (`< 2 GB` streaming target) | CPU-native sumcheck | Monolithic `10^8+` cyc on 1 CPU server | `< 800` field muls/cycle + sparse one-hot Dory commitments (`0`s are free; zero recursion!) |
| **Miden VM** v0.36 | **`20–25 kHz`** (1c M4 Max), **`170 kHz`** (16c M4 Max), **`270 kHz`** (64c EPYC BLAKE3; `152 kHz` Poseidon2); `11 GB` at `2^20` | CPU/SIMD-first | Edge/client proving (`2^20` in `3.7–5.9 s`) | 64-bit Goldilocks Plonky3 monolithic up to `2^29` cycles |
| **Nexus 1.0/2.0 -> 3.0** | v1/v2 Curve IVC: **`~200–813 Hz`** (`~4 GB` flat RAM); v3.0 Stwo: `~1,000x` faster | CPU M31 SIMD | Distributed network | **Direct analog:** Nexus 1.0/2.0 curve-folding IVC ran at hundreds of Hz for the exact same reason Sherlock runs at Hz — curve MSMs per batch — prompting Nexus 3.0's move to Stwo |
| **zkWasm** (Halo2 KZG) | Slow on CPU | **`~5–20 kHz`** (1x RTX 4090, `k=18..22`, `64–128 GB` RAM) | Parallel `continuation-batcher` | Closest proof-system cousin (**Halo2 KZG**): achieves `5–20 kHz` only by packing **`65k–1M` steps per `k=22` slice** + GPU MSM/NTT! |
| **Mangrove** (`2024/416`) & **Neo / SuperNeo** (`2025/294`, `2026/242`) | Mangrove: `2^24` gates in `~2 min` (`390 MB` RAM), `2^32` gates in `~8 h` (`800 MB` RAM); Neo/SuperNeo: small-field CCS folding w/ pay-per-bit Ajtai commitments | Laptop CPU | Streaming IVC | Shows how small-field/lattice folding avoids non-native curve collapses (`507k` of Sherlock's `823k` node rows), though Midnight Ledger 9 fixes KZG BLS12-381 (`tree.md:27,111`) |

**Three decisive prior-art lessons for Sherlock (verified URLs listed in §Evidence List):**
1. **Why Sherlock is `~0.24 Hz` while STARK zkVMs hit `MHz` (and `zkWasm` hits `5–20 kHz`):** Sherlock folds over a **single 255-bit pairing curve (BLS12-381 KZG)** with no 2-cycle (`architecture.md:55`), so every `k = 20` node pays `507,091` rows of non-native field/ECC accumulator collapse (`design.md:128`) and `~62 s` of 255-bit CPU MSM+NTT — yet at `S = 16`, it amortizes that `62 s` tax over **only 16 steps** (`zkWasm` amortizes Halo2 KZG over `65,536–1,048,576` steps/slice!). Raising `S` inside the leaf is therefore the exact architectural lever dictated by Halo2 KZG economics.
2. **Independent validation of `SVM-010` and `SRC-023`:** RISC Zero's `risc0/binfmt/src/exit_code.rs` explicitly warns that `ExitCode::SessionLimit` is **never emitted** because host-driven `SystemSplit` and page-fault cycles cannot prove deterministic limit exhaustion; SP1 Hypercube (`crates/core/executor/src/opts.rs`) likewise splits `GAS_TRACE_CHUNK_THRESHOLD` from `MINIMAL_TRACE_CHUNK_THRESHOLD` because shard cadence changes memory-boundary row counts. Sherlock's rule that fuel/outcomes are purely logical (`SVM-010`) and claims are partition-independent (`SRC-023`) avoids a known production zkVM trap.
3. **Universal validation of `C6`:** Every production zkVM on-chain verifier (`IRiscZeroVerifier.sol`, `SP1VerifierGroth16.sol`, OpenVM `root`, Ziren `wrap_bn254`, Pico, Starknet, Miden) strips intermediate memory commitments and shard counts before hitting L1, publishing only program identity + canonical I/O digests. *(Note: Sherlock additionally publishes `n_steps` alongside `claim_d` per `SRC-006`/`SRC-007` while excluding `n_steps` from `claim_d`, which is safe ONLY because `n_steps` counts deterministic logical owner steps, never padded circuit rows.)*

---

## Q5. [LEAD SECTION] Prover Throughput, Memory Footprint, Scaling to `2^64`, `S` Amortization, and Tested Targets on This Host

### 5.1 Host Hardware & Enforced Memory Envelope

All benchmarks in `qa/sherlock/runs.jsonl` (`713` recorded runs) and `research/` executed on this exact bare-metal machine:

| Component | Measured / Verified Specification | Citation |
|---|---|---|
| **Chassis & OS** | `ASUS ProArt P14 H7407BA`, native Linux `7.3.0-41-omarchy` (`aarch64`, bare metal, not WSL), `Samsung 9100 PRO` NVMe | `/sys/devices/virtual/dmi/id/*`, `uname -a`, `lspci` |
| **CPU Topology** | **20 cores / 20 threads** (no SMT, 1 NUMA node): `10x Cortex-X925` (`4,004 MHz` max) + `10x Cortex-A725` (`2,808 MHz` max); `25 MiB` L2 + `24 MiB` L3; `asimd`, `sve`, `sve2`, `sha3`, `sha512` | `lscpu` |
| **Unified RAM** | **`120.28 GiB`** (`129,147,846,656 B`) shared LPDDR5X (`273 GB/s` nominal, **`244 GB/s` measured R+W**); `120.28 GiB` swap (`4 kB` used) | `/proc/meminfo`, `gpu.md:15,18`, `design.md:327` |
| **Integrated GPU** | NVIDIA `RTX Spark N1X` (`GB20B`, `sm_121`, `48 SMs = 6,144` cores, `24 MB` L2, CUDA `13.4` V13.4.92, driver `615.78.08`); `u32 mad` `7.36 T/s` vs `u64 mul+mulhi` `0.54 T/s`. **Device memory counts in process RSS** (`cudaMalloc` 1 GiB -> `+1.0 GB` RSS; `cudaMallocManaged` 1 GiB -> `+1.3 GB` RSS). Prover kernels unbuilt past G0 smoke (`F23`). | `gpu.md:14-20`, `design.md:327,335`, `requirements.md:43` |
| **CPU Crypto Primitives** | `blst` G1 `multi_exp` (`2^20`, 20t): **`0.51 s`** (`39 ms` proj->affine); scalar `best_fft` (20t): **`60–78 ms`** (`2^20`), **`468–544 ms`** (`2^23` extended quotient domain!) | `gpu.md:21-22`, `design.md:327` |
| **Kernel & Policy Caps** | Per-process cap: **`48 GiB`** (`51.54 GB`, `SRC-016`); owner aggregate cap: **`100 GiB`** (`107.37 GB`, `SRC-011`, `SRC-016`) + `20 GiB` `MemAvailable` preflight reserve (`runs.jsonl:616`); active `heavy.slice` `72G` (`speed-levers.json:70`, `92G` in `t1b-node-concurrent-3.commands:4`); leaf cap: **`< 16 GB` RSS, `< 120 s` prove** (`SRC-014`, `e2-e4.md:19`). | `requirements.md:36,39,41`, `budgets.json:235-236,403-404` |

### 5.2 Consolidated Empirical Prover Baselines (Measured vs. `ESTIMATE`)

| Circuit / Phase | Domain `k` & Active Rows | Solo Prove Wall (20t) | Peak RSS | Proof / VK Bytes | Source / Citation |
|---|---|---|---|---|---|
| **M2 / A11 C6 Digest Leaf** | `k = 13` (`8,192`) | **`2.55 s`** prove (`3.40 s` warm cmd, `77.78 s` cold w/ keygen) | **`0.64 GB`** warm (`3.09 GB` cold) | `3,824 B` proof, `2,005 B` VK | `prover.md:11`, `runs.jsonl:638-639`, `e2-e4.md:28` |
| **Chip Leaf `E8` (`S=16, D=20`)** | `k = 13` (`6,511` rows) | `ESTIMATE` **`2.0–2.8 s`** (no KZG run yet) | `ESTIMATE` **`0.5–0.8 GB`** | `ESTIMATE` `~4,200 B` | `design.md:129`, `experiments-4.md:89-128`, `BRIEF.md:19` |
| **Full-VM Chip Leaf (`S=16, D=64`)** | `ESTIMATE` `k = 15..16` (`24k–38k` rows) | `ESTIMATE` **`4.0–6.5 s`** | `ESTIMATE` **`1.0–2.2 GB`** | `ESTIMATE` `~4,200 B` | Scaled from `k=13` (`2.55 s`) & `k=20` (`66.16 s`) via `d = 0.884` |
| **Stand-In `k = 20` Leaf (`E4`)** | `k = 20` (`742,327` rows) | **`66.16 s`** solo (`69.11 s` in `E4` concurrent; `23.87 s` PK, `87.04 s` SRS+VK, `0.015 s` verify) | **`11.14 GB`** (`11,139,792,896 B`) | **`3,824 B`** proof, `2,005 B` VK | `e2-e4.md:7-19`, `runs.jsonl:709,711` |
| **IVC Node `SHKIVC06`** | `k = 20` (`652,437` stand-in; `741,708..736` C6 `k=13..20`; `823,466` `E8` chip) | **`59.62–64.95 s`** at 20t (`61.07 s` `A16-node-step`; `72.73 s` 10t; `75.77 s` 6t; `102.96 s` 4t). Chip-keyed `823k`: `ESTIMATE` **`63–67 s`** | **`16.55–16.58 GB`** single step; **`17.31–17.38 GB`** fold+decider | **`6,432 B`** proof | `runs.jsonl:599,603,616,617,648,649`, `manifests/node.json`, `design.md:130,161` |
| **Node under `E4` Mixed Contention** | `k = 20` node (20t) + `k = 20` leaf `model_pick` (20t) | **`90.2 s`** (`~1.43x` slowdown vs `61–65 s` solo) | **`17.55 GB`** aggregate (non-overlapping prove peaks) | `6,432 B` | `e2-e4.md:19`, `runs.jsonl:711` |
| **`T1(b)` 3x Concurrent Nodes** | `3 x` `k = 20` genesis nodes (`6` threads each = `18` threads) | **`128.09 / 130.68 / 131.50 s`** (`f3 = 1.690–1.736x` vs `75.77 s` 6t solo; **`43.83 s/proof` net**) | **`47.73 GB`** aggregate (`16.29–16.33 GB`/proc) | `6,432 B` each | `runs.jsonl:616,618`, `c0-first-profile.md:57-59` |
| **Run Decider (`RunDeciderRelation`)** | `k = 20` (`929,166` model rows, `1,032,467` table rows) | **`70.35 s`** (`A16-fold3-decider`) to **`77.88 s`** (`c0-gp-r5-t20`) | Inside **`17.35 GB`** fold+decider process | **`6,432 B`** proof, `2,217 B` contract VK | `runs.jsonl:617,649`, `manifests/run-decider.json`, `manifests/sherlock-run-contract.json` |
| **Fixed Process Setup + Tail** | SRS+PK load (`~211.1 s` in `A16`) + `SRC-010` verify (`2.86 s` + `0.015 s`/proof) + `SRC-020` `validate_call` (`163.94 s`) | **`~378 s` (`6.3 min`)** fixed per-run non-prove overhead (+ `70.35 s` decider = **`~448 s` fixed tail**) | `15.00 GB` (`validate_call` `A6`), `0.51 GB` (`verify` `A5`) | — | `runs.jsonl:78,87,649`, `design.md:306` |

**Why single-host concurrency saturates at `1.39x` (`c0-first-profile.md:51-59`):** At 20 threads on `k = 20`, domain-proportional work is `d = 0.884` (`m_msm = 0.256`, `q_ntt = 0.566–0.573` over `2^23` extended coset FFTs, union `m_or_q = 0.77`), while average core utilization is only `U = 9.77 / 20` cores because `2^23` scalar FFTs (`256 MB` working buffers) and projective-to-affine SRS scans saturate the shared `244 GB/s` LPDDR5X bus and `24 MiB` L3 cache. Running 3 concurrent `k = 20` provers (`T1(b)`) multiplies memory-bus traffic 3x, inflating per-proof wall from `75.77 s` to `131.50 s` (`f3 = 1.736x`).

### 5.3 Seconds per Step & Effective `Hz` Across Folding Modes (at Current `S = 16`)

Using full-VM chip leaf `t_leaf = 4.0–6.0 s` (`ESTIMATE` at `k_leaf = 15..16`) and chip-keyed `SHKIVC06` node `t_node = 62.0–65.0 s` (`ESTIMATE` from `59.6–63.6 s` `A16` + `+11%` active rows at `d = 0.884`; central `66.0 s` combined leaf+node per 16 steps):

| Folding / Execution Mode (`S = 16` unless noted) | Asymptotic Time per 16-Step Leaf | Effective Seconds / Step | Effective Throughput (`Hz`) | Peak Host RSS | Bottleneck Diagnosis |
|---|---|---|---|---|---|
| **1. Sequential `S = 1` (Option F baseline)** | `16 x (4.0 + 62.0) = 1,056 s` (`ESTIMATE`) | **`66.0 s/step`** (`ESTIMATE`) | **`0.015 Hz`** (`ESTIMATE`) | `17.4 GB` | 100% node-dominated; `293` steps takes `5.5 h` (fails `SRC-011`) |
| **2. Sequential `S = 16` (1 proc, 20t, leaf -> node)** | `4.0 s + 62.0 s = 66.0 s` (`64.1–70.0 s` `ESTIMATE`) | **`4.13 s/step`** (`4.01–4.38 s` `ESTIMATE`) | **`0.242 Hz`** (`0.229–0.250 Hz` `ESTIMATE`) | **`17.4 GB`** (measured `A16`) | **`94%` of runtime is the `k=20` node fold!** Zero contention, lowest RAM |
| **3. Oversubscribed Pipelined Spine (`S = 16`, 20t node + 20t leaf worker, `E4` pattern)** | `max(90.2 s, 4.5 s) = 90.2 s` (`E4` measured node slowdown) | **`5.64 s/step`** (`ESTIMATE`) | **`0.177 Hz`** (`ESTIMATE`) — **SLOWER!** | `17.6–19.5 GB` (`E4`) | **Anti-pattern at `S = 16`:** hiding a `4 s` leaf by contending with a `62 s` node adds `+28 s` of bus contention! Even ideal `18t + 2t` pinning (`0` contention) yields at best `62 / 16 = 3.88 s/step` (`0.258 Hz`, `+6%`) |
| **4. GP-4 Binary Tree PCD (`{U, R, N, C, D}`) on THIS Single Host (`P = 3` x 6t workers)** | `M` leaves need `ceil(M/2)` `R`-lifts + `ceil(M/2)-1` `N`-merges = **`~M` `k=20` proofs** (`tree.md:44`). At `T1(b)` `131.50 s / 3 = 43.83 s/node` + `~1.5 s` leaf = `45.3 s` (`ESTIMATE`) | **`2.83 s/step`** (`ESTIMATE`) | **`0.353 Hz`** (`ESTIMATE`, `+46%` over serial) | **`47.73 GB`** (measured `T1(b)`) | Binary tree does **not** reduce total `k=20` proof count (`~M` either way), and shared LPDDR5X bandwidth caps 1-host parallel speedup at `~1.4x` for `2.7x` the RAM |
| **5. GP-4 Binary Tree PCD across Multi-Host Cluster (`W >= M/2` 120 GB machines)** | Depth `1 + ceil(log2(M/2))` serial `k=20` stages (`~63 s`/stage) (`ESTIMATE`) | `O((log2 M) / (16 M))` (`~0.15 s/step` at `4,096` steps on 128 hosts `ESTIMATE`) | **`~6.5 Hz`** at `4,096` steps (`W=128`), scales w/ `W` (`ESTIMATE`) | `17.4 GB` per host | **Where GP-4 tree wins:** horizontal scaling across separate memory buses cuts latency from `O(M)` to `O(log2 M)` |

### 5.4 Concrete Scaling Table Across Run Lengths (`16` Steps to `2^64` Steps) and Why `2^64` Is Physically Impossible

All columns below assume `S = 16` on **this single host**, showing both **Sequential 1-proc (20t)** (`66.0 s`/leaf `ESTIMATE` + `448 s` fixed setup/decider/verify/validate tail) and **3-way Single-Host Tree/Concurrent** (`45.3 s`/leaf `ESTIMATE` + `448 s` tail). Proof count in linear IVC is `M` leaves (`3,824 B`) + `M` nodes (`6,432 B`) + `1` decider (`6,432 B`) = `2M + 1` proofs, so the `SRC-019` GP bundle stores `10,256 * M + 6,432` raw proof bytes (plus `~15 KiB` manifests and `SRC-010` independent verification at `~0.015 s`/proof, `runs.jsonl:709`). Compact streamed witness trace is estimated at `256 B–2 KiB` per step (`ESTIMATE`).

| Run Length (`N` Steps) | Leaves `M` (`S=16`) | Total Proofs (`2M+1`) | Wall-Clock: Sequential 20t (`ESTIMATE`) | Wall-Clock: 3-Way Host Tree (`ESTIMATE`) | Peak Host RSS (Streaming Prover) | `SRC-019` GP Bundle Proof Bytes (`ESTIMATE`) | Compact Witness / Trace Storage (`ESTIMATE`) | Architectural & Physical Feasibility |
|---|---|---|---|---|---|---|---|---|
| **`16`** (`2^4`) | `1` | `3` | **`8.6 min`** (`514 s`; `136 s` prove-only) | **`8.6 min`** (1 leaf, no tree parallelism) | **`17.35 GB`** (measured `A16`) | **`16.3 KiB`** (`16,688 B`) | `4–32 KiB` | **Smoke target:** fits trivially (`< 20 min` `G-B`) |
| **`256`** (`2^8`) *(cf. `fact(3)=293`, `list123=303`)* | `16` *(19)* | `33` *(39)* | **`25.1 min`** (`1,504 s`; *`28.4 min` at 293–303 steps*) | **`19.6 min`** (`1,173 s`; *`21.8 min` at 303*) | **`17.4 GB`** seq / **`47.7 GB`** 3-way | **`166.5 KiB`** (`170,528 B`) | `64–512 KiB` | **Qualification `G-C` target:** passes `< 2 h` (`SRC-011`) with **`4.2x` margin** |
| **`4,096`** (`2^12`) | `256` | `513` | **`4.82 h`** (`17,352 s`) | **`3.35 h`** (`12,053 s`) | **`17.4 GB`** seq / **`47.7 GB`** 3-way | **`2.51 MiB`** (`2.63 MB`) | `1–8 MiB` | **Recommended soak target:** tests multi-segment `PAGE` + continuations in 1 afternoon |
| **`65,536`** (`2^16`) | `4,096` | `8,193` | **`3.13 days`** (`75.2 h`) | **`2.16 days`** (`51.7 h`) | **`17.4 GB`** seq (+ `< 50 MB` host tree) | **`40.07 MiB`** (`42.0 MB`; `verify` `~2 min`) | `16–128 MiB` | Multi-day weekend job at `S=16` (`~3.8 h` if `S=256`, §5.5) |
| **`2,097,152`** (`2^21`, **`D3` ceiling**; exact `D3` max = `2,097,136`) | `131,072` (`D3` max `131,071`) | `262,145` | **`100.2 days`** (`2,404 h` = `3.3 months`) | **`68.8 days`** (`1,651 h`) | **`17.4 GB`** prover + `~128 MB` host tree (`2^20` cells) | **`1.25 GiB`** (`1.34 GB`; `verify` `~66 min`) | `0.5–4.0 GiB` | **`D3` hard ceiling (`2,097,136` steps)**; already `> 3 months` on 1 host at `S=16` (`~5.4 days` at `S=256`) |
| **`16,777,216`** (`2^24`) | `1,048,576` | `2,097,153` | **`2.19 years`** (`801 days`) | **`1.50 years`** (`550 days`) | **`17.4 GB`** prover + `~1.0 GB` host tree (`2^23` cells) | **`10.02 GiB`** (`10.75 GB`; `verify` `~8.7 h`) | `4–32 GiB` | **Unprovable in 1 `D3` run**; feasible only on multi-host cluster + larger `S` (`~43 days` 1-host at `S=256`) |
| **`4,294,967,296`** (`2^32`) | `268,435,456` | `536,870,913` | **`561.4 years`** (`1.77 x 10^10 s`) | **`385.3 years`** | Prover `17.4 GB`, **HOST OOM (`> 256 GiB` RAM)** if `2^32` cells live in RAM! | **`2.50 TiB`** (`2.75 TB` — exceeds `150 GiB` disk guard unless pruned!) | `1–8 TiB` | **Exceeds single-host human lifetime, RAM (for dense `2^32` heap), and NVMe** without KZG->STARK backend shift |
| **`18,446,744,073,709,551,616`** (`2^64`) | `2^60` (`1.153 x 10^18`) | `2.306 x 10^18` | **`2.411 x 10^12` years** (`2.41` **trillion years** = `175x` age of universe) | **`1.655 x 10^12` years** | **Impossible** (`2^64` live cells = `> 1 Zettabyte` RAM) | **`11.82 Zettabytes`** (`1.182 x 10^22 B`) | `4.7–36.9 Zettabytes` | **Physically impossible in our universe** (see physics breakdown below) |

#### Quantitative Proof That `2^64` Steps Is Physically Impossible (`ESTIMATE`s Grounding the Owner's Point)

1. **Sequential Execution Wall (Even With ZERO Cryptography):** Every step `h + 1` of a deterministic VM depends on the post-state of step `h`, so parallel cores cannot compress sequential execution latency. Even if an idealized hardware core executed **1 full Sherlock VM step per clock cycle at this host's `4.004 GHz` max Cortex-X925 frequency** (`0.25 ns/step`), `2^64` steps requires:
   $$\frac{2^{64}}{4.004 \times 10^9\text{ steps/s}} = 4.607 \times 10^9\text{ s} = \mathbf{146.0\text{ years}}$$
   At a realistic compiled interpreter speed of `25 MHz` (`40 ns/step`), **unproved execution alone takes `23,382` years**.
2. **Proving Time:**
   - On this host (`0.242 Hz` at `S = 16`): **`2.411 trillion years`** (`175x` the `13.8`-billion-year age of the cosmos).
   - Even if Sherlock achieved `5.3 Hz` (`S = 512` pipelined): **`110.2 billion years`** (`8.0x` the age of the cosmos).
   - Even on a hypothetical **1,000-GPU RTX 5090 STARK cluster running at `10 GHz` aggregate** (ignoring sequential execution dependency!): `2^64 / 10^10 = 1.845 x 10^9 s` = **`58.5 years`** nonstop.
3. **Energy & Thermodynamics:**
   - At this laptop/workstation's `~80 W` sustained proving draw over `7.61 x 10^19 s`: **`6.09 x 10^21` Joules** (`1.69 x 10^15 kWh`), roughly **`10x` all human civilization's annual primary energy consumption** (`~6 x 10^20 J/yr`).
   - Even on an idealized `12 MHz` RTX 5090 STARK prover (`~500 W` = `41.7 uJ/step`), proving `2^64` steps consumes **`7.69 x 10^14` Joules (`213.6 GWh`)** — the entire annual electrical output of a `25 MW` power plant — and emits `11.82 Zettabytes` of Sherlock `SRC-019` proof bytes (exceeding planetary SSD manufacturing capacity).
   - Therefore, `2^64` in the spec must **never** be described as an executable target or circuit size; it is strictly the 64-bit arithmetic non-wraparound invariant of `SRC-004`.

### 5.5 Shifting the Bottleneck: `S` (Steps per Leaf) vs. Multi-Leaf Node vs. GPU

Why is Sherlock running at `0.24 Hz` (`4.13 s/step`) today? Because `S_MAX = 16` (`ivc.rs:77`, `SRC-014`) was sized before two decisive experiments landed on 2026-11-10:
- **`E2` (`e2-e4.md:25-35`):** Raising `k_leaf` from `13` to **`20`** adds only **`+4` node rows per bit (`+28` rows total!)** and **`0` decider rows**.
- **`E4` (`e2-e4.md:7-19`, `runs.jsonl:709`):** A `k_leaf = 20` circuit (`742,327` rows) proves in **`66.16 s`** at **`11.14 GB` peak RSS** — comfortably inside the `< 16 GB, < 120 s` leaf gate (`K_LEAF_MAX = 20`, `requirements.md:39`)!

Meanwhile, in the chip leaf (`experiments-4.md:63-128`, `design.md:196-208`), the marginal active-row cost per step (including `25` Core+ALU rows, `~46` `sherlock-value/1` rows, `~0.25` cold `PAGE` reads at `D = 64` = `~1,155` rows, and amortized `B = S / 16` Tail-Buffered `M3` flushes at `~560` rows/step) is **`ESTIMATE` `~1,800` rows/step** (`+ ~6,000` fixed `pre_d`/`post_d`/`PROG`-root rows per leaf)!

*(Critical coupling to Q2: under unmodified `D3` `C = 8`, a leaf can append at most 8 cells total, forcing allocation-heavy programs to close leaves after `~16–36` steps regardless of `S`! Scaling `S` past `16` REQUIRES allowing `B = ceil(S / 16)` segment flushes per leaf under Option (d), or widening `C`.)*

#### `ESTIMATE` Throughput Scaling vs. `S` on This Single Host (`D = 64` Option (d), `B = S / 16` flushes/leaf)

*(Leaf prove time `t_leaf(k)` scaled from measured `66.16 s` at `k = 20` and `2.55 s` at `k = 13` via domain-proportional `d = 0.884` (`c0-first-profile.md:51`); `t_node = 63.0 s` solo; pipelined `10t leaf + 10t node` uses `1.35x` contention factor from `T1(b)`/`E4`, `~28 GB` aggregate RSS.)*

| `S` (Steps / Leaf) | Max Appends `8B` | `ESTIMATE` Leaf Rows | `k_leaf` | `ESTIMATE` `t_leaf` (20t) | Sequential `t_leaf + t_node` | Sequential `s/step` (`Hz`) | Pipelined `1` Leaf + `1` Node (`Hz`, `~28 GB` RSS) | `fact(3)` (`293` steps) E2E Wall (`ESTIMATE`) | `4,096`-Step Soak E2E Wall (`ESTIMATE`) |
|---|---|---|---|---|---|---|---|---|---|
| **`16`** *(current)* | `8` | `~35,000` | `16` | `~5.5 s` | `68.5 s` | `4.28 s` (**`0.23 Hz`**) | `0.24 Hz` (not worth pipelining) | **`29.2 min`** (`19` leaves) | **`4.99 h`** (`256` leaves) |
| **`32`** | `16` | `~64,000` | `16` | `~6.2 s` | `69.2 s` | `2.16 s` (**`0.46 Hz`**) | `0.48 Hz` | **`19.0 min`** (`10` leaves) | **`2.59 h`** (`128` leaves) |
| **`64`** | `32` | `~121,000` | `17` | `~11.5 s` | `74.5 s` | `1.16 s` (**`0.86 Hz`**) | `0.95 Hz` | **`13.7 min`** (`5` leaves) | **`1.47 h`** (`64` leaves) |
| **`128`** | `64` | `~236,000` | `18` | `~20.5 s` | `83.5 s` | `0.65 s` (**`1.53 Hz`**) | **`1.82 Hz`** | **`11.7 min`** (`3` leaves) | **`52.0 min`** (`32` leaves) |
| **`256`** | `128` | `~467,000` | `19` | `~36.5 s` | `99.5 s` | `0.39 s` (**`2.57 Hz`**) | **`3.10 Hz`** (`~82.5 s` stage) | **`10.8 min`** (`2` leaves) | **`34.0 min`** (`16` leaves) |
| **`512`** | `256` | `~928,000` | `20` | `~68.0 s` *(cf. `E4` `66.2 s`)* | `131.0 s` | `0.26 s` (**`3.91 Hz`**) | **`5.02 Hz`** (`~102 s` stage) | **`9.7 min`** (`1` leaf!) | **`21.9 min`** (`8` leaves) |

**Comparison of the Four Throughput Levers:**
1. **Raising `S` in the chip leaf (`S = 16 -> 128..512`, `k_leaf = 16 -> 18..20`):** **`6.5x–21x` speedup on 1 host (`0.23 Hz -> 1.5–5.0 Hz`)**, `+4..16` node rows, `<= 28 GB` RSS, **zero new proof primitives or GPU risk**. *(Note: `S = 16` already clears the 320-step `< 2 h` `SRC-011` gate in `~29 min`, so keeping `S = 16` for initial M4 bring-up is low-risk, while raising `S_MAX` to `128` or `256` before freezing mainnet keys unlocks `6x–11x` higher practical run lengths!)*
2. **Batching multiple leaves per node (`self + leaf + leaf` or `R` 2-leaf lift):** **Inferior on 1 host.** Adding a 3rd verifier instance to a node pushes `node_rows` to `1,078,147` (`k = 21`, `F22` in `budgets.json:342-370`), doubling node time and canceling the 2-leaf gain; and in `GP-4` (`architecture.md:75`), `M/2` `R`-lifts (`U + U -> SEG`) plus `M/2 - 1` `N`-merges (`SEG + SEG -> SEG`) still total **`M` `k = 20` proofs**!
3. **Single-host tree concurrency (`P = 3` nodes):** **`1.39x` speedup** (`T1(b)`), capped by unified LPDDR5X bandwidth (`f3 = 1.736x`) at `47.73 GB` RSS. Valuable mainly on **multi-host clusters**, where `GP-4` gives `O(log2 M)` wall time.
4. **Integrated `sm_121` GPU (`mori-gpu`, `gpu.md:335`):** `m_or_q = 0.77` gives a theoretical Amdahl ceiling of `1 / (1 - 0.77) = 4.35x` and realistic `ESTIMATE` **`1.6x–2.6x` speedup** on `t_node` (`63 s -> ~25–40 s`), because both CPU and GPU share the **same `244 GB/s` LPDDR5X memory bus** (`gpu.md:18`) and device buffers count against the `48 GiB` process RSS cap (`gpu.md:16`). Combined with `S = 256..512`, `sm_121` could push this host to `ESTIMATE` **`~8–12 Hz`** (`~10^6` steps/day), still unbuilt/unfunded today (`dissent.md:181`).

### 5.6 Recommended Concrete Tested Targets for the Spec

The spec (`GP-001`, `SRC-011`, `SRC-015`, `SAS-001`) should explicitly name **three empirical tested tiers** on this 20-core, 120 GiB host — separating fast regression, qualification gate `G-C`, and multi-segment/continuation soak — while assigning `2^64` exclusively to boundary negative tests and formal proof:

| Target Tier | Concrete Workload & Step Count | Leaf / Fold Shape (at `S = 16`) | Seeded Wall-Clock Cap (`ESTIMATE` Expected) | Enforced RSS Cap (`SRC-016`) | What It Proves |
|---|---|---|---|---|---|
| **Tier 1: Smoke / Gate `G-B`** (`SRC-015(b)`, row `A18`) | (a) `entry 7;` (`6` steps, partial leaf w/ `10` disabled slots); (b) 2-leaf link smoke (`20–32` steps) | `1` leaf & `2` leaves (`3–5` total proofs) | **`<= 20 min` (`1,200 s`)** *(expected `~8.6–9.8 min`)* | **`<= 24 GiB`**/proc (`<= 48 GiB` agg) | Genesis check (`SRC-021`), partial-leaf padding (`SRC-014`), non-genesis fold (`prev_h > 0`), decider + `SRC-010` + `SRC-020` |
| **Tier 2: Qualification / Gate `G-C`** (`SRC-011`, `SRC-015(c)`, rows `A19, A19b, A33, A38`) | `fact(3)` (`293` steps), `list123` (`303` steps), unseen `P3` (`<= 320` steps), plus 3-partition replay check (`SRC-023`) | `19–20` leaves (`39–41` proofs, `~205 KiB` bundle) | **`<= 2 h` (`7,200 s`)** *(expected `~28–32 min` seq at `S=16`)* | **`<= 48 GiB`**/proc, **`<= 100 GiB`** agg (`~17.4 GB` seq actual) | Full rule inventory, cross-leaf `PAGE` reads, `D7` frame cache, `sherlock-value/1`, `SRC-023` partition replay refusal |
| **Tier 3: Multi-Segment & Continuation Soak** *(New row recommended for `GP-001` / `SAS-001`)* | Deterministic recursive/heap workload of **`4,096` steps (`2^12`)** allocating `>= 512` cells, proved **(i)** as 1 run (`256` leaves) and **(ii)** as a **2-chunk `GP-2` continuation (`2,048 + 2,048` steps)** | `256` leaves (`513` proofs, `2.51 MiB` bundle) | **`<= 6 h` (`21,600 s`)** at `S = 16` *(expected `~4.8–5.0 h`; `< 45 min` if `S >= 128`)* | **`<= 24 GiB`** single-proc (`<= 48 GiB` cap) — asserts `RSS(leaf 256) - RSS(leaf 1) <= 1 GiB` | Deep multi-segment `M3` flushes, flat `O(1)` streaming prover RSS (no leak across 256 folds), `SRC-019` bundle scaling, and end-to-end `GP-2` continuation equivalence (`claim_d(1-run) == claim_d(2-chunk)`) |
| **Boundary / Formal Only (Never Executed to Height)** | `prev_h = 2^64 - 1`, `steps = S_MAX + 1`, `steps = 0`, `steps = p - 1` (`SRC-004` `raw_heights`) + Quint `F20` & Lean `F21` (`L5`) | Single-node constraint boundary + inductive proof | `< 5 min` (`MockProver` / single node negative + `lake build`) | `<= 16 GiB` | Proves `h < 2^64` non-wraparound soundness without claiming physically impossible `2^64`-step execution |

---

## Evidence List

### Repository Paths, Commits, and Line Citations (`c57351dd43cec8cb9f2db55d8350c0388c1f65ab`)
- `research/sherlock-runlength-consensus-20261111/BRIEF.md:1-79`
- `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:1-58` (especially `:9-18` GP-1, `:20-29` GP-2, `:31-40` GP-3, `:42-52` GP-4)
- `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:1-66` (`SVM-002` `:10`, `SVM-010..012` `:18-20`, `SRC-001..023` `:26-48`, `GP-001..004` `:56-59`, `SAS-001` `:65`)
- `openspec/changes/sherlock-vm-consensus-20261107/design.md:121-225,281-342` (§2.7.11 `:125-137`, C6 `:138-146`, §3.1 `:150-162`, §3.2 `:163-225`, §3.5 `:281-309`, §3.6 `:310-324`, §3.7 `:325-338`)
- `openspec/changes/sherlock-vm-consensus-20261107/dissent.md:150-232` (speed replan `:150-185`, design review `:186-193`, C6 & `D-1..D-6` `:194-226`, `D3` ceiling conflict `:228-232`)
- `openspec/changes/sherlock-vm-consensus-20261107/tasks.md:1-144` & `assurance.md:1-70`
- `qa/sherlock/runs.jsonl:49,50,77,78,80,87,93,245,394,487,527-566,598-618,638-641,648-649,708-711` (`713` total run records)
- `qa/sherlock/budgets.json:1-584` (`:225-317` C0/T1b, `:332-375` F22 tree probe, `:235-236,403-404` 48/100 GiB caps)
- `qa/sherlock/decisions/speed-levers.json:1-115`
- `qa/sherlock/manifests/{c6-leaf,node,run-decider,sherlock-run-contract,parent,terminal-decider}.json`
- `qa/sherlock/profiles/c0-gp-r4-t20/GP-resident_pk-t20-0-off.json:23-80` & `qa/sherlock/t1b-node-concurrent-3.commands:1-10`
- `qa/sherlock/rowprobe-20261110/{README.md,rows.jsonl}`
- `wiki-llm/recursion-perf/profiles/c0-first-profile.md:19-65`
- `research/sherlock-recursion-design-review-20261110/experiments/{e2-e4.md,e2-rows.jsonl,e3.json,e4-solo.json,e4-concurrent.json}`
- `research/sherlock-processor-analogy-20261110/{study.md,d7-frames.md,experiments.md,experiments-2.md,experiments-3.md,experiments-4.md}`
- `research/sherlock-c6-outcome-ledger9-review-20261110/review.md:1-259`
- `research/sherlock-gp-architecture-20261110/architecture.md:1-215`
- `research/sherlock-speed-replan-20261108/{prover.md,tree.md,gpu.md}`
- `crates/mori-recursion/src/ivc.rs:24-26,64-116,363-422,456-565` & `crates/mori-recursion/src/run_decider.rs:48-81,190-329`
- `crates/mori-chip/src/{lib.rs:42,rf.rs:32-95,rfcore.rs:61-220,mem.rs:117-242,m3.rs:50-55,96-144,289-448,poseidon.rs:52-61,113-115,range.rs:30-35,92-129,alu.rs:23-26,87-130,vsel.rs:25-119}`
- `crates/mori-backend/src/{proving.rs:21-26,parameters.rs:12-60,sherlock_c6_child.rs:409-520}` & `tools/sherlock-sdk/sherlock-run.compact:19-47`

### External Prior-Art URLs Verified
- **RISC Zero:** `https://raw.githubusercontent.com/risc0/risc0/main/risc0/binfmt/src/image.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/binfmt/src/sys_state.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/binfmt/src/exit_code.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/circuit/rv32im/src/lib.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/circuit/rv32im/src/execute/platform.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/zkvm/src/claim/work.rs`, `https://raw.githubusercontent.com/risc0/risc0-ethereum/main/contracts/src/IRiscZeroVerifier.sol`, `https://dev.risczero.com/api/zkvm/receipts`, `https://medium.com/@CFrontier_Labs/unlocking-po2-22-segment-proving-in-risc0-zkvm-81c4cb9f4232`
- **SP1:** `https://docs.succinct.xyz/docs/sp1/hypercube/shards`, `https://raw.githubusercontent.com/succinctlabs/sp1/main/crates/primitives/src/consts.rs`, `https://raw.githubusercontent.com/succinctlabs/sp1/main/crates/core/executor/src/opts.rs`, `https://raw.githubusercontent.com/succinctlabs/sp1/main/crates/hypercube/src/air/public_values.rs`, `https://raw.githubusercontent.com/succinctlabs/sp1-contracts/main/contracts/src/v5.0.0/SP1VerifierGroth16.sol`, `https://blog.succinct.xyz/real-time-proving-16-gpus/`
- **Jolt / Twist & Shout:** `https://jolt.a16zcrypto.com/`, `https://jolt.a16zcrypto.com/roadmap/streaming.html`, `https://a16zfrontier.com/posts/article/jolt-6x-speedup/`, `https://raw.githubusercontent.com/a16z/jolt/main/common/src/constants.rs`, `https://eprint.iacr.org/2025/105`
- **Nexus:** `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/README.md`, `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/common/src/constants/mod.rs`, `https://fenbushi.vc/2025/08/29/benchmarking-zkvms-current-state-and-prospects/`
- **Valida:** `https://arxiv.org/html/2505.08114v1`, `https://raw.githubusercontent.com/lita-xyz/valida-vm/main/machine/src/machine.rs`, `https://raw.githubusercontent.com/lita-xyz/valida-vm/main/machine/src/persistence.rs`, `https://raw.githubusercontent.com/lita-xyz/valida-vm/main/memory/src/lib.rs`
- **Cairo / Stwo:** `https://raw.githubusercontent.com/starkware-libs/stwo-cairo/main/README.md`, `https://raw.githubusercontent.com/starkware-libs/stwo-cairo/main/stwo_cairo_verifier/crates/cairo_air/src/lib.cairo`, `https://raw.githubusercontent.com/starkware-libs/stwo-cairo/main/stwo_cairo_verifier/crates/cairo_air/src/components/memory_address_to_id.cairo`, `https://zksecurity.github.io/stark-book/cairo/bootloader.html`
- **zkWasm:** `https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/README.md`, `https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/crates/zkwasm/src/circuits/etable/mod.rs`, `https://raw.githubusercontent.com/DelphinusLab/continuation-batcher/main/README.md`
- **Miden VM:** `https://raw.githubusercontent.com/0xMiden/miden-vm/main/core/src/lib.rs`, `https://raw.githubusercontent.com/0xMiden/miden-vm/main/core/src/program/claim.rs`, `https://raw.githubusercontent.com/0xMiden/miden-vm/main/processor/src/execution_options.rs`, `https://raw.githubusercontent.com/0xMiden/miden-vm/main/docs/src/performance.md`
- **OpenVM:** `https://docs.openvm.dev/specs/architecture/continuations/`, `https://docs.openvm.dev/specs/architecture/memory/`, `https://blog.openvm.dev/2.0-beta`, `https://www.axiom.xyz/blog/2-1`
- **Ziren 2.0 / zkMIPS:** `https://eprint.iacr.org/2026/2330`, `https://www.zkm.io/blog/ziren-2-0`, `https://docs.zkm.io/design/continuation.html`, `https://docs.zkm.io/design/prover-architecture/prover-architecture.html`, `https://raw.githubusercontent.com/ProjectZKM/Ziren/main/crates/verifier/src/lib.rs`
- **Pico 3.1 & 2025–2026 Folding / PCS:** `https://blog.brevis.network/2026/11/05/pico-3-1-128-bit-provable-security-still-real-time-on-4-gpus-now-fully-open-source/`, `https://eprint.iacr.org/2024/416` (Mangrove), `https://eprint.iacr.org/2025/247` (LatticeFold+), `https://eprint.iacr.org/2025/294` & `https://eprint.iacr.org/2026/242` (Neo / SuperNeo)

---

## What Was Not Checked

1. **No live builds, `MockProver` runs, `cost_model` invocations, or real KZG proofs were executed** (strictly prohibited by study rules). All chip-leaf proving times (`E8` at `k = 13` and full-VM leaf scaling across `S = 16..512` and `D = 20..67`), chip-keyed `SHKIVC06` proving wall (`823,466` rows vs measured `741,708`-row `A16` wall), and Option (d) 8-slot tail barrel-shifter row counts (`~560` mux rows) are analytical `ESTIMATE`s derived from measured `E2`, `E4`, `E8`, and `C0` fits, not fresh timing receipts.
2. **Overlapping `prove`-phase contention between a chip leaf (`k = 15..19`) and a `k = 20` node (`A23`):** As documented in `e2-e4.md:19`, during `E4`'s concurrent run the `k = 20` node finished proving at `148 s` while the `k = 20` leaf was still in CPU-heavy `model_pick` (`361.7 s`), before the leaf's KZG `prove` phase began. True simultaneous `prove` + `prove` contention between a `k = 16..19` chip leaf on `A725` efficiency cores and a `k = 20` node on `X925` performance cores has not been empirically measured on this host.
3. **Live Midnight Preview / Mainnet `LedgerParameters` transaction size and fee ceilings (task 6.8, row `A41`, `tasks.md:129`):** While the decider proof size (`6,432 B`) and `acceptRun` VK (`2,217 B`) are measured locally, live Preview node transaction limits for `SherlockClaims` / `CONT_JOIN` public inputs were not queried.
4. **Exact normal form of `decl_schema_d` inside `ty_d` (`dissent.md:204-207` `D-2`, `SRC-022` `[OPEN]`) and exact `cost_model` rows of the `{U, R, F, N, C, D}` GP-4 relations (`architecture.md:152-153`):** Neither has been implemented or probed in `crates/mori-recursion` yet; `GP-4` tree proof counts use the measured `self + self = 730,243`-row (`k = 20`) probe from `budgets.json:342-370`.
5. **No other lane's directory (`lanes/A`, `lanes/B`, `lanes/C`, `lanes/E`) was listed or read** during Round 1.
