# Lane D — Round 2: Cross-Review, Recursion & Wall-Clock Convergence, and Modular Architecture

Study: `sherlock-lookup-cache-20261111` (Round 2 — Cross-Review & Convergence).
Lead angle: Recursion Fit, `VerifierGadget` Mechanics, Prover Wall-Clock, System Interactions (`Q2`–`Q3`), and Modular Swappable Architecture (`MODULARITY.md`).
Repository state inspected: commit `9ffc2fe76f523453632cf3df03ca761095bc7c13` (`Cargo.toml:27-45` pins `midnight-ledger` tag `ledger-9.1.0.1`, `midnight-zkir` tag `zkir-3.2.0`, `midnight-curves = "=0.3.1"`, `midnight-proofs = "=0.8.3"`, `midnight-circuits = "=7.3.0"`, `midnight-zk-stdlib = "=2.4.0"`), including the Experiment C1 interface scaffold in `crates/mori-chip/src/membackend.rs:1-500` and `crates/mori-chip/src/dense.rs:1-756`.
General-purpose recursion rule enforced throughout: all of `GP-1` through `GP-4` (`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:1-58`, `requirements.md:56-59`) hold without narrowing any clause.

---

## 1. Cross-Review of Lanes A, B, C, and E (`round1.md`)

### 1.1 Summary Convergence Matrix

| Proposal / Mechanism | Source Lane(s) | Lane D Round 2 Verdict | Quantitative & Architectural Reason |
|---|---|---|---|
| **`M3-Tail64` lazy 8-cell tail carry** (`len = 8*q + r`, `tail_d` carry, flush to `TOP = 61` only when `r_pre + used >= 8` with `old_leaf == EMPTY_SEG != 0`) | `A`, Run-Length `CONSENSUS.md` (`membackend.rs`, `dense.rs`) | **ACCEPT (upgrades D's R1 partial-page RMW)** | Achieves $2^{64}$ dense cells (`GP-001`) with `0` top-tree Merkle walks when `r_pre + used < 8`; eliminates D's R1 old-prefix re-opening path (`-4,340` leaf rows/arena `ESTIMATE` on non-flushing leaves). |
| **`D7` register-cached `top` + Poseidon LIFO hash-chain stacks** (`fp_hash`, `tp_hash`) + slot-0 scratch zeroing (`v0=v1=v2=0`) | `A`, `C`, Run-Length `CONSENSUS.md` | **ACCEPT** | Moves strictly-LIFO `FRAMES` and `TASKS` off the random-access heap (`210–560` rows on cold spill vs `4,621` rows `ESTIMATE`) and enforces state canonicality (`Aggregation.lean:201` `digest_inj`). |
| **8-cell segment-granularity L1 `PAGE` + `ROM` cold cache on `bus`** (`q_prov = 1`, `act`-multiplied tuples) | `D` (`R1`), `C`, `A`, `E` | **ACCEPT (extends `dense.rs` `cold_read`)** | `dense.rs:716-752` currently costs `4,621` rows (`ESTIMATE`) per individual cold read port; caching an 8-cell cold segment on `bus` costs `5,889` rows (`ESTIMATE`) once to serve **all 8 adjacent records** (`~736` rows/cell) across unlimited hits in the leaf. |
| **Merged 2-column tagged `DEC + RANGE` fixed table** (`6,145` rows $< 8,192$, 1 `cs.lookup`, 2 `TableColumn`s, 54-bit `ctrl_word`) | `C` (`lanes/C/round1.md:175-186`) | **ACCEPT (beats `A`/`E`'s bus-folded `DEC`)** | Cuts leaf lookups `4 -> 3` and fixed columns `-1`, saving **`-29,062` Node rows (`823,466 -> ~794,404` `ESTIMATE`)** and **`-51,662` rows in `GP-4` `RunLift` `R` (`ESTIMATE`)** while consuming **zero** `CORE` `bus` request lanes (unlike `A`/`E`). |
| **64-bit schoolbook `OP_MUL` + `(SHK_VAL, 1)` sponge-prefix `vd` + `VD_{NIL,FALSE,TRUE}` constants** | `C` (`lanes/C/round1.md:163-166, 204-207`) | **ACCEPT** | Prevents $(2^{128}-1)^2 > r_{\text{BLS}}$ modular wraparound, shares `(lo64, hi64)` with `sherlock-value/1`, and saves `70–210` rows (`ESTIMATE`) per `vd` digest with `+0` Node rows. |
| **Explicit `u128` input `RANGE` check inside `ALU` (`Exploit D1`) + Red-Team bus/unpacking guards (`Exploits A1–A5, C1–C2, E1–E2, G1`)** | `E` (`lanes/E/round1.md:72-155, 215-218`) | **ACCEPT** | Closes `alu.rs:8-9, 111-125` unchecked `< 2^128` assumption (`OP_LT(r-1, 5) == 1`) at `+0` Node rows and `< 600` leaf rows (`ESTIMATE`); prevents `GHSA-fh29-29h9-qm9h` / `CVE-2026-62333` / `stwo-cairo` bug classes. |
| **`ZM-LogUp` (`Instance(0)` Stage-1 pre-commitment + binary `d_root` + `(gamma, alpha)` + Horner boundary)** | `B` (`lanes/B/round1.md:96-156, 208-216`) | **REJECT as Primary M4 Default; ACCEPT as Swappable Backend B** | Sound within one self-contained run on stock `VerifierGadget` (`phases == 1`), **but** breaks $O(1)$ Tier-2 checkpoint pause/resume (`O(|M|)` re-import across epochs), pays `+33.6 min` (`ESTIMATE`) of Node folds for 32 `INIT_PROG` loader leaves, adds `+18k..34k` rows (`ESTIMATE`) in `RunLift` `R`, kills prove-while-executing, and saves `< 2%` net wall-clock vs `M3-Tail64`. |
| **Folding `DEC` onto `bus` (`TAG_DEC`)** | `A` (`:18,125`), `E` (`:99-101, 268`) | **REJECT (superseded by C's tagged fixed table)** | Consumes 1 `CORE` `bus` request lane on every active step (`16` requests/leaf at `S=16`), forcing an extra request row in `CORE`; C's merged `DEC+RANGE` gets the same `-1` lookup + `-1` fixed col for free on `bus`. |
| **`TOP = 30..32` ($2^{32}..2^{35}$ cells) heap cap** | `C` (`:292`), `E` (`:201`) | **REJECT** | Violates `GP-001` (`requirements.md:56`: no circuit constant may block a run of `< 2^64` steps). `TOP = 61` (`2^64` cells/arena) costs only `+2,030` leaf rows/walk over `TOP = 32` (`ESTIMATE`) and `+0` Node rows. |
| **Single-pass Jubjub `y-increment` `EC-MSH`** | Evaluated by `A`, `B`, `C`, `D`, `E` | **REJECT (Unanimous)** | Requires $O(|M|)$ finalization sweep leaves at `Halt`/`Fault`/`OutOfFuel` (`violating GP-1`), adds `+4` `IVC_STATE` scalars, and risks `ePrint 2026/590` algebraic curve attacks if `x_0` is not hashed first. |

---

### 1.2 Detailed Review of Lane A (`lanes/A/round1.md`)

- **Accepted from Lane A:**
  1. **Lazy 8-Cell Segment Tail Carry (`M3-Tail64`, `lanes/A/round1.md:16, 61-74, 226`):** Carrying `(len, root, tail_d)` per arena where `root` commits completed 8-cell segments `0 .. floor(len/8)` in a fixed `TOP = 61` binary tree and `tail_d = Tree8(imp ? rec_i : PAD)` commits the `r = len mod 8` open records (`dense.rs:5-33, 112-119`) strictly dominates Lane D Round 1's `DIPM` partial-page RMW (`lanes/D/round1.md:145-148`). In `DIPM`, any leaf with `used > 0` and `r_pre != 0` had to walk `TOP = 61` siblings to open the old partial page from `pre.mem_root` and walk `TOP = 61` siblings to write the extended page (`2 * 61 * 70 = 8,540` rows `ESTIMATE`). Under `M3-Tail64`, `tail_d` opens in `7` permutations (`490` rows `ESTIMATE`), and the `TOP = 61` tree walk executes (`f_j = 1`) **if and only if** `r_pre + used >= 8` (and when `f_j = 1`, the old leaf is the compile-time constant `EMPTY_SEG != 0`, `dense.rs:76-79, 414-418`, eliminating old-record sibling hashing).
  2. **`D7` LIFO Hash-Chain Stacks (`lanes/A/round1.md:16, 226`; `CONSENSUS.md` §1.2.3):** Keeping the active frame/task in `CORE` registers and spilling older frames/tasks into Poseidon LIFO chains `fp_hash = H(DOM_FRAME, prev_fp_hash, frame_fields...)` and `tp_hash = H(DOM_TASK, prev_tp_hash, task_fields...)` eliminates heap Merkle walks on function call/return (`3–8` perms = `210–560` rows `ESTIMATE` on cold spill/fill, `0` rows while register-resident).
  3. **Hashed `x_0` Requirement on `y-increment` `EC-MSH` (`lanes/A/round1.md:88-96`):** Correctly notes that raw affine `x_0 = tuple + byte` falls directly to the algebraic forgery of Hankerson/Mohnblatt/Nguyen (`https://eprint.iacr.org/2026/590` breaking `https://eprint.iacr.org/2025/1503`), requiring `x_0 = Poseidon(DOM_ECMH, tuple) + k` (`k in [0, 256)`) + prime-order cofactor clearing `[8]P` before addition, and even then loses due to $O(|M|)$ boundary sweeps.
- **Rejected from Lane A:**
  1. **Bus-Multiplexed `DEC` (`lanes/A/round1.md:18, 125-131`):** Putting `DEC` rows on the shared dynamic `bus` (`q_prov = 1`) saves `dec.rs`'s `cs.lookup`, **but** requires every step slot in `CORE` to spend 1 `bus` request lane `(CHIP_DEC, syn, ...)` (`16` requests at `S = 16`, `64` at `S = 64`), halving the per-row `bus` bandwidth available for `MEM`, `ROM`, `ALU`, and `HASH`. Lane C's merged 2-column tagged `DEC + RANGE` fixed table (`§1.4`) achieves the **exact same `3`-lookup count** while leaving 100% of `bus` bandwidth for dynamic memory/ALU/hash requests.

---

### 1.3 Detailed Audit of Lane B (`lanes/B/round1.md` — `ZM-LogUp` Mechanics, Recursion Cost & System Limits)

Lane B (`lanes/B/round1.md:96-156, 174-268`) proposed **`ZM-LogUp`**: a two-stage write-once LogUp memory argument + Horner boundary state compression using `Instance(0)` (`committed_instances`) on stock `midnight-circuits = 7.3.0`. Because Round 1 assessments of `(b)` differed sharply (`B` recommended `ZM-LogUp`; `A, C, D, E` rejected `(b)`), we audit `ZM-LogUp` against `VerifierGadget`, `RunLift` `R`, `RunDecider`, `GP-2..GP-4`, and wall-clock latency with exact precision:

#### 1.3.1 What Lane B Got Right (Accepted into Swappable Backend B)
1. **Zero-Fork Compatibility with Stock `VerifierGadget` (`phases == 1`):** Unlike in-proof multi-phase LogUp (which panics at `midnight-circuits-7.3.0/src/verifier/verifier_gadget.rs:310` and `expressions/mod.rs:49`), `ZM-LogUp` commits a short public-input vector `P_i` into `Instance(0)` (`committed_instances[0] = C_i`) in Stage 1, hashes $C_0 \dots C_{L-1}$ into a binary commitment tree `d_root`, derives $(\gamma, \alpha) = \text{Poseidon}(\text{DOM\_MEM\_CHAL}, d_{\text{root}}, \text{prog\_d}, \text{input\_d})$, and supplies `(d_acc, gamma, alpha, sum_pre, sum_post)` in `Instance(1)` during Stage 2 (`lanes/B/round1.md:99-112`). Because `VerifierGadget::parse_trace` absorbs `assigned_committed_instances` (`verifier_gadget.rs:298-300`) and `assigned_instances` (`lines 302-306`) **before** `advice_commitments` (`lines 314-316`) and opens `Instance(0)` queries via KZG (`lines 435-436, 628-634`), all leaf gates remain Phase-0 (`cs.phases().count() == 1`), unit-rotation (`{-1, 0, +1}`), and degree $\le 5$. This **defeats** Lane E's `Exploit B1` (`lanes/E/round1.md:107-113`), which attacked uncommitted `Instance(1)` challenges.
2. **No Intra-Run $O(|M|)$ Teardown Sweep Needed Within One Closed Epoch (`lanes/B/round1.md:105-108`):** In Lane D Round 1 (`lanes/D/round1.md:76`), we noted that write-once LogUp avoids an $O(|M|)$ halt sweep *only* if write multiplicity $m_w$ is known at the write leaf. Lane B achieves exactly that by running Stage 1 across the whole run first, counting **how many leaves open cell $w$ in their leaf-local cold cache** (`-1` per leaf cache entry, $+m_w$ committed in `P_i` at $w$'s unique allocation leaf). Unread cells (`m_w = 0`) contribute $+0 / (\alpha - \text{enc}_{\gamma}(w)) = 0$, so within a single closed run no teardown sweep leaves are needed at `Halt`, `Fault`, or `OutOfFuel`.
3. **Associative `d_root` Preserves Intra-Run `GP-4` Tree Reduction and `SRC-023` Final Claim:** Hashing leaf commitment digests `d_i = Poseidon(DOM_COM, enc(C_i))` in a binary merge tree `Poseidon(DOM_TREE, d_L, d_R)` (`lanes/B/round1.md:208-216`) allows binary `N` folds within one run and keeps the final decided C6 claim `[prog_d, input_d, outcome_d, n_steps]` independent of `S`.

#### 1.3.2 Five Structural & Quantitative Reasons `ZM-LogUp` Loses as Primary Architecture (Rejected as M4 Default)

1. **Fatal Conflict with $O(1)$ Pause/Resume Checkpointing (`GP-001` & Run-Length `CONSENSUS.md` Tier-2 `SHK_CONT_V1`):**
   - `GP-001` (`openspec/changes/sherlock-vm-consensus-20261107/requirements.md:56`) and Run-Length `CONSENSUS.md` (§1.3) require multi-session pause/resume (`Epoch 1: [0, h_1)`, `Epoch 2: [h_1, h_2)`) under the same key set with $O(1)$ boundary handoff (`canon_state_d(h_1)`).
   - Under `ZM-LogUp`, when Epoch 1 runs steps `[0, h_1)`, Epoch 2 has **not been executed yet** (it may depend on scheduling or resume days later after host Maintenance/reboot). Thus at step $t < h_1$, the prover **cannot know** $m_{w,\text{epoch2}}$ (how many leaves in `[h_1, h_2)` will read cell $w$)!
   - Consequently, `ZM-LogUp` faces an inescapable trilemma across multi-epoch runs:
     - *(Option i)* Seal Epoch 1 with $m_w = m_{w,\text{epoch1}}$ (`sum_mem == 0` at $h_1$). Then Epoch 2 starts with **zero cryptographic commitment to any of the $|M(h_1)|$ live heap cells** from Epoch 1! To read prior-epoch cells safely, Epoch 2 must run `INIT_MEM` loader leaves re-committing all $|M_{\text{live}}(h_1)|$ cells into Epoch 2's `d_root_2` — turning $O(1)$ continuation into an **$O(|M|)$ loader-leaf storm** (`16,384` extra Node folds = **`~17.2 hours` `ESTIMATE`** at just $2^{20} = 1\text{M}$ cells at 64 cells/loader leaf!).
     - *(Option ii)* Do **not** seal or prove *any* Stage-2 leaf of Epoch 1 until the entire multi-day run `[0, N)` finishes Stage-1 execution (`lanes/B/round1.md:190`). That **destroys** live checkpoint proofs (a crash at step $0.99 N$ loses 100% of proving progress, since zero Stage-2 leaf/node proofs exist yet!), **destroys** prove-while-executing, and forces host disk/RAM to buffer the full witness/read-multiplicity graph of up to $2^{64}$ steps!
     - *(Option iii)* Carry a Merkle `mem_root` alongside `ZM-LogUp` for cross-epoch cold reads — which puts `DenseCfg` right back into the leaf key anyway, paying **both** systems' column/node costs!
   - By contrast, `M3-Tail64` (`DenseCfg` + L1 `PAGE`/`ROM`) commits the entire $2^{64}$-cell heap in `4` Arena `(len, root, tail_d)` tuples (`12` scalars inside `canon_state_d(h)`), so Epoch 2 resumes in **$O(1)$ (`0` loader leaves, `1` continuation splice fold)**!

2. **`INIT_PROG` / `INIT_INPUT` Loader Leaves Destroy Short- and Medium-Run Wall-Clock (`lanes/B/round1.md:124-125`):**
   - Lane B (`lanes/B/round1.md:124-125`) proposes avoiding a `prog_root` Merkle walk by prepending `INIT_PROG` loader leaves (`32–64` rows/leaf) that inject program/type/constructor rows into the `ZM-LogUp` bus, noting *"a $10^6$-instruction program needs 31,250 loader leaves"*.
   - **Every loader leaf is a child proof verified by a `k = 20` Node (`~61–65 s` in `N`, or `~32 s` per leaf in 2-leaf `R`)!**
   - Even a **2,048-row admitted `/1` program** (`11` tables in `c6_phys_program.rs`: expressions, parameters, constructors, ctor_fields, types, rules, constants; `lanes/C/round1.md:56`) at `64` rows/loader leaf requires **32 `INIT_PROG` leaves = `32 × 63 s = 2,016 s` (`+33.6 minutes`, `ESTIMATE`) of `k = 20` Node proving before step `h = 0` even executes** — failing Gate `G-B <= 20 min` (`requirements.md:38`) on a 6-step `entry 7;` program! And Lane B's $10^6$-row example (`31,250` loader leaves) would require **`~1,968,750 s` (`~22.8 CPU-days`, `ESTIMATE`)** of Node proving just to load the program!
   - By contrast, `prog_root` Merkle + intra-leaf `ROM` page caching (`M3-Tail64`) requires **`0` loader leaves**; work scales strictly with **distinct 8-row pages actually executed** ($O(\text{touched})$, never $O(|P|)$).

3. **Hidden Cost in `GP-4` `RunLift` `R` (`U + U -> SEG`) and Subgroup-Check Soundness (`verifier/types.rs:126`):**
   - In 1-leaf linear IVC (`SHKIVC06`), replacing fixed `[C::identity()]` (`crates/mori-recursion/src/inner.rs:181`) with a variable witness `Instance(0)` commitment $C_i$ adds 1 foreign G1 witness assignment, 5 Poseidon `common_point` absorptions (`verifier_gadget.rs:298-300`, `transcript_gadget.rs:100-112`), 1 scalar query (`line 436`), 1 `Poseidon(DOM_COM, enc(C_i))` hash (`3` perms), and 1 variable G1 MSM base in `collapse`.
   - Under **`GP-4` (`research/sherlock-gp-architecture-20261110/architecture.md:72, 206-219`)**, `RunLift` `R` verifies **two** `U` proofs in one `k = 20` circuit, so **every variable `Instance(0)` cost is doubled**:
     - **If `SelfEmulation::assign_without_subgroup_check` (`midnight-circuits-7.3.0/src/verifier/types.rs:127-131, 172-178`) is used:** `+2` non-subgroup-checked G1 assignments + `+2` variable MSM bases (`~14,600` rows) + `10` transcript scalar absorptions + `2` `DOM_COM` hashes + `1` `DOM_TREE` hash (`7` Poseidon perms) + `+5` `SEG` public scalars = **`+18,000` to `+22,500` rows in `R` (`ESTIMATE`)**.
     - **Subgroup soundness warning in `midnight-circuits-7.3.0`:** Line `126` of `midnight-circuits-7.3.0/src/verifier/types.rs` explicitly carries `// TODO: Provide a formal analysis of subgroup soundness in this context`, and `weierstrass_chip.rs:17-19` states *"We require that the emulated elliptic curve do not have low-order points."* Because BLS12-381 $\mathbb{G}_1$ has a 127-bit cofactor $h_1 = (x-1)^2/3$, `crates/mori-recursion/src/inner.rs:69` and `run_decider.rs:259` enforce prime-order subgroup checks (`AssignmentInstructions::assign`) on untrusted witness curve points. Enforcing subgroup membership on $C_{2j}, C_{2j+1}$ adds another **`~4,200–5,500` rows per point = `+8,400` to `+11,000` rows in `R` (`+26,400` to `+33,500` total rows in `R`, `ESTIMATE`)**!

4. **`RunDecider` `Pow2Range` Table Margin (`1,032,467` / `1,048,570`) Discipline:**
   - Widening `IVC_STATE` by `+4` scalars (`d_acc, gamma, alpha, sum_mem`) adds `~1,200–2,800` circuit rows (`ESTIMATE`) in `RunDeciderRelation` (`run_decider.rs:91-97, 218-245`) for `Poseidon(DOM_MEM_CHAL, d_root, prog_d, input_d) == (gamma, alpha)` and `assert_equal_to_fixed(sum_mem, 0)`. That preserves `table_rows == 1,032,467` **if and only if** none of the new decider checks invoke `Pow2Range` bit decompositions on the unloaded tag set `{2, 3, 5, 6, 7, 9, 10, 11, 12, 13}` (whose sum `16,108` exceeds the `16,103`-row usable table margin at `k = 20`, `lanes/D/round1.md:215-221`).

5. **Wall-Clock Reality: `< 2%` Leaf Savings vs Node Bottleneck:**
   - At `S = 16` (`19` leaves on `303`-step `list123`), `ZM-LogUp` (`k_leaf = 13`, `2.55 s`) vs `M3-Tail64` + L1 `PAGE`/`ROM` (`k_leaf = 15..16`, `~4.1–6.2 s` `ESTIMATE`) batched across `P = 3` leaf workers saves **`~13–23 s` out of `~1,340 s` (`1.0%–1.7%` net wall-clock, `ESTIMATE`)** — less than the cost of **half of one** `INIT_PROG` loader leaf (`31.5–63 s`)!
   - **Required algebraic guard if Backend B (`ZM-LogUp`) is built:** In `lanes/B/round1.md:107`, every inactive write/read slot in `P_i` (`act_i = 0`) MUST enforce `act_i * (1 - act_i) == 0`, `inv_i * (alpha - enc_i) == act_i`, and `(1 - act_i) * inv_i == 0` (matching `mori-chip` degree $\le 3$) so disabled slots contribute exact `0` to `sum_mem` and cannot forge `inv_i` when `alpha == enc_i`.

---

### 1.4 Detailed Review of Lane C (`lanes/C/round1.md`)

- **Accepted from Lane C:**
  1. **108-Rule `/1` Audit + Gated 11-Bit Syndrome (`<= 2,048` rows) + 54-Bit Packed `ctrl_word` (`lanes/C/round1.md:54-68, 175-186`):** Collapsing inactive sub-tags (`site` gated off when `helper != None`, reducing raw `partition.json` cubes from `3,153` to `~620` active rows) fits the universal `/1` decoder cleanly in `<= 2,048` rows (`1` scalar key, `1` scalar 54-bit `ctrl_word`).
  2. **Merged 2-Column Tagged `DEC + RANGE` Fixed Table (`6,145` rows $< 8,192`, `lanes/C/round1.md:175-186`):**
     - `Row 0`: `(0, 0)` (neutral row when `q_rng == 0 && q_dec == 0`);
     - `Rows 1..=4096`: `(2^64 + v, 1)` for `v in 0..4096` (`RANGE` 12-bit limbs; expandable to `16,384` rows / 14-bit limbs at `k_leaf >= 15` or `32,768` rows / 15-bit limbs at `k_leaf = 16` with `+0` Node rows!);
     - `Rows 4097..=4096+N_dec`: `(2 * 2^64 + syn_i, 2^64 + ctrl_i)` for `DEC`.
     - Because `q_rng` (`RANGE` region) and `q_dec` (`CORE` decode row) are 100% row-disjoint (`q_rng * q_dec == 0`), a single `cs.lookup` expression handles both chips, reducing leaf lookups from `4` to **`3`** and fixed `TableColumn`s from `3` (`1` `RANGE` + `2` `DEC`) to **`2`**, saving **`-22,600 - 6,462 = -29,062` Node rows (`823,466 -> ~794,404`, `ESTIMATE`)** and **`-45,200 - 6,462 = -51,662` rows in `GP-4` `RunLift` `R` (`ESTIMATE`)**!
  3. **64-Bit-Limb Schoolbook `OP_MUL` (`lanes/C/round1.md:163-166`):** Necessary because $(2^{128}-1)^2 \approx 2^{256} > r_{\text{BLS}} \approx 2^{254.86}$ wraps modulo $r_{\text{BLS}}$; decomposing into `(lo64, hi64)` keeps partial products $< 3 \cdot 2^{128} \ll r_{\text{BLS}}$ and shares limbs with `sherlock-value/1`.
  4. **`(SHK_VAL, 1)` Poseidon Sponge-Prefix Precomputation + `VD_{NIL,FALSE,TRUE}` Constants (`lanes/C/round1.md:204-207`):** Because `sherlock-value/1` (`crates/mori-core/src/sherlock/value_digest.rs:76-152`) always absorbs `(SHK_VAL, 1)` in chunk 0, initializing `PoseidonCfg` from compile-time `(C_0, C_1, C_2) = Perm(0, SHK_VAL, 1)` is bit-for-bit identical to `sherlock-value/1` and saves `70` rows (`1` permutation) on every dynamic `Bool`/`UInt128`/`Constructor` allocation, while constant `VD_NIL`, `VD_FALSE`, `VD_TRUE` save all `210` rows on `==`, `<`, `!`, and `Nil`.
- **Rejected from Lane C:**
  1. **`TOP = 32` Quadrant RMW (`lanes/C/round1.md:292, 297-302`):** Replace with 4-arena `M3-Tail64` at `TOP = 61` (`dense.rs`, $2^{64}$ cells per arena) so `GP-001` holds unconditionally and partial segments never pay old-path Merkle openings.

---

### 1.5 Detailed Review of Lane E (`lanes/E/round1.md`)

- **Accepted from Lane E:**
  1. **Self-Protected `ALU` Input Range Checks (`Exploit D1`, `lanes/E/round1.md:134-138`):** In `crates/mori-chip/src/alu.rs:8-9, 111-125`, `AluConfig` currently range-checks only the output/borrow witness `w < 2^128`, relying on the caller to guarantee `x, y < 2^128`. As Lane E proved (`Exploit D1`) and real-world SP1 Lean-FV audit findings confirmed (`https://zkevm.ethereum.foundation/blog/sp1-fv`, unverified cross-chip preconditions), modular rule circuits (`MODULARITY.md:10`) MUST NOT rely on unverified cross-module range assumptions: checking `x, y < 2^128` inside `ALU` (2 `range1` decompositions = `+24` rows/op at 12-bit limbs, or `+16` rows/op at 16-bit limbs -> `+256..384` leaf rows at `16` ops/leaf, `ESTIMATE`, **`+0` Node rows**) eliminates `OP_LT` and `OP_ADD` modular wraparound forgeries unconditionally.
  2. **Complete Bus & Boundary Hardening Checklist (`Exploits A1–A5, C1–C2, E1–E2, G1`, `lanes/E/round1.md:72-155, 215-218`):** Every module kit in Section 4 below bakes in `A1` (`addr < cur_len`), `A2` (`act * tuple` + boolean `act`), `A3` (`addr < base` vs `addr >= base`), `A4` (`8 * addr + kind` injectivity + domain-tagged `iv`), `C1` (boolean/range gates on unpacked `ctrl_word`), `C2` (`OP_INVALID_OOB` sentinel + `pc < prog_len`), `E1/E2` (fixed capacity `iv` + inline `vd`), and `G1` (zero deferred accumulators in `SET`).
- **Rejected from Lane E:**
  1. **`WB[4]` Raw Words + `TOP = 30` (`lanes/E/round1.md:174-184, 201`):** Carrying `tail_d` (`1` scalar per arena in `dense.rs`) instead of `WB[4]` raw words (`12` scalars per arena = `48` boundary scalars across 4 arenas!) saves `36` boundary scalars per state digest (`18` Poseidon permutations = `1,260` leaf rows `ESTIMATE`), and `TOP = 61` replaces `TOP = 30` to satisfy `GP-001`.
  2. **Bus-Folded `DEC` (`lanes/E/round1.md:268-269`):** Replaced by Lane C's merged 2-column tagged `DEC + RANGE` fixed table (`§1.4`).

---

## 2. Explicit Answers to the Four Questions in `LEAD-NOTE-ROUND2.md`

### 2.1 Question (1): Merkle (`M3-Tail64` + L1 `PAGE`/`ROM` Cache) vs Run-Wide Write-Once LogUp (`ZM-LogUp`)

**Final Decision:**
- **Memory (`DATA`, `FIELDS`, `ENV`, `SLOTS`), `PROG`, and `TYPE`/`CTOR`/`CONST` tables MUST use the Merkle backend (`M3-Tail64` / `DenseCfg` + 8-cell L1 `PAGE` and `ROM` cold-segment caches on `bus`, plus `D7` LIFO hash chains for `FRAMES`/`TASKS`) as Sherlock's primary production backend (Backend A).**
- **`ZM-LogUp` is retained strictly as an optional swappable Backend B behind `MemBackend` + `BoundaryScheme` (`§4`) for single-epoch head-to-head A/B benchmarking — and even in Backend B, `PROG` and `TYPE` tables should use `prog_root` Merkle + intra-leaf `ROM` caching rather than `INIT_PROG` loader leaves.**

| Requirement / Dimension | **Backend A: `M3-Tail64` (`DenseCfg`) + 8-Cell L1 `PAGE`/`ROM` Cache** | **Backend B: Run-Wide Write-Once `ZM-LogUp` (`Instance(0)` + `d_root`)** | Winner |
|---|---|---|---|
| **Memory (`2^64` cells, `GP-1` / `GP-001`)** | 4 dense arenas (`DATA`, `FIELDS`, `ENV`, `SLOTS`), each `TOP = 61` (`2^61 × 8 = 2^64` cells/arena), `0` segments burned on 0-append leaves (`dense.rs:278-295, 721-729`). | 64-bit `addr` tags in `P_i`, **but** Stage 1 must materialize & count read multiplicities `m_w` over the entire run in host RAM/disk before proving Leaf 0 (`O(|M|)` host state across the whole run). | **Merkle (`M3-Tail64`)** |
| **`PROG` & `TYPE`/`CTOR`/`CONST` Tables** | Committed under `prog_root` (`design.md:84`); `K_rom = 2..4` 8-row `ROM` cache lines opened per leaf (`~3,080–6,160` leaf rows `ESTIMATE`, **`0` extra leaves / `0` extra Node proofs**). | `INIT_PROG` loader leaves (`lanes/B/round1.md:124-125`): costs **`1` `k = 20` Node fold (`~63 s`) per 64 program rows** (`+33.6 min` for 2,048 rows; `+22.8 days` for $10^6$ rows `ESTIMATE`). | **Merkle (`prog_root` + `ROM` cache)** |
| **`SI-12..SI-14` Field Slices (`dissent.md` `D-1`)** | 4-arena dense counters (`len_fields` advances by exact `+1` per `KIND_FIELD` append, zero leaf-close gaps); `start + k` holds verbatim; 8-cell `PAGE` line fetches 8 contiguous fields in 1 walk! | Dense per-arena counters also preserve `start + k`, serving 1 cold field per `K_m` cache slot (8-field constructor cold read uses 8 `K_m` slots in `P_i`). | **Tie on semantics; Merkle wins spatial burst** |
| **`SRC-023` Partition Independence & C6 Claim** | **Strongest form:** `canon_state_d(h)` (`regs`, `fp_hash`, `tp_hash`, 4×`(len, root, tail_d)`) is **bit-for-bit identical at every step `h`** regardless of `S` or early-close cuts; `Aggregation.lean:201, 1154` `digest_inj` & `statement_independent_of_S` apply **unchanged**; C6 `[prog_d, input_d, outcome_d, n_steps]` unchanged. | Final C6 claim `[prog_d, input_d, outcome_d, n_steps]` is partition-independent (`sum_mem == 0`), **but** intermediate `d_root`, `(gamma, alpha)`, and `sum_mem` depend on leaf chunking; `Aggregation.lean` requires a new Stage-1/Stage-2 rational-sum soundness proof. | **Merkle (`M3-Tail64`)** |
| **`GP-2` Multi-Run & Checkpoint Continuations** | **`O(1)` (`1` continuation splice fold on `canon_state_d(h)`, `0` memory re-import leaves)**; consumer runs verify `vd` in $O(|v_{\text{out}}|)$. | Cross-run `GP-2` via `vd` works, **but** multi-epoch `GP-001` / Tier-2 checkpoint continuations reading prior-epoch cells require **`O(|M_live|)` re-import leaves** (`~17.2 h` per $10^6$ cells `ESTIMATE`). | **Merkle (`M3-Tail64`)** |
| **`GP-3` Foreign Proofs (`F`)** | `IVC_STATE` stays narrow; Node at `~794k–807k` rows (`> 140k` headroom below `950,000`); Decider `table_rows == 1,032,467` untouched. | Adds `+4..5` `SEG` scalars (`d_sub, gamma, alpha, sum_mem`), tightening `C` and `D` state plumbing. | **Merkle (`M3-Tail64`)** |
| **`GP-4` Arbitrary Tree / Diamond DAG (`R, N, C`)** | Single-pass: any subtree or DAG branch proves **immediately** as execution reaches those steps; `RunLift` `R` saves **`-51,662` rows (`ESTIMATE`)** via tagged `DEC+RANGE` and has `0` variable `Instance(0)` commitments. | Intra-run `R`/`N` trees must wait behind a **global full-run Stage-1 barrier**; `RunLift` `R` pays **`+18,000` to `+33,500` extra rows (`ESTIMATE`)** for **2** variable `Instance(0)` G1 commitments! | **Merkle (`M3-Tail64`)** |

---

### 2.2 Question (2): Two-Pass Cost (`ZM-LogUp` Stage 1) vs One-Pass Alternatives (`M3-Tail64` + L1 `PAGE` vs Jubjub `y-increment` `EC-MSH`)

#### 2.2.1 Quantified Impact of `ZM-LogUp` Stage-1 Pre-Commitment (`ESTIMATE`)

In `ZM-LogUp`, before **any** Stage-2 leaf proof (`0 .. L-1`) can begin synthesis, the host must execute Stage 1 across the **entire** $N$-step run ($L = \lceil N/S \rceil$ leaves):
1. Execute all $N$ steps, record every allocation `(addr, rec)` and every per-leaf cold-cache miss set, and increment the global multiplicity table `m_w[addr]` for every leaf `j > write_leaf(addr)` that touches `addr`.
2. For each leaf $i \in [0, L)$, assemble `P_i` (`~320–512` scalars for `pre_regs`, `post_regs`, writes + `m_w`, and cold reads) into the Lagrange basis of `Instance(0)` and compute KZG commitment $C_i = \sum_{r=0}^{|P_i|-1} P_{i,r} \mathcal{L}_r \in \mathbb{G}_1$ (`~0.5–1.5 ms`/leaf MSM over precomputed Lagrange SRS bases), plus `d_i = Poseidon(DOM_COM, enc(C_i))` and binary tree `d_root`.

| Operational Metric | **One-Pass `M3-Tail64` + L1 `PAGE`/`ROM`** | **Two-Pass `ZM-LogUp` (`Stage 1` + `Stage 2`)** | **One-Pass Jubjub `y-inc` `EC-MSH`** |
|---|---|---|---|
| **Stage-1 Pre-Pass Latency Before Leaf 0 Starts (`ESTIMATE`)** | **`0 s`** (Leaf 0 proves immediately after step `S` finishes!) | `~0.1–0.3 s` at $N = 303$; `~8–25 s` at $N = 10^6$; **`~15–45 min` dead-time** at $N = 10^8$ (`ESTIMATE`) | **`0 s`** during execution, **but** $O(|M|)$ finalization leaves at end! |
| **Host Trace / Multiplicity RAM & Disk Buffering (`ESTIMATE`)** | **`O(|M_live|)` sparse tree (`~64–96 B`/cell)**; leaf witnesses streamed and dropped every `S` steps (`< 50 MB` trace buffer) | **`O(N + |M|)` full-run leaf trace + `m_w` array** (`~2–6 GB` at $N = 10^6$; **`~200–600 GB` disk spool** at $N = 10^8$ because Leaf 0 needs `m_w` from step $10^8$!) | **`O(|M|)` live `(addr, val, last_clk)` map** (must all be swept into finalization leaves at Halt!) |
| **Prove-While-Executing (Streaming Pipeline)** | **Yes (100%):** Executor streams `S` steps $\to$ Leaf worker proves `U_i` $\to$ Node worker folds `N_i` (`GP-4` `R`/`N` subtrees fold live!) | **No (0%):** Impossible to prove `U_0` while executing step $1,000$ because $(\gamma, \alpha)$ depends on $C_{L-1}$ and `P_0`'s `m_w` depends on reads at step $N - 1$! | **Partial:** Execution leaves stream, **then stalls** at Halt to prove $\lceil |M| / C_{\text{sweep}} \rceil$ teardown leaves + `k = 20` Node folds! |
| **Tier-2 Checkpoint Pause / Crash Recovery (`CONSENSUS.md` §1.3)** | **Exact `O(1)` every `K` leaves:** Sealed segment proof `[0, h_k)` is already valid on disk; resume from `canon_state_d(h_k)` with `0` re-work | **Broken (`O(|M|)` or `0%` progress saved):** Either `0` Stage-2 proofs exist if crash occurs during Stage 1, or sealing Epoch 1 forces `O(|M_live|)` re-import Node proofs in Epoch 2! | **Broken (`O(|M_live|)` sweep + re-init leaves at every checkpoint!)** |
| **Net Wall-Clock (`list123`, `303` steps, `S = 16`, `2,048`-row prog, `ESTIMATE`)** | `19` leaves (`~38 s` on `P=3`) + `19` Nodes (`1,197 s`) + Decider/Setup (`135 s`) = **`~22.8 min`** (`~8.0 min` at `S = 64`!) | Without `INIT_PROG` (hybrid Merkle prog): `~22.5 min` (`saves ~20 s`); **With `INIT_PROG` (32 loader leaves): `~56.4 min` (`+147%` SLOWER!)** | `19` exec leaves + `~8` sweep leaves = `27` Nodes (`1,701 s`) + Decider (`135 s`) = **`~31.3 min` (`+37%` SLOWER!)** |

**Bottom line on Question (2):** One-pass **`M3-Tail64` + L1 `PAGE`/`ROM` cache** strictly dominates both two-pass `ZM-LogUp` (preserving prove-while-executing, bounded memory, `0` loader leaves, and `O(1)` Tier-2 checkpoints while giving within `~1.5%` of `ZM-LogUp`'s ideal leaf time) and one-pass Jubjub `y-increment` `EC-MSH` (avoiding `EC-MSH`'s $O(|M|)$ finalization Node storm and `+4` Jubjub `IVC_STATE` coordinates).

---

### 2.3 Question (3): `VerifierGadget` Single-Phase Audit & Zero-Fork Compatibility on `midnight-circuits = 7.3.0`

We confirm from direct source inspection of `midnight-circuits-7.3.0` and `crates/mori-recursion`:

1. **Exact Single-Phase & Rotation Constraints in `midnight-circuits-7.3.0`:**
   - `midnight-circuits-7.3.0/src/verifier/verifier_gadget.rs:308-316`:
     - Line `310`: `assert_eq!(cs.phases().count(), 1);` — panics if any advice column is allocated in `SecondPhase` or later.
     - Lines `314-316`: reads every advice commitment in a single unphased loop before squeezing PLONK's internal `theta` (`line 319`), `beta` (`line 327`), `gamma` (`line 328`), and `trash_challenge` (`line 341`).
   - `midnight-circuits-7.3.0/src/verifier/expressions/mod.rs:49`:
     - `Expression::Challenge(_) => panic!("We do not suport multi-phase yet"),` — panics if any gate or lookup expression references a Halo2 `Challenge`.
   - `midnight-circuits-7.3.0/src/verifier/verifier_gadget.rs:604-610`:
     - Supports only `Rotation(-1)`, `Rotation(0)`, and `Rotation(1)`.

2. **Zero-Fork Compatibility Verification for Both Backends (`midnight-circuits = 7.3.0`, `midnight-proofs = 0.8.3`):**

| `VerifierGadget` / `RunDecider` Invariant | **Backend A (`M3-Tail64` + L1 `PAGE`/`ROM` + Tagged `DEC+RANGE`)** | **Backend B (`ZM-LogUp` via Stage-1 `Instance(0)`)** |
|---|---|---|
| `cs.phases().count()` (`verifier_gadget.rs:310`) | **`1`** (100% `FirstPhase`, zero `Challenge`s) | **`1`** (`P_i` is in `Instance(0)`, `(gamma, alpha)` are `Instance(1)` scalars; all advice columns in `FirstPhase`) |
| `Expression::Challenge(_)` (`expressions/mod.rs:49`) | **None** | **None** (`gamma, alpha` queried via `Instance(1)` / copy-constrained advice cells) |
| Query rotations (`verifier_gadget.rs:604-610`) | **`{-1, 0, +1}` only** (`dense.rs`, `membackend.rs`, `poseidon.rs`, `alu.rs`, `table.rs`) | **`{-1, 0, +1}` only** |
| `Instance(0)` handling (`inner.rs:181`, `verifier_gadget.rs:298, 435, 628`) | Fixed `[bls.assign_fixed(l, C::identity())?]` (**unchanged** from `inner.rs:181`) | Witnessed `C_i` assigned in `N`/`R`, absorbed via `common_point` (`line 299`), queried at `Rotation::cur()` (`line 436`), opened in `multi_prepare` (`line 671`) — **zero fork of `midnight-circuits`**, modifies only `crates/mori-recursion` |
| Leaf VK profile `(adv / eq / fix / lkp / deg)` (`ESTIMATE`) | **`12 / 13 / 20..21 / 3 / deg 6`** (`-1` lookup via Tagged `DEC+RANGE`!) | **`12 / 13 / 19..20 / 3 / deg 6`** (`+1` queried `Instance(0)` column) |
| `k = 20` Node `N` rows (`823,466` E8 baseline, `950k` gate) (`ESTIMATE`) | **`~800,866` to `~807,328` rows** (**`-16,138` to `-22,600` rows BELOW E8!**) | **`~808,000` to `~819,000` rows** (`-29,062` from 3 lookups, offset by `+11k..24k` for var `Instance(0)` + `d_acc` hash + carry) |
| `k = 20` `RunLift` `R` (`U + U -> SEG`) delta vs E8-pair (`ESTIMATE`) | **`-51,662` rows** (`-45,200` from `-1` lookup × 2 proofs, `-6,462` from `-1` fixed col) | **`-25,000` to `-18,000` rows** (`-51,662` from 3 lookups, offset by **`+26,400..33,500`** for 2× subgroup-checked `Instance(0)` G1s + hashes) |
| `k = 20` `RunDecider` `table_rows` (`1,032,467` / `1,048,570` usable) | **Exact `1,032,467` (`+0` table rows; `16,103` margin 100% preserved)** | **`1,032,467` (`+0` table rows)** *iff* new decider checks (`sum_mem == 0`, `Poseidon` challenge check) avoid `Pow2Range` tags `{2,3,5,6,7,9..13}` |

---

### 2.4 Question (4): Reconciling `S = 16` vs `S = 64` or `128` with `SRC-014` and Run-Length `CONSENSUS.md`

In Round 1, Lanes B and C observed that caching shrinks per-step leaf rows enough that `S = 64` (`k_leaf = 16..17`) or `S = 128` fits in reasonable leaf domains, whereas `SRC-014` (`openspec/changes/sherlock-vm-consensus-20261107/requirements.md:39`) sets `S_MAX = 16`, `k_leaf <= 16`, and Run-Length `CONSENSUS.md` (§1.5.2, Dissent `D-Q5.1`) keeps `S = 16`, `k_leaf = 16` as the M4 baseline while flagging `S = 64`, `k_leaf = 17` as a post-M4 speed lever.

**Exact Technical Reconciliation (3-Stage Rule):**
1. **Freeze `S = 16`, `k_leaf = 16` (`W = 1` flush unit per arena, `K_page = 2`, `K_rom = 2`) as the Milestone M4 Default (`SRC-014` Preserved Unchanged for M4):**
   - At `S = 16` with `M3-Tail64` (`W = 1` per arena: `4` arenas $\times$ `~9,400` rows + `2` 8-cell `PAGE` lines `11,778` rows + `2` 8-row `ROM` lines `~5,200` rows + `CORE`/`ALU`/`vd`/`state_d` `~8,500` rows $\approx$ **`~48,000–58,000` rows `ESTIMATE`**, comfortably `< 65,530` at `k_leaf = 16`), Sherlock **already clears** `SRC-011` Gate `G-C < 2 h` (`303` steps in **`~22.8 min` linear IVC** or **`~11 min` `GP-4` tree `ESTIMATE`**), uses `< 1.5 GB` leaf prover RSS (`3` workers `< 4.5 GB`), satisfies `W = 1` progress without multi-flush livelock (since `S = 16` steps allocate on average `3.2` cells total across all arenas, `design.md:119`, and early-closes cleanly if any single arena crosses `8 + (8 - r_pre)` appends), and honors `CONSENSUS.md`'s rule (`SRC-011`) that no unmeasured lever is credited before measurement.
2. **Parameterize `LeafShape { s_slots, w_flush, k_page, k_rom, limb_bits }` Cleanly in `crates/mori-chip` (`+0` Architectural Rework):**
   - Changing `S` from `16` to `64` changes `k_leaf` (`16 -> 17`, adding **only `+4` rows** to the `k = 20` Node, `e2-e4.md:25-35`!) and adjusts fixed selector periodicity (`MidnightVK` changes once when compiled for `S = 64`, while `CORE`, `MEM`, `DenseCfg`, `Node`, and `Decider` code stay 100% identical).
3. **Benchmark `S = 64`, `k_leaf = 17` (`W = 2` on hot arenas, `K_page = 4`, `K_rom = 4`, `~98,000–122,000 < 131,066` rows `ESTIMATE`) Immediately After M4 Integration; Reject `S = 128`:**
   - Why **`S = 64` (`k_leaf = 17`)** is the sweet spot for an `SRC-014` amendment (`S_MAX: 16 -> 64`, `k_leaf <= 17`): leaf prove time rises from `~6.2 s` to `~10.5 s` (`~2.0 GB` RSS `ESTIMATE`), still `6x` faster than one `63 s` Node fold, cutting `list123` Node folds from `19` to `5` (`~22.8 min -> ~8.0 min` linear IVC, `-65%` wall-clock `ESTIMATE`!).
   - Why **`S = 128` (`k_leaf = 18..19`, `19–35.5 s`, `3.8–6.8 GB` RSS `ESTIMATE`)** is rejected on CPU: (i) 3 concurrent `k = 18..19` leaf workers contend with the `k = 20` Node on the `273 GB/s` LPDDR5X bus (`e2-e4.md:19`), and (ii) pointer-chasing functional programs suffer diminishing effective $\bar{s} < 128$ due to early leaf closes whenever `K_page` cold segments fill before slot 128.

---

## 3. Explicit Relationship to Run-Length `CONSENSUS.md`

**Explicit Compatibility Statement:** Our converged architecture is **100% compatible with — and is a direct, strictly additive extension of — `research/sherlock-runlength-consensus-20261111/CONSENSUS.md`.** It **does NOT replace** any consensus decision in `CONSENSUS.md`.

Specifically:
1. **Adopted Verbatim from `CONSENSUS.md`:**
   - **4-Arena `M3-Tail64` Dense Heap (`CONSENSUS.md` §1.2.3, already prototyped in `crates/mori-chip/src/{membackend.rs,dense.rs}`):** Separate dense 64-bit counters and `(len, root, tail_d)` boundary triples for `DATA` (`KIND_DATA = 1`), `FIELDS` (`KIND_FIELD = 2`), `ENV` (`KIND_ENV = 3`), and `SLOTS` (`KIND_SLOT = 4`), `TOP = 61` (`2^64` cells/arena), `EMPTY_SEG != 0 != PAD`, and `0` tree walks when `floor(len_post/8) == floor(len_pre/8)`.
   - **`D7` LIFO Hash-Chain Stacks (`CONSENSUS.md` §1.2.3):** Register-resident `top` frame/task + Poseidon chains `fp_hash`, `tp_hash`.
   - **Canonical State Digest & Two-Tier Continuations (`CONSENSUS.md` §1.3):** Slot-0 `v0 = v1 = v2 = 0` scratch zeroing, canonical step-indexed `canon_state_d(h)` (`SRC-023`), Tier-1 host checkpointing, and Tier-2 `SHK_CONT_V1` cross-proof continuation splicing (`Mode 1` $O(1)$ recursive fold + `Mode 2` opt-in batch checkpoint claim), plus `SET.n_claims < 2^64` check (`CONSENSUS.md` `Exploit E-1`).
   - **Baseline `S = 16`, `k_leaf = 16` (`CONSENSUS.md` §1.5.2).**
2. **Four Additive Lookup/Cache Extensions Contributed by This Study on Top of `CONSENSUS.md`:**
   - **Extension E-A (L1 8-Cell Segment `PAGE` & `ROM` Cache on `bus`):** Replaces `CONSENSUS.md` / `dense.rs:716-752`'s uncached per-port `cold_read` (`4,621` rows per individual read) with deduplicated 8-cell cold segment providers on `bus`, cutting repeated and spatially adjacent cold reads (`SI-12` constructor slices, `ENV` lookups, `PROG` basic blocks, `types`/`constants`) from `4,621` rows/read to **`0` rows on intra-leaf hits** (`5,889` rows once per 8-cell segment `ESTIMATE`).
   - **Extension E-B (Merged 2-Column Tagged `DEC + RANGE` Fixed Table):** Cuts leaf lookups `4 -> 3` and fixed columns `-1`, winning **`-29,062` Node rows** and **`-51,662` `RunLift` `R` rows (`ESTIMATE`)** — giving `N` and `R` massive `k = 20` headroom for `CONSENSUS.md`'s `CONT-FOLD` splice arm!
   - **Extension E-C (`(SHK_VAL, 1)` Sponge-Prefix `vd` + `VD_{NIL,FALSE,TRUE}` + 64-Bit `OP_MUL` + Self-Ranged `ALU`):** Cuts dynamic `vd` hashing by `-70` rows/alloc, boolean/nil `vd` by `-210` rows, and closes `Exploit D1` (`alu.rs` input range guard) at `+0` Node rows.
   - **Extension E-D (Modular Swappable `MemBackend` + `BoundaryScheme` Seam):** Preserves `membackend.rs`'s clean `WindowCfg` / `MemBackend` split so `DensePagedBackend` (Backend A) and `ZmLogUpBackend` (Backend B) swap without touching `CORE` or `WindowCfg`.

---

## 4. Modular Architecture & Swappable Interfaces (`MODULARITY.md`)

All leaf modules share `Cols { a: [Column<Advice>; 12] }` (`crates/mori-chip/src/lib.rs:44-50`), `BusConfig` (`2` 5-tuple `lookup_any` request lanes on `a[0..5]` and `a[5..10]` into 1 provider lane on `a[0..5]`, `crates/mori-chip/src/bus.rs:47-67`), and `TaggedTableCfg` (`1` `cs.lookup` into `2` `TableColumn`s), keeping the total leaf lookup count at **3** and advice columns at **12**.

### 4.1 Frozen Global Bus & Table Tag Contract

Every `bus` tuple `(chip, key, a, b, c)` has **non-zero `chip` iff active**, and `(0, 0, 0, 0, 0)` on all inactive request/provider rows (`D4`/`D8`):

| `chip` ID | Subsystem | `key` Encoding (Injective Functional Domain) | `(a, b, c)` Payload | Provider Module |
|---:|---|---|---|---|
| `0` | Neutral / Inactive | `0` | `(0, 0, 0)` | Mandatory row 0 of provider region; all disabled slots emit `0 * tuple` |
| `1` (`CHIP_MEM`) | Heap Arenas (`DATA, FIELD, ENV, SLOT`) | `8 * addr + kind` (`0 <= addr < 2^64`, `kind in 1..=4`, `membackend.rs:55-58`) | `(d0, d1, d2)` (for `DATA`: `(ctor_or_tag, payload_or_slice, vd)`) | `M4: WindowCfg` (`addr >= 8*q_pre`) **or** `M5: MemBackend` cold cache (`addr < 8*q_pre`) |
| `2` (`CHIP_ALU`) | Checked 128-bit `ALU` | `op_code` (`1=ADD, 2=LT, 3=SUB, 4=MUL64, 5=DIVMOD, 6=EQ`) | `(x, y, z + 2^128 * flag)` (for `DIVMOD`: 2-row or `(x, y, q)` + `(y, q, r)` pair) | `M3: AluCfg` |
| `3` (`CHIP_HASH`) | 2-to-1 Tagged Poseidon | `iv(dom, sub)` (`((dom << 16) \| sub) << 32 != 0`, `dense.rs:60-62`) | `(in0, in1, out)` | `M6: PoseidonCfg` |
| `4` (`CHIP_ROM`) | Admitted Program / Type / Const Tables | `8 * row_idx + rom_subtable` (`rom_sub in 1..=7`: `Expr, Param, Ctor, CtorField, Type, Rule, Const`) | `(w0, w1, w2_or_vd_or_ty_d)` | `M5: MemBackend` `ROM` cache (`prog_root` cold segments) |
| `5` (`CHIP_VD`) | Dynamic `sherlock-value/1` Digest | `val_kind` (`1=UInt128, 2=Ctor0, 3=Ctor1, 4=Ctor2, 5=CtorSlice`) | `(x0, x1, vd_out)` | `M7: ValDigestCfg` |

**Frozen Tagged Fixed Table (`M2: TaggedTableCfg`, 1 `cs.lookup`, 2 `TableColumn`s `(tbl_key, tbl_val)`):**
- `Row 0`: `(0, 0)` (queried when `q_rng == 0 && q_dec == 0`);
- `RANGE` (`q_rng = 1`): queries `(2^64 + limb, 1)` against `Rows 1..=2^B` (`B = 12` -> `4,096` rows, or `B = 14` -> `16,384` rows at `k_leaf = 16`);
- `DEC` (`q_dec = 1`): queries `(2 * 2^64 + syn_11b, 2^64 + ctrl_54b)` against `Rows (2^B + 1)..=(2^B + N_dec)` (`N_dec <= 2,048`).

---

### 4.2 Per-Module Specification & Verification Kits (`M1`–`M10`)

Each module kit specifies: **(a)**Owned files, **(b)** Frozen interface & soundness invariants, **(c)** `ESTIMATE` row budget, **(d)** Self-contained Quint model, **(e)** `MockProver` unit harness & named mutants (`membackend::overwrite` pattern), and **(f)** Rust skeleton with `Requires / Ensures / Check` contracts (`research/skeleton-coding-20261108/pilot-protocol.md:53-55`).

---

#### Module `M1`: `CORE` (Step Slot Engine, Architectural Registers & 108 Rule Bodies)
- **Owned files:** `crates/mori-chip/src/core/{mod.rs, slot.rs, regs.rs, unpack.rs}` and rule-family files `crates/mori-chip/src/core/rules/{eval.rs, call_bind.rs, construct.rs, structural.rs, environment.rs, arithmetic.rs, admin.rs}` (parallelizable per rule family!).
- **Interface & Invariants:**
  - Consumes `BoundaryPre` cells (`regs[0..29]`, `fp_hash`, `tp_hash`, `arena_len[0..4]`, `prog_len`) and emits `BoundaryPost` cells via copy constraints; **never** touches `mem_root`, `tail_d`, `d_acc`, `gamma`, or `sum_mem` (`MODULARITY.md:5,7`!).
  - Issues up to **4 `bus` requests/slot** (2 `q_req = 1` rows in a 14-row slot) and **1 `q_dec = 1` lookup/slot** into `M2: TaggedTableCfg`.
  - **Invariants:** (I1.1) `en_0 >= en_1 >= ... >= en_{S-1}` boolean prefix, `steps = sum en_s`; when `en_s = 0`, all architectural registers, `fp_hash`, `tp_hash`, and `arena_len[0..4]` copy unchanged (`post == pre`) and all bus requests emit `(0,0,0,0,0)`. (I1.2) Slot 0 enforces `v0 = v1 = v2 = 0` when scratch values are dead (`CONSENSUS.md` §1.2.3). (I1.3) Every memory read at `(arena, addr)` asserts `addr < cur_len[arena]` via `M3: ALU` `OP_LT` (`Exploit A1`) and pins `kind = kfix` (`Exploit A4`); every memory append writes at exact `cur_len[arena]` and increments `cur_len' = cur_len + 1`. (I1.4) Every instruction fetch checks `pc < prog_len` (`Exploit C2`). (I1.5) Unpacked `ctrl_54b` fields satisfy explicit boolean/domain gates before summing to `ctrl_54b` (`Exploit C1`).
- **Row budget (`ESTIMATE`):** `14` rows/slot $\implies$ **`224` rows at `S = 16`** (`896` rows at `S = 64`); **`+0` Node rows** beyond baseline `12` advice / `13` eq columns.
- **Quint model:** `models/sherlock/quint/modCore.qnt` — stubbing `bus` and `DEC` as mathematical maps, checks `invStepRefinesOwnerRuntime`, `invDisabledIdentity`, `invCausalReadAddrLtCurLen`, `invSlot0ScratchZero`.
- **`MockProver` mutants (`crates/mori-chip/src/core/tests.rs`):** `core_mut_en_non_prefix`, `core_mut_disabled_mutates_reg`, `core_mut_ctrl_carry_borrow_forged` (`c0=2, c1=0`), `core_mut_read_at_cur_len_future` (`Exploit A1`), `core_mut_pc_ge_prog_len` (`Exploit C2`), `core_mut_slot0_nonzero_dead_scratch`.
- **Function skeleton & contracts:**
  ```rust
  pub struct CoreStepIo {
      pub en: Cell,
      pub regs_in: [Cell; 29],
      pub fp_hash_in: Cell,
      pub tp_hash_in: Cell,
      pub arena_len_in: [Cell; 4],
  }
  /// Synthesize one microstep slot `s` across shared columns `Cols`.
  /// Requires: `io.en` constrained boolean; `s == 0` implies dead scratch lanes constrained zero.
  /// Ensures: if `en == 0`, returned state equals `io` on every lane and all bus requests are `(0,0,0,0,0)`;
  ///          if `en == 1`, `(syn_11b, ctrl_54b)` matches `TaggedTableCfg::q_dec`, every unpacked flag is
  ///          range/boolean gated, reads prove `addr < arena_len_in[k]`, appends use `addr == arena_len_in[k]`.
  /// Check: `MockProver` rejects `core_mut_*` and matches `owner_runtime::step_once` on all 108 rule fixtures.
  pub fn synth_slot(
      cfg: &CoreCfg, r: &mut Region<'_, F>, o: &mut usize, io: &CoreStepIo, w: &SlotWitness, names: &mut Names,
  ) -> Result<CoreStepOut, Error>;
  ```

---

#### Module `M2`: `DEC + RANGE` (`TaggedTableCfg` — Merged Fixed Lookup Table)
- **Owned files:** `crates/mori-chip/src/table.rs` (superseding standalone `dec.rs` and `range.rs` lookup registrations while keeping `range1` limb-sum gates).
- **Interface & Invariants:**
  - Registers **1** `meta.lookup("tagged_dec_range", ...)` and **2** `TableColumn`s `(tbl_key, tbl_val)`.
  - Exposes `range_u128(r, o, x_src) -> Result<(Cell, [Cell; 2]), Error>` (returns range-checked `x < 2^128` plus `(lo64, hi64)` sub-limbs!) and `enable_dec_row(r, row)`.
  - **Invariants:** (I2.1) `q_rng * q_dec == 0` on every circuit row (structural region disjointness, verified by synthesis assertion + VK polynomial check). (I2.2) Domain tags `0`, `1 * 2^64`, `2 * 2^64` partition neutral, `RANGE`, and `DEC` keys with zero overlap (`limb < 2^64`, `syn_11b < 2^11`). (I2.3) `range_u128` running sum starts at `x`, subtracts `limb_i * 2^{B*i}`, and constrains the final remainder to `0` (`range.rs:64-88`).
- **Row budget (`ESTIMATE`):** Fixed table `6,145` rows (`B = 12`, fits `k >= 13`) or `18,433` rows (`B = 14`, fits `k >= 15`), `0` advice trace rows added for `DEC`, `9–12` trace rows per `range_u128` call; **`-29,062` Node rows** and **`-51,662` `RunLift` `R` rows (`ESTIMATE`)** relative to separate `DEC` + `RANGE` lookups!
- **Quint model:** `models/sherlock/quint/modTaggedTable.qnt` — verifies `invTagDisjoint`, `invRangeExact128`, `invDecDeterministic108`.
- **`MockProver` mutants:** `tbl_mut_range_val_4096`, `tbl_mut_range_borrows_dec_tag`, `tbl_mut_dec_unknown_syndrome`, `tbl_mut_u128_final_acc_nonzero`, `tbl_mut_lo64_hi64_carry_bleed`.
- **Function skeleton & contracts:**
  ```rust
  /// Configure the single merged DEC + RANGE lookup over disjoint selectors `q_rng` and `q_dec`.
  /// Requires: `q_rng` and `q_dec` are never both enabled on the same row.
  /// Ensures: `q_rng == 1` forces `a[11] in [0, 2^LIMB_BITS)`; `q_dec == 1` forces `(syn, ctrl) in DEC_108`.
  /// Check: `cs.lookups().len()` increases by 1 and `TableColumn` count by 2; rejects `tbl_mut_*`.
  pub fn configure(meta: &mut ConstraintSystem<F>, cols: Cols) -> TaggedTableCfg;
  ```

---

#### Module `M3`: `ALU` (Self-Ranged 128-Bit Checked Arithmetic & Comparisons)
- **Owned files:** `crates/mori-chip/src/alu.rs`.
- **Interface & Invariants:**
  - Serves `(CHIP_ALU, op, x, y, z + 2^128 * flag)` on `bus` (`q_prov = 1`, gated by `act_alu`).
  - Supports `OP_ADD`, `OP_SUB`, `OP_LT`, `OP_EQ`, `OP_MUL64` (64-bit limbs `x = x0 + 2^64 x1`, `y = y0 + 2^64 y1`, `flag = is_nonzero(x1*y1 + c1)`), and `OP_DIVMOD` (`y == 0 -> div0 trap` else `x = y*q + r` and `r < y`).
  - **Invariants:** (I3.1 — **Closes `Exploit D1` unconditionally**) Every active `ALU` slot range-checks **both inputs `x, y < 2^128` AND output/witness `z, w < 2^128`** via `M2: TaggedTableCfg::range_u128` (`x0, x1, y0, y1 < 2^64` come directly from `range_u128`!). (I3.2) Inactive `ALU` slots (`act_alu = 0`) emit `(0,0,0,0,0)` on `bus`. (I3.3) Zero field-modulus wraparound: max intermediate value in `OP_MUL64` and `OP_DIVMOD` is $< 3 \cdot 2^{128} < 2^{130} \ll r_{\text{BLS}}$.
- **Row budget (`ESTIMATE`):** `~26–34` rows per active `ALU` slot (`3` `u128` decompositions + 2 gate rows) $\implies$ **`~416–544` rows for `16` `ALU` slots at `S = 16`** (`~1,664–2,176` rows for `64` slots at `S = 64`); **`+0` Node rows**.
- **Quint model:** `models/sherlock/quint/modAlu.qnt` — checks `invAddSubExact`, `invMulNoModWrap`, `invDivModUnique`, `invMaliciousFieldInputRejected` (tests `x = p - 1`).
- **`MockProver` mutants:** `alu_mut_unranged_input_lt_p_minus_1` (`Exploit D1`), `alu_mut_mul_overflow_flag_suppressed`, `alu_mut_sub_underflow_masked`, `alu_mut_divmod_rem_ge_divisor`, `alu_mut_inactive_slot_forges_tuple`.
- **Function skeleton & contracts:**
  ```rust
  /// Synthesize `n_slots` self-protected 128-bit ALU provider slots on `bus`.
  /// Requires: `tbl` configured on the same `Cols`.
  /// Ensures: for every slot with `act == 1`, `x < 2^128`, `y < 2^128`, `z < 2^128`, `flag in {0,1}`,
  ///          and `(z, flag)` equals the exact C6 `sherlock-helper-microstep/1` scalar result of `(op, x, y)`;
  ///          for `act == 0`, bus provider tuple is `(0, 0, 0, 0, 0)`.
  /// Check: `MockProver` rejects `alu_mut_*` including `x = F::ZERO - F::ONE`.
  pub fn synth_alu_region(
      cfg: &AluCfg, tbl: &TaggedTableCfg, r: &mut Region<'_, F>, o: &mut usize, slots: &[AluSlotW], names: &mut Names,
  ) -> Result<(), Error>;
  ```

---

#### Module `M4`: `MEM` (`WindowCfg` Backend-Neutral Frontier & `D7` Stacks — Frozen Seam)
- **Owned files:** `crates/mori-chip/src/membackend.rs` (`WindowCfg` — already implemented!) + `crates/mori-chip/src/stacks.rs` (`StacksCfg` for `D7` `fp_hash` / `tp_hash` LIFO chains).
- **Interface & Invariants:**
  - `WindowCfg::window` (`membackend.rs:375-459`) runs **once per arena** (`DATA`, `FIELD`, `ENV`, `SLOT`, `kfix = 1..=4`), decomposing `len_pre` and `len_post` into `TOP + 3 = 64` boolean bits (`base = 8 * q_pre`) and exposing `8 * (W + 1)` window rows (`W = 1` -> `16` rows at `S = 16`).
  - Active window rows (`act = 1`) serve `(CHIP_MEM, 8 * addr + kind, d0, d1, d2)` on `bus`; padding rows (`act = 0`) serve `(0,0,0,0,0)`.
  - Hands `&Window` (`len_pre, len_post, q_pre, base, rows[0..8*(W+1)]`) to `M5: MemBackend::persist` — **`WindowCfg` is 100% oblivious to whether `M5` is Merkle (`DensePagedBackend`) or LogUp (`ZmLogUpBackend`)**!
  - **Invariants (`membackend.rs:19-30`):** (I4.1) `0 <= len_pre <= len_post < 2^64`, `q_pre = floor(len_pre / 8)`. (I4.2) `imp <= act` boolean prefixes ending at `len_pre` and `len_post`; last window row has `act == 0` (`W_LAST`). (I4.3) `kind == kfix` (`1..=4`) on active rows and `0` on inactive rows. (I4.4) `StacksCfg` proves `fp_hash' = Poseidon(iv(DOM_FRAME, 0), [fp_hash, frame...])` on spill and inverse opening on fill.
- **Row budget (`ESTIMATE`):** Per arena at `TOP = 61, W = 1`: `128` frontier bit rows + `16` window rows = `144` rows $\times$ `4` arenas = **`576` rows** (`+ ~560–1,120` rows for `2` `D7` stack spill/fill slots); **`+0` Node rows** beyond `kfix` + 1 packed `VSel` column.
- **Quint model:** `models/sherlock/quint/modMemWindow.qnt` — verifies `invDensePrefix`, `invKindInjective`, `invNoLeafCloseGap` (`SI-12`), `invLifoStackExact`.
- **`MockProver` mutants (`dense/tests.rs:720-804`):** `c1_mut_counter_wrap_2_64`, `c1_mut_counter_field_wrap`, `c1_mut_pad_row_on_bus`, `c1_mut_kind_out_of_range`, `c1_mut_arena_kind_pinned`, `c1_mut_close_advances_len`, `c1_mut_append_value_mismatch`, `stack_mut_pop_wrong_prev_hash`.
- **Function skeleton & contracts:** Matches `WindowCfg::window` in `crates/mori-chip/src/membackend.rs:375-459` verbatim.

---

#### Module `M5`: `MEMORY BACKEND` (Swappable Trait `MemBackend`: `M5A` `DensePagedBackend` vs `M5B` `ZmLogUpBackend`)
- **Owned files:**
  - Trait definition: `crates/mori-chip/src/membackend.rs` (extending `MemBackend` with batch L1 cold-cache & `ROM`-cache providers on `bus`).
  - **Backend A (`M5A` — Default):** `crates/mori-chip/src/dense.rs` + `crates/mori-chip/src/page.rs` (`DensePagedBackend`: `M3-Tail64` `persist` + 8-cell aligned `PAGE` cold segment cache + 8-row `ROM` cold segment cache against `prog_root`).
  - **Backend B (`M5B` — Benchmark Alternative):** `crates/mori-chip/src/zmlogup.rs` (`ZmLogUpBackend`: Stage-1 `Instance(0)` `P_i` commitment + `(gamma, alpha)` rational sum updates on `new_write = act - imp` and cold cache reads).
- **Swappable Trait Contract (`CORE` and `WindowCfg` Untouched):**
  ```rust
  pub trait MemBackend {
      type ArenaToken; // M5A: [root, tail_d] (2 Cells); M5B: [tail_fp] + shared [d_acc, gamma, alpha, sum_mem]
      type LeafW;
      type ColdCacheW;
      type RomCacheW;

      /// Authenticate `win`'s `imp == 1` tail against `pre`, commit newly active records (`act - imp == 1`),
      /// and return the post `ArenaToken`.
      /// Requires: `win` produced by `WindowCfg::window` with `TOP = 61`.
      /// Ensures: `imp == 1` records equal the tail committed in `pre`; all `act == 1` records are bound into
      ///          the returned post `ArenaToken` (via `TOP=61` Merkle flush + `tail_d` in M5A, or `tail_fp` +
      ///          `+m_w / (alpha - enc_gamma(addr, rec))` into `sum_mem` in M5B).
      fn persist_arena(
          &self, pc: &PoseidonCfg, tbl: &TaggedTableCfg, r: &mut Region<'_, F>, o: &mut usize,
          arena_id: u64, win: &Window, pre: &Self::ArenaToken, w: &Self::LeafW, names: &mut Names,
      ) -> Result<Self::ArenaToken, Error>;

      /// Populate `K_page` deduplicated cold cache lines (`addr < win.base`) onto `bus` (`CHIP_MEM`).
      /// Requires: `pre` is the arena's pre-leaf token; `win.base == 8 * win.q_pre`.
      /// Ensures: every active provider row emitted on `bus` has `act in {0,1}`, `addr < win.base` (`Exploit A3`),
      ///          and `(addr, rec)` authenticated against `pre` (`TOP=61` walk to `pre.root` in M5A; `-1/(alpha-enc)`
      ///          in M5B); inactive cache rows emit `(0,0,0,0,0)` (`Exploit A2`).
      fn provide_cold_cache(
          &self, pc: &PoseidonCfg, tbl: &TaggedTableCfg, r: &mut Region<'_, F>, o: &mut usize,
          arena_id: u64, base: &Cell, pre: &Self::ArenaToken, w: &Self::ColdCacheW, names: &mut Names,
      ) -> Result<(), Error>;

      /// Populate `K_rom` deduplicated 8-row program/type/const cache lines onto `bus` (`CHIP_ROM`).
      /// Requires: `prog_root` is copy-constrained to the leaf statement's `prog_d` root.
      /// Ensures: active rows emit `(CHIP_ROM, 8*idx + sub, w0, w1, w2)` authenticated against `prog_root`;
      ///          inactive rows emit `(0,0,0,0,0)`.
      fn provide_rom_cache(
          &self, pc: &PoseidonCfg, r: &mut Region<'_, F>, o: &mut usize,
          prog_root: &Cell, w: &Self::RomCacheW, names: &mut Names,
      ) -> Result<(), Error>;
  }
  ```
- **Row budget (`ESTIMATE`):**
  - **`M5A` (`DensePagedBackend`, `S = 16`, `TOP = 61`, `W = 1`/arena, `K_page = 2` 8-cell lines, `K_rom = 2` 8-row lines, `TOP_ROM = 16`):**
    - `persist_arena` $\times 4$: per arena `15` `rec_d` (`2,100` rows) + `3` `tree8` (`1,470` rows) + `2` `TOP = 61` shared-bit walks (`8,540` rows) + masks/mux (`26` rows) = `12,136` rows/arena (`or ~9,336` rows/arena on arenas configured with `W = 1` where `r_pre + used` uses lazy `1`-unit walk; **optimization:** sharing `W_global = 2` dynamic flush walkers across all 4 arenas cuts `persist` across all 4 arenas from `48,544` rows to **`~31,400` rows `ESTIMATE`**!) + `2` 8-cell `PAGE` lines (`2 × 5,889 = 11,778` rows) + `2` 8-row `ROM` lines (`2 × 2,739 = 5,478` rows) = **`~48,656` rows (`k_leaf = 16`, `< 65,530`)**, **`+0` Node rows** over `DenseCfg`!
  - **`M5B` (`ZmLogUpBackend` + `ROM` Merkle cache, `S = 16`):** `~550` LogUp Horner/inverse rows + `5,478` `ROM` cache rows = **`~6,028` leaf rows (`k_leaf = 13..14`)**, **`+11,000..24,000` Node rows** and **`+26,400..33,500` `RunLift` `R` rows (`ESTIMATE`)**.
- **Quint models:** `models/sherlock/quint/modBackendDensePaged.qnt` (`M5A`) and `models/sherlock/quint/modBackendZmLogUp.qnt` (`M5B`), both verified against the **identical** abstract `MemBackendSpec` contract!
- **`MockProver` mutants:**
  - `M5A`: `dense_mut_old_walk_nonempty` (`x0 != EMPTY_SEG`), `dense_mut_no_flush_mutates_root`, `dense_mut_tail_open_tampered`, `page_mut_cold_addr_ge_base` (`Exploit A3`), `page_mut_inactive_line_nonzero` (`Exploit A2`), `rom_mut_uses_mem_root` (`Exploit A5`).
  - `M5B`: `logup_mut_uncommitted_pi_tamper`, `logup_mut_inactive_inv_nonzero`, `logup_mut_wrong_multiplicity_unbalanced`, `logup_mut_forged_tail_fp`.

---

#### Module `M6`: `HASH` (`PoseidonCfg` — Width-3 Sponge & Capacity-Tagged Bus Provider)
- **Owned files:** `crates/mori-chip/src/poseidon.rs` (already implemented, `797` lines; add `act_hash`-gated `q_prov` row for variable-count bus hashes).
- **Interface & Invariants:**
  - `sponge_iv(r, o, ins, ivv, provide_on_bus, ...)` (`poseidon.rs:107-120, 519`).
  - **Invariants:** (I6.1) Capacity element starts at `len + iv(dom, sub)` folded into fixed round-constant column `rc[2]` (`poseidon.rs:22-26, 101-106`), so prover **cannot** forge or swap domain tags (`Exploit E1`). (I6.2) Distinct `(dom, sub, len)` triples have strictly disjoint initial capacities (`iv` is a multiple of $2^{32}$, `len < 2^{32}`). (I6.3) Unused bus-provider hash slots (`act_hash = 0`) multiply `(CHIP_HASH, key, in0, in1, out)` by `act_hash = 0`.
- **Row budget:** Measured **`70` rows per permutation** (`poseidon.rs:59`); **`+0` Node rows**.
- **Quint model:** `models/sherlock/quint/modHash.qnt` (`invDomainDisjoint`, `invDisabledZeroTuple`).
- **`MockProver` mutants:** `hash_mut_round_sbox_tamper`, `hash_mut_chain_prev_break`, `hash_mut_disabled_provider_nonzero`, `hash_mut_cross_domain_replay`.

---

#### Module `M7`: `VALUE DIGEST` (`ValDigestCfg` — `sherlock-value/1` with `(SHK_VAL, 1)` Prefix)
- **Owned files:** `crates/mori-chip/src/valdigest.rs`.
- **Interface & Invariants:**
  - Computes canonical address-free `sherlock-value/1` digests (`SRC-022`, `crates/mori-core/src/sherlock/value_digest.rs:76-152`) and serves `(CHIP_VD, val_kind, x0, x1, vd)` on `bus` (plus incremental `BuildPhase::Copy` child-`vd` sponge accumulation).
  - Initializes dynamic `vd` sponges from compile-time constants `(C0, C1, C2) = PoseidonPerm([SHK_VAL, 1, len + iv])` (`-70` rows/digest) and exposes compile-time constants `VD_NIL`, `VD_FALSE`, `VD_TRUE`.
  - **Invariants:** (I7.1) Bit-for-bit equality with `mori_core::sherlock::value_digest::value_digest` across `Nil`, `Bool`, `UInt128(lo64, hi64)`, `Constructor(ty_d, ord, arity, child_vds...)`, `Fault(kind, code)`, and `OutOfFuel`. (I7.2) Constructor `ty_d` is authenticated from `CHIP_ROM` (`Types` subtable) and child `vd`s from `CHIP_MEM` (`DATA` record slot `d2`), closing `Exploit E2`.
- **Row budget (`ESTIMATE`):** `0` rows for `Nil`/`Bool`; `70` rows for `UInt128` (`1` perm instead of `2`); `140 + 70 * ceil(arity / 2)` rows for `Constructor` $\implies$ **`~420–840` rows/leaf at `S = 16`**; **`+0` Node rows**.
- **Quint model:** `models/sherlock/quint/modValDigest.qnt` (`invAddressFreeEq`, `invPrefixMatchesFullSponge`, `invTamperedVdRejected`).
- **`MockProver` mutants:** `vd_mut_prefix_mismatch_ref`, `vd_mut_unauthenticated_ctor_vd` (`Exploit E2`), `vd_mut_uint128_limb_swap`, `vd_mut_fault_collides_value`.
- **Function skeleton & contracts:**
  ```rust
  /// Verify `sherlock-value/1` digest slots from precomputed `(SHK_VAL, 1)` sponge state.
  /// Requires: `UInt128` inputs `(lo64, hi64)` range-checked `< 2^64` via `TaggedTableCfg`.
  /// Ensures: emitted `vd` equals `mori_core::sherlock::value_digest` on the semantic value.
  /// Check: differential test over 1,000 random nested ADTs/scalars/outcomes + rejects `vd_mut_*`.
  pub fn synth_vd_slots(
      cfg: &ValDigestCfg, pc: &PoseidonCfg, r: &mut Region<'_, F>, o: &mut usize, slots: &[VdSlotW], names: &mut Names,
  ) -> Result<(), Error>;
  ```

---

#### Module `M8`: `BOUNDARY COMMITMENT` (Swappable Trait `BoundaryScheme`: `M8A` `PoseidonBoundaryCfg` vs `M8B` `HornerBoundaryCfg`)
- **Owned files:** `crates/mori-chip/src/boundary.rs`.
- **Interface & Invariants (`MODULARITY.md:7`):**
  - Wraps the canonical boundary vector `StateVec` (`29` `CORE` architectural cells + `fp_hash` + `tp_hash` + `4` arena `(len, ArenaToken)` = `43` scalars under `M5A`) into the leaf's public statement (`Instance(1)` and optional `Instance(0)`):
    - **`M8A` (`PoseidonBoundaryCfg` — Default, paired with `M5A`):** `Instance(0) = [C::identity()]`; `Instance(1) = [h_pre, pre_d, post_d, prog_d, input_d, outcome_d, done, steps]` (`8` scalars, identical to `ivc.rs:263-273`!), where `pre_d = sponge_iv(DOM_STATE, StateVec_pre)` and `post_d = sponge_iv(DOM_STATE, StateVec_post)` (`22` perms each = `3,080` rows total `ESTIMATE`).
    - **`M8B` (`HornerBoundaryCfg` — Benchmark Alternative, paired with `M5B`):** Commits `StateVec_pre` and `StateVec_post` inside `Instance(0)` in Stage 1 and evaluates $\sum_j \gamma^j u_j$ in `~86` leaf rows (`ESTIMATE`), exposing `[h_pre, fp_pre, fp_post, prog_d, input_d, outcome_d, done, steps, d_com, gamma, alpha, sum_pre, sum_post]` in `Instance(1)`.
  - **Invariants:** (I8.1) Collision-resistant injectivity (`digest_inj`, `Aggregation.lean:201`) over `StateVec`. (I8.2) Genesis check (`h_pre == 0`): constrains initial `regs`, empty stacks `fp_hash = tp_hash = EMPTY_STACK`, `arena_len = [n_d_init, n_f_init, 0, 0]`, and input admission binding to `input_d`. (I8.3) Terminal check (`done == 1`): constrains `outcome_d` to the canonical `sherlock-value/1` outcome digest (`Halted(vd)`, `Fault(kind, code)`, or `OutOfFuel`).
- **Quint model:** `models/sherlock/quint/modBoundary.qnt` (`invDigestInj`, `invGenesisExact`, `invTerminalOutcomeExact`).
- **`MockProver` mutants:** `bnd_mut_pre_post_splice`, `bnd_mut_genesis_dirty_heap`, `bnd_mut_done_forged_outcome_d`, `bnd_mut_horner_uncommitted_cell`.

---

#### Module `M9`: `CONTINUATION` (`Tier 1` Host Checkpoint & `Tier 2` `SHK_CONT_V1` Splice)
- **Owned files:** `crates/mori-chip/src/cont.rs` + `crates/mori-recursion/src/cont.rs`.
- **Interface & Invariants (`CONSENSUS.md` §1.3):**
  - **Tier 1:** Deterministic snapshot serializer `(h, StateVec, sparse_arenas, stack_chains, acc_pis)` so single-run proving stops at any leaf boundary and resumes later with the **exact same** `[prog_d, input_d, outcome_d, n_steps]`.
  - **Tier 2 (`SHK_CONT_V1`):** Binds adjacent epoch segments `[h_0, h_1)` (`post_d == s_1`) and `[h_1, h_2)` (`pre_d == s_1`) in `O(1)` inside `N`'s `SEG + SEG -> SEG` join (`Mode 1`, invisible on-chain) or emits `claim_d = Poseidon(SHK_CONT_V1, prog_d, input_d, h_0, s_0, h_1, s_1, done, outcome_d)` (`Mode 2`).
- **Quint model:** `models/sherlock/quint/modContinuation.qnt` (`invResumeSameC6Claim`, `invSpliceSoundness`, `invNoCrossEpochReplay`).
- **`MockProver` mutants:** `cont_mut_height_gap`, `cont_mut_state_digest_mismatch`, `cont_mut_prog_d_swap`, `cont_mut_double_halt`.

---

#### Module `M10`: `NODE INTEGRATION + DECIDER` (`{R, F, N, C, D}` & `BoundaryAdapter`)
- **Owned files:** `crates/mori-recursion/src/{inner.rs, ivc.rs, run_lift.rs, run_decider.rs}`.
- **Interface & Invariants (`MODULARITY.md:9`):**
  - `BoundaryAdapter` abstracts child statement chaining across `M8A` (`PoseidonBoundaryCfg`: `left.post_d == right.pre_d`, `committed = [C::identity()]`) and `M8B` (`HornerBoundaryCfg`: witnessed `Instance(0)` $C_i$ with subgroup check + `d_tree = Poseidon(DOM_TREE, d_L, d_R)` + `sum_seg = sum_L + sum_R` + constant `(gamma, alpha)`).
  - Enforces `h_next = h_cur + steps` with `64`-bit range check (`H_BITS = 64`, `ivc.rs:24, 79`), `SET.n_claims < 2^64` (`CONSENSUS.md` `Exploit E-1`), and **zero** unloaded `Pow2Range` tags `{2,3,5,6,7,9..13}` in `RunDeciderRelation` (`table_rows == 1,032,467` asserted in CI!).
- **Row budget (`ESTIMATE`, Backend A + Tagged `DEC+RANGE`):**
  - `k = 20` Node `N`: **`~800,866`–`807,328` rows** (`142,000+` headroom below `950,000` gate!).
  - `k = 20` `RunLift` `R` (`U + U -> SEG`): **`~735,000`–`775,000` rows** (`175,000+` headroom below `950,000`!).
  - `k = 20` `RunDecider` `D`: **`~929,166`–`931,000` circuit rows**, **exact `1,032,467` table rows** (`16,103` margin intact).
- **Quint model:** `models/sherlock/quint/modNodeDag.qnt` (`invTreeAssociative`, `invDiamondIdempotentSet` [`Exploit G1`], `invClaimCountNoWrap`).
- **`MockProver` / `cost_model` checks:** `node_mut_chain_digest_break`, `node_mut_height_overflow_2_64`, `decider_assert_table_rows_1032467`, `r_mut_second_leaf_dropped`.

---

### 4.3 Dependency Graph, Parallel Build Waves & Whole-Leaf Integration Harness (`MODULARITY.md:11-12`)

```
Wave 0 (Freeze Contracts & Traits — 1 day, serial):
  crates/mori-chip/src/{lib.rs, bus.rs, membackend.rs, boundary.rs}
       │
       ├──► Wave 1 (7 Independent Parallel Module Lanes — zero file overlap!):
       │      ├─ Lane 1: M2 `table.rs` (Tagged DEC+RANGE + 108-rule partition.json compiler)
       │      ├─ Lane 2: M3 `alu.rs` (Self-ranged ADD/SUB/MUL64/DIVMOD/LT/EQ)
       │      ├─ Lane 3: M4 `stacks.rs` + `membackend.rs` tests (D7 LIFO chains + 4-arena WindowCfg)
       │      ├─ Lane 4: M5A `dense.rs` + `page.rs` (Shared-walker M3-Tail64 + 8-cell PAGE & ROM cache)
       │      ├─ Lane 5: M6 + M7 `poseidon.rs` + `valdigest.rs` ((SHK_VAL,1) prefix + VD constants)
       │      ├─ Lane 6: M1 `core/{slot,regs,unpack}.rs` + 7 rule-family sub-lanes (`rules/*.rs`)
       │      └─ Lane 7 (Optional A/B): M5B + M8B `zmlogup.rs` + `HornerBoundaryCfg`
       │
       ├──► Wave 2 (Whole-Leaf Composition & Differential Oracle — `crates/mori-chip/src/leaf_u.rs`):
       │      Compose M1 + M2 + M3 + M4 + M5A + M6 + M7 + M8A -> Universal Leaf `U`
       │
       └──► Wave 3 (Recursion & Continuation Integration — `crates/mori-recursion/`):
              M9 (`cont.rs`) + M10 (`R`, `N`, `C`, `D` + `cost_model` gates + E2E real KZG proof)
```

**Whole-Leaf Differential Integration Harness (`MODULARITY.md:12`, `crates/mori-chip/tests/u_differential.rs`):**
1. **Reference Oracle:** Executes real `sherlock-k-a/1` programs (`entry 7`, `fact(3)`, `fold`, `list123`, `> 8`-field cross-leaf `Construct` for `SI-12` / `GP-001`, `0`-append pure-compute loop, explicit `Fault`, and `OutOfFuel`) through `crates/mori-core`'s reference machine (`owner_runtime.rs`), recording the exact semantic state and `sherlock-value/1` digest at every step $h$.
2. **Multi-Partition & Backend Swap Equivalence:** Synthesizes and `MockProver`-verifies the **exact same trace** across leaf schedules `S = 16`, `S = 64`, irregular early-close cuts `[1, 0_alloc, 7, 9, ...]`, and Tier-1/Tier-2 stop-and-resume cuts, asserting:
   - Bit-identical `canon_state_d(h)` at every shared boundary step $h$ (`SRC-023` + `Aggregation.lean` `digest_inj`);
   - Bit-identical final `[prog_d, input_d, outcome_d, n_steps]` across Backend A (`DensePagedBackend`) and Backend B (`ZmLogUpBackend`);
   - Automated raw-witness fault injection (Arguzz-style single-cell tamper across `Names` registry of all 10 modules): 100% of output-altering or invariant-breaking mutations rejected by `MockProver::verify().is_err()`.

---

## 5. Final Converged Positions on `Q1`–`Q5`

- **`Q1` (What the Cache Holds):** Adopt **(a)** intra-leaf 8-cell cold segment `PAGE` caching (`CHIP_MEM`) + **(c)** intra-leaf 8-row `ROM` caching (`CHIP_ROM` under `prog_root`) + **merged 2-column tagged `DEC + RANGE` fixed table** (`6,145` rows, `1` `cs.lookup`) + **(d)** self-ranged gate `ALU` (`ADD/SUB/MUL64/DIVMOD/LT/EQ`) + **(e)** inline `SVM-012` stored `vd` with `(SHK_VAL, 1)` prefix precomputation & `VD_{NIL,FALSE,TRUE}` constants, backed by **(f)** 4-arena `M3-Tail64` (`TOP = 61`, $2^{64}$ cells/arena) + `D7` LIFO hash-chain stacks (`fp_hash`, `tp_hash`). Reject `ZM-LogUp` and `EC-MSH` as primary backends; isolate `ZM-LogUp` behind `MemBackend` + `BoundaryScheme` as optional Backend B.
- **`Q2` (Recursion Fit & Wall-Clock):** Stock `midnight-circuits = 7.3.0` (`phases == 1`, rotations `{-1, 0, +1}`) is preserved with **zero fork**. Merged `DEC + RANGE` reduces leaf lookups `4 -> 3`, cutting `k = 20` Node rows by **`-29,062` (`~794k–807k` rows `ESTIMATE`, below E8's `823,466`!)** and `GP-4` `RunLift` `R` rows by **`-51,662` (`ESTIMATE`)**, while leaving `RunDecider` `table_rows` at **exact `1,032,467`**. Freeze `S = 16`, `k_leaf = 16` (`~22.8 min` `list123` linear IVC `ESTIMATE`) for M4; benchmark parameterized `S = 64`, `k_leaf = 17` (`~8.0 min` `ESTIMATE`) post-M4.
- **`Q3` (Protocol & General-Purpose Interactions):** Satisfies C6 outcome-only publication, step-exact `canon_state_d(h)` partition independence (`SRC-023` + `Aggregation.lean:201, 1154` unchanged), $2^{64}$ step & cell ceilings with zero pure-compute segment burn (`GP-001`), verbatim `start + k` `SI-12..SI-14` field slices via 4-arena dense counters, `O(1)` Tier-2 continuations, and full `GP-1..GP-4` (`U, R, F, N, C, D`) DAG composition.
- **`Q4` (Prior Art):** Directly mirrors production continuations in RISC Zero (`PagedMemory` Merkle page-in/out) and OpenVM (`PersistentBoundaryAir` Merkle boundary + intra-segment bus) combined with Cairo's write-once continuous memory and Airbender/Sherlock's inductive range-check discipline, while avoiding the disclosed soundness pitfalls of `stwo-cairo` (unabsorbed `claimed_sum`), OpenVM (`GHSA-fh29-29h9-qm9h` non-boolean bus flags, `CVE-2026-62333` variable-depth Merkle splice), SP1 (`alu` cross-chip precondition gap), and `ePrint 2026/590` (unhashed `EC-MSH` curve relation).
- **`Q5` (First Prototype & Execution Order):** Execute Wave 0 (`membackend.rs` cold-cache/ROM extension + `boundary.rs` traits) on top of the existing C1 `WindowCfg`/`DenseCfg` scaffold (`crates/mori-chip/src/{membackend.rs,dense.rs}`), followed in parallel by Wave 1 (`table.rs` Tagged `DEC+RANGE`, self-ranged `alu.rs`, shared-walker `page.rs` + `dense.rs`, `(SHK_VAL,1)` `valdigest.rs`, `stacks.rs`, and `core/`), gated by `cost_model` (`lookups == 3`, `node.rows <= 815,000` `ESTIMATE`, `decider.table_rows == 1,032,467`), per-module Quint models, and raw-witness `MockProver` mutant suites.

---

## 6. What Was Not Checked

1. **No builds (`cargo check`/`cargo test`), `cost_model`, `MockProver` (including the new `c1_rows` ignored test at `crates/mori-chip/src/dense/tests.rs:841`), Quint, Lean, or real KZG proving runs were executed** during Round 2 (`BRIEF.md:65` no-heavy-commands rule). Every unmeasured row count, `k` bound, memory figure, and wall-clock projection is calculated from measured repository baselines and marked `ESTIMATE`.
2. **Exact row output of `c1_rows` (`dense/tests.rs:841-874`) and shared-walker 4-arena `DenseCfg`:** While `membackend.rs` and `dense.rs` were inspected line by line (`1,256` lines total), `c1_rows` has not yet been executed in this study to record the exact `circuit_model` row count of `DenseCfg` before sharing flush walkers across the 4 arenas.
3. **Worst-case per-step `bus` request count across all 108 `/1` rules & empirical early-close step average $\bar{s}$ at `S = 64`:** Whether every one of the 108 rules fits within `4` `bus` requests/slot (`2` request rows in `M1: CORE`) or requires `6` requests/slot (`3` request rows = `+16` leaf rows at `S = 16`, `+0` Node rows) must be verified mechanically when compiling `partition.json` in Wave 1 (`M1`/`M2`).
4. **Formal proof or refutation of Midnight's `SelfEmulation::assign_without_subgroup_check` `TODO` (`midnight-circuits-7.3.0/src/verifier/types.rs:126`):** Backend A (`M5A` + `M8A`) never calls `assign_without_subgroup_check` on untrusted public-input commitments (`Instance(0)` stays fixed `C::identity()`); if Backend B (`ZmLogUpBackend`) is ever benchmarked in `mori-recursion`, it should use subgroup-checked `AssignmentInstructions::assign` until `types.rs:126` is formally discharged.
5. **Upstream language/pin items outside `mori-chip`:** Canonical normal form of `decl_schema_d` (`dissent.md` `D-2`), `VAL_NONE` encoding (`design.md:214`), `ENTRY`/`ARG` rule inventory additions (`design.md:216`), and `midnight-proofs` `0.8.2` vs `0.8.3` byte parity (`tasks.md` `2.17`).
6. **Strict workspace isolation:** No files in `research/sherlock-final-design-20261111/drafts/*` and no other lane's `round2.md` were listed or read; only `/home/hoskinson/Projects/Moriarty/research/sherlock-lookup-cache-20261111/lanes/D/round2.md` was written.
