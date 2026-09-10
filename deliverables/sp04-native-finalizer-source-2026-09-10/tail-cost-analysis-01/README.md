# Finalizer tail: field costs and the point-validation boundary

This source analysis adds **6,790 conditional region-height units** for specified field-boundary operations and fixed-value slots. It does not complete the verifier cost bound. The original proposal, schedule supplement and independent audit remain unchanged. This new calculation is authored research awaiting independent review.

Only the already cached, checksum-pinned circuits7.2.4 sources were inspected. [Source evidence](source-evidence.json) records archive/member digests and relevant numbered excerpts. [count-tail.py](count-tail.py) verifies those bytes and the original manifests, reuses the reviewed pure-Python auxiliary-bound calculation, and emits [results](results.json). No Rust, compiler, prover, wallet, service or network was invoked.

## Quantified field tail

These bounds use the same hypothetical native5/foreign14 columns and four parallel eight-bit range-check layout as the arithmetic supplement. Inputs must already have constrained well-formed limbs; selection bits must already be constrained. Copy-only regions are charged one row conservatively. Fixed requests receive no cache discount.

| Operation | Source and derivation | Conditional height sum |
| --- | --- | ---: |
| Mask two Fp12 Miller outputs for identity inputs | 24 Fp selections ×7 native one-row selects; `field_chip.rs:721–752`, `native_chip.rs:329–358,1480–1498` | 168 |
| Assign the Fp12 inverse witness | 12 Fp assignments ×7 limb ranges ×2 rows; `field_chip.rs:424–452` | 168 |
| Inverse-product and final identity equalities | 24 fixed Fp equalities ×7 limbs ×(fixed request + copy region); `field_chip.rs:592–617`, `native_chip.rs:751–794` | 336 |
| Two prepared-line tables | 2×68×6 Fp slots ×7 fixed-limb requests | 5,712 |
| Four Frobenius applications | 4×(5 Fp2 +4 Fp) coefficient slots ×7 fixed-limb requests | 392 |
| Reusable field zero and one for masks | 2×7 fixed-limb requests | 14 |
| **Specified field and constant-slot subtotal** | **672 +6,118** | **6,790** |

The inverse-product multiplication is already included in the prior 9,112,880 arithmetic subtotal and is not counted again. Equality here uses the proposed normalized circuit values; it does not establish unique external byte encoding. The line and Frobenius rows are **slot counts**. Exact inner-SRS values, nondegeneracy, subgroup checks for those constants, line-table derivation and Montgomery-to-canonical conversion remain unfrozen. A constant assignment can constrain the wrong constant just as effectively as the right one.

## On-curve cost is not subgroup membership

The exact on-curve gate first computes `x*x`, then enforces `y² = x³+4` conditionally with a two-row auxiliary identity (`ecc/foreign/gates/weierstrass/on_curve.rs:65–175,304–489`). The port gives auxiliary widths **116,120,117**. Under the same layout, its region subtotal is `45 +2 +3*(4+1) =62` for already assigned, normalized coordinates.

`assign_without_subgroup_check` adds two Fp coordinate assignments, a Boolean identity bit and its negation: `28 +1 +2 +62 =93` per point (`weierstrass_chip.rs:945–962,1090–1120`). This is a bound on that source path, **not a canonical point validator**. When its identity condition is true, the on-curve gate asserts nothing. The witness generator writes `(0,0,true)` for identity, but those host assignments do not by themselves constrain the external encoding or force identity coordinates to zero.

The prepared accumulator does not establish a blanket subgroup invariant. `AssignedMsm::assign` calls `S::assign_without_subgroup_check` for its variable bases (`verifier/msm.rs:346–385`). The corresponding `SelfEmulation` comment states that omission is harmless in PLONK, followed by an explicit **TODO for formal subgroup soundness analysis** (`verifier/types.rs:122–129`). That comment is not evidence of strict native acceptance correspondence.

There is also a concrete precondition to resolve before treating the subgroup-capable assignment as a validated replacement:

1. `weierstrass_chip.rs:339–360` witnesses an on-curve cofactor root Q, then computes `[h]Q`.
2. The BLS G1 cofactor is `h=0x396c8c005555e1568c00aaab0000aaab` (`ecc/curves.rs:277–317`). The public integer check confirms `h*r=p-z`, with negative BLS parameter z.
3. Its constant multiplication enters `mul_by_u128`: **125 doublings and 47 incomplete additions**, plus the surrounding identity substitutions (`weierstrass_chip.rs:880–905,1652–1709`).
4. That helper's incomplete-addition argument assumes every nonidentity input has order r. The cofactor-root entry only establishes on-curve membership. That prime-order premise has not been established for a malicious root.

This is an unresolved soundness/precondition question, not a demonstrated exploit. Honest root generation inside the prime-order subgroup does not settle it. The script counts the literal loop but deliberately gives **no sound subgroup-check row bound**.

## Which points need which evidence

| Point origin | Required boundary |
| --- | --- |
| Fixed inner SRS G2 points and fixed verification-key G1 bases | Validate and freeze exact canonical constants and the correct inner SRS/statement binding. They need not become new unconstrained point witnesses. |
| Variable proof or carried-accumulator bases | Establish the chosen native byte/point/subgroup acceptance relation. The existing assignment path above is insufficient evidence that this was already done. |
| Final G1 values computed by constrained MSMs | Subgroup membership may follow from validated inputs and sound group operations. That invariant and their equality to the required accumulated statement must be demonstrated before dropping duplicate checks. |
| New independently assigned final G1 witnesses | Require membership, identity/encoding constraints and equality to the required constrained MSM outputs. Checking a detached point pair permits statement substitution. |

Checking only two final aggregates is not automatically equivalent to checking every point that a strict native parser accepts. Cancellation of torsion or unbound encodings must not silently widen the relation. No number of dynamic bases or scalar lengths is frozen here, so multiplying the93-row entry cost by an invented count would be misleading.

## Exact remaining engineering boundary

Specify the point representation passed from native preparation into the finalizer, including canonical bytes if required, identity semantics and the origin of every variable base. Resolve the subgroup/cofactor-helper precondition above or select a separately justified complete membership construction. Then freeze the actual accumulator/MSM shape and count its operations against the same column configuration.

Lookup-table loading, full column configuration, floor-planner constant placement, permutation/blinding/domain overhead, native transcript constraints and outer statement binding still lack a complete bound. The single-pass planner can allocate constant rows after a region; summing arithmetic regions alone does not account for arbitrary constant requests from the rest of a verifier. Neither this tail nor its sum with the prior arithmetic subtotal selects a k, SRS, backend, deployment or resource allocation. These are eligible static F0 questions before F0a authorship; an already implemented pairing is not a prerequisite to resolving them.

Reproduce the public calculation from this directory with `python3 count-tail.py`. It returned exit0. The completed prior audit applies only to its original frozen material; this new analysis still requires independent review.
