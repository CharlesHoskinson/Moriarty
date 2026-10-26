---
title: "Loki speed-up scout: witness synthesis and circuit layout"
date: 2026-10-24
component: witness-synthesis
sources:
  - Loki source: /home/charles/repos/loki-proofs/src/plonk/prover.rs (compute_trace 57-265, create_proof 397-447, parse_advices 517-665, WitnessCollection 855-1003), src/circuit/floor_planner/single_pass.rs, src/circuit/floor_planner/v1.rs, src/circuit/layouter.rs, src/utils/rational.rs, src/poly/mod.rs (batch_invert_rational 290-317), src/poly/kzg/msm.rs (msm_specific 166-175, msm_cpu 178-215)
  - Loki docs: /home/charles/repos/loki-proofs/LOKI-PROFILE.md (k16 phase table), LOKI-CPU.md (lines 165-183, region-closure replay decision)
  - midnight-circuits 7.2.4 (registry copy): src/field/foreign/params.rs 278-289, src/field/foreign/field_chip.rs 899-1475, src/ecc/foreign/weierstrass_chip.rs 797-1930, src/verifier/mod.rs 120-160 (fixed_bases), src/verifier/verifier_gadget.rs 332, src/verifier/msm.rs 149-156 and 481-490, src/verifier/expressions/mod.rs 49, src/hash/poseidon/poseidon_chip.rs and constants/mod.rs
  - Sherlock strawman: /home/charles/repos/projectfenrir/recursion/stagenet-strawman/verifier-patch/midnight-circuits (diff vs 7.2.4 in weierstrass_chip.rs), recl9/Cargo.toml, recl9/src/node.rs, recl9/src/inner.rs (verify_children_incircuit 227-256), recl9/logs/cost-node-arities.log, recl9/logs/node-keygen.log, recl9/logs/node-prove.log
  - midnight-zk-stdlib-2.3.5 src/lib.rs:1845 (FloorPlanner = SimpleFloorPlanner)
  - Web captures (saved): /home/charles/repos/Moriarty-factory/wiki-llm/loki/speedups/raw/witness-synthesis/ (index in README.txt there). Scroll halo2 floor planner: https://raw.githubusercontent.com/scroll-tech/halo2/develop/halo2_proofs/src/circuit/floor_planner/single_pass.rs (captured as scroll-single_pass.rs). halo2-base 0.5.4 docs: https://docs.rs/crate/halo2-base/0.5.4/source/src/lib.rs and https://docs.rs/crate/halo2-base/0.5.4/source/src/virtual_region/copper_chip.rs. midnight-zk PRs: https://github.com/midnightntwrk/midnight-zk/pull/543 and https://github.com/midnightntwrk/midnight-zk/pull/537 (gh pr view, both merged 2026-10-01).
---

## Question

Where does witness synthesis and circuit layout spend time in Loki's `create_proof`, how many times does each region closure run, what is allocated per phase, what do the foreign-field BLS12-381 chips and the in-circuit verifier cost in witness work, and can synthesis run in parallel or be cached across recursion levels, under the Track A rule?

## Prerequisite: the Sherlock parent is not proven by Loki today

`recursion/stagenet-strawman/recl9/Cargo.toml` pins stock `midnight-proofs = "=0.8.2"` from crates.io. Its `[patch.crates-io]` section repoints only `midnight-circuits` at the Fenrir-hardened copy in `verifier-patch/`. Loki (`midnight-proofs` 0.8.3 fork) is not in the parent's dependency graph. Every Loki hook below is therefore inert for the Sherlock parent until the strawman also patches `midnight-proofs` to `/home/charles/repos/loki-proofs`. That is row 0 of any plan, not a technique.

The prover loop is the same in the two registry copies: `src/plonk/prover.rs` is identical between midnight-proofs 0.8.2 and 0.8.3 in the crates.io registry. Loki's copy differs from 0.8.3 only by the added `loki-profile` guards. The conclusions about `parse_advices` and the floor planners transfer to the stock prover. Other files were not diffed.

## Measured facts for the parent shape (node, arity 4, k = 20)

From `recl9/logs/node-keygen.log` and `recl9/logs/cost-node-arities.log` (Oct 17 runs):

| Quantity | Value | Source |
|---|---:|---|
| Rows (keygen cost model) | 958,095 | node-keygen.log |
| Usable rows at k = 20 | 1,048,569 | cost log |
| Advice columns | 15 | cost log |
| Fixed columns | 45 | cost log |
| Lookups / permutations | 5 / 18 | cost log |
| Table rows | 1,032,465 | cost log |
| Keygen wall | 622.8 s, peak RSS 4.5 GB | node-keygen.log |
| Prove | pk 104.5 s, prove 867.3 s, 268% CPU, max RSS 14.99 GB | node-prove.log |

The brief's 220 s, 4-thread figure is not in these logs. The recl9 logs were written on a different host and user (paths under `/home/charl/...`, this machine is `/home/charles`), and the recorded prove ran at 268% CPU, so it was a contended run. Use 867 s as the recorded number for that host, and cite the 220 s run by host before using it.

Phase split of the parent's `create_proof` is **unmeasured**. Loki's `loki-profile` feature cannot run on the parent until the patch in the prerequisite is applied. The k16 `loki-profile` example in `LOKI-PROFILE.md` gives `witness_synthesis.phase_0` 23.98 ms and `phase_1` 13.40 ms out of 1,127 ms (3.3% together). That example is not a foreign-field circuit, so do not extrapolate it to the parent.

## 1. create_proof synthesis phases (src/plonk/prover.rs)

`create_proof` (397-447) calls `compute_trace` then `finalise_proof`. Inside `compute_trace` (57-265), in order:

1. Hash the VK into the transcript; `compute_instances` (line ~449).
2. `parse_advices` (517-665): the witness phase. Details in section 2.
3. Squeeze theta. `lookup.commit_permuted` per lookup (`lookup_prepare`).
4. Squeeze beta, gamma. `permutation.commit` per circuit (`permutation_products`).
5. `lookup.commit_product` per lookup (`lookup_products`).
6. Squeeze trash challenge; `trash.commit` (`trash`).
7. `vanishing::Argument::commit` (random h-blinding); squeeze y.
8. `lagrange_to_coeff` on every advice polynomial (lines 234-251).

Only step 2 is witness synthesis. Steps 3 to 8 are the transcript-ordered argument commitments, which other components cover.

## 2. Inside parse_advices (prover.rs:517-665)

For each phase in `pk.vk.cs.phases()`, and for each circuit, the code:

- builds `column_indices` for the phase (lines 541-551);
- allocates `witness.advice = vec![domain.empty_lagrange_rational(); meta.num_advice_columns]` (line 584). This is every advice column, not only the phase's own;
- calls `ConcreteCircuit::FloorPlanner::synthesize(&mut witness, circuit, config.clone(), constants)` (597-601);
- runs `batch_invert_rational` on the phase's columns (604-617). Its denominator inversions are parallel (`batch_invert_parallel`, `src/poly/mod.rs:301`), but the final per-cell map that assembles `F` values is serial;
- replaces unusable rows (`unusable_rows_start..`) with `F::random(rng)`, one cell at a time (621-630);
- commits each advice column with `CS::commit_lagrange` in a serial `map().collect()` (637-639), then writes each commitment to the transcript (643-645). `commit_lagrange` calls `msm_specific` (`src/poly/kzg/mod.rs:83-92`);
- after all circuits in the phase, squeezes the phase's challenges (667-672 region).

`configure` is also run at the top of every `parse_advices` call (lines 523-527). It fills a throwaway `ConstraintSystem` (`meta`) and the result is then discarded, because `meta = &pk.vk.cs` is used for everything else. The comment at 530 says selector optimisation cannot be applied here. The cost of that configure is unmeasured.

Per-cell cost in witness collection: `WitnessCollection::assign_advice` (919-948) returns early for columns of other phases (932) without calling the value closure. Cells are stored as `Rational<F>` (a 3-variant enum, `src/utils/rational.rs:12-20`) and converted with `batch_invert_rational`. Layout estimate: about 72 B per cell (64 B of field elements plus the tag). This is not measured.

Parallelism: nothing in `parse_advices` or the floor planners is parallel. Synthesis is serial per circuit. `rayon` is used in the permutation, lookup, poly, kzg `msm` and `evaluation` modules, not here.

## 3. Floor planners and the double run

Both planners run the region closure twice. The inner-body double run is what matters for cost.

- **SimpleFloorPlanner** (`single_pass.rs:27-37`): calls `circuit.synthesize` once. `SingleChipLayouter::assign_region` (81-147) first runs the closure against a `RegionShape` (lines 85-88) to learn the row count and columns, then chooses `region_start` from the global column-usage map and runs the closure again for real. Value closures are skipped in the shape pass (`RegionShape` already skips them, per `LOKI-CPU.md:172-183`). Only non-closure work inside a region body runs twice. Whether the parent has such work is unresolved; see section 10.
- **V1** (`v1.rs:61-100`): calls `circuit.without_witnesses().synthesize` for measurement (shape pass over regions), then `circuit.synthesize` again for assignment. So region bodies run twice and the whole top-level body runs twice, one of those on the witness-free clone. This is more work than SimpleFloorPlanner for the same circuit, with a better row packing.
- **ZkStdLib** (midnight-zk-stdlib-2.3.5 `src/lib.rs:1845`) uses SimpleFloorPlanner. The parent therefore runs region bodies twice.

Phase count for the parent: 1. `phases()` (`plonk/circuit.rs:2317`) yields one phase unless some advice column is in a later phase. midnight-circuits never uses challenges: `verifier/expressions/mod.rs:49` panics on `Expression::Challenge` ("We do not suport multi-phase yet"), and `verifier_gadget.rs:332` asserts `cs.phases().count() == 1` for the verified circuit. ZkStdLib has no phase references. So the parent's synthesis runs once per proof. The closure multiplier is 2 (shape plus assign), not 2 times phases.

Upstream caveat: midnight-zk PR #543, "Move advice columns to phase-1 group" (merged 2026-10-01, https://github.com/midnightntwrk/midnight-zk/pull/543), moves the logup multiplicities and advice columns into a phase-1 commitment group, and PR #537 (https://github.com/midnightntwrk/midnight-zk/pull/537) moves the permutation argument into the generic phase groups. Newer stock midnight-zk therefore has phase structure that 0.8.2/0.8.3 do not. Any upgrade of the parent's proof system needs this section re-checked, because phase count multiplies the closure runs and the per-phase allocation.

## 4. Advice column allocation per phase

For a single-phase circuit, `column_indices` is every advice column, so the full-width `Rational` allocation (line 584) has no waste. The waste exists only for multi-phase circuits, and the parent is single-phase. For the parent, the structural numbers are: 15 columns times 2^20 cells times about 72 B is about 1.13 GB of transient `Rational` storage, then about 0.50 GB of `F` output after `batch_invert_rational` (the `Rational` vectors are consumed there). Peak is about 1.6 GB out of 15 GB RSS, roughly 10%. This is a memory lever, not a time lever, and it is unmeasured.

## 5. midnight-circuits chips in the parent

**Foreign-field BLS12-381 base field.** `src/field/foreign/params.rs:278-289`: `LOG2_BASE = 56`, `NB_LIMBS = 7` (7 x 56 = 392 bits, enough for the 381-bit base), `RC_LIMB_SIZE = 15`, moduli `2^134` and `2^134 - 1`. Each foreign multiplication (`field_chip.rs:899-926` then `assign_mul` 1414-1475):

- normalises both operands if not well formed (`normalize`, 1568);
- computes the quotient-like witness `z = x*y` (or `x/y` as `z*y = x`) with `num-bigint` conversions: `bi_to_limbs` at line 1439, then `bigint_to_fe` for 7 limbs (1434-1436);
- range-checks 7 limbs with `assign_lower_than_fixed` (1446-1456);
- calls `mul::assert_mul` (`gates/mul.rs:206`), whose range-check region is `assign_region` (line 220).

The `assign_mul` value computation (1430-1439) is at chip level, outside any region closure, so it runs once. The `assign_region` body in `assert_mul` is where a double run would hit, if its closure does non-value work. Unresolved.

**Foreign-field EC (`ForeignWeierstrassEccChip`, `weierstrass_chip.rs:143`).**

- `msm_by_bounded_scalars` (810-878): sets `WS = 4`; uses GLV splitting (`glv_split`, 1958) for scalars with bound above `NUM_BITS/2 + WS` when `C::has_cubic_endomorphism()`; then `windowed_msm::<4>`.
- `windowed_msm` (1771-1925): for every base, builds a 16-entry table `[-α, p-α, ..., 15p-α]` with 15 `incomplete_add` calls (1807-1826) and loads it with `load_multi_select_table`. The main loop runs `max_len` windows, each doing 4 `double`s, then per base one `multi_select` and one `incomplete_add`.
- Structural cost with `l` bases and `W` windows: about `15 l` adds for tables, plus `W (4 D + l (S + A))` for the loop, where D, A, S are a foreign double, add and table select. Each D or A is several foreign multiplications plus range checks. With GLV, `W` is about half of `max_len`.
- Witness values for every `incomplete_add` are computed eagerly in the chip (`r_curve` at line 1144) and assigned with `assign_point_unchecked`.

**Caching in the chip.** `msm_randomness` (1726-1748) caches `r` and `neg_alpha` per window size, per chip instance. The tables are **not** cached: they are rebuilt on every MSM call (1807-1826).

**Fixed bases.** `fixed_bases` (`src/verifier/mod.rs:120-140`) returns `-G`, one base per fixed-column commitment, and one per permutation commitment. For the leaf VK of the parent, the cost log gives 45 fixed columns and 18 permutations. Taking those as 45 fixed-column commitments and 18 permutation commitments (assumption: the log's "permutations" field counts commitments), the leaf VK contributes 63 constant bases plus `-G`. `verify_children_incircuit` (`recl9/src/inner.rs:243-247`) assigns them once, and `resolve_fixed_bases` (`verifier/msm.rs:481-490`) merges them into the single collapsed MSM. Their 15-add tables are rebuilt in-circuit on every parent proof, although the points are constants. That is `63 x 15 = 945` foreign adds of constant table entries per parent proof, under the stated assumption. The loop cost (per-window select and add) stays, because the scalars are still variable. Against the variable-base table cost, which depends on the number of child commitments and is not extracted, this is a substantial share but not all of the table work.

**Patched copy.** `verifier-patch/midnight-circuits` differs from 7.2.4 only in `weierstrass_chip.rs` (per `fenrir-msm-shift.patch`):

- In `mul_by_u128` (the double-and-add used for `l*r` and `alpha`), for curves with cofactor > 1 (BLS G1 has cofactor > 1), it normalises `p.x` and adds `incomplete_assert_different_x` before each addition (about lines 1668-1726). Few such calls per MSM, so cost is small.
- The random shift `r` is assigned with the checked `AssignmentInstructions::assign` instead of `assign_without_subgroup_check` (about 1758-1770). This runs once per chip because of the cache.

**Verifier gadget (`src/verifier/verifier_gadget.rs`, `verifier/traces.rs`).** `parse_trace` (296), `verify_algebraic_constraints` (401) and `prepare` (713) are per child proof. `recl9/src/inner.rs::verify_children_incircuit` (227-256) does the following:

- assigns the leaf VK once (`assign_fixed_vk`, line 242);
- assigns the fixed bases once and shares them across children (243-247);
- calls `verifier.prepare` per child, which parses the child's `Value<Vec<u8>>` proof and replays the transcript as witness values;
- calls `verifier.accumulate` (random linear combination of children's MSMs, `accumulator.rs:206`), then `resolve_fixed_bases`, then one `collapse`. So all children and the fixed bases finish in a single MSM.

Witness work per parent proof therefore has four parts: proof-byte parsing, Poseidon transcript replay (CPU values), the MSM witness (tables and loop), and the foreign mul and range-check witnesses. There is no cache of parsed child proofs or transcript values across synthesis calls. With one synthesis per proof, that is not a loss, but nothing carries over between two proofs of the same leaf either.

**Poseidon (`src/hash/poseidon/poseidon_chip.rs`, `constants/mod.rs`).** `WIDTH = 3`, `RATE = 2`, 8 full rounds, 60 partial rounds. Partial rounds are batched six at a time through `NB_SKIPS_CIRCUIT = 5` (`round_skips.rs`), so the permutation is 10 partial-round gates plus 8 full-round gates, in one region. Witness values come from `partial_round_cpu_for_circuits` (line ~386). This is already optimised. A Poseidon2 swap would change the transcript hash and break Track A, so it is not in scope.

## 6. Parallel and multithreaded synthesis

- **Loki today:** synthesis is serial. The `commit_lagrange` loop and the blinding loop are serial. Only the batch inverse denominators are parallel.
- **MSM is already internally parallel.** `msm_cpu` (`src/poly/kzg/msm.rs:191-215`) splits BLS12-381 G1 MSMs into `rayon::current_num_threads()` chunks with `par_chunks` and runs blst `multi_exp` per chunk on the caller's Rayon pool. The comment notes that nested commitments share the same worker budget. So an outer rayon loop over the 15 advice commitments would mostly contend with the inner one and buy little. The `cuda` feature can route `msm_specific` to the GPU (`msm.rs:170-173`), which is a separate path.
- **Region-parallel assignment, with a published reference.** Scroll's halo2 fork gates `assign_regions` behind a `parallel_syn` feature in `halo2_proofs/src/circuit/floor_planner/single_pass.rs`, at about line 209 (captured as `raw/witness-synthesis/scroll-single_pass.rs`, from https://raw.githubusercontent.com/scroll-tech/halo2/develop/halo2_proofs/src/circuit/floor_planner/single_pass.rs). The method: a sequential shape pass per sub-region (`RegionShape`) to fix each region's row range and update the column-usage map in order; `cs.fork(&ranges)` to create one sub-constraint-system per range; `into_par_iter` over the closures with a sub-layouter each; `cs.merge(sub_cs)` in order; constants assigned sequentially afterwards. The fetched capture shows the mechanism but no timings, so the speed-up is "no published measurement found" in this pass. The design maps directly onto Loki's `SingleChipLayouter` (`single_pass.rs:81-147`).
- **Multi-threaded witness generation, a second design.** halo2-base 0.5.4 (Axiom, a halo2 eDSL layer) describes "built-in circuit tuning and support for multi-threaded witness generation" in its crate description (captured as `raw/witness-synthesis/halo2-base-lib.rs`, from https://docs.rs/crate/halo2-base/0.5.4/source/src/lib.rs). Its `Context` is documented as "a single thread of an execution trace" and CPU thread-local (captured as `halo2-base-thread.rs`, from https://docs.rs/crate/halo2-base/0.5.4/source/src/virtual_region/copper_chip.rs). Witness is built in virtual regions and mapped to physical rows at the end. This is context for the design, not a Loki hook. No numbers were found, and `parallelize_in` was not located (the docs.rs page returned 404).
- **Constraint on any parallel synthesis:** `blinding` uses `rng` in cell order, and the transcript receives commitments in circuit and column order. The byte-reproducibility test in Loki (`LOKI-CPU.md`) therefore constrains the draw order. Any parallel version has to keep draws and transcript writes in the serial order, by computing commitments in parallel and writing them afterwards.
- **Across circuits:** `parse_advices` loops over `circuits` serially (prover.rs:574). The parent has one circuit per proof, so this helps nothing for k20. It helps only multi-circuit `create_proof` calls, which is relevant to the 16-transaction batch throughput model in `10x/LOKI-10X.md`, not to the single-parent latency.
- **Across proofs:** outside `create_proof`, and covered by the scheduler component.

## 7. Witness caching across recursion levels

- Within a proof: `msm_randomness` (per chip, per WS), fixed VK and fixed-base assignment (shared across children), and the single collapsed MSM. These are already in place.
- Not cached: MSM tables (rebuilt per call), parsed child proof values, and the transcript replay, which are all per proof.
- Across proofs of the same key: `configure` is re-run per proof (prover.rs:523-527), and `pk.fixed_values` are re-used. Layout and fixed constants could be cached in the prover (Track A) if their values depend only on the VK. The in-circuit fixed-base tables are constants that could be cached in the circuit (Track B, needs new keys).
- `LOKI-CPU.md:172-183` deliberately keeps region-closure replay. Its reasoning holds: a closure may carry cross-region copies, instance queries and side effects, so skipping the shape pass cannot generally preserve stock semantics. The safe lever is moving expensive witness computation into `Value` suppliers, which the shape pass already skips.

## 8. Ranked table (top 5, gain divided by effort)

Track A = prover-only or circuit-side with identical proof bytes and VK. Track B = new keys or VK. Gains are unmeasured for the parent unless marked. "No published measurement" means exactly that: a source was located but gives no number.

| Rank | Technique | Track | Hook | Effort | Expected gain on k20 parent | Source | Correctness risk vs Lean contract |
|---|---|---|---|---|---|---|---|
| 0 | Measure first: patch `midnight-proofs` to loki-proofs in recl9, run `loki-profile`, and count closure invocations (atomic counter in the region body) | prerequisite | `recl9/Cargo.toml` `[patch.crates-io]`; `examples/loki-profile.rs` | S | Decides the rest. Converts the 3.3% k16 figure into a parent number | Loki measurement (LOKI-PROFILE.md) | none |
| 1 | Region-parallel `SingleChipLayouter::assign_regions` (Scroll-style: sequential shape pass, fork CS per region, parallel second pass, ordered merge) | A | `loki-proofs/src/circuit/floor_planner/single_pass.rs:81-147`; `prover.rs:597` | M-L | Bounded by the share of region-body work, which is unknown (the shape pass is not the cost). No published measurement found | https://raw.githubusercontent.com/scroll-tech/halo2/develop/halo2_proofs/src/circuit/floor_planner/single_pass.rs (captured) | medium-high: region order, rng draws and transcript order must match the serial loop byte for byte |
| 2 | Pre-compute fixed-base tables as constant assignments (or fixed columns) in the in-circuit MSM, instead of rebuilding 15 `incomplete_add`s per fixed base per parent proof | B | `midnight-circuits/src/ecc/foreign/weierstrass_chip.rs:1807-1826`; `verifier/msm.rs:481-490` | L | Removes about 945 constant foreign adds per parent proof under the 63-fixed-base assumption (45 fixed columns + 18 permutations); loop cost unchanged. Not timed; no published measurement | Structural, from midnight-circuits source and recl9 cost log | high: new VK and keys; soundness of constant tables must be re-audited. Track B label required |
| 3 | Cache the throwaway `configure` result per `ProvingKey` (or store `meta` config alongside `pk`) | A | `prover.rs:523-527` | S | Small, unmeasured. Removes one full configure per proof | Structural (prover.rs comment at 530) | none, if the cached config is identical to a fresh configure |
| 4 | Drop the per-column `Rational` allocation for single-phase circuits (allocate `F` values directly when the cell is trivial, or reuse the buffer across phases) | A | `prover.rs:584`, `batch_invert_rational` in `src/poly/mod.rs:290` | S-M | Memory, not time: about 1.1 GB transient out of about 15 GB RSS (about 10%). Unmeasured | Structural (section 4) | none if values are unchanged |

Demoted, not ranked: outer rayon over the 15 advice commitments. `msm_cpu` already runs BLS G1 MSMs with rayon chunks (`msm.rs:191-215`), so the outer loop would mostly contend. Only revisit if profiling shows idle cores during `advice_commitments`.

Items considered and not ranked: removing the shape pass (unsafe per `LOKI-CPU.md`); Poseidon2 (changes the transcript hash, Track-incompatible); cross-circuit parallel synthesis (helps multi-circuit batches, not the single parent); the halo2-base virtual-region design (context only, no numbers).

## 9. What would have to be true for each to pay off

- **Row 1 pays off** if the region-body work (not the shape pass) is a large share of synthesis on the parent, and the ordered merge keeps transcript and rng draws identical. Per-region work must be big enough to amortise the fork and merge.
- **Row 2 pays off** if the 945 constant table adds are a meaningful share of the parent's witness and constraint work. It is the strongest circuit-side lever in this component, but it needs keys and a new audit.
- **Row 3 pays off** only if `configure` is expensive for the ZkStdLib constraint system. Measure it first.
- **Row 4 pays off** only if memory, not time, is the binding constraint, for example if the 16 GB RSS has to come down to fit two heavy proofs.

## 10. Open questions and gaps

- Synthesis wall time for the parent: unmeasured. The 220 s figure is from a different host and user; cite that run before comparing.
- Whether the parent's region bodies contain non-value work that runs twice: unresolved. Needs the counter in row 0.
- Foreign-field mul witness vs range-check witness split: unmeasured.
- The fixed-base count (63 assumed) is derived from the cost log, not from the VK. Confirm by counting `fixed_commitments` and `permutation().commitments()` on the actual leaf VK.
- midnight-zk upstream (PRs #537, #543) moved advice and permutation columns into phase groups after 0.8.3. Phase count for any future parent upgrade must be re-read.
- Not located: scroll-tech `parallel_syn` benchmarks; PSE halo2 synthesis issues; `halo2-base` `parallelize_in` (docs.rs 404).
