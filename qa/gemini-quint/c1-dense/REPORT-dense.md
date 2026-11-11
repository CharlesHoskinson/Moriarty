# Soundness Review Report: C1 Dense Tail-Buffered Merkle Memory (`membackend.rs`, `dense.rs`, `poseidon.rs`)

Independent formal soundness review of the Sherlock chip-leaf dense tail-buffered Merkle memory backend (Experiment C1) as built in:
- `crates/mori-chip/src/membackend.rs` (`MemBackend` trait, `WindowCfg`, `Win`, `Bits`, `Inc`)
- `crates/mori-chip/src/dense.rs` (`DenseCfg`, `DenseModel`, `persist`, `cold_read`, `walk`, `mask`, `flush`, `geq`, capacity-tagged domain helpers)
- `crates/mori-chip/src/dense/tests.rs` (`CoreArena`, `Chip`, 9 positive `MockProver` tests, 21 negative mutant tests)
- `crates/mori-chip/src/poseidon.rs` (`SpongeCfg::sponge_iv`, `sponge_native_iv`)
- `crates/mori-chip/src/bus.rs` (`Bus::provide`, `Bus::request`, `Bus::lookup_any`)

Companion Quint specification and test suite:
- `qa/gemini-quint/c1-dense/dense.qnt`
- `qa/gemini-quint/c1-dense/dense_test.qnt`

---

## 1. Per-Question Verdicts and Required Code Constraints

| # | Question | Verdict | Summary & Code Constraint |
|---|---|---|---|
| **1** | `(len, root, tail_d)` after a leaf depends only on the append sequence, under any leaf partition (`SRC-023`). | **SOUND WITH CONDITIONS** | Holds under **C1** (CORE append chain & boundary stitch), **C2** (`TOP >= 2` and `u.sibs.len() == w.top`), and Poseidon collision resistance. Verified across 4 partitions (`[9]`, `[4,5]`, `[3,0,6]`, `[1..1]`) in `src023PartitionIndependenceTest` and matching `clean/PartitionIndependence.lean`. |
| **2** | A flush happens iff a segment boundary is crossed (`floor(len_post/8) > floor(len_pre/8)`), never into a filled slot (old leaf must be `EMPTY_SEG`, non-zero), and no slot is written twice. | **SOUND WITH CONDITIONS** (Gap at `TOP == 1`) | **Real code gap when `TOP == 1`** (`crates/mori-chip/src/dense.rs:593-599`): in `DenseCfg::walk`, `if l + 1 == n { WALK_LAST } else if l == 0 && first == First::Empty { WALK_EMPTY }` gives `WALK_LAST` precedence over `WALK_EMPTY` when `n == 1` (`TOP == 1`), leaving `x0 == EMPTY_SEG` completely unconstrained. Also `DenseCfg::persist` (`dense.rs:634-683`) lacks `assert_eq!(u.sibs.len(), w.top)` and `assert!(w.top >= 2)`. Sound for all `TOP >= 2` (including `T = 8` and production `TOP = 61, 63`) under **C2**. |
| **3** | The tail is canonical (`PAD` above `r`) and passes through unchanged when nothing is appended; a forged tail across a leaf boundary is refused. | **SOUND** | Enforced by `"dense mask"` (`dense.rs:356-363`), copy equality `r.constrain_equal(t_pre.cell(), pre_tail.cell())` (`dense.rs:661`), `"dense flush"` (`dense.rs:376-384`), and the post-tail mux chain (`dense.rs:706-713`). |
| **4** | The bus key `8 * addr + kind` is injective, also across four arenas through the kind pin; `kind in 0..=4`. | **SOUND** | `"mem win row"` (`membackend.rs:259-266`) constrains `key = act * (8 * addr + kind)`, `kind in 0..=4` via degree-5 product, and `act * kf * (kind - kf) == 0`. Since `0 <= kind <= 4 < 8` and `8 * addr + kind < 2^{69} < p`, `(addr, kind) -> 8 * addr + kind` is injective and isolates the 4 arenas (`kfix = 1..4`) on a shared bus. |
| **5** | Window reads and old-memory reads cover disjoint address ranges; every read is below `len_cur` and carries the expected kind. | **SOUND WITH CONDITIONS** | Enforced in `dense/tests.rs:119-150, 229-296` and `dense.rs:716-752` under **C3** (read port gates & `TOP + 3`-bit decompositions of `diff` and `d2`, plus binding `lc` to the step's running `len` for intra-leaf reads before a subsequent append). |
| **6** | `len <= 2^(TOP+3) - 1`; the counter cannot wrap in the field; a leaf at the last segment is provable for any `W`. | **SOUND** | `WindowCfg::window` (`membackend.rs:388-389`) decomposes `len_pre` and `len_post` into `TOP + 3` bits (`"mem bits last"` forces `acc_out == 0`, `membackend.rs:316-321`). `DenseCfg::persist` (`dense.rs:675-680`) increments `idx_j = idx_{j-1} + f_j` rather than `q_pre + j`, so at `q_pre = 2^TOP - 1` (`f_j = 0`) `idx_j` stays `2^TOP - 1 < 2^TOP` for any `W >= 1`. |
| **7** | Appends and new window rows correspond one to one (state CORE's append-chain assumption exactly). | **SOUND WITH CONDITIONS** | Holds under **C1** (`A1`–`A4` below). Because `bus.lookup_any` (`bus.rs:50-61`) checks set inclusion `RequestSet <= ProviderSet` (not multiset equality), the bijection relies on CORE's contiguous address chain `len_pre .. len_post` combined with key injectivity and the window's active prefix `[q_pre * 8, len_post)`. |
| **8** | Domain separation with tags in the sponge capacity instead of the first input (deviation from AUDIT finding 8): is it equivalent for every hash use here? | **SOUND** | `sponge_iv` (`poseidon.rs:298-354`) folds `F::from(n as u64) + iv(dom, sub)` into the fixed column `rc[2]` on `q_first` (`poseidon.rs:336`). Because `(dom, sub, n) -> (dom << 48) + (sub << 32) + n` (`dense.rs:54-62`) is injective across all call sites (`DOM_REC`, `DOM_PAD`, `DOM_EMPTY`, `DOM_NODE(h)` for `h in 1..=TOP+3`, and `dom = 0`), capacity tagging is prover-immutable and cryptographically equivalent to first-input tagging in the ideal-permutation sponge model while saving 1 permutation per tree node. |

---

### Explicit Code Constraints (`C1`–`C4`)

- **C1 (CORE Append-Chain & Boundary Contract — Questions 1, 7).**
  Because `Bus::lookup_any` (`crates/mori-chip/src/bus.rs:50-61`) proves only that every enabled request tuple appears in the union of enabled provider rows, CORE and the leaf boundary must enforce four constraints for each arena with `m >= 0` appends in a leaf:
  1. **(`A1` Start at `len_pre`)** If `m == 0`, `len_post == len_pre` (`r.constrain_equal(len_pre.cell(), len_post.cell())`, `dense/tests.rs:194`); if `m > 0`, append `0`'s address cell is copy-constrained to `len_pre` (`dense/tests.rs:197`).
  2. **(`A2` Unit step)** Each append `j in 0..m-1` increments its address by `+1` via `WindowCfg::inc` (`next == cur + 1`, `membackend.rs:322-327`, `dense/tests.rs:196-200`).
  3. **(`A3` End at `len_post`)** Append `m - 1`'s output address `next` is copy-constrained to `len_post` (`r.constrain_equal(cur.cell(), len_post.cell())`, `dense/tests.rs:210`).
  4. **(`A4` Bus request & kind pin)** Each append `j` enables `bus.request` (`q_req = 1`) with `chip = CHIP_MEM`, `key = 8 * addr_j + kind_j`, `kind_j == kfix_j in 1..=4`, and payload `[d0_j, d1_j, d2_j]` (`dense/tests.rs:102-118, 202-209`). Across leaf boundaries, `(len_post, root_post, tail_post)` of leaf `t` must be copy-stitched (or public-input equal) to `(len_pre, root_pre, tail_pre)` of leaf `t + 1` (`dense/tests.rs:334-340`).
  - *Why `A1`–`A4` imply a 1-to-1 bijection:* In `WindowCfg::window` (`membackend.rs:248-305`), active rows form a contiguous prefix of addresses `[q_pre * 8, len_post)`, where imported rows cover `[q_pre * 8, len_pre)` (`imp = 1`) and new active rows cover `[len_pre, len_post)` (`imp = 0, act = 1`), with one row per integer address (`nxt.addr == cur.addr + 1`). By `A1`–`A4`, CORE issues `m = len_post - len_pre` bus requests at distinct addresses `len_pre, len_pre + 1, ..., len_post - 1` with `kind_j in 1..=4`. Since `key = 8 * addr + kind` with `0 <= kind < 8` uniquely determines `addr = floor(key / 8)` and `kind = key mod 8`, request `j` can only match the unique window row at `addr_j = len_pre + j` in that arena (`kfix` pin). Neither duplicate window rows at the same address nor unrequested active window rows can exist.

- **C2 (Minimum Top-Tree Depth `TOP >= 2` and Synthesis Asserts — Question 2).**
  `WindowCfg::configure` and `DenseCfg::persist` must require `top >= 2` (`assert!(w.top >= 2)`) and `assert_eq!(u.sibs.len(), w.top)` (matching `DenseCfg::cold_read` at `dense.rs:726`, which already asserts `assert_eq!(w.sibs.len(), w.top + 3)`), OR `DenseCfg::walk` (`dense.rs:593-599`) must copy-constrain `x0` / combine `WALK_LAST` and `WALK_EMPTY` when `n == 1`. See Section 4 for the exact `TOP == 1` exploit.

- **C3 (CORE Read Port & Running-Length Contract — Question 5).**
  Every read of `(addr, kind_expected)` at current length `lc` must enforce the constraints in `dense/tests.rs:119-150, 229-296`:
  1. `cold in {0, 1}` (`c * (1 - c) == 0`);
  2. `diff = cold * (base - 1 - addr) + (1 - cold) * (addr - base)` decomposed into `TOP + 3` bits via `WindowCfg::bits` (`base = q_pre * 8` copied from `win.q_pre` scaled by 8);
  3. `d2 = lc - 1 - addr` decomposed into `TOP + 3` bits via `WindowCfg::bits`;
  4. `kind == kfix` (`kfix in 1..=4`) and `key = 8 * addr + kind`;
  5. Warm path (`warm = 1 - cold`): `bus.request` with `warm * [CHIP_MEM, key, d0, d1, d2w]`;
  6. Cold path (`cold`): `DenseCfg::cold_read` with `enable` copied from `cold`, `pre_root` copied from `pre.root`, and `[addr, kind, d0, d1, d2w]` copied from the read port (`dense.rs:716-752`);
  7. In a real multi-step leaf where a read occurs *before* a later append in the same leaf, `lc` must be bound to that step's running `len` from the append chain rather than `len_post` (as noted in `research/sherlock-final-design-20261111/EXPERIMENTS.md:166`; the test harness `dense/tests.rs:241` only offers `at_post: bool` selecting `len_pre` or `len_post`).

- **C4 (Genesis Initialization — Questions 1, 2, 4).**
  At genesis (`len = 0`), an empty arena must initialize `(len = 0, root = empty_at(&f, TOP + 3), tail_d = tail_d(&f, &[]))` (`dense.rs:86-119, 144-152`). Any pre-populated `Init` records must have `kind in 1..=4` (matching the arena's `kfix`) and canonical `(len, root, tail_d)` built from `DenseModel::push`.

---

## 2. Numbered Findings, Witnesses, Source Lines, and Closing Constraints

1. **Finding F1 (Real Code Gap at `TOP == 1`: `WALK_LAST` Shadows `WALK_EMPTY` in `DenseCfg::walk`).**
   - **Witness:** `buildTampered("top1_walk_empty_shadowed", 23)` in `dense.qnt:1126-1165` (`mutant23_top1WalkEmptyShadowedTest` in `dense_test.qnt:360-367`).
   - **Source lines:** `crates/mori-chip/src/dense.rs:568-571, 593-599, 681`.
   - **Details:** In `DenseCfg::persist` (`dense.rs:681`), `old_walk` passes `x0` as `Src::known(f, empty_seg(&self.f))` (`Src::Val`), which assigns the witness cell without a copy constraint (`dense.rs:568-571` only copy-constrains `Src::Cell`). It relies on selector `self.s_walk_empty` (`"dense walk empty"`, `dense.rs:407-415`) at `l == 0` to enforce `cur(m, 5) - empty == 0`. However, `DenseCfg::walk` (`dense.rs:593-599`) selects the row selector with:
     ```rust
     let q = if l + 1 == n {
         self.s_walk_last
     } else if l == 0 && first == First::Empty {
         self.s_walk_empty
     } else {
         self.s_walk
     };
     ```
     When `n == 1` (`TOP == 1`, or if `u.sibs` has length 1 because `persist` does not assert `u.sibs.len() == w.top`), level `l = 0` satisfies `l + 1 == n`, enabling `s_walk_last` instead of `s_walk_empty`. As a result, `x0 == EMPTY_SEG` is never constrained, allowing a prover to overwrite an already-occupied segment slot (`oldRoot == rin` passes with `x0 = occRoot`).
   - **Closing constraint:** **C2** (`TOP >= 2`, `assert_eq!(u.sibs.len(), w.top)` in `persist`, or fixing selector priority / copying `x0` from a fixed cell). For `TOP >= 2` (all tested configurations `T = 8` and `TOP = 61, 63`), `l = 0` has `l + 1 = 1 < n`, so `s_walk_empty` is enabled and closes `empty_seg_forgery` (`buildTampered("empty_seg_forgery", 6)`).

2. **Finding F2 (Cross-Leaf Tail Authenticity).**
   - **Witness:** `buildTampered("forged_tail_across_leaf_boundary", 1)` (`dense.qnt:1049-1071`, `mutant01_forgedTailAcrossLeafBoundaryTest`).
   - **Source lines:** `crates/mori-chip/src/dense.rs:651-661`.
   - **Closing constraint:** `r.constrain_equal(t_pre.cell(), pre_tail.cell())` (`dense.rs:661`), which binds the segment-tree root `t_pre` of the `m_imp`-masked window rows `0..7` to `pre.tail_d`.

3. **Finding F3 (Canonical Tail Padding Above `r`).**
   - **Witness:** `buildTampered("non_canonical_tail", 2)` (`dense.qnt:1072-1086`, `mutant02_nonCanonicalTailTest`).
   - **Source lines:** `crates/mori-chip/src/dense.rs:356-363, 526-546, 651-673`.
   - **Closing constraint:** Gate `"dense mask"` (`out - pad - sel * (h - pad) == 0`, `dense.rs:361`) on `m_imp` and `m_act`, together with prefix monotonicity of `imp` and `act` (`membackend.rs:274-275`).

4. **Finding F4 (Mandatory Flush on Segment Boundary Crossing & No Spurious Flush).**
   - **Witnesses:** `buildTampered("flush_skipped", 3)` (`dense.qnt:1087-1102`, `mutant03_flushSkippedTest`) and `buildTampered("flush_spurious", 4)` (`dense.qnt:1103-1112`, `mutant04_flushSpuriousTest`).
   - **Source lines:** `crates/mori-chip/src/dense.rs:376-384, 676-704`.
   - **Closing constraint:** Copy of `f_j = win.rows[8 * j + 7].act` into `"dense flush"` (`dense.rs:677, 691`) and gate equation `out - rin - f * (new - rin) == 0` (`dense.rs:382`). Because `win.rows[0].addr = 8 * q_pre` and active rows are `[8 * q_pre, len_post)`, `win.rows[8 * j + 7].act == 1` iff `len_post >= 8 * (q_pre + j + 1)`, i.e. `floor(len_post / 8) > q_pre + j`.

5. **Finding F5 (No Overwrite of Occupied Segment Slot & Shared Old/New Walk Path).**
   - **Witnesses:** `buildTampered("address_reuse_occupied_slot", 5)` (`dense.qnt:1126-1165`, `mutant05_addressReuseOccupiedSlotTest`), `buildTampered("empty_seg_forgery", 6)` (`mutant06_emptySegForgeryTest`), and `buildTampered("new_walk_unshared_sibs", 21)` (`dense.qnt:1475-1500`, `mutant21_newWalkUnsharedSibsTest`).
   - **Source lines:** `crates/mori-chip/src/dense.rs:381, 407-415, 572-579, 681-689`.
   - **Closing constraints:** Gate `"dense flush"` equation `f * (old - rin) == 0` (`dense.rs:381`), `"dense walk empty"` equation `cur(m, 5) - empty == 0` (`dense.rs:413`), and the copy constraints in `DenseCfg::walk` (`dense.rs:572-579`) sharing `[s, b, acc_in, acc_out]` at every level `l` between `old_walk` and `new_walk`.

6. **Finding F6 (Interior Window Row Address Reuse).**
   - **Witness:** `buildTampered("address_reuse_window_row", 7)` (`dense.qnt:1166-1208`, `mutant07_addressReuseWindowRowTest`).
   - **Source lines:** `crates/mori-chip/src/membackend.rs:269-281`.
   - **Details:** Row `0` has `addr = 8 * q_pre` (`"mem win first"`), the `imp` `1 -> 0` boundary row has `addr = len_pre - 1`, and the `act` `1 -> 0` boundary row has `addr = len_post - 1`. However, an *interior* imported tail row (such as row `1` when `r_pre = 3`) is neither at row `0` nor at the `imp`/`act` transition boundaries, nor is its `addr` hashed into `t_pre`. Without `nxt.addr == cur.addr + 1`, row `1` can set `addr = 8 * q_pre` (reusing row `0`'s address) and serve a forged value to a warm read of `8 * q_pre`.
   - **Closing constraint:** `"mem win chain"` equation `nxt(m, a[5]) - cur(m, a[5]) - one() == 0` (`membackend.rs:272`).

7. **Finding F7 (Read Bounds `addr < len_cur` and Warm/Cold Disjointness).**
   - **Witnesses:** `buildTampered("read_at_len", 8)` (`mutant08_readAtLenTest`), `buildTampered("read_beyond_len", 9)` (`mutant09_readBeyondLenTest`), `buildTampered("future_record_read", 10)` (`mutant10_futureRecordReadTest`), `buildTampered("cold_value_forged", 13)` (`mutant13_coldValueForgedTest`), and `buildTampered("cold_route_forged", 14)` (`dense.qnt:1264-1314`, `mutant14_coldRouteForgedTest`).
   - **Source lines:** `crates/mori-chip/src/dense/tests.rs:119-145, 241-295` and `crates/mori-chip/src/dense.rs:385-392, 716-752`.
   - **Closing constraints:** `TOP + 3`-bit decompositions of `diff = cold * (base - 1 - addr) + (1 - cold) * (addr - base)` and `d2 = lc - 1 - addr` (`dense/tests.rs:286-287`), `cold_read`'s unconditional `TOP + 3`-bit walk on `rd.addr` (`dense.rs:730-740`), and `"dense geq"` `en * (a - b) == 0` (`dense.rs:390`). Because `2^(TOP+3) <= 2^66 \ll p`, negative differences `base - 1 - addr` or `lc - 1 - addr` wrap to `p - \Delta > 2^254` and cannot decompose into `TOP + 3` bits.

8. **Finding F8 (Bus Key Injectivity `8 * addr + kind`, Kind Range `0..=4`, Arena Pin `kfix`, and Padding Row Silence).**
   - **Witnesses:** `buildTampered("wrong_kind_warm", 11)` (`mutant11_wrongKindWarmTest`), `buildTampered("wrong_kind_cold", 12)` (`mutant12_wrongKindColdTest`), `buildTampered("pad_row_on_bus", 16)` (`mutant16_padRowOnBusTest`), `buildTampered("kind_out_of_range", 17)` (`dense.qnt:1353-1392`, `mutant17_kindOutOfRangeTest`), and `buildTampered("arena_kind_pinned", 18)` (`dense.qnt:1393-1427`, `mutant18_arenaKindPinnedTest`).
   - **Source lines:** `crates/mori-chip/src/membackend.rs:248-268` and `crates/mori-chip/src/dense/tests.rs:102-150`.
   - **Details:** If `kind in 0..=4` is omitted (`kind_out_of_range`), two active window rows at consecutive addresses `addr = 8` (`kind = 9`) and `addr = 9` (`kind = -7`) swap their bus keys (`8*8 + 9 = 73 = 8*9 + 1` and `8*9 - 7 = 65 = 8*8 + 1`), satisfying CORE's append requests while corrupting the committed tail digest.
   - **Closing constraints:** `"mem win row"` (`membackend.rs:259-266`): `chip = act * CHIP_MEM`, `key = act * (8 * addr + kind)`, `na * kind == 0`, `na * d_j == 0`, `kind * (kind - 1) * (kind - 2) * (kind - 3) * (kind - 4) == 0`, and `act * kf * (kind - kf) == 0`.

9. **Finding F9 (Counter Range `len <= 2^(TOP+3) - 1` and Last-Segment Completeness for `W >= 2`).**
   - **Witnesses:** `buildTampered("counter_wrap_2_64", 15)` (`dense.qnt:1315-1331`, `mutant15_counterWrap264Test`) and `buildLastSegmentHonest("walk_idx_unconditional_inc")` (`dense.qnt:990-1010`, `lastSegmentCounterEdgeAndTwoUnitsTest` in `dense_test.qnt:117-131`).
   - **Source lines:** `crates/mori-chip/src/membackend.rs:306-321, 388-389` and `crates/mori-chip/src/dense.rs:675-680`.
   - **Closing constraints:** `WindowCfg::bits` (`"mem bits last"` `acc_out == 0` on bit `TOP + 2`) bounds `len_pre, len_post in 0..=2^(TOP+3)-1`, and `"mem win last"` (`membackend.rs:302-305`) forces `win.rows[8*(W+1)-1].act == 0` so at least one padding row always remains in the window. In `DenseCfg::persist` (`dense.rs:675-680`), `idx_j = idx_{j-1} + f_j` ensures that at the last segment `q_pre = 2^TOP - 1` (where `len_post <= 2^(TOP+3) - 1` forces `f_j == 0` for all `j`), every flush unit walks `idx_j = 2^TOP - 1 < 2^TOP` rather than overflowing `TOP` bits at `q_pre + j`.

10. **Finding F10 (Bijection Between CORE Appends and Active New Window Rows).**
    - **Witnesses:** `buildTampered("close_advances_len", 19)` (`dense.qnt:1428-1448`, `mutant19_closeAdvancesLenTest`) and `buildTampered("append_value_mismatch", 20)` (`dense.qnt:1449-1474`, `mutant20_appendValueMismatchTest`).
    - **Source lines:** `crates/mori-chip/src/membackend.rs:276, 322-327` and `crates/mori-chip/src/dense/tests.rs:102-118, 193-211`.
    - **Closing constraints:** **C1** (`A1`–`A4`) together with `"mem win chain"` equation `ca * (1 - na) * (ca_addr + 1 - lp) == 0` (`membackend.rs:276`).

11. **Finding F11 (Capacity-Based Domain Separation and Tree-Height Binding).**
    - **Witness:** `buildTampered("domain_tag_dropped", 22)` (`dense.qnt:1501-1528`, `mutant22_domainTagDroppedTest` and `domainTagInjectivityTest` in `dense_test.qnt:134-140, 350-358`).
    - **Source lines:** `crates/mori-chip/src/poseidon.rs:107-120, 298-354` and `crates/mori-chip/src/dense.rs:54-83`.
    - **Details:** Both `pad_d` (`dense.rs:71-73`) and `empty_seg` (`dense.rs:77-79`) call `sponge_native_iv(f, &[0, 0], iv)` on `[0, 0]`, and `node_h(f, h, x, y)` (`dense.rs:81-83`) calls `sponge_native_iv(f, &[x, y], iv(DOM_NODE, h))`. If capacity domain/height tags are dropped (`iv = 0`), `pad_d() == empty_seg()` and `node_h(1..2, x, y) == node_h(3..4, x, y)`, making a height-`SEG_BITS` tail tree `[rec_d(r), pad_d, pad_d, pad_d]` identical to a height-`TOP + SEG_BITS` top tree `[rec_d(r), empty_seg, empty_seg, empty_seg]` and allowing `persist`'s `old_walk` to accept a tail tree as `pre.root`.
    - **Closing constraint:** `SpongeCfg::sponge_iv` (`poseidon.rs:336`) adds `F::from(n as u64) + iv` into the fixed column `rc[2]` on `q_first`, with injective `iv(dom, sub) = (dom << 48) + (sub << 32)` (`dense.rs:60-62`). Note also that `tail_d` (`r < 8`) and full segment roots (`r = 8`) share the `ts[j]` segment trees in `persist` (`dense.rs:664-673`) and cannot collide because leaf `7` of a partial tail (`r < 8`) is `pad_d()` (`DOM_PAD = 2, n = 2`) whereas leaf `7` of a full segment (`r = 8`) is `rec_d(r_7)` (`DOM_REC = 1, n = 4`).

---

## 3. Attack Questions to Mutant Modules Mapping

| # | Attack Question | Mutant Module (`--main`) | Witness (`dense.qnt`) | Corresponding Rust Test (`dense/tests.rs`) | Expected `quint run --invariant invRefine` Exit | Honest Witness Refines Under Mutant? |
|---|---|---|---|---|---|---|
| 1 | Forged tail across leaf boundary (Q1, Q3) | `dense_forged_tail_across_leaf_boundary` | `buildTampered(..., 1)` | `c1_mut_forged_tail_across_leaf_boundary` (`:429`) | `1` (`[violation]`) | Yes (`honestUnderAllMutantsRefinesTest`) |
| 2 | Non-canonical tail padding above `r` (Q3) | `dense_non_canonical_tail` | `buildTampered(..., 2)` | `c1_mut_non_canonical_tail` (`:456`) | `1` (`[violation]`) | Yes |
| 3 | Segment boundary crossed without flushing (Q1, Q2) | `dense_flush_skipped` | `buildTampered(..., 3)` | `c1_mut_flush_skipped` (`:482`) | `1` (`[violation]`) | Yes |
| 4 | Spurious flush when no boundary crossed (Q1, Q2) | `dense_flush_spurious` | `buildTampered(..., 4)` | `c1_mut_flush_spurious` (`:505`) | `1` (`[violation]`) | Yes |
| 5 | Flush into an already-occupied tree slot (Q2) | `dense_address_reuse_occupied_slot` | `buildTampered(..., 5)` | `c1_mut_address_reuse_occupied_slot` (`:527`) | `1` (`[violation]`) | Yes |
| 6 | Old walk starts at occupying root instead of `EMPTY_SEG` (Q2) | `dense_empty_seg_forgery` | `buildTampered(..., 6)` | `c1_mut_empty_seg_forgery` (`:559`) | `1` (`[violation]`) | Yes |
| 7 | Window row reuses previous row's address (Q4, Q7) | `dense_address_reuse_window_row` | `buildTampered(..., 7)` | `c1_mut_address_reuse_window_row` (`:584`) | `1` (`[violation]`) | Yes |
| 8 | Read at `addr == len_cur` (Q5) | `dense_read_at_len` | `buildTampered(..., 8)` | `c1_mut_read_at_len` (`:601`) | `1` (`[violation]`) | Yes |
| 9 | Read at `addr > len_cur` (Q5) | `dense_read_beyond_len` | `buildTampered(..., 9)` | `c1_mut_read_beyond_len` (`:614`) | `1` (`[violation]`) | Yes |
| 10 | Intra-leaf read of future record at `len_pre` (Q5) | `dense_future_record_read` | `buildTampered(..., 10)` | `c1_mut_future_record_read` (`:627`) | `1` (`[violation]`) | Yes |
| 11 | Warm read returns record of wrong `kind` (Q4, Q5) | `dense_wrong_kind_warm` | `buildTampered(..., 11)` | `c1_mut_wrong_kind_warm` (`:647`) | `1` (`[violation]`) | Yes |
| 12 | Cold read returns record of wrong `kind` (Q4, Q5) | `dense_wrong_kind_cold` | `buildTampered(..., 12)` | `c1_mut_wrong_kind_cold` (`:665`) | `1` (`[violation]`) | Yes |
| 13 | Cold read returns forged payload word `d0` (Q5) | `dense_cold_value_forged` | `buildTampered(..., 13)` | `c1_mut_cold_value_forged` (`:688`) | `1` (`[violation]`) | Yes |
| 14 | Window address `addr >= base` routed to cold read (Q5) | `dense_cold_route_forged` | `buildTampered(..., 14)` | `c1_mut_cold_route_forged` (`:703`) | `1` (`[violation]`) | Yes |
| 15 | Length counter exceeds `2^(TOP+3) - 1` / wraps (Q6) | `dense_counter_wrap_2_64` | `buildTampered(..., 15)` | `c1_mut_counter_wrap_2_64` (`:724`) | `1` (`[violation]`) | Yes |
| 16 | Inactive padding window row provides bus tuple (Q4, Q7) | `dense_pad_row_on_bus` | `buildTampered(..., 16)` | `c1_mut_pad_row_on_bus` (`:744`) | `1` (`[violation]`) | Yes |
| 17 | Window `kind` outside `0..=4` aliases `8*addr + kind` (Q4) | `dense_kind_out_of_range` | `buildTampered(..., 17)` | `c1_mut_kind_out_of_range` (`:758`) | `1` (`[violation]`) | Yes |
| 18 | Cross-arena write/read with wrong `kfix` on shared bus (Q4) | `dense_arena_kind_pinned` | `buildTampered(..., 18)` | `c1_mut_arena_kind_pinned` (`:777`) | `1` (`[violation]`) | Yes |
| 19 | Leaf close advances `len_post` beyond CORE appends (Q7) | `dense_close_advances_len` | `buildTampered(..., 19)` | `c1_mut_close_advances_len` (`:795`) | `1` (`[violation]`) | Yes |
| 20 | Window row payload disagrees with CORE append (Q7) | `dense_append_value_mismatch` | `buildTampered(..., 20)` | `c1_mut_append_value_mismatch` (`:818`) | `1` (`[violation]`) | Yes |
| 21 | New walk uses different siblings/bits from old walk (Q2) | `dense_new_walk_unshared_sibs` | `buildTampered(..., 21)` | (gate/copy check at `dense.rs:572-579`) | `1` (`[violation]`) | Yes |
| 22 | Sponge capacity domain/height tags omitted (Q8) | `dense_domain_tag_dropped` | `buildTampered(..., 22)` | (domain tags at `dense.rs:54-83`) | `1` (`[violation]`) | Yes |
| 23 | `TOP == 1` (`n == 1`) `WALK_LAST` shadows `WALK_EMPTY` (Q2) | `dense_top1_walk_empty_shadowed` | `buildTampered(..., 23)` | **Real code gap** (`dense.rs:593-599`) | `1` (`[violation]`) | Yes |
| — | Unconditional `idx_j = q_pre + j` at last segment (`W = 2`) (Q6) | (tested in `dense_test`) | `buildLastSegmentHonest("walk_idx_unconditional_inc")` | `c1_mut_walk_idx_unconditional_inc` (`:835`) | Completeness test in `dense_test` | Honest witness at `q_pre = 2^TOP - 1, W = 2` is refused under mutant, accepted under `dense_sound` |

---

## 4. Real Gap in the Rust Code and Failing `MockProver` Test Specification

### The Bug (`crates/mori-chip/src/dense.rs:568-571, 593-599, 681`)

In `DenseCfg::walk` (`crates/mori-chip/src/dense.rs:548-617`):
1. At level `l == 0`, `x0` is copy-constrained to the walk row's `wx` (`a[5]`) **only** when `x0` is `Src::Cell` (`dense.rs:568-571`):
   ```rust
   if l == 0 {
       if let Src::Cell(x) = &x0 {
           r.constrain_equal(wx.cell(), x.cell())?;
       }
   }
   ```
2. In `DenseCfg::persist` (`dense.rs:681`), `old_walk` passes `x0` as `Src::known(f, empty_seg(&self.f))` (`Src::Val`), relying on the selector `self.s_walk_empty` (`"dense walk empty"`, `dense.rs:407-415`, which adds the polynomial `cur(m, 5) - empty == 0`) to constrain `wx == empty_seg(&self.f)` at `l == 0`.
3. However, the selector assignment at `dense.rs:593-599` is:
   ```rust
   let q = if l + 1 == n {
       self.s_walk_last
   } else if l == 0 && first == First::Empty {
       self.s_walk_empty
   } else {
       self.s_walk
   };
   ```
   When `n == 1` (`TOP == 1`, or whenever `u.sibs.len() == 1`, since `DenseCfg::persist` at `dense.rs:634-683` does not assert `assert_eq!(u.sibs.len(), w.top)`), the single level `l = 0` satisfies `l + 1 == n`. Therefore `self.s_walk_last` is enabled and **`self.s_walk_empty` is never enabled**. Neither a copy constraint nor `"dense walk empty"` constrains `wx == EMPTY_SEG`.

### Concrete Failing `MockProver` Test (`c1_gap_top1_empty_seg_forgery_accepted`)

In `crates/mori-chip/src/dense/tests.rs`:
- **Circuit configuration:** Instantiate `Chip::<1, 1>` (`TOP = 1, W = 1`, total capacity `2^(1+3) = 16` records across 2 segment slots `q in {0, 1}`).
- **Pre-state (`m_occupied`):**
  1. Start with `m = DenseModel::new(1)`. Push 5 records `r(10)..r(14)` (`len = 5`, so slot `q = 0` is flushed with segment `S_0`, `q_pre = 0`, `r_pre = 5`).
  2. Overwrite slot `q = 0` in `m.segs` with a different already-occupying segment digest `occ_seg = tree_seg(&f, &[rec_d(r(90)); 8])`, so `pre_root = node_h(&f, 4, occ_seg, empty_seg(&f))` has slot `q = 0` already occupied by `occ_seg != empty_seg(&f)`.
- **Leaf witness:**
  1. Run `assign_arena` with `appends = &[r(15), r(16), r(17)]` (3 records, crossing the segment boundary `5 -> 8`, so `f_0 = 1` at `idx_0 = q_pre = 0`).
  2. In `DenseCfg::persist`'s `old_walk` of unit `0` (`l = 0`, the first and only level of `old_walk`), overwrite advice column `a[5]` (`wx`, the walk leaf cell) from `empty_seg(&f)` to `occ_seg`, and overwrite `old_walk`'s sponge input (`in0` at `q_first`, `a[2]`) and output (`out` at the final permutation row, `a[0]`) to the sponge of `(occ_seg, sib_0)` at height `4`, which equals `pre_root` (`rin`)!
- **Expected vs. actual result:**
  - **Should fail:** `"dense walk empty"` (`cur(m, 5) - empty == 0`) or `"dense flush"` (`f * (old - rin) == 0`), because slot `q = 0` in `pre_root` holds `occ_seg != EMPTY_SEG`.
  - **Actually happens:** `MockProver::run(K, &chip, vec![])?.verify()` **returns `Ok(())`**! Because `n = 1`, `l = 0` enables `s_walk_last` (`"dense walk last"`) instead of `s_walk_empty` (`"dense walk empty"`), so `a[5] == EMPTY_SEG` is never checked, `old == pre_root` passes `"dense flush"`, and the occupied slot `q = 0` is silently overwritten.

---

## 5. Model Abstractions and Differences from the Rust Code

1. **Segment and tree sizes (`dense.qnt:26-31`):**
   - Rust (`crates/mori-chip/src/membackend.rs:19-25`, `dense/tests.rs:34`): `SEGC = 8`, `SEG_BITS = 3`, `TOP = 5` in tests (`T = 8`, capacity `2^8 = 256`) and `TOP = 61` (`MAX_LEN = 2^64`) in production.
   - Quint (`dense.qnt:26-31`): `SEGC = 4`, `SEG_BITS = 2`, `TOP = 2`, `NB = TOP + SEG_BITS = 4`, `TOP_SLOTS = 4`, `MAX_LEN = 16`. The bus key formula `8 * addr + kind` (`dense.qnt:72`) and kind range `0..=4` are kept identical to the Rust code (`membackend.rs:260-265`).
2. **Field arithmetic vs. bounded integers (`dense.qnt:49, 616-631`):**
   - Rust operates over $\mathbb{F}_p$ (BLS12-381 scalar field, $p \approx 2^{255}$).
   - Quint uses 64-bit signed integers (`int`). In `checkBits` (`dense.qnt:616-631`), `rows.length() == NB`, `b in {0, 1}`, `accIn == 2 * accOut + b`, and `accOut == 0` on the last bit restrict the decomposed integer to `0..2^NB - 1` (`0..15`). Negative differences (`base - 1 - addr < 0` or `lc - 1 - addr < 0`) produce `accOut = -1 != 0` in `buildBits` (`dense.qnt:238-249`) and fail `checkBits`, faithfully matching the field wraparound `p - \Delta >= p - 16 > 2^NB` in $\mathbb{F}_p$.
3. **Poseidon sponge (`dense.qnt:77-115`):**
   - Rust (`crates/mori-chip/src/poseidon.rs:298-354`): width-3 Poseidon sponge over $\mathbb{F}_p$ with 8 full rounds + 56 partial rounds per permutation, adding `n + iv(dom, sub)` into fixed column `rc[2]` on `q_first`.
   - Quint (`dense.qnt:77-115`): models `sponge2Iv` (1 permutation) and `sponge4Iv` (2 chained permutations) over $\mathbb{Z}_{HP}$ (`HP = 1000003`, all intermediate products $< 2 \times 10^{12} \ll 2^{63}-1$) with capacity tag `n + iv(mt, dom, sub)` where `iv(mt, dom, sub) = dom * 1000 + sub * 10` (or `0` when `mt == "domain_tag_dropped"`).
4. **Row layout vs. structured records (`dense.qnt:232-360`):**
   - Rust lays out `Bits`, `Win`, `Walk`, `Mask`, `Flush`, and `Geq` in shared advice columns `a[0..7]` with copy constraints (`constrain_equal`) between regions.
   - Quint represents each sub-region as typed records (`BitRowW`, `WinRowW`, `WalkLevelW`, `FlushUnitCircuit`, `AppendPortW`, `ReadPortW`, `ArenaLeafW`) and models copy constraints as explicit equality predicates in `checkWindow`, `checkPersist`, `checkCoreAndReads`, and `acceptsChain`.

---

## 6. Exact `quint` Commands and Expected Exit Codes

### Honesty Note on Sandbox Execution
In this environment, running `quint --version` via the shell tool was refused/canceled by the sandbox (`Tool execution was canceled`), as noted in the task instructions. Therefore, **no `quint` command produced output inside this session**; all commands below are written to be executed by the lead's re-run harness.

### Commands and Expected Results

Run from the repository root (or `qa/gemini-quint/c1-dense/`):

| # | Command | Expected Exit Code | Expected Output / Meaning |
|---|---|---|---|
| 1 | `quint typecheck qa/gemini-quint/c1-dense/dense.qnt` | `0` | Typecheck passes |
| 2 | `quint typecheck qa/gemini-quint/c1-dense/dense_test.qnt` | `0` | Typecheck passes |
| 3 | `quint test --main dense_test qa/gemini-quint/c1-dense/dense_test.qnt` | `0` | 33 passing tests (6 positive completeness/SRC-023/pass-through/4-arena/last-segment tests, 1 domain-tag injectivity test, 1 non-vacuity test across all 23 mutants, 23 per-mutant counterexample tests, 3 exhaustive pure domain fold tests) |
| 4 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_sound --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `0` | `[ok]` — sound model satisfies `invRefine` |
| 5 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_sound --invariant reachAcceptedBadTamper --max-samples 4000 --max-steps 3 --seed 0x20261111` | `0` | `[ok]` — no accepted bad tamper exists in `dense_sound` |
| 6 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_sound --invariant reachHonestFlushAndReads --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` — non-vacuity: honest flushing leaf with 3 reads is reached (`acc: true, isTampered: false`) |
| 7 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_sound --invariant reachHonestCrossLeafTail --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` — non-vacuity: honest second leaf across tail boundary is reached (`acc: true, isTampered: false`) |
| 8 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_forged_tail_across_leaf_boundary --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 9 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_non_canonical_tail --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 10 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_flush_skipped --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 11 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_flush_spurious --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 12 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_address_reuse_occupied_slot --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 13 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_empty_seg_forgery --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 14 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_address_reuse_window_row --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 15 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_read_at_len --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 16 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_read_beyond_len --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 17 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_future_record_read --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 18 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_wrong_kind_warm --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 19 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_wrong_kind_cold --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 20 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_cold_value_forged --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 21 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_cold_route_forged --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 22 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_counter_wrap_2_64 --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 23 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_pad_row_on_bus --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 24 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_kind_out_of_range --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 25 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_arena_kind_pinned --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 26 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_close_advances_len --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 27 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_append_value_mismatch --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 28 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_new_walk_unshared_sibs --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 29 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_domain_tag_dropped --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
| 30 | `quint run qa/gemini-quint/c1-dense/dense.qnt --main dense_top1_walk_empty_shadowed --invariant invRefine --max-samples 4000 --max-steps 3 --seed 0x20261111` | `1` | `[violation]` (`acc: true, isTampered: true`) |
