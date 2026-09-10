# Literal-order subgroup relation: source-analysis draft

**A complete field-level schedule can specify `[r]P = O` without first assuming P has prime order.** The construction below has a conditional region-height bound of **368,188 per input point** under the previously stated layout. This is a mathematical specification and public Python model, not circuit authorship, an implemented Midnight API, an accepted design or an F0 go decision. Complete verifier/domain cost remains unknown.

The existing ECC API is not taken as a proved all-curve group law. Its module requires no low-order points (`weierstrass_chip.rs:14–19`); `assert_double` additionally invokes a no-order3 premise (`1163–1220`). The full BLS curve contains `(0,2)` of order3. The slope helper's equal-x fallback might allow some such cases, but that possibility is not a proof of the whole API's behavior. This draft instead specifies the required relation using the already pinned field primitives.

## Input relation and complete addition

Represent a point by `(x,y,b)`, with constrained Fp coordinates and Boolean b. Require `b => x=y=0` and `not b => y²=x³+4`. Thus O is `(0,0,true)`. External compressed-byte canonicality and its equality to these fields remain separate. Normalization follows every field add/subtract, so subsequent operands have well-formed limbs.

For valid P and Q, define these constrained quantities. `Z(a)` is the exact field-zero gadget, and `S(c,a,d)` selects a if c is true, otherwise d. All arithmetic below is in Fp.

```text
dx = Q.x - P.x; dy = Q.y - P.y; yy = P.y + P.y
xx = P.x * P.x; tangentNumerator = (xx + xx) + xx
ex = Z(dx); ey = Z(dy); vy = Z(yy)
active = (not ex) or (ey and not vy)
n = S(ex, tangentNumerator, dy)
d = S(ex, yy, dx)
safeD = S(active, d, 1)
assign Fp witness w; constrain safeD * w = 1
lambda = n * w
tx = lambda*lambda - (P.x + Q.x)
ty = lambda*(P.x - tx) - P.y
T = (S(active,tx,0), S(active,ty,0), not active)
U = S(Q.b, P, T)   // coordinate and Boolean selections
R = S(P.b, Q, U)
```

The denominator is nonzero by construction: when `ex=false`, dx is nonzero; when `ex=true` and active, `yy !=0`; otherwise safeD is1. The inverse witness therefore never needs an unconstrained host exception or an inactive division by zero.

For finite inputs, unequal x takes the ordinary secant formula. Equal x and equal nonzero y takes the tangent formula. Equal x with opposite y, including the zero-y tangent case, returns O. These cases exhaust valid short-Weierstrass points in characteristic other than2 or3. The final two selections handle identity inputs. Output validity and canonical identity follow from the formulas and selections. In particular, doubling `(0,2)` returns `(0,-2)` with no order3 exclusion. This is an algebraic justification; the Python checks below are additional examples, not a proof about implemented constraints.

## Literal integer schedule

Use the exact positive integer r from the pinned Fq modulus, **not a value constructed in Fq**. Converting r to Fq would produce zero and turn `[r]P=O` into a vacuous scalar-zero calculation.

The integer has255 bits and134 one-bits. Start A=P, omit its leading one-bit, then for each remaining bit set `A=Add(A,A)` and, for a one-bit, `A=Add(A,P)`. Finish by asserting A.b is true. This produces exactly **254 doubles +133 adds =387 calls** to the complete relation above, with no GLV split or scalar reduction. Identity is allowed and remains identity.

Under the stated BLS group parameters, `[r]P=O` characterizes the r-torsion subgroup. The group-order relation `h*r=p-z` and the recorded cofactor identify the intended G1 subgroup; no cofactor-root/incomplete-add helper is used. The public order3 example is rejected because `r mod3=1`.

## Source-supported conditional costs

[Source evidence](source-evidence.json) pins five exact circuits7.2.4 archive members and numbered excerpts. Prior arithmetic costs remain assumptions from the unchanged reviewed schedule:45 per normalized-input Fp product,56 per normalized field linear operation,7 per Fp selection,14 per assigned Fp value and14 per equality to a fixed Fp value. The layout is still five native arithmetic columns, fourteen foreign columns and four parallel eight-bit lookup columns; no domain or backend is selected.

`FieldChip::is_zero` checks seven limbs against the unique zero representation and ANDs the resulting bits (`field_chip.rs:694–713`). A native fixed-value equality has two arithmetic rows plus a fixed-zero request/copy check (`native_chip.rs:1332–1378`, `instructions/zero.rs:31–51`). Six native AND steps each cost at most a fixed-one request and a multiplication row. Thus one normalized Fp zero test costs at most `7*4+6*2=40`.

| Complete-add operation | Count | Conditional height |
| --- | ---: | ---: |
| Fp multiplication, including inverse equation | 5 | 225 |
| Normalized Fp add/subtract | 9 | 504 |
| Fp zero tests | 3 | 120 |
| Fp selections | 9 | 63 |
| Inverse witness assignment | 1 | 14 |
| Inverse-product equality to1 | 1 | 14 |
| Boolean NOT×3, AND×1, OR×1, selection×2 | fixed schedule | 11 |
| **One complete add** | | **951** |

Input assignment/on-curve/identity-bit work costs93 under the prior tail calculation. Two additional selections and two fixed equalities constrain `b => x=y=0`, costing42. Assign reusable field0/1 for14; the final Boolean assertion costs at most2. The per-point subtotal is therefore `135 +14 +387*951 +2 =368188`. Intermediate points are built from constrained field operations, not fresh unchecked host point assignments. No cache or inter-region packing savings are credited.

This bound excludes external byte decoding, point-to-transcript/accumulator binding, any extra canonical export constraints, table loading, full column/domain/permutation/blinding costs and the number of dynamic points in a complete verifier. Fixed SRS points still require exact constant validation. The existing prepared-MSM path does not establish that dynamic bases were subgroup-checked, so neither zero checks nor a hard-coded count of two point checks may be assumed without the origin analysis.

## Public checks and next boundary

`python3 check-relation.py` returned exit0. It verifies source/manifest pins and counts the straight-line model on all **3,267 point pairs across eight small prime fields**, including order2, order3 and identity cases, against a separate conventional group-law function. Public integer arithmetic over the actual BLS field also accepts O and a deterministic nonidentity cofactor-projected point, and rejects `(0,2)` under literal r. These are not native fixtures, P3 proof results, circuit synthesis or constraint satisfaction tests.

The next precise engineering step is independent review of this relation and cost mapping, then specification of which exact assigned points need the check and how each is bound to the native transcript/carried accumulator. This provides a finite static candidate before F0a. It does not authorize implementation, proving, a larger k or Preview deployment. Original proposals, audits, charges and hard gates remain untouched.
