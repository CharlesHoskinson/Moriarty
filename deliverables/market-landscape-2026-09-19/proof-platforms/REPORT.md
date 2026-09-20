# Proof-platform market comparison

Snapshot 2026-09-19. Primary official documentation captured with Scrapling. This is a bounded architectural market screen, not an implementation, formal theorem or deployed-state audit. Exact evidence bytes are in claims.json. Source pages were inspected for the cited claims; acquisition does not establish that every API or statement was reviewed.

## Most relevant overlaps

**Miden** is a particularly close semantic comparator for conditional settlement. Notes carry assets plus consumption scripts. Funding a note and consuming it are separate state transitions; multi-stage swaps are explicit. Its present site adds Guardian, an institutional coordination layer for state, co-signing and policy. The current homepage and Rust tutorial establish a testnet developer product, not verified live mainnet in this research. Old transaction-model details such as address sizes are not treated as current normative constants. [PP04–06]

**Aztec/Noir** supplies programmable private contracts, action-specific delegated authorization and nullifier replay protection. Authwits bind exact arguments and domain/version; private spending also needs note secrets. This is much closer to proof-bound authority than generic wallet signatures. Private-to-public calls are queued, and utility execution is unconstrained. Alpha is live mainnet but its own docs distinguish live from hardened. No document inspected establishes Moriarty's proposed financial DSL, total profile or all-history obligation theorem. These are not proven absences; applications could encode additional constraints. [PP01–03,PP09]

**Aleo/Leo** already provides a deployed language-and-network stack where execution produces ZK proofs. It cannot be dismissed as ordinary unproved scripting. Finalize remains an onchain operation, and rejected execution can retain fees. Governance for upgrades is explicit in immutable constructor logic. Proofs establish the implemented relation; this review does not establish automatic preservation of an independently signed economic specification. The overview's broad program-privacy claim conflicts with published deployment bytecode in the detailed transaction reference, so the latter limits conclusions. [PP07–08]

**Compact/Midnight** is the baseline Moriarty must improve upon, not a hypothetical future competitor. It already aims at provable transactions with private state. Moriarty's proposed value needs to be explicit financial/resource semantics, typed partial continuations, complete effects, independent intention refinement and target correspondence—not simply being another ZK language for Midnight. [PP10]

**Penumbra** already has private financial intents and a staged Swap/SwapClaim lifecycle. It is a specialized DEX design in the reviewed source, rather than an evidenced arbitrary financial workflow language. Its claims of optimal execution are source descriptions, not independently audited optimality results. [PP11]

## Market interpretation

General-purpose proof platforms can express application predicates. An honest comparison must allow a skilled team to implement many Moriarty behaviors on them. Moriarty's potential advantage is making those obligations compositional, mandatory for the supported profile, understandable to developers, and connected to the actual Midnight path. Choosing ZKIRv3 is target specialization, not proof of conceptual novelty.

This review found substantial adjacent products. It did not establish a ready-made product satisfying the full Moriarty contract. That is a bounded evidence conclusion, not a claim of worldwide uniqueness. Compare against Anoma and Valence first, then Miden and Daml, before treating the design as novel.

## Acquisition limits

The former Aleo VM PDF URL redirects to a documentation homepage; it was not read as a PDF. The IACR Leo paper is robots-disallowed and was not fetched. Renegade's official help article was found in the web index but the local host failed DNS; no durable source claim is made here. The old Starknet OS URL returned404 locally; indexed snippets alone are not treated as current evidence. No PDF was successfully acquired by this track, so no PixelRAG reading is claimed. Sibling tracks own acquired papers.
