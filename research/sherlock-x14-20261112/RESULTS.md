# X14: general foreign verifier, operation inventory and unit budget

Date: 2026-11-12. Loop item X-EXP of `research/sherlock-e-path-20261112/LOOP.md`. Branch `x-exp` (from `final-design-2` at `efb0eb78a`). Experiment definition: `research/sherlock-final-design-20261111/ARCHITECTURE.md` section 5.4, row X14: "a costed operation inventory for one PLONK/halo2 verifier (parse, transcript, expression evaluation, MSM steps, final check with the core SRS and with another SRS), and the largest unit that fits the node beside one child verification". It decides the unit budget and schedule of FV-SCHED, the cost of AD-22 (a) (owner decision 2026-11-12: the final check in circuit), and whether the general path fits the node.

Labels: MEASURED is read from `out/` (shared machine, no `--exclusive`, so the times are not timings). DERIVED is computed from MEASURED numbers by a stated rule. ESTIMATE is a count from the verifier code with stated assumptions. READ is a code reading.

## Result

**Gate result: under the written node bounds, no BLS12-381 operation AIR of the stock OpenVM mod-builder fits.** Every operation AIR needs more than 100 interactions (the per-AIR bound of the internal set). The G1 operations also go over the 2,000-column bound of the node. Once these two bounds are put aside, every operation AIR proves and verifies at 2^16 rows inside a unit node beside one child verification. A unit then holds tens of thousands of operations. To close the gate, a choice that no ruling covers is needed (options below).

### Operation AIRs (MEASURED, `out/x14-ops.jsonl`, `out/x14-run.jsonl`)

One operation per row, OpenVM `FieldExpr` (mod-builder), 8-bit limbs, the stock `VariableRangeCheckerAir`. Each AIR proves and verifies alone at 2^4 rows.

| Operation | Limbs | Width | Constraints | Interactions | Degree | Within 100 interactions | Width within the 463 left in the unit node |
|---|---:|---:|---:|---:|---:|---|---|
| `fr_mul` (BLS12-381 scalar field) | 32 | 194 | 66 | 129 | 2 | no | yes |
| `fp_mul` (BLS12-381 base field) | 48 | 290 | 98 | 193 | 2 | no | yes |
| `g1_add` (affine, x1 != x2) | 48 | 772 | 292 | 579 | 2 | no | no |
| `g1_double` (affine, y != 0) | 48 | 869 | 389 | 772 | 2 | no | no |

With the 16-bit RANGE table of LD-S10x-3 (owner confirmed 2026-11-12) in place of the mod-builder default of 17 bits: `fr_mul` and `fp_mul` do not change, `g1_add` goes to 1,063 columns and 870 interactions, `g1_double` to 1,160 columns and 1,063 interactions (MEASURED). Larger limbs do not reduce the count. With 12-bit and 16-bit limbs, trace generation refuses: the carries go over the range checker (`range exceeded: 393215 >= 262144`, MEASURED). The interactions grow with the limb count, about 4 per limb (DERIVED: 129 = 4 x 32 + 1, 193 = 4 x 48 + 1).

### The unit node (MEASURED, `out/x14-run.jsonl`)

The unit node is arity 1 at the internal set. Its child is one internal-set node proof, which stands in for an `FV` child: a node over a W1 leaf of 2^16 rows. It adds one operation AIR and its range checker.

| Unit node | AIRs | Sum of widths | Max interactions per AIR | Tallest AIR | Stacked width (op AIR at 2^12 / 2^14 / 2^16 / 2^17 / 2^18) | Proved and verified at 2^16 |
|---|---:|---:|---:|---|---|---|
| base (child verification only) | 39 | 1,537 | 77 | 2^16 | 34 | (child node: yes) |
| + `fr_mul` | 41 | 1,735 | 129 | range checker 2^18 | 37 / 42 / 60 / 84 / 133 | yes |
| + `fp_mul` | 41 | 1,831 | 193 | range checker 2^18 | 38 / 45 / 72 / 108 / 181 | yes |
| + `g1_add` | 41 | 2,313 | 579 | range checker 2^18 | 42 / 60 / 132 / 229 / 422 | yes |
| + `g1_double` | 41 | 2,410 | 772 | range checker 2^18 | 43 / 63 / 144 / 253 / 470 | yes |

Keygen and the prover accept AIRs above 100 interactions and above 2,000 total columns. The written bounds of the internal set (ARCHITECTURE line 2265: "keygen refuses a node outside them") are design bounds, and this backend does not enforce them (READ from the run). The stacked layout fits at every height to 2^18 (MEASURED). Only 2^16 was proved.

### Operation inventory of one halo2 verifier (ESTIMATE)

The foreign proof is the registered `ivc_node` circuit of `mori-recursion` (midnight-proofs 0.8.3, KZG over BLS12-381), costed by `crates/mori-recursion/tests/x14_inventory.rs`. MEASURED `CircuitModel`: k 20, 652,433 rows, advice 15, fixed 45, lookups 5, permutations 18, column queries 69, point sets 5, proof 6,320 bytes. The stand-in leaf is k 7 and 2,800 bytes (`out/x14-inv.log`). The verifier steps are READ from `midnight-proofs` `plonk/verifier.rs` and `poly/kzg/mod.rs`.

| Phase | Count (ESTIMATE, `ivc_node`) | Operation rows | Units at 2^16 rows per operation AIR (DERIVED) |
|---|---|---|---:|
| parse | at most 131 G1 points (6,320 / 48) and 197 scalars (6,320 / 32); upper bounds for each kind alone | on-curve check: 1 `fp_mul`-class row set per point; scalar range: limb checks | under 1 |
| transcript | about 10 challenge squeezes (theta, beta, gamma, trash, y, x, x1 to x4) over all proof elements | not costed: no AIR exists for the midnight transcript hash | PENDING |
| expression evaluation | 69 column queries, gate, permutation and lookup argument terms, vanishing `x^n` (20 squarings at k 20), Lagrange terms | `fr_mul` rows; the gate polynomial count is not in `CircuitModel` | PENDING (likely under 1) |
| MSM (multi-open, KZG) | at most about 196 bases (131 + 45 fixed + 18 permutation + 2) with 255-bit scalars; one shared-doubling accumulator: 255 doublings and about 196 x 128 = 25,088 additions | 255 `g1_double` + about 25,100 `g1_add` | 1 (2^15 rows of `g1_add`) |
| final check, AD-22 (a) | two Miller loops (shared squarings) and one final exponentiation: about 19,000 to 20,000 `fp_mul` equivalents (Fp2 = 3, Fp6 = 18, Fp12 multiply 54 and square 36 `fp_mul`; 63 doublings, 5 additions; final exponentiation about 7,700) | about 20,000 `fp_mul` rows | 1 (2^15 rows) |
| final check with another SRS | the same pairing count; only the G2 constants change | about 20,000 `fp_mul` rows | 1 |

DERIVED unit budget: once the interaction and column bounds are set aside, one unit holds one operation AIR of 2^16 rows, proved, so 65,536 operations of one kind. The layout fits to 2^18 (262,144 operations, not proved). Then the MSM and the in-circuit final check of one `ivc_node` proof need about 2 to 3 units, before the transcript and expression evaluation units. With incomplete affine formulas, the MSM needs exceptional-case handling (x1 = x2, the point at infinity); that is not costed here.

## What has to be chosen (no ruling covers it; options with a recommendation)

The 100-interaction bound per AIR and the 2,000-column bound are the internal-set shape bounds (ARCHITECTURE section 1.9, DG-004). No stock BLS12-381 operation AIR meets them.

1. **Re-encode the operation AIRs to fit the bounds (recommended).** Check two 8-bit limbs per bus message with a tuple range checker of 2 x 8 bits (OpenVM `RangeTupleChecker`, a 2^16-row table, same crate, no new dependency). Build each G1 operation as a short sequence of `fp_mul`-class rows chained over a bus, not as one wide row. DERIVED: about (129 - 1) / 2 + 1 = 65 interactions for `fr_mul` and (193 - 1) / 2 + 1 = 97 for `fp_mul`, with widths under 300. G1 addition is then about 3 `fp_mul`-class rows and doubling about 4, so the MSM becomes about 80,000 rows of one AIR (2 units at 2^16). This is new AIR code, not a stock chip, and it is unmeasured. It keeps the node key set and the bounds unchanged.
2. **Raise the per-AIR interaction bound of the internal set** (for example to 200, which fits stock `fr_mul` and `fp_mul`, with G1 operations decomposed as in option 1), and keep 2,000 columns. Cost: every node key changes before FP-8 freezes the key set. The cost to the next node of verifying a unit node whose AIRs have 193 interactions is unmeasured (this run measured the unit node itself, not its parent).
3. **Split each operation across two AIRs** (half the limbs each, joined by a bus). This halves the interactions per AIR and uses 2 of the 11 free AIR slots per operation kind (X7 residual). It costs more AIRs than option 1 and fewer code changes than option 1's G1 decomposition.

Recommendation: option 1. The node bounds stay as written, so GP-3 is not narrowed. The next measurement is the tuple-checked `fp_mul` AIR in the unit node.

## Gate status

- **GP-3 general verifier (FV-SCHED unit budget):** NOT closed. Unit budget MEASURED with the interaction and column bounds set aside (one operation AIR at 2^16 proved, 2^18 laid out); within the written bounds, 0 stock operation AIRs fit. FREEZE-PENDING on the choice above.
- **AD-22 (a) cost:** about 20,000 `fp_mul` rows per final check, so 1 unit per foreign SRS (ESTIMATE). The owner's ruling stands; the encoding is open.
- **Transcript and expression evaluation:** PENDING (no AIR for the midnight transcript hash; no gate count).

## Method

- `crates/mori-recursion/tests/x14_inventory.rs` (with a `[[test]]` entry in `crates/mori-recursion/Cargo.toml`): the `CircuitModel` of the registered stand-in leaf and `ivc_node` circuits (`cargo test --release -p mori-recursion --features sherlock-recursion --locked --test x14_inventory -- --ignored --nocapture`, rc 0).
- `src/x_exp_node.rs` (the bench of X10, X7 and X14; this copy adds the `ops` mode and the `X14_LIMB_BITS` and `X14_RANGE_BITS` knobs): mode `x14 16 fr_mul,fp_mul,g1_add,g1_double 12,14,16,17,18 16` (rc 0, 513 s) and mode `ops` through `out/x14-ops.sh` (limb and range sweep, rc 0). All under `heavy-slot`.
- Valid inputs: random field elements for the multiplications, and multiples of the BLS12-381 G1 generator (repeated doubling) for the G1 operations.

## Files

- `out/x14-inv.log`: the two `CircuitModel` lines.
- `out/x14-run.jsonl`, `out/x14-run.err`: operation AIR shapes, child node, unit node base, unit nodes with each operation at five heights.
- `out/x14-ops.jsonl`, `out/x14-ops.sh`: the limb and range sweep.
- `src/x_exp_node.rs`: bench source.
