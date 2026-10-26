# Wave 1 leads

OPEN QUESTION — Plan before source collection; all are research leads, not measured gains.

1. cuSnark original quotient breakdown: separate reported quotient evaluation from NTT, recover circuit dimensions and GPU count. https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover
2. Snarkify BLS12-381 benchmark implementation: check timing envelope and warm-up; establish what 1.13 ms excludes. https://github.com/snarkify
3. Pipelonk full paper/artifact: determine whether segmentation helps a fitting 32 GB workload and respects challenge dependencies. https://doi.org/10.1145/3774934.3786448 — delegated to helper 1.
4. Midnight exact domain/constraint code: degree thresholds, periodic factors, and existing pruning. https://github.com/midnightntwrk/midnight-zk — delegated domain audit to helper 2; parent follows pruned FFT.
5. ICICLE NTT API: device residency, batches, ordering and twiddle-memory tradeoffs. https://github.com/ingonyama-zk/icicle/blob/main/docs/versioned_docs/version-4.0.0/api/cpp/ntt.md
6. sppark LDE implementation: input-zero pruning versus full padded transforms. https://github.com/supranational/sppark/tree/main/ntt
7. NVIDIA cuPQC device NTT: exact modulus/size limits before considering fusion. https://docs.nvidia.com/cuda/cupqc/libraries/cupqc_ntt/cupqc_ntt_feature.html
8. ZKProphet full text: distinguish integer arithmetic/occupancy limits from bandwidth and launch overhead. https://arxiv.org/html/2509.22684v1
9. Independent BLS12-381 consumer-GPU negative results: register pressure, four-step NTT and Tensor Core feasibility. https://github.com/Artemarius/cuda-zkp-ntt
10. CUDA execution guidance: pinned transfers, streams, CUDA Graphs and causal overlap bounds. https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html
