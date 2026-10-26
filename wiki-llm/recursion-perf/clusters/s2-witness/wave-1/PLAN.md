# s2-witness wave 1 research plan

OPEN QUESTION — Investigate these eight leads; source availability and relevance decide their final rank. No implementation, installation, proof benchmark, commit, or deployment is part of this research pass.

1. Midnight 0.8.3 floor planner and advice collection: distinguish duplicate structural traversal from duplicate arithmetic; identify a key-bound layout replay seam.
2. Axiom halo2-base virtual regions: deterministic parallel contexts and witness-only mode as a design precedent, without importing its gates or keys.
3. gnark solver: dependency levels, minimum task grain, and native hints as a concrete multicore scheduler precedent.
4. circom-witnesscalc plus Circom native parallel components and Mopro benchmarks: compiled witness DAGs versus opaque closure execution; isolate workload and baseline differences.
5. powdr-legacy JIT witness generation: ahead-of-time operation scheduling and generated machine code; check field-size and fallback restrictions.
6. ff batch inversion plus ICICLE custom field programs: move inversions to dependency frontiers and fuse bulk witness operations, retaining original cell outputs.
7. BLST/MCL SIMD capabilities: establish whether a BLS12-381 Fr vector backend exists for this host's instruction set; reject small-field and AVX-512-only analogies.
8. Actual G5 K-A stream: measure the potential to eliminate JSON/display/digest materialization and repeated adjacent snapshots using a typed layout sink; establish whether this can affect parent witness time at all.

SOURCE — Earlier studies read: `../../wiki-llm/loki/nodes/loki-09-cpu-witness-field.md` (the requested `10x/loki-09-cpu-witness-field.md` does not exist) and `../../wiki-llm/loki/10x/astra-1/PROPOSAL.md`. They already propose lazy inversion, sparse rational buffers, and a specialized witness compiler. Wave 1 must sharpen or refute these, not present them as discoveries.

ESTIMATE — All speed bounds will use explicit Amdahl fractions. The recorded cuSnark 363.28 s is a combined witness/advice-plus-lookup-preparation bucket, not measured Sherlock witness time. Separate parent, full chain, and throughput claims.
