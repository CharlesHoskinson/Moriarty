# Wave 2 research plan

**SOURCE:** This wave starts from wave-1/FINDINGS.md and the resolved Loki study at `../loki/nodes/loki-10-integration-verification.md`; the requested `10x/` path is absent. Existing observations are historical, not new measurements.

| Lead | Concrete source / question | Owner |
|---|---|---|
| 1 | Pinned midnight-proofs 0.8.3 `plonk/mod.rs`: what preparation is retained in a PK? | Lead, Scrapling |
| 2 | Pinned 0.8.3 `poly/kzg/params.rs`: coefficient/Lagrange SRS residency and representations | Lead, Scrapling |
| 3 | Pinned stdlib 2.3.5: key serialization, architecture, relation identity and row sizing | Lead, prior capture plus source inspection |
| 4 | Private recl9 and stand-in reports: separate recorded PK-stage RSS from prepared-key allocation; row census | Lead, summary only with git hash |
| 5 | Midnight manage-DUST guidance: reservations, TTL and kept-synced wallet | Helper 1 via AgentBus |
| 6 | systemd resource-control: hard/soft caps for persistent worker | Helper 2 via AgentBus |
| 7 | Linux unix(7): authenticated local callers using peer credentials | Lead, Scrapling |
| 8 | Midnight wallet-sdk repository: sync lifecycle and state persistence | Helper 1 via AgentBus |
| 9 | CUDA programming guide: persistent contexts/device memory and asynchronous transfer limits | Helper 2 via AgentBus |

**OPEN QUESTION:** No implementation, new proving run or stagenet contact is authorized in this wave. Design and offline source/arithmetic audits only. Resolve C10 into an explicit process budget, authenticated manifest protocol, and failure/retry state machine. Refute k19 from total rows if the existing circuit exceeds capacity; distinguish future feasibility from current feasibility.

**SOURCE — Collection outcome:** wallet-sdk URL returned 404; the resolved official Node wallet service guide was fetched instead. CUDA host-memory URL returned 404; the helper fetched the official understanding-memory chapter. Two overlapping AgentBus waits lost their MQTT connections; both replies were subsequently recovered, and subsequent bus calls were sequential. These are collection failures, not proving failures. The private VK inspection only decoded public artifact header/count metadata and wrote summaries, not source text or witnesses. No graph build, install, factory check, benchmark, commit or stagenet action was performed.
