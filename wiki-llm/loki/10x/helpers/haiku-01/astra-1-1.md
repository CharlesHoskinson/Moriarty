# astra-1-1: GPU prover benchmarks (BLS12-381 focus)

## Question
Fastest primary-published GPU full Plonkish/KZG provers relevant to BLS12-381 (cuZK, cuSnark, GZKP, ICICLE Halo2, Axiom, Ingonyama, Midnight PR157 / Nocy). Exact hardware, field, workload, full-prover vs MSM timings, transfer/keygen exclusions.

## Answer
- **No primary-published full Plonkish/KZG GPU prover timing on BLS12-381 was found.** Every full-prover number below is either on BN254, on an unstated curve, or from a Groth16 system.
- **Nocy (Midnight CUDA proof server)** is the only full-prover GPU number for a Midnight-stack prover found: RTX 5090 vs AMD EPYC 7642, k17 4.4 s vs 12.3 s, k18 8.8 s vs 24.6 s (self-reported, avg of 10 runs). The **curve is not stated** in the README or forum post. Midnight's `midnight-curves` is BLS12-381 (local `~/repos/midnight-zk/curves/Cargo.toml:3`), but that does not confirm Nocy's curve. Treat as unverified.
- **EZKL + ICICLE (Halo2 KZG aggregate, Nov 2023)**: ~98% MSM-time reduction (ICICLE MSM ~50x faster on average) and ~35% total aggregate-proof-time reduction vs CPU. GPU model and curve are not stated on the page. CPU baseline is AWS c6a.8xlarge (AMD EPYC 7R13). Likely BN254 (EZKL/halo2 KZG default), but this is my inference, not a source claim.
- **cuZK (TCHES 2023, ePrint 2022/1321)** is the best-documented BLS12-381 GPU result, but it is **Groth16, not Plonkish**. V100, BLS12-381. MSM 2^19–2^23 speedup 2.08x–2.29x; Groth16 end-to-end 2^20 4.45 s (bellperson) vs 1.68 s (cuZK), 2.65x. Component context only.
- **Midnight PR157** (ICICLE v4.0.0 CUDA POC) is **CLOSED, "[DO NOT MERGE]"**, with **no benchmark numbers** in body or comments. The author says the work moved to the `gpu-integration` branch of midnight-ledger, which is not present locally.
- **GZKP (ASPLOS 2023)**: abstract says up to 48.1x and 17.6x vs state-of-the-art on standard benchmarks and a real-world workload. Its baseline, hardware and curve are not captured (DOI page blocked by a challenge), so these are not usable as BLS12-381 claims.
- **cuSnark**: no such paper found. The closest match is cuZK. Not substituted.
- **Axiom / Scroll halo2 GPU**: no published CUDA/KZG timing found. Snarkify claims 1.25x over the nearest GPU competitor for a Scroll halo2 zkEVM stack, a vendor claim with no baseline or hardware given.
- **Ingonyama Medium post** (fast danksharding using ICICLE) returned 403 to Scrapling, so it was not captured.
- **ICICLE curve support**: the ICICLE repo lists bls12_381 among supported curves (`build.sh -curve=...`), and says official CUDA support is CUDA 12+ (raw/haiku01-icicle-repo.md).

## Raw numbers
| System | Source (URL + location) | Curve | Proof system | Workload | GPU | CPU | Timed scope | Transfers incl.? | Keygen/witness excluded? | Time / speedup | Notes |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Nocy proof server | forum.midnight.network/t/...-972 ; github.com/Nocy-io/nocy-gpu-proof-server README "Benchmarks" | not stated | Midnight/halo2-style Plonkish (variant of midnight-proofs 6.1.0-alpha.6, per README) | k17 / k18 | RTX 5090 | AMD EPYC 7642 | not stated | not stated | not stated | k17: 4.4 s vs 12.3 s; k18: 8.8 s vs 24.6 s; "2.8x" | Self-reported; avg of 10 runs; no independent review |
| EZKL + ICICLE | hackmd.io/@swezkl/HJRCV5MSp (sec. "Benchmarking Results"); hackmd.io/ftEdP5GgT5WxiN6r5ZaRuw | not stated (likely BN254, inferred) | Halo2 KZG aggregate | aggregate circuit (4 proofs) | not stated | AWS c6a.8xlarge, AMD EPYC 7R13 | aggregate command; MSM and total | not stated | not stated | MSM ~98% reduction (~50x avg); total aggregate ~35% reduction | Nov 2023; integration PR zkonduit/ezkl#586 |
| cuZK (MSM) | github.com/maciejskorski/cuZK README table; eprint.iacr.org/2022/1321 | BLS12-381 | Groth16 MSM | 2^19–2^23 | NVIDIA V100 (32 GB) | not stated | MSM only | not stated | not stated | 2^19 0.23 s → 0.12 s (2.08x); 2^22 1.30 s → 0.58 s (2.25x); 2^23 2.64 s → 1.15 s (2.29x) | Bellperson baseline; CUDA 11.5, Ubuntu 20.04 |
| cuZK (Groth16) | same README table | BLS12-381 | Groth16 end-to-end | 2^19–2^23 | V100 | not stated | full Groth16 prover (per README) | not stated | not stated | 2^20 4.45 s → 1.68 s (2.65x); 2^23 29.13 s → 9.91 s (2.94x) | Component/Groth16 context |
| cuZK (paper) | eprint 2022/1321 (captured) | BLS12-381 | Groth16 | — | — | — | MSM module / end-to-end | — | — | MSM >2.08x (up to 2.94x); end-to-end >2.65x (up to 4.86x); Filecoin 2.18x | Paper numbers; TCHES page blocked by Anubis |
| ZKProphet (study) | arxiv.org/html/2509.22684v1, Table II | BLS12-377 and BLS12-381 supported; test curve per library | Groth16 kernels | 2^15–2^26 MSM/NTT | NVIDIA A40 48 GB (primary); also V100/T4/3090/A100/L4/L40S/H100 for FF_mul | dual AMD EPYC 7742, 2 TB | MSM and NTT kernels vs CPU | not stated | not stated | MSM speedup 34.1x (2^15) to 799.5x (2^26, ymc); NTT up to 50.6x (2^22, cuZK) | Key finding: NTT up to 90% of latency once MSM is optimized |
| GZKP | elmi.hbku.edu.qa/.../gzkp-a-gpu-accelerated-zero-knowledge-proof-system/ (abstract) | not stated | ZKP (ASPLOS 2023) | not stated | not stated | not stated | not stated | not stated | not stated | up to 48.1x and 17.6x vs SOTA | Baseline/hardware not captured (DOI blocked) |
| Chimera (Spartan) | eprint.iacr.org/2026/1388 | BN254 | Spartan R1CS (not Plonkish) | N=2^17 sumcheck; 2^24–2^27 end-to-end | GPU (not named in capture) | not named in capture | sumcheck / end-to-end | not stated | not stated | sumcheck 7.99x at N=2^17; end-to-end 1.27x–1.64x (2^24–2^27) | Not Plonkish; out of scope except as context |
| Snarkify (Scroll halo2 zkEVM) | lu.ma/fz3x308c (talk page, search snippet only) | not stated | halo2 zkEVM | not stated | not stated | not stated | not stated | — | — | 1.25x vs nearest competitor (vendor claim) | Not captured; snippet only |

## Caveats
- **Curve not verified for the two Plonkish full-prover numbers (Nocy, EZKL).** Do not transfer their speedups to BLS12-381 without a re-run.
- **Hardware differs across rows** (V100 vs A40 vs RTX 5090); none are the same box, so cross-row ratios are not meaningful.
- **Transfer and keygen exclusions are not stated in any Plonkish source** found. Cannot be verified.
- **Captures that failed or were blocked**: Ingonyama Medium (403); cuZK TCHES page (Anubis challenge, `raw/haiku01-cuzk-tches-page.md` is a challenge page); GZKP DOI (Cloudflare challenge). Only the HBKU abstract was usable for GZKP.
- **Path deviation**: astra-1 asked for captures under `loki/10x/astra-1/raw/haiku01-*.md`. My standing rules restrict writes to `loki/10x/helpers/haiku-01/`, so all captures are in `loki/10x/helpers/haiku-01/raw/haiku01-*.md`. astra-1 should read from there.
- **Not found**: cuSnark (no such paper; closest is cuZK); Axiom halo2 GPU numbers; ICICLE-Halo2 BLS12-381 numbers (the icicle-halo2 repo itself was not located); the midnight-ledger gpu-integration branch (not local).
- **ePrint 2026/1729** (Hulek et al., unified benchmark framework) lists ICICLE and Halo2 among provers, but the abstract gives no per-system timings; PDF not captured.

## Captures (raw/)
haiku01-cuzk-eprint.md, haiku01-cuzk-repo.md, haiku01-cuzk-tches-page.md (blocked), haiku01-zkprophet-abs.md, haiku01-zkprophet-html.md, haiku01-eprint-2026-1729.md, haiku01-eprint-2026-1388.md, haiku01-ezkl-icicle-hackmd.md, haiku01-ezkl-benchmarks-hackmd.md, haiku01-ezkl-pr586.md, haiku01-icicle-repo.md, haiku01-halo2curves-issue84.md, haiku01-midnight-pr157.md, haiku01-nocy-forum.md, haiku01-nocy-repo.md, haiku01-gzkp-hbku.md, haiku01-gzkp-doi.md (blocked), haiku01-zkflex-abs.md, haiku01-ingonyama-danksharding.md (403).

## Verification note
- The cuZK paper-claim row (MSM >2.08x up to 2.94x; end-to-end 2.65x up to 4.86x; Filecoin 2.18x) comes from search-result summaries. The captured ePrint page (raw/haiku01-cuzk-eprint.md) contains only the abstract header and BibTeX, so re-check against the PDF before quoting.
- The GZKP 48.1x / 17.6x figures are confirmed from the HBKU abstract (raw/haiku01-gzkp-hbku.md, abstract line).

## Addendum (astra-1 follow-up): FluxZK, Impeller, DAG vs witness generation

### Answer
- **FluxZK (SJTU-Liquid, SC 2026)**: abstract (raw/haiku01-fluxzk-sc26.md) describes FluxMSM and FluxNTT on data-center GPUs, scaling to 2^30 elements, with an **"estimated" up-to-2.16x improvement** over existing solutions in real ZKP applications. Hardware, curve, baseline, and whether witness generation is included are **not** in the abstract. Its code is at github.com/SJTU-Liquid/FluxZK (referenced by the halo2 fork; not captured). Researchr listing has no abstract (raw/haiku01-fluxzk-researchr.md failed: SSL reset).
- **Impeller (SJTU-Liquid, ASPLOS 2027 artifact)**: compiler and runtime for graph-based ZK computation on heterogeneous CPU-GPU systems (raw/haiku01-impeller-repo.md). The README has **no timing numbers**. The companion halo2 fork (raw/haiku01-impeller-halo2.md) integrates it into Halo2 proof generation and calls `halo2_proofs::plonk::jit::create_proof_gpu`; it reuses FluxZK GPU kernels. The zkevm-circuits repo (raw/haiku01-impeller-zkevm.md) holds the zkEVM and aggregation workloads, with a "small configuration first" note for reproduction. **The Impeller paper's numbers were not reachable**; the paper itself is not captured.
- **Curve**: not stated in any Impeller/halo2/zkevm repo capture (grep for bn254/bls/curve returned nothing in those pages). Treat the Halo2 KZG curve as **unverified**.
- **DAG vs witness generation**: none of the captured pages separates polynomial-DAG time from circuit witness generation. The Impeller README says the compiler "fuses compatible operations" and that transfers are a bottleneck, but gives no split. Cannot answer this from sources; this needs the ASPLOS 2027 paper or a re-run with the zkevm-circuits benchmark (`cargo test -p zkevm-circuits --profile bench --bench benchmark_list -- super_circuit`).
- **Nocy contrary evidence**: confirmed in the README (raw/haiku01-nocy-repo.md): k17 4.4 s vs 12.3 s, k18 8.8 s vs 24.6 s, i.e. 2.795x and 2.8x, RTX 5090 vs EPYC 7642, 10-run average. Curve, witness scope and transfer scope not stated.
- **TensorZKP (ePrint 2026/1250, adjacent)**: GPU Tensor Core ZKP framework; at 2^25 gates the proof time is 215.28 ms per the abstract. Curve, hardware and baseline are not captured (raw/haiku01-eprint-2026-1250.md, abstract only). Mark as unverified and outside the BLS12-381 Plonkish scope unless the paper confirms the field.

### Raw numbers (addendum)
| System | Source | Curve | Proof system | Workload | GPU | CPU | Scope | Result | Notes |
|---|---|---|---|---|---|---|---|---|---|
| FluxZK | sc26.conference-program.com id=pap593 (abstract) | not stated | MSM/NTT kernels | up to 2^30 elements | "data-center GPUs" (model not stated) | not stated | MSM/NTT (not stated as full prover) | estimated up to 2.16x vs existing | "estimated" |
| Impeller | github.com/SJTU-Liquid/Impeller README | not stated | graph-compiled ZK (Halo2 integration) | zkEVM / aggregation workloads | CUDA 12.8 (paper setup) | not stated | not stated | no numbers in README | paper not captured |
| TensorZKP | eprint 2026/1250 (abstract) | not stated | sum-check / Spielman encoder | 2^25 gates | GPU Tensor Cores (model not captured) | not stated | full proof for 2^25-gate circuit | 215.28 ms proof; 0.85 ms inner product; 4.04 ms deg-2 sum-check; 11.58 ms encoder | Abstract only |
| Nocy | github.com/Nocy-io/nocy-gpu-proof-server README | not stated | Midnight Plonkish (variant of midnight-proofs 6.1.0-alpha.6) | k17 / k18 | RTX 5090 | EPYC 7642 | not stated | 4.4 s vs 12.3 s; 8.8 s vs 24.6 s (2.8x) | Self-reported |

### Addendum caveats
- The Impeller/FluxZK ASPLOS and SC papers were not captured. Their claims are unverified beyond the abstracts.
- SJTU-Liquid/halo2 and zkevm-circuits pages were captured from github.com main pages; not cloned.
- Sources for this addendum: raw/haiku01-fluxzk-sc26.md, raw/haiku01-impeller-repo.md, raw/haiku01-impeller-halo2.md, raw/haiku01-impeller-zkevm.md, raw/haiku01-impeller-halo2lib.md, raw/haiku01-sjtu-liquid-org.md, raw/haiku01-eprint-2026-1250.md, raw/haiku01-nocy-repo.md.
