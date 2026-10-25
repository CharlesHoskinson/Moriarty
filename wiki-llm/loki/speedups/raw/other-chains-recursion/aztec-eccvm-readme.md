Barretenberg: ECCVM (ElliptiC Curve Virtual Machine) in Barretenberg

|  |
| --- |
| Barretenberg  The ZK-SNARK library at the core of Aztec |

Loading...

Searching...

No Matches

ECCVM (ElliptiC Curve Virtual Machine) in Barretenberg

> ‍**Warning:** This document is intended to provide an overview of the ECCVM in barretenberg. It is not a complete specification and does not cover all edge cases or optimizations. The source code should be consulted for a complete understanding of the implementation.

Punchline
=========

The ECCVM efficiently proves the correct execution of accumulated elliptic curve operations on the BN-254 curve. It does this by witnessing the correct execution into a table of numbers (in the same field as the base field of the elliptic curve) and applying polynomial constraints, multiset equality-checks, and lookup arguments.

Contents
========

* Notation
* Glossary and Common Confusions — easily-confused terms, and where things live in code
* Bird's eye overview/motivation
* Op Queue — operations, scalar decomposition, the input trace
* Architecture
* Straus Algorithm for MSM — wNAF, precomputation, the MSM loop
* Tables — the Transcript, Precomputed, and MSM columns
* Multisets and Lookups — how the three tables communicate
* Zero-Knowledge — the masking pipeline (the ECCVM is always ZK)
* Relations and Subrelations — the named `SubrelationIndex` enums

Notation
========

* $\mathbb{F}\_q$ is the prime field of size $q = 21888242871839275222246405745257275088696311157297823662689037894645226208583$.
* $\mathbb{F}\_r$ is the prime field of size $r = 21888242871839275222246405745257275088548364400416034343698204186575808495617$.
* $E/\mathbb{F}\_q$ is the elliptic curve whose Weierstrass equation is $y^2 = x^3 + 3$. This is known as the *BN-254* curve.
* The element $\mathcal{O}$ refers to the neutral element of $E$, i.e., the point at infinity. We internally represent it in affine coordinates as $(0, 0)$ for efficiency, although $(0, 0)$ is not a point on the curve.
* $C/\mathbb{F}\_r$ is the elliptic curve whose Weierstrass equation is $y^2 = x^3 - 17$. This is known as the *Grumpkin* curve.

We have the following facts:

* $2r>q>r$
* $C(\mathbb{F}\_r)$ is a cyclic group of order $q$, i.e., is isomorphic to $\mathbb{Z}/q\mathbb{Z}$
* $E(\mathbb{F}\_q)$ is a cyclic group of order $r$, i.e., is isomorphic to $\mathbb{Z}/r\mathbb{Z}$.

In general, $\mathbb{Z}/q\mathbb{Z}$ and $\mathbb{Z}/r\mathbb{Z}$ refer to the additive abelian groups; we use $\mathbb{F}\_q$ and $\mathbb{F}\_r$ when we require the multiplicative structure. We do not strictly abide by this convention (common in cryptography), but it helps disambiguate usage.

We also use the following constants:

* `NUM_WNAF_DIGIT_BITS = 4` (denoted $w$)
* `NUM_SCALAR_BITS = 128`
* `NUM_WNAF_DIGITS_PER_SCALAR = NUM_SCALAR_BITS / NUM_WNAF_DIGIT_BITS = 32`
* `ADDITIONS_PER_ROW = 4`

Finally, the terminology `pc` stands for *point-counter*. (In particular, it does *not* stand for "program counter".)

Glossary and Common Confusions
==============================

This section is a quick reference for terms and columns that are easy to confuse. The full treatment is in the linked sections.

Where things live in the code
-----------------------------

| Concern | Location |
| --- | --- |
| Columns / entities (the canonical list) | `eccvm/eccvm_flavor.hpp` (`PrecomputedEntities`, `WireNonShiftedEntities`, `WireToBeShifted*Entities`, `ShiftedEntities`) |
| Witness generation | `eccvm/transcript_builder.hpp`, `eccvm/precomputed_tables_builder.hpp`, `eccvm/msm_builder.hpp` |
| Constraints | `relations/ecc_vm/` — one relation per concern, each with a named `SubrelationIndex` enum (see Relations and Subrelations) |

When this doc names a column or subrelation, it uses the exact code identifier, so a rename in code surfaces here as a stale reference.

Easily-confused terms
---------------------

| Term | The trap | What it actually means |
| --- | --- | --- |
| `transcript_msm_transition` vs. `msm_transition` | same name, opposite ends of an MSM | **Transcript** column: `1` on the **last** `mul` row of an MSM (where the Transcript table reads back the MSM result). **MSM** column: `1` on the **first** row of a new MSM block. Different columns in different tables. |
| `transcript_pc` vs. `msm_pc` | both "point counters" | Both are **decreasing** counters over short (128-bit) `mul`s. `transcript_pc` decrements **per `mul` row** (by `2 − z1zero − z2zero`); `msm_pc` is **constant within an MSM segment** and skips values (it equals the number of muls completed *before* that segment). |
| `pc` | "program counter" | **Point**-counter: counts `mul` operations only (not `add`s), and counts **down** (both for cheaper commitments). |
| `msm_round` (0–32) vs. `precompute_round` (0–7) | which "round"? | A scalar has `NUM_WNAF_DIGITS_PER_SCALAR = 32` wNAF digits **plus** a skew digit. In the **MSM** table `msm_round ∈ {0,…,32}`: round 31 is the last addition round, the skew is round 32; `msm_round_minus_31_inv` is the witness for the "is this round 31?" check. In the **Precomputed** table `precompute_round ∈ {0,…,7}` — 8 rows per scalar, 4 wNAF digits per row. |
| `eq` vs. `eq_and_reset` |  | `eq` checks `A == P` (opcode 2); `eq_and_reset` checks `A == P` **and then** sets `A ← O` (opcode 3). |
| the "three tables" | three separate circuits/proofs | Three **disjoint column groups of one trace** — `transcript_*`, `precompute_*`, `msm_*` — committed under one ECCVM flavor and linked by multiset checks + lookups. |
| multiset checks a.k.a. "strict lookup arguments" | a kind of lookup | A **multiset-\_equality\_ check**: it asserts that two multisets of tuples are equal — every "write" has exactly one matching "read" — via the grand-product `ECCVMSetRelation`. Its *purpose* is **inter-table communication**: a VM trace has no copy constraints (unlike a circuit, where wires can be equated directly), so the disjoint `transcript_*` / `precompute_*` / `msm_*` column groups instead "talk" to each other by emitting matching tuples into a shared multiset — e.g. the Precomputed table writes the `(pc, round, wnaf_slice, …)` tuples that the MSM table must read back. Distinct from the conventional log-derivative **lookups** (`ECCVMLookupRelation`). |
| $\mathcal{O} = (0,0)$ | `(0,0)` is a real point | `(0,0)` is **not on the curve**; it is the internal affine stand-in for the point at infinity. Infinity cases are tracked by separate `*_infinity` flag columns, not by the coordinates. |
| `lagrange_second` / `lagrange_third` | the literal 2nd / 3rd rows of the trace | They name **roles at the top of the *active* region — not absolute trace rows**. The first `TRACE_OFFSET = NUM_DISABLED_ROWS_IN_SUMCHECK` rows are **disabled masking rows** for ZK (see Zero-Knowledge), so the real execution starts *below* them: `lagrange_first` pins row `TRACE_OFFSET` (first active row), `lagrange_second` pins `TRACE_OFFSET + 1` (the **hiding-op row**), `lagrange_third` pins `TRACE_OFFSET + 2` (the **first real-op row**), and `lagrange_last` pins the final row. So `lagrange_second` is the masking-offset "second" active row, not the 2nd coefficient of the committed polynomial. |
| `z1`, `z2` vs. `mul_scalar_full` | three scalars? | `mul_scalar_full` is the full $\mathbb{F}\_r$ scalar; `z1`, `z2` (each < 2¹²⁸) are its endomorphism decomposition `s = z1 − λ·z2` (see Decomposing scalars). |
| `slice` (compressed) vs. wNAF digit (signed) | two encodings of one digit | A wNAF digit is an **odd** integer in `{−15,…,15}`; the **compressed slice** is `(digit + 15)/2 ∈ {0,…,15}`, used to index the precomputed point table. |
| "wNAF" here | textbook (w)NAF | The decomposition used here, `s = Σⱼ aⱼ·2⁴ʲ − skew`, is **bespoke**; do not map it onto standard windowed NAF (see the warning under wNAF). |
| `offset_generator` / "relaxed completeness" | a soundness hole | Every MSM accumulator starts at a fixed `offset_generator` so EC additions never hit the doubling/identity edge cases. This **relaxes completeness** (a few valid op-queues cannot be proven) but **not soundness**; the excluded inputs are vanishingly unlikely. |

Bird's eye overview/motivation
==============================

In a nutshell, the ECCVM is a simple virtual machine to facilitate the verification of native elliptic curve computations. Given an `op_queue` of BN-254 operations, the ECCVM compiles the execution of these operations into an *execution trace representation* over $\mathbb{F}\_q$ (the field of definition / base field of BN-254). This field is also the scalar field of Grumpkin.

In a bit more detail, the ECCVM is a compiler that takes a sequence of operations (in BN-254) and produces a table of numbers (in $\mathbb{F}\_q$), such that the correct evaluation of the sequence of operations precisely corresponds to polynomial constraints vanishing on the rows of this table. Moreover, these polynomial constraints are independent of the specific sequence of operations. As our tables of numbers have elements in $\mathbb{F}\_q$, the *native field* of the circuit is $\mathbb{F}\_q$. When we prove these constraints, we use the Grumpkin curve $C$.

The core complication comes from the *efficient* handling of scalar multiplications. Due to MSM optimizations, we effectively produce *three* tables, where each table has its own set of multivariate polynomials, such that correct evaluation corresponds to those polynomials vanishing row-wise. These tables "communicate" via strict lookup arguments and multiset-equality checks.

Earlier [documentation](https://hackmd.io/@aztec-network/rJ5xhuCsn?type=view) exists. While it does not exactly match the current codebase, it is a helpful guide; this document is an updated explication.

Op Queue
========

We first specify the allowable operations; the OpQueue is roughly a list of operations on a fixed elliptic curve, including a running accumulator which propagates from instruction to instruction. It may be seen as a finite state machine processing simple elliptic curve operations with a single memory register.

Operations
----------

At any moment we have an accumulated value $A$, and the potential operations are: `add`, `mul`, `eq`, `reset`, `eq_and_reset`. There are four selectors $q\_{\text{add}}, q\_{\text{mul}}, q\_{\text{eq}}, q\_{\text{reset}}$, so all operations except `eq_and_reset` correspond to a unique selector being on. Given an operation, we have an associated opcode value:

| `EccOpCode` | Op Code Value |
| --- | --- |
| `add` | $1000\_2 \equiv 8$ |
| `mul` | $0100\_2 \equiv 4$ |
| `eq_and_reset` | $0011\_2 \equiv 3$ |
| `eq` | $0010\_2 \equiv 2$ |
| `reset` | $0001\_2 \equiv 1$ |

On the level of selectors, the `opcode_value` column equals

$$8 \cdot q\_{\text{add}} + 4 \cdot q\_{\text{mul}} + 2 \cdot q\_{\text{eq}} + q\_{\text{reset}}.$$

### Description of operations

* `add` takes a point $P$ and updates $A \leftarrow A + P$.
* `mul` takes $P$ and $s \in \mathbb{F}\_r$ and updates $A \leftarrow A + sP$.
* `eq` takes $P$ and "checks" $A == P$.
* `reset` sets $A \leftarrow \mathcal{O}$.
* `eq_and_reset` takes $P$, checks $A == P$, and then sets $A \leftarrow \mathcal{O}$.

Decomposing scalars
-------------------

*Decomposing scalars* is an important optimization for (multi)scalar multiplications, especially when many scalars are 128-bit.

Both $\mathbb{F}\_r$ and $\mathbb{F}\_q$ have primitive cube roots of unity (their orders are $\equiv 1 \pmod{3}$). Fix $\beta \in \mathbb{F}\_q$ a primitive cube root of unity. It induces an order-6 automorphism $\varphi$ of BN-254: $$ \varphi: (x,y) \mapsto (\beta x, -y). $$

As $E(\mathbb{F}\_q) \cong \mathbb{Z}/r\mathbb{Z}$, and the natural map $\mathbb{F}\_r \rightarrow \mathrm{End}(\mathbb{Z}/r\mathbb{Z})$ (the abelian-group endomorphisms) is an isomorphism, $\varphi$ corresponds to $\zeta \in \mathbb{F}\_r$ satisfying $$ \zeta^2 - \zeta + 1 = 0. $$ In particular, $\lambda := -\zeta$ is a cube root of unity in $\mathbb{F}\_r$ and satisfies $\lambda^2 + \lambda + 1 = 0$.

Given $s \in \mathbb{Z}/r\mathbb{Z}$, we can write $s = z\_1 - \lambda z\_2 = z\_1 + \zeta z\_2$ with "small" $z\_i$. Consider the lattice $L := \ker\big( \mathbb{Z}^2 \to \mathbb{Z}/r\mathbb{Z}\big)$, $(a,b)\mapsto a + \zeta b$. A fundamental domain around the origin lies inside a box with side length $B := \frac{\sqrt{3r}}{2} < 2^{128}$, hence $z\_i$ fit in 128 bits. See `split_into_endomorphism_scalars` method in the field module for details.

Column representation (a.k.a. the Input Trace)
----------------------------------------------

An operation in the OpQueue may be entered into a table as follows:

| `op` | `X` | `Y` | `z_1` | `z_2` | `mul_scalar_full` |

Here, `op` is the value of the operation, $(X, Y)$ are the *affine* coordinates of $P$, `mul_scalar_full` stands for "full scalar if the operation is `mul`" (so is an element of $\mathbb{F}\_r$), and `z_1` and `z_2` are a decomposition of `mul_scalar_full` as explained above. In particular, `z_1` and `z_2` may each be represented by 128 bits.

VM operations
-------------

The column representation is naturally equivalent to the representation as a VM operation.

struct ECCVMOperation {

using Curve = curve::BN254;

using AffineElement = Curve::Group::affine\_element;

using Fr = Curve::ScalarField;

EccOpCode op\_code = {};

AffineElement base\_point = { 0, 0 };

uint256\_t z1 = 0;

uint256\_t z2 = 0;

Fr mul\_scalar\_full = 0;

bool operator==(const ECCVMOperation& other) const = default;

};

Op Queue
--------

From the perspective of the ECCVM, the `ECCOpQueue` just contains a list of `ECCVMOperation`s, i.e., it is just an Input Trace. It is worth noting that the `ECCOpQueue` class indeed contains more moving parts, to link together the ECCVM with the rest of the Goblin protocol.

State Machine and the execution trace
-------------------------------------

As explained, the `ECCOpQueue` corresponds to a one-register finite state machine whose primitives are a set of operations on our elliptic curve.

From this perspective, the goal of the ECCVM is to compile the execution of this state machine. The ECCVM takes in an `ECCOpQueue`, which corresponds to the execution of a list of operations in BN-254, and constructs three tables, together with a collection of multivariate polynomials for each table, along with some lookups and multiset constraints. (The number of variables of a polynomial associated with a table is precisely the number of columns of that table.) Then the key claim is that if (1) the polynomials associated to each table vanish on every row, (2) the lookups are satisfied, and some multi-set equivalences hold (which mediate *between* tables), then the tables corresponds to the correct execution of the `ECCOpQueue`, i.e., to the correct execution of the one-register elliptic curve state machine.

Breaking abstraction, the *reason* we choose this model of witnessing the computation is that it is straightforward to SNARK.

Architecture
============

In trying to build the execution trace of `ECCOpQueue`, the `mul` opcode is the only one that is non-trivial to evaluate, especially efficiently. One straightforward way to encode the `mul` operation is to break up the scalar into its bit representation and use a double-and-add procedure. We opt for the Straus MSM algorithm with $w=4$, which requires more precomputing but is significantly more efficient.

High level summary of the operation of the VM
---------------------------------------------

Before we dive into the Straus algorithm, here is the high-level organization. We go "row by row" in the `ECCOpQueue`; if the instruction is *not* a `mul`, the `Transcript` table handles it. If it is a `mul` operation, it is *automatically* part of an MSM (potentially one of length 1), and we defer evaluation to the Straus mechanism (which involves two separate tables: an `MSM` table and a `Precomputed` table). Eventually, at the *end* of an MSM (i.e., if an op is a `mul` and the next op is not), the Transcript Columns will pick up the claimed evaluation from the MSM tables and continue along their merry way.

To do this in a moderately efficient manner is involved; we include logic for skipping computations when we can. For instance, if we have a `mul` operation with the base point $P=\mathcal{O}$, then we will have a column that bears witness to this fact and skip the explicit scalar multiplication. Analogously, if the scalar is 0 in a `mul` operation, we also encode skipping the explicit scalar multiplication. This does not merely allow us to save work; it dramatically simplifies the actual MSM computations (especially recursively), by throwing out circumstances when there can be case logic. However, this, together with the delegation of work to multiple tables, itself required by the Straus algorithm, nonetheless results in somewhat complicated column structure.

However, at least some of this complexity is forced on us; in Barretenberg, we represent the $\mathcal{O}$ of an elliptic curve in Weierstrass form as $(0, 0)$ for efficiency. (Note that $\mathcal{O}$ is always chosen to be the point-at-infinity and in particular it has no "affine representation". Note further that $(0, 0)$ is indeed not a point on our elliptic curve!) These issues are worth keeping in mind when examining the ECCVM.

Straus Algorithm for MSM
========================

Recall, our high-level goal is to compute $$\sum\_{i=0}^{m-1} s\_i P\_i,$$ where $s\_i\in \mathbb F\_r$ and $P\_i$ are points on BN-254, i.e., we want to evaluate a multi-scalar multiplication of length $m$. We set $w=4$, as this is our main use-case. (In the code, this is represented as `static constexpr size_t NUM_WNAF_DIGIT_BITS = 4;`.) We have seen about that, setting $P^\prime\_i:=\varphi(P\_i) = \lambda P\_i$, we may write $s\_iP\_i = z\_{i, 1}P\_i - z\_{i, 2}P^\prime\_i$, where $z\_{i,j}$ has no more than 128 bits. We therefore assume that our scalars have no greater than 128 bits.

wNAF
----

The first thing to specify is our windowed non-adjacent form (wNAF). This is an optimization for computing scalar multiplication. Moreover, the fact that we are working with an elliptic curve in Weierstrass form effectively halves the number of precomputes we need to perform.

**Warning**: our implementation is *not* what is usually called wNAF. To avoid confusion, we simply avoid discussion on traditional (w)NAF.

Here is the key mathematical claim: for a 128-bit positive number $s$, we can uniquely write: $$s = \sum\_{j=0}^{31} a\_j 2^{4j} - \text{skew},$$ where

* each $a\_j\in \lbrace -2^{4}+1, -2^{4}+3,\ldots, 2^{4}-1\rbrace$
* $\text{skew}\in\lbrace 0, 1\rbrace$.

In our implementation, we force $a\_{31}>0$ to guarantee that $s$ is positive. Note that the exponent in the range of the digits $a\_j$ is determined by `NUM_WNAF_DIGIT_BITS` $= w = 4$. The existence of the `skew` bit is to ensure that we can represent *even* numbers.

The above decomposition is referred to in the code as the wNAF representation. Each $a\_i$ is referred to either as a wNAF slice or digit.

We will come shortly to the algorithm, but as for the motivation: in our implementation, the neutral point of the group (i.e., point-at-infinity) poses some technical challenges for us. We work with the *affine* representation of elliptic curve points, and $\mathcal{O}$ certainly has no natural affine-coordiante representation. We choose to internally represent it as $(0, 0)$ (not a point on our curve!) and handle it with separate logic. It is therefore advantageous to avoid having to extraneously perform operations involving $\mathcal{O}$, especially when we are implementing the recursive ECCVM verifier.

Straus
------

Here is the problem: efficiently compute $$\sum\_i s\_i P\_i,$$ where the $s\_i$ are 128-bit numbers and $P\_i$ are points in BN-254. (Recall that we reduce to the case of 128-bit scalars by decomposing, as explained above.)

To do this, we break up our computation into steps.

### Precomputation

For each $s\_i$, we expand it in wNAF form:$s\_i = \sum\_{j=0}^{31} a\_{i, j} 2^{4j} - \text{skew}\_i$.

For every $P\_i$, precompute and store the multiples: $$\lbrace -15P\_i, -13P\_i, \ldots, 13P\_i, 15P\_i\rbrace$$ as well as $2P\_i$. Note that, $E$ is represented in Weierstrass form, $nP$ and $-nP$ have the same affine $x$-coordinate and the $y$-coordinates differ by a sign.

### Algorithm

Here are the static variables we need.

* `NUM_WNAF_DIGITS_PER_SCALAR=32`.
* `NUM_WNAF_DIGIT_BITS = 4`.
* `ADDITIONS_PER_ROW = 4`. This says that we can do 4 primitive EC additions per "row" of the virtual machine.

1. Set $A = \mathcal{O}$ to be the neutral element of the group.
2. For $j\in [0, \ldots, 31]$, do:
   1. For $k\in [0,\ldots, \lceil \frac{m-1}{4}\rceil]$ (here, $k$ is the "row" in the VM), do:
      1. Set $A\leftarrow A + a\_{4k, 31-j}P\_{4k} + a\_{4k+1, 31-j}P\_{4k+1} + a\_{4k+2, 31-j}P\_{4k+2} + a\_{4k+3, 31-j}P\_{4k+3}$, where the individual scalar multiples are *looked up* from the precomputed tables indicated in precomputation. (No accumulations if the points $P\_{4k+j}$ don't exist, which can potentially hold for $k=\lceil \frac{m-1}{4}\rceil$ and some $j$.)
   2. If $j\neq 31$, set $A\leftarrow 2^w A= 16 A$.
3. For $j = 32$, do:
   1. For $k\in [0,\ldots, \lceil \frac{m-1}{4}\rceil]$, do:
      1. Set $A\leftarrow A + \sigma\_{4k}P\_{4k} + \sigma\_{4k+1}P\_{4k+1} + \sigma\_{4k+2}P\_{4k+2} + \sigma\_{4k+3}P\_{4k+3}$, where $\sigma\_i$ is the `skew` bit of $P\_i$.
4. Return $A$.

We picture this algorithm as follows. We build a table, the $i^{\text{th}}$ row of which is the wNAF expansion of $s\_i$ in most-significant to least-significant order. This means that the first column corresponds to the most significant digit ($a\_{-, 31}$).

We work column by column (this is the $j$-loop); for every vertical chunk of 4 elements, we accumulate (i.e., add to an accumulator $A$) looked up values corresponding to the digit/base-point pair. In the pseudo-code, we have an index $31-j$ because we want to proceed in decreasing order of significant digits. (Looking forward, a "row" of the MSM table in the ECCVM can handle 4 such additions.) We do this until we exhaust the column. We then multiply the accumulator by $16$ (as long as we are not at the last digit) and go to the next column. Finally, at the end we handle the `skew` digit.

Tables
======

We have three tables that mediate the computation. As explained above, all of the computations are easy except for scalar multiplications. We process the computation and chunk what looks like scalar multiplications into MSMs. Here is the brief outline.

* `transcript_builder`. The transcript columns organize and process all of the computations *except for the scalar multiplications*. In particular, the Transcript Columns *do not bear witness* to the intermediate computations necessary for MSMs. However, they still "access" the results of these computations.
* `precomputed_tables_builder`. The precomputed columns are: for every $P$ that occurs in an MSM (which was syntactically pulled out by the `transcript_builder`), we compute/store $\lbrace P, 3P, \ldots, 15P, 2P\rbrace$.
* `msm_builder` actually computes/constrains the MSMs via the Straus algorithm.

A final note: apart from four Lagrange (precomputed selector) columns — `lagrange_first`, `lagrange_second` (the hiding-op row), `lagrange_third` (the first real-op row), and `lagrange_last` — and the ZK `gemini_masking_poly`, all columns are either 1. part of the input trace; or 2. witness columns committed to by the Prover.

In the following tables, each column has a defined "value range". If the range is not $\mathbb{F}\_q$, the column must be range constrained, either with an explicit range check or implicitly through range constraints placed on other columns that define relations over the target column.

Transcript Columns
------------------

| column name | builder name | value range | computation | description |
| --- | --- | --- | --- | --- |
|  |  |  | **Populated in the first loop** |  |
| `transcript_msm_infinity` | transcript\_msm\_infinity | $\lbrace 0, 1 \rbrace$ | `msm_output.is_point_at_infinity();` | are we at the end of an MSM *and* is the output the point at infinity? |
| `transcript_accumulator_not_empty` | accumulator\_not\_empty | $\lbrace 0, 1 \rbrace$ | `row.accumulator_not_empty = !state.is_accumulator_empty;`, `final_row.accumulator_not_empty = !updated_state.is_accumulator_empty;` | not(is the accumulator either empty or point-at-infinity?) |
| `transcript_add` | q\_add | $\lbrace 0, 1 \rbrace$ |  | is opcode |
| `transcript_mul` | q\_mul | $\lbrace 0, 1 \rbrace$ |  | is opcode |
| `transcript_eq` | q\_eq | $\lbrace 0, 1\rbrace$ |  | is opcode |
| `transcript_reset_accumulator` | q\_reset\_accumulator | $\lbrace 0, 1 \rbrace$ |  | does opcode reset accumulator? |
| `transcript_msm_transition` | msm\_transition | $\lbrace 0, 1\rbrace$ | `msm_transition = is_mul && next_not_msm && (state.count + num_muls > 0);` | are we at the end of an msm? i.e., is current transcript row the final `mul` opcode of a MSM |
| `transcript_pc` | pc | $\mathbb{F}\_q$ | `updated_state.pc = state.pc - num_muls;` | *decreasing* point counter. Only takes into count `mul` operations, not `add` operations. |
| `transcript_msm_count` | msm\_count | $\mathbb{F}\_q$ | `updated_state.count = current_ongoing_msm ? state.count + num_muls : 0;` | Number of muls so far in the *current* MSM (NOT INCLUDING the current step) |
| `transcript_msm_count_zero_at_transition` | msm\_count\_zero\_at\_transition | $\lbrace 0, 1\rbrace$ | `((state.count + num_muls) == 0) && entry.op_code.mul && next_not_msm;` | is the number of scalar muls we have completed at the end of our "MSM block" zero? (note that from the definition, if this variable is non-zero, then `msm_transition == 0`.) |
| `transcript_Px` | base\_x | $\mathbb{F}\_q$ |  | (input trace) $x$-coordinate of base point $P$ |
| `transcript_Py` | base\_y | $\mathbb{F}\_q$ |  | (input trace) $y$-coordinate of base point $P$ |
| `transcript_base_infinity` | base\_infinity | $\lbrace 0, 1\rbrace$ |  | is $P=\mathcal{O}$? |
| `transcript_z1` | z1 | $[0,2^{128})$ |  | (input trace) first part of decomposed scalar |
| `transcript_z2` | z2 | $[0,2^{128})$ |  | (input trace) second part of decomposed scalar |
| `transcript_z1zero` | z1\_zero | $\lbrace 0, 1\rbrace$ |  | is z1 zero? |
| `transcript_z2zero` | z2\_zero | $\lbrace 0, 1\rbrace$ |  | is z2 zero? |
| `transcript_op` | op\_code | $\in \mathbb{F}\_q$ | `entry.op_code.value();` | 8 `q_add` + 4 `q_mul` + 2 `q_eq` + `q_reset` |
|  |  |  | **Populated after converting from projective to affine coordinates** |  |
| `transcript_accumulator_x` | accumulator\_x | $\mathbb{F}\_q$ |  | x-coordinate of accumulator $A$ |
| `transcript_accumulator_y` | accumulator\_y | $\mathbb{F}\_q$ |  | y-coordinate of accumulator $A$ |
| `transcript_msm_x` | msm\_output\_x | $\mathbb{F}\_q$ |  | if we are at the end of an MSM, (output of MSM) + `offset_generator()` = `(msm_output_x, msm_output_y)`, else 0 |
| `transcript_msm_y` | msm\_output\_y | $\mathbb{F}\_q$ |  | if we are at the end of an MSM, (output of MSM) + `offset_generator()` = `(msm_output_x, msm_output_y)`, else 0 |
| `transcript_msm_intermediate_x` | transcript\_msm\_intermediate\_x | $\mathbb{F}\_q$ |  | if we are at the end of an MSM, (output of MSM) = `(transcript_msm_intermediate_x, transcript_msm_intermediate_y)`, else 0 |
| `transcript_msm_intermediate_y` | transcript\_msm\_intermediate\_y | $\mathbb{F}\_q$ |  | if we are at the end of an MSM, (output of MSM) = `(transcript_msm_intermediate_x, transcript_msm_intermediate_y)`, else 0 |
| `transcript_add_x_equal` | transcript\_add\_x\_equal | $\lbrace 0, 1\rbrace$ | `(vm_x == accumulator_x) or (vm_infinity && accumulator_infinity);` | do the accumulator and the point we are adding have the same $x$-value? (here, the two point we are adding is either part of an `add` instruction or the output of an MSM). 0 if we are not accumulating anything. |
| `transcript_add_y_equal` | transcript\_add\_y\_equal | $\lbrace 0, 1\rbrace$ | `(vm_y == accumulator_y) or (vm_infinity && accumulator_infinity);` | do the accumulator and the point we are adding have the same $y$-value? 0 if we are not accumulating anything. |
| `transcript_base_x_inverse` | base\_x\_inverse | $\mathbb{F}\_q$ |  | if adding a point to the accumulator and the $x$ values are not equal, the inverse of the difference of the $x$ values. (witnesses `transcript_add_x_equal == 0` |
| `transcript_base_y_inverse` | base\_y\_inverse | $\mathbb{F}\_q$ |  | if adding a point to the accumulator and the $y$ values are not equal, the inverse of the difference of the $y$ values. (witnesses `transcript_add_y_equal == 0` |
| `transcript_add_lambda` | transcript\_add\_lambda | $\mathbb{F}\_q$ |  | if adding a point into the accumulator, contains the lambda gradient: the slope of the line between $A$ and $P$ |
| `transcript_msm_x_inverse` | transcript\_msm\_x\_inverse | $\mathbb{F}\_q$ |  | used to validate transcript\_msm\_infinity correct; if the former is zero, this is the inverse of the $x$ coordinate of the (non-shifted) output of the MSM |
| `transcript_msm_count_at_transition_inverse` | msm\_count\_at\_transition\_inverse | $\mathbb{F}\_q$ |  | used to validate transcript\_msm\_count\_zero\_at\_transition |

Transcript description and algorithm
------------------------------------

In the above table, we have a reference what the transcript columns are. Here, we provide a natural-language summary of witness-generation, which in turn directly implies what the constraints should look like. Some of the apparent complexity comes from the fact that, for efficiency, we do operations in *projective coordinates* and then normalize them all at the end. (This requires fewer field-inversions.)

We start our top row with `transcript_msm_count = 0` and `transcript_accumulator_not_empty = 0`. This corresponds to saying "there are no active multiplications in our MSM" and "the accumulator is $\mathcal{O}$".

We process each `op`.

If the `op` is an `add`, we process the addition as follows. We have an accumulated value $A$ and a point $P$ to add. If `transcript_base_infinity = 1`, we don't need to do anything: $P=\mathcal{O}$. Similarly, if `transcript_accumulator_not_empty = 0`, then we just (potentially) need to change `transcript_accumulator_not_empty`, `transcript_accumulator_x` and `transcript_accumulator_y`. Otherwise, we need to check `transcript_add_x_equal`: the formula for point addition requires dividing by $\Delta x$, and in particular is not well-constrained either when adding points that are negative of each other or adding the same point to itself. (These two cases may be easily distinguished by examining `transcript_add_y_equal`). If we are *not* in this case, we need the help of of `transcript_add_lambda`, which is the slope between the points $P$ and $A$. (This slope will happily not be $\infty$, as we have ruled out the only occasions it had to be.)

The value $A\leftarrow A + P$ will of course involve different `transcript_accumulator_x` and `transcript_accumulator_y`, but may also cause `transcript_accumulator_not_empty` to flip.

We emphasize: we *do not* modify `transcript_pc` in this case. Indeed, that variable is only modified based on the number of small scalar `mul`s we are doing.

If the `op` is `eq`, we process the op as follows. We have an accumulated value $A$ and a point $P$. Due to our non-uniform representation of $\mathcal{O}$, we must break up into cases.

* Both are $\mathcal{O}$ (i.e., `transcript_accumulator_not_empty = 0` and `transcript_base_infinity=1`). Then accept!
* Neither is equal to $\mathcal{O}$. Then we linearly compare `transcript_accumulator_x-transcript_Px` and `transcript_accumulator_y-transcript_Py` and accept if both are $0$.
* Exactly one is equal to $\mathcal{O}$. Then reject!

If our `op` is `eq_reset`, we do the same as for `eq`, but we also set `transcript_accumulator_not_empty` $\leftarrow 0$.

If our `op` is a `mul`, with scalars `z1` and `z2`, the situation is more complicated. Now we have to update auxiliary wires. As explained, *every* `mul` operation is understood to be part of an MSM.

* `transcript_msm_count` counts the number of active short-scalar multiplications *up to and not including* the current `mul` op. The value of this column at the *next* row increments by `2 - transcript_z1zero - transcript_z2zero`.
* In other words, we simply avoid (our deferred) computations if `transcript_z1zero = 1` and/or `transcript_z2zero = 1`.
* Similarly, `transcript_pc` *decrements* by `2 - transcript_z1zero - transcript_z2zero`. We use a decreasing point counter (only counting short `mul`s) for efficiency reasons, as it allows for cheaper commitments.
* If the next `op` is not a `mul`, and the total number of active `mul` operations (which is `transcript_msm_count + (2 - transcript_z1zero - transcript_z2zero)`) is non-zero, set the `transcript_msm_transition = 1`. Else, set `transcript_msm_count_zero_at_transition = 1`. Either way, the current `mul` then represents the end of an MSM. This is where `transcript_msm_count_at_transition_inverse` is used.
* If `transcript_msm_transition = 0`, then `transcript_msm_x`, `transcript_msm_y`, `transcript_msm_intermediate_x`, and `transcript_msm_intermediate_y` are all $0$. (In particular, this holds when we are in the middle of an MSM.) Otherwise, we call `transcript_msm_x` and `transcript_msm_y` from the multiset argument, i.e., from the MSM table. Then the values of `transcript_msm_intermediate_x` and `transcript_msm_intermediate_y` are obtained by subtracting off the `OFFSET`.

### Transcript size

The size of the *non-zero* part of the table is the length of the `OpQueue` + 1 (we have shiftable columns). We have organized our wire values so that zero-padding is compatible with the polynomial constraints. (See e.g. the *decreasing* point counter.)

Precomputed Columns
-------------------

As the set of precomputed columns is small, we include the code snippet.

struct PointTablePrecomputationRow {

int s1 = 0;

int s2 = 0;

int s3 = 0;

int s4 = 0;

int s5 = 0;

int s6 = 0;

int s7 = 0;

int s8 = 0;

bool skew = false;

bool point\_transition = false;

uint32\_t pc = 0;

uint32\_t round = 0;

uint256\_t scalar\_sum = 0;

AffineElement precompute\_accumulator{

0, 0

}; // contains a precomputed element, i.e., something in {P, 3P, ..., 15P}.

AffineElement precompute\_double{ 0, 0 };

};

As discussed in Decomposing Scalars, WLOG our scalars have 128 bits and we may expand them in $w=4$ wNAF:

$$s = \sum\_{j=0}^{31} a\_j 2^{4j} - \text{skew},$$ where

* each $a\_j\in \lbrace -2^{4}+1, -2^{4}+3,\ldots, 2^{4}-1\rbrace$
* $\text{skew}\in\lbrace 0, 1\rbrace$.

Given a wNAF digit $\in \lbrace -15, -13, \ldots, 15\rbrace$, we $\text{compress}$ it via the map: $$\text{compress}\colon d\mapsto \frac{d+15}{2},$$ which is of course a bijection $\lbrace -15, -13, \ldots, 15\rbrace\rightarrow \lbrace 0,\ldots, 15\rbrace$. (This compression is helpful for indexing later: looking forward, the values $[-15P, -13P, \ldots, 15P]$ will be stored in an array, so if we want to looking up $kP$, where $k\in \lbrace -15, -13, \ldots, 15\rbrace$, we can go to the $\text{compress}(k)$ index of our array associated to $P$.)

The following is one row in the Precomputed table; there are `NUM_WNAF_DIGITS_PER_SCALAR / WNAF_DIGITS_PER_ROW == 32/4 = 8` rows. The row index is `i`. (This number is is also witnessed as `round`.)

| column name | builder name | value range | computation | description |
| --- | --- | --- | --- | --- |
| `precompute_s1hi` | s1 | $[0, 4)$ |  | first two bits of $\text{compress}(a\_{31 - 4i})$ |
| `precompute_s1lo` | s2 | $[0, 4)$ |  | second two bits of $\text{compress}(a\_{31 - 4i})$ |
| `precompute_s2hi` | s3 | $[0, 4)$ |  | first two bits of $\text{compress}(a\_{31 - (4i + 1)})$ |
| `precompute_s2lo` | s4 | $[0, 4)$ |  | second two bits of $\text{compress}(a\_{31 - (4i + 1)})$ |
| `precompute_s3hi` | s5 | $[0, 4)$ |  | first two bits of $\text{compress}(a\_{31 - (4i + 2)})$ |
| `precompute_s3lo` | s6 | $[0, 4)$ |  | second two bits of $\text{compress}(a\_{31 - (4i + 2)})$ |
| `precompute_s4hi` | s7 | $[0, 4)$ |  | first two bits of $\text{compress}(a\_{31 - (4i + 3)})$ |
| `precompute_s4lo` | s8 | $[0, 4)$ |  | second two bits of $\text{compress}(a\_{31 - (4i + 3)})$ |
| `precompute_skew` | skew | $\lbrace 0,7\rbrace$ |  | skew bit |
| `precompute_point_transition` | point\_transition | $\lbrace 0,1\rbrace$ |  | are we at the last row corresponding to this scalar? |
| `precompute_pc` | pc | $\mathbb{F}\_q$ |  | value of the point counter of this EC operation |
| `precompute_round` | round | $\mathbb{F}\_q$ |  | "row" of the computation, i.e., `i`. |
| `precompute_scalar_sum` | scalar\_sum | $\mathbb{F}\_q$ |  | sum up-to-now of the digits |
| `precompute_tx`, `precompute_ty` | precompute\_accumulator | $E(\mathbb{F}\_q)\subset \mathbb{F}\_q\times \mathbb{F}\_q$ |  | $(15-2i)P$ |
| `precompute_dx`, `precompute_dy` | precompute\_double | $E(\mathbb{F}\_q)\subset \mathbb{F}\_q\times \mathbb{F}\_q$ |  | $2P$ |
| `precompute_select` |  | $\lbrace 0,1\rbrace$ |  | if 1, evaluate Straus precomputation algorithm at current row |

Precomputed Description and Algorithm
-------------------------------------

First, let us recall the structure of `ScalarMul`.

template <typename CycleGroup> struct ScalarMul {

uint32\_t pc;

uint256\_t scalar;

typename CycleGroup::affine\_element base\_point;

std::array<int, NUM\_WNAF\_DIGITS\_PER\_SCALAR>

wnaf\_digits; // [a\_{n-1}, a\_{n-1}, ..., a\_{0}], where each a\_i ∈ {-2ʷ⁻¹ + 1, -2ʷ⁻¹ + 3, ..., 2ʷ⁻¹ - 3, 2ʷ⁻¹ -

// 1}. (here, w = `NUM\_WNAF\_DIGIT\_BITS`). in particular, a\_i is an odd integer with

// absolute value less than 2ʷ. Represents the number `scalar` = ∑ᵢ aᵢ 2⁴ⁱ - `wnaf\_skew`.

bool wnaf\_skew; // necessary to represent \_even\_ integers

// size bumped by 1 to record base\_point.dbl()

std::array<typename CycleGroup::affine\_element, POINT\_TABLE\_SIZE + 1> precomputed\_table;

};

Note that, with respect to the decomposition in wnaf, `wnaf_digits[i]`= $a\_{31-i}$. Indeed, the order of the array `wnaf_digits` is from highest-order to lowest-order.

Given a `ScalarMul`, it is easy to construct the 8 rows of the Precomputed Table. As explained, `WNAF_DIGITS_PER_ROW = 4`; hence the `NUM_WNAF_DIGITS_PER_SCALAR = 32` digits in may be broken up into 8 rows, where each row corresponds to 4 wNAF digits, each of which is in $\lbrace -15, -13, \ldots, 13, 15\rbrace$.

1. For $i = 0 .. 7$
   1. For each of the 4 digits in the row: `wnaf_digits[4i]`, `wnaf_digits[4i+1]`, `wnaf_digits[4i+2]`, and `wnaf_digits[4i+3]`, `compress` from $\lbrace -15, -13, \ldots, 13, 15\rbrace\rightarrow \lbrace 0,\ldots 15\rbrace$ via the monotonic map $z\mapsto \frac{z+15}{2}$. Then our compressed digits are in the latter range.
   2. extract the first and last pair of bits and fill in order in corresponding parts of the table: `precompute_s1hi`, `precompute_s1lo`, `precompute_s2hi`, `precompute_s2lo`, `precompute_s3hi`, `precompute_s3lo`, `precompute_s4hi`, `precompute_s4lo` correspond to the 2-bit decompositions of the compressed wNAF digits.
   3. The value `precompute_point_transition` is set to 1 if this is the last row (i.e., `i == 7`) for the current scalar, else 0. This tracks if the next row in the table corresponds to a new `ScalarMul`.
   4. The value `precompute_pc` is copied from the corresponding `ScalarMul.pc`.
   5. The value `precompute_round` is set to the row index `i`.
   6. The value `precompute_scalar_sum` accumulates the *scalar reconstruction*: $\displaystyle \sum\_{j=0}^{4i+3} a\_{31-j} \cdot 2^{4j}$. (Here, our current row is $i$.) In other words: at row $i$, we implicit consider the string of digits `wnaf_digits[0]`, ..., `wnaf_digits[4i+3]`; `precompute_scalar_sum` is precisely the value of the $4i$-digit number corresponding to this string of digits.
   7. The value `(precompute_tx, precompute_ty)` stores the precomputed point $(15-2i)P$. (Note that this reflects a coincidence: the number of rows (per scalar multiplication) is same as the number of odd multiples of $P$ that we need to store.)
   8. The value `(precompute_dx, precompute_dy)` stores $2P$. (In particular, $2P$ is stored on all $8$ rows coming from a given `ScalarMul`.)

The constraints are straightforward.

* We must range constrain the `precompute_s1hi`, `precompute_s1lo`, `precompute_s2hi`, `precompute_s2lo`, `precompute_s3hi`, `precompute_s3lo`, `precompute_s4hi`, `precompute_s4lo`. We do this via the polynomial $((x-1)^2 - 1)((x-2)^2-1)$, a quartic constraint.
* We constrain that `precompute_scalar_sum` is updated correctly at each row.
* When `precompute_point_transition = 1`, when we constrain that original `scalar` is `precompute_scalar_sum - precompute_skew`.
* We constrain the elliptic curve values. Note that we may assume that $P\neq \mathcal{O}$; indeed, we only populate this table when we are doing non-trivial scalar multiplications. It follows that $nP\neq \mathcal{O}$ for $0<n< r$, as $r$ is prime. This means that the following constraints have *no special case analysis*:
  + if `precompute_point_transition = 1`, constrain `2P = (precompute_dx, precompute_dy)`
  + if `precompute_point_transition = 0`, constrain `(shift(precompute_dx), shift(precompute_dy)) = (precompute_dx, precompute_dy)`. (Here, `shift` means "the next value of the column".)
  + if `precompute_point_transition = 0`, constrain `(precompute_tx, precompute_ty) = (precompute_dx, precompute_dy) + (shift(precompute_tx), shift(precompute_ty))`, where the latter addition of course happens on $E$.
* We emphasize that these EC constraints will only be turned on after the first row, as these values have no *neutral* value that we can use for the first row (especially as it is critical that they are never $\mathcal{O}$).
* We constrain `precompute_round` as follows. (Note that it *is not* naively range-constrained.)
  + If `precompute_point_transition = 1`, then set `precompute_round = 7` and `shift(precompute_round) = 0`. (We are at the end of this block of precomputes for our `ScalarMul`), so if the next block is to be well-formed, the next round element better be $0$. Note that this is compatible with zero-padding.
  + If `precompute_point_transition = 0`, set `shift(precompute_round) - precompute_round = 1`.

Precomputed Size
----------------

For every *non-trivial* short scalar `mul`, we fill in $8$ non-trivial rows to the precomputed table. Here, non-trivial means: $P\neq \mathcal{O}$ and $z\neq 0$, where $z$ is the short (128-bit) scalar we are multiplying by. This means that for $m$ (non-trivial) short scalar `mul` operations, we add $8m$ rows to the precomputed table.

MSM columns
-----------

This table is the most algorithmically involved.

struct alignas(64) MSMRow {

uint32\_t pc = 0; // decreasing point-counter, over all half-length (128 bit) scalar muls used to compute

// the required MSMs. however, this value is \_constant\_ on a given MSM and more precisely

// refers to the number of half-length scalar muls completed up until we have started

// the current MSM.

uint32\_t msm\_size = 0; // the number of points in the current MSM. (this is \_constant\_ on MSM blocks.)

uint32\_t msm\_count = 0; // number of multiplications processed so far (not including this row) in current MSM

// round (a.k.a. wNAF digit slot). this specifically refers to the number of wNAF-digit

// \* point scalar products we have looked up and accumulated.

uint32\_t msm\_round = 0; // current "round" of MSM, in {0, ..., 32}. (final round deals with the `skew` bit.)

// here, 32 = `NUM\_WNAF\_DIGITS\_PER\_SCALAR`.

bool msm\_transition = false; // is 1 if the \*current\* row starts the processing of a different MSM, else 0.

bool q\_add = false;

bool q\_double = false;

bool q\_skew = false;

// Each row in the MSM portion of the ECCVM can handle (up to) 4 point-additions.

// For each row in the VM we represent the point addition data via a size-4 array of

// AddState objects.

struct AddState {

bool add = false; // are we adding a point at this location in the VM?

// e.g if the MSM is of size-2 then the 3rd and 4th AddState objects will have this set

// to `false`.

int slice = 0; // wNAF slice value. This has values in {0, ..., 15} and corresponds to an odd number in the

// range {-15, -13, ..., 15} via the monotonic bijection.

AffineElement point{ 0, 0 }; // point being added into the accumulator

FF lambda = 0; // when adding `point` into the accumulator via Affine point addition, the value of `lambda`

// (i.e., the slope of the line). (we need this as a witness in the circuit.)

FF collision\_inverse = 0; // collision\_inverse` is used to validate we are not hitting point addition edge

// case exceptions, i.e., we want the VM proof to fail if we're doing a point

// addition where (x1 == x2). to do this, we simply provide an inverse to x1 - x2.

};

std::array<AddState, 4> add\_state{ AddState{ false, 0, { 0, 0 }, 0, 0 },

AddState{ false, 0, { 0, 0 }, 0, 0 },

AddState{ false, 0, { 0, 0 }, 0, 0 },

AddState{ false, 0, { 0, 0 }, 0, 0 } };

FF accumulator\_x = 0;

FF accumulator\_y = 0;

};

| column name | builder name | value range | computation | description |
| --- | --- | --- | --- | --- |
| `msm_pc` | pc | $\mathbb{F}\_q$ |  | counter over all half-length (128 bit) scalar muls used to compute the required MSMs. constant on a given MSM, refers more precisely to the number of *completed* scalar muls up until the current MSM. in particular, this skips values, unlike `transcript_pc`. |
| `msm_size_of_msm` | msm\_size | $\mathbb{F}\_q$ |  | the number of points that will be scaled and summed |
| `msm_count` | msm\_count | $\mathbb{F}\_q$ | `row.msm_count = static_cast<uint32_t>(offset);` | number of wNAF-multiplications processed so far *in this round*. |
| `msm_round` | msm\_round | $[0, 32]$ |  | current "round" of MSM, in $\lbrace 0, \ldots, 32\rbrace$, which corresponds to the wNAF digit being processed. (final round deals with the `skew` bit.) |
| `msm_transition` | msm\_transition | $\lbrace 0, 1\rbrace$ | `(digit_idx == 0) && (row_idx == 0)` | is 1 if the current row starts the processing of a different MSM, else 0 . Note this *not* the same as the description of `transcript_msm_transition` |
| `msm_add` | q\_add | $\lbrace 0, 1\rbrace$ |  | 1 if we are adding points in the Straus MSM algorithm at current row |
| `msm_double` | q\_double | $\lbrace 0, 1\rbrace$ |  | 1 if we are doubling accumulator in the Straus MSM algorithm at current row |
| `msm_skew` | q\_skew | $\lbrace 0, 1\rbrace$ |  | 1 if we are incorporating skew points in the Straus MSM algorithm at current row |
| `msm_x1` | add\_state[0].point.x | $\mathbb{F}\_q$ |  | $x$-coordinate of first potential point (corresponding to add\_state[0]) to add in Straus MSM round |
| `msm_y1` | add\_state[0].point.y | $\mathbb{F}\_q$ |  | $y$-coordinate of first potential point (corresponding to add\_state[0]) to add in Straus MSM round |
| `msm_x2` | add\_state[1].point.x | $\mathbb{F}\_q$ |  | $x$-coordinate of second potential point (corresponding to add\_state[1]) to add in Straus MSM |
| `msm_y2` | add\_state[1].point.y | $\mathbb{F}\_q$ |  | $y$-coordinate of second potential point (corresponding to add\_state[1]) to add in Straus MSM |
| `msm_x3` | add\_state[2].point.x | $\mathbb{F}\_q$ |  | x-coordinate of third potential point (corresponding to add\_state[2]) to add in Straus MSM round |
| `msm_y3` | add\_state[2].point.y | $\mathbb{F}\_q$ |  | y-coordinate of third potential point (corresponding to add\_state[2]) to add in Straus MSM round |
| `msm_x4` | add\_state[3].point.x | $\mathbb{F}\_q$ |  | x-coordinate of fourth potential point (corresponding to add\_state[3]) to add in Straus MSM round |
| `msm_y4` | add\_state[3].point.y | $\mathbb{F}\_q$ |  | y-coordinate of fourth potential point (corresponding to add\_state[3]) to add in Straus MSM round |
| `msm_add1` | add\_state[0].add | $\lbrace 0, 1\rbrace$ |  | are we adding msm\_x1/msm\_y1 (resp. add\_state[0]) into accumulator at current round? |
| `msm_add2` | add\_state[1].add | $\lbrace 0, 1\rbrace$ |  | are we adding msm\_x2/msm\_y2 (resp. add\_state[1]) into accumulator at current round? |
| `msm_add3` | add\_state[2].add | $\lbrace 0, 1\rbrace$ |  | are we adding msm\_x3/msm\_y3 (resp. add\_state[2]) into accumulator at current round? |
| `msm_add4` | add\_state[3].add | $\lbrace 0, 1\rbrace$ |  | are we adding msm\_x4/msm\_y4 (resp. add\_state[3]) into accumulator at current round? |
| `msm_slice1` | add\_state[0].slice | $[0, 15]$ |  | wNAF slice value (a.k.a. digit) for first point (corresponds to odd number in $\lbrace -15, -13, \ldots, 15\rbrace$ via the monotonic bijection) |
| `msm_slice2` | add\_state[1].slice | $[0, 15]$ |  | wNAF slice value (a.k.a. digit) for second point |
| `msm_slice3` | add\_state[2].slice | $[0, 15]$ |  | wNAF slice value (a.k.a. digit) for third point |
| `msm_slice4` | add\_state[3].slice | $[0, 15]$ |  | wNAF slice value (a.k.a. digit) for fourth point |
| `msm_lambda1` | add\_state[0].lambda | $\mathbb{F}\_q$ |  | if add\_state[0].add==1 (eqiv. if msm\_add1 == 1), slope of the line between the two points being added. else 0. |
| `msm_lambda2` | add\_state[1].lambda | $\mathbb{F}\_q$ |  | if add\_state[1].add==1 (eqiv. if msm\_add2 == 1), slope of the line between the two points being added. else 0. |
| `msm_lambda3` | add\_state[2].lambda | $\mathbb{F}\_q$ |  | if add\_state[2].add==1 (eqiv. if msm\_add3 == 1), slope of the line between the two points being added. else 0. |
| `msm_lambda4` | add\_state[3].lambda | $\mathbb{F}\_q$ |  | if add\_state[3].add==1 (eqiv. if msm\_add3 == 1), slope of the line between the two points being added. else 0. |
| `msm_collision_x1` | add\_state[0].collision\_inverse | $\mathbb{F}\_q$ |  | if add\_state[0].add == 1, the difference of the $x$ values of the accumulator and the point being added. used to ensure incomplete ecc addition exceptions not triggered if msm\_add1 = 1 |
| `msm_collision_x2` | add\_state[1].collision\_inverse | $\mathbb{F}\_q$ |  | if add\_state[1].add == 1, the difference of the $x$ values of the accumulator and the point being added. |
| `msm_collision_x3` | add\_state[2].collision\_inverse | $\mathbb{F}\_q$ |  | if add\_state[2].add == 1, the difference of the $x$ values of the accumulator and the point being added. |
| `msm_collision_x4` | add\_state[3].collision\_inverse | $\mathbb{F}\_q$ |  | if add\_state[3].add == 1, the difference of the $x$ values of the accumulator and the point being added. |
| `msm_accumulator_x` | accumulator\_x | $\mathbb{F}\_q$ |  | (accumulator\_x, accumulator\_y) = $A$ is the accumulated point |
| `msm_accumulator_y` | accumulator\_y | $\mathbb{F}\_q$ |  | $y$-coordinate of the accumulated point $A$ |
| `msm_round_minus_31_inv` | msm\_round\_minus\_31\_inv | $\mathbb{F}\_q$ |  | witness used to test whether a row is at the final addition round `msm_round == 31`: the inverse of `msm_round - 31` on the rows that read it (masked elsewhere). Used by the round-transition / skew constraints `ROUND_TRANSITION_SKEW_IMPLIES_ROUND_31` and `DOUBLE_SHIFT_FORBIDS_ROUND_31` of `ECCVMMSMRelation`. |

MSM algorithm and description
-----------------------------

We have already given a high-level summary of the Straus algorithm. Let us get into the weeds!

The function signature is the following:

static std::tuple<std::vector<MSMRow>, std::array<std::vector<size\_t>, 2>> compute\_rows(

const std::vector<MSM>& msms, const uint32\_t total\_number\_of\_muls, const size\_t num\_msm\_rows)

In other words, `compute_rows` takes in a vector of MSMs (each of which is a vector of `ScalarMul`s), together with the total number of non-zero `mul` operations we compute and the (easy-to-compute) a priori size bound `num_msm_rows`, and returns a vector of `MSMRow`s and two vectors, which will represent our point-counts (i.e., will be fed into the lookup argument).

Before we get into the content, note that we may assume that no point is $\mathcal{O}$ in any of the MSMs. Indeed, this is due to checks done by the Transcript Columns. However, it is in principle possible that some of the scalars are $0$; we do not force `transcript_z1zero = 0` $\Rightarrow$ `transcript_z1 != 0`

Each row (after the first row) in the MSM table will belong to one of the MSMs we are assigned to compute in `msms`. For an `msm` of size `m`, the number of rows that will be added in the MSM table is (substituting `NUM_WNAF_DIGITS_PER_SCALAR = 32` and `ADDITIONS_PER_ROW = 4`):

$$(32 + 1)\left\lceil \frac{m}{4}\right\rceil + (32 - 1) = 33\frac{m}{4} + 31.$$ There is one other quirk we should explicate before entering the algorithm. In general, the logic for affine elliptic curve addition can have cases: when the $x$ coordinates match up. (Doubling cannot have cases for points on our affine elliptic curve because there is no $\mathbb{F}\_q$-rational $2$-torsion.) Moreover, in general our logic must branch if either our base or the accumulator is $\mathcal{O}$. As we have indicated several times above, for optimization, we *represent* $\mathcal{O}$ as $(0, 0)$ in the code. It is advantageous to avoid this branching logic. We do so by *relaxing completeness*. In particular, we start off the accumulator of every MSM with a fixed `offset_generator`. This is a fixed point of $E$ that we may consider pseudo-random (though it is fixed and indeed hardcoded). Then we decree that for our MSM to be valid, in the course of the Straus algorithm, whenever I accumulate $A\leftarrow A + P$, the $x$-coordinates of $A$ and $P$ differ. This condition of being valid may be witnessed by the prover providing the inverse of the difference of the $x$-coordinates every time it is necessary.

This indeed breaks completeness, inasmuch as there are valid `EccOpQueue`s which will not be able to be compiled into a valid execution trace. However, this is vanishingly unlikely, in the course of any normal operations.

Finally, we may describe the algorithm. We implicitly organize our data in the following type of table (as indicated in the Straus Section). Each row of our table corresponds to a scalar multiplication: the elements of the row are the wNAF digits (including the `skew` bit). In other words, the columns of our table correspond to wNAF digits. Our algorithm will proceed column by column, from most significant to least significant digit, processing one vertical chunk of four elements after another. To emphasize: this table syntactically encoding our MSM is *not* what we refer to as the MSM table of the VM, which rather witnesses the correct execution of the MSM.

1. Set the first row of the MSM table (of our VM) to be 0.
2. Initialize lookup table read counts: `point_table_read_counts[0]` and `point_table_read_counts[1]` to track the positive and negative lookups corresponding to $nP$, where $n\in \lbrace -15, -13, \ldots, 13, 15\rbrace$. Each table will have size `total_number_of_muls * 8` (since `POINT_TABLE_SIZE/2 = 8`).
3. Compute the MSM row boundaries: for each MSM, fill out the indices of where it starts and the starting `msm_pc`. This requires a calculation of the number of rows required, which we come back to in the next section.
4. First pass: populate `point_table_read_counts` based on `msm[point_idx].wnaf_digits[digit_idx]`. Update read counts based on skew as well.

We deviate from the witness generation algorithm here. In the code, in order to minimize the number of field divisions, we first compute in projective coordinates, then batch-normalize back to affine to fill in the values affine values. Here we just specify the values of the various columns in a more naive way.

1. Set the accumulator at the beginning of every `msm` to be `offset_generator`. (This allows us to avoid case-logic in EC addition.)
2. For `digit-position` (a.k.a. column of my syntactic MSM table) in $0..31$:
   1. Populate the rows of the VM's MSM table as follows.
      1. Check if the row corresponds to a new `msm`. If so, set `msm_transition = 1`.
      2. Process the (no greater than) `ADDITIONS_PER_ROW` points per row:
         1. Get the up-until-now value of the accumulator and set into `(msm_accumulator_x, msm_accumulator_y)`. For the first row of an MSM, this is `offset_generator`, for a non-first row of an MSM this involves processing the previous row of the MSM table.
         2. Set `msm_add = 1`, `msm_double = 0`, and `msm_skew = 0`.
         3. Set the booleans `msm_add1`, `msm_add2`, `msm_add3`, and `msm_add4` to the correct values (all should be one if we haven't yet exhausted the column, if we are at the end of a column and $m$ is not divisible by 4, only the first $m\text{ mod} 4$ should be turned on).
         4. For each point that is "on", record the following (which all correspond to members of `AddState`):
            1. the slice a.k.a. digit value. (This has values in $\lbrace 0,\ldots,15\rbrace$ and corresponds to the elements $\lbrace -15, -13, \ldots, 13, 15\rbrace$.) These are filled in `msm_slice1`, `msm_slice2`, `msm_slice3`, and `msm_slice4`.
            2. The precomputed value of the slice/digit times the corresponding base point. These are filled in `msm_x1`, `msm_y1`, `msm_x2`, `msm_y2`, `msm_x3`, `msm_y3`, and `msm_x4`, `msm_y4`. Note that, as we are proceeding vertically, the base points corresponding to `msm_slice1`, `msm_slice2`, `msm_slice3`, and `msm_slice4` may very well all be different.
            3. Auxiliary values needed to compute the sum of the accumulator and the points-to-be-added into the accumulator: in particular, the slope of the line between the (intermediate) accumulator and the point-to-be-added. These are contained in `msm_lambda1`, `msm_lambda2`, `msm_lambda3`, and `msm_lambda4`. Here, there is a subtle point: we do not explicitly record the intermediate values of the accumulator in this row in our VM's MSM table, although `msm_lambda2`, `msm_lambda3`, and `msm_lambda4` reflect these values. Indeed, if $Q\_1$ has coordinates `(msm_x1, msm_y1)`, $Q\_2$ has coordinates `(msm_x2, msm_y2)`, and our accumulator is starting at $A$, then `msm_lambda1` is the slope between the line $A$ and $Q\_1$, while `msm_lambda2` is the slope between the line $A+Q\_1$ and $Q\_2$. However, $A + Q\_1$ is *not* explicitly recorded in our MSM table.
            4. For each point that is "on", fill in the following values `msm_collision_x1`, `msm_collision_x2`, `msm_collision_x3`, and `msm_collision_x4`. These are the differences in the $x$ values between the (intermediate) accumulator and the point-to-be-added. This witnesses/verifies the fact that we don't have edge-case logic for the addition. As with the $\lambda$ values, these reflect the intermediate values of the accumulator although that intermediate value is *not* explicitly recorded in our MSM table.
      3. Process the 4 doublings, as long as we are not at the last wnaf digit. This involves adding a *single* row to the MSM table.
         1. Set `msm_add = 0`, `msm_double = 1`, and `msm_skew = 0`.
         2. Get the value of `msm_accumulator_x` and `msm_accumulator_y` from the last row.
         3. The values: `msm_count`, `msm_transition`, `msm_slice1`, `msm_slice2`, `msm_slice3`, `msm_slice4`, `msm_x1`, `msm_y1`, `msm_x2`, `msm_y2`, `msm_x3`, `msm_y3`, `msm_x4`, `msm_y4`, `msm_collision_x1`, `msm_collision_x2`, `msm_collision_x3`, and `msm_collision_x4` are all set to $0$.
         4. We set `msm_lambda1`, `msm_lambda2`, `msm_lambda3`, and `msm_lambda4` correctly: they are each the slope of the line passing through the current *intermediate* accumulator tangent to $E$. For instance, `msm_lambda1` is the slope of the line through $A$, `msm_lambda2` is the slope through $2A$, etc.
      4. Process the skew digit in an analogous way to the processing of the additions.

MSM size
--------

Suppose we have an MSM of short scalars of size $m$. Then the number of rows we add to the MSM table of the VM is (with `NUM_WNAF_DIGITS_PER_SCALAR = 32` and `ADDITIONS_PER_ROW = 4`):

$$(32 + 1)\left\lceil \frac{m}{4}\right\rceil + (32 - 1) = 33\frac{m}{4} + 31.$$ Indeed, there are $\lceil m/4 \rceil$ `add`-rows per digit (`ADDITIONS_PER_ROW = 4`), and there are `NUM_WNAF_DIGITS_PER_SCALAR + 1` $= 33$ digits per scalar (where the last digit is the `skew` digit). Finally, the last term comes from the doublings.

Note that in the regime where we have a few long MSMs, this is asymptotic to $8.25m$, which is comparable to the $8m$ we get from the precomputed columns. On the other hand, if we have many very short MSMs, the size of this table dominates what was produced by the precomputed columns.

Multisets and Lookups
=====================

As explained in the introduction, we sometimes treat these three sets of disjoint columns as three separate tables. There must be a mechanism to ensure that they "communicate" with each other. We do *not* use bare copy-constraints; instead, we use three multisets equality checks (also called "strict lookup arguments", where every write has precisely one corresponding read). The goal of these section is to sketch how these constraints, together with the lookups, fully piece together the ECCVM. We emphasize that this is merely a sketch; for full details, please see the [set relation](../relations/ecc_vm/ecc_set_relation_impl.hpp).

Multisets
---------

The basic structure: each term corresponds to *two* multisets. (One could refer to these as an input multiset and an output multiset, but this directionality is purely psychological and we avoid it.) One table contributes to one of the multisets, another table constributes to the other multiset, and the term is *satisfied* if the two multisets are equal.

### First term: <tt>(pc, round, wnaf\_slice)</tt>

This facilitates communication between the Precomputed table and the MSM table. Recall that `pc` stands for point-counter. `round` refers to the wNAF digit-place being processed and `wnaf_slice` is the *compressed* digit (i.e., it is a way of representing the actual wNAF digit). Recall that the skew digit's place corresponds to `round == 32`. This multiset check ensures that at every `round`, for a given point, the wNAF digit computed by the Precomputed table is actually being used by the MSM table.

### Second term: <tt>(pc, P.x, P.y, scalar-multiplier)</tt>

This facilitates communication between the Precomputed table (and more specifically, the PointTable) and the Transcript table. More precisely, it ensures that the Precomputed table has done the wNAF decomposition correctly for the scalar corresponding to the point at position `pc`.

### Third term: <tt>(pc, P.x, P.y, msm-size)</tt>

This facilitates communication between the MSM table and the Transcript table. More precisely, this links the *output* of the MSM (that is performed by the MSM table) to what is written in the Transcript table. We also ensure that `msm-size` is correctly inputed into the MSM table.

Lookups
-------

Unlike the multisets (a.k.a. "strict lookup arguments"), the lookups here are more conventional. For every non-trivial point $P$, there is a lookup table (computed by the Precomputed table) that contains `(pc, compressed_slice, (2 * (compressed_slice) - 15)[P])`, where `compressed_slice` is in the range {0, ..., 15}. The MSM table will look up the relevant value as it goes through the Straus algorithm. For full details, please see [lookup relation](../relations/ecc_vm/ecc_lookup_relation.hpp).

Zero-Knowledge
==============

The ECCVM is **always zero-knowledge** — `HasZK = true` in `eccvm_flavor.hpp`, and there is no non-ZK ECCVM flavor. It hides the witness with the same pipeline as Ultra/Mega Honk, so the canonical description lives in the [Ultra Honk Zero-Knowledge section](../ultra_honk/README.md#zero-knowledge); the ECCVM-specific pieces are:

* **ZK sumcheck.** The first `NUM_DISABLED_ROWS_IN_SUMCHECK` (= 4) rows are disabled in Sumcheck (`TRACE_OFFSET`), and the round univariates are masked by a Libra polynomial whose evaluation is proven via [SmallSubgroupIPA](/api/md_src_2barretenberg_2commitment__schemes_2small__subgroup__ipa_2_r_e_a_d_m_e) (the `Libra …` entries in the proof). Witness polynomials are randomized with `Polynomial::add_masking()` during proving-key construction. See also [Sumcheck.md](/api/md_src_2barretenberg_2sumcheck_2_sumcheck).
* **ZK PCS.** The ECCVM PCS opening is a **TripleIPA** reduction: the sumcheck's unshifted, shifted, and `pow` (univariate) openings are rho-batched into a single Grumpkin IPA claim (see [TripleIPA](/api/md_src_2barretenberg_2commitment__schemes_2triple__ipa_2_p_r_o_t_o_c_o_l)). Hiding uses two masks. `gemini_masking_poly` (the lone `MaskingEntities` member) is a dense random polynomial carried in the unshifted batch; it blinds the IPA transcript and the two cross-sums that contract against it. A small `pow_mask` univariate (sent as `TripleIPA:pow_mask_commitment`) blinds the remaining cross-sum, which has no unshifted term.

These hide the ECCVM **trace**. Hiding of the **op-queue contents** themselves — the random ECC ops that mask the accumulated table the ECCVM proves over — is contributed upstream by the batch-merge ZK prefix and the hiding kernel, not by the ECCVM trace: see [BATCH\_MERGE\_PROTOCOL.md](../goblin/BATCH_MERGE_PROTOCOL.md#adding-zk) and [MERGE\_PROTOCOL.md](../goblin/MERGE_PROTOCOL.md#zk-considerations).

Relations and Subrelations
==========================

The constraints sketched above are organized into relations under [`relations/ecc_vm/`](../relations/ecc_vm/). Each relation exposes a named `SubrelationIndex` enum whose order matches its `SUBRELATION_PARTIAL_LENGTHS`; the names below are those enum values. (The flavor also instantiates "short" variants of these relations for the short-monomial Sumcheck optimization; some split their `msm_transition`-gated subrelations into a separate relation, e.g. `ECCVMTranscriptMSMTransitionShortRelation`.)

| Relation | File | Constrains |
| --- | --- | --- |
| `ECCVMTranscriptRelation` | `ecc_transcript_relation.hpp` | Transcript columns |
| `ECCVMPointTableRelation` | `ecc_point_table_relation.hpp` | Precomputed point table (EC doubling/addition) |
| `ECCVMWnafRelation` | `ecc_wnaf_relation.hpp` | Precomputed wNAF decomposition |
| `ECCVMMSMRelation` | `ecc_msm_relation.hpp` | MSM columns (Straus algorithm) |
| `ECCVMSetRelation` | `ecc_set_relation.hpp` | Multiset (permutation) grand product |
| `ECCVMLookupRelation` | `ecc_lookup_relation.hpp` | Log-derivative point-table lookups |
| `ECCVMBoolsRelation` | `ecc_bools_relation.hpp` | Boolean ($x(x-1)=0$) checks on selectors/flags |
| `ECCVMShiftableInitRelation` | `ecc_shiftable_init_relation.hpp` | `lagrange_first · col = 0` boundary pins |

<tt>ECCVMTranscriptRelation</tt> (Transcript columns)
-----------------------------------------------------

| Subrelation | Meaning |
| --- | --- |
| `Z1_ZERO_CHECK`, `Z2_ZERO_CHECK` | if the `z1zero`/`z2zero` flag is set, the scalar is 0 |
| `OPCODE_WELL_FORMED` | `op = q_reset + 2·q_eq + 4·q_mul + 8·q_add` |
| `PC_UPDATE` | `pc` decrements by the number of muls |
| `MSM_COUNT_ZERO_AT_TRANSITION` | witnesses `msm_count_zero_at_transition` |
| `MSM_TRANSITION` | `msm_transition = q_mul·(1−q_mul_shift)·(1−msm_count_zero_at_transition)` |
| `MSM_COUNT_ZERO_WHEN_NOT_MUL` | `msm_count` is 0 on non-`mul` rows |
| `MSM_COUNT_INCREMENT_ACROSS_ROWS` | `msm_count` increments correctly across `mul` rows |
| `OPCODE_EXCLUSION` | `q_mul`/`q_add` are mutually exclusive with the other opcodes |
| `EQ_X_DIFF`, `EQ_Y_DIFF` | `eq` opcode x/y coordinate comparisons |
| `BOUNDARY_ACCUMULATOR_EMPTY` | `is_accumulator_empty = 1` at the third row |
| `BOUNDARY_MSM_COUNT_AND_PC` | `msm_count = 0` at third row, `pc = 0` at last row |
| `ON_CURVE_CHECK` | input points are on the curve |
| `LAMBDA_RELATION` | slope relation for add/msm group operations |
| `ACCUMULATOR_X_UPDATE`, `ACCUMULATOR_Y_UPDATE`, `ACCUMULATOR_EMPTY_UPDATE` | accumulator x / y / empty-flag updates |
| `ADD_X_EQUAL_CHECK`, `ADD_Y_EQUAL_CHECK` | validate the `add_x_equal` / `add_y_equal` flags |
| `HIDING_ROW_EQ`, `HIDING_ROW_RESET` | hiding-op row: `q_eq = 1` and `q_reset = 1` |
| `INFINITY_BASE_PX`, `INFINITY_BASE_PY` | `Px`/`Py = 0` when the base point is infinity |
| `INFINITY_ACC_X`, `INFINITY_ACC_Y` | `acc_x`/`acc_y = 0` when the accumulator is empty |
| `OFFSET_GENERATOR_X`, `OFFSET_GENERATOR_Y` | subtract the MSM offset generator (x / y) |
| `MSM_INFINITY_X_DIFF`, `MSM_INFINITY_Y_SUM`, `MSM_INFINITY_INVERSE` | MSM-output infinity checks |

The last five subrelations (`OFFSET_GENERATOR_*`, `MSM_INFINITY_*`) are gated entirely by `msm_transition` and are grouped contiguously at the end so the short-monomial flavor can split them into a separately-skippable relation.

<tt>ECCVMPointTableRelation</tt> (Precomputed point table)
----------------------------------------------------------

| Subrelation | Meaning |
| --- | --- |
| `DOUBLE_X`, `DOUBLE_Y` | `D = 2·T` when `point_transition = 1` |
| `D_PROPAGATE_X`, `D_PROPAGATE_Y` | `D_shift = D` when not at a transition |
| `ADD_X`, `ADD_Y` | `T = T_shift + D` when not at a transition |

<tt>ECCVMWnafRelation</tt> (Precomputed wNAF decomposition)
-----------------------------------------------------------

| Subrelation | Meaning |
| --- | --- |
| `RANGE_S1HI` … `RANGE_S4LO` | each 2-bit slice is in {0,1,2,3} |
| `SCALAR_SUM_CHECK` | scalar-sum accumulation across rows |
| `ROUND_CHECK` | round increment / `round == 7` check |
| `ROUND_SHIFT_ZERO`, `SCALAR_SUM_SHIFT_ZERO` | round / scalar-sum shift is 0 at a transition or first row |
| `PC_CHECK` | `pc` decrement / propagation |
| `SKEW_RANGE` | skew is 0 or 7 |
| `INACTIVE_SLICE_W0` … `W3`, `INACTIVE_ROUND`, `INACTIVE_PC`, `INACTIVE_POINT_TRANSITION` | when `precompute_select = 0`, force slices / round / pc / point\_transition to 0 |
| `FIRST_SLICE_POSITIVE` | `s1hi_shift ≥ 2` at transitions (forces a positive top digit) |
| `PRECOMPUTE_SELECT_SHAPE` | `precompute_select` is monotonically non-decreasing after the first row |

<tt>ECCVMMSMRelation</tt> (MSM columns, Straus algorithm)
---------------------------------------------------------

| Subrelation | Meaning |
| --- | --- |
| `ADD_ACC_X/Y`, `ADD_SLOPE_1..4` | addition round: accumulator update + 4 slope constraints |
| `DOUBLE_ACC_X/Y`, `DOUBLE_SLOPE_1..4` | doubling round: accumulator update + 4 slope constraints |
| `SKEW_ACC_X/Y`, `SKEW_SLOPE_1..4` | skew round: accumulator update + 4 slope constraints |
| `COLLISION_CHECK_1..4` | x-coordinate non-equality for the 4 potential additions |
| `INACTIVE_SLICE_1..4` | force `slice_i = 0` when `add_i = 0` |
| `PHASE_SELECTOR_MUTUAL_EXCLUSIVITY` | at most one of `q_add`, `q_double`, `q_skew` active |
| `ROUND_TRANSITION_FORCES_DELTA_ONE` | a round transition forces `round_delta = 1` |
| `ROUND_TRANSITION_SKEW_IMPLIES_ROUND_31` | round transition + skew ⇒ `round = 31` |
| `ROUND_TRANSITION_EXACTLY_ONE_DOUBLE_OR_SKEW`, `ROUND_TRANSITION_NEEDS_DOUBLE_OR_SKEW` | next row has exactly one of double/skew |
| `DOUBLE_IMPLIES_NEXT_IS_ADD` | a double row is followed by an add row |
| `COUNT_SHIFT_ZERO_ON_ROUND_CHANGE`, `COUNT_INCREMENT_WITHIN_ROUND`, `COUNT_ZERO_AT_ROUND_BOUNDARY_OR_TRANSITION` | `msm_count` evolution |
| `MSM_TRANSITION_ROUND_ZERO` | a transition implies `round = 0` |
| `MSM_TRANSITION_PC` | `pc = pc_shift + msm_size` at a transition |
| `ADD_CONTINUITY_2..4`, `ADD_CROSS_ROW_CONTINUITY`, `ADD1_DECOMPOSITION` | add-flag continuity; `add1 = q_add + q_skew` |
| `SKEW_PERSISTS_UNTIL_MSM_TRANSITION`, `SKEW_IMPLIES_ROUND_32` | skew persists to the transition; `q_skew ⇒ round = 32` |
| `DOUBLE_REQUIRES_ROUND_CHANGE`, `DOUBLE_SHIFT_FORBIDS_ROUND_31` | doubling forces a round change; a double-shift row cannot be round 31 |
| `IDLE_ROW_PRESERVES_ACC_X/Y` | accumulator preserved when no phase selector is active |
| `MSM_TRANSITION_AT_ACTIVE_START` | `msm_transition = 1` at the first row of every MSM block |
| `MSM_PC_CONTINUITY`, `MSM_PC_SKEW_CONTINUITY` | `msm_pc` constant within an MSM segment / across consecutive skew rows (soundness: prevents segment swaps) |

<tt>ECCVMSetRelation</tt> (multiset / permutation grand product)
----------------------------------------------------------------

| Subrelation | Meaning |
| --- | --- |
| `GRAND_PRODUCT` | the multiset grand product `z_perm` is constructed correctly |
| `LEFT_SHIFTABLE` | the grand-product polynomial is left-shiftable (boundary) |

These enforce the multiset equalities described in Multisets. The `z_perm = 0` boundary pin now lives in `ECCVMShiftableInitRelation::Z_PERM_INIT`.

<tt>ECCVMLookupRelation</tt> (log-derivative point-table lookups)
-----------------------------------------------------------------

| Subrelation | Meaning |
| --- | --- |
| `GRAND_PRODUCT` | the log-derivative inverse accumulation is correct |
| `LEFT_SHIFTABLE` | boundary of the accumulator |

These enforce the lookups described in Lookups.

<tt>ECCVMBoolsRelation</tt> (boolean checks)
--------------------------------------------

Each subrelation enforces `col·(col − 1) = 0` for one selector or flag: `BOOL_Q_EQ`, `BOOL_Q_ADD`, `BOOL_Q_MUL`, `BOOL_Q_RESET_ACCUMULATOR`, `BOOL_MSM_TRANSITION`, `BOOL_ACCUMULATOR_NOT_EMPTY`, `BOOL_Z1_ZERO`, `BOOL_Z2_ZERO`, `BOOL_ADD_X_EQUAL`, `BOOL_ADD_Y_EQUAL`, `BOOL_BASE_INFINITY`, `BOOL_MSM_INFINITY`, `BOOL_MSM_COUNT_ZERO_AT_TRANSITION`, `BOOL_MSM_TRANSITION_MSM`, `BOOL_PRECOMPUTE_POINT_TRANSITION`, `BOOL_MSM_ADD`, `BOOL_MSM_DOUBLE`, `BOOL_MSM_SKEW`, `BOOL_PRECOMPUTE_SELECT`, `BOOL_MSM_ADD1`, `BOOL_MSM_ADD2`, `BOOL_MSM_ADD3`, `BOOL_MSM_ADD4`.

<tt>ECCVMShiftableInitRelation</tt> (boundary pins)
---------------------------------------------------

Centralizes every direct `lagrange_first · col = 0` pin (degree 2 in the witnesses). The soundness-critical ones are `Z_PERM_INIT` (moved here from `ECCVMSetRelation`) and `TRANSCRIPT_ACCUMULATOR_NOT_EMPTY_INIT` (moved here from `ECCVMTranscriptRelation`), followed by `PRECOMPUTE_SELECT_INIT`, `TRANSCRIPT_MUL_INIT`, `TRANSCRIPT_PC_INIT`. The other shiftable-column pins are defense-in-depth pins on the shiftable precompute / MSM / transcript columns: `PRECOMPUTE_SCALAR_SUM_INIT`, `PRECOMPUTE_DX_INIT`, `PRECOMPUTE_DY_INIT`, `PRECOMPUTE_TX_INIT`, `PRECOMPUTE_TY_INIT`, `MSM_TRANSITION_INIT`, `MSM_ADD_INIT`, `MSM_DOUBLE_INIT`, `MSM_SKEW_INIT`, `MSM_ACCUMULATOR_X_INIT`, `MSM_ACCUMULATOR_Y_INIT`, `MSM_COUNT_INIT`, `MSM_ROUND_INIT`, `MSM_ADD1_INIT`, `MSM_PC_INIT`, `TRANSCRIPT_MSM_COUNT_INIT`.

The non-shiftable translation columns are pinned by `TRANSCRIPT_OP_INIT`, `TRANSCRIPT_PX_INIT`, `TRANSCRIPT_PY_INIT`, `TRANSCRIPT_Z1_INIT`, and `TRANSCRIPT_Z2_INIT`. These make the zero translation row at `TRACE_OFFSET` a consequence of ECCVM relations, without relying on PCS enforcement. Translation removes the masking head and divides by `x^(TRACE_OFFSET + 1)`, so this intervening coefficient must be zero.

* Generated by [![doxygen](doxygen.svg)](https://www.doxygen.org/) 1.9.8