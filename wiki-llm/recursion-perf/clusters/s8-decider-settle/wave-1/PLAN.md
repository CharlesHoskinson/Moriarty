# Wave 1 source plan

**OPEN QUESTION —** The nine leads below were the initial research plan.

**SOURCE —** No new performance measurements are claimed; conclusions and captures are indexed in FINDINGS.md and SOURCES.md.

1. Residue-check improvements beyond the existing gadget: https://eprint.iacr.org/2024/640 and https://github.com/Consensys/gnark/tree/master/std/algebra/emulated/sw_bls12381 (helper 1).
2. Ledger-reconstructible architecture and range-table floor: https://github.com/midnightntwrk/midnight-zk ; compare pinned stdlib 2.3.5 and local cost reports (lead).
3. Extension-field arithmetic, sparse line multiplication, delayed normalization: https://github.com/Consensys/gnark/tree/master/std/algebra/emulated/fields_bls12381 plus local tower/flat implementation (lead).
4. Fuse final node/parent with decider: https://github.com/midnightntwrk/midnight-zk/tree/main/aggregation ; measure removed verifier against domain growth (lead).
5. Terminal-proof service residency and proof-server version/caching: https://docs.midnight.network/develop/tutorial/using/proof-server (helper 2), pinned local entrypoints (lead).
6. DUST fee-bearing proof multiplicity, registration exception, sponsorship: https://docs.midnight.network/guides/manage-dust and https://midnight.network/blog/dust-sponsorship-on-midnight (helper 2).
7. Official SRS, transcript and verifier architecture envelope: https://github.com/midnightntwrk/midnight-ledger and pinned ledger-9 sources (lead).
8. GPU complement after structural reductions: https://github.com/supranational/sppark ; compare earlier Loki Amdahl model, do not repeat its integration design (lead).
9. Distinguish stock verifier harness overhead from contract verification; amortized batch settlement versus latency and finality: recorded A4/B4 and recl9 reports plus current wallet transaction pipeline (lead).

**SOURCE —** Writes confined to raw/s8-decider-settle and clusters/s8-decider-settle. No build, benchmark, installation, wallet operation, commit or submission in this research wave. Earlier study supplied at loki/10x was absent; read loki/nodes/loki-10-integration-verification.md instead.
