# Sherlock Modular Architecture — Area 3: Recursion and System

## 0. Scope, Notation, and Non-Negotiable Architecture Guardrails

This document specifies **Area 3: Recursion and System** (`research/sherlock-final-design-20261111/ARCH-BRIEF.md:21,48-68`, `research/sherlock-final-design-20261111/MODULARITY.md:1-12`):
1. **Module `M10: NODE INTEGRATION`** (`ChipNode` / 2-leaf `RunLift` `R`, binary polymorphic `Node` `N` covering segment merge `M`, seal `C`, `GP-2` dependency resolution, `GP-3` registry foreign lift `F` + adapter `A_c`, `GP-4` diamond-safe `SET` union `U_set`, and base-resolution closing proof `C_close` across swappable leaf profiles `Profile A` and `Profile B`).
2. **Module `M11: DECIDER`** (`RunDecider` `SHKRUND1`, `ClaimDecider`, outcome-only C6 statement `[prog_d, input_d, outcome_d, n_steps]`, exact `Pow2RangeChip` `table_rows == 1,032,467` proof, and Midnight Ledger `9.1.0.1` / `zkir-3.2.0` verification boundary).
3. **Module `M9: CONTINUATION`** (Tier-1 streaming/parallel `3×6t` segment-tree scheduler with `O(log L)` RAM accumulators; Mode-1 retained-store `SEG` checkpoint and resume via `M: SEG + SEG -> SEG`; Mode-2 open-research `sherlock-ckpt/1` + lazy `UNFOLD` hook).
4. **Module `M12: HARNESS`** (whole-leaf and whole-run differential integration and mutant harness against `sherlock-k-a/1`).
5. **System-Wide Artifacts**: complete 10-module / 13-unit dependency DAG and wave build order across Areas 1–3, the Canonical Module Kit Template (`research/skeleton-coding-20261108/pilot-protocol.md:1-227`), the post-prototype OpenSpec 1.17.0 change blueprint (`ARCH-BRIEF.md:62-68`), open Area 3 decisions, conflicts with existing specs/studies, and unchecked items.

### Citation and Measurement Conventions
- Repository paths are cited relative to the workspace root with exact line numbers (`path:line`).
- `MP:` denotes pinned `midnight-proofs-0.8.3/src/` (`Cargo.lock:2137-2138`); `MC:` denotes pinned `midnight-circuits-7.3.0/src/`; `ZS:` denotes pinned `midnight-zk-stdlib-2.4.0/src/`; `L9:` denotes Midnight Ledger `9.1.0.1` (`a2b30128`, `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:103`).
- Every unmeasured row count, proof size, memory figure, or wall-clock figure is marked `ESTIMATE`. Every per-step throughput `ESTIMATE` states its explicit step-per-leaf occupancy (`S_eff` enabled owner steps out of `S` keyed slots, `research/sherlock-runlength-consensus-20261111/AUDIT.md:107-109`).

### Preservation of `GP-1` through `GP-4` and Audit Guardrails
`GP-1` through `GP-4` (`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:7-53`, `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:56-59`) are preserved verbatim without narrowing:
1. **No occurrence cap on `SET` claim union (`GP-4`, `research/sherlock-runlength-consensus-20261111/AUDIT.md:19-22`):** `SET + SET -> SET` (`U_set`) **MUST NOT** carry or range-check an occurrence counter `n_claims < 2^64` (superseding `research/sherlock-gp-architecture-20261110/architecture.md:100,103`). A depth-64 diamond claim DAG (`GP-4`) contains ~128 proofs and ~65 distinct claims while legitimately inducing $2^{64}$ path occurrences of a shared root claim; an occurrence range check rejects honest diamond DAGs, whereas an unchecked occurrence counter wraps modulo $r$ at depth 255. `SET` commits strictly to `set_root` (content-addressed Poseidon DAG digest), and settlement linearity is enforced per distinct `claim_d` nullifier in `openClaim` (`architecture.md:367,391`).
2. **Mode-1 stop/resume checkpoints compose via `SEG + SEG -> SEG`, NEVER `GP-2` (`GP-1`, `GP-2`, `AUDIT.md:39-42`):** `GP-002` (`requirements.md:57`) explicitly forbids accepting an unsealed segment statement or state digest as a cross-run claim (`GP-001` at `:56` likewise forbids accepting an intermediate proof as a run or claim). Mode-1 cross-session checkpointing persists a proven `SEG` prefix plus the retained segment store and resumes via `M: SEG + SEG -> SEG` segment recursion (`0` loader leaves). `GP-2` consumes **sealed terminal `SET` claims** `claim_d = Poseidon("SHK_RUN", 1, prog_d, input_d, outcome_d)` (`SRC-007`, `requirements.md:32`; `openspec/changes/sherlock-vm-consensus-20261107/dissent.md:209-212` `D-3`).
3. **Mode-2 portable value-only checkpointing (`sherlock-ckpt/1` + lazy `UNFOLD`) stays isolated as an open-research hook (`AUDIT.md:33-38`):** Unresolved representation and step-count blockers (`m > 8` contiguous `FIELDS`/`ENV`/`SLOTS` slice authentication inside a 15-row `WindowCfg`, zero-loader-step `n_steps` accounting, and `Profile B` epoch handoff) are quarantined behind a non-M4 extension hook rather than baked into M4 constraints.
4. **Address-free `shape_d` and canonical `outcome_d` (`SRC-022`, `SRC-023`, `GP-004`):** Physical execution commitments (`pre_d`, `post_d`, `com_pre`, `com_post`, `mem_root`, `len`, `fp`, `tp`) never appear in `outcome_d`, `claim_d`, `shape_d`, or any decider public instance (`requirements.md:31,47-48,59`; `dissent.md:218-222` `D-6`).

---

## 1. Module `M10: NODE INTEGRATION` (`RunLift` `R`, Polymorphic `Node` `N`, `ForeignLift` `F`, `ClosingProof` `C_close`)

### 1.1 Purpose
`NODE INTEGRATION` lifts fixed-key `ChipLeaf` (`U`) execution proofs and registry-admitted foreign proofs into KZG-accumulated recursion certificates (`SEG` for open execution segments and `SET` for sealed run/foreign claim DAGs) at constant circuit domain `k = 20`. It isolates all `VerifierGadget` in-circuit PLONK verification, named fixed-base accumulator routing, segment linkage, `SRC-023` physical-state erasure at seal, and `GP-2`/`GP-3`/`GP-4` composition rules from both leaf internals and the on-chain decider.

### 1.2 Frozen Interface

#### Proof-System Constraints Inherited from `VerifierGadget` (`MC:verifier/verifier_gadget.rs:285-365,604-610`)
Every inner proof verified in circuit (`ChipLeaf` `U`, adapter `A_c`, `RunLift` `R`, `ForeignLift` `F`, `Node` `N`, and `ClosingProof` `C_close`) uses a Poseidon sponge transcript (`crates/mori-recursion/src/inner.rs:102-104`) and MUST obey three `midnight-circuits 7.3.0` invariants:
1. **Single challenge phase (`:310`):** `assert_eq!(cs.phases().count(), 1)`; `Expression::Challenge(_)` panics in `MC:verifier/expressions/mod.rs:49`.
2. **Exact instance column split (`:286-288`):** `cs.num_instance_columns() == assigned_committed_instances.len() + assigned_instances.len()`. Transcript absorption (`:296-306`) hashes `vk_transcript_repr`, then each committed-instance point $C_i \in \mathbb{G}_1$, then for each ordinary instance column its fixed slice length `(instance.len() as u64)` followed by its scalar elements.
3. **Adjacent rotations only (`:604-610`):** Every gate and lookup expression in any verified child MUST restrict rotations to `{-1, 0, +1}`.

#### Leaf-to-Node Boundary Contract (`ChipLeaf` `U` -> `ChipNode` / `RunLift` `R`)
Area 2 exposes two swappable leaf boundary profiles behind one `LeafBoundaryContract` trait (`research/sherlock-lookup-cache-20261111/CONSENSUS.md:142-222`). Both share the `Instance(1)` common header and differ only in `Instance(0)` (committed polynomial) and boundary scalars:

| Leaf Lane / Input | `Profile A` (`DensePageBackend` + `PoseidonBoundary`) | `Profile B` (`ZmLogUpBackend` + `HornerBoundary`) | Bit Range | Range-Check Owner |
|---|---|---|---|---|
| `Instance(0)` (`committed_instances[0]`) | Constant `C::identity()` (`crates/mori-recursion/src/inner.rs:181,297`), 1-pass prover | Witnessed KZG commitment $C_i = \text{Com}(P_i, r_{\text{blind}}) \in \mathbb{G}_1$ (`MP:plonk/prover.rs:32-49`), 2-pass host prover over 1-phase CS (`q_com_zero * Instance(0) == 0`) | On-curve $\mathbb{G}_1$ subgroup | `M10` (`Profile A`: constant identity; `Profile B`: **MUST** witness via subgroup-checked `AssignmentInstructions::assign` at `MC:ecc/foreign/weierstrass_chip.rs:343-360`, **NEVER** `assign_without_subgroup_check` at `:936-952`) |
| `binding` (`inst[0]`) | `F` (`zk-stdlib` `Instance(1)` row-0 prefix) | Same | Full `Fq` | Transcript-bound by `VerifierGadget` (`MC:verifier/verifier_gadget.rs:305`) |
| `prog_d` (`inst[1]`) | `Poseidon("SHK_PROG", prog_root, vm_ver, inv_id)` (`SVM-005`) | Same | Full `Fq` | Leaf `HASH` / `CORE` (`M1`/`M6`); equality-linked in `M10` |
| `input_d` (`inst[2]`) | `Poseidon("SHK_IN", val_d, fuel_lo, fuel_hi, stack_limit)` (`SRC-021`) | Same | Full `Fq` (`fuel_*`, `stack_limit < 2^64` checked at `h_pre == 0` in Leaf) | Leaf `Init` gate (`h_pre == 0`, `SRC-021`); equality-linked in `M10` |
| `rules_d` (`inst[3]`) | `Poseidon("SHK_RULES", rid_0..rid_{S-1})` (`SRC-001`) | Same | Full `Fq` | Leaf `CORE` (`M1`); absorbed into `VerifierGadget` transcript (`M10` checks slot count via `S_MAX`) |
| `h_pre` (`inst[4]`) | Segment start step index | Same | `[0, 2^64 - 1]` | **Both** Leaf (`M1`/`M3`) and `M10` (`constrain_heights` via `assigned_to_le_bytes(..., Some(8))`, `SRC-004`) |
| `steps` (`inst[5]`) | Count of enabled owner steps in leaf | Same | `[1, S]` (`S <= S_MAX = 16`) | **Both** Leaf (`M1`) and `M10` (`steps != 0`, `steps <= S_MAX`, `h_pre + steps < 2^64`, `SRC-004`) |
| `deps_d_pre`, `deps_d_post` (`inst[6..7]`) | Running Poseidon chain of `GP-2` consumed claims (`0` at `h_pre == 0`) | Same | Full `Fq` | Leaf `VerifySherlockClaim` rule; `M10` checks `L.deps_d_post == R.deps_d_pre` and matches `SET` at seal |
| `term_tag`, `outcome_d` (`inst[8..9]`) | `term_tag in {0=Running, 1=Halted, 2=Rejected, 3=Exhausted}`; `outcome_d` (`0` if `Running`, `SRC-022` digest if terminal) | Same | `term_tag in {0..3}`, `outcome_d in Fq` | Leaf `CORE` + `VALUE DIGEST` (`M1`+`M7`); `M10` enforces `L.term_tag == 0` on non-final segments and `term_tag in {1,2,3}` at seal `C` |
| Boundary payload (`inst[10..]`) | `[pre_d, post_d]` (`2` scalars; total `12` scalars) | `[com_pre, com_post, gamma_1, gamma_2, alpha_1, alpha_2, delta_sum_1, delta_sum_2]` (`8` scalars; total `18` scalars) | Full `Fq` | Leaf `BOUNDARY COMMITMENT` (`M8a` or `M8b`); `M10` enforces linkage (`L.post == R.pre`), challenge equality (`gamma`, `alpha`), and additive LogUp sum (`sum_L + sum_R`) |

#### Inter-Node Statements and Core Key Architecture (`architecture.md:74-108,427-437`)
To support both the M4 single-run milestone and full `GP-1..GP-4` without exceeding `k = 20`, `M10` defines four node-class relations (`bls12_381` + Poseidon inner transcript, `k = 20`, zero committed instances on node-class proofs themselves):

1. **`RunLift` `R` (`MAGIC = b"SHKLFT01"`, replaces 1-leaf `ChipNode` `SHKIVC06` when 2-leaf lift fits `k = 20`, with 1-leaf mode for odd tails or fallback):**
   - Verifies 1 or 2 `ChipLeaf` (`U`) proofs (`slot1_en in {0, 1}`) under baked constant `leaf_vk` (`MC:verifier/verifier_gadget.rs:116`), resolving `U`'s fixed bases once across both slots (`crates/mori-recursion/src/inner.rs:273-315`).
   - Emits `SEG` statement + `shape_d` + collapsed `(acc_lhs, acc_rhs)` (`ACC_LEN` scalars over `{U_bases}` -> collapsed to pure homogeneous accumulator with **zero** unresolved fixed-base names!). Because `R` resolves `U`'s constant bases inside `R` (`assign_vk_as_constants`), `R`'s output accumulator carries **only** `[G_0]` (`"com"` / homogeneous KZG points), adding zero leaf fixed-base names to `N`!
2. **`ForeignLift` `F` (`MAGIC = b"SHKFOR01"`, `GP-3` hook):**
   - Verifies 1 adapter-class proof `A_c` under a **witnessed** `MidnightVK` (`assign_vk`, `architecture.md:76-79,218-220`), resolves `A_c`'s ~40 fixed bases in circuit, folds `A_c`'s own accumulator together with `A_c`'s published inner foreign accumulator (`architecture.md:326-328`), verifies Poseidon Merkle membership of both `vk_id(A_c)` and `vk_id_inner` in `reg_root` (`architecture.md:300-302`), and emits a 1-claim `SET` statement (`claim_d = Poseidon("SHK_CLAIM", KIND_FOREIGN, vk_id_inner, adapter_id, out_d)`).
3. **Polymorphic Binary `Node` `N` (`MAGIC = b"SHKNOD01"`, self-recursive 2-slot DAG/tree engine):**
   - Two child verifier slots (`slot_j_en in {0, 1}`, `(slot_0_en, slot_1_en) != (0, 0)`). Each enabled slot selects `key_sel_j in {KEY_N, KEY_R, KEY_F}` via one-hot booleans summing to `slot_j_en` (`architecture.md:431-435`), binding the child's `vk_transcript_repr` against public inputs `[n_repr, r_repr, f_repr]` and routing deferred fixed-base scalars into the fixed finite union of `{N, R, F}` base names via `AssignedAccumulator::assign` (`MC:verifier/accumulator.rs:280`).
   - Public instances of `N`: `[binding, n_repr, r_repr, f_repr, reg_root, stmt_tag, p_0..p_K, shape_d, acc_0..acc_{A-1}]` (`stmt_tag in {TAG_SEG = 1, TAG_SET = 2}`).
   - Four transition modes selected by `(mode_sel)` booleans:
     - **`M: SEG + SEG -> SEG` (Segment Merge):** Checks `L.leaf_id == R.leaf_id`, `L.prog_d == R.prog_d`, `L.input_d == R.input_d`, `L.post_boundary == R.pre_boundary`, `L.h_hi == R.h_lo`, `L.term_tag == 0`, `L.deps_d_post == R.deps_d_pre`, `R.h_hi < 2^64` (via `assigned_to_le_bytes(..., Some(8))`). Under `Profile B`, checks `(L.gamma, L.alpha) == (R.gamma, R.alpha)` and sets `sum = L.sum + R.sum`.
     - **`C: SEG -> SET` (Unary Seal, `deps_d == 0`):** Requires `slot_0 = SEG`, `slot_1 = disabled`, `SEG.h_lo == 0` (which invokes Leaf's `h_pre == 0` `Init(prog_d, input_d)` guarantee per `SRC-021`), `SEG.term_tag in {1, 2, 3}` (refuses `0 = Running` per `SRC-006`), `SEG.deps_d_pre == 0`, `SEG.deps_d_post == 0`, and Merkle membership of `SEG.leaf_id` in `reg_root` (`GP-3`, `architecture.md:136-139`). Under `Profile B`, additionally requires `SEG.sum_1 == 0`, `SEG.sum_2 == 0`, and Fiat-Shamir binding `(gamma_1..2, alpha_1..2) == PoseidonFS(SEG.com_root_tree)` over the committed polynomial Merkle root accumulated along `M`. Emits `set_root = Poseidon("SHK_SET_LEAF", claim_d)` where `claim_d = Poseidon("SHK_RUN", 1, prog_d, input_d, outcome_d)` (`SRC-007`, `dissent.md:209-212` `D-3`) and auxiliary single-run header `(prog_d, input_d, outcome_d, n_steps = SEG.h_hi)` (erasing all `pre_d`, `post_d`, `com_*`, `sum_*` per `SRC-023` and C6!).
     - **`Resolve: SEG + SET -> SET` (`GP-2` Seal with Consumed Claims):** Same as `C`, except `slot_1` is an enabled `SET` proof and `SEG.deps_d_pre == 0`, `SEG.deps_d_post == slot_1.set_root` (`architecture.md:84,132-134`), discharging every `VerifySherlockClaim` assumption made by the run! Emits `set_root = Poseidon("SHK_SET_DEP", claim_d, slot_1.set_root)`.
     - **`U_set: SET + SET -> SET` (`GP-4` Claim-DAG Join):** Combines any two proven `SET` subgraphs (including diamond-shared ancestors!) into `set_root = Poseidon("SHK_SET_JOIN", slot_0.set_root, slot_1.set_root)` (`architecture.md:85,348-350`), with **ZERO `n_claims` counter and ZERO `< 2^64` range check** (`AUDIT.md:19-22`).
4. **`ClosingProof` `C_close` (`MAGIC = b"SHKCLS01"`, node-class base resolver before `DECIDER`):**
   - Verifies 1 root `N` proof with `stmt_tag == TAG_SET`, pins `[n_repr, r_repr, f_repr]` as circuit constants (`C_close` is keyed after `R`, `F`, `N`), resolves all three fixed-base name sets `{N, R, F}` into homogeneous curve points (`architecture.md:86-87,220-224`), folds `C_close`'s verifier accumulator with `N`'s carried accumulator, and publishes `[binding, reg_root, set_root, shape_d, single_run_flag, prog_d, input_d, outcome_d, n_steps, acc_collapsed_homog]` to `DECIDER`.
   - *M4 Fast-Path Specialization:* While M4 exercises `GP-1` single-run (`ChipNode` / `RunLift` + self-recursive `SEG` `Node` without `F`), `Node` carries only `{self}` fixed-base names (`crates/mori-recursion/src/ivc.rs:548-562`), so `RunDecider` resolves `{Node}` directly in one stage without `C_close` (`run_decider.rs:265-275`). Both paths share the exact `DECIDER` statement contract.

#### Range-Check Ownership Rule (`Some(8)` `Pow2Range` Discipline)
Every integer bound checked inside `R`, `F`, `N`, or `C_close` (`h_pre`, `steps`, `h_hi`, `S_MAX - steps`) MUST decompose via `native_gadget.assigned_to_le_bytes(layouter, &x, Some(8))` (or `Some(1)` for `u8` tags), querying only `Pow2Range` tag `bit_len = 8` (`256` table rows, already loaded by `VerifierGadget`). No node-class or decider-class circuit may call `assert_lower_than_fixed` or arbitrary-bit `assigned_to_le_chunks` that insert new `queried_tags` (`MC:field/decomposition/pow2range.rs:215-236`).

### 1.3 Invariants

**Guaranteed by `M10`:**
1. **Sound segment linkage (`INV-NODE-LINK`):** Every `SEG` emitted by `R` or `N(M)` covers a contiguous half-open step interval `[h_lo, h_hi)` with `0 <= h_lo < h_hi < 2^64`, identical `(leaf_id, prog_d, input_d)`, unbroken boundary state `post_i == pre_{i+1}`, unbroken `deps_d` chain, and `term_tag == 0` on every non-final child.
2. **Physical state erasure at seal (`INV-NODE-SEAL`, `SRC-006`, `SRC-023`):** Transition `C` / `Resolve` (`SEG -> SET`) enforces `h_lo == 0` and `term_tag in {1, 2, 3}`, discharges `deps_d_post` (`== 0` or `== slot_1.set_root`), verifies `Profile B` LogUp balance (`sum_1 == 0, sum_2 == 0` and Fiat-Shamir challenge derivation) when active, and strips `pre_d`, `post_d`, `com_*`, and `sum_*` from the emitted `SET` payload.
3. **Subgroup-safe committed-instance intake (`INV-NODE-SUBGROUP`):** Under `Profile B`, every witnessed `Instance(0)` commitment $C_i \in \mathbb{G}_1$ is checked on-curve and in the prime-order subgroup $r$ (`[r] C_i = \mathcal{O}$ via endomorphism in `AssignmentInstructions::assign`, `MC:ecc/foreign/weierstrass_chip.rs:343-360`), blocking BLS12-381 $\mathbb{G}_1$ cofactor-$11^2$ torsion-point grinding.
4. **Canonical disabled-slot neutrality (`INV-NODE-NEUTRAL`, `SRC-005`):** Any disabled child slot (`R` 1-leaf tail or `N` unary seal `C`) substitutes a fixed canonical fully-assigned neutral accumulator encoding *before* the collapse challenge is squeezed (`crates/mori-recursion/src/ivc.rs:383-401,538-546`; `architecture.md:343-346`), preventing unauthenticated carry injection.
5. **Sound named-key routing (`INV-NODE-NAMES`, `architecture.md:427-437`):** In `N`, one-hot key selectors `(s_N, s_R, s_F)` summing to `slot_j_en` bind the child's `vk_transcript_repr`, permitted output `stmt_tag` (`R -> SEG`, `F -> SET`, `N -> SEG|SET`), and fixed-base scalar routing `coeff[name, k] = s_key * raw_coeff[k]` under one canonical coordinate order, while forwarding `[n_repr, r_repr, f_repr, reg_root]` unchanged from children to parent.
6. **Diamond-safe `SET` union (`INV-NODE-DAG`, `GP-4`, `AUDIT.md:19-22`):** `U_set: SET + SET -> SET` is total over any pair of valid `SET` proofs at arbitrary DAG depth without arithmetic overflow, wrap, or occurrence cap.
7. **Address-free `shape_d` (`INV-NODE-SHAPE`, `GP-004`):** `shape_d = Poseidon("SHK_SHAPE", rel_tag, slot_bits, key_sels, child0_shape_d, child1_shape_d, local_meta_d)` hashes only relation tags, slot bits, heights/claims, and child `shape_d` values — never `pre_d`, `post_d`, `com_pre`, or `com_post`.

**Assumed from Upstream Modules:**
1. **From Area 1 (`B0..M3`, `M6`) + Area 2 (`M4..M8`):** `ChipLeaf` `U` uses `phases().count() == 1`, rotations in `{-1, 0, +1}`, enforces `Init(prog_d, input_d)` when `h_pre == 0` (`SRC-021`), enforces `rid_j = 0` and `post == pre` on disabled leaf slots (`SRC-001`, `SRC-014`), and computes canonical `outcome_d` (`sherlock-value/1`, `SRC-022`) on terminal steps.
2. **From Cryptographic Stack (`AggregationAxioms.lean:1-188`):** Collision resistance of Poseidon over `Fq`, KZG binding under `bls_midnight_2p20`, and `VerifierGadget::prepare` + `Accumulator::collapse` knowledge soundness.

### 1.4 Row Budget (`ESTIMATE`)

Measured baselines: E8 1-leaf `ChipNode` over the 4-lookup spike leaf (`43` advice, `15` fixed, `18` eq, `4` lookups) = **`823,466` rows** (`k = 20`, gate `<= 950,000`, `openspec/changes/sherlock-vm-consensus-20261107/design.md:105,129-131`); Node linear fit (`design.md:134`): `ESTIMATE 570,957 + 7,298*adv + 8,189*eq + 6,004*fix + ~22,600*lookup`; Node growth per leaf `k` bit = **`+4` rows/bit** (`741,708` at `k_leaf = 13` to `741,736` at `k_leaf = 20`, measured in `research/sherlock-recursion-design-review-20261110/experiments/e2-e4.md:25-35`); binary `self + self` probe = **`730,243` rows** (`architecture.md:206`). Usable domain at `k = 20` is `2^20 - 6 = 1,048,570` rows (`MP:dev/cost_model.rs:370-398`).

With Area 1's `TaggedTableConfig` (`4 -> 3` lookups, `-1` fixed column, saving `~28.6k` per single-leaf verifier and `~51.2k` across a shared-VK 2-leaf verifier `ESTIMATE`):

| Relation | Profile / Mode | Circuit Rows (`ESTIMATE` unless noted) | `Pow2Range` Table Rows | Domain `k` | Headroom to `1,048,570` |
|---|---|---|---|---|---|
| 1-Leaf `ChipNode` (`SHKIVC06`, fallback) | `Profile A` (`C::identity()`) | **`794,416 – 809,903` `ESTIMATE`** (vs `823,466` measured E8) | `1,032,467` | `k = 20` | `+238,667 .. +254,154` |
| 1-Leaf `ChipNode` (`SHKIVC06`, fallback) | `Profile B` (1× subgroup-checked $C_i$) | **`803,500 – 830,100` `ESTIMATE`** (`+9.1k..+20.2k` for $\mathbb{G}_1$ subgroup check) | `1,032,467` | `k = 20` | `+218,470 .. +245,070` |
| **2-Leaf `RunLift` `R: U + U -> SEG`** | **`Profile A` (shared VK + shared base MSM)** | **`994,820 – 1,022,000` `ESTIMATE`** (`< 1,048,570`; halves leaf-lift Node count!) | `1,032,467` | **`k = 20`** | **`+26,570 .. +53,750`** |
| 2-Leaf `RunLift` `R: U + U -> SEG` | `Profile B` (2× subgroup-checked $C_0, C_1$) | **`1,013,000 – 1,062,400` `ESTIMATE`** (straddles `1,048,570`; risks `k = 21`, see §8 D-A3-1) | `1,032,467` | `k = 20 / 21` | `-13,830 .. +35,570` |
| Polymorphic Binary `Node` `N` (`M`, `C`, `Resolve`, `U_set`) | Both (`Profile B` adds `+8` scalars & 4 Poseidon perms at `C`) | **`760,000 – 855,000` `ESTIMATE`** (`730,243` measured self+self + `30k–125k` per-slot collapse, `{N,R,F}` mux & Poseidon rules) | `1,032,467` | `k = 20` | `+193,570 .. +288,570` |
| `ForeignLift` `F` (`GP-3` witnessed `A_c` VK + 2 Merkle paths) | `Profile A` / `Profile B` (independent) | **`550,000 – 750,000` `ESTIMATE`** (`architecture.md:218-220`) | `1,032,467` | `k = 20` | `+298,570 .. +498,570` |
| `ClosingProof` `C_close` (1 `N` proof + `{N,R,F}` base resolution) | `GP-2..GP-4` full key set (`0` rows if M4 single-key fast-path) | **`700,000 – 950,000` `ESTIMATE`** (`architecture.md:220-221`) | `1,032,467` | `k = 20` | `+98,570 .. +348,570` |

**Throughput & Occupancy Disclosure (`ESTIMATE`, `AUDIT.md:107-109`):** One `k = 20` node-class proof takes `61–67 s` solo at `20` threads (`16.6–17.6 GB` RSS, measured `architecture.md:204`) and `~95–115 s` `ESTIMATE` per worker under `3×6t` concurrent scheduling (`1.69–1.74×` contention factor, `architecture.md:228-229`). For a run of $L$ leaves with 2-leaf `RunLift` `R` (`\lceil L/2 \rceil` `R` proofs + `\lceil L/2 \rceil` `N` merges/seal = `L` node-class proofs total vs `2L` with 1-leaf lift + binary merge, or `L` sequential steps in a linear chain):
- At **full leaf occupancy `S_eff = S = 16` steps/leaf** (`Profile A`, `3×6t` pool delivering `~1` node-class proof every `~35 s` `ESTIMATE`): amortized node wall-clock is **`~2.2 s/step` `ESTIMATE`** (`35 s / 16 steps`), proving a 303-step `list123` run (`19` leaves -> `10` `R` + `10` `N` = `20` node-class proofs) in **`~12–15 min` `ESTIMATE`** (well inside the `2 h` `SRC-011` cap).
- At **adversarial minimum occupancy `S_eff = 1` step/leaf** (`S = 16` keyed, early leaf close on every step): amortized node wall-clock scales `16×` to **`~35 s/step` `ESTIMATE`**.
- Post-M4 upgrade to `S = 64` at full occupancy `S_eff = 64` cuts node wall-clock `4×` further to **`~0.55 s/step` `ESTIMATE`**.

### 1.5 Verification

1. **Quint Specification (`models/sherlock/quint/node_pcd_dag.qnt`, extending `models/sherlock/quint/aggregation.qnt`):**
   - Replaces single-program linear/3-slot invariants `I8` ("program fixity") and `I9` ("no certificate folded twice", `architecture.md:369-371`) at the `SET` layer while keeping strict linear step coverage `I3` and unique genesis `I6` at the `SEG` layer.
   - Models all 6 node transitions (`R_1`, `R_2`, `F`, `N_merge`, `N_seal`, `N_resolve`, `N_join`, `C_close`) under both `Profile A` and `Profile B`, checking: (a) `SegCoversExactInterval`, (b) `SealRequiresGenesisAndTerminal`, (c) `SealErasesPhysicalAddresses` (`SRC-023`), (d) `DepsDischargedIffVerifiedSet` (`GP-2`), (e) `ForeignClaimRequiresDualRegistryMembership` (`GP-3`), (f) `DiamondDagDepth64NoOverflow` (`GP-4`, explicitly testing a symbolic depth-64 diamond DAG with $2^{64}$ path occurrences), and (g) `LogUpEpochZeroSumSoundness` (`Profile B`).
2. **`MockProver` Positive Tests (`crates/mori-recursion/tests/node_integration_mock.rs`):**
   - `pos_r_one_and_two_leaf_profile_a` and `pos_r_one_and_two_leaf_profile_b` (synthetic mock `U` relation matching `ChipLeaf`'s exact column/lookup arch at `k = 13`, verifying `table_rows == 1,032,467` and `circuit_rows < 1,048,570`).
   - `pos_n_seg_merge_associative` (`(L1 + L2) + (L3 + L4)` vs `((L1 + L2) + L3) + L4` yielding identical `claim_d`, distinct `shape_d`).
   - `pos_n_seal_all_three_outcomes` (`Halted(v)`, `Rejected(code)`, `Exhausted(kind)`).
   - `pos_gp2_resolve_consumed_claim` (Program B consuming sealed `SET` of Program A).
   - `pos_gp3_foreign_adapter_lift` (mock Compact adapter proof + inner VK registry check).
   - `pos_gp4_diamond_dag_reuse` (shared `SET` proof A fed into both B and C, then joined `U_set(B, C)`).
3. **Named Negative Mutants (fresh malicious witnesses per `SAS-003` and `SAS-004`, each paired with a weakened-constraint control):**
   - `mut_height_wrap_2p64`: `h_pre = 2^64 - 1, steps = 1` -> unsatisfiable in `constrain_heights` (`SRC-004`).
   - `mut_steps_zero_or_gt_smax`: `steps = 0` and `steps = S_MAX + 1` -> unsatisfiable (`SRC-004`, `SRC-014`).
   - `mut_broken_seg_boundary`: `L.post_d != R.pre_d` (or `L.com_post != R.com_pre` in `Profile B`) -> unsatisfiable in `N(M)`.
   - `mut_seal_running_or_nonzero_hlo`: `N(C)` on `SEG` with `term_tag == 0` (`Running`) or `h_lo == 1` -> unsatisfiable (`SRC-006`, `SRC-021`).
   - `mut_seal_undischarged_deps`: `N(C)` unary seal when `SEG.deps_d_post != 0`, or `N(Resolve)` with `SEG.deps_d_post != slot_1.set_root` -> unsatisfiable (`GP-002`).
   - `mut_profile_b_torsion_point`: `Profile B` `Instance(0)` set to $C_i + T_{11}$ where $11 \cdot T_{11} = \mathcal{O}$ in $\mathbb{G}_1(\mathbb{F}_p) \setminus \mathbb{G}_1[r]$ -> rejected by `AssignmentInstructions::assign` subgroup check (`INV-NODE-SUBGROUP`).
   - `mut_profile_b_nonzero_logup_sum_or_forged_fs`: `N(C)` with `sum_1 != 0` or `(gamma, alpha) != PoseidonFS(com_root)` -> unsatisfiable.
   - `mut_disabled_slot_noncanonical_carry`: disabled slot in `R` or `N` carrying non-neutral accumulator scalars -> unsatisfiable (`SRC-005`).
   - `mut_named_key_cross_slot_swap`: child proved under `KEY_F` routed into `KEY_R` coefficients or forged `stmt_tag = TAG_SEG` -> unsatisfiable (`INV-NODE-NAMES`).
   - `mut_gp3_unregistered_inner_vk`: valid adapter proof `A_c` in `reg_root` whose `vk_id_inner` is absent from `reg_root` -> unsatisfiable (`architecture.md:300-302`).

### 1.6 Lean Obligations (`ARCH-BRIEF.md:52-60`)

Zero `sorry` and zero undeclared `axiom` declarations (`#print axioms` checked by `scripts/qa/check-lean-aggregation.sh`, relying solely on typeclass parameters in `clean/fork/Clean/Sherlock/AggregationAxioms.lean:1-188`):

| Obligation | Plain-English Theorem Statement | Lean File Extended / Created | Axioms in `AggregationAxioms.lean` Relied On | Module Contract Discharged |
|---|---|---|---|---|
| **L-NODE-1: 2-Leaf `RunLift` & Binary `SEG` Soundness** | Every `SegCert` built from 1-leaf `lift1`, 2-leaf `lift2` (`R: U + U -> SEG`), and binary `mergeSeg` (`N: SEG + SEG -> SEG`) with valid collapsed accumulators implies a contiguous deterministic `Step` trace of exact length `h_hi - h_lo < 2^64` from decoded `pre` to decoded `post`, independent of binary bracketing (`same_leaves_same_seg`). | Extends `clean/fork/Clean/Sherlock/Aggregation.lean` (replaces 3-slot `NodeCert.node L (Option Leaf) R` at `:74-101` with `RunLiftCert` + binary `SegCert` and updates C6 `Seal` to hide `fin_d` at `:1085-1094`). | `Premises.digest_inj`, `Premises.progDigest_inj`, `Premises.leaf_sound`, `AccAlgebra.genesis_collapse_both`, `AccAlgebra.non_genesis_keeps_both` (`AggregationAxioms.lean:45-175`). | `INV-NODE-LINK`, `INV-NODE-NEUTRAL`, `SRC-004`, `SRC-005`, `SRC-017`. |
| **L-NODE-2 (Obligation 3): Whole-Leaf & Whole-Tree Modular Refinement** | If Area 1 (`CORE`, `DEC`, `ALU+RANGE`, `HASH`) and Area 2 (`MEM`, `MemBackend`, `ValueDigest`, `BoundaryCommit`) each satisfy their typed `BusContract` predicates on `(chip, key, a, b, c)` tuples and copy cells, then the composed `S`-slot `ChipLeaf` satisfies `Premises.leaf_sound`, and folding across any leaf partition `P = [s_1, ..., s_L]` (`1 <= s_i <= S`, `\sum s_i = N < 2^64`) produces the identical canonical `outcome_d` and `claim_d` as an uninterrupted `N`-step `sherlock-k-a/1` reference run (`Premises.det` + `SRC-023`). *(Note: small stateless arithmetic/lookup chips `ALU` and `RANGE` in Area 1 can be written directly as Clean `Circuit` definitions in `clean/fork/`, whereas `VerifierGadget` and KZG MSMs stay at the `AccAlgebra` premise boundary.)* | Creates `clean/fork/Clean/Sherlock/ModularRefinement.lean` (connects Area 1/2 `BusContract` + Obligation 1/2 memory/partition lemmas + Obligation 4 `vd_canonical` into `Premises.leaf_sound` and `Aggregation.lean`'s `result_unique`). | `Premises` (`AggregationAxioms.lean:45-95`), Poseidon collision-resistance premise `HashInj`, and (for `Profile B` only) Schwartz-Zippel bound $\epsilon_{\text{LogUp}} \le \frac{2 N_{\text{mem}}}{|Fq|}$ stated as an explicit probabilistic premise hypothesis. | `MODULARITY.md:8-10`, `SRC-023`, `SVM-011`, `INV-NODE-SEAL`. |
| **L-NODE-3 (Obligation 5a): `GP-2` Claim Chaining, `GP-3` Foreign Lift & `GP-4` Diamond DAG Soundness** | Define inductive `SetCert` (`seal`, `sealDep`, `foreignLift`, `joinSet`) over arbitrary DAGs (where one `SetCert` value may be referenced by both children of `joinSet` or `sealDep`, modeling depth-$d$ diamond fan-out with $2^d$ path occurrences and **no occurrence counter**). Prove by well-founded structural induction on `SetCert`: every leaf `claim_d` reachable in `set_root` via valid domain-separated Merkle openings (`LEAFCLAIM`, `DEP`, `JOIN`) is either (i) a genuine `Init`-to-terminal `sherlock-k-a/1` execution whose consumed `deps` recursively have valid `SetCert` witnesses (`GP-2`), or (ii) a foreign proof under an `A_c` + inner `vk_id` both present in `reg_root` (`GP-3`); and `joinSet` is total for all depths $d \ge 64$ (`GP-4`). | Creates `clean/fork/Clean/Sherlock/ClaimDag.lean` + extends `clean/fork/Clean/Sherlock/AggregationAxioms.lean` with `NamedAccAlgebra` (soundness of one-hot `{N, R, F}` base-name routing and `C_close` resolution) and `ForeignPremises`. | `Premises`, `NamedAccAlgebra`, `ForeignPremises`, `MerkleSetInj` (domain-separated Poseidon collision resistance over `LEAFCLAIM`, `DEP`, `JOIN`, `SHAPE`). | `GP-001`..`GP-004`, `INV-NODE-NAMES`, `INV-NODE-DAG`, `INV-NODE-SHAPE`. |

### 1.7 Code Skeleton

Owned files: `crates/mori-recursion/src/{boundary.rs, run_lift.rs, foreign_lift.rs, node.rs, closing.rs}` (superseding monolithic `crates/mori-recursion/src/ivc.rs` once parity gates pass; `ivc.rs` keeps a thin compatibility alias during migration and fixes `write_relation` at `:571-576` to serialize `S_MAX` per `SRC-014`).

```rust
// crates/mori-recursion/src/boundary.rs
//! Swappable leaf-to-node boundary contract (Profile A: Poseidon; Profile B: Horner + committed poly).

use midnight_curves::{Fq as F, G1Projective as C};
use midnight_proofs::circuit::Layouter;
use midnight_proofs::plonk::Error;
use midnight_zk_stdlib::ZkStdLib;
use midnight_circuits::types::{AssignedBit, AssignedNative};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundaryProfileKind {
    ProfileAPoseidon,
    ProfileBHornerZmLogUp,
}

#[derive(Clone, Debug)]
pub struct AssignedLeafHeader {
    pub binding: AssignedNative<F>,
    pub prog_d: AssignedNative<F>,
    pub input_d: AssignedNative<F>,
    pub rules_d: AssignedNative<F>,
    pub h_pre: AssignedNative<F>,
    pub steps: AssignedNative<F>,
    pub deps_d_pre: AssignedNative<F>,
    pub deps_d_post: AssignedNative<F>,
    pub term_tag: AssignedNative<F>,
    pub outcome_d: AssignedNative<F>,
}

pub trait InCircuitBoundaryLink: Clone + std::fmt::Debug {
    type AssignedSegBoundary: Clone + std::fmt::Debug;
    type LeafWitnessAux: Clone + Default + std::fmt::Debug;

    const PROFILE: BoundaryProfileKind;
    const BOUNDARY_SCALARS: usize;

    /// Requires: `leaf_slot` is witnessed for `ChipLeaf` under `Self::PROFILE`.
    /// Ensures: Profile A binds constant `C::identity()`; Profile B witnesses `C_i` via subgroup-checked `AssignmentInstructions::assign` (NEVER `assign_without_subgroup_check`).
    /// Check: `MockProver` mutant `mut_profile_b_torsion_point`.
    fn assign_leaf_committed_instance(
        stdlib: &ZkStdLib,
        layouter: &mut impl Layouter<F>,
        aux: &Self::LeafWitnessAux,
    ) -> Result<midnight_circuits::ecc::AssignedPoint<C, F>, Error>;

    /// Requires: `left` and `right` are adjacent child boundary payloads; `right_enabled` is boolean.
    /// Ensures: When `right_enabled == 1`, constrains `left.post == right.pre` (and equal challenges + summed LogUp deltas under Profile B); returns merged `[left.pre, mux(right_enabled, right.post, left.post)]`.
    fn link_adjacent(
        stdlib: &ZkStdLib,
        layouter: &mut impl Layouter<F>,
        left: &Self::AssignedSegBoundary,
        right: &Self::AssignedSegBoundary,
        right_enabled: &AssignedBit<F>,
    ) -> Result<Self::AssignedSegBoundary, Error>;

    /// Requires: `seg_boundary` belongs to a `SEG` being sealed at `h_lo == 0` and `term_tag in {1,2,3}`.
    /// Ensures: Profile A is a no-op (`Ok(())`); Profile B constrains `sum_1 == 0`, `sum_2 == 0`, and `(gamma_1..2, alpha_1..2) == PoseidonFS(com_root)`, then erases `seg_boundary` (`SRC-023`).
    fn enforce_seal_balance(
        stdlib: &ZkStdLib,
        layouter: &mut impl Layouter<F>,
        seg_boundary: &Self::AssignedSegBoundary,
    ) -> Result<(), Error>;
}

// crates/mori-recursion/src/run_lift.rs
//! `RunLift` `R`: 1-or-2 `ChipLeaf` (`U`) -> `SEG` relation (`MAGIC = b"SHKLFT01"`).

pub const RUN_LIFT_MAGIC: &[u8; 8] = b"SHKLFT01";
pub const S_MAX_DEFAULT: u64 = 16;

#[derive(Clone)]
pub struct RunLiftRelation<B: InCircuitBoundaryLink> {
    pub leaf_k: u32,
    pub leaf_arch: midnight_zk_stdlib::ZkStdLibArch,
    pub leaf_vk_bytes: Vec<u8>,
    pub s_max: u64,
    pub _profile: std::marker::PhantomData<B>,
}

impl<B: InCircuitBoundaryLink> RunLiftRelation<B> {
    /// Requires: `slot0` enabled (`1 <= steps <= s_max`); `slot1` either enabled adjacent leaf or canonical neutral slot (`slot1_en == 0`).
    /// Ensures: Verifies 1 or 2 `U` proofs sharing constant `leaf_vk` and one fixed-base MSM; range-checks `h_lo, steps, h_hi < 2^64` using ONLY `assigned_to_le_bytes(..., Some(8))`; outputs homogeneous collapsed `SEG` accumulator.
    /// Global invariants: `Pow2Range` `queried_tags` subset of `{0,1,4,8,14..19}` (`+0` table rows); `write_relation` serializes `(RUN_LIFT_MAGIC, B::PROFILE, s_max, leaf_k, leaf_vk_bytes)`.
    pub fn synthesize_run_lift(
        &self,
        stdlib: &ZkStdLib,
        layouter: &mut impl Layouter<F>,
        /* Assigned public SEG + shape_d + collapsed acc, and 1..2 leaf witness bundles */
    ) -> Result<(), Error>;
}

// crates/mori-recursion/src/foreign_lift.rs
//! `ForeignLift` `F`: Adapter proof `A_c` + registry Merkle paths -> `SET` (`GP-3`, `MAGIC = b"SHKFOR01"`).

pub const FOREIGN_LIFT_MAGIC: &[u8; 8] = b"SHKFOR01";

#[derive(Clone)]
pub struct ForeignLiftRelation {
    pub adapter_k: u32,
    pub adapter_arch: midnight_zk_stdlib::ZkStdLibArch,
    pub adapter_pi_len: usize,
    pub reg_depth: usize,
}

impl ForeignLiftRelation {
    /// Requires: Witnessed `adapter_vk` of class `(adapter_arch, adapter_k, adapter_pi_len)`, adapter proof publishing `[vk_id_inner, out_d, inner_acc]`, and two Merkle paths into `reg_root`.
    /// Ensures: Verifies adapter proof under witnessed `adapter_vk`, proves `(vk_id(adapter_vk), KIND_ADAPTER)` and `(vk_id_inner, KIND_DIRECT)` in `reg_root`, folds `adapter_acc` + `inner_acc`, and emits `SET` leaf `claim_d`.
    pub fn synthesize_foreign_lift(
        &self,
        stdlib: &ZkStdLib,
        layouter: &mut impl Layouter<F>,
    ) -> Result<(), Error>;
}

// crates/mori-recursion/src/node.rs
//! Polymorphic 2-slot self-recursive `Node` `N` (`M`, `C`, `Resolve`, `U_set`; `MAGIC = b"SHKNOD01"`).

pub const NODE_MAGIC: &[u8; 8] = b"SHKNOD01";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeMode {
    SegMerge,        // SEG + SEG -> SEG
    SegSealUnary,    // SEG + off -> SET (deps_d == 0)
    SegSealResolve,  // SEG + SET -> SET (GP-2: SEG.deps_d_post == SET.set_root)
    SetUnion,        // SET + SET -> SET (GP-4: ZERO n_claims counter / range check)
}

#[derive(Clone)]
pub struct PolymorphicNodeRelation<B: InCircuitBoundaryLink> {
    pub child_k: u32,
    pub child_arch: midnight_zk_stdlib::ZkStdLibArch,
    pub reg_depth: usize,
    pub _profile: std::marker::PhantomData<B>,
}

impl<B: InCircuitBoundaryLink> PolymorphicNodeRelation<B> {
    /// Requires: `(slot0_en, slot1_en) != (0, 0)`; each enabled slot selects one-hot `key in {N, R, F}` matching `[n_repr, r_repr, f_repr]`; disabled slot supplies canonical neutral encoding.
    /// Ensures: Enforces mode constraints (`SegMerge`, `SegSealUnary`, `SegSealResolve`, `SetUnion`), erases physical boundary fields on seal (`SRC-023`), routes fixed-base scalars over `{N, R, F}`, and imposes NO occurrence cap on `SetUnion` (`GP-4`).
    pub fn synthesize_node(
        &self,
        stdlib: &ZkStdLib,
        layouter: &mut impl Layouter<F>,
    ) -> Result<(), Error>;
}

// crates/mori-recursion/src/closing.rs
//! `ClosingProof` `C_close`: resolves `{N, R, F}` fixed bases into homogeneous KZG accumulator (`MAGIC = b"SHKCLS01"`).

pub const CLOSING_MAGIC: &[u8; 8] = b"SHKCLS01";

#[derive(Clone)]
pub struct ClosingRelation {
    pub node_vk_bytes: Vec<u8>,
    pub run_lift_vk_bytes: Vec<u8>,
    pub foreign_lift_vk_bytes: Vec<u8>,
}

impl ClosingRelation {
    /// Requires: Root `Node` proof has `stmt_tag == TAG_SET` and pinned `[n_repr, r_repr, f_repr]`.
    /// Ensures: Resolves all `{N, R, F}` fixed-base scalars against constant VK commitments, folds `C_close` + `N` accumulators into a homogeneous `[G_0]` accumulator for `DECIDER`.
    pub fn synthesize_closing(
        &self,
        stdlib: &ZkStdLib,
        layouter: &mut impl Layouter<F>,
    ) -> Result<(), Error>;
}
```

### 1.8 Agent Context Boundary

- **MUST read (~1,450 lines total):**
  1. This module kit (§1.1–§1.9) and the frozen `LeafBoundaryContract` trait header from Area 2 (`crates/mori-chip/src/membackend.rs:1-120` + `crates/mori-recursion/src/boundary.rs`).
  2. `crates/mori-recursion/src/inner.rs:1-316` and `crates/mori-recursion/src/ivc.rs:340-590` (existing `verify_children_incircuit`, `constrain_heights`, `constrain_genesis_neutral`, `AssignedAccumulator` patterns).
  3. `MC:verifier/verifier_gadget.rs:280-365`, `MC:verifier/accumulator.rs:260-310`, `MC:ecc/foreign/weierstrass_chip.rs:343-360`, and `MC:field/decomposition/pow2range.rs:215-236`.
  4. `clean/fork/Clean/Sherlock/{Aggregation.lean, AggregationAxioms.lean}` (when implementing Lean obligations).
- **MUST NOT need to read:**
  - Any Area 1 internal chip implementation (`crates/mori-chip/src/{rfcore.rs, m3.rs, vsel.rs, range.rs, poseidon.rs}`).
  - Any Area 2 internal Merkle/LogUp or `sherlock-value/1` traversal gate (`crates/mori-chip/src/dense.rs`, `leaf.rs` internal rows).
  - Compact SDK / TypeScript deployment scripts (`tools/sherlock-sdk/`) or `mori-ledger`.

### 1.9 OpenSpec 1.17.0 Capability Spec (`specs/sherlock-node-integration/spec.md`)

```markdown
# Capability: sherlock-node-integration

## Requirements

### Requirement: Leaf-Lift Segment Aggregation (`MOD-NODE-001`)
`RunLift` (`R`) SHALL verify 1 or 2 `ChipLeaf` proofs at `k = 20` under a constant leaf VK, enforce `0 <= h_lo < h_hi < 2^64` and `1 <= steps <= S_MAX` via byte-only `Pow2Range` decompositions, check subgroup membership on `Profile B` committed instances, and emit a `SEG` certificate.

#### Scenario: Two-leaf lift links contiguous leaves
- **WHEN** `RunLift` receives two valid `ChipLeaf` proofs with equal `(prog_d, input_d)`, `L.post == R.pre`, `L.term_tag == 0`, and `L.h_pre + L.steps == R.h_pre`
- **THEN** `RunLift` accepts and outputs `SEG(h_lo = L.h_pre, h_hi = R.h_pre + R.steps)` with `table_rows == 1,032,467`

#### Scenario: Torsion-grinding committed instance or height wrap is refused
- **WHEN** `Profile B` supplies a `G1` point outside prime subgroup `r` OR `h_pre + steps >= 2^64`
- **THEN** `RunLift` constraints are unsatisfiable (`MockProver` mutants `mut_profile_b_torsion_point`, `mut_height_wrap_2p64`; Lean `L-NODE-1`)

### Requirement: Physical-State-Erasing Seal and `GP-2` Dependency Resolution (`MOD-NODE-002`)
`Node` (`N`) seal modes SHALL require `h_lo == 0`, `term_tag in {1, 2, 3}`, registry membership of `leaf_id`, `Profile B` zero LogUp sum when active, and `deps_d_post` equal to `0` (unary `C`) or `slot_1.set_root` (`Resolve`), emitting `claim_d` with all physical state commitments erased.

#### Scenario: Dependent run discharges consumed claim set
- **WHEN** `Node` runs `SegSealResolve` on a halted `SEG` with `h_lo == 0` and `deps_d_post == S_A.set_root` alongside proven `SET` `S_A`
- **THEN** `Node` emits `SET` root `Poseidon("SHK_SET_DEP", claim_d, S_A.set_root)` containing no `pre_d`, `post_d`, or memory root (Lean `L-NODE-2`, `L-NODE-3`)

#### Scenario: Unfinished segment or undischarged dependency is refused at seal
- **WHEN** `Node` attempts to seal a `SEG` with `term_tag == 0` (`Running`), `h_lo != 0`, or `deps_d_post` mismatched against `slot_1`
- **THEN** `Node` constraints are unsatisfiable (`mut_seal_running_or_nonzero_hlo`, `mut_seal_undischarged_deps`)

### Requirement: Registry Foreign Proof Lift (`MOD-NODE-003`)
`ForeignLift` (`F`) SHALL verify an adapter proof `A_c` under a witnessed VK, fold both the adapter accumulator and its published inner foreign accumulator, verify Merkle membership of both `vk_id(A_c)` and `vk_id_inner` in `reg_root`, and emit a canonical `SET` foreign claim.

#### Scenario: Registered Compact adapter and contract VK produce a `SET` claim
- **WHEN** `ForeignLift` receives a valid `A_c` proof and valid `reg_root` paths for both `vk_id(A_c)` and `vk_id_inner`
- **THEN** `ForeignLift` accepts and outputs `SET` over `Poseidon("SHK_CLAIM", KIND_FOREIGN, vk_id_inner, adapter_id, out_d)`

#### Scenario: Unregistered inner foreign VK is refused
- **WHEN** `vk_id(A_c)` is in `reg_root` but `vk_id_inner` is absent from `reg_root`
- **THEN** `ForeignLift` constraints are unsatisfiable (`mut_gp3_unregistered_inner_vk`)

### Requirement: Arbitrary-Shape PCD and Uncapped Diamond `SET` Union (`MOD-NODE-004`)
`Node` (`N`) SHALL merge `SEG + SEG -> SEG` associatively and join `SET + SET -> SET` (`U_set`) under one fixed core key set with address-free `shape_d` and NO occurrence counter or `< 2^64` cap on `SET` claims.

#### Scenario: Depth-64 diamond claim DAG verifies without counter overflow
- **WHEN** a root `SET` claim is shared across both branches of 64 sequential `SetUnion` diamond stages ($2^{64}$ path occurrences)
- **THEN** every `Node` stage and Quint invariant `DiamondDagDepth64NoOverflow` accepts with constant proof byte size (Lean `L-NODE-3`)
```

---

## 2. Module `M11: DECIDER` (`RunDecider` `SHKRUND1`, `ClaimDecider`, and Ledger `9.1.0.1` Boundary)

### 2.1 Purpose
`DECIDER` verifies the terminal recursive proof (`ClosingProof` `C_close` in the full `{U, R, F, N, C_close, D}` key set, or the final single-key `Node` proof in M4 single-run mode) under a Blake2b on-chain transcript, folds its verifier accumulator with the carried KZG accumulator, executes the in-circuit BLS12-381 Miller-loop + final-exponentiation pairing check against SRS `[1]_2, [\tau]_2`, and exposes ONLY address-free public scalars to Midnight Ledger `9.1.0.1`. It enforces the tightest resource invariant in the system — `Pow2RangeChip` `table_rows == 1,032,467` out of `1,048,570` usable rows at `k = 20` — while bridging Moriarty's `zk-stdlib` `Relation` key to Compact `0.36.0` contract transcripts (`SherlockRun` and `SherlockClaims`).

### 2.2 Frozen Interface

#### Decider Circuit Public-Instance & On-Chain Transcript Contract
`DECIDER` is a `midnight_zk_stdlib::Relation` (`crates/mori-recursion/src/run_decider.rs:181`, `review.md:210`) proved at `k = 20` with `TranscriptHash = blake2b_simd::State` (`L9:transient-crypto/src/proofs.rs:65`, `review.md:204`), `Instance(0) = C::identity()` (no committed instances on chain, `L9:transient-crypto/src/proofs.rs:563-575`), and `Instance(1)` formatted as a Compact `0.36.0` public-input transcript (`SMN-003`, `requirements.md:105`):

1. **Single-Run Mode (`RunDeciderRelation`, `MAGIC = b"SHKRUND2"`, updating `SHKRUND1` at `run_decider.rs:48,279-301` per C6 `SRC-006`):**
   - Verifies either root `Node` (M4 fast-path, resolving `Node`'s fixed bases directly as in `run_decider.rs:265-275`) or `C_close` (resolving `C_close`'s `[G_0]` homogeneous base) with `single_run_flag == 1`.
   - Exposes **exactly four logical run scalars** embedded into the recaptured Compact `0.36.0` `acceptRun` transcript (`~60–70` transcript field slots total <= `MAX_CALL_PUBLIC_INPUTS = 65,536`, `review.md:215`):
     - `prog_d: F` (full `Fq`, matched against contract ledger field `program_d` in per-program `SherlockRun` `SRC-007`, or passed as call data in universal `SherlockRun` `GP-1`, `review.md:154`).
     - `input_d: F` (`Poseidon("SHK_IN", val_d, fuel_lo, fuel_hi, stack_limit)`, call data bound by proof, `SRC-007`, `SRC-021`).
     - `outcome_d: F` (`Poseidon("SHK_OUT", 1, tag, payload_d)` per `sherlock-value/1` `SRC-022`: `tag = 0` `Halted(vd(v))`, `tag = 1` `Rejected(code)`, `tag = 2` `Exhausted(kind)`).
     - `n_steps: F` (`[1, 2^64 - 1]`, range-checked in circuit via `assigned_to_le_bytes(layouter, &n_steps, Some(8))` and bound to Compact `Uint<64>`).
   - **Forbidden public lanes (`SRC-006`, `requirements.md:31`):** `fin_d`, `init_d`, `pre_d`, `post_d`, `mem_root`, `lanes_d`, `len`, `fp`, `tp`, `result_be`, or any reference/address. (Deletes legacy `[prog_d, init_d, fin_d, n_steps, result_be]` at `run_decider.rs:53-58,279-301`.)
   - **Contract replay & state rule (`SherlockRun.acceptRun`, `SRC-007`):** Computes `claim_d = Poseidon("SHK_RUN", 1, prog_d, input_d, outcome_d)` (`dissent.md:209-212` `D-3`: `n_steps` is stored beside `outcome_d`, NOT hashed into `claim_d`), asserts `!runs.member(claim_d)`, inserts `claim_d` into `runs: Set<Field>`, and stores `(outcome_d, n_steps)`.
2. **General-Purpose Batch / DAG Mode (`ClaimDeciderRelation`, `MAGIC = b"SHKCLMD1"`, `GP-1..GP-4`):**
   - Verifies `C_close` proof (`k = 20`), folds `C_close`'s verifier accumulator with `C_close`'s published homogeneous accumulator, executes `PairingChip::check_kzg` (`crates/mori-recursion/src/pairing.rs`), and exposes **exactly three logical scalars** in the `SherlockClaims.acceptRoot` transcript (`architecture.md:88-90,380-394`, **with `n_claims` deleted per `AUDIT.md:19-22`**):
     - `reg_root: F` (read from `SherlockClaims` contract state into the transcript so stale/foreign registries are refused).
     - `set_root: F` (root of the proven `SET` claim DAG).
     - `shape_d: F` (address-free proof-graph commitment, `GP-004`; recorded for auditability, never used for replay refusal or claim identity, `requirements.md:59`).
   - **Contract entrypoints (`SherlockClaims`, `architecture.md:380-395`):**
     - `acceptRoot(set_root: Field, shape_d: Field)`: verifies `ClaimDeciderRelation` proof against ledger `reg_root`, inserts `set_root` into `roots: Set<Field>`.
     - `openClaim(root: Field, item: Field, is_subroot: Boolean, path: Vector<64, MerkleStep>)`: ordinary Compact `0.36.0` circuit (`k <= 14`) verifying up to 64 levels of `JOIN`/`DEP`/`LEAFCLAIM` per call; if `is_subroot`, inserts `item` into `roots` (allowing arbitrary DAG depth $> 64$ across multiple calls, `architecture.md:388-391`); otherwise checks `!claims.member(item)` and inserts `item` (`claim_d`) into `claims: Set<Field>` so diamond-shared claims settle at most once (`architecture.md:367,391-393`).
     - `admitKey(vk_id: Field, kind: Uint<8>, new_reg_root: Field, merkle_witness: ...)`: owner maintenance-authority operation (`SMN-007`, `requirements.md:109`).
   - *Note:* If single-run `acceptRun` is exposed as a second Compact template over the same `C_close`-backed decider (`architecture.md:403-405`), `D` binds `(reg_root, set_root, shape_d, single_run_flag, prog_d, input_d, outcome_d, n_steps)` (`8` scalars) and `acceptRun` requires `single_run_flag == 1`.

#### Exact Proof of the `Pow2RangeChip` `table_rows == 1,032,467` Invariant (`MC:field/decomposition/pow2range.rs:215-236`)
In `midnight-circuits-7.3.0`, `Pow2RangeChip::load_table` (`MC:field/decomposition/pow2range.rs:215-236`) populates its single lookup table at synthesis end by unconditionally writing `1` row for `(tag = 0, val = 0)` and then, for each distinct `bit_len` in `self.queried_tags: BTreeSet<usize>` (`1 <= bit_len <= max_bit_len`), writing $2^{\text{bit\_len}}$ rows `(tag = bit_len, val = 0 .. 2^bit_len - 1)`.

In `RunDecider` (`review.md:213`), `PairingChip` ($\mathbb{F}_p / \mathbb{F}_{p^{12}}$ foreign-field arithmetic over `max_bit_len = 19`), `WeierstrassChip`, `VerifierGadget`, and `assigned_to_le_bytes` query **exact tag set** $T_{\text{dec}} = \{0, 1, 4, 8, 14, 15, 16, 17, 18, 19\}$:
$$\text{table\_rows}(T_{\text{dec}}) = 2^0 + 2^1 + 2^4 + 2^8 + \sum_{b=14}^{19} 2^b = 1 + 2 + 16 + 256 + 16{,}384 + 32{,}768 + 65{,}536 + 131{,}072 + 262{,}144 + 524{,}288 = \mathbf{1{,}032{,}467}.$$
At `k = 20` ($2^{20} = 1{,}048{,}576$), `midnight-proofs` reserves `blinding_factors + 1 = 6` unusable rows (`MP:dev/cost_model.rs:370-398`), leaving **`1,048,570` usable rows**. Therefore:
$$\text{Remaining Pow2Range table headroom} = 1{,}048{,}570 - 1{,}032{,}467 = \mathbf{16{,}103\text{ rows}}.$$
Consequences enforced as a hard module contract on `M11`:
1. **Unqueried tags are `{2, 3, 5, 6, 7, 9, 10, 11, 12, 13}`** (whose full sum is $4 + 8 + 32 + 64 + 128 + 512 + 1{,}024 + 2{,}048 + 4{,}096 + 8{,}192 = 16{,}108 > 16{,}103$, so activating the remaining sub-14 tags overflows `k = 20` into `k = 21` by 5 rows!).
2. **Every range/byte decomposition in `M11` MUST use `assigned_to_le_bytes(layouter, &val, Some(n_bytes))`** (`n_bytes = 8` for `u64` `n_steps`, `n_bytes = 1` for `u8` `code`/`kind`). Because `assigned_to_le_bytes` queries **only** `bit_len = 8` (`256` rows), which is already in $T_{\text{dec}}$, it adds **`+0` `table_rows`**, keeping `table_rows == 1,032,467` exact!
3. Calling `assert_lower_than_fixed` or `assigned_to_le_chunks` with non-8 bit chunks is **forbidden** in `M11` (checked by an automated `cost_model` assertion `assert_eq!(model.table_rows, 1_032_467)` in unit tests).

#### Midnight Ledger `9.1.0.1` Compatibility Contract (`review.md:198-253`, `SMN-001..007`)
- **On-chain verifier entrypoint:** `ProofVersioned::V3` calls `vk.verify(&PARAMS_VERIFIER, proof, pis)` (`L9:ledger/src/structure.rs:446-506`) over `bls_midnight_2p14` G2 verifier params (`ParamsVerifierKZG` checks no `k` upper bound, `MP:poly/kzg/params.rs:235-239`, `review.md:206-207`). `MidnightVK::read` reconstructs the PLONK `ConstraintSystem` via `ZkStdLib::configure(arch, k - 1)` (`ZS:lib.rs:1573-1594`, `review.md:210`) — **no ZKIR is used or required for `DECIDER`, `Node`, or `ChipLeaf`**.
- **Ledger hard caps (`SMN-003`, `SMN-006`):** Raw VK `<= 50,000` bytes (measured `2,189 B` raw / `2,217 B` tagged at `k = 20`, `review.md:214`), public inputs `<= 65,536` (`~60–70` slots, `review.md:215`), contract operations `<= 256` (`1` in `SherlockRun`, `3` in `SherlockClaims`, `review.md:216`), proof size `~6,320–6,432 B` (`<= 1 MiB` tx limit, `review.md:217-218`).
- **`0.8.3` Prover vs `0.8.2` Chain Verifier Gate (`review.md:201,246`):** Workspace proves with `midnight-proofs 0.8.3` (`Cargo.lock:2137-2138`), whereas `node-2.1.101` / `ledger-9.1.0.1` resolves `midnight-proofs 0.8.2` (`L9:Cargo.lock:3509-3510`). Before any network call, `SRC-020` (`requirements.md:45`) validates a `0.8.3`-generated decider proof inside a standalone verifier binary/container locked to `ledger-9.1.0.1`'s exact `0.8.2` `Cargo.lock`.

### 2.3 Invariants

**Guaranteed by `M11`:**
1. **Final KZG pairing discharge (`INV-DEC-PAIR`, `SAS-006`):** `DECIDER` verifies the constant-VK `Node`/`C_close` proof, folds its verifier accumulator with the carried accumulator, and enforces $e(P_{\text{lhs}}, [1]_2) = e(P_{\text{rhs}}, [\tau]_2)$ in circuit (`run_decider.rs:265-277`).
2. **Terminal-only publication & physical erasure (`INV-DEC-C6`, `SRC-006`):** No `Running` segment (`stmt_tag == TAG_SEG` or `term_tag == 0`) can satisfy `DECIDER`; zero physical state commitments (`pre_d`, `post_d`, `fin_d`, `init_d`, `mem_root`, `len`) enter the public instance column.
3. **Exact `Pow2Range` table ceiling (`INV-DEC-TABLE`):** `table_rows == 1,032,467 <= 1,048,570` and `circuit_rows <= 1,000,000` (`design.md:105`).
4. **Canonical transcript binding (`INV-DEC-TMPL`, `SMN-003`, `SRC-007`):** Every public field in the Compact `0.36.0` `acceptRun` / `acceptRoot` transcript is canonical in `Fq`, contains zero `noop 0` padding instructions, and binds `prog_d` / `reg_root` against ledger state.

**Assumed from Upstream Modules:**
1. **From `M10: NODE INTEGRATION`:** `C_close` (or M4 `Node`) guarantees that `TAG_SET` implies `h_lo == 0`, `term_tag in {1, 2, 3}`, discharged `deps_d`, and zero `Profile B` LogUp sum, and outputs an accumulator whose fixed bases are restricted to `C_close`'s `[G_0]` (or `Node`'s constant VK bases in M4 fast-path).
2. **From Target Tooling (`SMN-001`, `SMN-005`):** Client transaction builder attaches `>= 2` DUST spends while `midnight-ledger #838` is open and avoids `VerifierKeyRemove` while `midnight-node #2375` is open (`review.md:228,249`).

### 2.4 Row Budget (`ESTIMATE`)

| Circuit / Artifact | Circuit Rows | `Pow2Range` Table Rows | Domain `k` | Gate / Limit | Margin |
|---|---|---|---|---|---|
| `RunDecider` (E8 measured baseline over 1-key `ChipNode`, `design.md:130`, `review.md:213`) | `929,166` (measured) | `1,032,467` (measured) | `k = 20` | `<= 1,000,000` circuit / `<= 1,048,570` table | `+70,834` circuit / **`+16,103` table** |
| `RunDecider` (`SHKRUND2`, C6 4-scalar statement `[prog_d, input_d, outcome_d, n_steps]`, M4 1-key fast-path) | **`927,500 – 930,500` `ESTIMATE`** (removes 16-byte `result_be` packing; keeps `Some(8)` `n_steps` check; leaf `outcome_d` already hashed in `M7`) | **`1,032,467`** (`+0` rows) | `k = 20` | `<= 1,000,000` / `<= 1,048,570` | `+69,500..+72,500` / **`+16,103`** |
| `ClaimDecider` (`SHKCLMD1` over `C_close`, `[reg_root, set_root, shape_d]`) | **`810,000 – 860,000` `ESTIMATE`** (`C_close` already resolved `{N, R, F}` fixed bases, saving ~35–40 fixed-base MSMs vs 1-stage decider; `architecture.md:221-224`) | **`1,032,467`** (`+0` rows) | `k = 20` | `<= 1,000,000` / `<= 1,048,570` | `+140,000..+190,000` / **`+16,103`** |
| Decider Proving Wall-Clock & RSS (one-shot per run/DAG, independent of step count) | `~70 – 95 s` `ESTIMATE` at `20t`, `~18 – 22 GB` RSS `ESTIMATE` (`<= 48 GB` cap, `SRC-016`) | — | `k = 20` | Paid **once** per sealed bundle (`0 s/step` asymptotic) | — |
| On-Chain `acceptRun` / `acceptRoot` Verification (`review.md:214-227`) | VK `~2.2 KiB` (`<= 50 KB`), proof `~6.3–6.4 KiB` (`<= 1 MiB`), `~60–70` PIs (`<= 65,536`) | — | `k = 20` | `~3.59 ms` `ESTIMATE` (<0.5% block compute) | — |

### 2.5 Verification

1. **Quint Specification (`models/sherlock/quint/decider_contract.qnt`):**
   - Models `RunDecider` + `SherlockRun.acceptRun` and `ClaimDecider` + `SherlockClaims.{acceptRoot, openClaim, admitKey}`.
   - Checks invariants: `NoStateDigestOnChain` (`SRC-006`), `PartitionReplayRefusedByClaimD` (`SRC-007`, `SRC-023`: proving the same `(prog_d, input_d)` under two partitions yields two `shape_d` values and two physical `post_d` values, yet identical `claim_d`, and the second `acceptRun`/`openClaim` is refused), `DiamondAncestorOpenedAtMostOnce` (`architecture.md:367`), and `StaleOrForeignRegRootRefused`.
2. **`MockProver` & Local Ledger `9.1.0.1` Positive Tests (`crates/mori-recursion/tests/decider_c6.rs`, `crates/mori-ledger/tests/validate_call_9_1_0_1.rs`):**
   - `pos_decider_table_rows_exact`: runs `cost_model` on `RunDeciderRelation` and `ClaimDeciderRelation` and asserts `circuit_rows <= 1_000_000` and **`table_rows == 1_032_467`**.
   - `pos_decider_all_outcomes`: `MockProver` acceptance for `Halted(v)`, `Rejected(code)`, and `Exhausted(kind)`.
   - `pos_ledger_9_1_0_1_validate_call`: proves `RunDecider` with `0.8.3` and verifies the full Compact `0.36.0` `acceptRun` transaction against a proof-verifying `ledger-9.1.0.1` (`0.8.2` lockfile) instance with mainnet parameters (`SRC-020`, `SMN-003`).
3. **Named Negative Mutants (`SAS-003`, `SAS-004`):**
   - `mut_decider_accept_unsealed_seg`: feed an unsealed `TAG_SEG` (`Running`) proof witness into `DECIDER` -> unsatisfiable (`SRC-006`).
   - `mut_decider_corrupt_pairing_carry`: flip one limb of `acc_rhs` in witness while recomputing Poseidon collapse transcript -> rejected by `PairingChip` check (`SAS-006`).
   - `mut_decider_nsteps_overflow_2p64`: `n_steps = 2^64` -> unsatisfiable in `assigned_to_le_bytes(..., Some(8))`.
   - `mut_decider_five_pi_flips` (`SRC-020` / row `A32`): mutate each of `prog_d`, `input_d`, `outcome_d`, `n_steps`, and contract `program_d` / `reg_root` individually on a real proof -> refused by `validate_call`.
   - `mut_contract_cross_partition_replay` (`SRC-023` / row `A38`): submit a second valid decider proof of the same `(prog_d, input_d)` proved under a different leaf partition -> refused by `SherlockRun` / `SherlockClaims` duplicate `claim_d` check.
   - `mut_pow2range_forbidden_tag_guard`: unit test injecting a synthetic `assigned_to_le_chunks(..., 9)` confirms our CI table-row guard catches `table_rows > 1,032_467`.

### 2.6 Lean Obligations (`ARCH-BRIEF.md:52-60`)

| Obligation | Plain-English Theorem Statement | Lean File Extended / Created | Axioms in `AggregationAxioms.lean` Relied On | Module Contract Discharged |
|---|---|---|---|---|
| **L-DEC-1: Outcome-Only Decider Soundness & Replay Uniqueness (`SRC-006`, `SRC-007`, `SRC-023`)** | If `AcceptRun(prog_d, input_d, outcome_d, n_steps)` (or `OpenClaim(reg_root, set_root, claim_d)`) holds over a pairing-verified `Decided` certificate, then (1) there exists a unique canonical reference run from `Init(prog_d, input_d)` terminating in `outcome` (`Halted`, `Rejected`, or `Exhausted`, never `Running`) in `n_steps < 2^64` steps with `outcomeDigest(outcome) = outcome_d`, (2) `claim_d` depends only on `(prog_d, input_d, outcome_d)` and is invariant across all leaf partitions and aggregation bracketings, and (3) inserting `claim_d` into the contract's monotonic `claims` set guarantees at-most-once settlement even when `claim_d` has $2^d$ paths in `set_root`. | Extends `clean/fork/Clean/Sherlock/Aggregation.lean` (`Decided`, `AcceptRun`, `result_unique` at `:1085-1260`) and `clean/fork/Clean/Sherlock/ClaimDag.lean` (`AcceptRoot`, `OpenClaim`, `openClaim_at_most_once`). | `Premises`, `PairingDeciderSound` (terminal KZG pairing discharges homogeneous `Acc`), `OutcomeDigestInj` (from Area 2 Obligation 4 `vd_canonical`). | `INV-DEC-PAIR`, `INV-DEC-C6`, `SRC-006`, `SRC-007`, `SRC-023`, `GP-002`, `GP-004`. |

### 2.7 Code Skeleton

Owned files: `crates/mori-recursion/src/{run_decider.rs, claim_decider.rs}`, `crates/mori-ledger/src/template.rs`, `tools/sherlock-sdk/{sherlock-run.compact, sherlock-claims.compact}`.

```rust
// crates/mori-recursion/src/run_decider.rs (updated C6 interface)
//! Terminal `RunDecider` (`MAGIC = b"SHKRUND2"`): C6 outcome-only `[prog_d, input_d, outcome_d, n_steps]`.

use midnight_curves::Fq as F;
use midnight_proofs::{circuit::Layouter, plonk::Error};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};

pub const RUN_DECIDER_MAGIC: &[u8; 8] = b"SHKRUND2";
pub const DECIDER_K: u32 = 20;
pub const EXPECTED_POW2RANGE_TABLE_ROWS: usize = 1_032_467;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunPublicStatement {
    pub prog_d: F,
    pub input_d: F,
    pub outcome_d: F,
    pub n_steps: u64,
}

impl RunPublicStatement {
    /// Ensures: Returns `Poseidon("SHK_RUN", 1, prog_d, input_d, outcome_d)` (`n_steps` excluded per `SRC-007`, `D-3`).
    pub fn claim_digest(&self) -> F;
}

#[derive(Clone)]
pub struct RunDeciderRelation {
    pub pred_arch: ZkStdLibArch,
    pub pred_vk_bytes: Vec<u8>,
    pub transcript_template_hash: [u8; 32],
}

impl Relation for RunDeciderRelation {
    type Statement = RunPublicStatement;
    type Witness = RunDeciderWitness;
    type Error = Error;

    /// Ensures: Configures `ZkStdLibArch` without enabling unused chips; `Pow2Range` `max_bit_len == 19`.
    fn used_chips(&self) -> ZkStdLibArch;

    /// Ensures: Emits exact Compact 0.36.0 `acceptRun` public-input vector binding `(prog_d, input_d, outcome_d, n_steps)` with ZERO state digests (`SRC-006`) and ZERO `noop 0` (`SMN-003`).
    fn format_public_inputs(stmt: &Self::Statement) -> Vec<F>;

    /// Requires: `wit` holds a sealed `TAG_SET` (`single_run_flag == 1`) predecessor proof (`C_close` or M4 `Node`).
    /// Ensures: Pins `pred_vk` as constants, folds predecessor + carried accumulators, verifies KZG pairing, range-checks `n_steps` ONLY via `assigned_to_le_bytes(..., Some(8))` (`+0` `Pow2Range` table rows), and binds `format_public_inputs`.
    /// Global invariants: `circuit_rows <= 1_000_000`, `table_rows == 1_032_467 < 1_048_570`.
    fn circuit(
        &self,
        stdlib: &ZkStdLib,
        layouter: &mut impl Layouter<F>,
        stmt: Self::Statement,
        wit: midnight_proofs::circuit::Value<Self::Witness>,
    ) -> Result<(), Error>;

    /// Ensures: Serializes `(RUN_DECIDER_MAGIC, DECIDER_K, transcript_template_hash, pred_vk_bytes)` (`SRC-008`, `SRC-009`).
    fn write_relation<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()>;
}

// crates/mori-recursion/src/claim_decider.rs
//! General-purpose `ClaimDecider` (`MAGIC = b"SHKCLMD1"`): `[reg_root, set_root, shape_d]` for `SherlockClaims`.

pub const CLAIM_DECIDER_MAGIC: &[u8; 8] = b"SHKCLMD1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClaimBatchPublicStatement {
    pub reg_root: F,
    pub set_root: F,
    pub shape_d: F, // Note: NO `n_claims` field (`AUDIT.md:19-22`).
}

#[derive(Clone)]
pub struct ClaimDeciderRelation {
    pub closing_arch: ZkStdLibArch,
    pub closing_vk_bytes: Vec<u8>,
    pub accept_root_template_hash: [u8; 32],
}
```

### 2.8 Agent Context Boundary

- **MUST read (~950 lines total):**
  1. This module kit (§2.1–§2.9) and `ClosingRelation` / `PolymorphicNodeRelation` public-instance layout from §1.2 / §1.7.
  2. `crates/mori-recursion/src/{run_decider.rs, pairing.rs}` (`:1-383`).
  3. `MC:field/decomposition/pow2range.rs:200-250` and `MP:dev/cost_model.rs:125-145,370-398`.
  4. `crates/mori-ledger/src/template.rs` and `tools/sherlock-sdk/sherlock-run.compact:1-50`.
- **MUST NOT need to read:**
  - Any `crates/mori-chip/` file or `mori-core` interpreter rule implementation.
  - Multi-leaf scheduler or checkpointing code (`M9: CONTINUATION`).

### 2.9 OpenSpec 1.17.0 Capability Spec (`specs/sherlock-decider/spec.md`)

```markdown
# Capability: sherlock-decider

## Requirements

### Requirement: Outcome-Only Terminal Pairing Decider (`MOD-DEC-001`)
`RunDecider` SHALL verify the sealed predecessor proof at `k = 20` under Blake2b, fold and pair the KZG accumulator, refuse `Running` states, and expose ONLY `[prog_d, input_d, outcome_d, n_steps]` with zero physical state digests.

#### Scenario: Terminal run proof verifies on Ledger 9.1.0.1
- **WHEN** `RunDecider` proves a sealed `Halted`, `Rejected`, or `Exhausted` run and submits to local proof-verifying Ledger `9.1.0.1` (`0.8.2` verifier)
- **THEN** `validate_call` accepts `acceptRun` and mutating any one of `(prog_d, input_d, outcome_d, n_steps, program_d)` is refused (`SRC-006`, `SRC-020`)

#### Scenario: Unsealed `Running` segment or corrupted accumulator is refused
- **WHEN** `RunDecider` is given a `TAG_SEG` (`Running`) witness OR a mutated carried accumulator point
- **THEN** `RunDecider` circuit constraints are unsatisfiable (`mut_decider_accept_unsealed_seg`, `mut_decider_corrupt_pairing_carry`)

### Requirement: Exact `Pow2Range` Table Row Preservation (`MOD-DEC-002`)
Every range check inside `RunDecider` and `ClaimDecider` SHALL use `assigned_to_le_bytes(..., Some(n_bytes))` so `Pow2RangeChip` `table_rows` equals `1,032,467` (`<= 1,048,570`) and `circuit_rows <= 1,000,000`.

#### Scenario: Cost model enforces zero new `Pow2Range` tags
- **WHEN** `cost_model` evaluates `RunDeciderRelation` and `ClaimDeciderRelation`
- **THEN** `model.table_rows == 1_032_467` and `model.rows <= 1_000_000` (`pos_decider_table_rows_exact`)

### Requirement: Partition-Invariant Replay Refusal and Multi-Step DAG Opening (`MOD-DEC-003`)
`SherlockRun` and `SherlockClaims` SHALL key replay refusal on `claim_d = Poseidon("SHK_RUN", 1, prog_d, input_d, outcome_d)` (or foreign `claim_d`), independent of leaf partition or `shape_d`, and `openClaim` SHALL open DAGs deeper than 64 via authenticated sub-roots without any `n_claims` cap.

#### Scenario: Re-proving the same run under a new leaf partition is refused as a replay
- **WHEN** `(prog_d, input_d)` is proved under `S=16` uniform leaves and again under `{1, 3, 7, 16}` early-close leaves
- **THEN** both proofs yield the identical `claim_d` and the second `acceptRun`/`openClaim` call is refused (`SRC-007`, `SRC-023`, Lean `L-DEC-1`)
```

---

## 3. Module `M9: CONTINUATION` (Intra-Session Segment Tree & Cross-Session Checkpointing)

### 3.1 Purpose
`CONTINUATION` orchestrates unbounded multi-leaf execution (`N < 2^64` steps, `GP-1`) on a bounded-RAM host by streaming witness chunks from the host engine into a parallel `3×6t` segment-tree prover pool that holds at most `O(log L)` open `SEG` proofs in memory while persisting the full `SRC-019` verification bundle to disk. It also provides cross-session stop-and-resume (`Mode 1` production default) by persisting a proven `SEG` prefix plus the retained segment store and composing resumed suffixes strictly via `M: SEG + SEG -> SEG` segment recursion (`0` loader leaves, never `GP-2`), while isolating portable value-only checkpointing (`Mode 2`) behind an explicit open-research boundary.

### 3.2 Frozen Interface

#### Tier 1: Intra-Session Streaming / Parallel Binary Segment Tree (`research/sherlock-runlength-consensus-20261111/CONSENSUS.md:45,204-215`)
1. **Concurrency & Kernel Memory Envelope (`SRC-011`, `SRC-016`, `requirements.md:36,41`):**
   - Runs inside a single parent `systemd-run --user --scope` cgroup with `MemoryMax=100G` and `MemorySwapMax=0`, each worker process capped at `<= 48 GiB` (`DW_DEFAULT_RSS_CAP`).
   - Default topology on the 20-core / 120 GB qualification host: **`P = 3` prover workers × `RAYON_NUM_THREADS = 6` (`18` cores)** + **`2` cores** dedicated to the single-producer witness stream (`moriarty-sherlock-c6-witness-stream/1`, `SVM-007`) and segment-store I/O. (Avoids the measured `20t + 20t` contention cliff that degrades node proving from `61–67 s` to `90.2 s`, `research/sherlock-recursion-design-review-20261110/experiments/e2-e4.md:14-19`.)
2. **Online Binary Reduction (`O(log L)` RAM Stack):**
   - Under **`Profile A` (`DensePageBackend` + `PoseidonBoundary`):** Leaf chunks `(chunk_idx = i, steps in 1..=S)` are self-contained from `(pre_d, Merkle witnesses)` (`architecture.md:256`). As pairs `(U_{2m}, U_{2m+1})` finish, a worker immediately proves `R: U + U -> SEG` at height `0` (with 1-leaf `R` on an odd terminal tail). Completed `SEG` nodes enter a binary carry stack indexed by subtree rank `r >= 0`: whenever two adjacent `SEG` proofs at rank `r` are ready, a worker proves `N(SegMerge): SEG + SEG -> SEG` at rank `r + 1` and drops both child proofs from RAM after flushing them to the `SRC-019` disk bundle. For $L < 2^{64}$ leaves, the in-RAM stack holds **at most `64` open `SEG` artifacts** (`< 1 MiB` RAM).
   - Under **`Profile B` (`ZmLogUpBackend` + `HornerBoundary`):** Because leaf proofs share run-wide Fiat-Shamir challenges $(\gamma_{1..2}, \alpha_{1..2})$ derived from all leaf `Instance(0)` commitments $C_0, \dots, C_{L-1}$ (`research/sherlock-lookup-cache-20261111/CONSENSUS.md:155-182`), Tier-1 executes in two passes: **Pass 1** streams execution, writes segment-store records, and computes each leaf's KZG commitment $C_i = \text{Com}(P_i, r_i)$ (~1 MSM of size $2^{13..16}$ per leaf), accumulating the commitment Merkle root `com_root` in `O(log L)` RAM; **Pass 2** derives $(\gamma_{1..2}, \alpha_{1..2}) = \text{PoseidonFS}(\text{com\_root})$ and dispatches embarrassingly parallel `U`, `R`, and `N(SegMerge)` jobs to the `3×6t` pool.
3. **Versioned GP Bundle Output (`SRC-019`, `SRC-010`, `requirements.md:35,44`):**
   - Every completed `U`, `R`, `F`, `N`, `C_close`, and `D` proof is flushed before RAM eviction into an append-only bundle directory bound by one `bundle_manifest.json` SHA-256 hash (`magic = "sherlock-gp-bundle/1"`, key manifests per `SRC-008`, DAG edge table, and public statement) so an out-of-process verifier (`SRC-010`) reconstructs every accumulator link prior to any network call (`SRC-020`).

#### Mode 1: Production Default — Retained-Store `SEG` Checkpoint & Resume (`sherlock-seg-ckpt/1`)
When a long run stops (operator pause, maintenance window, or wall-cap split) and resumes under the same key set (`GP-001`, `requirements.md:56`):
- **Persisted Artifact (`SherlockSegCheckpointV1`, `magic = b"SHKSCK01"`):**
  1. `key_set_manifest_sha256: [u8; 32]` (`SRC-008` — pins `{U, R, F, N, C_close, D}` keys and `BoundaryProfileKind`).
  2. `prog_d: F`, `prog_root: F`, `input_d: F` (`[Fq; 3]`).
  3. `h_lo: u64` (`== 0` for a root prefix checkpoint) and `h_hi: u64` (`1 <= h_hi < 2^64`, exact step count proved so far).
  4. `post_arch_state_bytes: Vec<u8>` (the 39 logical/register carrier fields at `h_hi`, `design.md` §3.2(5), plus active tail buffer).
  5. `segment_store_snapshot: SegmentStoreHandle` (content-addressed snapshot of the committed external page/arena store `DATA`, `FIELDS`, `ENV`, `SLOTS`, `FRAMES`, `TASKS` matching `post_boundary`).
  6. `deps_d_post: F` (running digest of `GP-2` claims consumed in `[0, h_hi)`).
  7. `open_seg_stack: Vec<ProvenSegArtifact>` (`1 ..= 64` proven `SEG` proofs + serialized KZG accumulators covering `[0, h_hi)` — collapsed into a single `SEG` proof if flushed on pause).
  8. `bundle_prefix_manifest_sha256: [u8; 32]` (`SRC-019` audit lineage for `[0, h_hi)`).
- **Exact Resume & Composition Rule (`AUDIT.md:39-42`):**
  1. The resuming prover verifies `key_set_manifest_sha256`, re-checks `open_seg_stack` via the standalone `SRC-010` verifier, restores the host interpreter at `h_pre = ckpt.h_hi` against `segment_store_snapshot` (`0` loader steps, `0` loader leaves!), and continues proving leaves `[h_hi, N)` into `SEG_suffix`.
  2. Prefix and suffix combine **strictly via `N(SegMerge): SEG_prefix + SEG_suffix -> SEG_full`**, followed by a single terminal seal `N(SegSealUnary | SegSealResolve): SEG_full -> SET` once the machine halts/rejects/exhausts at step `N`.
  3. **Hard Prohibition (`GP-001`, `GP-002`, `AUDIT.md:39-42`):** `SherlockSegCheckpointV1` carries `term_tag == 0` (`Running`) and physical `post_boundary`; it **MUST NEVER** be sealed into `SET`, submitted to `DECIDER`, or consumed by `VerifySherlockClaim` (`GP-2`). Under `Profile B`, Mode-1 pause/resume before termination snapshots the Pass-1 `Instance(0)` commitment log + witness store (deferring Pass-2 challenge squeezing until termination) because `Profile B` challenges bind the complete run's `com_root`.

#### Mode 2: Open-Research Extension Hook — Portable Value-Only `sherlock-ckpt/1` + Lazy `UNFOLD` (`AUDIT.md:33-38`)
Mode 2 aims to let a run pause, discard its physical Merkle/LogUp store (`len`, `fp`, `tp`, arena addresses), serialize only address-free `sherlock-value/1` (`vd`) digests (`sherlock-ckpt/1`), and resume in a fresh arena via lazy `UNFOLD` micro-operations (`research/sherlock-runlength-consensus-20261111/CONSENSUS.md:216-235`). Per `AUDIT.md:33-38`, Mode 2 is **NOT** part of M4 or the frozen core key set because three concrete design problems remain open:
1. **`m > 8` Slice Materialization:** `sherlock-k-a/1` constructors, closures, and slot frames can exceed 8 contiguous cells (`m > 8`); authenticating a multi-chunk `UNFOLD` against a single `vd` inside Area 1/2's fixed 15-row step window without stateful partial-unfold hazards is unspecified.
2. **`n_steps` Preservation (`GP-001`, `SRC-023`):** `GP-001` (`requirements.md:56`) mandates that a stopped-and-resumed run produce the **exact same `n_steps`** as an uninterrupted run. If `UNFOLD` consumes step height `h`, `n_steps` changes with checkpoint frequency (violating `GP-001` and `SRC-023`); if `UNFOLD` runs at `delta_h = 0`, a separate termination/progress measure is required so zero-height steps cannot loop or forge state transitions.
3. **39-Field Live-Root Coverage & `Profile B` Epoch Split:** Proving that a portable digest covers all 39 register/stack/task roots without an `O(arena)` scan at pause time, and closing a `Profile B` LogUp epoch at `Running` without exposing physical state as a `SET` claim, requires dedicated post-M4 circuits and Lean proofs.

Accordingly, `M9` exposes only a quarantined `ExperimentalPortableCheckpoint` marker trait gated behind `#[cfg(feature = "experimental-mode2-ckpt")]` (disabled in workspace default features and refused by `SRC-008` release manifests).

### 3.3 Invariants

**Guaranteed by `M9`:**
1. **Statement & Claim Equivalence Under Stop/Resume (`INV-CONT-EQUIV`, `GP-001`, `SRC-023`):** Proving a deterministic `N`-step run continuously in one session or across $K \ge 1$ Mode-1 `SherlockSegCheckpointV1` pauses produces bit-identical `(prog_d, input_d, outcome_d, n_steps)` and `claim_d`.
2. **Bounded `O(log L)` Prover RAM & Kernel Cgroup Enforcement (`INV-CONT-MEM`, `SRC-016`):** Active in-RAM `SEG` proofs never exceed `64` entries (`\lceil \log_2 L \rceil + 2`), per-worker RSS stays `<= 48 GiB`, and aggregate worker RSS stays `<= 100 GiB` enforced by `MemoryMax=100G`.
3. **Non-Acceptance of Checkpoints as Runs or `GP-2` Claims (`INV-CONT-NO-SEAL`, `GP-001`, `GP-002`):** Every Mode-1 checkpoint artifact carries `stmt_tag == TAG_SEG` and `term_tag == 0`; neither `Node` seal (`C`/`Resolve`) nor `DECIDER` accepts an unsealed checkpoint.
4. **Complete `SRC-019` Lineage Across Resume (`INV-CONT-BUNDLE`):** Resuming from `SherlockSegCheckpointV1` splices the prefix `SRC-019` bundle and suffix proofs into one hash-bound `sherlock-gp-bundle/1` manifest verified end-to-end by `SRC-010`.

**Assumed from Other Modules:**
1. **From Area 2 (`M5a`/`M5b`):** `DensePageBackend` (or `ZmLogUpBackend` Pass-1 log) implements deterministic snapshot serialization/deserialization keyed by content hash.
2. **From `M10: NODE INTEGRATION`:** Associativity of `N(SegMerge): SEG + SEG -> SEG` (`L-NODE-1`) so off-power-of-two pause points merge with `SEG_suffix` without altering `SEG` semantics.

### 3.4 Row Budget (`ESTIMATE`)

`M9` is a host orchestration, scheduling, and serialization module — it adds **`0` new PLONK gates or lookup rows** to `U`, `R`, `F`, `N`, `C_close`, or `D`:

| Metric | Value (`ESTIMATE` unless noted) | Basis / Cap |
|---|---|---|
| Additional circuit / table rows in `U`, `R`, `N`, `D` | **`+0` rows** (exact) | Uses `M10`'s existing `N(SegMerge): SEG + SEG -> SEG` |
| Loader leaves emitted on Mode-1 resume | **`0` leaves** (exact) | Retained `SegmentStoreHandle` + `post_boundary` links directly via `SegMerge` |
| Extra `Node` proofs per Mode-1 pause/resume | **`0` proofs** if paused at a `2^r` leaf boundary; **`<= 1` `N(SegMerge)` proof** (`~760k–855k` rows `ESTIMATE`) if flushed mid-tree | Binary carry-stack flush |
| Open `SEG` stack RAM footprint (`L < 2^64` leaves) | **`<= 64` entries × `~12 KiB` = `< 768 KiB` `ESTIMATE`** | `~6.4 KiB` proof + `~4 KiB` serialized accumulator + metadata |
| Concurrent `3×6t` Worker Pool Peak Aggregate RSS | **`~50 – 60 GiB` `ESTIMATE`** (`3 × 16.6–19.5 GiB` node workers) | Kernel-enforced `<= 100 GiB` (`SRC-016`), `< 48 GiB` per process |
| `fact(3)` (`293` steps, `19` leaves at `S_eff = 15.4`, `Profile A`) End-to-End Makespan | **`~15 – 22 min` `ESTIMATE`** (leaves + `10` `R` + `10` `N` on `3×6t` + `1` `RunDecider` + `SRC-010` + `SRC-020`) | Seeded cap `< 120 min` (`SRC-011`, `SRC-015`) |

### 3.5 Verification

1. **Quint Specification (`models/sherlock/quint/continuation_scheduler.qnt`):**
   - Models streaming leaf arrival, asynchronous `3`-worker completion out of order, `O(log L)` rank-stack folding, arbitrary Mode-1 pause/resume cuts `0 < h_1 < h_2 < N`, and `SRC-019` bundle manifest assembly.
   - Verifies invariants: `MaxStackDepthLeLogLPlus2`, `ResumeProducesIdenticalClaimD`, `CheckpointRejectedBySealAndGP2`, and `OutOfOrderWorkerCompletionPreservesSegInterval`.
2. **Positive Integration Tests (`crates/mori-recursion/tests/continuation_mode1.rs`):**
   - `pos_tier1_streaming_stack_bound`: feeds `1,000` mock leaf receipts through `SegmentTreeScheduler` with randomized worker latencies, asserting max live stack len `<= 11`, valid `SRC-019` bundle hash, and identical root `SEG` interval `[0, 16_000)`.
   - `pos_mode1_stop_resume_twice_allocating_run` (`GP-001` acceptance): runs an allocating program (`> 1` leaf, building a `> 8`-field constructor across a leaf boundary) uninterrupted and again with two Mode-1 checkpoints (one at a power-of-two leaf boundary, one at an odd leaf `L = 3`), confirming `0` loader leaves and identical `[prog_d, input_d, outcome_d, n_steps]` and `claim_d`.
3. **Named Negative Mutants:**
   - `mut_ckpt_fed_as_gp2_claim`: attempt to pass `SherlockSegCheckpointV1` into `Node` `SegSealResolve` (`GP-2`) or `RunDecider` -> refused at type/tag check and unsatisfiable in circuit (`GP-001`, `GP-002`, `AUDIT.md:39-42`).
   - `mut_ckpt_tampered_store_cell`: flip one word in `segment_store_snapshot` while keeping `post_boundary` -> suffix leaf #0 fails `BoundaryCommitment` / Merkle root opening in `MockProver`.
   - `mut_ckpt_mismatched_key_manifest`: resume checkpoint under a modified `S_MAX` or `leaf_vk` -> refused at preflight (`SRC-008`, `SAS-011`) and rejected by `VerifierGadget` transcript (`MC:verifier/verifier_gadget.rs:296`).
   - `mut_bundle_missing_intermediate_proof`: delete one intermediate `R` proof from the `SRC-019` bundle -> refused by standalone `SRC-010` bundle verifier (`requirements.md:35,44`).

### 3.6 Lean Obligations (`ARCH-BRIEF.md:52-60`)

| Obligation | Plain-English Theorem Statement | Lean File Extended / Created | Axioms in `AggregationAxioms.lean` Relied On | Module Contract Discharged |
|---|---|---|---|---|
| **L-CONT-1 (Obligation 5b): Mode-1 Segment Continuation Equivalence & Non-Sealability** | For any valid leaf sequence `ls = ls_prefix ++ ls_suffix` (`ls_prefix != []`, `ls_suffix != []`), folding `ls_prefix` into `seg_pre : SegCert`, serializing/restoring `(seg_pre, store_at_split)`, folding `ls_suffix` into `seg_suf : SegCert`, and merging `SegCert.mergeSeg seg_pre seg_suf` yields a valid `SegCert` with the exact same `(prog_d, input_d, pre_0, post_N, 0, N)` — and after `seal`, the exact same `outcome_d`, `n_steps`, and `claim_d` — as uninterrupted folding `foldAll ls`; furthermore, no unsealed `seg_pre` (`term_tag = Running`) has a constructor in `SetCert` or `Decided` (`unsealed_ckpt_not_claim`). | Extends `clean/fork/Clean/Sherlock/Aggregation.lean` (`same_leaves_same_stmt` at `:708` lifted to binary `RunLift` + `SegCert`) and `clean/fork/Clean/Sherlock/ClaimDag.lean` (`mode1_resume_eq_uninterrupted`, `unsealed_ckpt_not_claim`). | `Premises` (`det`, `leaf_sound`, `digest_inj`) and `AccAlgebra` (`AggregationAxioms.lean:45-175`). | `INV-CONT-EQUIV`, `INV-CONT-NO-SEAL`, `GP-001`, `GP-002`, `AUDIT.md:39-42`. |

### 3.7 Code Skeleton

Owned files: `crates/mori-recursion/src/{scheduler.rs, checkpoint.rs, bundle.rs}`.

```rust
// crates/mori-recursion/src/scheduler.rs
//! Tier-1 streaming binary segment-tree scheduler (`O(log L)` RAM carry stack, `3x6t` pool).

use std::path::PathBuf;
use midnight_curves::Fq as F;
use crate::boundary::InCircuitBoundaryLink;

#[derive(Clone, Debug)]
pub struct SchedulerConfig {
    pub worker_processes: usize,       // default: 3
    pub threads_per_worker: usize,     // default: 6 (3 * 6 = 18 cores; 2 cores for witness + I/O)
    pub per_worker_rss_cap_gib: usize, // <= 48 GiB (`SRC-016`)
    pub aggregate_rss_cap_gib: usize,  // <= 100 GiB (`SRC-016`)
    pub bundle_dir: PathBuf,           // `SRC-019` append-only bundle output
}

#[derive(Clone, Debug)]
pub struct ProvenSegArtifact {
    pub rank: u32,
    pub h_lo: u64,
    pub h_hi: u64,
    pub shape_d: F,
    pub proof_rel_path: PathBuf,
    pub serialized_stmt_and_acc: Vec<u8>,
}

pub struct SegmentTreeScheduler<B: InCircuitBoundaryLink> {
    cfg: SchedulerConfig,
    carry_stack: Vec<Option<ProvenSegArtifact>>, // Invariant: len <= 64 (`O(log L)`)
    _profile: std::marker::PhantomData<B>,
}

impl<B: InCircuitBoundaryLink> SegmentTreeScheduler<B> {
    /// Requires: `cfg.worker_processes * cfg.per_worker_rss_cap_gib` honors `SRC-016` (`<= 100 GiB` scope).
    /// Ensures: Initializes empty `O(log L)` carry stack and opens `SRC-019` `bundle_dir`.
    pub fn new(cfg: SchedulerConfig) -> Result<Self, SchedulerError>;

    /// Requires: `leaf_chunk` is the next contiguous witness chunk (`h_pre == current_h_hi`, `1 <= steps <= S`).
    /// Ensures: Flushes completed proofs to `bundle_dir`, folds adjacent pairs via `RunLift` (`R`) and equal-rank `SEG`s via `Node` (`SegMerge`), keeping `carry_stack.iter().flatten().count() <= 64`.
    pub fn push_leaf_chunk(&mut self, leaf_chunk: LeafWitnessChunk<B>) -> Result<(), SchedulerError>;

    /// Requires: At least one leaf chunk pushed; run is pausing while still `Running` (`term_tag == 0`).
    /// Ensures: Collapses `carry_stack` into `SherlockSegCheckpointV1` + retained store snapshot (`0` loader leaves; NEVER seals into `SET` or `GP-2` claim).
    pub fn pause_mode1_checkpoint(
        self,
        store_snapshot: SegmentStoreHandle,
        post_arch_state_bytes: Vec<u8>,
    ) -> Result<crate::checkpoint::SherlockSegCheckpointV1, SchedulerError>;

    /// Requires: Last pushed leaf chunk reached terminal outcome (`term_tag in {1, 2, 3}`) and `h_lo == 0`.
    /// Ensures: Folds remaining `carry_stack` via `SegMerge`, executes `SegSealUnary` or `SegSealResolve`, runs `ClosingRelation` (if GP mode) + `DECIDER`, and finalizes `SRC-019` manifest.
    pub fn finish_and_decide(
        self,
        consumed_deps_set: Option<ProvenSetArtifact>,
    ) -> Result<SealedBundleReceipt, SchedulerError>;
}

// crates/mori-recursion/src/checkpoint.rs
//! Mode-1 retained-store `SEG` checkpoint (`SHKSCK01`) and quarantined Mode-2 research stub.

pub const MODE1_CKPT_MAGIC: &[u8; 8] = b"SHKSCK01";

#[derive(Clone, Debug)]
pub struct SherlockSegCheckpointV1 {
    pub magic: [u8; 8],
    pub key_set_manifest_sha256: [u8; 32],
    pub prog_d: F,
    pub prog_root: F,
    pub input_d: F,
    pub h_lo: u64,
    pub h_hi: u64,
    pub deps_d_post: F,
    pub post_arch_state_bytes: Vec<u8>,
    pub segment_store_snapshot: SegmentStoreHandle,
    pub prefix_seg: crate::scheduler::ProvenSegArtifact,
    pub bundle_prefix_manifest_sha256: [u8; 32],
}

impl SherlockSegCheckpointV1 {
    /// Requires: `ckpt` has `magic == MODE1_CKPT_MAGIC`, `h_lo == 0`, `h_hi > 0`, and passes standalone `SRC-010` verification against `expected_key_manifest`.
    /// Ensures: Seeds `SegmentTreeScheduler` with `ckpt.prefix_seg` so suffix leaves `[h_hi, N)` merge strictly via `N(SegMerge): SEG + SEG -> SEG` with `0` loader leaves (`AUDIT.md:39-42`).
    pub fn resume_scheduler<B: InCircuitBoundaryLink>(
        self,
        cfg: crate::scheduler::SchedulerConfig,
        expected_key_manifest: &[u8; 32],
    ) -> Result<(crate::scheduler::SegmentTreeScheduler<B>, RestoredHostSession), CheckpointError>;
}

/// Open-research hook ONLY (`AUDIT.md:33-38`): disabled in production builds until `m > 8` slice
/// `UNFOLD`, `delta_h == 0` `n_steps` invariance, and 39-field root coverage are formally solved.
#[cfg(feature = "experimental-mode2-ckpt")]
pub trait ExperimentalPortableValueCheckpoint {
    fn export_portable_ckpt_v1(&self) -> Result<Vec<u8>, CheckpointError>;
}
```

### 3.8 Agent Context Boundary

- **MUST read (~800 lines total):**
  1. This module kit (§3.1–§3.9), `RunLiftRelation` / `PolymorphicNodeRelation` signatures (§1.7), and Area 2's `MemBackend` snapshot trait (`crates/mori-chip/src/membackend.rs:1-120`).
  2. `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:35-44,56-57` (`SRC-010..019`, `GP-001`, `GP-002`) and `research/sherlock-runlength-consensus-20261111/AUDIT.md:19-45`.
- **MUST NOT need to read:**
  - `VerifierGadget`, `WeierstrassChip`, `PairingChip`, or `Pow2RangeChip` internals.
  - Area 1 processor gates (`CORE`, `DEC`, `ALU`, `HASH`) or Compact contract code.

### 3.9 OpenSpec 1.17.0 Capability Spec (`specs/sherlock-continuation/spec.md`)

```markdown
# Capability: sherlock-continuation

## Requirements

### Requirement: Bounded-RAM Parallel Segment-Tree Scheduling (`MOD-CONT-001`)
`SegmentTreeScheduler` SHALL fold leaf chunks through `RunLift` and binary `Node` (`SegMerge`) using at most `64` in-RAM `SEG` entries (`O(log L)`), kernel-enforced `MemoryMax <= 100 GiB` aggregate (`<= 48 GiB` per worker), and append-only `SRC-019` disk bundle persistence.

#### Scenario: Multi-leaf run completes within `O(log L)` carry stack and writes full GP bundle
- **WHEN** `SegmentTreeScheduler` processes `L >= 19` leaves concurrently across `3x6t` workers
- **THEN** live in-RAM `SEG` count never exceeds `ceil(log2(L)) + 2` and standalone `SRC-010` verification accepts the persisted bundle

### Requirement: Mode-1 Retained-Store Segment Checkpoint and Resume (`MOD-CONT-002`)
`SherlockSegCheckpointV1` SHALL pause an active run by persisting its proven `SEG` prefix and retained segment store, and SHALL resume at `h_pre = ckpt.h_hi` with `0` loader leaves via `Node` `SegMerge` (`SEG + SEG -> SEG`), producing identical `[prog_d, input_d, outcome_d, n_steps]` and `claim_d` as an uninterrupted run.

#### Scenario: Allocating multi-leaf run stopped and resumed matches uninterrupted `claim_d`
- **WHEN** a run allocating across leaf boundaries (`> 8`-field constructor) pauses via `pause_mode1_checkpoint`, restores, and finishes
- **THEN** `0` loader leaves are proved and `(prog_d, input_d, outcome_d, n_steps, claim_d)` equal the uninterrupted run (`GP-001`, Lean `L-CONT-1`)

#### Scenario: Unsealed Mode-1 checkpoint is refused as a run or `GP-2` claim
- **WHEN** a `SherlockSegCheckpointV1` (`term_tag == 0`) is passed to `RunDecider` or `Node` `SegSealResolve` (`GP-2`)
- **THEN** both refuse acceptance (`mut_ckpt_fed_as_gp2_claim`, `GP-001`, `GP-002`)
```

---

## 4. Module `M12: HARNESS` — Whole-Leaf & Whole-Run Differential Integration Harness Against `sherlock-k-a/1` (`MODULARITY.md:12`)

### 4.1 Purpose
`HARNESS` provides a single authoritative differential test and mutant-injection framework that executes every module change — both per-leaf (`ChipLeaf` under `MockProver`) and whole-run (`RunLift` + `Node` + `DECIDER` across multiple leaf partitions and Mode-1 checkpoints) — step-by-step against the independent reference machine `sherlock-k-a/1` (`crates/mori-core/src/sherlock/reference.rs`). It enforces `SAS-003` (panic/synthesis errors earn zero soundness credit), `SAS-004` (fresh malicious-witness mutants paired with weakened-constraint controls), `SAS-005` (complete ordered step-by-step logical projection, not final return value alone), and `SRC-023` (partition invariance across `Profile A` and `Profile B`).

### 4.2 Frozen Interface

#### Reference Oracle & Step-by-Step Logical Projection (`SAS-005`, `SVM-011`)
The harness drives two completely separate producers from the same `(ProgramImage, RunInput)` pair (`SVM-007`, `requirements.md:15`):
1. **Reference Oracle (`Ka1ReferenceRunner`):** Wraps `mori_core::sherlock::reference::Machine` (`sherlock-k-a/1` + `sherlock-helper-microstep/1` event table, `SVM-001`) with **zero knowledge** of leaf boundaries, `WindowCfg`, Merkle trees, LogUp polynomials, or physical arena addresses (`SVM-010`, `SVM-011`).
2. **Prover Under Test (`ModularProverRunner<M, B>`):** Instantiates `ChipLeaf<M, B>` over any combination of Area 2 backends (`(DensePageBackend, PoseidonBoundary)` = `Profile A` or `(ZmLogUpBackend, HornerBoundary)` = `Profile B`) and partition policy `PartitionSchedule`.

After **every enabled slot `s` (`0 <= s < steps`)** and at **every leaf close**, both sides emit an address-erased `LogicalStepObservation`:
- `step_height: u64` (`h_pre + s + 1`),
- `fired_rule_lane: u16` (`rid_s` in `1..=|R|`, matching `SRC-001`),
- `control_tag: u8` (`Eval`, `Apply`, `Return`, `Helper`, `Halted`, `Rejected`, `Exhausted`),
- `fuel_remaining: u128` and `stack_depth: u64` (checking `SVM-010` that leaf closes leave fuel and logical counters untouched),
- `top_value_vd: Option<F>` (`sherlock-value/1` canonical digest whenever a value is created or returned, `SVM-012`),
- `consumed_deps_d: F`,
- `terminal_outcome_d: Option<F>` (`SRC-022`).

#### Multi-Partition `SRC-023` Matrix & Two-Tier Gate Architecture
Every integration run executes each corpus vector `(P1: entry 7, P2: fact(3), P3: list123, P4: input_apply, P5: ctor_gt8_cross_leaf, P6: shared_vs_dag_dup_vd, P7: rejected_all_codes, P8: exhausted_fuel_and_stack)` across **four mandatory schedules**:
- `PartitionSchedule::UniformFull` (`[S, S, ..., rem]`, default `S = 16`),
- `PartitionSchedule::AdversarialEarlyClose` (cycling `[1, 3, 7, 16]`, forcing early leaf closes mid-helper, mid-`>8`-field constructor build, and mid-frame-unwind),
- `PartitionSchedule::SingleStep` (`[1, 1, ..., 1]`, worst-case boundary stress),
- `PartitionSchedule::Mode1StopResume` (`[1, 3, 7, 16]` paused after leaf `2` and resumed via `SherlockSegCheckpointV1`).

Gate execution splits into two tiers (`SRC-016`, `SWF-004`):
- **Tier A — Fast PR / Module-Change Gate (`cargo test -p mori-chip --test diff_leaf_vs_sherlock_ka1`, `RUST_TEST_THREADS=4`, `< 60 s`):** Runs `MockProver::run(k_leaf, &leaf_circuit, instances)` on every leaf of all 8 corpus vectors across all 4 schedules for **both** `Profile A` and `Profile B` (plus host-simulated `Node` linkage + `Pow2Range` `table_rows == 1,032,467` `cost_model` check), catching any Area 1/2/3 regression in seconds without SRS keygen.
- **Tier B — Heavy Recursion Gate (`scripts/doctorwatson heavy` running `diff_run_vs_sherlock_ka1`, `#[ignore]` in plain `cargo test` per `SRC-016`):** Proves real KZG `ChipLeaf` -> `RunLift` -> `Node` -> `RunDecider` / `ClaimDecider` bundles on `bls_midnight_2p20`, runs the out-of-process `SRC-010` bundle verifier, and executes `SRC-020` `validate_call` against `ledger-9.1.0.1` (`0.8.2` verifier), asserting that `UniformFull`, `AdversarialEarlyClose`, `SingleStep`, and `Mode1StopResume` emit **identical `[prog_d, input_d, outcome_d, n_steps]` and `claim_d`** and that submitting the second partition's proof to `SherlockRun` is refused as a `claim_d` replay (`SRC-007`, `SRC-023`).

#### `SAS-003` / `SAS-004` Mutant Campaign Engine
Every module kit registers its named `MutantSpec` entries in `HARNESS`. For each mutant `m`, `MutantCampaignRunner` enforces:
1. **Valid Baseline First (`SAS-003`):** Unmutated witness passes `MockProver::verify_par()` (`Ok(())`).
2. **Exact Verifier-Stage Attribution (`SAS-003`):** If applying `m` causes a Rust panic, `Error::Synthesis`, decoder refusal, or timeout, `HARNESS` classifies `m` as `UntestedStage(PanicOrSynthesisError)` and **fails the gate** — only `Err(VerifyFailure::ConstraintNotSatisfied | Lookup | Permutation | CellNotAssigned)` (or `Pairing`/`Transcript` failure at the targeted KZG layer) earns `ConstraintRejected` credit!
3. **Weakened-Constraint Control (`SAS-004`):** Every constraint-closing mutant campaign supplies a `ControlToggle` (a test-only circuit configuration flag disabling the specific gate under test) and proves that with `ControlToggle` active, the malicious witness **passes** `MockProver`, proving the mutant genuinely tests that constraint rather than an accidental upstream invariant.

### 4.3 Invariants

**Guaranteed by `M12`:**
1. **Zero silent semantic drift (`INV-HARN-DIFF`, `SAS-005`, `SVM-011`):** Any divergence between `ChipLeaf` and `sherlock-k-a/1` in rule selection, fuel/depth accounting, intermediate `vd`, or terminal `outcome_d` — even if final `Halted` scalar output accidentally coincides — fails the per-step `LogicalStepObservation` diff.
2. **Partition & Backend Equivalence (`INV-HARN-SRC023`, `SRC-023`):** `Profile A` and `Profile B` under all four partition schedules yield bit-equal `LogicalStepObservation` traces and bit-equal `(prog_d, input_d, outcome_d, n_steps, claim_d)`.
3. **Strict `SAS-003` / `SAS-004` Soundness Accounting (`INV-HARN-MUT`):** Zero panics or `SynthesisError` accepted as negative-test passes; zero public-input-only flips misclassified as raw-witness constraint checks.
4. **Fenrir-Free Isolation (`INV-HARN-NO-FENRIR`, `SWF-014`):** Zero corpus fixtures or test helpers reference external paths (`projectfenrir` or `rollup-subarch`); all vectors are generated inside `crates/mori-core` / `crates/mori-chip`.

**Assumed from Other Modules:**
1. **From Areas 1–3 (`B0..M11`):** Every module exposes a deterministic witness-mutator hook (`#[cfg(any(test, feature = "qa-mutants"))]`) allowing raw post-witness cell overrides *after* honest witness generation so synthesis never aborts early (`SAS-003`).

### 4.4 Row Budget (`ESTIMATE`)

`M12` is test/qualification infrastructure only (`#[cfg(test)]` / `crates/mori-chip/src/harness.rs` dev-support) and adds **`0` rows, `0` columns, and `0` gates** to any keyed circuit:

| Gate Tier | Scope | Runtime & Memory (`ESTIMATE`) | Enforcement |
|---|---|---|---|
| Tier A: `diff_leaf_vs_sherlock_ka1` (`MockProver`, `k = 13..15`) | 8 corpus vectors × 4 schedules × 2 profiles (`A` & `B`) + `cost_model` row/table gates + per-module `MutantSpec` suite | **`~25 – 55 s` wall `ESTIMATE`**, **`< 4 GiB` RSS** (`RUST_TEST_THREADS=4`) | Runs on **every module PR / edit** (`MODULARITY.md:12`) |
| Tier B: `diff_run_vs_sherlock_ka1` (`bls_midnight_2p20` real KZG + `SRC-010` + `SRC-020`) | `entry 7` (4 partitions + Mode-1 resume + replay refusal) + `fact(3)` + `list123` + `GP-2/3/4` DAG acceptance | `entry 7`: **`< 20 min`**; `fact(3)` / `list123`: **`~15 – 25 min` `ESTIMATE`** (`< 2 h` cap), **`<= 100 GiB` aggregate RSS** | Runs via `scripts/doctorwatson heavy` before milestone gates (`SRC-015`, `SRC-016`) |

### 4.5 Verification

1. **Quint Trace-Projection Model (`models/sherlock/quint/harness_projection.qnt`):**
   - Verifies that `LogicalStepObservation` is a complete distinguishing bisimulation projection for `SVM-010`, `SVM-011`, `SVM-012`, and `SRC-023` (seeding an intermediate fuel-glitch mutant whose final `Halted` result still matches, confirming `SAS-005` detects it at the exact step).
2. **Positive Corpus Self-Tests (`crates/mori-chip/tests/diff_leaf_vs_sherlock_ka1.rs`):**
   - Runs `P1..P8` end-to-end against `sherlock-k-a/1`, asserting `0` observation diffs, `vd(shared_dag) == vd(duplicated_tree)` (`P6`, `SRC-023`), and `> 8`-field constructor assembly across a forced leaf cut (`P5`, `GP-001`).
3. **Named Meta-Mutants (testing the harness itself per `SAS-003`..`SAS-005`):**
   - `meta_mut_panic_in_synthesis_rejected`: injects a `panic!()` mutant -> confirms `MutantCampaignRunner` returns `Err(HarnessFailure::PanicMaskingConstraint)` (`SAS-003`).
   - `meta_mut_missing_weakened_control_rejected`: registers a mutant whose `ControlToggle` still fails `MockProver` -> confirms `MutantCampaignRunner` flags `PartialUncorroboratedControl` (`SAS-004`).
   - `meta_mut_intermediate_fuel_drift_same_result`: alters intermediate `fuel_lo` on step 2 and restores it on step 3 -> caught by `SAS-005` step-2 projection diff.
   - `meta_mut_address_leak_into_vd`: XORs arena `addr` into `vd` -> caught immediately by `AdversarialEarlyClose` vs `UniformFull` `SRC-023` comparison.

### 4.6 Lean Obligations (`ARCH-BRIEF.md:52-60`)

| Obligation | Plain-English Theorem Statement | Lean File Extended / Created | Axioms in `AggregationAxioms.lean` Relied On | Module Contract Discharged |
|---|---|---|---|---|
| **L-HARN-1: Observational Projection Adequacy (`SAS-005`, `SVM-011`)** | Define `projectObs : LogicalState -> LogicalStepObservation` in Lean matching `harness.rs::LogicalStepObservation` field-for-field, and prove in `ModularRefinement.lean` that if a chip leaf trace `t_chip` satisfies `BusContract` and emits per-step observations equal to `List.map projectObs t_ref` of `sherlock-k-a/1`, then terminal `outcomeDigest` and `n_steps` equal `sherlock-k-a/1` and leaf-close transitions (`enable = 0` / boundary rollover) satisfy `projectObs post = projectObs pre`. | Extends `clean/fork/Clean/Sherlock/ModularRefinement.lean` (`obs_projection_adequate`, `leaf_close_preserves_logical`). | `Premises` (`AggregationAxioms.lean:45-95`) and `vd_canonical` (Area 2 Obligation 4). | `INV-HARN-DIFF`, `INV-HARN-SRC023`, `SAS-005`, `SVM-010`, `SVM-011`. |

### 4.7 Code Skeleton

Owned files: `crates/mori-chip/src/harness.rs`, `crates/mori-chip/tests/diff_leaf_vs_sherlock_ka1.rs`, `crates/mori-recursion/tests/diff_run_vs_sherlock_ka1.rs`.

```rust
// crates/mori-chip/src/harness.rs
//! Whole-leaf & whole-run differential oracle against `sherlock-k-a/1` and `SAS-003/004` mutant runner.

use midnight_curves::Fq as F;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogicalStepObservation {
    pub step_height: u64,
    pub fired_rule_lane: u16,
    pub control_tag: u8,
    pub fuel_remaining: u128,
    pub stack_depth: u64,
    pub top_value_vd: Option<F>,
    pub consumed_deps_d: F,
    pub terminal_outcome_d: Option<F>,
}

#[derive(Clone, Debug)]
pub enum PartitionSchedule {
    UniformFull { s: usize },                // [16, 16, ...]
    AdversarialEarlyClose { cycle: Vec<usize> }, // [1, 3, 7, 16]
    SingleStep,                              // [1, 1, ...]
    Mode1StopResume { cycle: Vec<usize>, pause_after_leaf: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReachedRejectionStage {
    MockProverGateConstraint,
    MockProverLookup,
    MockProverPermutation,
    KzgPairingOrTranscript,
}

pub struct MutantSpec<C> {
    pub id: &'static str,
    pub module: &'static str,
    pub target_stage: ReachedRejectionStage,
    /// Mutates assigned post-synthesis raw witness cells (`SAS-003`: MUST NOT panic or fail synthesis).
    pub apply_raw_witness_mutation: fn(&mut C),
    /// Weakened-constraint circuit factory (`SAS-004`: MUST allow `apply_raw_witness_mutation` to PASS).
    pub build_weakened_control_circuit: fn(&C) -> C,
}

/// Requires: `prog` and `input` are valid `sherlock-helper-microstep/1` inputs; `schedules.len() >= 3`.
/// Ensures: Steps `sherlock-k-a/1` once to obtain expected `Vec<LogicalStepObservation>`, runs `MockProver` on every leaf of every schedule, asserts step-by-step observation equality (`SAS-005`), leaf-close counter invariance (`SVM-010`), and cross-schedule `(prog_d, input_d, outcome_d, n_steps, claim_d)` equality (`SRC-023`).
pub fn assert_leaf_differential_matrix<M, B>(
    prog: &mori_core::sherlock::ProgramImage,
    input: &mori_core::sherlock::RunInput,
    schedules: &[PartitionSchedule],
) -> Result<crate::harness::DifferentialReceipt, HarnessError>;

/// Requires: `baseline_circuit` passes `MockProver` (`Ok(())`).
/// Ensures: Refuses `Panic`, `Error::Synthesis`, or failed weakened-control (`SAS-003`, `SAS-004`); returns `MutantReceipt` only when baseline passes, weakened control passes with mutant, and production circuit fails at `spec.target_stage`.
pub fn run_mutant_spec<C: midnight_proofs::plonk::Circuit<F> + Clone>(
    k: u32,
    baseline_circuit: &C,
    instances: &[Vec<F>],
    spec: &MutantSpec<C>,
) -> Result<MutantReceipt, HarnessError>;
```

### 4.8 Agent Context Boundary

- **MUST read (~850 lines total):**
  1. This module kit (§4.1–§4.9), Area 1's `ChipLeaf` top-level `Circuit<F>` entry (`crates/mori-chip/src/leaf.rs:1-180`), and Area 2's `MemBackend` / `BoundaryCommitment` trait headers (`crates/mori-chip/src/membackend.rs:1-120`).
  2. `crates/mori-core/src/sherlock/reference.rs:1-220` (`sherlock-k-a/1` step interface) and `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:18-20,47-48,67-69` (`SVM-010..012`, `SRC-022..023`, `SAS-003..005`).
- **MUST NOT need to read:**
  - Internal polynomial/gate math of `ALU`, `DEC`, `HASH`, `PairingChip`, or Compact deployment scripts.

### 4.9 OpenSpec 1.17.0 Capability Spec (`specs/sherlock-harness/spec.md`)

```markdown
# Capability: sherlock-harness

## Requirements

### Requirement: Per-Step Differential Oracle Against `sherlock-k-a/1` (`MOD-HARN-001`)
`assert_leaf_differential_matrix` SHALL execute every corpus vector on `sherlock-k-a/1` and on `ChipLeaf` across `UniformFull(16)`, `AdversarialEarlyClose([1,3,7,16])`, `SingleStep`, and `Mode1StopResume`, verifying per-step `LogicalStepObservation` equality and identical `(prog_d, input_d, outcome_d, n_steps, claim_d)` across both `Profile A` and `Profile B`.

#### Scenario: Multi-partition differential matrix passes on every module change
- **WHEN** `diff_leaf_vs_sherlock_ka1` runs on a PR touching any Area 1–3 module
- **THEN** `MockProver` accepts every leaf across all four schedules and `0` per-step observation or `claim_d` diffs occur (`SAS-005`, `SRC-023`, Lean `L-HARN-1`)

#### Scenario: Address leak into `vd` or intermediate fuel drift is caught immediately
- **WHEN** a mutant alters `vd` by a physical address OR perturbs intermediate `fuel` while restoring final output
- **THEN** `assert_leaf_differential_matrix` fails at the exact divergent step (`meta_mut_address_leak_into_vd`, `meta_mut_intermediate_fuel_drift_same_result`)

### Requirement: Panic-Safe Raw-Witness Mutant Attribution (`MOD-HARN-002`)
`run_mutant_spec` SHALL require a passing baseline, a passing weakened-constraint control under the raw-witness mutation, and rejection at the declared `ReachedRejectionStage`, refusing credit on any panic, timeout, or `Error::Synthesis`.

#### Scenario: Synthesis panic earns zero constraint-rejection credit
- **WHEN** a negative mutant triggers a Rust panic or `Error::Synthesis` instead of `ConstraintNotSatisfied` / `Lookup`
- **THEN** `run_mutant_spec` returns `Err(PanicMaskingConstraint)` (`SAS-003`, `SAS-004`)
```
