Open source drop: CUDA-accelerated Midnight proof server (nocy GPU Proof Server) - Ecosystem - Midnight Network
[Midnight Network](/)

[Open source drop: CUDA-accelerated Midnight proof server (nocy GPU Proof Server)](/t/open-source-drop-cuda-accelerated-midnight-proof-server-nocy-gpu-proof-server/972)
========================================================================================================================================================================

[Ecosystem](/c/ecosystem/7)

[ecosytem](https://forum.midnight.network/tag/ecosytem),
[community](https://forum.midnight.network/tag/community)

[nocy](https://forum.midnight.network/u/nocy)


January 20, 2026, 10:08am
1

Hey Midnight community ![:waving_hand:](https://emoji.discourse-cdn.com/twitter/waving_hand.png?v=15 ":waving_hand:")

We just open-sourced a new piece of infra we think is *directly* tied to whether programmable privacy feels fast in practice:

**nocy GPU Proof Server** - a Midnight proof server adapted for **GPU proving via CUDA**, benchmarked on an **NVIDIA GeForce RTX 5090**.

Repo: [https://github.com/Nocy-io/nocy-gpu-proof-server](https://github.com/Nocy-io/nocy-gpu-proof-server?utm_source=chatgpt.com)

### Why this matters

In privacy-native apps, **proof generation latency** is one of the biggest UX bottlenecks. Speeding up proving doesn’t change the privacy model, it lowers the “privacy tax” so real flows feel responsive.

Who benefits:

* **Users:** shielded actions feel lighter (less waiting on proofs)
* **Builders:** privacy apps stop feeling “proof-heavy”
* **Infra providers:** a clearer path to run high-performance proving as part of a stack

### Benchmarks (from our initial setup)

The README includes a comparison between **RTX 5090 (GPU)** and **AMD EPYC 7642 (CPU)**, averaged over 10 runs:

* **k17:** 4.4s (GPU) vs 12.3s (CPU)
* **k18:** 8.8s (GPU) vs 24.6s (CPU)  
  That’s **~2.8× faster** proving vs CPU-only in this setup.

### API routes

The server exposes:

* `POST /prove` — generate a zero-knowledge proof
* `POST /check` — validate preimages
* `GET /health` — health check

### Related open source (already public)

We’re doing staged OSS and already have wallet infra public:

* [https://github.com/Nocy-io/nocy-wallet-indexer](https://github.com/Nocy-io/nocy-wallet-indexer?utm_source=chatgpt.com)
* [https://github.com/Nocy-io/nocy-wallet-sidecar](https://github.com/Nocy-io/nocy-wallet-sidecar?utm_source=chatgpt.com)

### What we’d love from the community

If you have different GPUs / driver versions / server setups, we’d really value:

* benchmark comparisons (and circuit sizes tested)
* build notes (what worked cleanly)
* issues/PRs for stability + reproducibility

2 Likes

[Spy](https://forum.midnight.network/u/Spy)


January 21, 2026, 6:15pm
2

This is WOW thank you for directing me to this, I appreciate it.

### Related topics

| Topic |  | Replies | Views | Activity |
| --- | --- | --- | --- | --- |
| [Please use `proof-server` version `4.0.0`](https://forum.midnight.network/t/please-use-proof-server-version-4-0-0/727)  [Updates](/c/updates/8) | 0 | 180 | December 2, 2025 |
| [Nocy on Midnight Preview: Wallet + Explorer + CLOB Markets (Live) — Quickstart + Faucet + OSS Updates](https://forum.midnight.network/t/nocy-on-midnight-preview-wallet-explorer-clob-markets-live-quickstart-faucet-oss-updates/962)  [Ecosystem](/c/ecosystem/7) [ecosytem](https://forum.midnight.network/tag/ecosytem/6) ,  [community](https://forum.midnight.network/tag/community/7) | 0 | 190 | January 14, 2026 |
| [Problem with Proof-Server](https://forum.midnight.network/t/problem-with-proof-server/111)  [Technical Questions](/c/technical-questions/5) [learning](https://forum.midnight.network/tag/learning/3) | 7 | 147 | August 4, 2025 |
| [Shielded kernel operations (receiveShielded, mintShieldedToken) fail with proof server 400](https://forum.midnight.network/t/shielded-kernel-operations-receiveshielded-mintshieldedtoken-fail-with-proof-server-400/1137)  [Technical Questions](/c/technical-questions/5) [compact](https://forum.midnight.network/tag/compact/2) ,  [ecosytem](https://forum.midnight.network/tag/ecosytem/6) | 2 | 64 | April 14, 2026 |
| [Proof Verification on Midnight](https://forum.midnight.network/t/proof-verification-on-midnight/219)  [Technical Questions](/c/technical-questions/5) [compact](https://forum.midnight.network/tag/compact/2) ,  [learning](https://forum.midnight.network/tag/learning/3) | 1 | 151 | September 18, 2025 |

* [Home](/)
* [Categories](/categories)
* [Guidelines](/guidelines)

Powered by [Discourse](https://www.discourse.org), best viewed with JavaScript enabled