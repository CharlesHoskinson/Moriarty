# Fixed-G2 pairing schedule supplement

This supplement advances the missing F0 cost question. It specifies an arithmetic schedule before circuit authorship; **an implemented pairing circuit is not a prerequisite to doing this F0 analysis**. The original proposal remains unchanged. No backend, deployment, k, SRS, resource allowance or go decision is selected here.

The schedule has a checked conservative arithmetic-region height sum of **9,112,880** for one explicit unoptimized construction and hypothetical source-supported layout. This is **not a complete finalizer/verifier row bound**, a measured circuit, a minimum row requirement, or evidence that any particular k fits. It includes no proving-time or memory estimate. The remaining terms are identified below.

## Pinned host schedule

The native lock pins BLST 0.3.17, archive SHA-256 `c20659f9bbee16cbbd2f7393e40ab6309f5a98f76a2eb57a995ec508b72387fe`. The unchanged public sources [pairing.c](pairing.c) and [fp12_tower.c](fp12_tower.c) are copied from that checksum-verified archive, with their original license headers. The native Rust wrapper calls this prepared-line path separately for the two pairing terms, then multiplies their Miller outputs.

For each fixed **nonidentity** G2 input, `pairing.c:272–335` uses one initial line and five groups `(offset,length) = (1,2), (4,3), (8,9), (18,32), (51,16)`. Each group uses an addition line followed by its specified doubling lines. This gives 68 line evaluations, 62 Fp12 squares, 67 sparse Fp12 products, one Fp12 conjugation, two Fp additions and one Fp negation to prepare `(-2x,2y)`. Each line evaluation uses four fixed-constant Fp products (`287–296`). G2 line generation itself is fixed preprocessing, subject to exact SRS binding; it is not variable witness work to be trusted without that binding.

`pairing.c:355–403` gives the following total schedule:

| Operation | Two Miller terms and combination | Final exponentiation |
| --- | ---: | ---: |
| Line evaluations | 136 | 0 |
| Ordinary Fp12 squares | 124 | 0 |
| Sparse Fp12 products | 134 | 0 |
| Dense Fp12 products | 1 | 35 |
| Cyclotomic squares | 0 | 315 |
| Fp12 conjugations | 2 | 10 |
| Frobenius maps | 0 | powers 2, 3, 2, 1 |
| Fp12 inverse | 0 | 1 |

[count-schedule.py](count-schedule.py) checks group offsets and operation counts directly against the saved host function bodies and symbolically transcribes the exponent chain. Its [results](schedule-results.json) passed without executing BLST or any circuit.

**The exponent distinction matters.** The host chain computes exponent `3E`, where `E=(p^12−1)/r`, modulo `p^12−1`. It does not compute the same target-group value as exponent E. For a nonzero element of the stated extension field, both give the same identity test: `gcd(3,r)=1`. This is an algebraic identity-predicate observation, not verification of a native pairing fixture. The use as native proof acceptance still requires valid point/subgroup inputs and correctly bound SRS constants.

## Fully specified conservative arithmetic construction

Use the pinned tower `Fp2 = Fp[u]/(u²+1)`, `Fp6 = Fp2[v]/(v³−u−1)`, `Fp12 = Fp6[w]/(w²−v)`. Each Fp2 product is four Fp products, one add and one subtract. Each Fp6 product is schoolbook convolution: nine Fp2 products, six Fp2 additions and two multiplications by `1+u`. Each Fp12 product is four Fp6 products, two Fp6 additions and one multiplication by v. Multiplication by `1+u` maps `(a,b)` to `(a−b,a+b)`; multiplication by v rotates the Fp6 coefficients and applies that map once.

Thus a dense Fp12 product costs **144 Fp products, 105 additions, 45 subtractions** in this deliberately unoptimized construction. Use this same dense routine for every sparse product and both ordinary and cyclotomic squares. It computes the same field square, avoiding a dependency on an implemented cyclotomic optimization. Copies and coefficient permutations are wiring. Fixed coefficients use the generic product bound rather than claiming cheaper constant arithmetic.

For each Frobenius map use the host coefficient schedule at `fp12_tower.c:667–731`: five Fp2 fixed-coefficient products, four Fp fixed-coefficient products, and six Fp negations for odd powers. Fp12 conjugation negates its last six Fp coefficients. Freeze the correctly decoded constants from the pinned source; native Montgomery arrays are not the proposed canonical limb encoding.

Replace the inverse operation with twelve assigned Fp coefficients and constrain a dense product `f * inverseWitness = 1` through twelve field equalities. This enforces nonzero f; a host inverse or host zero check is not the predicate. Assert the final Fp12 value is identity through another twelve equalities. The inverse witness's field limbs and every comparison remain constrained. No unconstrained success flag enters the relation.

**Normalization policy:** immediately normalize every Fp addition, subtraction and negation before reuse. Multiplication receives well-formed operands and returns well-formed limbs through the existing gadget. Consequently no unbounded lazy-add chain is assumed. Seven 56-bit limbs follow the shifted representation previously checked. Negation is explicitly a subtraction from the fixed zero followed by normalization.

Including the inverse-product check, the arithmetic core has 610 dense Fp12 product instances. Expansion gives **88,480 Fp products, 64,074 additions, 27,470 subtractions and 86 negations**, with at most **91,630 normalization calls**. These are construction counts, not the operation count of BLST's optimized machine code.

## Exceptional inputs and remaining boundary work

The fixed G2 constants must be valid subgroup elements from the exact inner SRS. The source distinguishes a negative generator built during verifier-parameter reading from the negative stored g2 used when converting full parameters. Their equivalence and nonidentity assumptions must be established for the selected SRS. A degenerate SRS cannot be silently accepted as a useful fixture.

Variable G1 identity is a valid pairing case. Constrain its identity bit from the canonical point representation and curve/subgroup boundary. For a nonidentity fixed G2, compute the division-free Miller arithmetic, then select the Fp12 identity when the G1 input is infinity, matching the Rust host convention. This costs twelve Fp selections per term, 24 total, in addition to the identity-bit derivation. Only the product of these selected Miller results reaches the inverse relation. A fixed G2 identity, if a separate specification actually allowed it, must select the known constant-one term instead of reading an absent 68-entry line table; it is excluded from the current nondegenerate SRS premise.

Do not invert a possibly zero raw Miller result before identity masking. If the final masked input is zero, the constrained inverse-product equation fails. Malformed/noncanonical points, wrong subgroup points, false identity bits and a substituted fixed line table must fail their own constraints. Their costs are explicit remaining terms, not assumed host preflight checks. P3's positive fixture must have both evaluated pairing sides nonidentity, so these exceptional branches cannot make the test vacuous.

## Mapping to exact outer Fp gadget and row accounting

[gadget-source-evidence.json](gadget-source-evidence.json) retains exact members and excerpts from checksum-verified circuits 7.2.4 and proofs 0.8.2. All lines below refer to that archive, not a moving working copy.

| Operation | Source mapping and accounting |
| --- | --- |
| Fp multiply | `field/foreign/field_chip.rs:899–925,1414–1475`; seven output limb assignments, then `gates/mul.rs:206–319` two-row identity plus u/v range checks. Normalized input calls do not normalize again. |
| Fp add/subtract | `field_chip.rs:787–896`: seven two-term native linear combinations with shifted-limb corrections, then explicit normalization. |
| Normalize | `field_chip.rs:1566–1628`, `gates/norm.rs:287–345`: two-row identity, seven output range checks and the auxiliary u/v range checks. |
| Native linear combination | `field/native/native_chip.rs:425–510`: at most four terms per row with five arithmetic advice columns; a two-term add/subtract uses one arithmetic row. |
| Range checks | `decomposition/chip.rs:247–298,405–427`: optimized decomposition row groups. Existing-cell assertions also use a copy-only region (`instructions.rs:55–67`), conservatively charged one row here. |
| Fixed values | `native_chip.rs:569–598`: cached fixed assignments. The bound credits no cache savings and charges each relevant fixed request one row. |

The **illustrative** layout uses the existing `configure_from_scratch` values: four parallel range checks, at most eight bits per tag (`field_chip.rs:1827–1852`), five native arithmetic advice columns and fourteen foreign arithmetic advice columns. This is an explicit model parameter, not selection of an outer deployment. The older trait parameter `RC_LIMB_SIZE=15` must not be confused with this actual lookup configuration.

[count-gadget-cost.py](count-gadget-cost.py) ports the integer bounds from `gates/mul.rs:60–118`, `gates/norm.rs:65–128`, and `foreign/util.rs:198–314`, including LCM sufficiency and no-native-wrap assertions. It ports the optimal decomposition recurrence and bound rounding. Its [results](gadget-cost-results.json) give:

| Quantity | Checked value |
| --- | --- |
| Multiplication auxiliary widths | 116, 117, 117 bits |
| Normalization auxiliary widths | 69, 92, 92 bits |
| Range rows for one 56-bit limb | 2 |
| Multiply two-row identity plus output/auxiliary range regions | 31 |
| Normalization two-row identity plus output/auxiliary range regions | 35 |
| Multiply including all 14 fixed-limb requests without caching | at most 45 |
| Add/subtract/negate plus normalization and uncached zero requests | at most 56 |

The arithmetic-region height sum is therefore `88,480*45 + 91,630*56 = 9,112,880`. This credits no parallel packing. The single-pass floor planner places a region at the maximum prior endpoint of its columns, then advances those columns by the region height (`proofs/src/circuit/floor_planner/single_pass.rs:98–106`). Thus the sum is a conservative column-height account for these specified region calls; extra regions remain separately charged. It is not an executed circuit layout or a global domain size.

**Unbounded tail, stated precisely:** canonical G1 decoding/on-curve/subgroup and identity-bit derivation; exact fixed SRS and Frobenius constant assignments; 24 field selections; twelve inverse-witness assignments; 24 field equalities; lookup-table loading, complete column configuration and permutation/blinding/domain overhead; native transcript parsing/preparation, carried-accumulator MSM, and outer statement binding. No finite upper bound for this whole tail is claimed here. Therefore no complete-verifier k or fit follows. An upper bound exceeding a small domain also does not prove that an optimized construction cannot fit that domain.

The next missing source fact is the **complete boundary-and-layout cost**, not the existence of an already implemented pairing. It can be resolved in F0 by specifying the canonical point/identity encoding, then inspecting/counting the exact G1 assignment/subgroup, field selection/equality, constant-loader and floor-planner/domain paths. That source accounting and the now-explicit arithmetic schedule permit a later reviewed estimate or a bounded discriminating probe decision; no circular requirement to implement F0a first is imposed.

## Validation and scope

Run the two pure Python source/arithmetic checks, in order:

```sh
python3 count-schedule.py
python3 count-gadget-cost.py
```

Both returned exit 0. These checks count and evaluate public integer formulas; they do not compile, synthesize, execute circuits, validate a pairing fixture, prove, contact Midnight or read private data. The source transcription and mathematical construction remain subject to independent review. The prior no-go proposal, k17 failure, zero recursive proofs and all accumulated charges are preserved. Mandatory PCD and deployment acceptance remain open.
