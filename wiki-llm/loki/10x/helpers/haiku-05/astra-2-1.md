# astra-2-1: row and lookup diet audit (Sherlock A102/A80 parent)

## Question

Audit the exact local Sherlock A102/A80 parent rows (if retained), recl9 REPORT.md and source, and the foreign ECC / range / MSM code in midnight-circuits-7.2.4. Find concrete routes to k19/k18 (table tags, fixed-base partition, scalar bounds) and quantify honest advice / table / column / degree budgets per parent and decider. Read-only. No repo edits, no benchmark. (Read-only: no cargo build or synthesis was run.)

## Answer

- **The Sherlock A102/A80 parent has no retained row counts.** No `sherlock-parent-keygen` output, no parent key, and no sherlock cost log exists under `recl9/logs`, `recl9/out`, or `recl9/keys` (only `decider`, `leaf4`, `node4`). `recl9/src/sherlock_cli.rs` prints `cost` JSON from `keygen_parent` (`sherlock_cli.rs:223-235`), but its output was not kept. `chain/evidence/sherlock-communication-opening-source-20261020/README.md:30` says the new decider's k and costs are "unknown". The only measured proxy is the homogeneous arity-2 node (574,040 advice / 1,032,465 table, k20).
- **Table rows are fully explained by a binary tag set (derived, high confidence).** `Pow2RangeChip::load_table` (`midnight-circuits-7.2.4/src/field/decomposition/pow2range.rs:216-236`) writes one row for tag 0 and 2^n rows for each queried tag n >= 1. Decomposing the measured totals in binary gives exactly:
  - homogeneous node 1,032,465 = 1 + (2^4 + 2^8 + 2^14 + 2^15 + 2^16 + 2^17 + 2^18 + 2^19), so tags {4, 8, 14, 15, 16, 17, 18, 19};
  - decider 1,032,209 = 1 + (2^4 + 2^14 + ... + 2^19), so tags {4, 14, 15, 16, 17, 18, 19}, without tag 8.
  - This is an inference from the arithmetic, not from reading `queried_tags` at runtime. Confirm by printing `queried_tags` in a synthesis run (not done). It is consistent with both measured totals but is not proof of the tag set.
- **The table is driven by the largest limbs, and tags 18 and 19 are the big ones.** Tag 19 alone is 524,288 rows and tag 18 alone is 262,144. The planner sets `max_bit_len = k-1` (`midnight-zk-stdlib-2.3.5/src/lib.rs:1873`). `compute_optimal_limb_sizes` (`midnight-circuits-7.2.4/src/field/decomposition/cpu_utils.rs:139-180`) minimises the number of chunks, not table rows, so it prefers 18–19-bit limbs. Node-level table: removing tag 19 gives 508,177; removing 18 and 19 gives 246,033, which fits k18 (usable 262,137).
- **Advice is the binding constraint, not the table.** Advice rows for the arity-2 proxy are 574,040, which is already above k19 usable (524,281). Table alone does not make k19 or k18 possible. Node 06's claim that "removing the table bottleneck alone leaves advice above k19" holds.
- **Per-child advice share (measured sweep).** Arity 2 → 3 → 4 gives 574,040 → 765,907 → 957,724 (interim crate). Slope about 191,842 advice rows per child; intercept about 190,356 per node (statement, batch wrapper, collapse, and the rest). The Sherlock parent has two children, so the proxy is about 574k advice, but its actual children have different VKs, so this is a proxy only.
- **k19 threshold (proxy):** advice must fall by 8.67% (to 524,281). The table is already within k19 if tags 18 and 19 are not both present, so it is not a separate blocker at k19 with the observed tag set.
- **k18 threshold (proxy):** advice must fall by 54.3% from 574,040 (to 262,137), or by 72.6% from the hardened arity-4 figure 958,095. Table must also be ≤262,137, which requires tags ≤ 17 with sum < 262,136. Stock at k18 gives max_bit_len 17. The full set {1..17} is 262,143 rows, which is 6 rows over, so one small tag must be dropped (e.g. 8 or 4).
- **Fixed-base terms:** every named fixed base is one entry in the collapse MSM. `fixed_bases` (`midnight-circuits-7.2.4/src/verifier/mod.rs:120-137`) gives `-G` plus one per fixed commitment and one per permutation commitment of each VK. The Sherlock parent merges both VKs' names into one map (`sherlock_link.rs:397-401`). Resolution moves each named scalar into the variable-base list (`verifier/msm.rs:481-490`), so no partition is done today. Column counts: the cost model's `fixed_columns` already includes permutation columns (node 45 = 27 fixed + 18 permutation; leaf 32 = 22 fixed + 10 permutation, per astra-2's review, consistent with the `permutations` field in the cost log). So the leaf VK gives about 22 + 10 + 1 = 33 names, not 40+. The Sherlock child VK name counts are unknown (not retained; child k13/k12 are reported in Moriarty `docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json`).
- **Scalar bounds are worst-case in-circuit.** `assign_bounded_scalars` uses `AssignedBoundedScalar::new(s, None)` (`verifier/utils.rs:64-76`). Verifier scalars are therefore full-width (255 bits) unless a bound is explicitly reduced. Stock already has bound-aware MSM: `msm_by_bounded_scalars` chunks each scalar by its own bit bound with WS=4 (`bits.div_ceil(WS)`) and GLV-splits any scalar with bound > 132 bits (`ecc/foreign/weierstrass_chip.rs:810-860`). But the padding in `windowed_msm` (`weierstrass_chip.rs:1788`, `max_len` over all sequences) still forces short sequences through the longest loop, so per-bound savings are not fully realised. The distinct new opportunity is tighter bound proofs for accumulator scalars plus grouping by bound so the padding is not paid. Node 06's "scalar-bound-aware scheduling" is therefore partly present; the new part is the bound proofs and the grouping.
- **ECC column formula checks:** `NB_LIMBS + max(NB_LIMBS, 2 + moduli.len()) + 1 = 7 + max(7, 4) + 1 = 15` (`ecc/foreign/weierstrass_chip.rs:105`), with `LOG2_BASE=56`, `NB_LIMBS=7`, two moduli (`field/foreign/params.rs:277-283`).
- **Multi-select tables use advice rows, not the table extent.** `load_multi_select_table` calls `fill_dynamic_lookup_row` (`ecc/foreign/weierstrass_chip.rs:1405-1415`), not `assign_table`. A precomputed fixed-base window table therefore costs advice rows (roughly 16 entries per window per base, times the number of windows, uncounted and unmeasured), not table rows.
- **Stock MSM is already bound-aware in the fixtures:** `msm_3` (three full-width scalars) costs 39,935 rows stock and 55,630 hardened; `msm_by_bounded_scalars_3` (widths 4, 12, 255) costs 30,630 stock and 46,318 hardened (`ecc.rs:555-630`; `PATCH-REPORT.md:83-88`). The gap between the two stock fixtures (about 9.3k) is what two short scalars save, but this is an average including setup, not a marginal per-term cost.

## Raw numbers

| Quantity | Value | Source |
|---|---:|---|
| Node arity 2, interim crate: advice / table / k | 574,040 / 1,032,465 / 20 | `recl9/REPORT.md` §3 node sizing; `recl9/logs/cost-node-arity4-interim.log` |
| Node arity 3, interim: advice | 765,907 | REPORT §3 |
| Node arity 4, interim: advice | 957,724 | `cost-node-arity4-interim.log` (`rows`) |
| Node arity 4, hardened (certified): advice | 958,095 | REPORT §3 |
| Decider: advice / table / k | 841,682 / 1,032,209 / 20 | `recl9/keys/decider/keygen.stats.json` |
| Node fixed columns / lookups / permutations / advice columns | 45 / 5 / 18 / 15 | `cost-node-arity4-interim.log` |
| Usable rows at k19 / k18 / k20 | 524,281 / 262,137 / 1,048,569 | cost-model `usable_rows_at_k` |
| Pow2range table: tag n rows | 2^n (tag 0: 1) | `pow2range.rs:221-232` |
| Tag 19 / 18 / 17 rows alone | 524,288 / 262,144 / 131,072 | arithmetic |
| Table after removing tags 19, 18 | 246,033 | arithmetic from the tag set above |
| Table, all tags 1..17 | 262,143 | arithmetic; k18 usable is 262,137 |
| Arity sweep slope / intercept | 191,842 / 190,356 per child / per node | arithmetic from arity 2 and 4 |
| k19 reduction needed (proxy) | 8.67% | 1 - 524,281/574,040 |
| k18 reduction needed (proxy, arity 2) | 54.34% | 1 - 262,137/574,040 |
| k18 reduction needed (proxy, arity 4 hardened) | 72.64% | 1 - 262,137/958,095 |
| 255 bits / 19-bit vs 15-bit limbs | 13.4 vs 17.0 chunks | arithmetic (chunk count, not rows) |
| ECC advice columns | 15 | `weierstrass_chip.rs:105` |
| msm_3 stock / hardened rows | 39,935 / 55,630 | `cost-model.json:2236`; `PATCH-REPORT.md:83-88` |
| msm_by_bounded_scalars_3 stock / hardened | 30,630 / 46,318 | `PATCH-REPORT.md:83-88` |
| Hardened patch, arity 4 extra advice | +371 | REPORT §3 |

## Budgets per parent and decider (proxies, not measurements)

- **Sherlock A102/A80 parent (unmeasured):** proxy advice about 574k (2 children at about 191.8k each, plus about 190k overhead). Table is the same tag family as the homogeneous node unless the planner differs. Expect k20 unless advice drops at least 8.7% (k19) or 54.3% (k18). Note: `keygen_link_parent` derives k from `cost_model(rel, None).k` (`sherlock_link.rs:968-974`); a real number needs a keygen run.
- **Decider (measured):** advice 841,682 forces k20 regardless of table (above k19 usable). Table 1,032,209 is also above k19 usable only if tag 19 is present (it is).
- **Partition route:** the fixed-base count per child is `fixed + perm + 1` names (`fixed_base_names`, shared `-G`). For the leaf VK that is about 22 fixed + 10 permutation + 1 (per astra-2's column split; the cost-model "32" already includes permutations). Sherlock child counts are unknown. Their share of the 574k is unmeasured, so the saving from partitioning cannot be bounded yet. Fixture averages (13k per term stock) are too high to use per term, because they include point assignment.

## Corrections to node 06 (LOKI-DESIGN / nodes/loki-06)

- Node 06 writes that the 55,630 and 46,318 hardened fixtures are "those MSM fixtures" from 39,935 "respectively". The 46,318 is the bounded fixture, whose stock value is 30,630, not 39,935. Keep the stock/hardened pairs separate.
- Node 06 lists "scalar-bound-aware MSM scheduling" as new. The stock `msm_by_bounded_scalars` (`weierstrass_chip.rs:810-860`) already chunks by bound and GLV-splits; the open part is only tighter per-scalar bounds, since all verifier scalars are worst-case (`verifier/utils.rs:64-76`).
- Node 06's table reasoning is consistent with a binary tag set (see Answer). Its "15-bit limb cap" would cut table extent at k20 but adds about 21% to chunk count, so it helps only if table is the binding constraint. At k18 with a 17-bit cap, table (246k with 18–19 removed) is not binding; advice is.

## Corrections after astra-2 review

- Fixed-column counts include permutation columns (see Answer). Do not add permutations twice, and do not infer 40+ leaf names from 32.
- The binary tag decode is inference only. Limiting the planner to 17 bits does not remove exactly tags 18 and 19 without possibly introducing other tags and advice; the k18 table margin (246k vs 262k) is conditional on that.
- Actual Sherlock children are k13 (A102) and k12 (A80) per Moriarty `RUN-REPORT.json`. The actual parent row count is still unknown. The 574k / 191.8k-per-child figures are a homogeneous leaf-child proxy and probably overstate a parent whose children are that small. Re-derive from the actual child VKs before using the 8.7% and 54% thresholds.
- Scalar-bound scheduling exists (`msm_by_bounded_scalars`), but `windowed_msm` padding (`weierstrass_chip.rs:1788`) still forces short sequences through the longest loop. The potential gain is distinct from the new bound proofs.

## Caveats

- No Sherlock parent row, key, or cost output is retained; every Sherlock number above is a proxy from the homogeneous arity-2/4 node, which has different children (leaf VK, not A102/A80).
- Table-tag reconstruction is arithmetic inference; `queried_tags` was not dumped, and no synthesis was run.
- `cost_model(rel, Some(k))` may or may not apply `max_bit_len = k-1` the same way as the Relation's `synthesize` (`lib.rs:1873`); unverified.
- Multi-select partition row costs were not counted and depend on how `fill_dynamic_lookup_row` lays out cells; not measured.
- Interim vs certified: the 958,095 figure is certified; the 574,040 / 765,907 / 957,724 sweep is the interim shift-only crate. Hardened adds about +371 advice rows at arity 4 (about +186 per child estimated), so the arity-2 hardened figure is an estimate.
- The k18 table margin (246k vs 262k) assumes the observed tag set; a different child set could query tag 18 or 19 again.
- Nothing was built, synthesized, or benchmarked; no repo was modified.

## Key file references

- `/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md` §2–§3
- `/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/logs/cost-node-arity4-interim.log`
- `/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/keys/decider/keygen.stats.json`
- `/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:397-416, 444-500, 968-974`
- `/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_cli.rs:223-235`
- `/home/charles/repos/projectfenrir/recursion/stagenet-strawman/verifier-patch/PATCH-REPORT.md:83-88`
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/field/decomposition/pow2range.rs:216-236`
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/field/decomposition/cpu_utils.rs:139-180`
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/ecc/foreign/weierstrass_chip.rs:105, 810-860, 1405-1415`
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/mod.rs:120-137`
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/msm.rs:131-137, 375-377, 451-463, 481-490`
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/utils.rs:64-76`
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/instructions/ecc.rs:555-630`
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1873`
- `/home/charles/repos/Moriarty-factory/wiki-llm/loki/nodes/loki-06-recursive-circuit-cost.md`
