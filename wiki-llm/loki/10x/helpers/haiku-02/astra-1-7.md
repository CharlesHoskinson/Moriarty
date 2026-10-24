# astra-1-7: Same-key witness compiler feasibility (Sherlock VK)

## Question (astra-1)
Can a symbolic tape or static region layout plus custom native/foreign witness kernels fill the EXACT same advice cells for the fixed Sherlock VK? What blocks it (witness-dependent layout, Rust closures/RefCells, conditional inverse zero cases, curve ladders)? What is the deepest serial dependency? Does retained CPU witness already fit the budget of about 37 s at 882 s (4.25% floor)? Give exact code hooks and a minimal experiment. No heavy runs. Follows `astra-1-2.md` and `astra-1-2-followup-witness.md` in this directory.

Path abbreviations: `P` = midnight-proofs-0.8.3, `C` = midnight-circuits-7.2.4, `S` = midnight-zk-stdlib-2.3.5 (all under `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`); `R` = recl9 `src/` (`/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src`).

## Answer (short)
- **Structure can be static.** The Sherlock link circuit's structure is fixed by the relation offsets, the children's VKs and the constants. Its code branches on witness values only in error checks (`error_if_known_and`, and `if b.len() != n` on an offset-derived length). A one-time structural pass can record regions, columns, copies and selectors, and a separate witness pass can fill advice values from a recipe. The recipe needs typed instrumentation of the chip/field operations (a plain `Assignment` sees only layout and already-evaluated values).
- **Exact equality is not achievable as stated, even on CPU.** The MSM randomness point is drawn from `OsRng` when each ECC chip is constructed (`C/src/ecc/foreign/weierstrass_chip.rs:994`), and `ZkStdLib::new` builds that chip on every synthesize pass (`S/src/lib.rs:408-418`). Advice cells carrying that point (`r`, `-alpha`, and the table entries built from them) therefore differ run to run. Blinding rows are RNG-filled by design (`P/src/plonk/prover.rs:557-559`). The valid equality target is: identical cells under an injected test-only random point and RNG stream, plus constraint satisfaction and independent acceptance under production randomness.
- **Value-level equality is feasible, with listed conventions.** Each non-blinded advice cell is a deterministic function of (witness, injected random point, proof bytes). Exact reproduction requires the same inverse-zero defaults, the same GLV decomposition algorithm and the same limb conventions (section 2).
- **Deepest serial dependency is about 10^3 to 10^4 group or hash operations, not 10^6.** The critical path runs: child transcript sponge (sequential Poseidon permutations) → Fiat-Shamir scalars → accumulated windowed MSM (sequential accumulator) → collapse → public-input constraints (section 4). Parallelization needs scans and fused kernels, not a depth change.
- **Budget: undecided.** No measured synthesis (witness) time exists. The 5.687 s `model` phase is structural and excludes witness closures (see `astra-1-2-followup-witness.md`, section A). Whether retained CPU witness fits inside the 37.16 s unchanged-work cap (f ≤ 4.2105% of T0) cannot be decided from current data. Section 6 gives the minimal measurement that decides it.

## 1. Structure vs. witness: what is static
- The Sherlock link circuit (`R/sherlock_link.rs:444-530`, `impl Relation::circuit`) computes its shape from `self.offsets()`, the VKs and the fixed bases. Witness enters only through `w.as_ref().map(...)` and `Value` closures, except:
  - `error_if_known_and` on instance lengths (`sherlock_link.rs` near line 452: `(x,y) != (n,n)`), which errors but does not change structure;
  - `if b.len() != n` (near line 456), on an offset-derived length.
- Child verification: `verifier.prepare(...)` takes `proof: Value<Vec<u8>>` (`sherlock_link.rs` near 476-478; `C/src/verifier/verifier_gadget.rs:713-719`). Transcript reads follow the VK-defined count: `C/src/verifier/transcript_gadget.rs:69-86` ("The amount of points read from the proof is fixed for a given Architecture"). Malformed proofs yield default points (`read_point`, `:130-147`), which keeps structure fixed.
- Phases: `C/src/verifier/verifier_gadget.rs:16` says children have `vk.cs.num_challenges = 1` and empty `phases()`. The parent's own phases are not stated in the sources read. `P/src/plonk/circuit.rs:2317-2321` returns phase 0 even when no advice phase exists, so the parent is probably single-phase: two synthesis passes per `parse_advices` (shape and assign; `single_pass.rs:93` and `:111-114`). This is an inference, not measured.
- Order-dependent structural side effects that a tape must preserve (not witness-dependent):
  - `tag_cnt` (`Rc<RefCell<u64>>`, `weierstrass_chip.rs:160, 1004`; incremented at `:1517-1519` and `:1835-1836`) assigns table tags `F::from(tag_cnt + i)` (`:1519`, `:1871`, `:1886`). Tags are fixed-column content, so they are part of the VK. Reordering table loads changes the VK.
  - `used_*` flags (`S/src/lib.rs:366-375`, set during synthesis) decide which tables are loaded at the end (`MidnightCircuit::synthesize`, `S/src/lib.rs:1868-1900`). These are relation-derived but must match keygen.
  - `msm_randomness` cache (`RefCell`; `weierstrass_chip.rs:1726-1751`): the first call samples `r`, and later window sizes reuse it. Order matters.

## 2. Blockers, ranked
1. **Non-reproducible random point** (`weierstrass_chip.rs:994`, `OsRng`; chip rebuilt per pass at `S/src/lib.rs:408-418`). Blocks exact-cell equality. Hook: inject a test-only point in a scratch copy of `C`, never in the repos.
2. **Order-dependent RefCell/Rc state** (`tag_cnt`, `msm_randomness`, `used_*`). A tape must record structure from one sequential pass, then fill values. Any change in call sequence changes tags and therefore the VK.
3. **Inverse zero conventions.** Advice values for zero denominators are defined by `unwrap_or`:
   - `C/src/field/native/native_chip.rs:377` (`assign_with_shifted_inverse`: `unwrap_or(F::ZERO)`);
   - `:775` (`assert_not_equal`: `unwrap_or(F::ZERO)`);
   - `:1253` and `:1298` (equality auxiliary: `unwrap_or(F::ONE)`).
   A batched inverse must special-case zero: zero-preserving batch inversion gives 0 for the first two sites, but the equality auxiliary must yield ONE. Node 09 already flags this (`loki-09`, field arithmetic section).
4. **Curve ladders with exact algorithms.** GLV decomposition (`C/src/utils/util.rs:110-`, BigInt xgcd with per-call Vec allocation) must be ported exactly to reproduce `(s1,s2)` cells; a different valid decomposition gives different cells that still verify. The windowed MSM (`weierstrass_chip.rs:1862-1909`) is a fixed sequential accumulator; a parallel reduction produces different intermediate cells (valid, not identical) unless it reproduces the same prefix values.
5. **Foreign-field limb conventions.** Limb decomposition, bounds and wrap-around for negative lower bounds (`C/src/field/foreign/field_chip.rs:61-64`, `:430-434`; `C/src/biguint/biguint_gadget.rs:745-751`) define the exact cells. Porting requires the same base and bound conventions.
6. **Eager proof decoding during synthesis.** `init_with_proof` extends `proof_bytes` from the `Value` (`transcript_gadget.rs:69-86`), and `read_point` decodes eagerly (`:130-147`), so G1 decompression (a square root per point, in blst) runs on the host during synthesis. Small but unavoidable CPU cost in any witness path that reads proofs.
7. **Error paths are witness-dependent.** `completeness_error_if` (`weierstrass_chip.rs:1012`, used at `:1900`) and `error_if_known_and` raise errors when values collide or sizes mismatch. A device path must detect and raise the same errors; a silent kernel producing a wrong cell is a soundness-relevant divergence.
8. **Blinding rows** come from the transcript RNG (`P/src/plonk/prover.rs:557-559`) and differ every proof by design. "Exact cells" can only cover non-blinded rows.

## 3. What a tape can and cannot reproduce
- Can (given the injected random point and the same algorithms): every non-blinded advice cell whose value is a deterministic function of inputs, including foreign-field limb values, GLV splits, MSM tables and accumulator prefixes, and native inverses with the conventions above.
- Cannot, in production: byte-identical advice across runs (random point, blinding). Two CPU runs already differ. Acceptance must therefore be "same cells under a fixed test RNG" plus "valid under fresh randomness, accepted by the unchanged verifier". This matches astra-1's correction on lookup ordering, which depends on `HashMap` iteration order (`P/src/plonk/lookup/prover.rs:399`, `:430`).

## 4. Deepest serial dependency (witness-only critical path)
Using the verifier structure from `C` and the relation from `R`:
1. Child transcript sponge: sequential Poseidon permutations. Depth ≈ (permutations) × 68 round-steps (8 full + 60 partial; node 06 CODE claim, not re-verified). Permutation count is ESTIMATE: about 15 field elements per absorbed G1 point (`C/src/ecc/foreign/weierstrass_chip.rs:235-247`) at rate 2, times the child's point and scalar counts (not recorded). The two children run as parallel chains.
2. Fiat-Shamir scalars depend on (1).
3. Accumulated windowed MSM over both children's bases (`verifier.accumulate`, `sherlock_link.rs`, near line 480): depth ≈ 32 × (4 + 2B) sequential group operations for B total bases with GLV (`weierstrass_chip.rs:1875-1900`; GLV condition at `:850`). B is not recorded; B = 100 gives about 6.5k operations (ESTIMATE).
4. `acc.collapse` and public-input constraints (`sherlock_link.rs`, near lines 520-528): short.
- The critical path is about 10^3 to 10^4 operations, mostly in (1) and (3). Each group operation needs one Fp inversion for lambda (`weierstrass_chip.rs:1182` in `assert_double`, `:1266` in `assert_add`).
- Interpretation: depth is far below 10^6. A parallel witness DAG is blocked by implementation (closure order, RefCell state, sequential accumulator), not by depth. A GPU kernel per operation would pay roughly 10^4 launches per proof chain; fused kernels or scans remove that.

## 5. Budget check (arithmetic only, astra-1's formula)
- T0 = 882.590 s (`D/samples/parent-proof.json`, prove interval including PK drop).
- Target 10x: total ≤ 88.26 s.
- Exact cap, following astra-1's accounting: 10x needs S = 1/[f + (1−f)/20 + 0.01] ≥ 10, which gives f ≤ 0.04/0.95 = 4.2105%. Applied to T0 = 882.590 s, the cap on ALL unchanged work (witness CPU included, plus any other unchanged tail) is 37.16 s. It is not a witness-only budget; the 37.5 s I wrote before was an approximation with a different split and is withdrawn.
- Known: the prove interval contains two synthesis passes (if single-phase) plus all cryptography. Not known: the witness-closure share of synthesis. The `model` phase (5.687 s) is structure-only and may consist of two DevAssembly passes (`automatic_fitting_model`, `sherlock_link.rs:982-1006` in `docs/results/sherlock-native-source44-20261022/source`), which would be about 2.8 s per structural pass. That is an inference from the probing logic, not a log.
- Interpretation: the 37.16 s cap is shared by everything left unchanged. If the parent has two synthesis passes and the cap is spent on witness alone, that is about 18.6 s per pass. Present data cannot decide this, and the cap must also cover any unchanged host tail. Treat as open.

## 6. Minimal experiment (no proving, no GPU, no repo edits)
Goal: measure retained synthesis cost per pass on the Sherlock parent witness, with commitments and quotient excluded.
1. In a scratch copy of `P` (outside the repo), add `Instant` timers around `ConcreteCircuit::FloorPlanner::synthesize` in `parse_advices` (`P/src/plonk/prover.rs:533-539`) and around the shape and assign passes (`single_pass.rs:93`, `:111-114`). Return early after `parse_advices`, before `batch_invert_rational` (`:540`), so no commitments run.
2. Run once with the parent witness (the child proofs and instance used by `prove_link_parent`, `R/sherlock_link.rs:1020-1050`). Record per-pass wall time, the count of `assign_advice` calls, and the count of `Value` closures actually evaluated (a counter in `WitnessCollection::assign_advice`, `prover.rs:845-872`).
3. Separately count structure with a counting `Assignment` that ignores values, as `DevAssembly` does (`P/src/dev/cost_model.rs:526-560`). This gives structure-only time. The difference between (2) and (3) is the witness-closure cost.
4. Repeat with the random point injected (scratch `C` copy) to confirm the advice cells are deterministic across two runs; then compare cell by cell against a second implementation if one exists.
Expected output: per-pass synthesis time with and without closures, the number of closure evaluations, and whether a second run reproduces cells under the injected point. This decides whether a CPU-only witness path can stay inside the 37.16 s unchanged-work cap (with any other unchanged tail subtracted).

## 7. Eager Value host-control boundary (where the host must look at witness values)
Synthesis is a host program. Its control flow depends on witness values in a few places, and the rest is value-only data flow. The boundary below is what a specialized recipe must reproduce exactly, or hand back to the host.

| # | Site | Kind | Effect | Device-movable? |
|---|---|---|---|---|
| A1 | `C/src/ecc/foreign/weierstrass_chip.rs:1012-1013` `completeness_error_if` (uses at `:1900` in MSM accumulate, and in table build) | error on witness predicate (acc == ±addend) | returns `CompletenessFailure`; no structure change | Yes, as a predicate flag per op, reduced to one host error |
| A2 | `weierstrass_chip.rs:1536` multi_select index-order check (`error_if_known_and`, i >= j) | error on witness predicate | fails synthesis; no structure change | Yes, flag |
| A3 | `C/src/ecc/native/edwards_chip.rs:1221` (s is zero) and `C/src/field/foreign/field_chip.rs:1427` (division by zero) | error on witness predicate | fails synthesis | Yes, flag |
| A4 | `sherlock_link.rs` near 452 (`error_if_known_and` on instance lengths) | error on instance shape | fails synthesis | Host (shape, cheap) |
| B1 | `C/src/ecc/foreign/weierstrass_chip.rs:690`, `C/src/ecc/foreign/edwards_chip.rs:814` `if b {p} else {q}` in `select` | value select | data only | Yes |
| B2 | `weierstrass_chip.rs:1520-1530` multi_select `position` search, `unwrap_or(0)` | value-dependent index | index cell values only | Yes (a comparison-reduction) |
| C1 | `transcript_gadget.rs:69-86` `init_with_proof` (bytes extended from the Value) and `:130-147` `read_point` (`Ok` vs `Err` chooses decoded point or `default`) | eager host decode of witness bytes | structure fixed (VK-defined count); value of default point differs | Decoding is host; the `Ok`/`Err` select is data-only once decoded |
| C2 | point decompression in the decode step (`MC/src/bls12_381/g1.rs` `from_bytes`; square root inferred, not traced into blst) | host arithmetic | CPU cost only | Host (or device sqrt) |
| D1 | `C/src/field/native/native_gadget.rs:281, 366, 791, 1172-1177, 1351` `constrained_cells` cache (`HashMap<AssignedNative, BigUint>`, `:76`) | structural cache | may skip emitting a bound check, changing rows | Structural (keyed by cell identity, `P/src/circuit/mod.rs:122-135`, not by value), but history-dependent: must replay the same sequence |
| E1 | `C/src/ecc/foreign/weierstrass_chip.rs:1845-1856` / `:1875-1909` / `:850` GLV branch | structural (bounds `s.1`), not value | structure fixed | Structure |

Reading: no branch of the circuit's structure reads a witness value. Structure depends on bounds (`s.1`), offsets, VK and an order-dependent cache (D1). Value-dependent host control is limited to error predicates (A1-A4) and to the decode step C1. Everything else (B1, B2, the closure arithmetic) is value-only data flow, which a recipe can carry as a DAG. So the eager host boundary is narrow: decoding plus error predicates, not control of layout.

## 8. Specialized witness recipe (proposed design, same-key)
Design:
1. **Record once per (VK, relation, configuration).** CORRECTION: a recording `Assignment` alone cannot extract the op DAG. `assign_advice` receives the closure result already evaluated (`P/src/plonk/prover.rs:845-872`, `:871`), so only layout is visible. The recipe needs typed instrumentation: a generic `RecipeValue` in place of `F` in the chip/field operations (`assign`, `invert`, `mul`, GLV, limb split, multi_select), recording op kind and input ids. Structure as in `P/src/dev/cost_model.rs:526-560`, plus, for each advice cell, a recipe node: op kind, input cell ids, constants, and the random-point input as a named source). Record copies, errors (A1-A4 predicates as nodes), and the D1 cache decisions. This is a one-time structural pass per key, which fits the same-key rule and is outside the per-proof budget.
2. **Make the random point an input.** Replace the OsRng draw (`weierstrass_chip.rs:994`) with a recipe input supplied per proof (production: fresh; test: fixed). This is the only way the advice cells become reproducible across runs.
3. **Evaluate per proof.** A topological evaluation of the recipe DAG, one level at a time. The number of levels is the dependency depth from section 4 of `astra-1-7`, about 10^3 to 10^4 (ESTIMATE), each level data-parallel across cells. Inversions use batch inversion with the zero conventions (ZERO at `native_chip.rs:377`, `:775`; ONE at `:1253`, `:1298`). Lookups (`pow2range`) and limb splits are recipe ops over integers, not BigUint closures.
4. **Reproduce errors as flags.** Each A-site predicate is a boolean node; any true flag causes the same host error as today.
5. **Transcript stays on host.** The Poseidon sponge (C1, section 4 of `astra-1-7`) is a sequential chain of its own; it can be run on host, with challenges feeding the MSM scalars that are computed on device.

What the recipe removes: the closure machinery (RefCell/Rc state, BigUint allocations per limb, BigInt xgcd per scalar, the `position` search with `table_values.clone()`), each of which is per-call host overhead. What it does not remove: the dependency depth (about 10^3 to 10^4 levels, so a level-per-launch GPU design needs fusion), the transcript chain, and the requirement that the recipe reproduces the closure semantics exactly.

Blockers that remain: (i) recipe extraction must not change the call order that defines tags and cache decisions (section 2 of `astra-1-7`); (ii) the GLV and limb algorithms must be ported exactly for exact-cell equality; (iii) blinding rows stay RNG-filled and differ per proof.

## 9. Minimal experiment for the recipe (no proving, no GPU)
1. Recording pass: a scratch copy of the chip/field layer generic over a `RecipeValue` (not in the repo; a plain `Assignment` records layout only, see section 8.1) that records the recipe for the Sherlock parent relation at the fixed VK, with the random point as an input. Output: node count, levels (critical path), counts per op kind, error-node count.
2. CPU interpreter: a single-threaded evaluator of the recipe for one witness. Time it; compare every non-blinded advice cell to the reference `WitnessCollection` values under the same injected random point (the reference is the scratch copy of `P` from section 6, step 1).
3. Parallel check: run the same evaluator with rayon per level; record speedup vs. levels to see whether the depth or the arithmetic dominates.
Decision rule: if (2) is well below the 37.16 s unchanged-work cap per proof (after other unchanged tail is subtracted) and cell equality holds, the CPU recipe path already meets the witness budget; if not, measure (3) to see whether level parallelism closes the gap.

## Raw numbers table

| Item | Value | Status | Source |
|---|---|---|---|
| T0 (parent prove interval) | 882.590 s | MEASURED | `D/samples/parent-proof.json` |
| Unchanged-work cap at 10x, 20x bulk, 1% overhead | f ≤ 4.2105%, i.e. 37.16 s of T0 for all unchanged work incl. witness | arithmetic | this file |
| Model (structure) phase | 5.687 s | MEASURED; structure only | `D/samples/parent-proof.json` |
| Structural passes in model phase | probably 2 | ESTIMATE from `automatic_fitting_model` | `sherlock_link.rs:982-1006` (source44) |
| MSM window count (GLV) | 32 | CODE | `weierstrass_chip.rs:1875`, `:850` |
| MSM sequential ops | 32 × (4 + 2B) | ESTIMATE; B unknown | `:1875-1909` |
| Poseidon rounds per permutation | 68 | node 06 CODE claim | `loki-06` |
| Child verifier challenges | 1; phases empty | CODE | `C/src/verifier/verifier_gadget.rs:16` |
| Random point source | OsRng per chip | CODE | `weierstrass_chip.rs:994`; `S/src/lib.rs:408-418` |
| Inverse-zero defaults | ZERO at 377, 775; ONE at 1253, 1298 | CODE | `C/src/field/native/native_chip.rs` |
| Blinding RNG | serial `F::random` | CODE | `P/src/plonk/prover.rs:557-559` |

## Caveats
- No synthesis timing exists for any proof. All budget statements are conditional on the unmeasured witness-closure share.
- The structural pass count, the parent's phase count, B (bases), and child point and permutation counts are not recorded. Depth numbers are formulas with ESTIMATE inputs.
- The random-point finding comes from `C/src/ecc/foreign/weierstrass_chip.rs:994` and chip construction in `S/src/lib.rs:408-418`. I read `ZkStdLib::new` but did not trace every call path that rebuilds the chip.
- That `verifier.accumulate` merges both children into one MSM is taken from the comment and call structure in `sherlock_link.rs`; the exact base count was not traced.
- No builds, proofs, benchmarks or repo changes were made. Only local source, logs and result docs were read.
