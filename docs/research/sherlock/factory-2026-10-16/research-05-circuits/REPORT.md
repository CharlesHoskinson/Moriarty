# Circuit testing and underconstraints

**Source claims**

The Halo2 repository issue reports that Veridise's Picus analysis found `RunningSum` underconstrained for a supported configuration: 255-bit words split into 85 three-bit windows over Pallas, with `strict = true` (issue description, lines 160–163). The reported cause is a representation range wider than the field: values `x` and `p+x` can both have a base-8 decomposition while mapping to the same field element (line 162). The report proposes a regression test exhibiting two decompositions for zero; a Halo2 contributor later says that this non-uniqueness was known and intended, but should be documented more clearly (lines 163, 172–182).

**Provisional Sherlock adaptation (inference)**

For a small bounded circuit QA case, keep the public statement fixed, obtain one honest witness, then separately supply a deliberately altered intermediate witness that preserves the field equations but violates the intended integer/range meaning. Run the constraint checker on both and assert that the honest witness passes and the altered witness fails. This tests whether constraints enforce the intended relation, rather than merely whether the ordinary witness generator emits a valid assignment. For range/decomposition gadgets, include boundary and modulus-wraparound cases, and keep any accepted ambiguity explicit in the spec. Treat this as a targeted regression idea, not a soundness conclusion; Opus owns security review.

**Limitation / counterpoint**

The issue concerns one Halo2 gadget configuration and records that its non-unique decomposition is intentional. This example does not establish a general testing method or show that every multiple-witness relation is a bug. A mutation test also covers only the mutations chosen; it cannot establish full constraint completeness or soundness.
