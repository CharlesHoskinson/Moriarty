# Modular Sherlock Architecture — Area 1: The Processor (`BUS`, `CORE`, `DEC`, `ALU + RANGE`, `HASH`)

Date: 2026-11-11
Scope: Area 1 (`research/sherlock-final-design-20261111/ARCH-BRIEF.md:19, 23-38, 48-68`; `research/sherlock-final-design-20261111/MODULARITY.md:1-12`).

---

## 0. Conventions, Verbatim `GP-1`–`GP-4` Target, and Area 1 Proof-System Envelope

### 0.1 Citation and Estimate Conventions

- Repository paths are relative to `/home/hoskinson/Projects/Moriarty/`.
- `MP:<path>:<lines>` denotes pinned `midnight-proofs-0.8.3/src/<path>:<lines>`.
- `MC:<path>:<lines>` denotes pinned `midnight-circuits-7.3.0/src/<path>:<lines>`.
- Scalar field `F` is BLS12-381 `Fq` (`crates/mori-chip/src/lib.rs:41`), prime modulus `r = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001` (`254.86` bits; `2^254 < r < 2^255`).
- Every unmeasured row count, column-fit delta, or proving-time figure is tagged `ESTIMATE`. Measured figures cite their exact source and line.

### 0.2 Verbatim Preservation of `GP-1` through `GP-4`

Area 1 preserves `GP-1` through `GP-4` (`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:7-53`; `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:56-59`) verbatim without narrowing:

- **`GP-1` Universal VM (`general-purpose.md:7-17`, `requirements.md:56` `GP-001`):**
  > "One fixed key set proves the execution of every program of the Sherlock language, on every input, for every run length and every outcome. A new program needs no new key, no new contract and no new ceremony."
  - Programs and inputs are data (`prog_d`, `input_d` in public statement, never circuit constants; `general-purpose.md:11`).
  - Every outcome is provable: `Halted(v)` for every value type (as a `sherlock-value/1` canonical value digest), `Rejected(code)`, and `Exhausted(kind)` (`general-purpose.md:12`).
  - Capacity does not depend on the key: state exceeding in-leaf carriers runs against committed memory (`general-purpose.md:13`).
  - Run length is bounded only by the 64-bit step counter `h < 2^64` (`SRC-004`, `general-purpose.md:14`, `requirements.md:56`): no circuit domain, key, final proof size, leaf count, memory address counter, or segment index sets a lower limit, and a step that allocates no memory consumes no memory address. Proving may stop and resume under the same key set with identical `(prog_d, input_d, outcome_d, n_steps)`, and execution is partition-independent (`SRC-023`, `requirements.md:27`).
- **`GP-2` Cross-run composition (`general-purpose.md:20-28`, `requirements.md:57` `GP-002`):**
  > "A proven run can be consumed by another proof, and many runs can be combined into one proof."
  - Consumed claims `(prog_d', input_d', outcome_d')` are sealed terminal claims checked in-circuit; when the consumer inspects the value behind `outcome_d'`, it verifies the `sherlock-value/1` opening (`requirements.md:57`).
- **`GP-3` Foreign proofs (`general-purpose.md:31-39`, `requirements.md:58` `GP-003`):**
  > "The recursion verifies and aggregates proofs of circuits other than the Sherlock leaf. These include Compact contract circuits and other PLONK/halo2 circuits on the same curve and commitment scheme."
  - Foreign verifying keys are authenticated against a registry commitment in-circuit and mapped via an authenticated statement adapter (`requirements.md:58`).
- **`GP-4` Any shape (proof-carrying data) (`general-purpose.md:42-51`, `requirements.md:59` `GP-004`):**
  > "Aggregation is not limited to a linear chain: it covers chains, trees and DAGs of any depth, with a constant-size final proof."
  - Node arity `>= 2` plus new work under one key set; shape commitment binds address-free descriptors only (`requirements.md:59`).

**Area 1 Non-Narrowing Consequences:** No Area 1 module bakes program AST constants, `Bounds`, rule subsets, leaf indices, or step-indexed memory offsets into verifying keys (`GP-001`). `CORE` tracks `h < 2^64` (`SRC-004`), advances allocation counters strictly by real allocations (`GP-001`), preserves `SRC-023` partition independence across arbitrary leaf cuts (`AUDIT.md:67-69`), and exposes `CHIP_HASH = 6` (`DOM_CLAIM`, `SHK_VAL`) so `GP-2` claim openings execute as ordinary data-driven steps.

### 0.3 Area 1 Recursive Verifier Envelope (`VerifierGadget` Hard Constraints)

Every leaf circuit proved under Area 1 is verified inside the recursive node by `midnight-circuits-7.3.0` `VerifierGadget` (`MC:verifier/verifier_gadget.rs`). Three hard constraints govern every Area 1 gate, lookup, and table (`research/sherlock-lookup-cache-20261111/CONSENSUS.md:69-83`, `research/sherlock-runlength-consensus-20261111/AUDIT.md:89`):

1. **Strictly Single-Phase Advice (`Phase(0)` Only):** `VerifierGadget::verify` asserts `assert_eq!(cs.phases().count(), 1)` (`MC:verifier/verifier_gadget.rs:310`) and absorbs all `0..cs.num_advice_columns()` commitments in one transcript pass before squeezing internal PLONK `theta`, `beta`, `gamma`, and `trash_challenge` (`MC:verifier/verifier_gadget.rs:314-341`).
2. **Zero Verifier Challenges Inside Gates:** `MC:verifier/expressions/mod.rs:49` panics on `Expression::Challenge(_)` (`panic!("We do not suport multi-phase yet")`). No Area 1 module may allocate `Phase(1)` columns or reference `Challenge` in `create_gate` or `lookup_any`.
3. **Unit Advice/Fixed Rotations Only (`{-1, 0, +1}`):** `MC:verifier/verifier_gadget.rs:604-610` (`get_point`) maps `rotation.0` exclusively over `-1 => &x_prev`, `0 => &x`, `1 => &x_next`, panicking on `|rotation.0| > 1`. Multi-row connections (`CORE` step boundaries, `PoseidonCfg` block chaining) MUST use adjacent rows or copy constraints (`constrain_equal`), never `Rotation(+-2)` or wider.

### 0.4 Leaf Shape and Recursive Node Cost Model Baseline

- **Measured E8 Leaf Baseline (`openspec/changes/sherlock-vm-consensus-20261107/design.md:125, 130-131`; `crates/mori-chip/src/rfcore.rs:833-843`; `research/sherlock-lookup-cache-20261111/CONSENSUS.md:129-136`):**
  - E8 1-leaf recursive node (`k = 20`, gate `<= 950,000` rows, `design.md:105`): `823,466` circuit rows (`design.md:130`).
  - E8 decider (`k = 20`, gate `<= 1,000,000` circuit rows, usable `2^20 - 6 = 1,048,570` rows, `design.md:105`): `929,166` circuit rows and `1,032,467` `Pow2Range` table rows (`16,103` `Pow2Range` rows of headroom below `1,048,570`, `design.md:131`, `AUDIT.md:76`).
- **Linear Recursive Node Sensitivity (`design.md:134`, `crates/mori-chip/src/vsel.rs:4`):**
  - 1-leaf `ChipNode` row model (`ESTIMATE` for deltas off E8):
    `node_rows ≈ 570,957 + 7,298 * N_adv + 8,189 * N_eq + 6,004 * N_fix + 22,600 * N_lookup` (`~6,460` Node rows per filled fixed/table column in E6/E8 marginal measurements, `vsel.rs:4`, `CONSENSUS.md:145`).
  - 2-leaf `RunLift` node `R` (`dissent.md:88`, `CONSENSUS.md:134-136`): verifies 2 leaves (`+1` leaf verifier pass), scaling per-leaf column/lookup marginal costs by `ESTIMATE ~1.76x..1.78x` (`~40,000` `R` rows per PLONK lookup; `~11,200..11,660` `R` rows per fixed/table column).
  - Leaf row count `2^{k_leaf}` enters `ChipNode` only logarithmically (`+4` Node rows per `+1` increase in `k_leaf`, `+0` Decider rows; `design.md:135`).

---

## 1. Module `BUS` — Inter-Module Dynamic Lookup Bus, 2-Column `TaggedTableConfig`, and Copy-Cell Protocol

### 1.1 Purpose

`BUS` defines the sole inter-module communication substrate of the Sherlock leaf circuit, Partitioned into two dynamic 5-tuple `lookup_any` channels (`Bus`, `crates/mori-chip/src/bus.rs:33-62`), one merged 2-column domain-tagged static `lookup` table (`TaggedTableConfig`), and explicit boundary cell-copy ports (`constrain_equal`). It enforces module isolation (`MODULARITY.md:7-10`) so every processor and memory module is implemented, tested, and formally verified solely against its tuple contract without reading any sibling module's columns or polynomial gates.

### 1.2 Frozen Interface

#### A. Dynamic Lookup Bus Schema (`crates/mori-chip/src/bus.rs:1-62`, Extended)

Every dynamic bus tuple is a 5-element `Fq` vector `(chip, key, a, b, c)` carried over shared advice columns `a[0..5]` (channel `bus0`) and `a[5..10]` (channel `bus1`). Both channels look up against the single provider column slice `a[0..5]` (`bus.rs:50-61`):
- Request side (`q_req` selector): `(q_req * chip, q_req * key, q_req * a, q_req * b, q_req * c)`.
- Provider side (`q_prov` selector): `(q_prov * chip, q_prov * key, q_prov * a, q_prov * b, q_prov * c)`.
- Disjointness gate (`bus.rs:64`): `Expression::Constant(0)` structurally enforced by region layouter disjointness and explicit invariant `q_req(row) * q_prov(row) == 0`.

**Why `DEC` and `RANGE` Moved Off the Dynamic Bus onto 2-Column `TaggedTableConfig`:**
1. *Static vs witness-dependent tables:* `DEC` (`<= 2,048` rows) and `RANGE` (`4,096` or `16,384` rows) depend only on the Sherlock `/1` language specification, never on program or run witness (`GP-001`). Serving them over the dynamic witness bus (`a[0..5]` with `q_prov = 1`) would require materializing `6,144..18,432` advice rows plus multiplicity or set-inclusion witness assignments per leaf, wasting advice capacity under `k_leaf = 16`.
2. *Eliminating 1 PLONK lookup and 1 fixed `TableColumn`:* In the E8 spike (`rfcore.rs:839-843, 941`, `dec.rs:25-30`, `range.rs:94-102`), `RANGE` used 1 `TableColumn` + 1 PLONK `lookup` and `DEC` used 2 `TableColumn`s + 1 PLONK `lookup` (`3` `TableColumn`s, `2` static lookups, `4` leaf lookups total). Packing a 1-bit domain tag into bit `16` of the key column merges both static tables into **2 `TableColumn`s (`t_dom_key`, `t_val`) and 1 PLONK `lookup`**, cutting total leaf PLONK lookups `4 -> 3` and fixed `TableColumn`s `3 -> 2` (`ESTIMATE -29,062` 1-leaf `ChipNode` rows and `ESTIMATE -51,204..-51,662` 2-leaf `RunLift` `R` rows; `CONSENSUS.md:145, 208`).
3. *Retiring `chip = 1`:* Production reserves `chip = 1` unused (`UNASSIGNED`) so legacy spike tuples (`CHIP_ALU = 1` in `bus.rs:30`) fail fast rather than aliasing production `CHIP_ALU = 2`.

#### B. Frozen Dynamic `chip` ID Registry (`0, 2, 3, 4, 5, 6`)

| `chip` | Constant | Requester(s) | Provider(s) | `key` Layout & Bit-Width | `a` (`col 2`) | `b` (`col 3`) | `c` (`col 4`) | Range-Check Owner |
|---|---|---|---|---|---|---|---|---|
| `0` | `CHIP_NOP` | `CORE` (unused lanes, `is_pad = 1` rows) | Any row with `q_prov = 0` (evaluates to `(0,0,0,0,0)`) | `0` (`0` bits) | `0` | `0` | `0` | `CORE`: constrains `(key, a, b, c) == (0,0,0,0)` whenever `chip == 0` (`D8` + requester zero-gate). |
| `1` | *(reserved)* | None | None (`0` provider rows) | Unsatisfiable if `q_req = 1` | — | — | — | Structural absence on `q_prov = 1` rows. |
| `2` | `CHIP_ALU` | `CORE`, `MEM` (bounds, if not inline) | `ALU` (`alu.rs`) | `op_key = op \| (ovf << 4)` (`5` bits: `op in 1..=9`, `ovf in {0,1}`) | `x` (`u128`, `< 2^128`) | `y` (`u128`, `< 2^128`) | `w` (`u128`, `< 2^128`) | **`ALU` unconditionally** range-checks `x, y, w < 2^128` via `RANGE` (`tag = 0`), plus boolean `ovf` and opcode selector `op in 1..=9` (`Exploit D1` fix). |
| `3` | `CHIP_MEM_ALLOC` | `CORE` (heap/env/list/ctor cell append) | `MEM` (`WindowCfg`, `membackend.rs`) | `mem_key = (arena << 67) \| (addr << 3) \| kind` (`70` bits: `arena in 0..=3`, `addr < 2^64`, `kind in 1..=7`) | `f0` (`< r`; `< 2^128` when scalar) | `f1` (`< r`; `< 2^128` when scalar/ptr) | `f2` (`< r`; `< 2^128` when scalar/len) | `MEM` checks `addr == len_curr` (sequential append, `GP-001`), `addr < 2^64` (`RANGE`), `arena`, `kind`; `CORE` + `ALU` range-check scalar payloads `< 2^128` on scalar write rules. |
| `4` | `CHIP_MEM_READ` | `CORE`, `VALUE DIGEST` | Disjoint split (`D2`): `MEM` (`base <= addr < len_post`) $\sqcup$ `MEMORY BACKEND` (`addr < base`) | `mem_key = (arena << 67) \| (addr << 3) \| kind` (`70` bits: `arena in 0..=3`, `addr < 2^64`, `kind in 1..=7`) | `f0` (`< r`) | `f1` (`< r`) | `f2` (`< r`) | `MEM` / `MEMORY BACKEND` owns `addr` range (`base <= addr < len_post` vs `addr < base`, `D2`, `membackend.rs:29, 258-278`); `CORE` (via `ALU` `OP_LT_U`) enforces `addr < len_curr` before read (`AUDIT.md:50`). |
| `5` | `CHIP_ROM` | `CORE` (AST, const, decl, dispatch plan rows) | `ROM` (`PAGE` static cache under `prog_d`) | `rom_key = (tbl_id << 32) \| row_idx` (`36` bits: `tbl_id in 1..=14`, `row_idx < 2^32`) | `col0` (`< 2^128` or `< r` digest limb) | `col1` (`< 2^128` or `< r`) | `col2` (`< 2^128` or `< r`) | `ROM` enforces `tbl_id in 1..=14`, `row_idx = 8 * blk_idx + off` (`blk_idx < 2^29`, `off in 0..=7`), and authenticates `(col0, col1, col2)` to `prog_d` (`DOM_LEAF8` / `DOM_PROG`); compiler admission guarantees scalar column ranges. |
| `6` | `CHIP_HASH` | `CORE` (`FRAMES`/`TASKS` push/pop, `DOM_CLAIM`), `VALUE DIGEST` (`SHK_VAL`) | `HASH` (`PoseidonCfg`, `poseidon.rs`) | `hash_key = (dom << 32) \| (sub << 16) \| absorb_len` (`40` bits: `dom in {9,10,11,12}`, `sub < 2^16`, `absorb_len in 1..=16`) | `in0` (`< r`) | `in1` (`< r`, or chain `prev_d`) | `out_d` (`< r` Poseidon rate-0 squeeze) | `HASH` enforces fixed/checked `(dom, sub, absorb_len)` mapped into `st[2] = absorb_len + (((dom << 16) \| sub) << 32)` (`dense.rs:60-62`, `poseidon.rs:101-110`) and executes all 68 Poseidon rounds unconditionally (`Exploit E3` fix). |

#### C. 4 Dynamic Bus Requests per Step at `+0` Node Rows (`Arch::Narrow`)

`Bus` exposes two `lookup_any` arguments (`bus0` on `a[0..5]`, `bus1` on `a[5..10]`, `bus.rs:50-61`). In the 7-row `Arch::Narrow` step layout (Section 2.2; compacted from the 13-row spike `rfcore.rs:169-221` where rows `6, 10` were passive copy-replicas of row `2`, `rfcore.rs:198-200`), `CORE` asserts `q_req = 1` on **two adjacent rows per step**:
- Step row `+5`: `bus0 = L0` (`a[0..5]`), `bus1 = L1` (`a[5..10]`).
- Step row `+6`: `bus0 = L2` (`a[0..5]`), `bus1 = L3` (`a[5..10]`).

Because Halo2 `lookup_any` evaluates one vector polynomial per registered lookup argument across all rows where `q_req = 1` (`MP:plonk/lookup.rs`), enabling `q_req = 1` on two rows per step reuses the exact same 2 `lookup_any` arguments, the same 10 advice columns `a[0..10]`, and the same `q_req` fixed column: **`4` dynamic bus requests per VM step at `+0` advice columns, `+0` fixed columns, `+0` PLONK lookups, and `+0` recursive `ChipNode` rows** (`CONSENSUS.md:26, 220`).

#### D. 2-Column Domain-Tagged Fixed Lookup (`TaggedTableConfig`)

`TaggedTableConfig` registers **one** Halo2 `cs.lookup` over **two** `TableColumn`s `(t_dom_key, t_val)`:
- **Domain encoding (`t_dom_key = (tag << 16) + key`):**
  - Row `0`: `(0, 0)` (`tag = 0, key = 0, val = 0`), absorbing every row where `q_rng == 0 && q_dec == 0`.
  - `tag = 0` (`RANGE`, rows `0..2^B`): `t_dom_key = limb` (`0 <= limb < 2^B`, `B = 12` default `4,096` rows or `B = 14` `16,384` rows; since `B <= 14 < 16`, `t_dom_key < 2^16` never collides with `tag = 1`), `t_val = 0`.
  - `tag = 1` (`DEC`, `N_dec <= 2,048` rows at indices `2^B .. 2^B + N_dec`): `t_dom_key = 65536 + packed_key` (`0 <= packed_key < 2^16`, so `65536 <= t_dom_key < 131072`), `t_val = ctrl_word` (`54`-bit packed microcode word, `< 2^54 < r`).
- **Mutual-exclusion & query expression (`q_rng * q_dec == 0`):**
  - Query on shared lookup cell `(x_lookup, v_lookup)` at `a[10], a[11]` (or per-region designated columns):
    - `expr_dom_key = q_rng * x_rng + q_dec * (Expression::Constant(F::from(65536)) + key_dec)`
    - `expr_val = q_dec * ctrl_dec` (when `q_rng == 1`, `q_dec == 0` so `expr_val == 0`, matching `t_val == 0` on all `tag = 0` rows).
  - Structural disjointness gate: `q_rng(row) * q_dec(row) == 0` enforced by region non-overlap (`RANGE` rows vs `CORE` decode row `+0`) and verified by a degree-2 fixed-column audit check at keygen.

#### E. Copy-Constrained Boundary Port Cells (`constrain_equal`)

Cross-module state handoffs outside per-step `Bus` queries use `Region::constrain_equal` (`MP:circuit/layouter.rs`) on `AssignedCell<F, F>` handles:
1. `ArchStatePort` (`CORE` slot `0` pre-state and slot `S - 1` post-state $\leftrightarrow$ `BOUNDARY COMMITMENT`): exact `NDIG` canonical state lanes (`prog_d`, `input_d`, `h`, `ctl`, `outcome`, `fuel_lo`, `fuel_hi`, `consumed_work`, `helper_work`, `helper_visits`, `reference_cursor`, `stack_limit`, `fcount`, `tcount`, `fp_hash`, `tp_hash`, `top_frame[0..4]`, `top_task[0..4]`, `env_ptr`, `alloc_len[0..4]`, `helper_packed[0..6]`, `result[0..3]`, `mem_root_or_acc`, `tail_cells[0..24]`). `CORE` owns internal step transitions; `BOUNDARY COMMITMENT` binds `pre_d` / `post_d` to public instance inputs.
2. `WindowBoundaryPort` (`CORE` $\leftrightarrow$ `MEM`): `len_pre`, `len_post`, and 8-record tail register array (`tail_pre[0..8]`, `tail_post[0..8]`).
3. `HashFixedPort` (`BOUNDARY COMMITMENT`, `MEM` flush, `ROM` Merkle $\leftrightarrow$ `HASH`): direct `AssignedCell<F, F>` chaining into dedicated `PoseidonCfg` 70-row blocks (`q_prov = 0` on bus, zero dynamic bus overhead; `poseidon.rs:433-460`).

### 1.3 Invariants

**Guaranteed by `BUS`:**
1. **Lookup Soundness (`LookupSound`, `bus.rs:50-61`):** Every row with `q_req == 1` has each of its two 5-tuples `(chip, key, a, b, c)` equal to `(0, 0, 0, 0, 0)` (matched by any `q_prov == 0` row) or equal to `(chip, key, a, b, c)` on at least one row with `q_prov == 1`.
2. **Static Domain Isolation (`TaggedTableConfig`):** Because `0 <= limb < 2^14 < 65536` for `tag = 0` and `65536 <= 65536 + packed_key < 131072` for `tag = 1`, no `q_rng = 1` query can ever match a `DEC` row and no `q_dec = 1` query (with `0 <= packed_key < 65536`) can ever match a `RANGE` row, including `(0, 0)`.
3. **Single-Phase Unit-Rotation Compliance:** `Bus` and `TaggedTableConfig` register zero `Phase(1)` columns, zero `Expression::Challenge` nodes, and exclusively `Rotation::cur()` (`0`) queries (`MC:verifier/verifier_gadget.rs:310, 604-610`).

**Mandatory Universal Provider Rules (Assumed from Every Provider Module & Enforced by Per-Module Tests, `design.md:94-102` `D1, D4, D8`, `bus.rs:12-20`):**
1. **Non-Zero Fixed Chip Constant (`D4`, `D8`):** Every active provider row (`q_prov == 1` and slot active `act == 1`) constrains `chip == CHIP_ID` for a compile-time constant `CHIP_ID in {2, 3, 4, 5, 6}` (`CHIP_ID != 0`). No active provider row may emit a witness-chosen `chip` or `chip == 0`.
2. **Inactive Provider Zero-Gating (`membackend.rs:258-264`):** Any provider region that sets fixed `q_prov = 1` across `N` potential slots with a boolean activity witness `act in {0, 1}` MUST gate all five columns to zero when `act == 0`: `chip = act * CHIP_ID`, `(1 - act) * key == 0`, `(1 - act) * a == 0`, `(1 - act) * b == 0`, `(1 - act) * c == 0`, collapsing unused slots to the canonical idle tuple `(0, 0, 0, 0, 0)`.
3. **Requester Unused-Lane Zero-Gating (`D8`):** Whenever a requester lane sets `chip == 0`, the requester MUST constrain `key == 0`, `a == 0`, `b == 0`, `c == 0` (otherwise `(0, k, a, b, c)` with `(k,a,b,c) != 0` fails lookup, or if any provider leaked `chip = 0`, would forge responses).
4. **Functional Determinism & Partition Disjointness (`D2`, `bus.rs:18-20`):** Within each `(chip, key)` (for `CHIP_MEM_ALLOC`, `CHIP_MEM_READ`, `CHIP_ROM`) or `(chip, key, a, b)` (for `CHIP_ALU`, `CHIP_HASH`), the remaining tuple fields are a deterministic partial function of the key/inputs. When two modules share one `chip` (`CHIP_MEM_READ = 4` shared by `MEM` and `MEMORY BACKEND`), their key predicates MUST be provably disjoint (`addr >= base` vs `addr < base`, `D2`).
5. **Single Declared Range-Check Owner:** Every tuple field has exactly one owner listed in Section 1.2.B; no provider may assume requester-side range bounds on inputs unless enforced by a strictly lower-level lookup (`ModularComposition.lean:63-66`).
6. **DAG Level Stratification (`ModularComposition.lean:17-19, 64`):** Modules obey strict tuple dependency level `level(TaggedTable[RANGE=0, DEC=1]) = 0 < level(CHIP_ALU=2, CHIP_MEM_ALLOC=3, CHIP_MEM_READ=4, CHIP_ROM=5, CHIP_HASH=6) = 1 < level(CORE_Goal) = 2` (with `VALUE DIGEST` at level `2` above `CHIP_MEM_READ`/`CHIP_HASH` if invoked by `CORE` at level `3`, or `CORE` invoking `CHIP_HASH` directly). No cycle exists on the bus.

### 1.4 Row Budget (`ESTIMATE`)

- **Leaf Circuit Rows (`S = 16` and `S = 64`):**
  - `Bus` itself consumes **`0` dedicated advice rows** (`q_req` and `q_prov` overlay `CORE` and provider regions).
  - `TaggedTableConfig` fixed table occupies `max(leaf_advice_rows, 2^B + N_dec)` rows:
    - With `B = 12` (`4,096` `RANGE` rows) and `N_dec <= 2,048` (`DEC` rows): `<= 6,144` table rows (`9.4%` of `k_leaf = 16` `65,530` usable rows; `4.7%` of `k_leaf = 17` `131,066` usable rows), adding **`0` extra `k_leaf` increments** at both `S = 16` and `S = 64` (`ESTIMATE`).
    - With `B = 14` (`16,384` `RANGE` rows) and `N_dec <= 2,048`: `<= 18,432` table rows (`28.1%` of `k_leaf = 16`), still **`0` extra `k_leaf` increments** (`ESTIMATE`).
- **Column & Lookup Footprint (`BUS` + `TaggedTableConfig` vs E8 Spike):**
  - Advice columns: `0` dedicated (uses shared `a[0..12]`, `lib.rs:44`).
  - Fixed selector columns: `2` (`q_req`, `q_prov` in `Bus`; `q_rng` and `q_dec` pack into `vsel` or `1` shared 3-code selector `0 = idle, 1 = rng, 2 = dec` of degree 2 -> `-1` fixed selector column vs separate `q_rng, q_dec`, `ESTIMATE`).
  - Fixed `TableColumn`s: `2` (`t_dom_key`, `t_val`), **`-1` fixed `TableColumn`** relative to E8 (`1` `RANGE` + `2` `DEC` = `3` `TableColumn`s, `rfcore.rs:839-843`).
  - PLONK lookups: `3` total (`bus0`, `bus1` via `lookup_any`, `tagged_table` via `lookup`), **`-1` PLONK lookup** relative to E8 (`4` lookups).
- **1-Leaf `ChipNode` and 2-Leaf `RunLift` `R` Impact (`ESTIMATE`):**
  - 1-leaf `ChipNode`: `-1` PLONK lookup (`-22,600` rows) + `-1` fixed `TableColumn` (`-6,462` rows) = **`ESTIMATE -29,062` Node rows** (and up to `ESTIMATE -35,066` Node rows if `q_rng`/`q_dec` share one 2-code `vsel` column).
  - 2-leaf `RunLift` `R`: **`ESTIMATE -51,204 .. -51,662` `R` rows** (`CONSENSUS.md:145`).

### 1.5 Verification

- **Quint Model (`specs/quint/sherlock_bus.qnt`, new ~180 LOC):**
  - Models `Tup = { chip: int, key: int, a: int, b: int, c: int }`, `TaggedEntry = { dom_key: int, val: int }`, provider sets per module (`CHIP_ALU`, `CHIP_MEM_ALLOC`, `CHIP_MEM_READ` split at `base`, `CHIP_ROM`, `CHIP_HASH`), and non-deterministic malicious witness injection on `act == 0` and `q_prov == 0` rows.
  - Checks invariants `InvNoZeroChipProvider`, `InvDisjointMemReadProviders`, `InvTaggedDomainNonAlias` (`limb < 65536` vs `65536 + packed_key`), and `InvDAGLevelOrdering`.
- **`MockProver` Positive Tests (`crates/mori-chip/src/bus.rs` & `tagged_table.rs`):**
  - `bus_four_requests_per_step_ok`: 2-row requester (`L0..L3`) mixing `CHIP_NOP`, `CHIP_ALU`, `CHIP_MEM_READ`, and `CHIP_HASH` passes `MockProver::run(13, ...)`.
  - `tagged_table_range_and_dec_coexist_ok`: simultaneous `q_rng = 1` (`limb = 4095`) and `q_dec = 1` (`packed_key = 0` and `packed_key = 2047`) rows pass on one 2-column table.
- **Named Negative Mutants (`SAS-003`, `SAS-004`):**
  1. `mut_bus_active_provider_zero_chip`: active provider sets `chip = 0` with non-zero payload `(0, 7, 99, 0, 0)` while requester queries `(0, 7, 99, 0, 0)` -> MUST fail `chip == CHIP_ID` provider gate (`D8`).
  2. `mut_bus_inactive_slot_leaks_payload`: provider row with `q_prov = 1, act = 0` leaves `(0, 0, 42, 0, 0)` unzeroed -> MUST fail `(1 - act) * a == 0` gate AND fail requester `(0,0,0,0,0)` lookup.
  3. `mut_bus_unassigned_chip_1`: requester emits `(1, 0, 0, 0, 0)` with `q_req = 1` -> MUST fail `lookup_any` (`Lookup` error at verifier stage, zero panics).
  4. `mut_tagged_dec_overflows_into_range`: malicious `CORE` witness sets `packed_key = p - 65536 + 17` (`expr_dom_key == 17` mod `r`, matching `RANGE` row `17` with `t_val = 0`) -> MUST fail because `packed_key` is reconstructed purely from boolean/range-bounded `CORE` pre-lookup fields (`0 <= packed_key < 65536`).
  5. `mut_tagged_simultaneous_q_rng_q_dec`: test harness deliberately overlaps `q_rng = 1` and `q_dec = 1` on one row -> MUST fail keygen selector-disjointness check (`q_rng * q_dec == 0`).

### 1.6 Lean Obligations

- **Theorem (`sherlock_leaf_modular_refinement`):** Instantiate `Clean.Sherlock.Modular.leaf_refines_step` (`clean/fork/Clean/Sherlock/ModularComposition.lean:131-137`) with concrete Sherlock tuple sum type `SherlockTup` (covering dynamic `Bus` `chip in {0, 2, 3, 4, 5, 6}` plus static `TaggedTable` `tag in {0, 1}`), concrete DAG level function `sherlockLevel : SherlockTup -> Nat` (`0` for `TaggedTable`, `1` for `ALU`/`MEM`/`MEMORY_BACKEND`/`ROM`/`HASH`, `2` for `VALUE_DIGEST`), and module list `[mTagged, mAlu, mMemWin, mMemBack, mRom, mHash, mVal, mCore]`. Prove that if every module satisfies `Contract SherlockValid sherlockLevel m` (`ModularComposition.lean:63-66`) and `LookupSound` holds on the leaf witness, then any accepted leaf witness establishes `Aggregation.Run (fun x y => y = sherlockStep1 x) (rows w).length (pre w) ((rows w).getLastD (pre w))`, discharging `Premises.leaf_sound` and `Premises.det` (`fun_det`, `ModularComposition.lean:123-125`; `clean/fork/Clean/Sherlock/Aggregation.lean:81-96`).
- **Lean File Extended / Created:** Creates `clean/fork/Clean/Sherlock/SherlockBusContract.lean` (plus `SherlockBusContractAxioms.lean` with `#print axioms` checks matching `ModularCompositionAxioms.lean:9-19`), importing `Clean.Sherlock.ModularComposition`.
- **Proof-System Premises / Axioms Relied On:** Zero custom Lean `axiom` declarations (`ModularCompositionAxioms.lean:3-7`). PLONK `lookup_any` / `lookup` soundness (establishing `LookupSound` up to Schwartz-Zippel soundness error `<= deg * 2^{k_leaf} / r < 2^{-235}`) is carried as the explicit theorem hypothesis `hl : LeafRel sherlockMods w`, matching `AggregationAxioms.lean` discipline.
- **Module Contract Discharged:** Discharges **Lean Obligation 3 (Modular Composition)** (`ARCH-BRIEF.md:57`), proving that each module kit can verify its `Contract` in complete isolation.

### 1.7 Code Skeleton

Owned files: `crates/mori-chip/src/bus.rs` (updated registry) and `crates/mori-chip/src/tagged_table.rs` (new).

```rust
// crates/mori-chip/src/bus.rs
use midnight_proofs::{
    circuit::{AssignedCell, Region},
    plonk::{Advice, Column, ConstraintSystem, Error, Selector},
};
use crate::F;

pub const BUS_WIDTH: usize = 5;
pub const CHIP_NOP: u64 = 0;
// ID 1 is intentionally unassigned (retired legacy spike ID).
pub const CHIP_ALU: u64 = 2;
pub const CHIP_MEM_ALLOC: u64 = 3;
pub const CHIP_MEM_READ: u64 = 4;
pub const CHIP_ROM: u64 = 5;
pub const CHIP_HASH: u64 = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BusTuple {
    pub chip: F,
    pub key: F,
    pub a: F,
    pub b: F,
    pub c: F,
}

#[derive(Clone, Debug)]
pub struct Bus {
    pub q_req: Selector,
    pub q_prov: Selector,
    pub cols: [Column<Advice>; 10],
}

impl Bus {
    /// Requires: `cs.phases().count() == 1`; `cols` are Phase(0) advice columns.
    /// Ensures: Registers `bus0` (`cols[0..5]`) and `bus1` (`cols[5..10]`) `lookup_any` against `cols[0..5]` with `Rotation::cur()` only.
    pub fn configure(cs: &mut ConstraintSystem<F>, cols: [Column<Advice>; 10]) -> Self;

    /// Requires: `row` is not enabled for `q_prov`; if `lanes[i].chip == 0`, all fields of `lanes[i]` are `0`.
    /// Ensures: Enables `q_req` at `row` and assigns `lanes[0]` to `cols[0..5]` and `lanes[1]` to `cols[5..10]`.
    pub fn assign_req_pair(
        &self,
        region: &mut Region<'_, F>,
        row: usize,
        lanes: [BusTuple; 2],
    ) -> Result<[[AssignedCell<F, F>; BUS_WIDTH]; 2], Error>;

    /// Requires: `row` is not enabled for `q_req`; if active, `tup.chip` equals a non-zero registered `CHIP_*` constant; if inactive, `tup == (0,0,0,0,0)`.
    /// Ensures: Enables `q_prov` at `row` and assigns `tup` to `cols[0..5]`.
    pub fn assign_prov(
        &self,
        region: &mut Region<'_, F>,
        row: usize,
        tup: BusTuple,
    ) -> Result<[AssignedCell<F, F>; BUS_WIDTH], Error>;
}

// crates/mori-chip/src/tagged_table.rs
use midnight_proofs::{
    circuit::Layouter,
    plonk::{Advice, Column, ConstraintSystem, Error, Fixed, TableColumn},
};
use crate::F;

pub const TAG_RANGE: u64 = 0;
pub const TAG_DEC: u64 = 1;
pub const TAG_DEC_OFFSET: u64 = 1 << 16; // 65,536

#[derive(Clone, Debug)]
pub struct TaggedTableConfig {
    /// Code column: 0 = idle, 1 = RANGE (`tag = 0`), 2 = DEC (`tag = 1`).
    pub q_code: Column<Fixed>,
    pub key_col: Column<Advice>,
    pub val_col: Column<Advice>,
    pub t_dom_key: TableColumn,
    pub t_val: TableColumn,
    pub range_bits: usize,
}

impl TaggedTableConfig {
    /// Requires: `range_bits` in `{12, 14}`; `key_col` and `val_col` are Phase(0) advice columns.
    /// Ensures: Registers a single degree-3 PLONK `lookup` over `(t_dom_key, t_val)` using Lagrange codes `{0, 1, 2}` on `q_code` and `Rotation::cur()` only.
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        q_code: Column<Fixed>,
        key_col: Column<Advice>,
        val_col: Column<Advice>,
        range_bits: usize,
    ) -> Self;

    /// Requires: `dec_rows.len() <= 2048`; every `(packed_key, ctrl_word)` has `packed_key < 65536` and unique `packed_key`.
    /// Ensures: Loads `[0..2^range_bits)` with `(limb, 0)` and `dec_rows` with `(65536 + packed_key, ctrl_word)`.
    pub fn load(
        &self,
        layouter: &mut impl Layouter<F>,
        dec_rows: &[(u16, u64)],
    ) -> Result<(), Error>;
}
```

### 1.8 Agent Context Boundary

- **MUST Read (`<= 4` files):**
  1. `crates/mori-chip/src/lib.rs:1-68` (`F`, `NCOL`, shared constants).
  2. `crates/mori-chip/src/bus.rs:1-254` (existing `Bus` spike to update).
  3. `crates/mori-chip/src/vsel.rs:1-120` (Lagrange code degree helper for `q_code`).
  4. `MC:verifier/verifier_gadget.rs:308-345, 604-610` (single-phase and unit-rotation verifier rules).
- **MUST NOT Need to Read:**
  - `crates/mori-core/src/sherlock/...` (`owner_runtime.rs`, AST/rule semantics).
  - `crates/mori-chip/src/{rfcore.rs, alu.rs, poseidon.rs, membackend.rs}` (provider internal gate equations).
  - `crates/mori-recursion/` or `crates/mori-backend/` (node/decider orchestration).

### 1.9 OpenSpec 1.17.0 Capability Spec (`specs/sherlock-bus/spec.md`)

- **`BUS-001` (Single-Phase Dynamic Bus):** The `BUS` module SHALL register two 5-column `lookup_any` channels over Phase(0) advice columns using `Rotation::cur()` only and zero verifier challenges, serving frozen chip IDs `{0, 2, 3, 4, 5, 6}` with ID `1` unassigned. *(246 chars)*
  - **Scenario `S-BUS-001a` (`MockProver` + `mut_bus_unassigned_chip_1`):** **WHEN** a step emits four valid requests (`L0..L3`) on two `q_req = 1` rows **THEN** `MockProver` accepts; **WHEN** any request sets `chip = 1` **THEN** `MockProver` rejects at the lookup check with zero panics.
- **`BUS-002` (Provider Zero-Gate and Non-Zero Chip Law):** Every provider region SHALL constrain active rows to its fixed non-zero `CHIP_ID` and SHALL gate all five tuple columns of inactive `q_prov = 1` slots to `(0, 0, 0, 0, 0)`; every requester SHALL gate `(key, a, b, c)` to `0` when `chip == 0`. *(246 chars)*
  - **Scenario `S-BUS-002a` (`mut_bus_active_provider_zero_chip`, `mut_bus_inactive_slot_leaks_payload`, Quint `InvNoZeroChipProvider`):** **WHEN** a malicious witness assigns `chip = 0` on an active provider row or non-zero payload on an inactive `q_prov = 1` row **THEN** the local provider gate rejects the witness.
- **`BUS-003` (Merged 2-Column Tagged Static Lookup):** `TaggedTableConfig` SHALL merge `RANGE` (`tag = 0`, `t_dom_key = limb < 2^14`, `t_val = 0`) and `DEC` (`tag = 1`, `t_dom_key = 65536 + packed_key`, `t_val = ctrl_word`) into one 2-column PLONK lookup with disjoint domain keys. *(233 chars)*
  - **Scenario `S-BUS-003a` (`mut_tagged_dec_overflows_into_range`, Lean `sherlock_leaf_modular_refinement`):** **WHEN** `CORE` queries `DEC` with bounded `packed_key < 65536` **THEN** `expr_dom_key >= 65536` cannot match any `RANGE` row, and `Clean/Sherlock/SherlockBusContract.lean` discharges `LeafRel -> Run`.

---

## 2. Module `CORE` — Register File, 7-Row `Arch::Narrow` Step Slots, `vsel` Packing, and Top-of-Stack Chains

### 2.1 Purpose

`CORE` owns the canonical architectural state of the Sherlock `/1` virtual machine across `S` consecutive step slots per leaf, executing decoded micro-operations over registers, top-of-stack `Frame` and `Task` caches, and four dynamic bus request lanes per step. It enforces monotone halt padding (`is_pad`), 64-bit step height `h < 2^64` (`SRC-004`), strict write-before-read zeroing on intra-step scratch lanes `v0..v2` (`SRC-023`, `AUDIT.md:67-69`), and copy-constrained boundary handoff.

### 2.2 Frozen Interface

#### A. Canonical Architectural State (`ArchState`, Copy-Constrained Port)

`CORE` exposes `pre_state: ArchStateCells` at slot `0` and `post_state: ArchStateCells` at slot `S - 1` via `constrain_equal` to `BOUNDARY COMMITMENT` and `MEM`. Every lane in `ArchState` is hashed into `pre_d` / `post_d` (`DOM_STATE = 8`), leaving **zero unhashed inter-step witness lanes**:

| Register Group | Lanes | Bit-Width / Domain | Range-Check & Representation Owner |
|---|---|---|---|
| Run Identity | `prog_d`, `input_d` (`2` field cells) | `< r` (digest) | Preserved `post == pre` across every slot in `CORE` (`GP-001`, `SRC-001`). |
| Step Height | `h` (`1` cell) | `0 <= h < 2^64` | `CORE` increments `h' = h + enable` (`enable in {0,1}`); `NODE` / `CORE` boundary checks `h_post < 2^64` via `RANGE` (`5` x `12`-bit + `4`-bit limb or `ALU` `< 2^64`, `SRC-004`). |
| Control & Outcome | `ctl_tag`, `ctl_aux`, `outcome_tag`, `outcome_code` (`4` cells) | `ctl_tag in 0..=7`, `outcome_tag in 0..=3`, `aux/code < 2^32` | `CORE` + `DEC` transition gates; `outcome_tag`: `0 = Running`, `1 = Halted`, `2 = Rejected`, `3 = Exhausted` (`GP-001`). |
| Fuel & Accounting | `fuel_lo`, `fuel_hi`, `consumed_work`, `helper_work`, `helper_visits`, `ref_cursor`, `stack_limit` (`7` cells) | `u64` limbs (`< 2^64`) | 128-bit `remaining_fuel = fuel_lo + 2^64 * fuel_hi` decremented via `ALU` (`OP_SUB`) on fuel-consuming steps; counters checked `< 2^64`. |
| LIFO Stack Anchors | `fcount`, `tcount`, `fp_hash`, `tp_hash` (`4` cells) | `fcount, tcount < 2^64`; `fp_hash, tp_hash < r` | `CORE` enforces `fcount == 0 => fp_hash == EMPTY_STACK_FRAME != 0` and `tcount == 0 => tp_hash == EMPTY_STACK_TASK != 0` (`AUDIT.md:63`); `fcount <= stack_limit`. |
| Top-of-Stack Cache | `top_frame[0..4]`, `top_task[0..4]` (`8` cells) | `kind/pc < 2^32`, payload `< 2^128` or `< r` | Held in registers (`owner_runtime.rs:3566-3591`): `RewriteTop` and top reads cost **`0` bus tuples and `0` hashes**; only deep push/pop (`fcount >= 1` push or `fcount >= 2` pop) invokes `CHIP_HASH = 6` (`DOM_FRAME = 9`, `DOM_TASK = 10`). |
| Heap & Env Cursors | `env_ptr`, `len_heap`, `len_env`, `len_list`, `len_ctor` (`5` cells; or unified `len_mem` + typed counters) | `< 2^64` | Advance **strictly by `k_alloc in 0..=4`** on allocating rules (`0` advance on non-allocating steps, `GP-001`, `AUDIT.md:45`). |
| Packed Helper & Result | `helper_packed[0..6]`, `result[0..3]` (`10` cells) | Packed tags/cursors `< 2^128`; `result = (v_tag, v_lo, v_hi_or_ptr)` | `23` logical helper scalars (`rf.rs:31-35`) packed into `6` canonical `u128` register cells via non-overlapping 32/64-bit sub-lanes; unpacked only by active helper rules and range-checked via `RANGE`/`ALU`. |

*Scratch Lanes `v0, v1, v2` (`rf.rs:34, 65-67`):* Explicitly **excluded** from `ArchState` and `pre_d`/`post_d`. Never copied across step boundaries (see Invariant 4 below).

#### B. Compacted 7-Row `Arch::Narrow` Step Slot Layout (`NCOL = 12` Shared Advice Columns)

Compacting the 13-row E8 spike (`rfcore.rs:169-221`, which interleaved four `F` rows and three duplicate `L` rows) into a 7-row `Arch::Narrow` slot over `a[0..12]` preserves `Rotation(-1, 0, +1)` adjacency while cutting `CORE` step rows by `46%`:

| Row Offset | Role | `a[0..10]` Assignment | `a[10..12]` Assignment | Active Lookups / Gates |
|---|---|---|---|---|
| `+0` | `PreState A` + `DEC` Query | `pre` architectural bank A (`10` cells; `constrain_equal` to previous slot row `+3` / boundary) | `a[10] = packed_key`, `a[11] = ctrl_word` | `TaggedTableConfig` (`q_code = 2`, `tag = 1` `DEC`) |
| `+1` | `PreState B` + Unpacked Ctrl A | `pre` architectural bank B (`10` cells) | `IsZero` inverses (`fuel_zero_inv`, `fc_zero_inv`) | Adjacency gates (`Rotation(-1, 0, +1)` against `+0` and `+2`) |
| `+2` | `PreState C` + Unpacked Ctrl B | `pre` architectural bank C (`10` cells, incl. `top_frame`, `top_task`) | Unpacked boolean control sub-fields + `v0` | `vsel` micro-op group A + boolean/domain checks on `ctrl_word` |
| `+3` | `PostState A` | `post` architectural bank A (`10` cells) | `v1, v2` intra-step scratch cells | `vsel` transition gates (`Rotation(-1)`/+`0`/ `+1`) |
| `+4` | `PostState B & C` Delta | `post` architectural bank B/C changed lanes (`constrain_equal` to next slot `+1/+2` or delta mux) | `IsZero` inverses (`tc_zero_inv`, `eq_diff_inv`) | Monotone `is_pad` & `EMPTY_STACK` gates |
| `+5` | `Bus Req Pair 0` (`L0, L1`) | `L0 = (chip0, key0, a0, b0, c0)` in `a[0..5]`; `L1 = (chip1, key1, a1, b1, c1)` in `a[5..10]` | `enable`, `is_pad` (`1 - enable`) | `Bus::q_req = 1` (`bus0`, `bus1`) |
| `+6` | `Bus Req Pair 1` (`L2, L3`) | `L2 = (chip2, key2, a2, b2, c2)` in `a[0..5]`; `L3 = (chip3, key3, a3, b3, c3)` in `a[5..10]` | Step-end link / `h_next` | `Bus::q_req = 1` (`bus0`, `bus1`) |

*(Note: Because `ArchState` has `41` cells and `3` pre-rows hold `30` cells, the remaining `11` cold/preserved cells or `PostState C` either occupy a tiny 8th row `h_slot = 8` — still `512` rows at `S = 64` — or pack bounded `< 2^32` control/counter scalars `(ctl_tag, ctl_aux, outcome_tag, outcome_code)` and `(fcount, tcount)` into single `< 2^128` carrier words decomposed on active transitions, fitting `30` carrier cells cleanly in `7` rows. Both `7`-row and `8`-row slots satisfy `k_leaf = 16` with `> 60,000` rows to spare; Section 2.4 budgets both.)*

#### C. `vsel` Lagrange Selector Packing (`crates/mori-chip/src/vsel.rs:1-173`)

Virtual selectors `Vsel` (`vsel.rs:36-41`) pack mutually exclusive rule micro-operations onto shared fixed code columns `Column<Fixed>` using univariate Lagrange basis polynomials `L_c(q) = prod_{m != c} (q - m) / (c - m)` (`vsel.rs:29-34`):
- A fixed column with `n` active codes (`1..=n`, `0` = off) contributes polynomial degree `n` (`vsel.rs:97`), keeping total gate degree `n + d_body <= MAX_DEG = 6` (`vsel.rs:25, 97`; e.g., degree-2 body -> up to `n = 4` mutually exclusive micro-ops per fixed column; degree-3 body -> `n = 3`).
- Greedy largest-degree-first bin packing (`VselPacker::pack`, `vsel.rs:90-119`) eliminates ~14 sparse fixed selector columns (`ESTIMATE -84,000 .. -90,440` Node rows vs 1-hot fixed selectors, `vsel.rs:4`).

#### D. Bus Tuples Requested and Provided by `CORE`

- **Provided on `Bus`:** None (`CORE` never sets `q_prov = 1`).
- **Requested on `Bus` (up to `4` tuples `L0..L3` per step on rows `+5, +6`):**
  - `CHIP_NOP = 0`: unused lanes and all 4 lanes when `is_pad == 1`.
  - `CHIP_ALU = 2`: fuel decrement (`OP_SUB`), scalar arithmetic (`BHelperScalarArithmetic`), unsigned/signed comparisons, pointer bounds (`addr < len_curr`), stack limit (`fcount < stack_limit`).
  - `CHIP_MEM_ALLOC = 3`: up to 2–3 cell appends per step (`EnvPutInsert`, `AppendCell`, `XAdtHelperBuildField`, `ScalarConstruct`).
  - `CHIP_MEM_READ = 4`: environment lookup (`EnvGetCell`), list/constructor field traversal (`BindPair`, `EqualField`, `MatchField`, `AppendCopy`).
  - `CHIP_ROM = 5`: AST node fetch, literal/const fetch, function table lookup (`FindRow`), ADT plan walk (`XAdtHelperPlanWalk`).
  - `CHIP_HASH = 6`: deep `Frame` push/pop (`DOM_FRAME = 9`) and deep `Task` push/pop (`DOM_TASK = 10`) when `fcount >= 1` (push) / `fcount >= 2` (pop), plus `GP-2` claim digest check (`DOM_CLAIM = 12`).
- **Requested on `TaggedTableConfig`:** `1` `DEC` lookup (`tag = 1`, row `+0`) per step when `enable == 1` (`q_code = 2 * enable`, so `is_pad == 1` sets `q_code = 0`, querying `(0, 0)`).

### 2.3 Invariants

**Guaranteed by `CORE`:**
1. **Non-Empty Leaf & Monotone Halt Padding (`SRC-001`, `SRC-014`, `design.md:44`):**
   - Slot `0` constrains `is_pad_0 == 0` (`enable_0 == 1`, `steps = sum_{j=0}^{S-1} enable_j >= 1`).
   - Every slot `j` constrains `is_pad_j * (1 - is_pad_j) == 0`; adjacent slots `j, j+1` constrain `(1 - is_pad_{j+1}) * is_pad_j == 0` (`is_pad` is 0-then-1 monotone).
   - Whenever slot `j` has `outcome_tag_post != 0` (`Halted`, `Rejected`, or `Exhausted`), if `j + 1 < S` then `is_pad_{j+1} == 1`; if `enable_j == 1`, `outcome_tag_pre == 0` (no active step executes from an already-terminal state).
   - When `is_pad_j == 1`, `post_state == pre_state` across every lane of `ArchState`, `h_post == h_pre`, `L0 == L1 == L2 == L3 == (0, 0, 0, 0, 0)`, and `q_code_dec == 0`.
2. **64-Bit Step Height Progression (`SRC-004`):** `h_{j+1} = h_j + (1 - is_pad_j)` and `h_S < 2^64` (checked once per leaf on `h_S` and `h_0` via `ALU`/`RANGE`, since `0 <= S < 2^64` prevents intra-leaf wrap over `Fq`).
3. **Top-of-Stack Cache & Non-Zero Empty Stack Sentinels (`AUDIT.md:63`, `owner_runtime.rs:3566-3591`):**
   - `fcount == 0 => fp_hash == EMPTY_STACK_FRAME` (`EMPTY_STACK_FRAME != 0`) and `top_frame == (0, 0, 0, 0)` (enforced via `fc_is_zero * (fp_hash - EMPTY_STACK_FRAME) == 0`).
   - `tcount == 0 => tp_hash == EMPTY_STACK_TASK` (`EMPTY_STACK_TASK != 0`) and `top_task == (0, 0, 0, 0)` (enforced via `tc_is_zero * (tp_hash - EMPTY_STACK_TASK) == 0`).
   - Push when `fcount == 0` writes `top_frame'` directly (`fcount' = 1`, `fp_hash' = EMPTY_STACK_FRAME`, `0` hashes); push when `fcount >= 1` spills `top_frame` via `CHIP_HASH = 6` (`fp_hash' = H(DOM_FRAME, fp_hash, pack(top_frame))`) and loads `top_frame'`; pop when `fcount >= 2` verifies `fp_hash == H(DOM_FRAME, fp_hash', pack(restored_frame))` via `CHIP_HASH = 6`; pop when `fcount == 1` sets `fcount' = 0, fp_hash' = EMPTY_STACK_FRAME, top_frame' = 0` (`0` hashes). Symmetric rules govern `TASKS`.
4. **Per-Slot Scratch Lane Isolation (`v0..v2` Write-Before-Read / Zeroing, `AUDIT.md:67-69` Finding 9, `SRC-023`):**
   - Scratch cells `v0, v1, v2` exist solely at row offsets `+2, +3` inside slot `j` with **zero equality constraints and zero rotation gates linking `v0..v2` of slot `j` to slot `j - 1` or `j + 1`**.
   - Every rule micro-op reading `v_k` MUST write `v_k` within the same slot `j` from `pre_state` or bus outputs `L0..L3`; when `v_k` is unused by the active rule (or `is_pad_j == 1`), `CORE` constrains `v_k == 0`. Thus `post_state` after `N` steps is a pure deterministic function of `pre_state_0`, preserving `SRC-023` across every leaf partition (`S = 16`, `S = 64`, or mixed).
5. **Parameterized `LeafShape` (`GP-001`):** `CoreConfig` takes `LeafShape { s_max: usize, .. }` (`s_max = 16` for M4 `SRC-014`; `s_max = 64` post-M4) with identical gates, columns, and lookup structure; changing `s_max` changes only region loop count and `k_leaf` (`16 -> 17`).

**Assumed from Other Modules:**
- `BUS` + `TaggedTableConfig`: lookup soundness (`LookupSound`).
- `DEC`: `ctrl_word` returned for `(tag = 1, packed_key)` encodes the exact `/1` rule transition for `packed_key`.
- `ALU`, `MEM` / `MEMORY BACKEND`, `ROM`, `HASH`: valid responses on `chip in {2, 3, 4, 5, 6}` per Section 1.2.B.
- `BOUNDARY COMMITMENT`: binds `slot[0].pre_state` and `slot[S-1].post_state` to public leaf statement digests `(pre_d, post_d)`.

### 2.4 Row Budget (`ESTIMATE`)

- **Leaf Circuit Rows:**
  - At `7` rows/slot (or `8` rows/slot if unpacked cold carriers use a 4th bank row):
    - `S = 16` (M4 default): `16 * 7 = 112` rows (`128` rows at `8` rows/slot) + `2` boundary staging rows = **`ESTIMATE 114 .. 130` advice rows** (`0.2%` of `k_leaf = 16` `65,530` usable rows).
    - `S = 64` (post-M4 amortized leaf): `64 * 7 = 448` rows (`512` rows at `8` rows/slot) + `2` boundary staging rows = **`ESTIMATE 450 .. 514` advice rows** (`0.4%` of `k_leaf = 17` `131,066` usable rows).
- **Column & Lookup Delta:**
  - Advice columns: `12` shared advice columns (`a[0..12]`, `lib.rs:44`) — `+0` advice columns over baseline.
  - Fixed columns: `ESTIMATE 4 .. 5` packed `vsel` code columns (`vsel.rs:90-119`, degree `<= 6`), `-3 .. -4` fixed columns relative to unpacked spike (`rfcore.rs`).
  - Equality-enabled columns: `12` advice + `2` boundary/instance = `14` (`+0` delta vs `CONSENSUS.md:129`).
  - Lookups: `0` new (`uses Bus + TaggedTableConfig`).
- **1-Leaf `ChipNode` and 2-Leaf `RunLift` `R` Impact (`ESTIMATE`):**
  - Moving `S = 16 -> S = 64` (`k_leaf = 16 -> 17`) adds **`ESTIMATE +4` rows** to 1-leaf `ChipNode` (`design.md:135`), **`ESTIMATE +8` rows** to 2-leaf `RunLift` `R`, and **`+0` rows** to the Decider (`design.md:135`), while cutting leaf count and recursion folds by **`4x`** on long runs.

### 2.5 Verification

- **Quint Model (`specs/quint/sherlock_core_slots.qnt`, new ~260 LOC):**
  - Models `S`-slot execution with top-of-stack `Frame`/`Task` caches, `fp_hash`/`tp_hash` chains with `EMPTY_STACK != 0`, `is_pad` monotonicity, `h < 2^64`, and non-deterministic scratch initialization if `v0..v2` were persistent vs per-slot fresh.
  - Proves `InvPartitionIndependence` (`run(steps, S=3) == run(steps, S=5)` for arbitrary multi-leaf runs, discharging finite-model check of `SRC-023`), `InvMonotonePad`, `InvEmptyStackSentinel`, and `InvZeroAllocNoAddrAdvance` (`GP-001`).
- **`MockProver` Positive Tests (`crates/mori-chip/src/rfcore.rs`):**
  - `core_leaf_all_rule_classes_s16_ok`: executes 16-step trace covering `Eval*`, `Return*`, `BHelper*`, `XAdt*`, deep `Frame`/`Task` spill + restore, and terminal `Halted` + 4 `is_pad = 1` rows.
  - `core_partition_s16_vs_s4_equal_post_state`: runs a 20-step allocating + constructor-building trace across `2 x (S=16)` vs `5 x (S=4)` leaves and asserts identical `post_state` and `n_steps = 20` (`GP-001`, `SRC-023`).
  - `core_height_max_u64_boundary_ok`: accepts `h_pre = 2^64 - 2, steps = 1 -> h_post = 2^64 - 1` (`GP-001`, `SRC-004`).
- **Named Negative Mutants (`SAS-003`, `SAS-004`):**
  1. `mut_core_pad_revival`: sets `is_pad = [0, 1, 0, 1, ...]` -> MUST fail `(1 - is_pad_{j+1}) * is_pad_j == 0`.
  2. `mut_core_all_padded_zero_step_leaf`: sets `is_pad_0 = 1` (`steps = 0`) -> MUST fail `is_pad_0 == 0` (`SRC-001`, `SRC-014`).
  3. `mut_core_step_after_terminal`: sets `outcome_tag_0 = 1` (`Halted`) and `is_pad_1 = 0` -> MUST fail post-terminal pad gate.
  4. `mut_core_height_wrap_2pow64`: sets `h_pre = 2^64 - 1, enable_0 = 1 -> h_post = 2^64` -> MUST fail `h_post < 2^64` range check (`SRC-004`, `GP-001`).
  5. `mut_core_scratch_cross_step_leak`: weakened control mutant passes `v0` across slot boundary vs production gate -> production rejects forged `v0` when unread/unwritten, confirming `AUDIT.md:67-69` closure.
  6. `mut_core_empty_stack_zero_digest`: sets `fcount = 0, fp_hash = 0` -> MUST fail `fc_is_zero * (fp_hash - EMPTY_STACK_FRAME) == 0` (`AUDIT.md:63`).
  7. `mut_core_pop_forged_restored_frame`: pops at `fcount = 2` with altered `restored_frame` -> MUST fail `CHIP_HASH = 6` lookup against `fp_hash`.

### 2.6 Lean Obligations

1. **Step Refinement (`core_spec_follows_step1`):** Prove the `hcoreSpec` hypothesis of `ModularComposition.leaf_refines_step` (`clean/fork/Clean/Sherlock/ModularComposition.lean:134`): whenever `mCore.rel w` holds and every tuple in `mCore.requests w` satisfies `SherlockValid`, the sequence of non-padded slot states `rows w` satisfies `Follows sherlockStep1 (pre w) (rows w)`, `rows.length >= 1` (`is_pad_0 == 0`), and padded rows (`is_pad == 1`) satisfy `post == pre` without advancing `h` or `rows.length`.
   - **Lean File:** Creates `clean/fork/Clean/Sherlock/CoreRefinement.lean` + `CoreRefinementAxioms.lean`.
   - **Premises / Axioms:** Collision resistance of Poseidon (`DOM_FRAME = 9`, `DOM_TASK = 10`) is stated as an explicit structure hypothesis `HashInjectiveOnReachableStacks` (matching `Premises` style in `Aggregation.lean:64-105`, zero Lean `axiom` keywords) so LIFO hash-chain pop uniquely recovers the pushed `Frame`/`Task` and `fp_hash == EMPTY_STACK_FRAME` holds only on the empty stack (`AUDIT.md:63`).
2. **Partition Independence (`core_partition_independent`, `SRC-023`, Lean Obligation 2 in `ARCH-BRIEF.md:56`):** Extend `clean/fork/Clean/Sherlock/PartitionIndependence.lean` (`PartitionIndependenceAxioms.lean`) to include top-of-stack `Frame`/`Task` caches and per-slot fresh `v0..v2` scratch variables, proving that for any two leaf partitions `(S_1, k_1)` and `(S_2, k_2)` of an `N`-step run (`sum steps_i = N`), `Aggregation.Run` determinism (`Modular.fun_det`, `ModularComposition.lean:123-125`; `Premises.det`, `Aggregation.lean:81`) yields identical terminal `ArchState`, `outcome_d`, and `n_steps`.

### 2.7 Code Skeleton

Owned files: `crates/mori-chip/src/rfcore.rs`, `crates/mori-chip/src/rf.rs`, `crates/mori-chip/src/vsel.rs`.

```rust
// crates/mori-chip/src/rfcore.rs
use midnight_proofs::{
    circuit::{AssignedCell, Layouter},
    plonk::{Advice, Column, ConstraintSystem, Error},
};
use crate::{bus::Bus, tagged_table::TaggedTableConfig, vsel::VselPacker, F};

pub const ARCH_STATE_CELLS: usize = 41;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeafShape {
    pub s_max: usize,       // 16 (M4) or 64 (post-M4)
    pub w_flush: usize,     // 8-cell flush blocks per leaf
    pub k_page: usize,      // Memory backend query budget
    pub k_rom: usize,       // Static ROM query budget
}

#[derive(Clone, Debug)]
pub struct ArchStateCells {
    pub cells: [AssignedCell<F, F>; ARCH_STATE_CELLS],
}

#[derive(Clone, Debug)]
pub struct LeafBoundary {
    pub pre: ArchStateCells,
    pub post: ArchStateCells,
    pub steps_taken: AssignedCell<F, F>,
}

#[derive(Clone, Debug)]
pub struct CoreConfig {
    pub shape: LeafShape,
    pub a: [Column<Advice>; 12],
    pub bus: Bus,
    pub tagged: TaggedTableConfig,
    pub vsel: VselPacker,
}

impl CoreConfig {
    /// Requires: `cs.phases().count() == 1`; `shape.s_max >= 1`; `bus` and `tagged` share `a[0..12]`.
    /// Ensures: Configures 7-row (or 8-row) `Arch::Narrow` step gates with degree <= 6, rotations in `{-1, 0, +1}`, monotone `is_pad`, `EMPTY_STACK != 0`, and scratch `v0..v2` isolation.
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        shape: LeafShape,
        a: [Column<Advice>; 12],
        bus: Bus,
        tagged: TaggedTableConfig,
    ) -> Self;

    /// Requires: `steps` contains `1..=shape.s_max` valid `/1` step witnesses; `steps[0]` is non-terminal on entry.
    /// Ensures: Assigns `shape.s_max` slots (padding remaining slots with `is_pad = 1`), emits 4 bus requests/slot and 1 `DEC` lookup/slot, checks `h_post < 2^64`, and returns boundary cells for `constrain_equal`.
    pub fn assign_leaf(
        &self,
        layouter: &mut impl Layouter<F>,
        steps: &[StepWitness],
    ) -> Result<LeafBoundary, Error>;
}
```

### 2.8 Agent Context Boundary

- **MUST Read (`<= 6` files):**
  1. `crates/mori-chip/src/bus.rs` & `crates/mori-chip/src/tagged_table.rs` (frozen tuple and `DEC` query interfaces).
  2. `crates/mori-chip/src/dec.rs` (unpacked `CtrlSignals` layout).
  3. `crates/mori-chip/src/rf.rs:1-302` & `crates/mori-chip/src/vsel.rs:1-173` (register indices and Lagrange packer).
  4. `crates/mori-chip/src/rfcore.rs:1-450` (existing `Arch::Narrow` slot prototype).
  5. `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:65-110, 3550-3600` (`/1` enabled rules and stack/helper transition semantics).
- **MUST NOT Need to Read:**
  - `crates/mori-chip/src/{alu.rs, range.rs, poseidon.rs, hashchip.rs, membackend.rs}` (internal gates of provider modules).
  - `crates/mori-recursion/` or Compactverifier contracts.

### 2.9 OpenSpec 1.17.0 Capability Spec (`specs/sherlock-core/spec.md`)

- **`CORE-001` (Monotone Halt Padding and 64-Bit Height):** `CORE` SHALL execute `1..=S` active `/1` steps per leaf (`is_pad_0 == 0`), enforce monotone `is_pad_{j+1} >= is_pad_j` and `is_pad == 1` after any terminal outcome, freeze all `ArchState` cells on padded slots, and constrain `h_post = h_pre + steps < 2^64`. *(258 chars)*
  - **Scenario `S-CORE-001a` (`mut_core_pad_revival`, `mut_core_all_padded_zero_step_leaf`, `mut_core_height_wrap_2pow64`):** **WHEN** `is_pad_0 = 1`, `is_pad` flips `1 -> 0`, or `h_post = 2^64` **THEN** `MockProver` rejects the witness; **WHEN** `h_post = 2^64 - 1` **THEN** `MockProver` accepts (`GP-001`, `SRC-004`).
- **`CORE-002` (Scratch Isolation and Partition Independence):** `CORE` SHALL exclude intra-step scratch cells `v0..v2` from cross-slot persistence and require per-slot write-before-read or zeroing so multi-leaf `ArchState` after `N` steps is invariant under leaf partitioning (`SRC-023`). *(225 chars)*
  - **Scenario `S-CORE-002a` (`core_partition_s16_vs_s4_equal_post_state`, Quint `InvPartitionIndependence`, Lean `core_partition_independent`):** **WHEN** an allocating multi-leaf run is proved at `S = 16` and `S = 4` **THEN** both runs yield identical terminal `ArchState`, `outcome_d`, and `n_steps`.
- **`CORE-003` (Top-of-Stack Register Cache and Non-Zero Empty Sentinels):** `CORE` SHALL hold top `Frame` and `Task` records in registers (`0` hashes on `RewriteTop` and `1 <-> 0` transitions), spill/restore deeper entries via `CHIP_HASH = 6`, and enforce `fcount == 0 => fp_hash == EMPTY_STACK_FRAME != 0` (and symmetric `TASKS`). *(259 chars)*
  - **Scenario `S-CORE-003a` (`mut_core_empty_stack_zero_digest`, `mut_core_pop_forged_restored_frame`):** **WHEN** `fcount = 0` with `fp_hash = 0` or a deep pop alters a restored frame field **THEN** `MockProver` rejects the witness.
