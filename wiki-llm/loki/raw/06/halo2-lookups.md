Lookup argument - The halo2 Book

Keyboard shortcuts
------------------

Press `←` or `→` to navigate between chapters

Press `S` or `/` to search in the book

Press `?` to show this help

Press `Esc` to hide this help

* Auto
* Light
* Rust
* Coal
* Navy
* Ayu

The halo2 Book
==============

[Lookup argument](#lookup-argument)
===================================

Halo 2 uses the following lookup technique, which allows for lookups in arbitrary sets, and
is arguably simpler than Plookup.

[Note on Language](#note-on-language)
-------------------------------------

In addition to the [general notes on language](../../design.html#note-on-language):

* We call the Z(X) polynomial (the grand product argument polynomial for the permutation
  argument) the “permutation product” column.

[Technique Description](#technique-description)
-----------------------------------------------

For ease of explanation, we’ll first describe a simplified version of the argument that
ignores zero knowledge.

We express lookups in terms of a “subset argument” over a table with 2k rows (numbered
from 0), and columns A and S.

The goal of the subset argument is to enforce that every cell in A is equal to *some*
cell in S. This means that more than one cell in A can be equal to the *same* cell in
S, and some cells in S don’t need to be equal to any of the cells in A.

* S might be fixed, but it doesn’t need to be. That is, we can support looking up values
  in either fixed or variable tables (where the latter includes advice columns).
* A and S can contain duplicates. If the sets represented by A and/or S are not
  naturally of size 2k, we extend S with duplicates and A with dummy values known
  to be in S.
  + Alternatively we could add a “lookup selector” that controls which elements of the A
    column participate in lookups. This would modify the occurrence of A(X) in the
    permutation rule below to replace A with, say, S0​ if a lookup is not selected.

Let ℓi​ be the Lagrange basis polynomial that evaluates to 1 at row i, and 0
otherwise.

We start by allowing the prover to supply permutation columns of A and S. Let’s call
these A′ and S′, respectively. We can enforce that they are permutations using a
permutation argument with product column Z with the rules:

Z(ωX)⋅(A′(X)+β)⋅(S′(X)+γ)−Z(X)⋅(A(X)+β)⋅(S(X)+γ)=0ℓ0​(X)⋅(1−Z(X))=0

i.e. provided that division by zero does not occur, we have for all i∈[0,2k):

Zi+1​=Zi​⋅(Ai′​+β)⋅(Si′​+γ)(Ai​+β)⋅(Si​+γ)​Z2k​=Z0​=1.

This is a version of the permutation argument which allows A′ and S′ to be
permutations of A and S, respectively, but doesn’t specify the exact permutations.
β and γ are separate challenges so that we can combine these two permutation
arguments into one without worrying that they might interfere with each other.

The goal of these permutations is to allow A′ and S′ to be arranged by the prover in a
particular way:

1. All the cells of column A′ are arranged so that like-valued cells are vertically
   adjacent to each other. This could be done by some kind of sorting algorithm, but all
   that matters is that like-valued cells are on consecutive rows in column A′, and that
   A′ is a permutation of A.
2. The first row in a sequence of like values in A′ is the row that has the
   corresponding value in S′. Apart from this constraint, S′ is any arbitrary
   permutation of S.

Now, we’ll enforce that either Ai′​=Si′​ or that Ai′​=Ai−1′​, using the rule

(A′(X)−S′(X))⋅(A′(X)−A′(ω−1X))=0

In addition, we enforce A0′​=S0′​ using the rule

ℓ0​(X)⋅(A′(X)−S′(X))=0

(The A′(X)−A′(ω−1X) term of the first rule here has no effect at row 0, even
though ω−1X “wraps”, because of the second rule.)

Together these constraints effectively force every element in A′ (and thus A) to equal
at least one element in S′ (and thus S). Proof: by induction on prefixes of the rows.

[Zero-knowledge adjustment](#zero-knowledge-adjustment)
-------------------------------------------------------

In order to achieve zero knowledge for the PLONK-based proof system, we will need the last
t rows of each column to be filled with random values. This requires an adjustment to the
lookup argument, because these random values would not satisfy the constraints described
above.

We limit the number of usable rows to u=2k−t−1. We add two selectors:

* qblind​ is set to 1 on the last t rows, and 0 elsewhere;
* qlast​ is set to 1 only on row u, and 0 elsewhere (i.e. it is set on the
  row in between the usable rows and the blinding rows).

We enable the constraints from above only for the usable rows:

(1−(qlast​(X)+qblind​(X)))⋅(Z(ωX)⋅(A′(X)+β)⋅(S′(X)+γ)−Z(X)⋅(A(X)+β)⋅(S(X)+γ))=0(1−(qlast​(X)+qblind​(X)))⋅(A′(X)−S′(X))⋅(A′(X)−A′(ω−1X))=0

The rules that are enabled on row 0 remain the same:

ℓ0​(X)⋅(A′(X)−S′(X))=0ℓ0​(X)⋅(1−Z(X))=0

Since we can no longer rely on the wraparound to ensure that the product Z becomes 1
again at ω2k, we would instead need to constrain Z(ωu) to 1. However,
there is a potential difficulty: if any of the values Ai​+β or Si​+γ are
zero for i∈[0,u), then it might not be possible to satisfy the permutation argument.
This occurs with negligible probability over choices of β and γ, but is an
obstacle to achieving *perfect* zero knowledge (because an adversary can rule out witnesses
that would cause this situation), as well as perfect completeness.

To ensure both perfect completeness and perfect zero knowledge, we allow Z(ωu)
to be either zero or one:

qlast​(X)⋅(Z(X)2−Z(X))=0

Now if Ai​+β or Si​+γ are zero for some i, we can set Zj​=0 for
i<j≤u, satisfying the constraint system.

Note that the challenges β and γ are chosen after committing to A and S
(and to A′ and S′), so the prover cannot force the case where some Ai​+β or
Si​+γ is zero to occur. Since this case occurs with negligible probability,
soundness is not affected.

[Cost](#cost)
-------------

* There is the original column A and the fixed column S.
* There is a permutation product column Z.
* There are the two permutations A′ and S′.
* The gates are all of low degree.

[Generalizations](#generalizations)
-----------------------------------

Halo 2’s lookup argument implementation generalizes the above technique in the following
ways:

* A and S can be extended to multiple columns, combined using a random challenge. A′
  and S′ stay as single columns.
  + The commitments to the columns of S can be precomputed, then combined cheaply once
    the challenge is known by taking advantage of the homomorphic property of Pedersen
    commitments.
  + The columns of A can be given as arbitrary polynomial expressions using relative
    references. These will be substituted into the product column constraint, subject to
    the maximum degree bound. This potentially saves one or more advice columns.
* Then, a lookup argument for an arbitrary-width relation can be implemented in terms of a
  subset argument, i.e. to constrain R(x,y,…) in each row, consider
  R as a set of tuples S (using the method of the previous point), and check
  that (x,y,…)∈R.
  + In the case where R represents a function, this implicitly also checks
    that the inputs are in the domain. This is typically what we want, and often saves an
    additional range check.
* We can support multiple tables in the same circuit, by combining them into a single
  table that includes a tag column to identify the original table.
  + The tag column could be merged with the “lookup selector” mentioned earlier, if this
    were implemented.

These generalizations are similar to those in sections 4 and 5 of the
[Plookup paper](https://eprint.iacr.org/2020/315.pdf). That is, the differences from
Plookup are in the subset argument. This argument can then be used in all the same ways;
for instance, the optimized range check technique in section 5 of the Plookup paper can
also be used with this subset argument.