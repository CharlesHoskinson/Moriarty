# astra-3-9: Independent memory feasibility review (host 64 GiB, device 24 GiB, B7)

## Question
Audit the proposed memory contract: 4 CPU witness workers + 1 GPU arithmetic context, 2 heavy proving keys (PKs) shared, 64 GiB host and 24 GiB device. Can it be feasible, conditional on profiles? How do phase-only advice/rational storage reduce memory? What must stay resident while waiting on the GPU? One k20 heavy GPU job at a time plus small leaf batches. Define host worker, proof workspace and CUDA context. Does B7 (peak under decimal 16 GB, swap 0) pass with 18 GiB PKs?

## Answer
- **Terms (kept separate in the contract):**
  - **host worker**: a CPU thread pool slot doing ZKIR preprocessing, witness synthesis and rational conversion for one job. Its memory is job-scoped: rational advice, Lagrange advice, denominators (node 01, `loki-01-prover-profile.md:101`, `:113`). It holds no PK.
  - **proof workspace**: the buffers of one `create_proof` call (advice, extended cosets, numerator, quotient, transcript), live from `parse_advices` to `finalise_proof`. Sized by n·q·columns; can sit on host, device, or both.
  - **CUDA context**: one per process per device, holding module loads, kernels, stream/allocator state and reserves. It is not per-worker and not per-proof. Its device footprint is not measured here; measure it with `cudaMemGetInfo` before and after context creation.
  - **PK (shared key)**: decoded, process-lifetime; shared across jobs with the same circuit. Not a workspace.
- **Arithmetic checks (node 01 formulas, k20, unit U = 2^20 x 32 B = 32 MiB, q = 4; F = 27 fixed, P = 18 permutation columns, historical node4 values):**
  - PK coset forms (F+P)·q·U + 3qU = 192 U = **6.00 GiB**.
  - PK coefficient forms (F+P)·U = 1.406 GiB; Lagrange forms (F+P)·U = 1.406 GiB.
  - Total PK payload (F+P)(2+q)U + 3qU = 282 U = **8.8125 GiB** per heavy key. Node 01's formula checks.
  - SRS: 2 x 144 x 2^20 = 0.281 GiB. FFT twiddles: 0.0625 GiB.
  - Per-proof workspace (sum of listed retained components only; incomplete, excludes MSM scratch, rational representations and overlapping temporaries): advice + instance cosets (17 x 128 MiB) 2.125 GiB; trace coefficients 41 x 32 MiB 1.281 GiB; numerator + 6 permutation cosets 7 x 128 MiB 0.875 GiB. Sum **4.281 GiB** per proof, excluding shared PK/SRS/twiddles.
  - Node 01's whole-proof total 13.44 GiB = PK 8.81 + SRS 0.28 + twiddles 0.06 + workspace 4.28. Node 01 says this is not a peak and that lifetimes differ. Its arithmetic is correct.
- **Phase-only storage (PROPOSAL; needs code confirmation):**
  - In `create_proof`, the fixed Lagrange values are used during lookups/permutation/numerator (`plonk/prover.rs:126,153,191`); the fixed extended cosets during the numerator and quotient (`prover.rs:641`); l0/l_last/permutation cosets at `prover.rs:651-654`. Fixed coefficient forms are only read in the opening step (`prover.rs:707`, `:771`).
  - So the coefficient forms (1.41 GiB) can stay on host until the opening, and Lagrange forms can be dropped after the phase that reads them. Device residency during the numerator then needs only the coset forms (6.00 GiB).
  - Advice: once advice is committed, its Lagrange values are only needed if a later lookup/permutation of that phase reads them. Keep coefficient forms (A·32 MiB = 0.47 GiB for 15 columns). Lookup auxiliaries are already materialised one lookup at a time (node 01).
  - Caveat: 4.28 GiB is a sum of buffers that do not all coexist. The true device peak per proof is lower, but it is unmeasured.
- **What must stay while waiting on the GPU:** the job's committed advice coefficients (or its host copy), the transcript state, the witness-independent PK host copy, and SRS on host. The host worker may free its rational and Lagrange advice once the GPU has taken ownership of the phase inputs. Keep witness buffers isolated and clear them after use (node 10 policy).
- **Scenarios, device (24 GiB budget; ESTIMATE from the arithmetic above, before context/reserve):**
  | Scenario | Device payload | Verdict |
  |---|---|---|
  | S1: one heavy, coset-only PK resident | 6.00 + 0.34 + 4.28 = **10.63 GiB** | fits; reserve ~2 GiB unmeasured |
  | S2: two heavy sharing one PK | 6.34 + 2 x 4.28 = **14.90 GiB** | fits if per-proof peak is ≤4.28 GiB, measured |
  | S3: node + decider, both coset-only | 12.00 + 0.34 + 2 x 4.28 = **20.91 GiB** | tight: 2 GiB reserve gives ~23 GiB; serialize heavy phases by default |
  | S4: node + decider, full PK (3 forms) | 17.63 + 0.34 + 2 x 4.28 = **26.5 GiB** | infeasible at 24 GiB |
  | S5: one heavy, full PK | 8.81 + 0.34 + 4.28 = **13.44 GiB** | fits |
  - So the 24 GiB device budget is feasible only with coset-only residency, one heavy per PK, and measured per-proof peaks. Two different heavy PKs cannot both be coset-resident with two concurrent proofs without a margin the arithmetic does not show.
- **Host (64 GiB, conditional):**
  - Decoded PKs: 2 x 8.81 = 17.63 GiB for node + decider (full forms in host RAM).
  - **Hazard:** the process-global LRU of decoded PKs holds five entries (`~/repos/midnight-ledger/transient-crypto/src/proofs.rs:250`, `PK_CACHE_SIZE = 5`, commit d89e0b63). At the node-01 payload, five decoded k20 keys = 44.06 GiB. The cache must be capped at the active set, or the decoded PK cache replaced by an explicit budgeted cache.
  - Proof workspace if CPU-resident: 2 x 4.28 = 8.56 GiB for two heavy jobs.
  - Host witness workers: rational advice 1.055 GiB + Lagrange advice 0.47 GiB + denominators (not measured) ≈ 1.5 GiB per in-flight job; 4 workers ≈ 6 GiB (ESTIMATE).
  - Sum for 2 PKs + 2 heavy workspaces + 4 witnesses: 17.63 + 0.34 + 8.56 + 6.0 ≈ **32.5 GiB** before process, allocator and decode transients. That fits 64 GiB conditionally. It does not fit with five cached PKs.
- **B7 (`qa/sherlock/predicates.yaml:154-158`): peak under decimal 16 GB = 14.90 GiB, swap 0.** Each 18 GiB PK payload (two keys at 8.81 GiB) exceeds that alone. Even a single heavy job (PK 8.81 + SRS 0.28 + workspace 4.28 = 13.37 GiB) leaves only ~1.5 GiB for witness, allocator and gzip decode transients. So B7 is at risk for one heavy job and cannot pass with two PKs. The 64 GiB host plan is a separate declared resource experiment, not B7 compliance, as astra-3 said.
- **Swap state observed now:** the host snapshot (raw/host-snapshot.txt, 2026-10-24T22:44:41Z) shows swap 32 GB total, **8 GB used**. B7 requires swap 0, so the host must be quiesced or its swap usage excluded from the scope the predicate measures. This is an observation of system state, not a prover measurement.
- **Heavy GPU job at a time plus small leaves:** S1 (10.63 GiB) + context and reserve (unmeasured, reserve ~2 GiB) leaves ~10 GiB of the 24 GiB budget. Leaves at k18 have 1/4 the linear workspace of k20 (ESTIMATE: ~1.07 GiB per leaf if the same columns; their own PK coset forms are not in the sources and must be read from their keygen stats). Two small leaves fit beside one heavy job.

## Raw numbers
| Item | Value | Source / status |
|---|---|---|
| U = k20 scalar array | 32 MiB | arithmetic |
| PK payload per heavy key (F=27, P=18, q=4) | 8.8125 GiB (coset-only 6.00) | node 01 `:103` formula, checked |
| Per-proof workspace (listed components; incomplete) | 4.281 GiB | node 01 `:103` components, checked |
| SRS + twiddles | 0.281 + 0.0625 GiB | node 01 `:103` |
| Decoded PK LRU capacity | 5 keys = 44.06 GiB at node-01 payload | midnight-ledger `proofs.rs:250` |
| B7 bound | 16 GB decimal = 14.90 GiB, swap 0 | `predicates.yaml:156` |
| Host swap at snapshot | 8 GB used of 32 GB | measured, raw/host-snapshot.txt |
| Device total | 32607 MiB (31.84 GiB) | measured, raw/host-snapshot.txt |
| Rational advice | 15 x 72 B x 2^20 = 1.055 GiB | node 01 `:101`, ESTIMATE (72 B layout unconfirmed) |

## Caveats
- All k20 PK and workspace numbers use node 01's historical node4 values (F=27, P=18, A=15, q=4). The decider's F, P and q are not in the sources; the "PK doubles across node and decider" premise is astra-3's assumption until the decider's keygen stats are read.
- The q=4 assumption: node 01 notes q=8 raises the PK term to 14.81 GiB, which would make S3 and S4 worse and S1 tighter.
- Per-proof workspace is NOT an upper bound: it sums only the listed retained components and excludes MSM scratch, rational representations and overlapping temporaries (corrected after astra-3 review). The real device peak depends on lifetimes not measured here.
- CUDA context and allocator reserve are unmeasured. Use `cudaMemGetInfo` before and after context creation.
- The 72 B rational cell and 1.055 GiB figure are unconfirmed. Use `size_of` instrumentation (node 01).
- No builds or runs were done; these are arithmetic and code-reading results.

## Sources
- Node 01: `~/repos/Moriarty-factory/wiki-llm/loki/nodes/loki-01-prover-profile.md` lines 53-62, 99-105, 113-116
- Moriarty predicate: `~/repos/Moriarty-factory/qa/sherlock/predicates.yaml` lines 154-158
- midnight-proofs 0.8.3: `src/plonk/mod.rs:338` (ProvingKey fields), `src/plonk/prover.rs:126,153,191,641,651-654,707,771`
- midnight-ledger `transient-crypto/src/proofs.rs:250-300` (commit d89e0b63)
- Host snapshot: `helpers/haiku-10/raw/host-snapshot.txt`

## Extension: lazy PK representation (B7-compatible path, same key)
Per-key figures (q=4, F+P=45 fixed and permutation polynomials, 3 extended masks = 48 coset polynomials, U = 32 MiB):
- Full host PK (values + coefficients + 48 cosets): 8.8125 GiB per key; two keys 17.63 GiB.
- Keep only values + coefficients on host: 2 x 45 x U = **2.8125 GiB per key**; two keys **5.625 GiB** (arithmetic checks).
- Coset set on device: 48 x 128 MiB = **6.0 GiB per key**; two keys 12.0 GiB (arithmetic checks).

**Device (24 GiB; reserve and context unmeasured):**
| Case | Device payload | Verdict |
|---|---|---|
| One key's cosets resident + one proof | 6.00 + 0.34 + 4.28 = 10.63 GiB | fits with ~13 GiB margin |
| Two keys' cosets resident + two proofs | 12.00 + 0.34 + 2 x 4.28 = 20.91 GiB | fits only if per-proof peak ≤ ~4.8 GiB with a 2 GiB reserve; else OOM |
| Two keys resident, one proof at a time | 12.00 + 0.34 + 4.28 = 16.6 GiB | fits |

- Threshold: two resident key sets leave `24 - 2 - 12 - 0.34 = 9.66 GiB` for all concurrent proof workspaces, i.e. ≤4.83 GiB each for two proofs. A measured per-proof peak above that breaks the plan, which is why one key uploaded at a time is the default.
- **Upload/eviction pricing (one key at a time):** switching keys evicts 6.0 GiB and uploads 6.0 GiB. Upload time is unmeasured on WSL2. Bounds: at the historical 12 GB/s pinned figure, about 0.52 s per switch; at the 63 GB/s theoretical Gen5 bound, about 0.10 s. Consecutive proofs with the same key keep the cosets resident, so the switch cost is paid only on key change. Pipelined upload behind the previous proof is possible but not measured.
- **Compute-per-use alternative (no device cosets):** each proof recomputes its coset set from coefficients: 48 forward coset NTTs of size 2^22, about 48 x 46.1M = 2.2e9 butterflies per proof (node 03 per-transform count, 46,137,344). Time is unmeasured. This trades 6 GiB of device memory for per-proof compute, and is the fallback when even one key set cannot be resident.
- **OOM handling (PROPOSAL):** check free device memory (`cudaMemGetInfo`) before admitting a proof. If the proof would not fit, wait; do not start it on partial reservations. A proof that fails mid-run is discarded and restarted from its beginning with fresh randomness (node 10 policy), never resumed from partial transcript state.

**Host, lazy path:**
- Host resident: 5.625 GiB (two keys' values + coefficients) + SRS 0.28 GiB + four witness workers at ~1.5 GiB each (ESTIMATE, unmeasured) = 6.0 GiB; total ≈ **11.9 GiB** plus allocator and process overhead (unmeasured).
- Derived coset sets are not kept on host. This is what makes the 12 GiB host figure disappear.
- **CPU fallback is not free:** a CPU-run proof needs the 6 GiB coset set per key on host (or recomputation per use). Enabling CPU fallback brings back the 12 GiB host term and can break B7. Recomputing cosets on CPU is the B7-safe option, at CPU cost.
- **Decode spike (critical):** `MidnightPK::read` in the current ledger path reads the whole PK into full forms (`midnight-proofs-0.8.3` PK fields, `src/plonk/mod.rs:338`; decode through `proofs.rs:250-300` in midnight-ledger). The lazy design requires a streaming decoder that stores only values and coefficients. With the current decoder, loading one 8.81 GiB key breaks B7 before any proving happens.

**Single ledger of residency (charge each buffer once):**
- Device: key-set cosets (6.0 GiB per resident key), SRS (0.28), twiddles (0.06), per-active-proof workspace (≤4.28 each, measured), context and reserve (measure).
- Host: key values + coefficients (2.81 per key), SRS (0.28), witness workers (~1.5 each, measured), CPU-fallback coset sets (6.0 per key, only if enabled, and then B7 is not satisfiable).
- Do not add PK full-form totals and device totals; a buffer lives in one place per phase. Node 01's 13.44 GiB is a sum that double-counts host and device copies if both are kept.

**Verdict:** the lazy same-key design is arithmetically feasible under B7 on paper (host ~12 GiB estimate, below 14.9 GiB), provided the decoder streams, witness workers stay near 1.5 GiB, and CPU fallback is recompute-only. It is not a B7 pass: B7 needs a measured peak under 16e9 bytes with swap 0, and the host currently shows 8 GB of swap in use. The 64 GiB host throughput test stays a separate resource experiment.
