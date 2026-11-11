# Sherlock Recursion Run Length, Memory Ceiling, and Continuations — Lane C Round 2 (Cross-Review & Final Stance)

**Study:** `sherlock-runlength-consensus-20261111` (`research/sherlock-runlength-consensus-20261111/BRIEF.md:1-79`)  
**Lane:** C — Specification, GP-1..GP-4 Semantics, C6 Outcome-Only Publication & Continuations  
**Repository Snapshot:** `c57351dd4` (`main`, 2026-11-11)  
**Cross-Reviewed Reports:** `lanes/A/round1.md`, `lanes/B/round1.md`, `lanes/D/round1.md`, `lanes/E/round1.md`

---

## Section 1: What Lane C Now Accepts from Lanes A, B, D, and E (and Updates to Lane C Round 1)

Cross-reviewing all five Round 1 reports against the repository (`crates/mori-core`, `crates/mori-chip`, `crates/mori-recursion`, `clean/fork/Clean/Sherlock/Aggregation.lean`, `models/sherlock/quint/aggregation.qnt`, `openspec/`, and `research/`) surfaced five high-value technical improvements that strengthen or correct parts of Lane C's Round 1 position:

### 1.1 Q2 — Upgrade from `Wide M3 + act[0]` Bypass to **Tail-Buffered Dense `M3` (`TOP = 61`, `2^64` Cells)** (Accepting from Lanes A, B, D, and E; `Exploit E-5`)
- **What Lane C proposed in Round 1 (`lanes/C/round1.md:189-199`):** Widening `M3` (`TOP = 61..64`) plus a 1-bit `act[0]` empty-leaf bypass so zero-append leaves (`used = 0`) do not advance `len` or consume a segment.
- **Why Tail-Buffered Dense `M3` is strictly stronger (verified via Lane E's `Exploit E-5`):**
  - Reading `clean/fork/Clean/Sherlock/Aggregation.lean:148-223,1136-1155` and `models/sherlock/quint/aggregation.qnt:12-15,59,89` confirms a critical formal nuance: in `Aggregation.lean`, `det` (`∀ p s t u, P.Step p s t → P.Step p s u → t = u`, line `1137`) and `digest_inj` (`Set.InjOn P.digest P.Carrier`, line `201`) require `leaf_sound` (`lines 211-213`) to produce a **deterministic `post_d` for the state reached after `h` steps**, and `aggregation.qnt:14,89` encodes `post_d == StateAt(prog_d, input_d, h + steps)` directly.
  - Our Round 1 `act[0]` bypass eliminated segment burn on `used = 0` leaves, **not** on partially filled leaves (`1 <= used <= 7`), which still padded `len_post = base + 8` on leaf close. Consequently, an allocating 16-step trace proved as one leaf `[16]` (`2` total appends -> `len = 16`) vs two leaves `[7, 9]` (`1` append each -> `len = 24`) produced **different `len` values, different subsequent `RefC::Data(addr)` addresses, and different `post_d` digests at `h = 16`**. Proving `SRC-023` under padded `M3` would therefore require quotienting `P.State` modulo dead padding cells, and `GP-4` parallel workers could not pre-compute intermediate `pre_d`/`post_d` digests without fixing leaf cut points in advance.
  - **Accepted upgrade:** Adopt **Tail-Buffered Dense `M3` (`M3-Tail`, `TOP = 61`, `2^64` cells, `W = 15` 7-old + 8-new window, `W_flush = 1`)** as independently converged on by Lanes A (`lanes/A/round1.md:155-182`), B (`lanes/B/round1.md:122-145`), D (`lanes/D/round1.md:107-118`), and E (`lanes/E/round1.md:121-151`). Advancing `len_post = len_pre + used` (`0 <= used <= 8`), carrying the `0..7` uncommitted tail cells `(len, committed_root, tail_d)` in `pre_d`/`post_d`, and flushing an aligned 8-cell subtree to the depth-`61` Merkle tree **if and only if** `floor(len_post / 8) > floor(len_pre / 8)` makes `post_d(h)` a **strictly deterministic, partition-independent function of `(prog_d, input_d, h)`**, satisfying `Aggregation.lean` and `aggregation.qnt` literally with zero quotienting.

### 1.2 Q2 — Drop Carbon's `D-1` Backward Cons-Linked Slices; Adopt **Single Unified Dense Arena + Poseidon Hash-Chain Stacks (`FRAMES`, `TASKS`)** (Accepting `Exploit E-12` from Lane E + Single-Arena Continuity from Lane B)
- **What Lanes A, C, and D proposed in Round 1:** Pairing dense or wide `M3` with Carbon's `D-1` backward cons-linked slices `(TAG_SLICE, elem_ref, prev_link)` (`openspec/changes/sherlock-vm-consensus-20261107/dissent.md:198-202`).
- **Why Lane E's `Exploit E-12` refutes backward cons-linked slices (verified in `mori-core`):**
  - Direct inspection of `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime/owner_adt_runtime.rs:907` (`impl_plan_walk`, rule `RuleIdC::XAdtHelperPlanWalk`) and `crates/mori-core/src/sherlock/helper_machine/families/structural.rs:90,179-180,287` (`BHelperMatchField`, `BHelperEqual`, `BHelperCompare`) confirms that **a single `/1` step executes `pre.field_at(&state.cur, &field_idx)`** (`fields.start + field_idx`) for arbitrary `0 <= field_idx < m` on an `m`-arity constructor.
  - If `fields` is stored as a backward cons-linked list pointing to field `m - 1`, projecting `field_idx = 0` in one `XAdtHelperPlanWalk` step requires **`m` sequential pointer dereferences (`m` `PAGE` Merkle openings!) in a single step**. Whenever `m > P_mem` (e.g., `m = 9` with `P_mem = 8`), either the step cannot fit a leaf (violating `GP-1` universal language coverage) or `PlanWalk` must be split into `m` sub-steps (altering `n_steps` and `consumed_work` against `SVM-001`!).
- **Better still — Single Unified Dense Arena (`W_flush = 1`) preserves `SI-12` and `SI-23` natively:**
  - Auditing **every** heap-allocating transition in `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:2960-3058,3506-3591`, `families/adt_build.rs:16-61`, and `families/environment.rs:201-284` proves (detailed in §2.2 and §3.2) that **Lane B's Single Unified Dense Arena (`len_post = len_pre + used`, `W_flush = 1`) + Poseidon Hash-Chain Stacks (`FRAMES`, `TASKS`)** (`research/sherlock-gp-architecture-20261110/architecture.md:126,245,251`; `lanes/B/round1.md:122-145`) keeps **every** constructor field slice (`SI-12`) and **every** `value_slots` slice (`SI-23`) **100% contiguous (`start + k`)** with **zero** extra arenas and **zero** multi-arena flush livelocks—saving `8,540` leaf rows over Lane E's 4-arena `W_flush = 2` variant.

### 1.3 Q1 — `u64` Counter/Address Symmetry (`TOP = 61`, `2^64` Cells) and Clean Separation Between `GP-1` Definition and `SAS-001` Qualification Bounds (Accepting from Lanes A, D, and E)
- With dense `M3-Tail` (`len_post = len_pre + used`), memory no longer burns 8 cells per 1-append leaf. Setting **`TOP = 61` (`8 * 2^61 = 2^64` cells)** makes both architectural non-overflow widths symmetric 64-bit unsigned counters (`h < 2^64` steps and `len < 2^64` cells; `lanes/A/round1.md:107-111`, `lanes/D/round1.md:62-66`, `lanes/E/round1.md:68-73`), rather than our Round 1 `TOP = 64` (`2^67` cells).
- We also accept Lane A and Lane E's point (`lanes/A/round1.md:94-98`, `lanes/E/round1.md:45,58-60`) that `general-purpose.md` should keep the normative universal definition cleanly separated from release-specific hardware qualification tiers (`SAS-001`, `SAS-012`), requiring a multi-leaf run (`>= 256` leaves / `>= 4,096` steps in `GP-001` qualification, `>= 64` leaves / `>= 1,024` steps in routine `G-C`) with constant final proof byte length while citing `SAS-001`/`SAS-012` for hardware-tested bounds.

### 1.4 Q3 — Continuation Hardening (`Exploit E-1`, `E-8`, `E-10`), Unmodified `orig_input_d` Threading, and Two-Mode Synthesis (`canon_state_d` + `sherlock-ckpt/1`) (Accepting from Lanes B, D, and E)
1. **Diamond-DAG `n_claims` & `total_steps` Range Checks (`Exploit E-1`, Lane E):** Verified in `research/sherlock-gp-architecture-20261110/architecture.md:100,107-111`: `SET` adds `n_claims = n_L + n_R` without stating a non-wrap bound. A depth-64 self-join diamond `D_{i+1} = JOIN(D_i, D_i)` reaches `2^64` in 64 proofs and wraps `0 mod r` in `<= 510` proofs, forging an empty/small claim count. Every additive counter across `GP-4` (`h`, `n_claims`, `total_steps`) **must** enforce `< 2^64` (or `u128 < 2^128` via two `< 2^64` limbs) at every recursive merge.
2. **Disjoint Continuation Domain (`Exploit E-8`, Lane E) & `u128` Scalar Range Check on Unfold (`Exploit E-10`, Lane E):** Verified in `crates/mori-chip/src/alu.rs:6-9,84-125`: `OP_LT` decomposes `diff = y - 1 - x` (`128` bits) **without** re-range-checking `x` and `y`, relying on the inductive invariant that every register/memory integer is `< 2^128` from genesis. Therefore, any checkpoint continuation must **(a)** use a disjoint domain tag (`SHK_CONT_V1`, never `Halted` or `SHK_RUN`), and **(b)** either chain back to `h_lo = 0` via `CONT_JOIN` (`Mode 1`) or pass every unfolded `Int` limb pair `(mag_lo, mag_hi)` through two `u64` (`RANGE`) checks on `UNFOLD` (`Mode 2`).
3. **Unmodified `orig_input_d` Threading (Lanes B & E):** Instead of embedding `Q_0` inside `input_d[0]` (our Round 1 primary row at `lanes/C/round1.md:333`, though we noted `origin_in_d` erasure at `:354`), carry unmodified `orig_input_d = H(SHK_IN, 1, val_d, fuel_lo, fuel_hi, stack_limit)` across every segment and bind `[h_lo, h_hi)` (`quantum_steps`) solely inside the internal `SHK_CONT_V1` header. Then an `M`-session continuation settles the **exact same `[prog_d, orig_input_d, outcome_d, n_steps]` and `claim_d`** as a single uninterrupted session.
4. **Two-Mode Continuation Synthesis (Unifying Lane E's `canon_state_d` with Lanes A/B/C/D's `sherlock-ckpt/1`):** Once Q2(d) makes `post_d(h)` deterministic and partition-independent, cross-session continuations where the prover retains the dense arena snapshot need **only** `left.post_canon_d == right.pre_canon_d` (`O(1)` boundary check, **zero** `UNFOLD` rows — Lane E's `canon_state_d`, Mode 1!), while address-free `sherlock-ckpt/1` + `u128`-checked lazy `UNFOLD` (Mode 2) serves **stateless/GC-compacted checkpoints** (`O(|live|)` state transfer, dropping dead historical allocations) **and** solves `SRC-021`'s `[OPEN]` structured input `val_d` admission (`design.md:190-191`).

### 1.5 Q4 & Q5 — SP1 `v6` Reconciliation, OpenVM `CHUNK = 8`, `verifier_gadget.rs` Proof, Continuation CVEs, and `R`-Lift Proof Count (`U + U -> SEG`)
- **Q4:**
  - **SP1 v6 Hypercube (`v6.1.0` vs `dev`):** Verified live that both Lane A (`v6.1.0` tag `749b0c63`, pure global `SepticDigest`) and Lane C (`dev` branch `crates/hypercube/src/air/public_values.rs:39-46,113-138`, which added `prev_merkle_root: [T; 8]`, `merkle_root: [T; 8]`, `trace_chunk_idx`, and `num_merkle_shard` for multi-chunk 48-bit memory) accurately read real upstream commits; §3.4 documents **both**.
  - **OpenVM `CHUNK = 8`:** Live fetch of `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/system/memory/controller/mod.rs` confirms `pub const CHUNK: usize = 8;` (`address_height = 29 - 3 = 26`, matching Lane A `lanes/A/round1.md:364`; correcting our Round 1 `CHUNK = 4` note).
  - **Why offline LogUp memory is blocked in Sherlock (`lanes/A/round1.md:148-151`):** Verified in `crates/mori-recursion/src/verifier_gadget.rs:16-17` that `prepare_verify` requires `num_challenges <= 1` (single-phase Plonk, zero Phase-2 Fiat-Shamir challenges).
  - **2024–2026 Continuation CVEs (`lanes/E/round1.md:302-313`):** Accept Lane E's CVE mapping (`OpenVM CVE-2026-62333`, `Jolt PR #975`, `RISC Zero r0vm 2.0.0`).
- **Q5 (`R`-lift proof count correction from Lanes B & D):**
  - Verified in `research/sherlock-gp-architecture-20261110/architecture.md:75`: in the 3-circuit `GP-4` split `(U, R, N)`, circuit `R` lifts **two** `U` leaves at once (`U + U -> SEG`). Thus folding `L` leaves takes `ceil(L / 2)` `R` proofs + `ceil(L / 2) - 1` `N` proofs = **`L` (`L - 1` for even `L >= 2`) `k = 20` proofs**—not `2L - 1` as our Round 1 (`lanes/C/round1.md:501`) assumed for 1-leaf lifts. With `3 × 6t` workers (`43.83 s`/node at `47.73 GB` RSS, `qa/sherlock/runs.jsonl:618`), single-host `GP-4` tree proving reaches **`~2.83–3.06 s/step`** (`~1.38×` faster than `4.13 s/step` serial, `ESTIMATE`), conditional on 2-leaf `R` fitting `< 950,000` rows at `k = 20` (§3.5).

---

## Section 2: What Lane C Rejects and Why Across Lanes A, B, D, and E

### 2.1 Reject Carbon's `D-1` Backward Cons-Linked Slices (Lanes A, D, and Lane C Round 1)
- **Where proposed:** `lanes/A/round1.md:133-136,169`, `lanes/C/round1.md:200-202`, `lanes/D/round1.md:107`.
- **Technical refutation:** In `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime/owner_adt_runtime.rs:886-909`, `XAdtHelperPlanWalk` executes `let value = pre.field_at(&state.cur, &field_idx)?;` in **one** `/1` step, where `field_idx` comes from compiled decision-tree `prog.fieldPaths` (`0 <= field_idx < ctor.arity`). Likewise `BHelperMatchField`, `BHelperEqual`, and `BHelperCompare` (`families/structural.rs:90,179-180,287`) project arbitrary field index `cursor` in 1 step. A backward cons-linked list `(TAG_SLICE, elem_ref, prev_link)` stores only the tail pointer (`m - 1`), turning `field_at(cur, 0)` into **`m` sequential pointer hops (`m` `PAGE` reads) inside a single step** (`Exploit E-12`). That either fails to prove constructors of arity `m > P_mem` (violating `GP-1` universal language support) or changes step counts relative to `/1` (`SVM-001`). Single-arena dense `M3-Tail` (§2.2) preserves `O(1)` `start + field_idx` indexing with **zero** extra `PAGE` hops.

### 2.2 Reject Lane E's 4-Arena Split and `W_flush = 2` Walk Penalty (`+8,540` Leaf Rows) (`lanes/E/round1.md:103,136`)
- **Where proposed:** `lanes/E/round1.md:103,132-136` splits `M3-Tail64` into 4 arenas (`DATA, FIELDS, ENV, SLOTS`) keyed by `tag || idx`, observes that `adt_build.rs:40-50` writes `1 FIELDS + 1 DATA` in one step, and therefore pays **`W_flush = 2` (`4 * 61 = 244` walk perms = `17,080` walk rows)** so two arenas can flush simultaneously (`59,809` total leaf rows).
- **Technical refutation from `mori-core` source:** Inspecting **every** allocation site in `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:2960-3058,3506-3591`, `families/adt_build.rs:16-61`, and `families/environment.rs:201-284` proves that **1 unified dense arena (`len_post = len_pre + used`) preserves 100% contiguity of both `SI-12` (`FIELDS`) and `SI-23` (`SLOTS`) with `W_flush = 1`**:
  1. **`DATA` and `ENV` are never slice-indexed:** They are accessed strictly by exact cell pointer `RefC::Data(addr)` or `EnvRootC(addr)`.
  2. **`SLOTS` (`value_slots`, `SI-23`) is written ONLY inside `HelperControlC::Append` (`environment.rs:241,261`):** Because `helper: Option<HelperControlC>` is a single exclusive sub-machine register, while `helper == Some(Append { dest_start, .. })` runs, **only** `append_cell` (`0` allocs), `append_copy` (`+1` slot), `append_empty` (`0` allocs), `append_tail` (`+1` slot), and `append_exit` (`0` allocs) can fire. Zero `DATA`, `FIELDS`, or `ENV` cells can interleave, so `[dest_start .. dest_start + src.len]` is **strictly contiguous (`dest_start + k`)** in the unified arena!
  3. **`FIELDS` (`data_fields`, `SI-12`) is written ONLY in `alloc_data` (`owner_runtime.rs:3507-3545`) and `HelperControlC::Build` (`adt_build.rs:37-55`):**
     - `alloc_data` (`Leaf`, `Cons`, `Branch`) pushes its `1` or `2` fields back-to-back `[len, len + 1]` and then its `Data` row at `len + 2` in the same step (`100%` contiguous; or `1` cell total if `<= 2` children are inlined).
     - While `helper == Some(Build { field_start, .. })` runs, **only** `BuildCell` (`0` allocs) and `BuildField` can fire—zero `ENV` or `SLOTS` writes can interleave. And on the final field (`GBuildLast`, `owner_runtime.rs:3547-3559`), the machine pushes `alloc_data_field` **first** at `addr = field_start + m - 1` and `alloc_constructor` **second** at `addr = field_start + m`! Thus all `m` fields sit contiguously at **`[field_start .. field_start + m - 1]`**, while `SI-13` (`child < owner`, since `field_start + m - 1 < field_start + m`) and `SI-14` (`prev_env < env`) hold identically on unified `len`!
  4. **Zero simultaneous-flush livelock (`W_flush = 1`):** Because `alloc_data` (`<= 3` cells) and `BuildField` (`<= 2` cells) are mutually exclusive (`owner_runtime.rs:2960`), **every `/1` step appends at most `3` cells** (`<= 8`). Closing a leaf whenever `used + next_step_appends > 8` guarantees `0 <= used <= 8`, so `floor((len_pre + used) / 8) - floor(len_pre / 8) in {0, 1}` **always**! `W_flush = 1` (`122` walk perms = `8,540` rows) is 100% complete and saves **`8,540` leaf rows** (`~14%` of the leaf) over Lane E. *(Logical resource counters `alloc.data_nodes`, `alloc.data_fields`, `alloc.environment_cells`, and `active_value_slots` remain separate `u128` registers for `P6` exhaustion, preserving `SVM-001` and `SVM-010` bit-for-bit.)*

### 2.3 Reject Lane B's `B = 32` (`2^35` Cells) Memory Narrowing (`lanes/B/round1.md:17,67,108,216`)
- **Where proposed:** Lane B recommends `B = 32` (`2^32` segments = `2^35` cells) over `B = 61` (`2^64` cells) (`lanes/B/round1.md:108,145`) and edits `GP-001` to permit per-run bounds `< 2^64` (`lanes/B/round1.md:67`).
- **Technical refutation:**
  1. **Zero prover speedup inside `k_leaf = 16`:** In Lane B's own table (`lanes/B/round1.md:108`), `B = 32` takes `36,561` rows (`k = 16`) and `B = 61` takes `50,771` rows (`k = 16`). Both compile to the **exact same Halo2 domain `2^16 = 65,536`** with **`0` added columns, `0` added lookups, `+0` node rows (`823,466`), and `+0` decider rows (`929,166`)**! In PLONK/KZG (`midnight-proofs`), FFT and MSM costs depend on `2^k` and column count, not unused padding rows inside the same `2^16` domain, and leaf proving (`~6–8 s`) is completely hidden behind the `~61 s` `k = 20` node anyway.
  2. **Unnecessary `GP-1` asymmetry:** Capping single-run cells at `2^35` while `h < 2^64` is `64`-bit forces `GP-001` to rely on cross-run continuations (with lazy `UNFOLD` overhead) just to defend `GP-1` non-narrowing (`BRIEF.md:8`). Setting **`TOP = 61` (`2^64` cells)** gives full 64-bit counter + address symmetry inside `k_leaf = 16` at **zero** recursive proving cost.

### 2.4 Reject Editing `general-purpose.md:9`'s Headline Sentence (Lanes B and D)
- **Where proposed:** Lane B (`lanes/B/round1.md:61`) rewrites `general-purpose.md:9` from *"for every run length"* to *"for any physically realizable run length"*; Lane D (`lanes/D/round1.md:56`) rewrites `:9` to *"for any finite run length"*.
- **Technical refutation:** `BRIEF.md:8,27` explicitly specifies: *"GP-1..GP-4 are never narrowed"* and *"Give exact replacement text for `general-purpose.md` GP-1 (the run-length bullet and the acceptance line)"*. Keeping `general-purpose.md:9-13` **100% verbatim** and placing the exact architectural explanation in the run-length bullet (`:14`) and acceptance line (`:16`) avoids any perception of softening the charter headline.

### 2.5 Reject Lane A's `Halted(Left(ms_v))` Continuation Option & Claim That Multi-Run Continuations Must Expose `K` Claims On-Chain (`lanes/A/round1.md:14,473,497-499`)
- **Where proposed:** Lane A (`lanes/A/round1.md:473`) considers `Halted(Left(ms_v))` as a continuation outcome and asserts (`:14, 497-499`) that multi-run GP-2 continuations either require guest trampolines or publish `K` replay entries in `settled`.
- **Technical refutation:**
  1. **Finality spoofing (`Exploit E-8`):** Any continuation segment returning `Halted(...)` (`tag = 0`) is accepted by the `SRC-006` Run Decider and `acceptRun` on-chain as a **completed execution** of `prog_d`, letting an attacker settle an intermediate checkpoint as a final output! Continuation claims **must** use a disjoint internal tag `SHK_CONT_V1` rejected by the Run Decider.
  2. **Zero on-chain leakage via `CONT_JOIN` cut-elimination:** Adding `CONT_JOIN` (`SHK_CONT_V1` cut-elimination, `~250–350` node rows `ESTIMATE`) inside `N` (`architecture.md:81-97`) folds `M` continuation runs off-chain into **one** terminal `SHK_RUN` claim (`1` C6 `claim_d` entry on-chain, `0` guest code changes).

### 2.6 Reject Lane D's Fuel-Splitting `WithFuel(in_0, fuel_a + fuel_b)` Continuation Scheme (`lanes/D/round1.md:176,185`)
- **Where proposed:** Lane D sets `input_d_k = H(SHK_IN_CONT, in_0, chk_d_{k-1}, fuel_k, stack_limit)` and has `CONT_JOIN` reconstruct `WithFuel(in_0, fuel_a + fuel_b)` (`lanes/D/round1.md:176,185`).
- **Technical refutation:**
  1. **Semantics divergence (`Exploit E-6`):** In `MachineImage` (`crates/mori-core/src/sherlock/helper_machine/shared.rs:1508-1510`), `initial_fuel` and `remaining_work` are distinct state registers initialized from `input_d`. Running segment 0 with `fuel_0 < initial_fuel` alters `initial_fuel` and `remaining_work` at every step compared to an unsplit run!
  2. **Unnecessary circuit complexity:** `WithFuel(in_0, fuel_a + fuel_b)` forces `N` to witness and re-hash `in_0`'s 6-element preimage on every merge. Threading unmodified `orig_input_d` while carrying exact `(initial_fuel, remaining_work, consumed_work)` inside the checkpoint (`Lanes B, C, E`) preserves exact `/1` state invariants and needs only equality `L.orig_input_d == R.orig_input_d` in `N`.

### 2.7 Reject Lane E's Claim That `sherlock-ckpt/1` + Lazy `UNFOLD` Should Be Discarded Altogether (`lanes/E/round1.md:201-205`)
- **Where proposed:** Lane E argues (`lanes/E/round1.md:201-205`) that address-free `sherlock-ckpt/1` + lazy `UNFOLD` should be rejected entirely in favor of raw `canon_state_d(h)` (`SHK_CONT_V1`).
- **Technical refutation:** `canon_state_d(h)` is indeed the right **Mode 1** fast path when the prover keeps the dense `MEM` snapshot across sessions (`0` `UNFOLD` cost). However:
  1. `canon_state_d(h)` requires transferring the **entire un-compacted append-only arena (`O(h)` historical cells, including dead garbage)** to the next worker—after `10^8` steps with `99.9%` garbage, `canon_state_d(h)` still drags `~10^8` cells of Merkle witness data, whereas `sherlock-ckpt/1` compacts the checkpoint to **`O(|live reachable graph|)`**; and
  2. **`SRC-021` (`design.md:190-191`) ALREADY needs lazy `UNFOLD`** to admit structured program inputs `val_d = vd(x)` at `h_lo = 0` without scanning `MEM`! Once `UNFOLD` range-checks unfolded `u128` integer limbs via `RANGE` (`2` rows, closing `Exploit E-10`), `sherlock-ckpt/1` + lazy `UNFOLD` is **100% sound** and reuses the exact `SRC-021` input gadget (`Mode 2`).

### 2.8 Reject Raising `S = 16` (`k_leaf = 16`) to `S = 256..512` (`k_leaf = 19..20`) for M4 (`lanes/D/round1.md:28,316-326`)
- **Where proposed:** Lane D proposes raising leaf step batching to `S = 256` (`k_leaf = 19..20`) or `S = 512` (`k_leaf = 20`) to reach `~0.25–0.40 s/step` (`lanes/D/round1.md:316-326`).
- **Technical refutation:**
  1. **Violates `design.md:105,129` (`k_leaf <= 16` M4 gate):** At `S = 256`, worst-case `M3-Tail` flushes (`3 * 256 / 8 = 96` segment flushes -> `96 * 122 * 70 = 819,840` walk rows alone!) and `PAGE` reads (`64..128` openings -> `295k..590k` rows) **exceed `2^19` and blow past `2^20 = 1,048,576`** unless aggressive early leaf-closing caps are imposed—which then underfills leaves on memory-active code!
  2. **Re-triggers measured `E4` memory-bandwidth contention:** In `experiments/e2-e4.md:15-19` (`qa/sherlock/runs.jsonl:709-711`), a `k = 20` leaf (`11.14 GB` RSS, `66.2 s` solo) overlapped with a `k = 20` node slowed the node from `~61 s` to **`90.2 s` (`+48%`)** due to LPDDR5X bandwidth saturation, whereas a `k = 16` leaf (`< 1.5 GB` RSS, `~6–8 s`) fits L3/SLC cache with near-zero node slowdown. Keep **`S = 16, k_leaf = 16` for M4**, and evaluate moderate `S = 64` (`k_leaf = 17..18`, `< 3 GB` RSS) only as a measured post-M4 (`M5`) optimization.

### 2.9 Reject Minor Q4 Prior-Art Inaccuracies in Peer Reports
- **Valida on-chain verifier (`lanes/E/round1.md:277`):** Valida (`lita-xyz/valida-vm`) has **no** on-chain single-proof Solidity/SNARK verifier in repository; `MultiSegmentMachineProof` (`machine/src/proof.rs`) exposes `Vec<SegmentProof>` off-chain.
- **Nexus v1/2 vs v3 (`lanes/A/round1.md:323`, `lanes/B/round1.md:249`):** Distinguish Nexus 1.0/2.0 (unbounded HyperNova + CycleFold IVC with Poseidon Merkle tries and 1-scalar SNARK compression) from Nexus 3.0 (`zkvm-spec-3.0.pdf`, single-proof Stwo over `M31` capped at `<= 2^28` rows with LogUp).

---

## Section 3: Lane C's Final Self-Contained Position on Q1 to Q5

### 3.1 Final Position on Q1 — `2^64`, GP-1, GP-001, and Honest "Unbounded" Semantics

#### 1. Direct Answer to the Owner's Questions (`BRIEF.md:6-7`)
- **What `2^64` is NOT:** `2^64` is **never** a circuit row count, trace table allocation, polynomial degree, or computation Sherlock tries to execute. Every Sherlock circuit has a small, constant row count fixed once at key generation:
  - Chip leaf (`S = 16`, Single-Arena `M3-Tail` `TOP = 61`): **`~41,443 – 50,683` rows at `k = 16`** (`2^16 = 65,536` rows, `ESTIMATE`; E8 spike = `6,511` rows at `k = 13`, `openspec/changes/sherlock-vm-consensus-20261107/design.md:129`).
  - Recursive node (`SHKIVC06` / `R` / `N`): **`~823,470 – 836,772` rows at `k = 20`** (`2^20 = 1,048,576` rows, `ESTIMATE` from `design.md:130`).
  - Run decider (`SHKRUND1`): **`929,166` rows at `k = 20`** (`design.md:131`).
  Proving an `N`-step run folds `ceil(N / 16)` fixed-size leaves incrementally; unexecuted steps cost **zero** work (`BRIEF.md:14`).
- **What `2^64` ACTUALLY is (`SRC-004`):** The **64-bit unsigned integer register width (`u64`)** of the step counter `h` (`H_BITS = 64` in `crates/mori-recursion/src/ivc.rs:79,359-376`) and dense memory cell counter `len < 2^64`. Because Halo2 arithmetic works modulo the 255-bit BLS12-381 prime `r ≈ 2^255`, an unchecked addition `prev_h + steps` could wrap `r - 1 + 1 ≡ 0 (mod r)` and forge `is_genesis = 1` mid-run (`clean/fork/Clean/Sherlock/Aggregation.lean:1208-1213`, theorem `unchecked_prev_wraps_to_genesis`). Range-checking `prev_h < 2^64` and `h < 2^64` costs **`~30–50` rows** (`0.005%` of the `k = 20` node, `research/sherlock-speed-replan-20261108/prover.md:181-186`, `ESTIMATE`) and guarantees `prev_h + steps < 2^65 < r` (`Aggregation.lean:1196-1206`, theorem `src004_height_bridge`)—identical to giving a CPU a 64-bit cycle counter (`u64`).
- **Per-run vs. multi-run bounds:** With Q2(d)'s Single-Arena `M3-Tail` (`TOP = 61`, `len < 2^64` cells), **no** 17-bit D3 segment cap remains; both single-run counters (`h < 2^64`, `len < 2^64`) sit millennia beyond physical reach, and multi-run continuations (`Q3`) compose across sessions under the same key set.
- **What "unbounded run length" honestly means with fixed VKs:**
  1. **Key & verifier universality (`O(1)` keys, `O(1)` `6,432 B` proof, `O(N)` prover):** Zero trace-length, program-size, or heap-size constants in PKs, VKs, or the on-chain contract (`general-purpose.md:9-13`).
  2. **Unreachable 64-bit counter/address widths (`h < 2^64`, `len < 2^64`):** Fixed-width `u64` non-wrap guards prevent field wraparound without bounding any physically realizable run.
  3. **Explicit separation of key universality from finite tested qualification (`SAS-001`, `SAS-012`, `requirements.md:65,76`):** Release artifacts report the exact finite step counts tested on hardware (`1,024` steps in `< 2 h` `G-C`; `4,096` steps in `GP-001` endurance soak), never claiming `2^64` steps were executed.

#### 2. Exact Drop-In Replacement Text for `general-purpose.md` GP-1
**Current lines (`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:14-16`; lines `9-13` kept verbatim):**
> Line 14: `- The run length is bounded only by the height field (below 2^64).`  
> Line 16: `Acceptance: one key set and one deployed verifier contract accept proofs of at least three programs that the key set was not built from, including one program larger than one leaf's carriers, one run with input, and one run of each outcome kind.`

**Drop-in replacement for `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:14-16`:**
```markdown
- Run length is not bounded by circuit size or by the key set. A run of N steps is proved incrementally by folding fixed-size S-step leaf proofs into a constant-size final proof, with proving work proportional only to the steps actually executed (no circuit is sized to the maximum run length). Within a single run, step and memory-cell counters use 64-bit unsigned non-overflow checks (h < 2^64 in SRC-004 and len < 2^64 in authenticated memory, preventing scalar-field wraparound modulo r) rather than trace pre-allocation; checkpointed multi-session runs, when used, fold under GP-2 and GP-4 into one on-chain proof under the same key set. Hardware-qualified run lengths are recorded separately under SAS-001 and SAS-012.

Acceptance: one key set and one deployed verifier contract accept proofs of at least three programs that the key set was not built from, including one program larger than one leaf's carriers, one multi-leaf run at or above the qualified long-run target whose final proof byte length equals that of a single-leaf run, one run with input, and one run of each outcome kind.
```

#### 3. Exact Drop-In Replacement Text for `requirements.md` `GP-001` (Strict EARS)
**Current line (`openspec/changes/sherlock-vm-consensus-20261107/requirements.md:56`):**
> `| GP-001 | U | GP-1 universal VM. One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every run length below 2^64 and every outcome (Halted(v) for every value type, as a digest of a canonical value encoding; Rejected(code); Exhausted(kind)); ...`

**Drop-in replacement row for `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:56`:**
```markdown
| GP-001 | U | GP-1 universal VM. One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every outcome (`Halted(v)` for every value type, as a digest of a canonical value encoding; `Rejected(code)`; `Exhausted(kind)`), and across any finite run length within the 64-bit non-wrap counter widths (`h < 2^64` in SRC-004 and `len < 2^64` in authenticated memory, or across multi-run continuations composed via GP-002 and GP-004) by folding fixed-size `S`-step segment proofs into a constant-size final proof with proving work proportional to the steps executed (`O(1)` circuit and proof size, no trace-length circuit constant). The public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants; capacity and run length SHALL NOT depend on the key (state larger than one leaf's carriers runs against committed memory, and no per-run memory-segment index SHALL cap execution below `2^64` cells or steps); a new program SHALL need no new key, contract or ceremony. When a supported run exceeds one leaf's step, storage or hashing capacity, the prover SHALL continue through authenticated memory and recursive decomposition under the same key set without changing logical execution or resource accounting. | One key set and one deployed verifier contract accept proofs of at least three programs the key set was not built from, including one program larger than one leaf's carriers, one multi-leaf run at or above the qualified long-run target (`>= 4,096` owner steps across `>= 256` leaves at `S = 16`, recorded per SAS-001/SAS-012) whose final proof byte length equals that of a single-leaf run (`6,432 B`), one run with input, and one run of each outcome kind (row A33). | `general-purpose.md` GP-1; SRC-004; SRC-017; SAS-001; SAS-012; C6 review 2026-11-10; runlength consensus 2026-11-11 |
```

#### 4. Matching Mirror for `openspec/changes/sherlock-vm-consensus-20261107/specs/sherlock-general-purpose/spec.md:7-15`
```markdown
### Requirement: GP-001 Universal VM
One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every outcome (`Halted(v)` for every value type as a digest of a canonical value encoding, `Rejected(code)`, `Exhausted(kind)`), and across any finite run length within the 64-bit non-wrap counter widths (`h < 2^64` in SRC-004 and `len < 2^64` in authenticated memory, or across multi-run continuations composed via GP-002 and GP-004) by folding fixed-size `S`-step segment proofs into a constant-size final proof with proving work proportional to the steps executed. The public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants. Capacity and run length SHALL NOT depend on the key: a program whose state does not fit one leaf's carriers runs against committed memory, and no per-run memory-segment index SHALL cap execution below `2^64` cells or steps. A new program SHALL need no new key, contract or ceremony.

When a supported run exceeds one leaf's step, storage or hashing capacity, the prover SHALL continue through authenticated memory and recursive decomposition under the same key set without changing logical execution or resource accounting.

#### Scenario: Programs and multi-leaf runs the keys were not built from
- **WHEN** one key set and one deployed verifier contract receive proofs of at least three programs the key set was not built from, including one program larger than one leaf's carriers, one multi-leaf run of at least 4,096 owner steps across at least 256 leaves (recorded under SAS-001/SAS-012), one run with input, and one run of each outcome kind.
- **THEN** every proof is accepted with no new key and no new contract, and the final proof byte length equals that of a single-leaf run (row A33).
```

#### 5. Downstream Requirements Impact Summary
- **`SRC-004` (`requirements.md:29`):** Keep `prev_h, h < 2^64`; add `len_pre, len_post < 2^64` in memory and **mandate `< 2^64` (or `u128 < 2^128`) range checks on `GP-4` `SET.n_claims` and `CONT.total_steps`** (`Exploit E-1`, `architecture.md:100`).
- **`SRC-006` & `SRC-007` (`requirements.md:31-32`):** Unchanged on-chain (`[prog_d, input_d, outcome_d, n_steps]` and `claim_d = H(SHK_RUN, 1, prog_d, input_d, outcome_d)`); Run Decider continues to reject non-terminal tags (`Running`, `SHK_CONT_V1`).
- **`SRC-021` (`requirements.md:46`):** Close `[OPEN]` structured `val_d` admission using the `u128`-range-checked lazy `UNFOLD` gadget (§3.3).
- **`SRC-022` & `SRC-023` (`requirements.md:47-48`):** `outcome_d` unchanged; `SRC-023` holds with **100% completeness and literal `post_d(h)` determinism** across all partitions (`S = 16`, `S = 1`, mixed).

---

### 3.2 Final Position on Q2 — Single-Arena Tail-Buffered Dense `M3` (`TOP = 61`, `2^64` Cells) + Hash-Chain Stacks

#### 1. Exact Architecture (Option (d))
1. **Single Unified Dense Append-Only Heap Arena (`TOP = 61`, `C = 8`, `W = 15`, `W_flush = 1`):**
   - Stores immutable 4-word tagged records `(1, kind_tag, word_a, word_b)` for `DATA`, `FIELDS`, `ENV`, and `SLOTS` in one dense address space `0 <= addr < len < 2^64` (`2^61` aligned 8-cell segments).
   - At leaf entry, `r_open = len_pre mod 8 in 0..7`, and the leaf witnesses the `r_open` uncommitted tail cells `T_pre[0 .. r_open - 1]` authenticated against `pre_d`'s `tail_d`, placing them at indices `0 .. r_open - 1` of a `W = 15`-slot leaf buffer (`r_open <= 7` + `used <= 8` -> `r_open + used <= 15`).
   - Each step appends `0, 1, 2, or 3` records (`owner_runtime.rs:2960,3506-3565`) at consecutive dense indices `len_cur, len_cur + 1, ...`, advancing `len_post = len_pre + used` (`used <= 8`).
   - **Flush predicate (`do_flush in {0, 1}`):** `do_flush = 1` iff `r_open + used >= 8` (equivalently `floor(len_post / 8) == floor(len_pre / 8) + 1`).
     - When `do_flush == 1`: buffer slots `0 .. 7` hash into one 8-cell subtree (`7` perms) inserted at segment index `seg = floor(len_pre / 8) < 2^61` (`old_leaf == 0` -> `new_leaf` via `2 * 61 = 122` perms in `WalkConfig`, `crates/mori-chip/src/m3.rs:104-144`), and residual slots `8 .. r_open + used - 1` (`0..7` cells) form `T_post` hashed into `tail_d'`.
     - When `do_flush == 0`: `committed_root' = committed_root` (`0` segments consumed!) and slots `0 .. r_open + used - 1` (`0..7` cells) form `T_post` hashed into `tail_d'`.
   - **`PAGE` reads:** Any read at `addr < len_cur` checks `addr >= 8 * floor(len_pre / 8)` to mux directly from the `W = 15` local buffer (`0` Merkle rows!), or else verifies a depth-`64` (`61 + 3`) Merkle opening against `committed_root` (`66` perms = `4,620` rows/read `ESTIMATE`).
2. **Poseidon Hash-Chain LIFO Stacks for `FRAMES` and `TASKS` (`architecture.md:126,245,251`):**
   - Top frame and top helper task live in registers; `PUSH` updates `frame_chain_d' = H(dom("SHK_FRM"), top_words..., frame_chain_d)` (`3` perms = `210` rows `ESTIMATE`), `POP` witnesses the parent frame and asserts hash equality (`3` perms), and `RewriteTop` updates registers in `0` perms. Neither `FRAMES` nor `TASKS` touches `M3` or `PAGE`.
3. **Why `SI-12..SI-14`, `SI-23`, and `SRC-023` Hold Exactly:**
   - As proved in §2.2 from `owner_runtime.rs:2960-3058,3506-3591`, `adt_build.rs:16-61`, and `environment.rs:201-284`, exclusive `HelperControlC` ownership guarantees zero interleaved allocations during `Build` or `Append`, keeping every `FIELDS` slice (`SI-12`) and `SLOTS` slice (`SI-23`) contiguous (`start + k`) with `child < owner` (`SI-13`) and `prev_env < env` (`SI-14`), while `post_d(h)` is bit-for-bit identical across all leaf partitions (`Aggregation.lean` `det` + `digest_inj`).

#### 2. Complete Cost Comparison Across Q2 Options (`S = 16`, `ESTIMATE`)

| Criterion | (a) Keep D3 (`TOP=17`) + GP-2 Cont. | (b1) Padded D3 (`TOP=32`, `2^35` cells) | (b2) Padded D3 (`TOP=64`, `2^67` cells) | (c1) 4-Arena `M3-Tail` (`W_flush=2`, Lane E) | **(d) Final: 1-Arena `M3-Tail` (`TOP=61`, `W_flush=1`) + Hash Stacks** |
|---|---|---|---|---|---|
| **Single-run cell cap** | `1,048,568` (`< 2^20`) | `2^35` (narrows `< 2^64`) | `2^67` (pads `len`) | `2^64` per arena | **`2^64` cells (`len < 2^64`, matches `h < 2^64`)** |
| **Empty-leaf segment burn** | `1` seg/leaf (`2^17` cap!) | `1` seg/leaf (`2^32` cap) | `1` seg/leaf (`2^64` cap) | **`0` segs (`do_flush=0`)** | **`0` segs (`do_flush = 0`)** |
| **Full leaf rows (`P_mem=4, P_prog=6`)** | `~20,511` (`k=15`, `ESTIMATE`) | `~29,751` (`k=15`, `ESTIMATE`) | `~49,491` (`k=16`, `ESTIMATE`) | `~50,569` (`k=16`, `ESTIMATE`) | **`~41,443` (`k = 16`, `24.1k` headroom, `ESTIMATE`)** |
| **Full leaf rows (`P_mem=6, P_prog=6`)** | `~23,591` (`k=15`, `ESTIMATE`) | `~34,931` (`k=16`, `ESTIMATE`) | `~59,011` (`k=16`, `ESTIMATE`) | `59,809` (`k=16`, `ESTIMATE`) | **`~50,683` (`k = 16`, `14.9k` headroom, `ESTIMATE`)** |
| **Full leaf rows (`P_mem=8, P_prog=8`)** | `~28,631` (`k=15`, `ESTIMATE`) | `~39,131` (`k=16`, `ESTIMATE`) | `~61,531` (`k=16`, `ESTIMATE`) | `~71,009` (**spills to `k=17`!**, `ESTIMATE`) | **`~61,883` (`k = 16`, fits `<= 65,536`!, `ESTIMATE`)** |
| **Added leaf cols / lookups** | `0` cols, `0` lookups | `0` cols, `0` lookups | `0` cols, `0` lookups | `+0..1` cols, `0` lookups | **`+0..1` cols (time-muxed), `0` lookups (`ESTIMATE`)** |
| **Node rows (`k = 20`)** | `823,466` (+ extra circuits) | `~823,470` (`ESTIMATE`) | `~823,470` (`ESTIMATE`) | `~823,470–836,772` (`ESTIMATE`) | **`~823,470 – 836,772` (`k = 20`, `> 113k` headroom, `ESTIMATE`)** |
| **Decider rows (`k = 20`)** | `929,166` | `929,166` (`+0`) | `929,166` (`+0`) | `929,166` (`+0`) | **`929,166` (`+0` rows)** |
| **Literal `post_d(h)` det. (`E-5`)** | **FAILS** (pads `len`) | **FAILS** (pads `len`) | **FAILS** (pads `len`) | **PASSES** (`Aggregation.lean`) | **PASSES** (`Aggregation.lean` & `aggregation.qnt` literally) |
| **`SI-12` & `SI-23` (`start + k`)** | Broken without map | Broken without map | Broken without map | Preserved (`W_flush=2`) | **Preserved natively (`W_flush = 1`)** |

---

### 3.3 Final Position on Q3 — Continuations, `GP-2`, `GP-4`, and C6 Outcome-Only Publication

#### 1. Three-Layer Execution & Continuation Hierarchy
- **Layer A — Intra-Run Segment Recursion (`U -> R -> N -> D`, Default for All Single-Session Runs `< 2^64`):**
  With Single-Arena `M3-Tail` (`TOP = 61`), `post_d(h)` is already canonical and partition-independent. Any single-session run of `N < 2^64` steps folds directly via linear IVC or `GP-4` binary tree (`SEG + SEG -> SEG`) into one `6,432 B` proof with **zero** cross-run state serialization.
- **Layer B — Cross-Session / Multi-Epoch Continuations (`SHK_CONT_V1` + `CONT_JOIN` in `N`, Guest-Transparent):**
  Because `/1` forbids source-level suspension (`SVM-009`, `owner_runtime.rs:4514-4523`) and user-space wrappers cannot capture mid-call `frames`, `env_root`, or `HelperControlC` (`SRC-021`), cross-session continuations operate as a **VM Continuation Mode** at deterministic logical step boundaries `[h_lo, h_hi)` (`quantum_steps`, never physical leaf fills, preserving `SRC-023` and `SVM-010`), supporting two interoperable state commitments:
  - **Mode 1 — Retained-Arena Fast Path (`canon_state_d(h)`, `O(1)` Stitch, `0` `UNFOLD` Rows):** When the next session retains the dense `MEM` arena snapshot, `ckpt_d = canon_state_d(h) = H(dom("SHK_STATE_V1"), regs_canon, frame_chain_d, task_chain_d, len, committed_root, tail_d)`. Session `k + 1` resumes in `O(1)` rows from `ckpt_d` with zero re-hashing or unfolding; `CONT_JOIN` inductively chains `h_lo = 0 -> h_1 -> ... -> N`, automatically preserving `alu.rs`'s `< 2^128` register/memory invariant!
  - **Mode 2 — Stateless / GC-Compacted Checkpoint (`sherlock-ckpt/1` + `u128`-Checked Lazy `UNFOLD`, Also Closing `SRC-021`):** When compacting unreachable garbage across long-lived epochs (reducing witness transfer from `O(h)` historical allocations to `O(|live|)`) or admitting structured initial input `val_d = vd(x)` at `h_lo = 0` (`SRC-021`, `design.md:190-191`), `ckpt_d` hashes recursive structural digests `(vd, ed, sd_contig, fd, hd)` + logical `counters_d` + `(orig_input_d, h_hi)`. At `Resume`, sealed handles `(TAG_SEALED, digest)` unfold lazily on first touch (`~3–4` perms = `210–280` rows/node `ESTIMATE`), **passing unfolded `Int` limbs `(mag_lo, mag_hi)` through two `64`-bit `RANGE` checks (`+2` rows) so `alu.rs:84-125` cannot be fed `>= 2^128` field elements (`Exploit E-10`)**. *(Note: contiguous slices `sd_contig` in Mode 2 materialize their `m`-element pointer block on `XAdtHelperPlanEntry` / `Append` entry or keep `Mode 1` for large-arity constructors so 1-step `field_at` stays `O(1)`.)*

#### 2. Exact Statements Across `M` Continuation Segments (`k = 0 .. M - 1`)

| Claim Kind | Domain Tag | Statement Tuple Verified Inside Recursion / Decider | Accepted On-Chain by `acceptRun`? |
|---|---|---|---|
| **Checkpoint Segment (`0 <= k < M - 1`, or `k = M - 1` if non-terminal)** | `dom("SHK_CONT_V1")` (`!= SHK_RUN`) | `(prog_d, orig_input_d, mode, h_lo, h_hi, pre_ckpt_d, post_ckpt_d, status = Running)` with `0 <= h_lo < h_hi < 2^64` (`pre_ckpt_d = INIT(orig_input_d)` iff `h_lo == 0`) | **NO** — rejected by `SRC-006` Run Decider (`Exploit E-8` blocked) |
| **Terminal Final Segment (`k = M - 1`)** | `dom("SHK_CONT_V1")` | `(prog_d, orig_input_d, mode, h_{M-1}, N, ckpt_{M-1}, outcome_d, status in {Halted, Rejected, Exhausted})` | **Only after `CONT_JOIN` reaches `h_lo == 0`!** |
| **Folded Terminal Run (`SRC-006` Decider Output)** | `dom("SHK_RUN")` (`v = 1`) | **`[prog_d, orig_input_d, outcome_d, n_steps = N]`** (`0 < N < 2^64`, `deps_d == EMPTY`) | **YES** — 1 `6,432 B` proof, `O(1)` verification |

#### 3. Binary Tree Folding (`CONT_JOIN` in `GP-2` + `GP-4`) & C6 `claim_d` Security
- **`CONT_JOIN` in `N` (`~250–350` node rows `ESTIMATE`):** Given left `SHK_CONT_V1(prog_d, orig_in, mode, h_0, h_1, c_0, c_1, Running)` and right `SHK_CONT_V1(prog_d, orig_in, mode, h_1, h_2, c_1, c_2, st_R)`, asserts equality on `(prog_d, orig_in, mode, h_1, c_1)`, enforces **`h_2 < 2^64`** (`SRC-004`) and **`SET.n_claims < 2^64`** (`Exploit E-1`), and emits `SHK_CONT_V1(prog_d, orig_in, mode, h_0, h_2, c_0, c_2, st_R)`, erasing `(h_1, c_1)` in `ceil(log2 M)` tree depth. When `h_0 == 0` and `st_R in {Halted, Rejected, Exhausted}`, `N` seals the claim to `SHK_RUN(prog_d, orig_in, outcome_d = c_2, n_steps = h_2)`.
- **C6 Outcome-Only & Loop/Replay Immunity:**
  1. **Bit-identical C6 `claim_d`:** `orig_input_d = H(SHK_IN, 1, val_d, fuel_lo, fuel_hi, stack_limit)` is never mutated, so `claim_d = H(SHK_RUN, 1, prog_d, orig_input_d, outcome_d)` is **identical** for `M = 1` and `M = 1,000`, inserting **one** 32-byte key into `settled` with zero state digests on-chain.
  2. **Deterministic loop / replay immunity:** Every checkpoint binds strictly increasing `h_hi` (`h_lo < h_hi`) and monotonic `consumed_work` / `remaining_work` (`counters_d`) plus `orig_input_d`, so visiting identical `(ctl, env, heap)` states in a loop produces distinct `(h_hi, ckpt_d)` pairs—preventing loop-skipping, cross-input splicing, and fuel rejuvenation.

---

### 3.4 Final Position on Q4 — Production zkVM Prior Art & Continuation CVE Synthesis

#### 1. Verified 10-System Comparison Matrix (All URLs Fetched `HTTP 200`)

| System | Per-Segment Limit | Architectural Run Limit | Memory Model & Address Width | Folding / Aggregation | On-Chain Verifier Public Inputs |
|---|---|---|---|---|---|
| **RISC Zero v3** | `2^13..2^24` cycles (`2^20` default) | `u64` cycles; unbounded `SystemSplit` | 32-bit (`4 GiB`), `1 KiB` pages, depth-22 **Poseidon2 Merkle** | `lift` -> binary `join` -> `resolve` -> `identity_p254` -> Groth16 | `(imageId, journalDigest)` + control roots; **`postState` zeroed on halt (`0` RAM/cycle data)**. |
| **SP1 v5 / v6** | v5: `2^22`; v6: `2^24` cycles, chip `<= 2^22` | `u64` (`4×16b` timestamp); unbounded | v5: 32b septic EC sum. v6: **48b (`256 TiB`)**; `v6.1.0` pure septic sum -> `dev` **septic intra-chunk + Poseidon2 Merkle inter-chunk** | `core` (Jagged PCS) -> `normalize` -> `compress` -> `shrink` -> BN254 | v5: `[vkey, pvDigest]`; v6: `[vkey, pvDigest, exitCode, vkRoot, nonce]`. **`0` shard/RAM data**. |
| **Jolt (RV64)** | Monolithic streaming (`2^24` default) | Arbitrary power-of-2 `T` (`< 2 GB` streaming) | 64-bit (`32 MiB` default heap); **Twist & Shout** sumcheck + **Dory** | Single `~50 KB` proof + BlindFold (recursion/EVM WIP) | Native `OutputSumcheck` over `(inputs, outputs, panic, MemoryLayout, T)`. |
| **Nexus (v1/2; v3)** | v1/2: IVC step; v3: Stwo `<= 2^28` | v1/2: unbounded IVC; v3: `< 2^28` rows | v1/2: 32b **Poseidon Merkle tries**; v3: 32b 2-pass + **LogUp** | v1/2: HyperNova+CycleFold -> Groth16; v3: single Stwo proof | v1/2: `1` scalar `H(vk,(F^Xi,x,y))` (hides `n`/RAM); v3: `(ELF, pub_I/O, ec)`. |
| **Valida** | `max_trace_height` (BabyBear) | Unbounded segments | `< (p-1)/2 ≈ 1 GiB`; **persistent cross-segment LogUp** | `MultiSegmentMachineProof` (`Vec<SegmentProof>`, no recursion) | Off-chain per-segment `(pc, fp, output, log_deg)`; **no L1 Solidity verifier**. |
| **Cairo / Stwo** | Stwo `M31`: `N <= 2^28` (`N*B^2 <= 2^30`) | Unbounded via `Cairo PIE` + **Bootloader** | `2^29` felts (`LOG_MEMORY_ADDRESS_BOUND = 29`); **write-once continuous RAM** | Recursive `stwo_cairo_verifier` tree under Bootloader | L1 `GpsStatementVerifier` sees top `(pc, ap, publicMemoryPages)`, hides inner PIEs. |
| **zkWasm** | Halo2 `2^18..2^22` rows | `u64` `eid`; unbounded slices | 32-bit Wasm (`64 KiB` pages); **KZG witness column equality** | `continuation-batcher` BN254 Halo2 recursion | `[kzgAcc, init/final image commit, pub I/O]`; hides slice count & mid-states. |
| **Miden v1.0-rc1** | `64 .. 2^29` cycles (`2^12` fragments) | `2^29`/run; scales via Tx/Batch/Block kernels | `[0, 2^32)` LogUp **Memory chiplet** + depth-64 **Advice SMT/MMR** | `TxKernel -> BatchKernel -> BlockKernel` + deferred precompiles | `ExecutionClaim` = 40 felts `(P, K, StackIn[16], StackOut[16])` (**`0` RAM/cycle data**). |
| **OpenVM v2.0** | `< 2^29` height, `15 GiB` RAM, `t < 2^29` | `u64` `instret`; unbounded segments | `addr_space_height=3`, `29`-bit ptr (`512 MiB`), **Poseidon2 Merkle (`CHUNK = 8`)** | `app -> leaf -> internal -> root -> static` BN254 Halo2 | `[kzgAcc[12], exeCommit, vmCommit, pubVals]`; **`Root` drops `final_root` & depth**. |
| **Ziren v2.0** | `clk < 2^26`, chip `< 2^22` rows | `< 2^26` shards (`> 10^14` insns) | 32-bit MIPS32r2; **KoalaBear septic EC multiset sum** | `prove_core` (WHIR+Jagged) -> `compress -> shrink -> wrap_bn254` | `[programVKey, sha256(pv)&mask253, VK_ROOT]` (`3` scalars, **`0` RAM/shard data**). |

**Primary Source URLs (Verified `HTTP 200`):**
- **RISC Zero:** `https://raw.githubusercontent.com/risc0/risc0/main/risc0/zkp/src/lib.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/circuit/rv32im/src/execute/platform.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/binfmt/src/lib.rs`, `https://raw.githubusercontent.com/risc0/risc0-ethereum/main/contracts/src/groth16/RiscZeroGroth16Verifier.sol`, `https://dev.risczero.com/api/recursion`.
- **SP1:** `https://docs.succinct.xyz/docs/sp1/hypercube/shards`, `https://docs.succinct.xyz/docs/sp1/hypercube/recursion`, `https://docs.succinct.xyz/assets/files/SP1_Turbo_Memory_Argument-b042ba18b58c4add20a8370f4802f077.pdf`, `https://raw.githubusercontent.com/succinctlabs/sp1/dev/crates/primitives/src/consts.rs`, `https://raw.githubusercontent.com/succinctlabs/sp1/dev/crates/hypercube/src/air/public_values.rs`, `https://raw.githubusercontent.com/succinctlabs/sp1-contracts/main/contracts/src/v6.0.0/SP1VerifierGroth16.sol`.
- **Jolt:** `https://a16zcrypto.com/posts/article/64-bit-proving-jolt/`, `https://eprint.iacr.org/2025/105`, `https://raw.githubusercontent.com/a16z/jolt/main/common/src/constants.rs`, `https://raw.githubusercontent.com/a16z/jolt/main/book/src/how/twist-shout.md`, `https://raw.githubusercontent.com/a16z/jolt/main/book/src/roadmap/streaming.md`.
- **Nexus:** `https://assets.nexus.xyz/whitepaper.pdf`, `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/specification/zkvm-spec-3.0.pdf`.
- **Valida:** `https://arxiv.org/html/2505.08114v1`, `https://raw.githubusercontent.com/lita-xyz/valida-vm/main/machine/src/persistence.rs`, `https://raw.githubusercontent.com/lita-xyz/valida-vm/main/machine/src/proof.rs`.
- **Cairo / Stwo:** `https://eprint.iacr.org/2024/278`, `https://raw.githubusercontent.com/starkware-libs/stwo/dev/crates/stwo/src/core/circle.rs`, `https://raw.githubusercontent.com/starkware-libs/stwo-cairo/main/crates/common/src/memory.rs`, `https://raw.githubusercontent.com/starkware-libs/starkex-contracts/master/evm-verifier/solidity/contracts/gps/GpsStatementVerifier.sol`.
- **zkWasm:** `https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/README.md`, `https://raw.githubusercontent.com/DelphinusLab/continuation-batcher/main/README.md`.
- **Miden:** `https://raw.githubusercontent.com/0xMiden/miden-vm/next/processor/src/execution_options.rs`, `https://raw.githubusercontent.com/0xMiden/miden-vm/next/docs/src/design/chiplets/memory.md`, `https://raw.githubusercontent.com/0xMiden/miden-vm/next/core/src/program/claim.rs`.
- **OpenVM:** `https://openvm.dev/whitepaper.pdf`, `https://docs.openvm.dev/specs/architecture/continuations/`, `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/arch/config.rs`, `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/system/memory/controller/mod.rs`, `https://raw.githubusercontent.com/openvm-org/openvm-solidity-sdk/main/src/v2.0-base/OpenVmHalo2Verifier.sol`.
- **Ziren & 2025–2026 Systems:** `https://eprint.iacr.org/2026/2330`, `https://docs.zkm.io/design/continuation.html`, `https://docs.zkm.io/design/memory-checking.html`, `https://eprint.iacr.org/2025/917` (Jagged PCS), `https://raw.githubusercontent.com/matter-labs/zksync-airbender/main/docs/subarguments_used.md` (Airbender), `https://eprint.iacr.org/2024/416` (Mangrove).

#### 2. Why Sherlock's Design Fits Both Theory and 2024–2026 Security Lessons
1. **Why tree-less offline memory is excluded in Sherlock M4:** Offline LogUp/septic memory requires Phase-2 challenge columns (blocked by `crates/mori-recursion/src/verifier_gadget.rs:16-17` `num_challenges <= 1`) and `O(|touched RAM|)` end-of-run finalization sweeps (violating `SRC-021` `O(1)` startup/shutdown—which is why even `succinctlabs/sp1` `dev` added `prev_merkle_root`/`merkle_root` across chunks for 48-bit RAM!).
2. **CVE defenses baked into Q2/Q3:**
   - *OpenVM `CVE-2026-62333` (unconstrained final memory commitment on public values):* Sherlock binds `outcome_d = vd(res)` directly inside the terminal step before `SRC-006` erases `post_d`.
   - *Jolt `PR #975` (forged initial memory state):* Sherlock enforces `is_genesis = is_zero(h_lo)` -> `committed_root == EMPTY_ROOT, len == 0, tail == empty` (`SRC-021`) and `SRC-004` (`h < 2^64`) so `is_genesis` can never re-fire.

---

### 3.5 Final Position on Q5 — Practical Run Length on This GB10 Host & Spec Tested Targets

#### 1. Measured Hardware Baseline & Corrected Throughput (`S = 16`, `k_leaf = 16`)
- **Host & Cgroup Caps (`prover.md:15`, `requirements.md:41`, `qa/sherlock/budgets.json:11-22`):** 20-core ARMv9 GB10 (`120.3 GiB` unified LPDDR5X); `48 GiB` single-process / `100 GiB` aggregate gate caps (`60 GiB` operational `sherlock-prover.slice`).
- **Measured timings (`qa/sherlock/runs.jsonl:617,618,648,649,709,711`):** Solo `k = 20` node = `59.62–64.95 s` (`61.07 s` med, `16.6–17.4 GB` RSS); `3 × 6t` concurrent `k = 20` nodes = `128.09–131.50 s` (`43.83 s` amortized/node, `47.73 GB` agg RSS); Run Decider = `67.12–77.88 s` (`70.35 s` med, `17.35 GB` RSS).
- **Two Single-Host Execution Modes (`S = 16`, `k_leaf = 16` leaf at `~6–8 s`, `< 1.5 GB` RSS, `ESTIMATE`):**
  1. **Low-RSS Pipelined Linear Mode (`~18–20 GB` RSS):** Leaf `i + 1` (`2t`) overlaps Node `i` (`18t`) -> **`~64–66 s` per 16-step leaf = `4.0 – 4.15 s / step`** (`~870–900` steps/h, `ESTIMATE`).
  2. **3-Worker `GP-4` Tree Mode (`~47.7–52 GB` RSS, using `architecture.md:75` 2-leaf `R` lift `U + U -> SEG`):** `L` leaves require `ceil(L / 2)` `R` lifts + `ceil(L / 2) - 1` `N` merges = **`L` (`L - 1`) `k = 20` proofs** + leaf batch (`~0.4 s/step`) -> **`~2.83 – 3.15 s / step`** (`~1,140–1,270` steps/h, `ESTIMATE`, contingent on 2-leaf `R` fitting `< 950,000` rows at `k = 20`).

| Wall-Clock Window | Pipelined Linear (`~19 GB` RSS, `ESTIMATE`) | 3-Worker `GP-4` Tree (`~50 GB` RSS, `ESTIMATE`) | `gp-bundle/1` Disk (`~644 B/step`, `ESTIMATE`) | Role |
|---|---|---|---|---|
| **`~23 min`** (`G-A` / single prog) | `~293 – 320` steps (`19–20` leaves) | `~410 – 450` steps (`26–28` leaves) | `~210 KB` | `fact(3)` (`293`) or `list123` (`303`) |
| **`2 hours`** (`G-C` budget) | **`~1,650 – 1,750` steps** (`~108` leaves) | **`~2,200 – 2,450` steps** (`~150` leaves) | `~1.1 – 1.6 MB` | Routine `G-C` gate (`fact(3)` + `list123` + `1,024`-step `P3`!) |
| **`3.5 – 4.7 hours`** | **`4,096` steps** (`256` leaves in `~4.6 h`) | **`4,096` steps** (`256` leaves in `~3.4 h`) | `~2.7 MB` | **Recommended `GP-001` (`A33`) endurance soak** |
| **`10 hours` (overnight)** | **`~8,500 – 8,800` steps** (`~540` leaves) | **`~11,400 – 12,500` steps** (`~750` leaves) | `~5.6 – 8.0 MB` | Overnight regression soak |
| **`24 hours` (1 day)** | **`~20,500 – 21,300` steps** (`~1,320` leaves) | **`~27,500 – 30,000` steps** (`~1,800` leaves) | `~13.6 – 19.3 MB` | Full-day single-host soak |

#### 2. Recommended Three-Tier Spec Qualification Targets (`SAS-001` / `SAS-012`)
1. **Routine `< 2 h` `G-C` Gate (`SRC-011`, `SRC-015`):** `fact(3)` (`293` steps) + `list123` (`303` steps) + unseen post-freeze `P3` of **`>= 1,024` owner steps (`>= 64` leaves at `S = 16`)** (`~1,620` combined steps -> `~112 min` serial or `~83 min` 3-worker tree, `< 120 min` `G-C` budget! `ESTIMATE`).
2. **`GP-001` (`A33`) Release Milestone Endurance Soak:** **`>= 4,096` owner steps across `>= 256` leaves** (`8` full binary tree levels, allocating `> 256` cells so live heap exceeds single-leaf carriers by `> 32×`, `~3.4–4.6 h` `ESTIMATE`), asserting `proof.len() == 6,432 B` equal to `entry 7;`.
3. **`SRC-023` (`A38`, Task `5.12`) Partition-Independence Replay Gate:** **`>= 64`-step allocating run** proved under `S = 16` (`4` leaves), `S = 1` (`64` leaves), and mixed (`12` leaves) (`80` total `k = 20` proofs, `~60–88 min` `ESTIMATE`), asserting byte-identical `[prog_d, input_d, outcome_d, n_steps]` and `claim_d`.

---

## What Was Not Checked

1. **No builds, tests, synthesis, or prover executions were run:** Per `BRIEF.md:67`, `cargo`, `lake`, `quint`, `MockProver`, and Halo2 benchmarks were not invoked; measured timings/RSS come strictly from committed `qa/sherlock/runs.jsonl` and `research/` logs, and all projected rows/timings are marked `ESTIMATE`.
2. **Exact row synthesis of `M3-Tail` (`W = 15` mux) and 2-leaf `R` (`U + U -> SEG`) in Halo2:** While E2 measured `1`-verifier node overhead (`823,466` rows at `k = 20`), the 2-leaf `R` circuit (`architecture.md:75`) embeds two leaf verifiers (`k = 16`) and has not yet been synthesized in `crates/mori-recursion` to verify its exact margin under `950,000` rows.
3. **Mechanization of `M3-Tail`, `CONT_JOIN`, and `SET.n_claims < 2^64` in `Aggregation.lean`:** `clean/fork/Clean/Sherlock/Aggregation.lean` currently proves `src004_height_bridge` for 1-chain/tree `SEG` heights; extending Lean theorems to `SET.n_claims` (`Exploit E-1`) and `SHK_CONT_V1` (`Q3`) remains open implementation work.
4. **Orthogonal open items (`D-2` `ty_d` normal form in `dissent.md`; live Midnight Preview/Mainnet `LedgerParameters` in `tasks.md` `6.8`):** Not modified or queried in this study.
