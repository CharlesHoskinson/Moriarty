# Lane 06 — exact backend boundary for universalMoriartyStep

**Status:** candidate, first design wave; no architecture selected. Security and all test judgments belong to Opus 5.5. Predicate: each machine transition has a circuit constraint owner and the same pre/post-state projection as its K rule; specified, not checked.

## Recommendation and alternatives

**Recommendation/inference:** one fixed, bounded Moriarty machine transition relation per admitted semantics/representation configuration, instantiated in Midnight’s existing self-recursive `IvcCircuit<T>`. Program identity is state/context data, rather than a freshly generated circuit for each financial program. A step processes one instruction, or a fixed maximum microstep batch with explicit active count. Circuit shape depends on configured bounds, never unbounded recursive structure. Configuration identifies semantics, field encoding and bounds; no Fenrir network version is inferred.

**Alternative:** a small family for arithmetic, collection access, frame control and effects, with a common state projection and an explicit family selector in a custom recursive wrapper. This could avoid charging every step for every opcode, but introduces multiple relation identities and composition layouts. **Counterargument:** a universal step can pay heavily for unused branches, authenticated memory and wide integer operations. Prefer the small family if later authorized measurements establish that cost; no cost superiority is claimed here. Per-program specialization remains a third fallback, with weaker circuit reuse.

## Concrete circuit and code inventory

**Source fact:** freshly captured Midnight `IvcTransition` separates native `transition` from `circuit_transition`; `IvcIO` supplies assignment and public-input formatting; `IvcState` separates native data from constant-size assigned summaries. Its `decider` is documented as off-circuit. `VerifierGadget::prepare` returns an assigned accumulator. 

**New lowering:** lower typed Moriarty programs into a frozen instruction table with explicit widths, evaluation order, constructors, frame actions and ordered effects. Supply a program-table opening plus selected instruction operands to `universalMoriartyStep(ctx, preState, witness) -> postState`; the circuit owns instruction selection and its semantic transition. Host execution produces openings, traces and frame/data preimages; it does not replace circuit transition ownership with an evaluator result flag.

**New relation code:** `MoriartyTransition` implements those IVC traits and opcode/data/frame gadgets. **Reused wrapper:** `IvcCircuit<MoriartyTransition>` provides recursive composition, not a second hand-written wrapper by default. Conceptually the transition and wrapper are two layers but can form one recurring circuit shape. **Reused endpoint:** adapt the statement projection to the user-demonstrated ledger9 custom-decider route, without `VerifyProof`. Its existence is a session premise; a generic native verifier does not imply another mandatory decider circuit. Exact endpoint correspondence remains designated-review work.

## State, recursive data and statement

Candidate assigned state: `(programRoot, pc, environmentRoot, valueStoreRoot, frameRoot, effectRoot, remainingFuel, status, initialStateRoot)`. Native state additionally holds typed values and frame preimages. Values use explicit tags and bounded-width payloads; recursive values use node references. Frames encode return PC, destination, lexical environment and continuation link. Options: bounded vectors, or committed node/frame stores with bounded access paths. The latter keeps assigned state fixed size but adds access gadgets and host storage work; neither has been cost-qualified.

Endpoint projection carries program/semantics identity, initial/final state and ordered-effect summaries, completion status and the existing ledger statement fields through a named adapter. Internal data/frame layout stays distinct from ledger statement layout.

## Minimal normal flow and model boundary

Compile typed program → initialize machine → witness one bounded transition → recurrent wrapper advances state → repeat → project completed result through existing custom-decider adapter → assemble ledger9 call. Proposed bounded specimens: two different programs sharing the same machine configuration, and one program constructing/deconstructing a recursive value across two call frames; these are demonstrator descriptions, not test results.

K should define `<code>`, `<pc>`, `<env>`, `<store>`, `<frames>`, `<effects>`, `<fuel>`, `<status>` and explicit instruction rules. Quotienting each rule to the assigned tuple is a separate correspondence obligation. Quint tracks bounded `pc`, frame depth, lifecycle status, fuel and effect sequence, abstracting payloads and commitments as identifiers. It models no `proofValid` predicate; cryptographic acceptance is outside that abstraction.

## Primary evidence and visuals

Fresh captures and SHA-256 receipts: `sources.json`. [Midnight IVC traits](https://github.com/midnightntwrk/midnight-zk/blob/fac0b16844f9b23922e47cd1dfc418ddd1e92968/aggregation/src/ivc/mod.rs#L55); [recursive gadget](https://github.com/midnightntwrk/midnight-zk/blob/fac0b16844f9b23922e47cd1dfc418ddd1e92968/circuits/src/verifier/verifier_gadget.rs#L810). Historical UPSTREAM’s main inventory uses `864f984a`; these fresh captures use `fac0b168`, including `AssignedVk<S>`/`S::PCS`, and establish no whole-tree equivalence.

[RISC Zero’s official overview](https://dev.risczero.com/proof-system/) distinguishes execution, recursion and final wrapping circuits. That separation is an analogy, not a proposed foreign proof-system port. [Actual primary layers figure](https://dev.risczero.com/assets/images/proof-system-layers-f17e0e6171d12f5dd1fd56f67e03ca6b.png); [receipt figure](https://dev.risczero.com/assets/images/receipt-f24f19ae61fadf1d375fcb18563b0f8e.png). No native/model execution or performance/acceptance claim.
