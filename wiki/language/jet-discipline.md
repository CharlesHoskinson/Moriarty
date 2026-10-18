---
id: language.jet-discipline
type: language
title: Jet discipline for Moriarty Core
status: draft
updated_at: 2026-09-13T06:00:00Z
sources:
  - SRC-0114
  - SRC-0115
created: 2026-09-13
updated: 2026-09-13
tags:
  - moriarty
  - research
  - language
  - formal
---

# Jet discipline for Moriarty Core

## The mechanism

**CLM-0971 — Simplicity's whole core is 99 lines.** Nine term constructors with
their denotational semantics beside them, over a type system of exactly three
constructors: unit, sum, product. No integers, no strings, no arrays, no
recursion and no loops, so every program is total and its cost is statically
knowable (CLM-0971; SRC-0114 `Coq/Simplicity/Core.v`, `Coq/Simplicity/Ty.v`;
source fact; reproduced; high; S4).

Moriarty's Core carries sixty constructors by comparison.

**CLM-0972 — A jet's semantics is the semantics of the expression it wraps.**
Defined as `Jet.jet A B t p := t (PrimitivePrimSem M)`. It is denotationally the
identity: deleting every jet from a program cannot change its meaning. A jet is
recognised by the Merkle hash of the expression it stands for, so the interpreter
may substitute a fast native implementation, and `jet_Parametric` proves jets are
transparent to any parametric relation
(CLM-0972; SRC-0114 `Coq/Simplicity/Primitive.v:200-252`; source fact; reproduced; high; S4).

This is the opposite of an unverified optimisation attribute. Nothing is trusted;
the fast path and the defined path are provably the same path.

## Why Moriarty needs it

**CLM-0973 — Two implementations of one meaning have already drifted.**
`financial-lifecycle-v2.ts` is 1,994 lines of which 1,959 are byte-identical to
`financial-lifecycle.ts`, while `financial-expression-v2.ts` and `-v3.ts` in the
same directory are 5-line re-export shims. The cheap idiom existed and was walked
past (CLM-0973; measured 2026-09-13 in `experiments/moriarty-language/src/successor/`;
measurement; reproduced; high; S4).

A jet discipline replaces "a K semantics and an evaluator maintained beside each
other" with "one definition plus annotations naming where a fast path is
permitted, checked against the definition".

## Mapping the sixty constructors

Three independent analyses, one per slice, in
`deliverables/jet-discipline-2026-09-13/`.

### Financial reads and effects (17 constructors)

**CLM-0974 — The twelve financial reads are projections, not effects.** The
obligation, balance and allowance maps are built from the `financialPre` value
passed into the machine constructor; the reduce rule is a phase branch, one keyed
lookup, a unit assertion and a field projection. Nothing outside the input is
consulted, so in Simplicity terms this is `take`/`drop` navigation plus one
partial lookup, and the lookup is not even a primitive in the `Prim.sem` sense
because it acts on an input value rather than an ambient environment
(CLM-0974; `financial-expression-v1.ts:157-166,423-445`; source fact; reproduced; high; S4).

**CLM-0975 — `Outstanding` is a jet of `Add(Principal, Accrued)` and the proof
already exists.** Admission is rejected unless `p + a == o`; repayment writes all
three together; accrual rounds interest once then adds it to both `accrued` and
`outstanding`; the liability cap rejects the transition rather than clamping. The
same invariant appears in the K semantics
(CLM-0975; `financial-lifecycle.ts:854-859,1614-1619,1791-1795,1818`;
`formal/k/moriarty.k:80,98`; source fact; reproduced; high; S4).

**CLM-0976 — Pre and post reads collapse to six reads plus a phase operand.** The
only differences are which map is selected and a static gate that fires when a
post read appears outside the `Ensure` suffix. `ReadPre` already carries a `view`
operand doing exactly this, so the collapse is precedent rather than invention
(CLM-0976; `financial-expression-v1.ts:283-285,310,476-482`; source fact; reproduced; high; S4).

The gate must remain a **typing rule on the phase operand**, never a value
threaded into a single environment. Weakening it to a value would create exactly
the shorter-and-less-sound spelling that [[wiki/language/zk-language-survey|CLM-0967]]
forbids.

**CLM-0977 — The five statements are not Core terms.** `Require` and `Ensure` are
assertions with `mzero` semantics and error codes that are commitment metadata;
`Let` is `pair` threading that a nameless term eliminates; `Emit` and `NextWrite`
are output construction outside the term language. None is a jet candidate
(CLM-0977; SRC-0114 `Coq/Simplicity/Alg.v:461-472`; source fact; reproduced; high; S4).

**CLM-0978 — Seventeen constructors reduce to two irreducible primitives**: a
keyed obligation lookup and a keyed party-asset lookup. Even these may be Core
terms if state is modelled as a bounded value, which the declared collection
capacity permits (CLM-0978; `deliverables/jet-discipline-2026-09-13/financial-effects-slice.md`;
analysis; reproduced; high; S4).

### Computation (23 constructors)

**CLM-0980 — The work meter breaks jet transparency, and this is the finding
that matters most.** Moriarty's work meter is observable in the result, so
expanding `And` into `Select` changes the observed work count by one unit per
expansion. Simplicity's jets need no such rule because Simplicity has no
observable cost meter. **A jet's cost in Moriarty must therefore be a declared
constant, not the cost of its expansion**, or the jet is not denotationally the
identity and CLM-0972 fails to transfer
(CLM-0980; `financial-expression-v1.ts:36-37`; analysis; reproduced; high; S4).

**CLM-0981 — K and TypeScript already disagree, today.** `LitUInt` admits widths
64, 128 and 256 in the TypeScript contract; the K inference rules admit 64 and
128 only. This is precisely the drift a jet discipline exists to prevent, and it
is present in the current implementation
(CLM-0981; `financial-expression-v1.ts:222`; `formal/k/expression-infer.k:126`;
source fact; reproduced; high; S4).

**CLM-0982 — Nine literal constructors are one construction.** They share a
single dynamic rule in both implementations and differ only in how the type is
spelled on the wire, so they collapse to `Lit(type, value)`
(CLM-0982; analysis; reproduced; high; S4).

**CLM-0983 — Twenty-three constructors reduce to nine primitives.** `Lit`,
`Select`, `Eq`, `Lt`, `Add`, `Sub`, `Mul`, `FloorDiv`, `CeilDiv`. The boolean
operators derive from `Select` and `Lit`; the remaining comparisons derive from
`Lt` and `Not`. Of the nine, the five arithmetic constructors are the jet
candidates, each carrying a per-range-class proof obligation; `Eq` and `Lt` are
marginal; `Lit` and `Select` are core rather than jets
(CLM-0983; `deliverables/jet-discipline-2026-09-13/computation-slice.md`;
analysis; reproduced; high; S4).

**CLM-0984 — The performance motive is conditional and unestablished.**
Arithmetic jets have a strong motive under a bit-blasted core and none under the
present native-bigint evaluator. Nothing in the formal directory or the
deliverables names a circuit or bit-level backend, so whether the motive applies
cannot be determined from the repository
(CLM-0984; analysis; reproduced; medium; S3).

**CLM-0985 — A syntactic rule blocks jets outright, and this is a hard
blocker.** The static rule for `ScaledAmount` requires the divisor to be
*syntactically* a `LitUInt` node. A jet wrapping that literal, or a `Let`-bound
copy of it, is rejected even though it is the same term under Merkle-root
recognition. Any jet discipline must first replace syntactic operand tests with
tests on the term's meaning or its root
(CLM-0985; `financial-expression-v1.ts:347`; source fact; reproduced; high; S4).

**CLM-0986 — The proposed basis rests on a constructor K does not admit.**
`Select` is the slice's only branch and the retained K definition does not admit
it, so the nine-primitive basis currently exists in one implementation only
(CLM-0986; `independent-constructor-inventory-01.json`; source fact; reproduced; high; S4).

**CLM-0987 — `CeilDiv` is not derivable inside Core/4's own types.** Although
`ceil(a/b) = -floor(-a/b)` holds mathematically, `0 - a` fails the range check
for unsigned `a`, and `floor((a+b-1)/b)` overflows where the primitive succeeds.
TypeScript and K implement it by different formulas, which is a concrete case for
one definition plus a checked fast path
(CLM-0987; `financial-expression-v1.ts:405`; `formal/k/expression-v1.k:141`;
source fact; reproduced; high; S4).

### Data (20 constructors)

**CLM-0988 — Nineteen of twenty have direct encodings, and the encoding is free
inside the evaluator.** Field names, enum member names and variant tags survive
into the dynamic rules only as JSON keys, and every use is either a statically
known lookup or a canonical-order equality that a name-to-position bijection
preserves. Names must survive at the boundary, in the snapshots coming in and the
post record and descriptors going out, so the change needs a codec rather than a
semantics change
(CLM-0988; `deliverables/jet-discipline-2026-09-13/data-slice.md`; analysis; reproduced; high; S4).

**CLM-0989 — `AccessField` and `AccessIndex` are not a semantic family.** They are
surface provenance, not a distinct meaning from the `Project` forms
(CLM-0989; analysis; reproduced; high; S4).

**CLM-0990 — Collapsing the four reads makes `ReadPre` better typed, not weaker,
and this resolves the soundness worry directly.** Today a post read outside the
`Ensure` suffix is caught by a static scope gate. Under navigation over one
structured input, the suffix is a check over a product of input and output, so a
post read in the prefix becomes **ill-typed** rather than merely scope-rejected,
and `next` has no component to read at all. The discipline that
[[wiki/language/zk-language-survey|CLM-0967]] protects is strengthened by the
change, not weakened. What breaks is observable rather than semantic: error
codes, node paths and per-node work counts
(CLM-0990; `financial-expression-v1.ts:285,388,490`; `formal/k/expression-v1.k:114`;
analysis; reproduced; high; S4).

**CLM-0991 — Three projections are partial and the nine-constructor core cannot
express them.** `ProjectSome`, `ProjectVariant`, `ProjectIndex` and narrowing
`ConvertUInt` need Simplicity's `assertl`/`assertr`, which sit outside the nine
combinators (CLM-0991; SRC-0114; analysis; reproduced; high; S4).

**CLM-0991a — `ConvertUInt` and `ScalarValue` are not jets over anything.**
`ConstructAmount`, `ConstructShares` and `ConvertUInt` share one pass-through
rule and their range check is unreachable because the child is already forced to
the same bound, making them pure type ascriptions. `ScalarValue` is four
operations under one name. No bit-level word type or two's-complement definition
exists in the repository, so there is no lower term for a jet to wrap
(CLM-0991a; `financial-expression-v1.ts:267,379-384`; `financial-expression-types-v1.ts:157`;
source fact; reproduced; high; S4).

**CLM-0992 — Twenty constructors reduce to six term formers**, of which two are
shared with other slices, so this slice contributes four of its own: `pair`,
`path`, `inject` and `case`
(CLM-0992; analysis; reproduced; high; S4).

## The governing caveat

**CLM-0993 — Moriarty cannot have jets yet, because a jet needs a lower-level
definition to be identical to.** All three slices reach the same conclusion from
different directions: the reduction to roughly fifteen primitives is available
today, but the *jet* property requires a bit-level or circuit definition for the
fast path to be checked against, and nothing in the formal directory or the
deliverables names such a backend. Choosing a basis and re-deriving the rest is
available now; jets are a later step that depends on that backend existing
(CLM-0993; all three slice analyses; analysis; reproduced; high; S4).

## Combined reduction

| Slice | Constructors | Reduces to |
| --- | ---: | --- |
| computation | 23 | 9 primitives, 5 jet candidates |
| data | 20 | 6 formers, 4 unique to the slice |
| financial and effects | 17 | 2 irreducible primitives |
| **total** | **60** | **roughly 15** |

## Open question recorded against this note

**CLM-0979 — The post-state invariant rests on the host.** `continueSuffix` does
not re-admit the host-supplied `financialPost`, so `p + a == o` is enforced on
admission and on transitions but not re-checked on a host-supplied post state.
Whether that matters depends on what the Merkle commitment is taken over, which
the repository does not settle
(CLM-0979; `financial-lifecycle.ts:589`; source fact; reproduced; medium; S3).

## Correction: what totality does and does not buy

**CLM-1019 — Totality does not shrink the checkers' undecided bucket, and an
earlier claim in this session that it would was wrong.** Circom templates are
unrolled before Picus, CIVER or AC⁴ ever see them, so those tools already consume
a finite, loop-free polynomial system, which is exactly what a total language
would hand them. Their undecided buckets come from Gröbner-basis blow-up on
non-linear gadgets over a 254-bit field, not from undecidability. Picus reports
30% undecided overall and 80% on large circomlib-core; CIVER 42%; AC⁴ 11%
imprecise plus 3% unknown. Picus's own failure analysis names the causes: BabyAdd
requires the Bernstein-Lange elliptic-curve addition theorem, and Num2BitsNeg
produces polynomials with degree and coefficients above 10^5
(CLM-1019; `deliverables/research-2026-09-13/correct-by-construction.md`;
retrieved sources; reproduced; high; S4).

**CLM-1020 — What totality does buy is that the per-constructor strategy is
viable, and the composition theorem has been proved twice.** Ozdemir and
colleagues (CAV 2023, CirC) prove that per-rule verification conditions imply a
correct field-blaster and that correctness composes across passes. Coglio and
colleagues verify Aleo snarkVM R1CS gadgets bottom-up from sub-gadget theorems in
ACL2. A finite constructor set therefore means roughly fifteen fixed obligations
discharged once, rather than per-circuit checking
(CLM-1020; research; reproduced; high; S4).

**CLM-1021 — The composition theorem's hypothesis is where the real bugs live.**
34 of the 95 under-constrained bugs in the USENIX SoK are missing input
constraints or unsafe reuse, where the gadget was correct and the *caller* failed
to discharge its precondition. The circom-pairing BigLessThan case is exactly
this: the sub-circuit was correct and the caller ignored its outputs. Composition
is therefore a proof obligation on call sites, not a free consequence
(CLM-1021; research; reproduced; high; S4).

**CLM-1022 — Any cryptographic primitive among our constructors inherits the hard
case.** If a hash or curve operation is one of the fifteen, its one-time proof is
precisely the obligation Picus could not discharge automatically, so it goes to a
proof assistant, as Coda does in Coq and Kestrel in ACL2
(CLM-1022; research; reproduced; high; S4).

**CLM-1023 — Midnight's own documentation says witness results should be treated
as untrusted input.** This is a hazard our lowerer must handle rather than assume
away (CLM-1023; research, citing Compact documentation; reproduced; high; S4).
