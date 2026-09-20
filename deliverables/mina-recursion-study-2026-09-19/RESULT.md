# Mina recursion case study for Moriarty

Status: bounded source/documentation research completed; graph scope and verification are recorded in the linked graph report. No Mina build, proof experiment, exploit reproduction, deployed-state verification or Midnight integration was performed. This study refines the proposed backend requirements; it does not certify either chain.

## Acquired scope

Four latest-default-branch shallow sparse repositories were acquired on 2026-09-19. Full tracked-path inventories and submodule pins are retained. Sparse materialization selects relevant recursion, crypto, transaction and developer API paths; it is not a complete checkout of every Mina ecosystem project.

| Repository | Branch | Exact commit |
|---|---|---|
| MinaProtocol/mina | compatible | `72333b33898cc092675c97d205229982cf79e7ac` |
| o1-labs/proof-systems | master | `886b5c1ca13cd2e2c634e4393ff5ff1efa5a3d3c` |
| o1-labs/o1js | main | `bb34e65e74de7ba6f6c46e34b936034d8faa5c18` |
| o1-labs/snarky | master | `3d1d68f52d0492a72aac563137e38fa5b10e1b7b` |

Mina's proof-systems gitlink matches the acquired proof-systems commit. Mina's Snarky and o1js's Mina gitlinks differ from the independently acquired HEADs. The full tuple was not built or qualified. Snarky's README identifies substantial obsolete code; its inclusion supplies interface/dependency context, not evidence that every historical backend is current.

Scrapling captured ten selected official documentation/audit resources with HTTP status and content hashes: developer proof/key/serialization APIs, the proof-system book and three Pickles chapters, an audit announcement and its PDF. Coverage is bounded to those resources and selected source ranges, not the entire documentation corpus. The book explicitly warns that it may not reflect current Mina behavior. Source inspection governs code-specific findings.

PixelRAG pixelshot rendered all ten pages of the December 2023 Pickles audit. Extracted text for all pages was read; physical pages 3, 5 and 6 were visually checked for revision/scope/findings. This was rendering and visual review, not a claimed vector-index retrieval experiment. The audit reviewed an older Pickles revision; its scope excluded a comprehensive Kimchi custom-gate/lookup review. It cannot certify the acquired 2026 commits. See [reading coverage](pdf-processing.json).

## What changes in Moriarty's requirements

[Recursion semantics](recursion-semantics.md) inspected Step/Wrap, deferred values, final accumulator verification, transcript construction, heterogeneous shapes and batching. [Developer integration](developer-integration.md) inspected ZkProgram, DynamicProof, state preconditions, auxiliary results, proof modes and caches. Root independently checked 20 exact pinned source-file hashes and 59 cited excerpt ranges; [verification](SOURCE-VERIFICATION.json). This checks attribution, not execution or soundness.

Eight mandatory refinements now supplement ZR01–ZR16 in Moriarty's next-backend contract:

1. **MNR01 — Transcript specification.** Specify absorption order, domain separation, encodings and challenge derivation across native and constrained verification.
2. **MNR02 — Deferred obligations.** Track every obligation from producer to final consumer; bind legitimate base cases, parent counts, active masks and padding.
3. **MNR03 — Heterogeneous compatibility.** Bind feature/shape/key/parameter profiles and separately enforce key integrity and owner/program authorization.
4. **MNR04 — Verified-value boundaries.** Parsing a proof or reading its fields is not verification. Auxiliary results and predicted effects need explicit bindings; settlement also needs current-state checks.
5. **MNR05 — Real proof mode.** Dummy proofs, disabled verification and simulation cannot qualify native production evidence.
6. **MNR06 — Parameter/cache provenance.** Authenticate artifact identity and semantic tuple; equal parameter dimensions do not prove compatibility.
7. **MNR07 — Batch/join soundness.** Specify randomness, cardinality, soundness composition and financial preservation under tree-shaped joins.
8. **MNR08 — Component assurance.** Trace specification to actual constraints and final verifier, with explicit audit/theorem coverage for every used component.

These are refinements of the existing native contract, not new crypto choices. Mina uses Pickles/Kimchi with IPA/Pasta mechanisms; Moriarty remains on Midnight's native Halo2-derived PLONK/KZG stack. Mina's code and historical audits do not transfer a theorem to Midnight. The six-month recursion horizon remains the user's planning assumption.

## Important distinctions

A proof of a relation is not authorization to use that relation. Dynamic key data can match its hash while the key is outside the owner's policy. A valid recursive computation also does not authenticate current spendability: settlement must connect its pre-state to authoritative current state and consume the appropriate resources.

Pickles final verification includes more than accepting one recursive circuit output. The source studies identify deferred scalar/challenge checks and separate accumulator verification. ZR08 must therefore cover the complete obligation lifecycle, not merely a function called verify or a single final pairing.

An inspected o1js example reads previous proof fields without requesting previous-proof verification. This is a source-level example observation, not a reproduced exploit or allegation about deployed Mina. It supplies a useful negative test for Moriarty's mandatory-predicate analysis. Likewise, an SRS-equality TODO and cache behavior motivate explicit parameter/provenance requirements; neither was shown exploitable here.

Bounded fan-in is not a bound on total ancestry. Larger trees must preserve predecessor coverage, common-ancestor accounting, signed budgets and residual duties. Compact proof size does not compress mutable state, deliver private witnesses or guarantee recovery liquidity.

## Graph and evidence

- [Interactive recursion graph](graphify-out/graph.html)
- [Graph report and extraction limitations](graphify-out/GRAPH_REPORT.md)
- [Repository acquisition pins](repos.json)
- [Scrapling receipts](doc-receipts.json)
- [PixelRAG processing and exact reading scope](pdf-processing.json)
- [Semantic source evidence](recursion-semantics-evidence.json)
- [Developer source evidence](developer-evidence.json)
- [Required refinement clauses](requirement-refinements.json)

The graph is a navigation artifact with source and inferred edges distinguished. It is not a proof-dependency theorem or evidence of a compatible integrated build.
