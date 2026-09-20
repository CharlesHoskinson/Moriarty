# NEAR teardown result

Research, six-expert synthesis and the initial MPLR/specification package are complete. Full workspace: `/home/charl/research/near-teardown-2026-09-19`. Moriarty changes are local in `/home/charl/Moriarty-aeon-study`, branch `research/aeon-integration-roadmap-20260919`; no push or product deployment occurred.

## Acquisition and graph coverage

The complete retrieved public inventories of `near`, `Near-One`, `near-examples` and `defuse-protocol` contain515 repository entries.513 have current default-branch commits; `near/partners-arterra` and `near/wlnc-airtable` are empty upstream repositories, verified by zero remote refs. [Clone manifest](clone-results.json) records pins, archive/fork status and limitations. This does not claim every independent ecosystem organization or private repository, all historical branches, LFS payloads or recursive submodules.

The clones contain146,868 tracked files. Graphify detected62,191 supported files, including37,120 code files and9,928 documents.515 AST graph outputs total485,028 nodes and1,125,507 edges across repositories, including duplicated fork/code entities.112 extraction warning lines are preserved; a graph edge is navigation evidence, not proof of behavior. Complete source-file ownership and AST graphs remain under the full workspace.

The [interactive catalog](graph/graph.html) combines the repository inventory, captured documentation and selected source-grounded semantic concepts:930 nodes and1,247 edges. It is the navigable top level, not a claim that930 nodes represent every code symbol. [Graph coverage](graph/coverage.json) and [AST coverage](graph/ast-coverage.json) expose the full partitioned corpus.

Scrapling attempted all348 URLs from the official docs.near.org, docs.near-intents.org and nomicon.io sitemaps:331 returned200;17 stale Nomicon URLs returned404. Repository documentation is also retained. Five selected official-paper PDFs were rendered/chunked through PixelRAG, and31 repository PDFs were rendered into776 page images. Actual visual reading is separately recorded: the runtime study inspected selected Nightshade/Doomslug pages; rendering does not establish full reading, and no embedding retrieval index is claimed.

## Six configured experts

Three **Claude Fable5.1 medium** experts studied DeFi strategy, cross-chain trust and Moriarty transaction patterns. Three **GPT6Astra medium** experts independently studied the abstraction interface, native runtime and Intents/escrow kernel. Exact Fable canonical-model receipts and native Astra configurations/reviews are retained. Earlier preparation workers are identified as preparation and do not substitute for the requested configured panel.

All six explicitly endorsed [common candidate revision1](panel/CONSENSUS-CANDIDATE-v1.md), SHA256 `ab2a21a1a5f165cc2a800f4d9941851d88c16d458962dd491e1ad742dafd7658`. [Final consensus evidence](panel/FINAL-CONSENSUS.json) preserves each vote and caveat. This is a research/specification conclusion, not a formal Council runtime receipt, cryptographic proof or release approval.

## Main findings

NEAR's native runtime, MPC/Chain Signatures, Intents verifier and hosted1Click/solver services have distinct authority and settlement boundaries. Native signature/key/nonce/accounting checks do not establish arbitrary program-intent correctness. Intents engine success can precede detached calls, withdrawal or external delivery.

NEAR-style staged execution requires persistent continuations, unique occurrence correlation, result vectors, per-stage authority and cumulative resource accounting. Joins can be ready with failed results. A returned receipt chain need not include every detached branch. Local rollback does not erase all retained fees, dependencies or allowed effects.

The adjacent escrow-swap contract is a direct, narrower precedent for funded waiting and partial fills. It does not supply the requested general combination of documents, signatures and proofs. Recovery tracking is asymmetric: maker recovery state is not proof that taker and fee-collector delivery settled. Admin/pause/upgrade capacities are source facts; live holders and deployed hashes were not attested.

Documentation discrepancies remain explicit: GasKeys version85 versus docs86; broad successful-refund prose versus failed system refunds burning value; first-receipt status language versus SuccessReceiptId traversal; and1Click deadline wording differing between quote-expiry descriptions and refund initiation. Correlation IDs and quote signatures do not imply idempotency or external delivery.

## Moriarty requirements and terminology

The user requires **partial transactions** and **conditional settlement with composable evidence requirements**: submit to a destination but withhold delivery until the configured combination of signatures, documents, proofs, recipient actions or other supported conditions holds. “Contingent settlement” remains an alias; “programmable escrow” is the funded/locked variant. Recorded submission, funding reservation, condition readiness and final delivery are distinct.

The [Obsidian MPLR theory log](../../wiki/research/mplr/index.md) contains18 stable research-draft requirements, with exact NEAR evidence pointers, positive/negative examples and open PL research directions. The [NEAR Diataxis bank](../../wiki/research/near/index.md) preserves six studies, selected primary captures, claims, terminology and a conceptual lifecycle tutorial. Candidate PL theories and language comparisons are research leads, not asserted solutions.

The [new OpenSpec/EARS change](../../openspec/changes/partial-and-conditional-transactions/proposal.md) contains18 MPLR-linked requirements and36 scenarios. [C0–C4 briefs](../../openspec/changes/partial-and-conditional-transactions/implementation-plan.md) extend existing P0/P2/P4/P6/P7. The [Pel workflow](../../openspec/changes/partial-and-conditional-transactions/implementation.pel) verifies before independent review and permits one bounded correction. Internal workflow records never become public deployment prerequisites.

Moriarty remains a permissionless language for all Midnight DeFi developers and must compile to pinned ZKIRv3. It must preserve complete effects, condition evidence, partial-fill bounds, separate asset/authority/liability equations, recovery races and residual duties. Timeout does not prove nonexecution; reliable success evidence resolves its corresponding uncertainty. Compensation is a new authorized action, not history rollback.

## Validation and remaining work

All12 OpenSpec changes passed strict validation. Packaged Foreman Pel check passed without warnings; plan returned bounded dynamic regions without diagnostics. The independent specification review reports no blockers after correcting MPLR-010 so known delivery is not forced to remain unresolved. The abstraction study reproduced an independent NEP413 signing vector and mutation rejection; native runtime/kernel studies did not run full builds or live transactions.

Compiler, proof, ledger, privacy and conformance implementation tasks remain open. The next authorized research stage is Daml: acquire its official documentation, study it and add or refine MPLRs. No panel vote closes the actual implementation or proof obligations.

## Completed Obsidian ingestion verification

Transaction `ingest-near-teardown-20260919` completed, followed by `repair-near-portable-links-20260919`. All 175 final paths match the transaction contents. The 100 captured primary-source hashes, six consensus vote hashes and ten independently reviewed specification file hashes were checked. Twelve NEAR notes and eighteen MPLRs are present. The post-ingestion lint introduces no dead links, provenance errors, or other actionable findings relative to the saved baseline. The vault retains 24 historic dead links and four historic stale-index findings; repeated Diátaxis basenames use explicit relative links.

The original ingestion failed because ledger timestamps used a noncanonical UTC format. The corrected bundle passed inspection and applied successfully. Twelve workspace-absolute links were replaced with portable deliverable links. The latest user decision makes conditional settlement with programmable escrow a major abstraction pillar; see the terminology note.

Fresh OpenSpec validation: 12 passed, 0 failed. Pel static check and plan completed successfully. These checks establish documentation and workflow validity, not runtime correctness. No implementation tasks were marked complete. The research checklist in the reviewed tasks file retains its review-time state; this completion receipt records the later evidence filing. Daml remains a separate outstanding study.

[Machine-readable verification](ingestion-verification.json) and [final vault lint](wiki-lint-final.json).
