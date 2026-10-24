# astra-3-4: CPU occupancy, concurrency, and the 4 CPU contexts + 1 GPU phase model

## Question
Is "4 CPU witness contexts + 1 heavy GPU phase" legal and resource-feasible on the pinned stack? Specifically: (1) Send/Sync of the synthesis path; (2) the pause point and what buffers exist while paused; (3) thread pools and oversubscription; (4) CPU-serial fractions; (5) PK sharing (Arc) versus duplication; (6) whether any per-proof CPU utilization is attributable; (7) the failure conditions of the bounded design.

No builds, no repo edits. All claims are CODE (local source, path:line), SOURCE (local receipts/stats), or ESTIMATE (arithmetic). No web sources used. Host facts: `nproc` = 20; `/proc/cpuinfo` model "Intel(R) Core(TM) Ultra 9 285K"; `free -g` total 90 GB, 73 GB available at time of check.

## Answer

### 1. Send/Sync gate (decides legality): contexts must be constructed in-thread; PK can be shared
- **CODE, `!Send`/`!Sync` circuit:** `midnight-zk-stdlib-2.3.5/src/lib.rs:1500-1505` `MidnightCircuit` holds `nb_public_inputs: Rc<RefCell<Option<usize>>>` and `circuit_error: Rc<RefCell<Option<R::Error>>>`, constructed at `lib.rs:1529-1530`. `Rc` is `!Send` and `!Sync`, so a `MidnightCircuit` cannot be moved to or shared between threads. Each CPU context must build its own `MidnightCircuit` inside its own thread.
- **CODE, ZkStdLib:** `lib.rs:366-375` and `474-483` hold `Rc<RefCell<bool>>` usage flags. `ZkStdLib` is created inside `MidnightCircuit::synthesize` (`lib.rs:1874` `ZkStdLib::new`), so it is per-synthesis and per-thread. Not a blocker.
- **CODE, bounds:** `Relation: Clone` only (`lib.rs:1795`), with no `Send`/`Sync`. `Circuit<F>` has no supertrait bounds (`midnight-proofs-0.8.3/src/plonk/circuit.rs:776`). `Layouter`/`RegionLayouter` use `SyncDeps`, a blanket no-op trait (`src/circuit/layouter.rs:15-17`). So the Rust type system does not forbid a `Relation` or `Circuit` from being `Send`; the `Rc` in `MidnightCircuit` is the only hard block.
- **CODE, PK and SRS are shareable:** `MidnightPK<R> { k, relation: R, pk: ProvingKey }` (`lib.rs:1620-1624`). `ProvingKey`, `VerifyingKey`, `Polynomial`, `Evaluator`, `permutation::ProvingKey` have no `Rc`/`RefCell`/`Cell`/raw pointers. A grep over `midnight-proofs-0.8.3/src/{plonk,poly,circuit,utils}` found no `Rc` or `RefCell` in key types (only `AssignedCell` and `unsafe` casts in `poly/kzg/msm.rs:185-186`). `ParamsKZG { g: Vec<E::G1>, g_lagrange, g2, s_g2 }` (`poly/kzg/params.rs:19-24`) is plain data. So `Arc<MidnightPK<R>>` is legal iff `R: Send + Sync`.
- **CODE, recl9 relations:** no `Rc`/`RefCell` in `recl9/src` relation types. The only `RefCell` is in `tower.rs:139,155` (field arithmetic counters, not a relation). `main.rs:850` comment confirms the binary is single-threaded in its own premise (env mutation). `Send` for `SherlockLinkRelation` / leaf / node / decider is **ESTIMATE by inspection**, not compiled. Moriarty `mori-backend` relation types (`midnight_zkir_v3::IrSource`) are not in the local registry and are UNVERIFIED.
- **Verdict:** 4 CPU witness contexts are legal as 4 threads, each constructing its own `MidnightCircuit` and running `create_proof` on its own witness, if the proof's pk is `Arc`-shared and `R: Send + Sync`. Failure condition: if any relation type (Moriarty IR) is `!Send`, the shared-PK design fails and each context must rebuild the PK (memory and time cost in §5).
- **CODE, the `circuits` slice:** `create_proof` (`midnight-proofs-0.8.3/src/plonk/prover.rs:348-360`) takes `circuits: &[ConcreteCircuit]`. The slice is processed serially inside one `compute_trace`: `parse_advices` loops `for ((circuit, advice), instances) in circuits.iter()...` (`prover.rs:515-517` region, serial), so **a slice is not synthesized in parallel**. The stdlib `prove` passes `std::slice::from_ref(circuit)` (`midnight-zk-stdlib-2.3.5/src/utils/plonk_api.rs:105-115`), so a single context = one circuit per proof.

### 2. Pause point and buffers held while paused
- **CODE, phase split:** `create_proof` = `compute_trace(...)` (`prover.rs:371-380`) then `finalise_proof(...)` (`prover.rs:387-396`). `compute_trace` does: `parse_advices` (synthesis via `SimpleFloorPlanner::synthesize` at `prover.rs:533`, then advice `lagrange_to_coeff` at `prover.rs:215`, then advice commitments at `prover.rs:569-571`), lookup permuted commitments, permutation product commitments, lookup product commitments, trash commitments, vanishing random commitment. `finalise_proof` does `compute_nu_poly` (`prover.rs:595`), the advice and instance **coset** NTTs (`prover.rs:612-630`), vanishing construct and evaluate, queries, multiopen.
- **CODE, `ProverTrace` (`midnight-proofs-0.8.3/src/plonk/traces.rs:14-29`)** is the pause object. Its fields and their sizes (all at domain size n, no extended cosets):
  - `advice_polys: Vec<Vec<Polynomial<F, Coeff>>>` (n per advice column; 15 columns for the k20 decider/node, see §5),
  - `instance_polys` (Coeff, n each),
  - `instance_values` (Lagrange, n each; `dead_code`, kept for split accumulation),
  - `lookups: Vec<Vec<lookup::prover::Committed>>` (each: `permuted_input_poly`, `permuted_table_poly`, `product_poly`, all Coeff, `lookup/prover.rs:34-38`), 5 lookups at k20 → 15 polys,
  - `permutations: Vec<permutation::prover::Committed>` (`sets: Vec<CommittedSet{ permutation_product_poly: Coeff }>`, `permutation/prover.rs:20-28`), count = ceil(18 permutation columns / chunk) sets (chunk unknown; ESTIMATE 2–4 sets),
  - `trashcans` (`trash_poly`, Coeff, `trash/prover.rs:16-18`), count unknown,
  - `vanishing: Committed { random_poly: Coeff }` (`vanishing/prover.rs:21-23`),
  - scalars and `challenges`, `beta`, `gamma`, `theta`, `trash_challenge`, `y`, and the transcript (carried separately).
- **ESTIMATE, pause-resident bytes per context (k20, n = 2^20, 32 MiB per Coeff poly):** advice 15 + lookups 15 + vanishing 1 + permutation sets ~2–4 + instance ≥1 + trash (unknown) = ~34–36 polys ≈ **1.1–1.2 GiB** before trash. ESTIMATE only; the trash count and instance column count are unmeasured.
- **Excluded from the pause:** no extended-domain buffers exist yet. The ext buffers (advice/instance cosets, `nu` poly, h pieces) are created only in `finalise_proof`. Per coset column at N = 2^22 (design [03]; quotient degree not verified here) = 128 MiB.
- **The pause must keep the PK alive:** `finalise_proof` reads `pk.fixed_cosets`, permutation cosets and `pk.ev` (`prover.rs:595` onwards, `permutation` at `plonk/mod.rs:346`), so the pause holds a PK reference, not a copy.
- **Safe-to-pause:** after `compute_trace` returns (`prover.rs:380`) and before `finalise_proof` (`prover.rs:381`). The `rng` and `transcript` must be moved with the trace.

### 3. Thread pools and oversubscription (CODE)
- **Rayon (global):** `midnight-proofs` `utils/multicore.rs` re-exports rayon. `current_num_threads()` is read in `midnight-curves-0.3.1/src/fft.rs:41` and `midnight-proofs` `plonk/evaluation.rs:317`, `plonk/vanishing/prover.rs:6`. `recl9/src/main.rs:171-173` records `threads()` = `RAYON_NUM_THREADS` or `"all"`. Rayon default thread count is the logical CPU count (rayon documentation; not re-verified in local source).
- **blst pool (separate, not rayon):** `blst-0.3.17/src/lib.rs:41-50` `da_pool()` is a process-global `static mut POOL` holding `Mutex<ThreadPool>`, initialised with `ThreadPool::default()` (`lib.rs:46`). `threadpool-1.8.1/src/lib.rs:688-689` sets `ThreadPool::new(num_cpus::get())`. So blst's pool = 20 workers here, sized independently of rayon and **not configurable by RAYON_NUM_THREADS**.
- **Which blst call the MSM uses:** `midnight-proofs` `poly/kzg/msm.rs:181-192` (0.8.3) → `midnight_curves::G1Projective::multi_exp` (`midnight-curves-0.3.1/src/bls12_381/g1.rs:628-647`) → blst `p1_affines::from(points)` (`g1.rs:637`) and `points.mult(scalar_bytes, 255)` (`g1.rs:644`) → `blst-0.3.17/src/pippenger.rs`: `from()` uses `da_pool` for `npoints >= 768` (`pippenger.rs:82`), `mult()` uses `da_pool` (`pippenger.rs:141`), grid dispatch for `npoints >= 32 && npoints >= ncpus` (`pippenger.rs:165`), all jobs `joined_execute` on the pool with Barrier joins (`pippenger.rs:110, 114, 212`). Each large MSM therefore saturates all 20 da_pool workers.
- **Oversubscription (CODE, bound):** a rayon-using FFT/evaluation step and a blst MSM from concurrent contexts can have up to 20 (rayon, shared by all contexts) + 20 (blst da_pool) = **40 runnable pool workers on 20 logical CPUs**, plus the context threads themselves. This is contention, not deadlock (jobs are FIFO-queued; barrier waiters are the caller and the last finisher only).
- **Fix options:** per-context `rayon::ThreadPoolBuilder::install` changes `current_num_threads()` in `fft.rs:41`, but the blst pool stays at 20 unless blst is forked. So a 4-context design cannot cap blst threads without a fork.

### 4. CPU-serial fractions inside one proof (CODE)
Each item is a single-core pass per proof or per commit. These are the parts 4 contexts can overlap:
- **Synthesis, 2 passes per region:** `SimpleFloorPlanner::assign_region` runs the region closure once for `RegionShape` (`midnight-proofs-0.8.3/src/circuit/floor_planner/single_pass.rs:90-93`) and again for the real assignment (`single_pass.rs:111-114`). The planner itself is serial.
- **Serial `parse_advices`:** circuits and advice columns are processed serially (`prover.rs:515-575`); each advice column's commitment is a serial `.map(|poly| CS::commit_lagrange(...)).collect()` (`prover.rs:569-571`), with each MSM internally parallel on da_pool.
- **Serial coset NTTs in `finalise_proof`:** advice and instance coset conversions are serial `.map().collect()` over columns (`prover.rs:612-630`); each FFT is parallel only internally.
- **Serial FFT preamble:** `midnight-curves-0.3.1/src/fft.rs:46-52` bit-reversal swap loop runs over the whole array on one thread; `fft.rs:53-61` precomputes the twiddle table serially (n/2 field multiplications). Only the butterflies (`rayon::join` at `fft.rs:107`) are parallel.
- **Serial MSM preamble:** `midnight-proofs` `poly/kzg/msm.rs:166-172` `filter(...).map(...).unzip()` copies scalars and bases into two new `Vec`s serially before the blst call (for n = 2^20: about 32 MiB scalars plus 144 MiB projective bases ESTIMATE, per commit). `midnight-curves g1.rs:639-642` packs scalar bytes serially. `g1.rs:637` converts the SRS bases to affine on every call (parallel only above 768 points, `pippenger.rs:82`), so the SRS affine conversion is repeated per commit.
- **Serial `commit` / permutation (keygen, `keygen_pk`):** `plonk/keygen.rs:322` synthesizes serially; `keygen.rs:334,337` `par_iter` over fixed columns (column-parallel); `permutation/keygen.rs:175` `par_iter_mut`. So keygen is partly column-parallel, unlike the prove path.
- **Verdict:** the serial fraction is material but **not measured**. No per-phase CPU profile exists for the k20 prove (LOKI L0 is the missing measurement). The serial items above are the overlap candidates for contexts.

### 5. Memory: Arc-share versus duplicate (ESTIMATE from CODE + SOURCE)
- **SOURCE, measured peak RSS:** decider prove (k20, `recl9/out/decider-b1/batch-1.stats.json`): `peak_rss_kb` 14,965,544 (14.97 GB); node prove (`recl9/out/node-b1/w.stats.json`): 14,988,176 kB; standin-d decider: 14,900,540 kB. These are whole-process peaks with one PK resident.
- **SOURCE, column counts (k20, `recl9/keys/decider/keygen.stats.json` and `recl9/keys/node4/keygen.stats.json` `cost` blocks):** `advice_columns` 15, `fixed_columns` 45, `lookups` 5, `permutations` 18, `rows` 841,682 (decider) / 958,095 (node), `table_rows` 1,032,209 / 1,032,465, `unusable_rows` 7. Interpretation of `permutations: 18` as permutation columns is ESTIMATE (name from `cost_json`, `recl9/src/main.rs:173-179`).
- **ESTIMATE, PK resident bytes (what `ProvingKey` keeps; `plonk/mod.rs:338-348`):**
  - `fixed_values`: 45 × 32 MiB = 1.41 GiB,
  - `fixed_polys` (coeff): 45 × 32 MiB = 1.41 GiB,
  - `fixed_cosets` (ext): 45 × 128 MiB = 5.63 GiB (N = 2^22 assumed, design [03]),
  - permutation polys 18 × 32 MiB = 0.56 GiB, permutation cosets 18 × 128 MiB = 2.25 GiB,
  - `l0`, `l_last`, `l_active_row` (ext): 3 × 128 MiB = 0.38 GiB,
  - **Total ≈ 11.6 GiB** (ESTIMATE; extended factor and whether all fixed columns are extended are not verified here). This is consistent with the 14.97 GB measured peak once ~3.4 GB of per-proof working set is added.
- **Per-context working set beyond PK (ESTIMATE):** pause state ~1.1 GiB (§2) + ext buffers during `finalise_proof` (advice cosets 15 × 128 MiB = 1.9 GiB; instance cosets; nu/h pieces) → **~3–4 GiB per context at the peak, transient in finalise**.
- **Duplicate PK in 4 contexts (ESTIMATE):** 4 × 11.6 = **46.5 GiB** of PK plus 4 × ~3.4 = 13.6 GiB working set → **~60 GB** at peak. Against 90 GB total and 73 GB available on this host, this is feasible only if nothing else runs, and it fails if the 4 contexts reach `finalise_proof` together with the ext buffers.
- **Arc-shared PK (ESTIMATE):** 11.6 + 4 × 3.4 ≈ **25 GiB**. Feasible with room.
- **GPU residency (design [03]):** the GPU phase needs the same PK cosets on device. 11.6 GiB of PK cosets alone plus ext columns can exceed the 32 GB card's budget if the design's 19 GiB worked total is also resident (design [03]; not re-verified here).

### 6. CPU utilization: what exists, and why it cannot be attributed
- **SOURCE, the only CPU counter:** `docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json:92` `prefinal_service_snapshot.cpu_seconds` = 1,820.361367, with `memory_peak_bytes` 15,678,128,128, `scope` = "mixed selected service snapshot before controller exit; neither final totals nor proof-isolated CPU/RSS", and `final_counter_available` = false. `README.md` in the same folder says the same. Phase timings in the same receipt (`phases.setup_pk.duration_seconds` 62.328385318; `phases.prove.duration_seconds` 882.589555363; `phases.model.duration_seconds` 5.687048131; `native_capture_seconds` 951.6142616319703; `root_capture_seconds` 954.127131041023).
- **Verdict:** 1,820 CPU-s over 951.6 wall-s would be 1.91 cores average, about 9.6% of 20 logical CPUs, **if the counter were per-proof**. It is not (service snapshot, not a final total). So **no qualified per-proof CPU utilization exists**, and any throughput ceiling built on it is not supported.
- **CODE, no per-phase CPU instrumentation:** recl9 records wall time (`Instant::now` around `setup_pk` and `prove`, e.g. `main.rs:348-349, 461-462, 597-598`), `peak_rss_kb` (its helper is called at `main.rs:330`, `381`), and `threads()` (`main.rs:171-173`). No `getrusage`/`utime`/`cpu_s` in recl9 `src` (grep: no hits). So CPU time per phase is unrecorded.
- **Consequence for the model:** the concurrency ceiling is 1/U_seq, where U_seq is one proof's average core utilization (CPU-seconds / (20 × wall)). It is unmeasured. The `2×` two-lane overlap bound from astra-3's message is a different (and weaker) statement and is not needed here.

### 7. Bounded design: 4 CPU contexts + 1 heavy GPU phase
**Model (ESTIMATE):** let a = single-core serial CPU-s per proof (synthesis ×2, serial FFT/MSM preambles, serial coset loops), b = parallelizable CPU-s per proof, e = parallel efficiency on 20 threads. Sequential wall W = a + b/(20e). Best concurrent throughput time per proof = max((a+b)/20, a). Throughput gain = W / max((a+b)/20, a). With a ≪ b, the gain ceiling is 1/e (about 1/U_seq); the gain is large only if e is low, i.e., if one proof leaves most cores idle. This is the refutation condition: if the parallel phases already run at e ≳ 0.7–0.8, 4 CPU contexts add little (≲1.4×) beyond overlapping the serial a. Unmeasured.

**Design, conditions under which it works:**
1. Each CPU context builds its own `MidnightCircuit` in its thread (§1, `Rc`).
2. The PK is `Arc<MidnightPK<R>>`, shared read-only; requires `R: Send + Sync` (§1). If `R` is `!Send` (not verified for Moriarty IR), fall back to per-context PK (§5, 46.5 GiB).
3. A context pauses after `compute_trace` (§2), holding ~1.1 GiB + transcript + rng. With 4 contexts paused, +4.4 GiB.
4. Admission to the GPU finalise phase: 1 heavy at a time (the design's 1 heavy GPU proof), and no more than 1 context in `finalise_proof`'s ext-buffer phase on CPU. This bounds the ext transient to ~3.4 GiB.
5. Thread budget: blst da_pool = 20 fixed (§3). Until blst MSMs leave the CPU (L3 GPU MSM), the CPU contexts cannot cap their MSM threads. Use `RAYON_NUM_THREADS` or `ThreadPoolBuilder` to cap rayon-only work (FFT, evaluation) to 20 − (GPU host threads).

**Failure conditions (when 4+1 is refuted):**
- **F1 (`!Send` relation):** if any relation type is `!Send` (Moriarty IR not inspected), shared PK is illegal → duplicate PK → ~60 GB peak; infeasible at 4.
- **F2 (memory):** duplicated PK at 4 contexts ≈ 60 GB against 90 GB total / 73 GB available; any context at `finalise_proof` with ext buffers beyond ~3.4 GiB breaks it.
- **F3 (oversubscription):** blst da_pool (20) + rayon (20) + contexts, all MSM-heavy, thrash 20 logical CPUs. Expected ratio of CPU time to wall stays ≤ 20, so throughput does not exceed the machine's capacity: any gain is capped at (20 × utilization headroom) — no more.
- **F4 (headroom):** if one proof already runs at e ≳ 0.7–0.8 on 20 threads, the gain is ≲ 1.4×; the 4 contexts then only hide `a`, which is unmeasured (L0 missing).
- **F5 (serial share):** if `a/(a+b)` < ~5%, the contexts give ≈ 1× beyond rayon's own work.

**Hard obstacles list (code-level):**
- `MidnightCircuit` `Rc<RefCell>` (`lib.rs:1502-1503`) → per-thread construction.
- `ZkStdLib` used-flags `Rc<RefCell<bool>>` (`lib.rs:366-375`) → per-synthesis only.
- `da_pool` is a process-global `static mut` with a `Mutex` clone per call (`blst-0.3.17/src/lib.rs:41-50`) and fixed size (`threadpool` default `num_cpus::get()`, `threadpool-1.8.1/src/lib.rs:689`) → not configurable without a blst fork.
- `msm_specific` serial filter/unzip (`poly/kzg/msm.rs:166-172`), `multi_exp` serial scalar packing (`midnight-curves g1.rs:639-642`) and per-call affine conversion of the SRS bases (`g1.rs:637`) → serial per-commit overhead; fixing needs upstream changes or a fork.
- `best_fft` serial bit-reversal and twiddle precompute (`midnight-curves fft.rs:46-61`).
- `parse_advices` and `finalise_proof` serial over columns (`prover.rs:515-575`, `612-630`).
- `SimpleFloorPlanner` double region execution (`single_pass.rs:90-93, 111-114`).
- `create_proof` serial over the `circuits` slice (`prover.rs:515-517`); no parallel batch prove exists in the pinned stdlib (`plonk_api.rs:105-115`).

## Raw numbers table

| Quantity | Value | Basis | Source |
|---|---|---|---|
| Host logical CPUs | 20 | CODE | `nproc`, `/proc/cpuinfo` |
| Host RAM total / available | 90 GB / 73 GB | SOURCE | `free -g` |
| Parent receipt phases | model 5.687 s; setup_pk 62.328 s; prove 882.590 s; native 951.614 s; root 954.127 s | SOURCE | `docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json` (`phases.*`, `nested_wall_observations.*`) |
| Parent receipt CPU | 1,820.361 s, unattributable | SOURCE (service snapshot) | same receipt, `prefinal_service_snapshot` |
| Decider k20 (recl9 `decider-b1`) | keygen_pk 46.892 s; prove 586.382 s; total 634.819 s; peak RSS 14,965,544 kB | SOURCE | `recl9/out/decider-b1/batch-1.stats.json` |
| Node k20 (recl9 `node-b1`) | keygen_pk 104.463 s; prove 867.280 s; total 973.712 s; peak RSS 14,988,176 kB | SOURCE | `recl9/out/node-b1/w.stats.json` |
| Node k20 keygen_vk (recl9 `node4`) | 622.827 s; peak RSS 4.50 GB | SOURCE | `recl9/keys/node4/keygen.stats.json` |
| Decider k20 keygen_vk (recl9 `decider`) | 411.979 s; peak RSS 4.77 GB | SOURCE | `recl9/keys/decider/keygen.stats.json` |
| Cost-model time in keygen stats | decider 131.800 s; node 335.252 s | SOURCE | same keygen stats (`cost.cost_model_seconds`) |
| k20 columns (decider and node) | advice 15, fixed 45, lookups 5, permutations 18 | SOURCE | `recl9/keys/*/keygen.stats.json` `cost` |
| k20 rows (decider / node) | 841,682 / 958,095; table 1,032,209 / 1,032,465 | SOURCE | same |
| Proof bytes (k20 decider, node) | 6,432 | SOURCE | `out/*/stats` |
| PK resident (k20, N = 2^22) | ~11.6 GiB | ESTIMATE | §5 formula |
| Pause-resident per context | ~1.1 GiB (≥ 34 polys × 32 MiB) | ESTIMATE | §2 |
| 4 duplicated PKs | ~46.5 GiB PK, ~60 GB peak | ESTIMATE | §5 |
| 4 Arc-shared PKs | ~25 GiB | ESTIMATE | §5 |
| Oversubscription bound | 20 (rayon) + 20 (blst da_pool) = 40 workers / 20 CPUs | CODE + ESTIMATE | §3 |
| Concurrency gain ceiling | 1/U_seq (U_seq unmeasured) | ESTIMATE | §6–7 |

## Caveats
- **Nothing is compiled.** Send/Sync of the actual `SherlockLinkRelation`, leaf, node and decider types, and of Moriarty's `IrSource`, is by inspection only. A build would confirm `MidnightPK<R>: Send + Sync`.
- **Pinned versions:** recl9 uses midnight-proofs 0.8.2 (identical plonk/keygen/permutation/prover sources to 0.8.3; the MSM dispatch differs at `msm.rs:178` in 0.8.2 vs `:181` in 0.8.3). The FFT, MSM preamble and pool findings are in midnight-curves 0.3.1 and blst 0.3.17 (recl9 `Cargo.lock`), shared by both.
- **Moriarty mori-backend proving** uses `midnight_zkir_v3`/`midnight_transient_crypto` (`proving.rs:475-480` `ir.keygen`, `ir.prove`), not in the local registry. Its pool and Send/Sync behaviour is UNVERIFIED.
- **Column counts** are from recl9 cost stats (k20 decider and node). The parent circuit's counts are not recorded locally.
- **Extended-domain factor (N = 2^22)** is taken from the design (LOKI-DESIGN [03]) and not re-derived; quotient degree of these circuits is unverified. The PK and pause sizes are ESTIMATE.
- **No CPU-per-phase measurement exists.** The serial fraction `a` and the parallel efficiency `e` are unmeasured. The concurrency ceiling is therefore not a number; the formula is the deliverable.
- **Rayon default thread count** is from rayon's documented default, not local source.
- The 1,820 CPU-s figure is a whole-service snapshot. It must not be used to derive per-proof utilization.

## Addendum (correction to section 6 and section 5, found during astra-3-7)
- **Per-invocation CPU receipts DO exist for node and decider.** `recl9/logs/node-prove.log` lines 5-16 and `recl9/logs/decider-prove.log` lines 5-16 are GNU `time -v` outputs for `straw-recl9 prove-node` and `prove` (CODE). They give user + system CPU, elapsed, percent CPU, max RSS, and context switches for each whole invocation (including PK setup):
  - node: 2534.17 s user + 81.31 s sys = 2615.48 CPU-s over 16:13.73 = 973.73 s elapsed; 268% CPU; max RSS 14,988,176 kB; involuntary context switches 4,599,052.
  - decider: 2486.95 s user + 59.80 s sys = 2546.75 CPU-s over 10:34.85 = 634.85 s; 401% CPU; max RSS 14,965,544 kB.
- These are attributable to one process, so the earlier statement that no per-proof CPU measurement exists is wrong for node and decider. The 1,820 CPU-s parent receipt remains whole-service and unattributable. Per-process utilization is 13.4% (node, 268/2000) and 20.1% (decider, 401/2000) of 20 logical CPUs. The concurrency-gain ceiling 1/U is therefore about 7.5x (node) and about 5.0x (decider), ESTIMATE, and only if the extra cores can be filled without contention. The 4.6M involuntary context switches in the node run show that contention is already present at 268%.
- **PK-resident measured:** `recl9/logs/node-prove.log` line 3 `[node] pk 104.5s rss 11799136 kB` (CODE). RSS after PK setup is about 11.25 GiB, which supports the 11.6 GiB PK-resident ESTIMATE in section 5 (now partly measured).

## Addendum 2 (corrections found during astra-3-10)
- **PK column double count fixed.** `midnight-proofs-0.8.3/src/dev/cost_model.rs:244-246` defines `fixed_columns = fixed.len() + permutation.columns` and `permutations = permutation.columns`. So the recorded 45 fixed columns already include the 18 permutation columns: 27 plain fixed + 18 permutation. Section 5's "45 fixed + 18 permutation" overcounted. Corrected PK payload is about 8.25 GiB (my unit mix) to 8.81 GiB (the PROPOSAL's uniform 6 units per column: 32 MiB x [(27+18)x6+12] = 8.8125 GiB). The 11.6 GiB figure in section 5 and section 9 is withdrawn. The measured `[node] pk` RSS of 11.25 GiB (node-prove.log line 3) is an upper bound including the SRS and other state.
- **Unit fix.** Peak RSS values in this file written as "15.0 GB" / "14.97 GB" / "14.99 GB" are kB values misread as decimal GB. 14,988,176 kB = 14.29 GiB = 15.35 GB decimal; 14,965,544 kB = 14.27 GiB.
- Peak-over-PK in the node prove is therefore about 14.29 - 11.25 = 3.0 GiB, which is the working-set figure I should have used for the 4-context and 5-slot memory arithmetic.

## Addendum 2 (corrections found during astra-3-10)
- **PK column double count fixed.** `midnight-proofs-0.8.3/src/dev/cost_model.rs:244-246` defines `fixed_columns = fixed.len() + permutation.columns` and `permutations = permutation.columns`. So the recorded 45 fixed columns already include the 18 permutation columns: 27 plain fixed + 18 permutation. Section 5's "45 fixed + 18 permutation" overcounted. Corrected PK payload is about 8.25 GiB (my unit mix) to 8.81 GiB (the PROPOSAL's uniform 6 units per column: 32 MiB x [(27+18)x6+12] = 8.8125 GiB). The 11.6 GiB figure in section 5 and section 9 is withdrawn. The measured `[node] pk` RSS of 11.25 GiB (node-prove.log line 3) is an upper bound including the SRS and other state.
- **Unit fix.** Peak RSS values in this file written as "15.0 GB" / "14.97 GB" / "14.99 GB" are kB values misread as decimal GB. 14,988,176 kB = 14.29 GiB = 15.35 GB decimal; 14,965,544 kB = 14.27 GiB.
- Peak-over-PK in the node prove is therefore about 14.29 - 11.25 = 3.0 GiB, which is the working-set figure to use for the 4-context and 5-slot memory arithmetic.
