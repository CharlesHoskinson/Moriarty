# Midnight-native PCD: reproduced measurements

Evidence for the [Moriarty PCD on Midnight report](../../deliverables/pcd-midnight-native-2026-09-11/REPORT.md) §14.2 and the [PCD roadmap](../../openspec/PCD-ROADMAP-2026-09-11.md). Measured on 2026-09-11 on one machine: Intel Core Ultra 7 365, 6 cores, 31 GiB RAM, WSL2 Linux 6.18. Treat every number as one data point on that machine.

| Measurement | Component | Log |
| --- | --- | --- |
| IVC example at K=18, 1, 10 and 100 steps, with negative controls | midnight-zk `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7` | `bench/logs/ivc_k18_n1000_s{1,10_neg,100}.log`; upstream K=17 failure in `ivc_n1000_s1.log` |
| `verify_proof` end-to-end, one level and two levels, with proving-side and verification-side controls | midnight-ledger pull request 738 head `416da99309cff5f953f029ff6c89211c00cf423c` | `bench/logs/e2e_verify_proof_*.log` |
| Offline proof-server proving of the Moriarty loan `initialize` circuit | `midnightntwrk/proof-server:8.1.0` | `bench/proof-server/` |

[MEASUREMENTS.md](MEASUREMENTS.md) records hardware, commits, exact commands, patches, SRS provenance, raw tables and what was not measured. `bench/logs/midnight-zk-ivc-bench.patch` and `bench/logs/verify_proof_e2e.orig.rs` are the only source changes, applied to benchmark copies to add timing and negative controls. Build trees, the 411 MB IVC proving key, the loan proving key and SRS files were not retained; their hashes are recorded.

Scope limits:

- None of these proofs was submitted to or accepted by a Midnight network.
- The pull request 738 tests generate an in-process test SRS.
- The proof-server proofs were not verified.
- Branch, join, stale-state and multi-party scenarios were not measured.

[source-pins.json](source-pins.json) lists the implementation commits and versions inspected for the report.
