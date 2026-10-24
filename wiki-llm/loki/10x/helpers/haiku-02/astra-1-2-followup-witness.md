# astra-1-2 follow-up: cost_model vs real witness, expensive witness closures, dependency span

## Question (from astra-1)
Audit what the cost model skips versus the real witness; find expensive `Value::map` closures or a serialized foreign-EC witness chain; decide whether an O(1e6) dependency span forbids a parallel witness DAG or only costs GPU kernel-launch overhead. Companion to `astra-1-2.md` (same directory). Path abbreviations as in `astra-1-2.md`: `P` = midnight-proofs-0.8.3, `C` = midnight-circuits-7.2.4, `S` = midnight-zk-stdlib-2.3.5 (all under `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`), `MC` = midnight-curves-0.3.1.

## Answer

### A. What `cost_model` skips (none of the witness closures run)
- The cost model is `DevAssembly::run` (`P/src/dev/cost_model.rs:423`). Its `assign_advice` ignores the value closure (`_to`, `:526-538`), and so does its `assign_fixed` (`:553-560`). So the cost model executes no witness code at all, for known or unknown witnesses. It walks regions, selectors, copies, lookups and row counts only.
- `Value::map`/`and_then`/`zip` skip closure bodies for unknown values (`P/src/circuit/value.rs:101-126`), but that matters only for the prove-time witness, because the cost model never calls the closures.
- `cost_model(rel, None)` is not one pass. recl9's `optimal_k` loops k = 9..=25 and runs `cost_model(Some(k))` each time until the table bottleneck is passed (`S/src/lib.rs:2132-2148`). The source44 snapshot (Fenrir recl9 backend, `docs/results/sherlock-native-source44-20261022/source/sherlock_link.rs`) instead uses `automatic_fitting_model` (`docs/results/sherlock-native-source44-20261022/source/sherlock_link.rs:982-1006`, probes starting at 9, stops at a fixed point; the parent's `model` phase is 5.687 s, `D/samples/parent-proof.json`). The number of structural passes in that 5.687 s is not logged.
- Real prove-time witness work: `create_proof` synthesizes once per advice phase (`P/src/plonk/prover.rs:501`, `:533`). `WitnessCollection::assign_advice` evaluates the value closure only for current-phase advice (`:859` early return; `:871` `to().into_field().assign()`). Its `assign_fixed` is a no-op (`:876-893`: "We only care about advice columns here").
- Fixed values, including lookup table values and selectors, are generated at key generation, not at prove time (`P/src/plonk/keygen.rs:117-138`). So `setup_pk` (parent: 62.3 s; `D/samples/parent-proof.json`) is where fixed/table values are produced, and the prove interval (882.6 s) is structure walk plus advice closures plus the proving pipeline.
- Conclusion: the cost-model time is a lower bound on structure only. It says nothing about witness arithmetic, so it cannot be used as an upper bound on synthesis, as astra-1 suspected. No measured witness-closure time exists anywhere in the sources I read.

### B. Expensive witness closures (ranked by estimated work; none timed)
1. Foreign-EC group-op witnesses: one Fp inversion per op for lambda, via blst's Euclidean inverse (`.invert()` in the closure).
   - Doubling: `C/src/ecc/foreign/weierstrass_chip.rs:1182-1190`, in `assert_double` (`:1174`), used by `double` (`:761`).
   - Addition: `:1266`, in `assert_add` (`:1257`), used by `add` (`:711`) and `incomplete_add` (`:1138`).
   - Each is followed by `base_field_chip().assign` for the lambda limbs, so the foreign-field conversions below also apply.
2. Foreign-field limb conversion, BigUint-based, per assignment:
   - `C/src/field/foreign/field_chip.rs:430-434`: `x.map(|v| (v - 1).to_biguint() ...bi_to_limbs(...))`, a BigUint conversion and limb split inside a closure.
   - `C/src/biguint/biguint_gadget.rs:745-751`: `value.map(|x| big_to_limbs(...))` per BigUint assignment.
   - `C/src/biguint/types.rs:164`: `biguint_to_limbs`, the witness helper.
   - These allocate per call. Count per EC op is not measured.
3. GLV scalar decomposition: an extended-Euclid loop with BigInt vectors per scalar.
   - Body: `C/src/utils/util.rs:110-` (`glv_scalar_decomposition`, xgcd at `:124-`, division loop from `:134`).
   - Call: `C/src/ecc/foreign/weierstrass_chip.rs:1958-1977` (`glv_split`, closure at `:1973`), used by `msm_by_bounded_scalars` for scalars wider than 128+4 bits (`:850`). BLS G1 has the endomorphism, so every full-width MSM scalar is split.
4. MSM table precompute: 14 sequential incomplete adds per base, plus `incomplete_assert_different_x` (`:1845-1856`; `:1600` for the check).
5. Multi-select index search: `C/src/ecc/foreign/weierstrass_chip.rs:1515-1530`. For each selected point it clones the whole table vector (`table_values.clone()`), then does a linear `position` with projective equality on 2^WS = 16 entries. This is O(k·16) point comparisons per window, plus allocations.
6. Native inverses in region closures: `C/src/field/native/native_chip.rs:367-377`, `:774`, `:1253` (node 09; they run in both synthesis passes).
- Poseidon witness generation was not traced to closure level (partially traced).

### C. Dependency span: the serialized chains
Dependency depth is a different quantity from rows. The node-4 historical 958,095 rows and 1,032,465 table rows are sizes, not depths. I found no single O(1e6)-deep chain in the code. The longest chains are:

1. **Windowed MSM accumulator (verifier path).** `C/src/ecc/foreign/weierstrass_chip.rs:1875-1909`:
   - `nb_iterations = max_len` windows (`:1875`); each window does WS = 4 doublings then one incomplete add per base, all chained through `acc` (`:1877-1900`).
   - With GLV, each original base becomes two half-width scalars. For 128-bit halves, `max_len` = ceil(128/4) = 32, so the depth is about 32 × (4 + 2B) sequential group ops for B original bases. For example, B = 50 gives about 3.3k sequential group ops (ESTIMATE; B for the parent's MSMs is not recorded).
   - The table precompute (14 adds per base, `:1845-1856`) is independent across bases.
   - Associativity of the group law means the accumulator can be restructured as a parallel tree or scan, but the witness must still produce every intermediate `acc` value (a prefix scan). The circuit as written is sequential.
2. **Cofactor chain (not on the transcript path).** `assign` with subgroup check (`C/src/ecc/foreign/weierstrass_chip.rs:343-361`) multiplies by the BLS G1 cofactor, 0x396c8c005555e1568c00aaab0000aaab (`C/src/ecc/curves.rs:286-287`). That is 126 bits with popcount 48 (computed here). `mul_by_u128` (`:1652-1700`) runs 125 sequential doublings for `tmp`, and 47 incomplete adds into `res`. Critical path about 125-130 group ops per such point.
   - However, the transcript read path avoids this: `C/src/verifier/transcript_gadget.rs:142` uses `assign_without_subgroup_check`, and so does `C/src/verifier/msm.rs:373`. I did not find a verifier-path `assign` with subgroup check, so this chain is probably not on the parent's critical path. Confirm against the actual relation before relying on it. CORRECTION (see `astra-1-12.md`, hardened-copy caveat): the Sherlock parent uses the Fenrir-hardened `midnight-circuits` (`R/Cargo.toml:36`), whose `msm_randomness` assigns the shift `r` with the checked `assign` (`verifier-patch/midnight-circuits/src/ecc/foreign/weierstrass_chip.rs:1766`). So the cofactor chain is present in the parent via `r`, feeding `neg_alpha` and the table precompute; it is independent of the transcript.
3. **Poseidon transcript (sponge).** Each permutation depends on the previous state. Node 06 (`loki-06`, the Poseidon CODE claim) gives 8 full + 60 partial rounds, rate 2, 20 rows per permutation; I did not re-verify those counts here. Depth is (permutations) × 68 round-steps. For an assumed ~50 points per child (about 15 field elements per point via `C/src/ecc/foreign/weierstrass_chip.rs:235-247`, about 8 rate-2 permutations per point), that gives about 400 permutations, or about 27k sequential round-steps per child (ESTIMATE; child counts not recorded).

Maximum estimated depth is therefore about 10^3 to 10^4 operations, not 10^6. Rows and columns scale with about 10^6, but depth does not.

### D. Verdict on the O(1e6) premise
- The dependency span does not forbid a parallel witness DAG. The deepest chains are about 10^3 (MSM accumulator, after GLV and window choices) and about 10^4 (transcript round-steps). Both are small relative to the work. Scan or tree restructuring of the MSM accumulator, and fused per-permutation kernels for Poseidon, would bring them down.
- A GPU design that launches one kernel per group op or per Poseidon round would pay about 10^4 launches per child. At a typical launch cost (not measured here), that is tens to hundreds of milliseconds per child. That is a launch-overhead problem, fixable by fusion.
- The real obstacles to a parallel witness DAG are structural, not depth:
  (a) the synthesizer is a single-threaded closure walk over a shared mutable `WitnessCollection` and layouter (`P/src/plonk/prover.rs:845-872`; `P/src/circuit/floor_planner/single_pass.rs:81-114`), so a DAG must be extracted by restructuring the relation's witness code;
  (b) row placement depends on the shape pass (`single_pass.rs:93`, region start from earlier regions' columns), which is serial but cheap;
  (c) the incomplete-add exceptional-case checks (`completeness_error_if`, defined at `:1012`, used at `:1900`) compare values in order and raise errors that must be reproduced exactly.

## Raw numbers table

| Item | Value | Status | Source |
|---|---|---|---|
| Cost model pass | structure only; no closure evaluated for advice or fixed | CODE | `P/src/dev/cost_model.rs:526-560` |
| Prove-time advice closure | evaluated for current-phase advice only | CODE | `P/src/plonk/prover.rs:859`, `:871` |
| Prove-time fixed closure | ignored | CODE | `P/src/plonk/prover.rs:876-893` |
| Fixed/table values | produced at keygen | CODE | `P/src/plonk/keygen.rs:117-138` |
| Parent model / setup_pk / prove | 5.687 s / 62.328 s / 882.590 s | MEASURED (docs) | `D/samples/parent-proof.json` |
| MSM window count (GLV 128-bit halves) | 32 | CODE-derived | `C/src/ecc/foreign/weierstrass_chip.rs:1875`; `:850` |
| MSM sequential ops per base-set | 32 × (4 + 2B) | ESTIMATE; B unknown | `:1875-1909` |
| Table precompute per base | 14 incomplete adds | CODE | `:1845-1856` |
| Cofactor (BLS G1) | 0x396c8c005555e1568c00aaab0000aaab (126 bits, popcount 48) | CODE + computed | `C/src/ecc/curves.rs:286-287` |
| Cofactor chain depth per assign | ~125-130 group ops | CODE-derived; not on transcript path | `:343-361`, `:1652-1700`; `transcript_gadget.rs:142` |
| Poseidon rounds per permutation | 68 (8 full + 60 partial) | node 06 CODE claim, not re-verified | `loki-06` |
| Poseidon permutations per child | ~400 | ESTIMATE; child counts unknown | derived |
| Multi-select table size | 16 (2^WS) | CODE | `:1845`; `:1515-1530` |
| Historical rows node4 / table rows | 958,095 / 1,032,465 (sizes, not depth) | CODE/log | `R/keys/node4/keygen.stats.json` |

## Caveats
- No witness-closure timing exists. The ranking is by operation class (inversions, BigUint allocations, BigInt xgcd, table scans), not by measured cost.
- B (bases per MSM), the child point and scalar counts, the Poseidon permutation count, and the parent's phase count are not recorded. The depth numbers are formulas with ESTIMATE inputs.
- The cofactor chain may be off the parent's critical path (section C.2); I did not trace the full relation source.
- Recommended measurement (no run performed here): timers around the `Value::map` closures in `assert_add`, `assert_double`, `glv_split` and `big_to_limbs` callers, plus a count of group ops and permutations per MSM, under the existing `loki-profile` plan (`loki-01` L0). Then compare the sum against the 882.6 s prove interval.
- No builds, proofs, benchmarks or repo changes were made.
