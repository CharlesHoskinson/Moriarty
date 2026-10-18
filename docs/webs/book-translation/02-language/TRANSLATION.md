# Layer 2 — Language

## Proposed product copy

Moriarty gives developers a financial language for describing what a program may do and what must remain true after it does it. A program can state the assets and amounts involved, who may authorize an action, the limits on fees and outcomes, and which duties continue after partial progress.

The compiler turns that program into an artifact for a named execution target. That artifact is handed to the proof and verification parts of the system, which must establish the relation the program describes. The language expresses the rule; a proof only establishes the rule encoded in the compiled relation. The handoff therefore carries the program and semantics identity alongside the proof-related artifacts, so a developer can inspect what each piece refers to.

Moriarty's current Rust/Cardano experiment and its specified Midnight target are separate routes. The current Rust path is experimental and does not establish the Midnight target. A guide should name the target, compiler and semantics versions, and evidence stage for each example.

## Layer 2 in the seven-layer explanation

Layer 2 asks how a developer expresses the computation and its public and private inputs. In the book, this layer compares circuit languages, proof-oriented virtual machines, general-purpose instruction sets, and transition-oriented languages. It is an explanatory taxonomy, not a claim that all systems use one language or a single sequence of independent stages. The book's Chapter 10 describes causal pressures between layers, including language choices shaped by arithmetization.

For Moriarty, the useful connection is the boundary between a developer's financial meaning and the compiled relation that later layers prove and verify. The compiler and relation are a visible artifact handoff in the story, not an assurance that the handoff is already complete for every target. The product copy should keep the contract's financial obligations visible: fees count against outcomes, and residual duties survive partial completion. Language support and compiler correspondence require evidence for the named target.

## Editorial boundaries

These are sourced educational proposals, not a cryptographic or security verdict. The supplied book excerpt is a broad ZK-language chapter; it does not establish Moriarty's implementation or support claims. The supplied product contract describes Moriarty's permissionless target architecture and financial semantics. The integrated concept describes a proposed Cardano Webs presentation and an experimental Rust/Cardano evidence path. Do not transfer Midnight's proof-stack description to Cardano or treat Compact/Midnight implementation as evidence for the current Rust/Cardano route.

The proposal is limited to Layer 2 and its cross-layer handoff. It does not fact-check the book as a whole. Opus 5.5 owns security fact-checking and adopted exact crypto wording. Questions below preserve claims that need that review.
