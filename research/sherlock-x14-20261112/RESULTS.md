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

With the 16-bit RANGE table of LD-S10x-3 (owner confirmed 2026-11-12) in place of the mod-builder default of 17 bits: `fr_mul` and `fp_mul` do not change, `g1_add` goes to 1,063 columns and 870 interactions, `g1_double` to 1,160 columns and 1,063 interactions (MEASURED).

Limb sweep (MEASURED, `out/x14-ops.jsonl`, `out/x14-ops2.jsonl` and their `.err` logs). The interactions grow with the limb count, about 4 per limb (DERIVED: 129 = 4 x 32 + 1, 193 = 4 x 48 + 1):

| Limb bits, range bits | `fr_mul` width / interactions | `fp_mul` width / interactions | Range table height |
|---|---|---|---|
| 8, 17 (stock) | 194 / 129 | 290 / 193 | 2^18 |
| 8, 16 (LD-S10x-3) | 194 / 129 | 290 / 193 | 2^17 |
| 10, 17 | 158 / 105 | trace generation refused (`range exceeded: 393144 >= 262144`) | 2^18 |
| 10, 18 | 158 / 105 | 236 / 157 | 2^19 (the internal height bound) |
| 12, 17 and 16, 17 | trace generation refused (`range exceeded`) | refused | |
| 12, 18 | refused (`range exceeded: 787248 >= 524288`) | refused | |

So wider limbs do not get any operation under 100 interactions: a range table above 18 bits passes the 2^19 height bound, and at 18 bits the best is 105 (`fr_mul`, 10-bit limbs). A RANGE table wider than 16 bits would also leave LD-S10x-3.

The bounds are the stated assumptions of the 100-bit security analysis of the parameter set, not a keygen check of this backend. stark-sdk `config/mod.rs` lists them under "Assumptions for 100-bit security": at most 1,000 constraints per AIR, 50 AIRs, 100 interactions per AIR, and 2,000 trace columns "unstacked, total across all AIRs" (READ, `openvm-org_stark-backend/crates/stark-sdk/src/config/mod.rs`, the `root_params_with_100_bits_security` comment; PROPOSAL line 200 cites lines 203-208). A unit node above them is outside the proven 100-bit analysis of the internal set (ARCHITECTURE section 1.9), even though it proves and verifies.

### The unit node (MEASURED, `out/x14-run.jsonl`)

The unit node is arity 1 at the internal set. Its child is one internal-set node proof, which stands in for an `FV` child: a node over a W1 leaf of 2^16 rows. It adds one operation AIR and its range checker.

| Unit node | AIRs | Sum of widths | Max interactions per AIR | Tallest AIR | Stacked width (op AIR at 2^12 / 2^14 / 2^16 / 2^17 / 2^18) | Proved and verified at 2^16 |
|---|---:|---:|---:|---|---|---|
| base (child verification only) | 39 | 1,537 | 77 | 2^16 | 34 | (child node: yes) |
| + `fr_mul` | 41 | 1,735 | 129 | range checker 2^18 | 37 / 42 / 60 / 84 / 133 | yes |
| + `fp_mul` | 41 | 1,831 | 193 | range checker 2^18 | 38 / 45 / 72 / 108 / 181 | yes |
| + `g1_add` | 41 | 2,313 | 579 | range checker 2^18 | 42 / 60 / 132 / 229 / 422 | yes |
| + `g1_double` | 41 | 2,410 | 772 | range checker 2^18 | 43 / 63 / 144 / 253 / 470 | yes |

Keygen and the prover accept AIRs above 100 interactions and above 2,000 total columns (MEASURED). The bounds are soundness assumptions (see above), and ARCHITECTURE line 2265 ("keygen refuses a node outside them") is a requirement for the production keygen gate, which this bench does not have. The stacked layout fits at every height to 2^18 (MEASURED). Only 2^16 was proved. Not measured: a parent node that verifies one of these unit nodes.

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
2. **Raise the per-AIR interaction bound of the internal set** (for example to 200, which fits stock `fr_mul` and `fp_mul`, with G1 operations decomposed as in option 1), and keep 2,000 columns. Cost: the bound is an assumption of the 100-bit analysis, so D3 must recompute the internal set's terms (LogUp and the others that depend on interactions) and keep every term at 100 bits or more. Every node key changes before FP-8 freezes the key set. The cost to the next node of verifying a unit node whose AIRs have 193 interactions is unmeasured (this run measured the unit node itself, not its parent).
3. **Split each operation across two AIRs** (half the limbs each, joined by a bus). This halves the interactions per AIR and uses 2 of the 11 free AIR slots per operation kind (X7 residual). It costs more AIRs than option 1 and fewer code changes than option 1's G1 decomposition.

Recommendation: option 1. The node bounds stay as written, so GP-3 is not narrowed. The next measurement is the tuple-checked `fp_mul` AIR in the unit node.

## Gate status

- **GP-3 general verifier (FV-SCHED unit budget):** NOT closed. Unit budget MEASURED with the interaction and column bounds set aside (one operation AIR at 2^16 proved, 2^18 laid out); within the written bounds, 0 stock operation AIRs fit. FREEZE-PENDING on the choice above.
- **AD-22 (a) cost:** about 20,000 `fp_mul` rows per final check, so 1 unit per foreign SRS (ESTIMATE). The owner's ruling stands; the encoding is open.
- **Transcript and expression evaluation:** PENDING (no AIR for the midnight transcript hash; no gate count).

Gemini Quint lane: none due for X14. The operation AIRs are stock OpenVM mod-builder chips used unchanged, and no Sherlock circuit changed. The re-encoded AIRs of option 1 are new circuits, and each one gets a lane when it is built.

## Method

- `crates/mori-recursion/tests/x14_inventory.rs` (with a `[[test]]` entry in `crates/mori-recursion/Cargo.toml`): the `CircuitModel` of the registered stand-in leaf and `ivc_node` circuits (`cargo test --release -p mori-recursion --features sherlock-recursion --locked --test x14_inventory -- --ignored --nocapture`, rc 0).
- `src/x_exp_node.rs` (the bench of X10, X7 and X14; this copy adds the `ops` mode and the `X14_LIMB_BITS` and `X14_RANGE_BITS` knobs): mode `x14 16 fr_mul,fp_mul,g1_add,g1_double 12,14,16,17,18 16` (rc 0, 513 s) and mode `ops` through `out/x14-ops.sh` (limb and range sweep, rc 0). All under `heavy-slot`.
- Valid inputs: random field elements for the multiplications, and multiples of the BLS12-381 G1 generator (repeated doubling) for the G1 operations.

## Files

- `out/x14-inv.log`: the two `CircuitModel` lines.
- `out/x14-run.jsonl`, `out/x14-run.err`: operation AIR shapes, child node, unit node base, unit nodes with each operation at five heights.
- `out/x14-ops.jsonl`, `out/x14-ops.err`, `out/x14-ops.sh`, `out/x14-ops2.jsonl`, `out/x14-ops2.err`, `out/x14-ops2.sh`: the limb and range sweeps.
- `src/x_exp_node.rs`: bench source.

## Follow-up under the owner rulings of 2026-11-12 20:35

Rulings (`research/sherlock-processor-final-20261111/DECISIONS-ASK.md`, "Owner rulings 2026-11-12 20:35"): X14 option 2. The internal set's per-AIR interaction bound is 200, G1 operations become `fp_mul`-class rows, and stock `fr_mul` and `fp_mul` are used unchanged. N-6 option 2: the node budget is 64 AIRs and 3,000 columns, before FP-8. D3 recomputes the internal-set terms. The bounds below are the ruling's; this section records what they give.

### (a) D3 at the new bounds (MEASURED, `out/d3-bounds.log`, `src/x_exp_bounds.rs`)

The D3 calculator call of `ud_sizing.rs` (`SoundnessCalculator::calculate_extended`), with the frozen internal set (OD-19): log blowup 3, `l_skip` 2, `n_stack` 17, `w_stack` 512, queries 97/82/81, folding PoW 0, `mu` PoW 9, query PoW 21, LogUp PoW 18, height 2^19, 1,000 constraints per AIR.

| Term | 50 AIRs, 2,000 columns, 100 interactions | 64 AIRs, 3,000 columns, 200 interactions |
|---|---:|---:|
| logup (grindable) | 102.7205 | 102.7205 |
| gkr_sumcheck | 122.0426 | 122.0426 |
| gkr_batching | 123.6276 | 123.6276 |
| zerocheck | 119.7207 | 119.7207 |
| constraint_batching | 113.6546 | 113.6546 |
| stacked | 111.6618 | **111.0768** |
| mu (grindable) | 101.6304 | 101.6304 |
| fold_rbr (grindable) | 102.6276 | 102.6276 |
| query (grindable) | 101.1643 | 101.1643 |
| shift_rbr | 101.1643 | 101.1643 |
| gamma | 117.0129 | 117.0129 |
| ood | 109.6276 | 109.6276 |
| weakest | 101.1643 | 101.1643 |
| gate (grindable >= 101, every >= 100) | PASS | PASS |

Every term stays at 100 bits or more, and every grindable term at 101 or more. No PoW or parameter changes: the smallest LogUp PoW and `mu` PoW that hold 101 bits are the frozen 18 and 9. Only the stacked term moves, through the column count. `n_logup` stays 29 under both bounds, because the field bound (`max_interaction_count`) is smaller than the parameter bound (`log2 64 + log2 200 + 19 - 2 = 31`), so the per-AIR interaction bound does not reach the LogUp term at this set. This is a calculator figure (G2 part), not the qualified proof (G1 to G4 open, ARCHITECTURE section 1.9).

### (b) The parent node over a unit node with 193-interaction AIRs (MEASURED, `out/x14b-parent.jsonl`)

The unit node is arity 1 over one internal-set node: stock `fp_mul` at 2^16 rows (193 interactions) and its range checker. It was proved and verified (41 AIRs, 1,831 columns, tallest AIR 2^18). The parent is an internal-set node over it, compared with the same parent over the plain child node:

| Parent | Child | AIRs | Columns | Max interactions per AIR | Tallest AIR | Stacked width | Main cells | Lifted interactions | Proved and verified |
|---|---|---:|---:|---:|---|---:|---:|---:|---|
| arity 1 | plain node | 39 | 1,537 | 77 | 2^16 | 34 | 17,296,624 | 1,506,468 | yes |
| arity 1 | unit node (`fp_mul`, 193) | 39 | 1,537 | 77 | 2^16 | 34 | 17,426,928 (+130,304, +0.75 %) | 1,554,852 | yes |
| arity 2 | plain nodes | 39 | 1,549 | 77 | 2^17 | 47 | 24,336,216 | 2,946,964 | yes |
| arity 2 | unit nodes | 39 | 1,549 | 77 | 2^17 | 48 | 24,596,824 (+260,608, +1.07 %) | 3,043,732 | yes |

So a child with 193-interaction AIRs costs the parent about 1 % more main cells and one more stacked column at arity 2. The AIR count, column count and tallest height do not change. The parent's own AIRs stay at 77 interactions or fewer, which is within the new and the old bound.

### (c) G1 as `fp_mul`-class row chains (MEASURED, `out/x14c-g1.jsonl`)

New AIRs (`src/x_exp_node.rs`, `ChainAir`). Each is one stock `FieldExpr` row with exactly one modular constraint (asserted), plus an `id` column and bus messages that carry its inputs and outputs. A message carries at most 2 field values (97 fields), under the LogUp message length bound of 128. `SourceAir` is a test harness that produces the input points and consumes the output; it is not part of the design.

| Chain | Row kind | Constraint | Width | Constraints | Interactions | Degree |
|---|---|---|---:|---:|---:|---:|
| add (3 rows per addition) | `L_add` | `lambda (x2 - x1) = y2 - y1` | 387 | 98 | 199 | 2 |
| | `X3` | `x3 = lambda^2 - x1 - x2` | 339 | 98 | 196 | 2 |
| | `Y3` | `y3 = lambda (x1 - x3) - y1` | 387 | 98 | 197 | 2 |
| double (4 rows per doubling) | `SQ` | `s = x^2` | 243 | 98 | 195 | 2 |
| | `L_dbl` | `lambda 2y = 3 s` | 339 | 98 | 199 | 2 |
| | `X3`, `Y3` | as above, `x2 = x1` | 339, 387 | 98 | 196, 197 | 2 |

`lambda = 3 x^2 / (2 y)` in one row needs two modular constraints (the mod-builder saves `x^2`), so the doubling has a separate `SQ` row. Every row kind stays at 199 interactions or fewer, which is within the 200 bound by 1. Each chain proves and verifies alone at 2^4 rows.

The unit node is arity 1 over one internal-set node, with the chain at 2^h rows (one G1 operation per row of each AIR):

| Unit | AIRs (without the harness) | Columns (without the 289-column harness) | Max interactions | Stacked width at 2^12 / 2^14 / 2^16 / 2^17 | Proved and verified at 2^16 |
|---|---:|---:|---:|---|---|
| add chain | 44 (43) | 2,943 (2,654) | 199 | 47 / 80 / 211 / 386 | yes |
| double chain | 45 (44) | 3,138 (2,849) | 199 | 48 / 86 / 235 / 435 | yes |

Without the harness, both units are inside 64 AIRs and 3,000 columns (DERIVED by subtracting the harness's 289 columns and 1 AIR). With the harness, the double-chain unit has 3,138 columns, over the bound; it proved anyway, because the bound is a soundness assumption, not a check (see above). The tallest AIR is the range checker at 2^18 (17 bits).

Findings:

- **One kind per unit.** A unit with both chains (`L_add`, `L_dbl`, `SQ`, shared `X3` and `Y3`, range) has 1,537 + 387 + 339 + 243 + 339 + 387 + 4 = 3,236 columns, over 3,000 (DERIVED). So the MSM schedule needs units of one kind: addition units, then one doubling unit (for example a bucket MSM: bucket and window sums are additions, and the 255 doublings that combine the windows come last). FV-SCHED must use this schedule (DERIVED).
- **Range checker of the units: an AFK pick that departs from an owner ruling.** With the 16-bit RANGE table of owner ruling LD-S10x-3, these row kinds need up to 3 modular constraints each (MEASURED: the one-constraint assertion fails with 3, `out/x14c-rb16.err`), which breaks the one-constraint-per-row design and the 200-interaction bound. Every measurement in (c) and (d) therefore uses a 17-bit range checker of the unit's own (2^18 rows, under the 2^19 height bound). This departs from LD-S10x-3 (16-bit RANGE table). It is an AFK pick, not a ruling: it **needs explicit owner sign-off**. If the owner keeps 16 bits for the FV units too, the G1 rows must be redesigned (3 constraints per row kind, or narrower limbs) and every figure in (c) and (d) must be measured again.
- **Gemini Quint lanes** (`qa/gemini-quint/x14-l-add`, `x14-l-dbl`, `x14-sq`, `x14-x3`, `x14-y3`): every lane finds the row kinds SOUND and COMPLETE only under conditions that the chain as built does not yet enforce. (1) The denominator must be non-zero: for `L_add` with x1 = x2 and y1 = y2, `lambda` is free (a soundness failure); with x1 = x2 and y1 != y2 there is no witness (a completeness failure for P + (-P)); `L_dbl` needs y != 0. (2) Every `id` must be unique: `id` is not constrained, so two rows with the same `id` can swap inputs on a split message. (3) The producer must range-check input limbs: `FieldExpr` range-checks only its outputs, quotient and carries. (4) No point at infinity is representable. Lead quint runs (`out/x14-lanes-quint*.txt`, final tables in each lane REPORT): after two lane fix turns and two mechanical lead edits (described in the reports), `SQ`, `L_dbl`, `X3` and `Y3` pass every command (typecheck, tests, the sound invariant, non-vacuity, all 5 mutants each). For `L_add` the model passes every check, including all 5 mutants, but its test file `l_add_test.qnt` does not parse (it names points the model does not define); that file is open.
- **What the conditions cost.** Built and measured in (d). The earlier ESTIMATE in this place (about 2,950 columns for the addition unit, under 3,000) was wrong: it left out the producer of the points. The measured unit is over the 3,000-column bound; see (d).

### (d) The condition rows (MEASURED, `out/x14cond-neg.jsonl`, `out/x14cond-g1.jsonl`)

The lanes of (c) found four conditions. The chain now enforces three of them with new rows and constraints (`src/x_exp_node.rs`, `g1_chain`, `PointsIn`, `forge_subrow`, `g1neg`):

| Condition | Mechanism | AIR change |
|---|---|---|
| Non-zero denominator, addition | `INV`: one `FieldExpr` row `w = 1 / (x2 - x1)` (`new_const(1) / (x2 - x1)`), constraint `w (x2 - x1) = 1`. `L_add` sends (id, x1, x2) on bus o+10, `INV` receives it. | new row kind; `L_add` 199 to 200 interactions |
| Non-zero denominator, doubling | `INV_y`: `w = 1 / y`, constraint `w y = 1`. `L_dbl` sends (id, y) on o+10. | new row kind; `L_dbl` 199 to 200 interactions |
| Unique `id` | In every `ChainAir` and in `PointsIn`: first row id = 0, transition next.id = id + 1 (id = row index). | 2 constraints per AIR (98 to 100), no interaction |
| Input range checks | `PointsIn`: columns id and the input points; it range-checks every limb to 8 bits on the unit's range bus and sends the point messages. The chain rows receive their inputs only from `PointsIn`. | new AIR; the old `SourceAir` harness now only receives the output (a 97-column sink) |
| Point at infinity | No new AIR. The inverse rows make the exceptional cases (P1 = P2, P1 = -P2, y = 0) unprovable, so the chain is sound on them and incomplete. An honest MSM must stay away from them: a random or offset accumulator start makes an exceptional case occur with negligible probability (complete with negligible failure, DERIVED). Identity points that appear as proof elements need a flag path. | open (DERIVED) |

Row kinds, standalone at 2^4 rows (MEASURED; both chains prove and verify):

| Chain | AIR | Width | Constraints | Interactions | Degree |
|---|---|---:|---:|---:|---:|
| add | `L_add` | 387 | 100 | 200 | 2 |
| | `X3`, `Y3` | 339, 387 | 100 | 196, 197 | 2 |
| | `INV` | 291 | 100 | 194 | 2 |
| | `PointsIn` (4 values) | 193 | 2 | 194 | 2 |
| double | `SQ`, `L_dbl` | 243, 339 | 100 | 195, 200 | 2 |
| | `X3`, `Y3` | 339, 387 | 100 | 196, 197 | 2 |
| | `INV_y` | 243 | 100 | 194 | 2 |
| | `PointsIn` (2 values) | 97 | 2 | 98 | 2 |

Every AIR is at or under 200 interactions. `L_add` and `L_dbl` are exactly at the bound.

**Negative tests** (mode `g1neg`, 2^4 rows, app set; MEASURED). The forged rows are built with `forge_subrow`, which computes q, carries and range counts from the forged values, so the only failing check is the one under test. Per case: the AIRs whose constraints fail (stock debug checker, run per AIR), the LogUp multiset check, and prove plus verify:

| Case | Constraint failures | LogUp balanced | Prove and verify |
|---|---|---|---|
| add, honest | none | yes | verified |
| add, P2 = P1, forged lambda = 5, forged w = 0 | `INV` only | yes | verify rejects (`SumClaimMismatch`) |
| add, row 1 with id 0 in every AIR | the id constraint of `L_add`, `X3`, `Y3`, `INV`, `PointsIn` | yes | verify rejects |
| dbl, honest | none | yes | verified |
| dbl, P = (0, 0), forged lambda = 5, forged w = 0 | `INV_y` only | yes | verify rejects |
| dbl, row 1 with id 0 in every AIR | the id constraint of `L_dbl`, `X3`, `Y3`, `INV_y`, `PointsIn`, `SQ` | yes | verify rejects |

In the P2 = P1 case, `L_add` itself passes with the forged lambda (its constraint degenerates to 0 = 0). That is the soundness gap of (c), and `INV` alone closes it.

**The unit node with the condition rows** (arity 1 over one internal-set node, `g1c 16 add,dbl 12,14,16,17 16`; MEASURED):

| Unit | AIRs (without the sink) | Columns (without the 97-column sink) | Max interactions | Stacked width at 2^12 / 2^14 / 2^16 / 2^17 | Proved and verified at 2^16 |
|---|---:|---:|---:|---|---|
| add chain | 46 (45) | 3,235 (3,138) | 200 | 49 / 89 / 247 / 459 | yes |
| double chain | 47 (46) | 3,286 (3,189) | 200 | 49 / 90 / 254 / 472 | yes |

`PointsIn` is part of the design here, not the harness, so it is counted. The prove runs were not `--exclusive`, so no prove time is reported.

**Finding: with the condition rows, both units are over the 3,000-column bound of ruling N-6 option 2** (add by 138 columns, double by 189; MEASURED widths, DERIVED differences). The bound is a soundness assumption of the calculator, not a keygen check, so the units still prove. Under the ruling they do not fit. The options (DERIVED from the measured widths; none built):

1. **Raise the column bound of the FV work units to 3,300.** D3 with the internal set at 1,000 constraints, 64 AIRs, 200 interactions (`out/d3-bounds-cols.log`, MEASURED by the calculator): 3,000 columns: stacked 111.0768, weakest 101.1643; 3,300: stacked 110.9393, weakest 101.1643; 3,500: stacked 110.8544, weakest 101.1643. Only the stacked term moves. Every term stays at least 100 and the grindable terms at least 101, with no PoW change. Both units fit under 3,300.
2. **Drop `PointsIn` where the inputs already come from range-checked rows.** In a real MSM, most inputs are outputs of earlier chain rows (saved vars, which `FieldExpr` range-checks). Only the external points (SRS and proof points) need a producer, and that producer can live in another unit. Add unit 3,138 - 193 = 2,945 (fits). Double unit 3,189 - 97 = 3,092 (still over).
3. **Drop `INV_y` once every input point is checked to be on the curve.** BLS12-381 G1 (y^2 = x^3 + 4) has no point with y = 0: -4 is not a cube in Fp (p = 1 mod 3; (-4)^((p-1)/3) != 1), and the curve order h r is odd (DERIVED, checked in Python). So on curve points y != 0, and doubling an on-curve point gives an on-curve point. With options 2 and 3, the double unit is 3,092 - 243 = 2,849 (fits). This moves the condition to an on-curve check at the point producer, which the chain does not have yet.

**Cuts tried to fit under 3,000 (lead direction, 2026-11-13).**

- **Double unit: drop `INV_y` (option 3), MEASURED** (`X14_NO_INV_Y=1`, `out/x14cond-noinvy.jsonl`, `out/x14cond-noinvy-neg.jsonl`). The unit is 46 AIRs and 3,043 columns with the sink, or **45 AIRs and 2,946 columns without it: within 3,000**. Max interactions 199. Stacked width 47 / 83 / 223 / 411 at 2^12 / 2^14 / 2^16 / 2^17. Proved and verified at 2^16. The cost is a new condition: without `INV_y`, the forged (0, 0) doubling of (d) **verifies** (MEASURED: no constraint fails, LogUp balances, verify accepts). So the producer of the doubling unit's input points must check that they are on the curve. That check lives outside the unit, at the point producer. Inputs that are outputs of earlier chain rows need no check: they are on the curve when the chain's inputs are.
- **Add unit: no cheap cut fits (DERIVED from the measured widths).** The sink is already left out of the counts. Dropping it changes nothing. Merging `INV` into `L_add` would share the 96 input columns (580 columns instead of 678), but the merged AIR has two modular constraints and about 392 interactions, over the 200 bound. Taking one input point from the chain's own output (accumulator feedback in a bucket MSM) shrinks `PointsIn` from 4 values to 2: 3,138 - 96 = 3,042, still over by 42, and it needs a feedback bus that is not built. Dropping `INV` reopens the soundness gap of (c), and dropping `PointsIn` leaves the inputs unchecked. So the add unit with its conditions is 45 AIRs and 3,138 columns.
- **AFK pick, FOR OWNER CONFIRMATION:** the FV work units take a column bound of **3,300**, in place of the owner's 3,000 (ruling N-6 option 2), so the add unit (3,138) fits. D3 at 3,300 (internal set, 1,000 constraints, 64 AIRs, 200 interactions): stacked 110.9393, weakest 101.1643, every term at least 100, grindable terms at least 101, no PoW change (MEASURED by the calculator, `out/d3-bounds-cols.log`). The double unit fits under the owner's 3,000 without `INV_y`. If the owner keeps 3,000 for every unit, the add chain needs a redesign (for example a feedback bus with a smaller producer and a further cut of about 42 columns), which is not designed.

Final unit shapes (without the sink):

| Unit | AIRs | Columns | Max interactions | Bound it needs | Label |
|---|---:|---:|---:|---|---|
| add chain: `L_add`, `X3`, `Y3`, `INV`, `PointsIn` (4 values), range | 45 | 3,138 | 200 | 3,300 (AFK pick, owner confirmation) | MEASURED |
| double chain: `SQ`, `L_dbl`, `X3`, `Y3`, `PointsIn` (2 values, on-curve checked by the producer), range | 45 | 2,946 | 199 | 3,000 (owner value) | MEASURED |

Two more open points (DERIVED):

- `PointsIn` sends and range-checks with multiplicity 1 on every row and has no `is_valid` column. A unit therefore works only at exactly 2^h operations. A partly filled unit needs an `is_valid` column in `PointsIn` (one column, a boolean constraint, and the sends multiplied by it).
- A unit with both chains is now 3,236 + 291 + 243 = 3,770 columns before any producer (DERIVED from the measured widths). So the schedule of one kind per unit from (c) holds more strongly.

**Gemini Quint lanes of the condition rows** (`qa/gemini-quint/x14-inv`, `x14-inv-y`, `x14-points-in`, `x14-chain-id`): first pass done, lead quint runs and fix turns in progress; the results follow in the next commit.

### X14 gate under the new bounds

| Check | Result | Label |
|---|---|---|
| stock `fr_mul` (129) and `fp_mul` (193) within 200 interactions | yes | MEASURED |
| G1 addition and doubling as `fp_mul`-class rows within 200 | yes, at most 199 without the condition rows; at most 200 with them (`L_add`, `L_dbl` at the bound) | MEASURED |
| condition rows (non-zero denominator, id = row index, input range checks) reject forged traces | yes, all 4 negative cases rejected, honest cases verify | MEASURED |
| unit node within 64 AIRs and 3,000 columns | `fr_mul` 41 / 1,735; `fp_mul` 41 / 1,831: yes. With the condition rows: add chain 45 / 3,138: **no** (needs 3,300, AFK pick for owner confirmation); double chain without `INV_y` 45 / 2,946: yes (46 / 3,189 with `INV_y`). Without the condition rows: 43 / 2,654 and 44 / 2,849 | MEASURED (chains: DERIVED without the sink) |
| D3 at 3,300 and 3,500 columns | weakest 101.1643, PASS (only the stacked term moves) | MEASURED (calculator) |
| one operation AIR at 2^16 beside one child verification | proved and verified for all four | MEASURED |
| the parent over such a unit | about +1 % main cells | MEASURED |
| D3 at the new bounds | weakest 101.1643, PASS | MEASURED (calculator) |

**GP-3 general verifier, FV-SCHED unit budget:** one unit holds 2^16 operations of one kind (2^17 laid out, not proved). For one `ivc_node` proof (ESTIMATE): the MSM is a bucket MSM with about 23,000 additions in 1 addition unit plus 1 doubling unit, and the final check (AD-22 (a)) is about 20,000 `fp_mul` rows in 1 unit. That is 3 arithmetic units before transcript and expression evaluation, which stay PENDING. Status: the arithmetic part of the unit budget is MEASURED. The gate stays **FREEZE-PENDING** on (1) owner confirmation of the AFK pick of 3,300 columns for the FV work units (the add unit is 3,138; the double unit fits 3,000 without `INV_y`, with an on-curve check at its point producer) and sign-off of the 17-bit range checker (a departure from LD-S10x-3), (2) the infinity flag path for identity points in proofs, (3) the transcript and expression-evaluation AIRs, and (4) the Gemini lanes of the condition rows (see (d)).

### ARCHITECTURE rows that must change (not edited here; `efb0eb78a` text)

- Section 1.9 parameter sets table, row "Internal (and the leaf layer on path C)", column "Shape bounds": "same per-AIR bounds" becomes 1,000 constraints and 200 interactions per AIR, 64 AIRs, 3,000 columns (only for the internal set; the leaf layer row on path C needs its own reading). Add the D3 figure 101.1643 at the new bounds.
- Line 2265 (NODE INTEGRATION, "The node's shape bounds are those of the internal set ... 1,000 constraints and 100 interactions per AIR, 50 AIRs, 2,000 columns"): the new bounds.
- GP-3 general verifier "Row budget" (line 2698, "Unknown. No figure is given") and kit FV-SCHED (line 2689): the unit budget, the one-kind-per-unit schedule, and the operation inventory above.
- X14 row of the experiments table (line 4008) and AD-22 (line 3457): the cost of the final check per SRS.
- N-6 (line 3489) and N-10 (line 3508): the rulings.
- AD-3 (line 3438), the kit PAGE/ROM block cache text at line 1481 ("`K_rom` ... not frozen"), V-4 (line 3562), the decision index row at line 3341 and X15 (line 4009): `K_rom = 15` FROZEN by the owner (2026-11-12 20:35).
- GP-3 general verifier and kit FV-SCHED (lines 2689, 2698): the G1 chain with its condition rows (`INV`, `INV_y`, id = row index, `PointsIn`): an addition is 4 rows plus the point producer, a doubling 5; the add unit is 45 AIRs / 3,138 columns and the double unit without `INV_y` 45 / 2,946 (MEASURED); the column bound of the FV work units is 3,300 as an AFK pick for owner confirmation (D3 unchanged at 101.1643); the double unit's point producer checks that inputs are on the curve; the point at infinity is avoided by the MSM schedule (random or offset accumulator) and needs a flag path for identity points in proofs.
- The RANGE table text of LD-S10x-3: the FV work units use a 17-bit range checker of their own. This is an AFK pick that departs from the ruling and is marked for owner sign-off; do not change the text before the owner signs off.
