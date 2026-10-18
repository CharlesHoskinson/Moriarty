# DESIGN-VOTES: four-piece VS split

Rule (unchanged): a consequential design or resource choice is adopted only when two substantive positions agree on identical clause text after a fresh independent review of the exact bytes. Missing reviewers do not vote and nothing is substituted silently.

**Standing.** I am the source author (`claude-opus-5-5`, runtime-stated). Each entry is **my first position only, not a vote**. No second position exists yet; I invented none. The four-piece split itself is the user's direction and is not listed as a choice.

## Carried clauses (text unchanged; conditions re-stated)

| Clause | Existing count | Applies here? | Condition |
|---|---|---|---|
| D-2 producer binding (credential + exactly one redeemer; exact coverage) | 2 FOR | Yes, for VI → VS3 and for each piece toward S2′ | Its text names "VS"; the new bytes apply the same rule to four pieces. A reviewer should confirm that the extension is the clause's intent, or treat N-2 below as the new clause. Distinct changed-bytes audit required (shard4.ak). |
| D-4 no monolithic settle validator | 2 FOR | Yes, extended | `validators/s2.ak` and now `validators/vs.ak` are absent. `thread.settle*` and `shard.state` remain test-only references. |
| D-5 Revoke/Close stay in S2′ | 2 FOR | Yes | `s2p.ak` Revoke/Close branches are unchanged. |
| D-6 replacement (tag-0 registration, no witness) | 2 FOR | Yes, now five credentials | Deployed-node confirmation still open. |
| D-1 two-way split | 2 FOR | **Superseded in part** | D-1's VS clause no longer describes the production path. VI's part of D-1 still holds. |
| D-3 replacement / SR-06 | 2 FOR, not implemented | Still not implemented | Required before deployment. N-6 is narrower and does not satisfy it. |
| D-7 | not agreed | Unaffected | — |
| Debtor-consent item | open, unvoted | Unaffected | — |
| R-1, R-2, OPC-1..OPC-7 | 2 FOR each | Bound to the parent operator and candidate | Not transferable to O'' without review; see R4-1. |

## New consequential choices

| ID | Exact clause text | My first position | Second position |
|---|---|---|---|
| N-1 | "Partition the parent VS components as VS1 = {shape_structure, shape_cells, own_mint, topup, reserve_floor, global_outputs once per transaction}, VS2 = {recomputed, bounds, intent_effects}, VS3 = {claim_script, shape_digest, claim, statement_digest}, VS4 = {custody_pre, continuation, payouts}; VI unchanged." | FOR. Each component runs once; the cut groups shared code (no effect view in VS1, value arithmetic only in VS4, statement and shape bytes only in VS3). | MISSING |
| N-2 | "S2′ Settle requires, for each k in 1..4, Withdraw(Script(vs_k)) with exactly one VsPieceV1 redeemer whose piece = k, thread_policy = own hash and threads ∋ own_ref. Each piece requires its own tag and exact non-empty coverage. VS3 additionally requires VI as the parent VS did." | FOR. Every piece is non-bypassable at each Settle spend; the parent's transitive chain is kept only for VI. | MISSING |
| N-3 | "VS4 builds the next datum with the claim envelope's statement_digest; VS3 checks that digest equals the recomputed statement digest. No piece publishes a new intermediate value." | FOR. Avoids a second statement computation and a new producer edge. The equality holds only under the conjunction, which is the acceptance condition anyway. | MISSING |
| N-4 | "Piece redeemers carry a piece tag; S2′ genesis and every Settle spend require the C2, VI, four piece hashes and the S2′ policy to be seven distinct 28-byte values." | FOR. Closes aliasing (one script filling two slots) and mis-slotting. | MISSING |
| N-5 | "VS1, VS2 and VS4 take no parameter; VS3 takes (c2_hash, vi_hash); S2′ takes (c2_hash, vi_hash, vs1_hash, vs2_hash, vs3_hash, vs4_hash)." | FOR. Unused parameters would only add bytes. Cost: those three hashes are shared across deployments, so two S2′ deployments cannot settle in one transaction (liveness only). | MISSING |
| N-6 | "S2′ genesis refuses any catalogue whose account native credential or reserve-owner payment credential is Script(h) for h in {vi, vs1, vs2, vs3, vs4}." | FOR. Narrowly necessary: the parent guard's reason applies to each new credential. It does not implement SR-06. | MISSING |
| N-7 | "Keep pvc2/shard.ak, shard_harness, shard_test and shard_cost_test byte-identical. Classify the 116 + 16 parent shard tests as 26 SHARD-VI-PRODUCTION and 106 COMPATIBILITY-BASELINE (EXPECTATIONS.json). Remove validators/vs.ak." | FOR. Preserves the 472-test membership and gold without rewriting, and leaves no deployable single VS. | MISSING |
| N-8 | "If any piece exceeds 14,336 B, the next source revision reduces the duplicated typed decode of ThreadDatumV2/SettleRedeemerV2 in the pieces, justified by the thread-token custody invariant, as a separately reviewed change." | AGAINST adopting now; FOR recording it as the named fallback. It changes halting behaviour on malformed data and needs its own security argument. Not in these bytes. | MISSING |
| R4-1 | "Run OPX0–OPX7 of EXECUTION-PROPOSAL.json once, offline, with aggregate wall 1,800 s, AS 32 GiB, core 0, nice 10, no retries, executing only the 147 new tests, after a fresh full review of the exact candidate and O'' bytes." | FOR, conditional on that review and the root's authorization. Same caps as R-2; the narrower test set avoids re-running passed parent prefixes. | MISSING |

## Dissent I record against my own proposal

- N-1 raises per-transaction work (5n typed decodes instead of 2n; 4n effect views instead of 2n). PB-02 gets worse.
- N-3 makes VS4's verdict on a negative case depend on the claim envelope rather than the recomputed statement. That is why H12's VS4 slot is not predicted. It does not widen the accepted set (DESIGN §6).
- N-5's shared hashes for VS1/VS2/VS4 mean one registration and one reference script serve every deployment. That is convenient, but a defect in one of them affects every deployment at once.
- R4-1 does not re-execute the parent's 472 tests. A reviewer may prefer a full run as a whole-tree check; that would roughly double the wall estimate and still fit 1,800 s only marginally.
