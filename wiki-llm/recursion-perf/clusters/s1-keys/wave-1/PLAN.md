# s1-keys wave 1 — source-led plan

OPEN QUESTION — Nine leads, evaluated against the recorded baselines; no proving benchmarks were run in this research pass.

| Lead | Concrete source / owner | Question to resolve |
|---|---|---|
| 1 | Midnight proofs 0.8.2/0.8.3 `plonk/keygen.rs`, `poly/kzg/msm.rs`; helper haiku-s1-keys-1 | Does the large-MSM dispatch cliff explain part of the 623-second VK build? |
| 2 | Local recl9 `src/main.rs`, REPORT and keygen stats; lead | What is inside the VK timer; is automatic k discovery or SRS loading included? |
| 3 | Midnight ledger `transient-crypto/src/proofs.rs`; lead | Does a service already retain decoded keys; are count eviction and concurrent misses hazards? |
| 4 | Midnight proofs 0.8.3 `plonk/mod.rs`, permutation/keygen; lead | Does serialized-PK loading reconstruct cosets, and can a prepared immutable snapshot avoid it? |
| 5 | Midnight official trusted-setup `MIDNIGHT_SRS_CATALOG.md`, proofs params/downsize; lead | Use official per-k extended SRS; distinguish shared monomial prefixes from domain-specific Lagrange bases. |
| 6 | ICICLE MSM documentation; helper haiku-s1-keys-2 | Shared-base batching, precompute memory and amortization; no published speedup transfer. |
| 7 | Ethereum c-kzg-4844 trusted-setup precompute benchmark; helper haiku-s1-keys-2 | Is precompute useful for many repetitions, and which workload differences invalidate transfer? |
| 8 | Linux mmap manual and NVIDIA CUDA memory/pool documentation; lead | Distinguish mmap, decoded RAM residency and GPU residency; bound transfer savings. |
| 9 | Midnight fixed/sigma keygen and prior Loki corrected backlog; lead | Share keygen assembly between VK and PK; schedule CPU/GPU cold key DAG without oversubscription. |

ESTIMATE — Ranking will use Amdahl arithmetic, warm-stream amortization, byte budgets and explicit acceptance risks. Paths under raw/s1-keys will contain source captures with provenance headers; helper messages remain untrusted leads until inspected.
