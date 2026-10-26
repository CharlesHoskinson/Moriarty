# s6-opening wave 1 — research leads

OPEN QUESTION — Investigate these nine concrete leads; source acquisition and analysis only, with no proving benchmarks claimed.

| Lead | Primary acquisition | Question that could change the decision |
| --- | --- | --- |
| 1. Midnight fflonk integration | midnightntwrk/midnight-zk PRs 565 and 575; helper 1 | Does t=4 save recursive verifier rows, and what does it cost the prover and SRS? |
| 2. SHPLONK algebra and implementation | ePrint 2020/081; PSE SHPLONK prover; helper 2 | How many large commitments and transcript barriers are unavoidable? |
| 3. Original fflonk construction | ePrint 2021/1167 | Is commitment packing a verifier optimization rather than a prover speedup? |
| 4. Resident GPU multiopen | axiom-crypto/halo2-gpu tree, SHPLONK prover and CUDA kernels | Which field operations can stay resident between the two opening commitments? |
| 5. Same-base batched MSM | ICICLE MSM API; helper 2 | Can child batches share SRS/precompute without altering their independent transcripts? |
| 6. FK20 amortized proofs | ePrint 2023/033 | Does its many-points/same-polynomial workload match two/four child proofs? |
| 7. Evaluation-form KZG | ethereum/c-kzg-4844 source | Can quotient evaluation and Lagrange-SRS MSM avoid coefficient conversion? |
| 8. Parallel polynomial division | ePrint 2024/1279 and scan algebra | Can linear-factor division use an associative scan instead of a serial recurrence? |
| 9. Cross-proof accumulator batching | snark-verifier KZG accumulation source | What work does batching actually eliminate, and which statement/transcript bindings must remain? |

SOURCE — Earlier studies read from the corrected path `../loki/nodes/loki-04-halo2-gpu-precedents.md` and `../loki/10x/LOKI-10X.md`; archived at `raw/s6-opening/prior-loki04.md` and `raw/s6-opening/prior-loki10x.md`. Existing GPU surveys are background, not new evidence or host measurements.
