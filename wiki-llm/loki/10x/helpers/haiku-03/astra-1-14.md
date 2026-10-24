# astra-1-14: MegaZK (ICS 2026) audit

## Status
**PRIMARY TEXT NOT CAPTURED.** The ACM DL page (`dl.acm.org/doi/10.1145/3797905.3807842`, and `/doi/full/`) returns a Cloudflare bot challenge to Scrapling (both `get` and `fetch`), and WebFetch gets HTTP 403. No arXiv or ePrint preprint and no artifact repository turned up in two searches. The abstract is not in Crossref. So the protocol, curve, absolute timings, sizes, baseline, and offload list cannot be verified from a primary source. Nothing below is a measured number from the paper.

## Verified (primary metadata, Crossref)
- Title: "MegaZK: A Memory Efficient GPU System Accelerating End-to-end Zero-Knowledge Proof".
- Venue: Proceedings of the 40th ACM International Conference on Supercomputing (ICS 2026), pp. 1165-1176, published 2026-07-05. DOI 10.1145/3797905.3807842.
- Authors (Crossref order): Muyang Li, Yueteng Yu, Bangyan Wang, Xiong Fan, Mingyu Gao, Shuwen Deng.
- Capture: `helpers/haiku-03/raw/h3d-megazk-crossref.json` (metadata only; no abstract).

## Secondhand (search-engine summary of the ACM abstract; NOT verified against the paper)
- Target: "Plonkish+FRI proof system" (FRI, not KZG).
- GPU: NVIDIA RTX 4090 (not 5090).
- Claims: 37.4x average end-to-end speedup over state-of-the-art CPU implementations; 24.3x average over existing GPU approaches; GPU memory reduced by 80.9%.
- Mechanisms named: "in-place segmented computing" to remove duplicated data in low-degree extensions; "Type-Aware Layered Scheduling" for witness generation; the summary says it covers witness generation.
- Treat all of the above as unverified until the paper text is read.

## Relevance to Midnight/Sherlock (conditional on the summary being right)
- **Protocol mismatch (main point).** Plonkish+FRI is not Midnight's BLS12-381 KZG/PLONK. Its FRI low-degree extension and memory tricks do not transfer to the KZG extended-domain quotient path in loki-03 without re-deriving them. Its speedups are not BLS12-381 MSM/NTT evidence.
- **Hardware.** RTX 4090, not 5090 (sm_89 vs sm_120). Do not carry its numbers over as 5090 numbers.
- **Headline numbers are not usable as ratios for Sherlock.** The 37.4x is an end-to-end speedup against an unspecified CPU baseline, for an unspecified circuit size. The 24.3x is against GPU baselines whose identities are unknown. Per the summary, the 80.9% memory cut comes from FRI LDE deduplication. Sherlock's KZG path would need a different memory argument.
- **Witness inclusion (per summary).** It claims to include witness generation, which is the piece the other leads flag as the serial tail. If verified, it is the most relevant new evidence on witness scheduling. Confirm from the paper before relying on it.

## Needed to close this
1. Read the full ACM paper: protocol and curve, absolute timings and proof sizes, baseline CPU and thread count, GPU driver, which operators run on GPU and which stay on host (witness, sorts, transcript), memory table.
2. Alternatives if the ACM page stays blocked: an institutional or browser session, the authors' pages, or any artifact repository the paper names. I did not try these; I don't have a browser tool loaded in this run.

## Caveats
- No absolute numbers, curve, or host-tail data from the paper were verified. Everything beyond the Crossref metadata is secondhand and must not be cited as measured.
- The search-engine summary also could not confirm the author list beyond what Crossref gives.
