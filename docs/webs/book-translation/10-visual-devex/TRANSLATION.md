# Visual and DevEx translation: seven-layer model in Moriarty Webs

**Role:** visual design and developer experience. This is a product translation proposal for review, not an implementation or a security judgment.

## Proposed story

Keep the current Webs opening intact: “Bring the required proofs to the transition,” with the invoice example as the first concrete case. Add a nearby “How the proof system fits together” navigator that presents the seven named concerns as a connected map. The reader can begin with the product question—what a consuming contract requires—and follow links into the proof machinery. Seven selectable nodes disclose concise explanations and the relationship to Webs, Strands, statements, proofs, and the consumer. A node is a lens on the system, not a claim that Moriarty implements a corresponding ZK component.

Use the book's exact order and names: Setup, Language, Witness, Arithmetization, Proof System, Cryptographic Bedrock, Verification. Draw causal connections as labeled, directional paths, not as a seven-step progress rail. Show the four examples from Chapter 10 (verification economics influencing proof/cryptographic choices; language/constraint co-design; witness/arithmetic fusion) and mark the Layer 4–6 proof core as a strongly coupled cluster. Put a visible legend on the graphic: “Conceptual map from *Proving Nothing*. Connections describe design pressures; they do not assert Moriarty implementation.” Keep runtime movement of a transaction distinct from the book's design-time dependency map.

Place a compact “Setup and proof-size facts” panel directly in the navigator, with a persistent entry point from the Implementation navigation. It should make the book's Groth16 fact easy to find: **192 bytes in the cited construction; circuit-specific setup**. Keep this separate from the 2023 Ethereum KZG ceremony's **141,416 participants**. Those are different systems and counts. Do not turn either into a count of Moriarty setups, circuits, deployments, or APIs. Pair the facts with a clearly labeled implementation-status appendix describing only evidence actually reviewed; this packet contains no current circuit inventory or verified Moriarty setup count, so those fields should say “not established in this material.”

Keep “Explore” and “Follow the value” paths available. The invoice case stays the default entry point and retains its existing amount-change interaction/CTA. Let users move from an invoice proof role to the relevant layer explanation and back to the example, without implying that the proof itself establishes ledger premises. The current explorer's required-role behavior remains the product story; the seven-layer map explains background concepts.

## Visual and interaction behavior

At desktop widths, show a labeled seven-node causal map beside a selected-node detail panel, with the invoice case and setup facts available in the same section. Use color and line style plus text labels to distinguish book claims, product mappings, and unresolved implementation status. Provide keyboard-operable node selection, visible focus, link labels, and an adjacent textual dependency list so the map is not the only way to understand it.

On narrow screens, replace the force-directed/canvas-like layout with a responsive SVG or stacked node list and an explicit “Related layers” disclosure. Avoid horizontal scrolling. Maintain at least 44 CSS-pixel hit areas for node controls and scenario tabs. The graph can simplify its geometry at small sizes while retaining labels, direction, and the proof-core grouping.

Respect `prefers-reduced-motion`: no continuously moving graph, pulsing nodes, or animated path tracing is required to understand relationships. If a transition is used, make it brief and suppress it for reduced-motion users. Keep every interaction's selected state available without animation.

## Copy boundary

The layer names, ordering, examples, and conceptual dependencies come from the pinned book excerpts. Statements connecting these ideas to Moriarty Webs are product mappings or inferences, not book facts. Describe Webs as the financial application/contract and required-Strand consumer shown in the supplied page. Describe a Strand as carrying a proof role and statement in the supplied explorer. Do not describe the seven layers as seven deployed Moriarty subsystems or claim proof coverage from a graph, UI preview, or screenshot.

The exact product implementation status, adopted cryptographic language, and any supported APIs belong in the separately owned implementation/security review. No implementation or security status was established by this visual review.
