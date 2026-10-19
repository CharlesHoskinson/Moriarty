# Current implementation order

Use the existing DoctorWatson loop and local CI. Source preparation may overlap; Root integrates and selects actual native or wallet operations. All checkboxes below refer to observed outcomes, not authored plans.

- [x] Refresh/query the current Sherlock Graphify source layer and latest proof-result addendum.
- [x] Recheck all 206 scoped Rust source hashes and preserve historical graph layers.
- [x] Inspect Fenrir's ledger-9 strawman brief, deterministic history generator/schema and decider source.
- [ ] Finish fresh Opus source audit, compilation and independent verification of the saved literal bundle through P14; preserve its literal-only scope.
- [x] Pin and inspect `recl9` lib/inner/stage/leaf/main, the inner adapter and `decider-route.mjs`; record the public SDK Blake2b versus child-Poseidon interface gap. `node.rs`, `decider.rs`, the StrawAnchor contract and the route's selected decider lineage remain unpinned. No compatibility is assumed.
- [ ] Pin the `recl9` node/decider relations and the one selected decider/wrapper lineage used by the route; resolve the SDK Blake2b versus Fenrir child-Poseidon transcript and private-VK interface; implement the missing Sherlock recursive relation/composition/terminal consumer with current designated ownership and reviews.
- [ ] Freeze independent minimal recursive and Fenrir-derived workload expectations before execution.
- [ ] Implement/adapt the deterministic dataset producer and independently replay its complete state transitions; exercise the current 16-transaction staging contract. Treat eight transactions as an optional separate adapter/local contrast with no presumed fixed-anchor settlement.
- [ ] Measure actual per-layer k, rows where exposed, proof/key sizes, parameter/keygen/prove/verify wall/CPU and RSS on the first recursive fixture.
- [ ] Rehearse real ledger-9 acceptance locally with retained positive and negative obligations.
- [ ] Confirm current Midnight stagenet/package compatibility, selected wallet balance and DUST, and deploy the actual Sherlock verifier/anchor integration.
- [ ] Validate a real recursive proof on stagenet and independently read back the complete settled effects and public receipts.
- [ ] Benchmark one sixteen-transaction full batch and two/four/six-batch prefixes as supported and eligible; keep any optional eight-transaction adapter experiment separate; preserve all failures/censored measurements.
- [ ] Publish the workload/adapter source, exact corpus and measurement manifest, raw results and scoped feasibility report; update Graphify, Diátaxis, roadmap and DoctorWatson lessons.

Next source task: the reviewed P14 verify-only prerequisite, followed by the actual recursive-path gap. Source drafts or infrastructure checks do not complete deployment, recursive verification or benchmarking. No existing resource class is extended here.
