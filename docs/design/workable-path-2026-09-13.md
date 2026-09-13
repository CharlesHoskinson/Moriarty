# Mori to ZKIR: a workable path

Synthesis of six retrieved-source research reports in
`deliverables/research-2026-09-13/`, 2026-09-13.

## The shape, decided

**A credible compiler with a verified checker.** The lowerer emits, alongside
each contract, the simulation relation mapping Core financial state to named
witness slots and constraint groups. A small checker, proven correct once,
discharges the obligations per contract.

Rejected, with reasons from the reports:

- **Full compiler verification.** CompCert is 42,000 lines of Coq and three
  person-years; CakeML's backend about 100,000 lines of HOL4. Proofs are redone
  when a pass changes.
- **Black-box translation validation.** Every published validator bridging a wide
  gap tops out at 80-97% of functions with long timeouts, and none has faced a
  target whose meaning is witness existence over a constraint system.
- **Formalising a Compact subset.** Every mechanisation in the JSCert lineage
  died at the standard it started with, and none crossed a standard boundary.
  Compact moved 0.31.0 to 0.34.0 in four months, and 0.31 changed the ZKIR
  representation of conditional branches with no syntax change at all.

## What the research settled

**No verified source-to-circuit compiler exists anywhere.** The closest are all
partial: CirC's field-blaster has per-rule verification conditions but is checked
only at 4-bit width with its calculus and flattening trusted; Aleo's Leo proofs
cover canonicalization and type inference with instruction-to-R1CS stated as
future work; CLAP proves lowering relative to a Lean circuit program. We would
not be behind the field. We would be at it.

**The correctness shape is settled.** Coglio's per-gadget soundness and
completeness, composed hierarchically. Two independent proofs exist that
per-rule obligations compose: Ozdemir and colleagues for CirC, Coglio and
colleagues for snarkVM.

**Totality does not shrink the checkers' undecided bucket.** Circom templates are
unrolled before Picus, CIVER or AC-4 see them, so those tools already consume the
finite loop-free systems a total language produces. Their walls are Groebner
blow-up, not undecidability. What totality buys is that the per-constructor
strategy is viable: a finite constructor set means a fixed number of obligations
discharged once.

**Automated field reasoning fails on exactly our hard cases.** Picus solves 70%,
with every failure a Groebner timeout on bit-decomposition or elliptic-curve
gadgets. Monolithic cvc5 cannot prove a 32-bit bit-decomposition deterministic in
a week. Anything cryptographic among our primitives goes to a proof assistant.

## The convergence worth acting on

**128-bit arithmetic is single-limb and safe; 256-bit is two-limb and is where
the hazard lives.** A 128-bit value sits inside the 255-bit BLS12-381 scalar
field with 127 bits of headroom. A 256-bit value does not, and two-limb
recomposition is non-injective without per-limb range checks and an alias check.
The worst recorded circuit bugs are in this class: CirC's comparison bug where
tested bits were not enforced, snarkVM accepting the bits of the modulus as a
decomposition of zero, and Dark Forest's comparison with unbounded inputs.

Three facts line up:

1. Our Core admits widths 64, 128 and 256 in TypeScript.
2. The K semantics admits 64 and 128 only. It has no 256-bit class at all.
3. 256-bit arises in exactly three places, all from widening: the product of two
   amounts, and the magnitude of a signed amount.

So the K/TypeScript disagreement and the worst circuit hazard are **the same
small set of cases**. Deciding them settles both. Either the product and
magnitude types are given an explicit two-limb representation with per-limb and
alias checks proven once, or they are removed from the surface and obtained by a
different route.

That decision is small, local, and unblocks the arithmetic obligations.

## Sequence

1. **Settle the range classes.** Decide K or TypeScript as authority and reconcile
   the 256-bit cases. Smallest item, blocks the most.
2. **Pin the operation footprint table.** Frame inference needs it and it exists
   nowhere as a single artifact.
3. **Write the lowerer for the 128-bit fragment only**, emitting the simulation
   relation as it goes. Single-limb arithmetic has 127 bits of headroom and
   avoids the entire recomposition hazard class.
4. **Discharge the per-constructor obligations**, soundness and completeness in
   Coglio's shape, in a proof assistant rather than in K.
5. **Prove the call-site discipline**, because that is where the bugs are: 34 of
   95 surveyed under-constrained bugs are missing input constraints or unsafe
   reuse, where the gadget was correct and the caller failed its precondition.
6. **Only then** extend to the two-limb cases, with the alias check proven first.

## On K

K has no native support for relating two definitions. The tool that did, `keq`,
was added in 2018 and deleted in 2023 with the Java backend; nothing replaced it.
`kprove` accepts one definition, and the Haskell backend's implication endpoint
requires matching sorts, so disjoint configurations cannot be compared.

Two routes with precedent: a product definition importing both semantics under
distinct cell names, estimated at three to six weeks for per-program validation;
or export both to Lean 4 with `klean`, which is what Runtime Verification are
doing for KEVM against Nethermind's Lean EVM, currently 309 manual theorems and
90 axioms and still in progress.

The second also moves the final check into a proof assistant's kernel rather than
leaving it inside K's backend, which matters because K's backend would otherwise
sit in the trusted computing base.

## The advantage we actually have

Not decidability. Three things:

- **A finite constructor set**, so obligations are proportional to the language
  rather than to the program.
- **No divergence**, so the coinductive half of the two-language equivalence
  proof system is never invoked and every proof is a finite structural induction.
- **We own the compiler**, so the simulation relation is known by construction
  rather than inferred, which is the step every black-box validator fails at.
