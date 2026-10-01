# Independent experiment test contract — 2026-09-30

Status: **specified-only**, independent preimplementation review. No experiment was run by this reviewer. This is an executable local authentication experiment contract, not adoption of a signed intent codec or product acceptance. Reviewer: delegated Codex agent `crypto_test_contract`; no independent provider/model approval is asserted.

Repository observations: Source/6 contract and `financial-agreement-source-v6-frontend.ts`, `mil4-s0-core-v5.ts`, and `mil4-s0-source-v6.ts` were inspected. Guarded status has no pending transactions; existing loan campaign remains blocked by stale admission inputs and unavailable accounting/live state. Read-only review does not dispatch that campaign. Published authoring beta remains `PreparedUnqualified`.

## Evidence and fixture contract

Recommendation: pin the official ledger checkout to full commit `9f9842ebed66cdff0f54d3fb09efc6a7cd077ed8` and record remote URL, branch, HEAD, clean/dirty status, lockfile digest, Rust version, exact commands, exit codes and test counts. The parent must verify acquisition; this reviewer did not verify that pin. The prospective zk pin is `0ededef0e605701fc5139ebdcf011b11f3d86ba7`. Never silently replace an unavailable crate or backend with a mock.

Generate transfer-with-fee, transfer-with-zero-fee and funded AccrualFirst repayment fixtures through actual beta analysis/lowering and Source/6 parsing. Export full AST, lowered intent, state, submitted effects, proposed head and prepared candidate. Freeze fixture hashes and independently assert economic values, effect order, allowance/work deltas and obligation changes before signing. Do not handwrite an unrelated message and call it a Moriarty intent test. Include repayment below accrued, across accrued and full settlement if fixtures permit. The fixture exporter must reject SourceRejected/CoreRejected input rather than signing its diagnostic.

## Exact statement to sign

Recommendation: use an explicitly experimental domain such as `moriarty-midnight-auth-experiment/1`, separate from the unimplemented canonical `moriarty-intent/3` protocol. Commit the complete closed envelope, or fixed-length digests of its canonical constituents with the algorithm named:

| Constituent | Required fields |
| --- | --- |
| Version and execution context | Envelope version; Source/6, Core/5 and intent discriminator; domain; chain/network and asset representation identity when provided by beta. Missing identity cannot be invented or treated as authenticated. |
| Agreement and selection | AST `programId` (agreement ID); selected action ID, source hash, policy digest; settlement asset and scale. Core `programId` contains the selected action ID, not agreement ID. |
| Authority and freshness | Signer, key reference, signature algorithm, public key or its unambiguous commitment, nonce, derived replay tuple `(domain,signer,nonce)`, pre-head, inclusive validity bounds. |
| Exact authorization | Full signed action including owner/payer, recipient, fee recipient, obligation, value, fee, repayment identity conversion; gross cap, fee cap and net floor; success-only failure policy, explicit empty observations/disclosures/retained effects/duties and none delegation/recovery. |
| Snapshot claim | Authenticated block's head, predecessor, round, ordered balance cells, allowance owner/remaining/spent, full obligation identity/accounting/status, replay state and both work counters; full lowered pre-state. |
| Proposed transition | Submitted action, complete ordered lowered effects, proposed post-head, full candidate post-state and requested outcome. |

Signing a snapshot authenticates the signer's commitment to a claim; it does not prove that snapshot is current ledger state. Source hash and policy digest are currently opaque strings: binding their spelling does not verify their derivation. `signedDigest` must be absent or explicitly derived outside the signed payload to avoid a self-reference. An envelope that additionally signs state/effects is a stronger local experiment statement, not evidence that the canonical intent format has been settled.

Use a documented deterministic byte frame: domain tag and codec version, typed fields in fixed order, explicit array counts, byte-length-prefixed UTF-8 strings, and fixed-width unsigned integers with named byte order (or an equivalently strict canonical codec). Preserve array order; distinguish absent, empty and null; reject unknown fields, duplicate fields, noncanonical integers, overflows and trailing bytes. Publish a golden message byte vector and digest. If generic JSON is used, define and test its complete canonical subset across the exporter and Rust; insertion-order JSON serialization alone is insufficient. Test concatenation ambiguity, numeric aliases, object-key reordering, Unicode/escaping and collection boundaries. Equivalent accepted input must encode identically; semantically distinct accepted fields must encode differently.

## Required positive and adversarial matrix

| Predicate | Positive | Required negative |
| --- | --- | --- |
| Official Rust signature verification | Fresh disposable test key signs the exact fixture frame and verifies; serialization round trip preserves verifier behavior. | Wrong key, altered message, altered signature, truncated/oversized/malformed signature and public key reject cleanly. Invalid encoding must not panic. |
| Full field binding | Every fixture verifies with its original signature. | Mutate every envelope leaf independently while retaining the original signature, including each of the four unverified bindings, every action quantity/endpoint, caps/floor, key reference, versions, nonce, validity, predecessor, each state cell, effects and successor. Each changed canonical message must fail signature verification. |
| Key authority | Valid signature plus a separately specified test authority map binds `(domain, signer, keyRef)` to the expected key at the tested head/round. | An attacker signs a fresh internally valid envelope naming another signer: signature verification succeeds but authority check fails. Wrong keyRef/domain, missing mapping and revoked/expired mapping fail. The mapping is a local stipulation unless authenticated. |
| Validity and history | At `notBefore` and `notAfter`, authorized unused intent on matching head is locally eligible. | One round before/after, inverted bounds, consumed replay, stale pre-head and unrelated predecessor reject in their own checks, even with a freshly valid signature. Reuse of the same message remains cryptographically valid but must be ineligible after consumption. |
| Core financial consistency | Independent expected complete effects and post cells match actual preparer output. | Missing/extra/reordered/changed effect, omitted creditor credit, extra zero-fee credit, insufficient funds/allowance/work, overflow, swapped creditor/asset and altered signed versus submitted action reject with no published post/effects. A freshly signed invalid transition remains semantically invalid. |
| Local atomic model | Successful transition updates head, replay, work and allowance together once. | Second use and stale concurrent candidate fail; rejection leaves all cells unchanged. Label this a local model test unless actual ledger state APIs enforce it. |

Keep `signature_valid`, `authority_valid`, `snapshot_membership_valid`, `transition_valid` and `ledger_accepted` separate in results. Do not return a combined authenticated/accepted label when only a subset ran. Mutation coverage must be enumerated in machine-readable results, including nonempty-array fixtures so element tests are real.

## Smallest genuine Rust extension tests

Recommendations, conditional on the pinned APIs and resources being available:

1. **Merkle membership:** use the official transient-crypto tree and path implementation for canonical fixture state leaves. Recompute root from a valid path; wrong leaf, sibling, direction/index and root must fail equality or validation. Update one leaf: old proof must fail against the new root and the new proof must pass. Bind account/asset/domain/cell kind into leaf bytes. A root invented locally is not an authenticated Midnight head. Existing local upstream source exposes membership and bad collapsed-update tests, but this review did not execute them or establish their presence at the requested pin.
2. **State:** exercise an actual official state container update and serialization/readback, checking untouched cells and wrong-key/root behavior. If only a generic Rust map or hand-written simulator is used, report a local model rather than Midnight state validation.
3. **Proof:** run the smallest existing upstream real prove-and-verify smoke using its actual verification key/public inputs; changed public input or proof must reject. Report exact circuit predicate. MockProver, verifier stubs, proof deserialization and valid signatures are not proofs of Moriarty financial execution. Do not launch a broad native or recursive proving campaign to satisfy this row; record unavailable resources honestly.
4. **Ledger:** use the smallest upstream local transaction validation/application fixture on disposable in-memory state, testing a valid operation and a rejected mutation with unchanged pre-state. Distinguish format/serialization, validation and application. A local ledger unit test is not Preview submission/finality and cannot establish Moriarty financial settlement without the actual compiled financial caller path.

Open product gates after any passing tests: canonical production `/3` encoding and verifier integration; authenticated owner/key registry; snapshot-to-head provenance; authenticated predecessor/successor relation; real atomic ledger compare-and-consume; compiler/native financial correspondence and mandatory contract/intent/transition/history proofs; native PCD; Preview financial settlement; complete ACTUS/DeFi coverage. No acceptance promotion follows from this contract or its local experiments.
