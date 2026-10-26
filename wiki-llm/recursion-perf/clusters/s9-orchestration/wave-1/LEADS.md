# Wave 1 lead plan

All entries are OPEN QUESTION research leads, not speedup claims. Source snapshots are under `raw/s9-orchestration/`. No prover benchmark is authorized or run in this research pass.

| # | Concrete source / owner | Question and falsifier |
|---|---|---|
| 1 | Earlier Loki scheduling/Astra model plus actual B3/B5 command records / lead | Does the historical denominator describe current latency or steady throughput? Reconcile workload identity and prefix count before computing ratios. |
| 2 | NVIDIA Nsight Systems guide + Linux cgroup-v2 / helper 2 and lead | Can an L0 event trace account for CPU core-seconds, GPU busy intervals, queue waits and both memory spaces without destroying overlap? |
| 3 | StarPU features and data-aware scheduling / lead | Would measured earliest-finish scheduling and memory-release priority beat static parent-first scheduling? Adopt mechanisms, not a new runtime dependency. |
| 4 | CUDA asynchronous execution guide / lead | Do event-driven recurrent CPU/GPU phases and bounded pinned transfers remove bubbles? Reject if transfer/compute overlap or resource headroom is absent. |
| 5 | CUDA stream-ordered allocator blog / helper 2 | Can phase-specific lifetimes reuse one heavy workspace and prevent admission deadlocks? Separate retained pool memory from live allocations. |
| 6 | CUDA constant-time graph launch benchmark / helper 2 | Is launch overhead material at k20, and can stable regions between transcript challenges be captured? |
| 7 | CUDA green-context scheduling benchmark, 2026-10-06 / lead | Would SM/workqueue partitioning reduce terminal tail latency on the installed driver? Reject throughput credit without bandwidth and coexistence measurements. |
| 8 | DIZK paper/repository / helper 1 | Is distributing unchanged arithmetic viable at k20 after network, memory and coordination costs? Identify original proof family and machine count. |
| 9 | zkBridge/deVirgo paper / helper 1 | Does data-parallel intra-proof distribution transfer to fixed PLONK/KZG? Reject direct protocol substitution. |
| 10 | Boundless protocol/repository / helper 1 | Can request leases, deadlines and verified completion improve a native worker pool? Distinguish marketplace dispatch from proof-format compatibility. |

SOURCE discovery note: supplied `loki/10x/loki-08-recursion-tree-scheduling.md` was absent; the existing source is `loki/nodes/loki-08-recursion-tree-scheduling.md`. The initial CUDA streams URL returned HTTP 404 and was replaced by the successfully fetched asynchronous-execution guide. The old Boundless documentation domain failed TLS validation; the official repository identified `docs.boundless.network`, which fetched successfully. Native Offer/callback source supplies deadline and verifier details. The DIZK PDF was fetched with Scrapling, but local PDF conversion was unavailable; selected protocol/setup source text was recovered with the public web PDF reader into `raw/s9-orchestration/dizk-paper-excerpts.md`, and the temporary PDF was removed so every raw file retains the required textual header. No TLS bypass, installation, commit, or proof execution.

SOURCE method note: both named AgentBus helpers contributed raw captures. Their summaries were treated as data and checked against the captures before adoption. A Muse Wonder brainstorm was requested through the lead; it had not arrived at the final source audit, so no finding depends on it.
