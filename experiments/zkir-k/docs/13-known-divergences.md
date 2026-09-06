# Known divergences

A divergence is a program and preimage on which `preprocess` (off-circuit) and `circuit` (in-circuit) disagree, on which the two pinned crates disagree, or on which the definition and an oracle disagree. Each finding gives the behaviour on each side, the K rule that models it, the case and its expected result, its relevance, and its upstream status from `wiki/contradictions.md` and `wiki/zkir/zkir-v3-divergence-review.md`. Relevance is to soundness (the circuit accepts what the specification rejects), to completeness (an honest witness cannot be proved) or to availability (the process aborts).

## Reproducing the cases

`tools/divergence_tests.py` holds thirty-one cases. Each case is a program written to `corpus/divergence/<name>.zkir`, a preimage built in memory (`inputs`, `binding_input` `42`, a commitment when the program asks for one), the expected off-circuit status on both sides, and the outcome one named gate must have. The script runs `checkedJob`, so a static failure appears as `wfError`, and it reports a crate exit code of 101 as `panic`. Every case also runs through the circuit oracle (12-oracles-and-differential-testing.md) and the line records its outcome; the injected cases carry an `--inject` perturbation of one register and the cell the comparison table predicts for it.

```sh
uv run --group zkir-k python experiments/zkir-k/tools/divergence_tests.py
```

Each line reads `PASS  <case> <finding> K=<status> Rust=<status> gate=<name>:<outcome> circuit=<outcome>`, followed for an injected case by `inject=<register>-><value> walk=<cell> AGREE`, and the run ends with `31/31 divergence cases behave as expected` (`evidence/zkir-k-divergence-tests-2026-09-06e.txt`). Outcomes are defined in 08-constraints-and-verdicts.md.

The script does not write the preimages to disk. To rerun `k02` by hand, write `k02.json` as `{"inputs": ["1"], "binding_input": "42"}`:

```sh
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py experiments/zkir-k/corpus/divergence/k02_transcript_too_short_panics.zkir k02.json --checked
```

This prints status `panic` with the K2 error and, in `all_verdicts`, `privateInput` and `add` as `unknown` (`register %x is not in the witness`).

## K1: Jubjub `from_coordinates` uses only the parity of `x`

Off-circuit, `from_coordinates_offcircuit` (`ir_instructions/from_coordinates.rs`) calls `JubjubExtended::from_xy` of midnight-circuits `ecc/curves.rs`, which decompresses `y` with the parity of `x` (`JubjubAffine::from_bytes`) and checks only that the result's `v` equals `y`: any `x` of the right parity yields the point with the true `x`. In circuit, `point_from_coordinates` pins the exact pair.

`fromCoordinatesV(native(X), native(Y))` in `zkir-ops.k` calls `jubjubFromXY` in `zkir-curves.k`, which keeps only `X modInt 2`:

```k
rule jubjubFromXY(X, Y) => #jubjubDecompress(X modInt 2, Y, fsqrt(fdiv(fsub(fsq(Y, #r), 1, #r), fadd(1, fmul(#jubjubD, fsq(Y, #r), #r), #r), #r), #r))
  requires Y >=Int 0 andBool Y <Int #r andBool X >=Int 0 andBool X <Int #r
rule #jubjubDecompress(Sign, Y, sqrtOk(U)) => ptOk(pt(U, Y)) requires U modInt 2 ==Int Sign
rule #jubjubDecompress(Sign, Y, sqrtOk(U)) => ptOk(pt(fneg(U, #r), Y))
  requires U modInt 2 =/=Int Sign andBool U =/=Int 0
```

The gate `#fromCoordsGate` in `zkir-constraints.k` uses the exact test `fromXY(#jubjub, X, Y)` and reports `violated("from_coordinates: (x, y) is not on the curve")`.

Case `k01_jubjub_from_coordinates_parity_only` feeds `(x + 2, y)` of the generator, with a `Point<Jubjub>` input to enable the chip. Both sides are ok, with identical memories (`%px` holds the true `x`), and the gate is violated. No false proof results; the concern is that the Agda trust base assumes `fromCoordsJ-coordsJ` (a successful call returns exactly `(x, y)`), which the crate does not satisfy. The finding is not among those of the arc-zkir review; report upstream.

## K2: a transcript read past the end panics

`preprocess` (`ir_vm.rs`) slices `preimage.private_transcript[idx..idx + w]` before checking its length (likewise `public_transcript_outputs`), so a short transcript aborts with `range end index 1 out of range for slice of length 0` at `zkir-v3/src/ir_vm.rs:379:33`. The extension crate does the same. When `Idx +Int encodedLen(T)` exceeds the transcript, `zkir-vm.k` rewrites `#input(boolOk(true), T, O, "private")` with `<genMode> false` to `#panicNow("range end index out of range: private transcript too short")`, and `#panicNow` sets `<status>` to `panic(S)`.

Case `k02_transcript_too_short_panics` runs an unguarded `private_input` against an empty private transcript. Both sides panic, and the gate `private_input` is unknown because the register never exists. Availability only. `wiki/contradictions.md` names K2 only inside the K3 disposition, as the same class of candidate robustness finding, and the case is its record.

## K3: a Bytes32 raw element outside canonical form panics

`decode_offcircuit` for `Bytes32` (`ir_instructions/encode.rs`) runs `assert_eq!(bytes[31], 0)` on the low element and asserts every byte but the first of the high element is zero, so an element with byte 31 set aborts at `encode.rs:155:17`. `zkir-values.k` models the assertion:

```k
rule decodeValue(ListItem(Lo) ListItem(Hi), bytes32()) => decPanic("assertion failed: Bytes32 low element uses byte 31 or high element exceeds a byte")
  requires notBool (fitsBits(Lo, 248) andBool fitsBits(Hi, 8))
```

`#toValue(decPanic(S))` becomes `vPanic(S)`, which `#put` turns into `#panicNow`. Case `k03_bytes32_input_assertion_panics` gives the input `[2^248, 0]` for a `Bytes<32>` and then runs `reverse_bytes`. Both sides panic and the gate `reverse_bytes` is unknown. The relevance is availability, and K3 is a candidate robustness finding for upstream.

## K4: the Jubjub chip is not enabled for two instructions

`IrSource::used_chips` (`ir_vm.rs`) enables the Jubjub chip only for `JubjubPoint` or `JubjubScalar` among the program inputs or a `public_input`/`private_input`, or for `hash_to_curve`. The circuit arms of `jubjub_scalar_from_native` (`jubjub_scalar_from_biguint`, `encode.rs`) and of native `from_coordinates` call `std_lib.jubjub()`, and `ZkStdLib::jubjub` (midnight-zk-stdlib `lib.rs`) unwraps with `expect("ZkStdLibArch must enable jubjub")`, so key generation panics.

`usedChips` in `zkir-constraints.k` reproduces the scanner. The `jubjubScalarFromNative` gate checks `#chipFor(jubjubScalar(), Chips)`, and the `fromCoordinates` gate checks `#fromCoordsChip`, which for two natives is `#chipFor(jubjubPoint(), Chips)`. Both report `synthErr("chip not initialised for ...")`.

Cases `k04_jubjub_scalar_from_native_chip` (native 7, then `ec_mul_generator`) and `k01b_jubjub_from_coordinates_no_chip` (the generator's coordinates, no Jubjub-typed input) are ok on both sides with the gate synthErr. Completeness. Finding 13 calls `from_bytes32` the only such entry (fixed in PR #656 and #667); these two are its extension, to be reported upstream.

## K5: `less_than` with 253 or 254 bits

Off-circuit, `resolve_operand_bits` rejects only `bits >= 255` (`checkBits(native(_), N) => cErr("Excessive bit bound") requires N >=Int #frBits` in `zkir-ops.k`). In circuit, `std.lower_than(layouter, &a, &b, u32::max(*bits + *bits % 2, 4))` pads the width and `bounded_of_element` (midnight-circuits `field/native/native_gadget.rs`) asserts `n <= MAX_BOUND_IN_BITS`, that is `F::NUM_BITS - 2` = 253. Bits 253 and 254 therefore pass `preprocess` and panic at key generation.

```k
rule eval(gate(lessThan(_, _, N, _)), _, _, _) => synthErr("less_than: padded bound " +String Int2String(#ltBits(N)) +String " exceeds MAX_BOUND_IN_BITS = 253")
  requires #ltBits(N) >Int 253
rule #ltBits(N) => maxInt(N +Int (N modInt 2), 4)
```

The static check in `zkir-syntax.k` keeps the specification's bound of 255, so the case reaches the gate. Case `k05_less_than_253_bits_keygen` uses bits 253 with inputs 1 and 2: ok on both sides, gate synthErr. Completeness, and the same class as finding 10; report upstream.

## K6: the two crates disagree on a non-canonical Bytes32 element

The element of K3 (low element at or above `2^248`, or high element at or above 256) is an assertion panic in the 92e8bdd3 crate but an ordinary decode error in the 2ffe2d1 crate, whose `decode_bytes` (`ir_instructions/encode.rs`) returns `None` for a non-zero byte outside the chunk. The definition models both: `decPanic` on the base surface, and under the strict decoding of the extension surface the rule in `zkir-values.k`

```k
rule #canonical(decPanic(_), _, bytes32()) => decErr("Failed to decode as Bytes32")
```

No divergence case covers K6, but `unit_values.py` has the check `dec bytes32 bad high, strict`. A run-level reproduction uses `corpus/midnight-zkir-2ffe2d1-tests/test_bytes32_proof.zkir` (inputs `%native`, `%secp_base`, `%secp_scalar`, `%raw: Bytes<32>`) with `k6.json`, whose sixth element is `2^248`:

```json
{"inputs": ["1", "1", "0", "1", "0", "452312848583266388373324160190187140051835877600158453279131187530910662656", "0"], "binding_input": "0"}
```

`zkir_run.py` on these files with `--ext` gives status `error` and the error `Failed to decode as Bytes32`. The oracle `repos/_build/midnight-zkir-2ffe2d1/target/release/zkir-oracle` gives

```json
{"status":"error","error":"Failed to decode [Fq(0x0100000000000000000000000000000000000000000000000000000000000000), Fq(0x0000000000000000000000000000000000000000000000000000000000000000)] as Bytes(32)"}
```

Without `--ext` the definition reports the K3 panic and the 92e8bdd3 oracle aborts with ``assertion `left == right` failed``, `left: 1`, `right: 0`. Availability on the base surface only. Reported with K3; the newer crate already rejects the element with an ordinary decode error.

## Extension surface: `test_eq` on byte strings of unequal length

At midnight-zkir 2ffe2d1, `test_eq_offcircuit` (`ir_instructions/eq.rs`) compares two `Bytes` vectors with `xs == ys`, so different lengths are false, while `test_eq_incircuit` requires `xs.len() == ys.len()` and otherwise returns `Error::Synthesis`. In `zkir-ext.k`, `eqDispatch(bytesV(_), bytes32(_)) => eqFalse()` and its mirror give the off-circuit value (`#testEq(eqFalse(), _, _) => vOk(native(0))` in `zkir-ops.k`). The gate falls to the generic different-types rule of `#eqSupported` in `zkir-constraints.k`, and because `tools/zkir_kast.py` maps `Bytes<32>` to `bytes32()` the message names `Bytes32`.

No divergence case covers the extension surface. Reproduction: `test_eq.zkir`

```json
{"version": {"major": 3, "minor": 0},
 "inputs": [{"name": "%a", "type": "Bytes<32>"}, {"name": "%b", "type": "Bytes<4>"}],
 "outputs": [], "do_communications_commitment": false,
 "instructions": [{"op": "test_eq", "a": "%a", "b": "%b", "output": "%eq"}]}
```

with `test_eq.json` `{"inputs": ["1", "0", "1"], "binding_input": "0"}`:

```sh
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py test_eq.zkir test_eq.json --ext --checked
repos/_build/midnight-zkir-2ffe2d1/target/release/zkir-oracle test_eq.zkir test_eq.json
```

Both give status ok and `%eq` encoded `["0"]`, while the K `all_verdicts` has `synthErr`, `Unsupported test_eq: Bytes32 == Bytes` for the `testEq` gate. Completeness. The finding is outside the arc-zkir review, which is pinned at 92e8bdd3.

## Findings of the arc-zkir review reproduced by the cases

Findings 1, 2 and 4 are retired as producer obligations or undefined behaviour. Finding 3 is by design, and finding 12 is a design omission, not an off-circuit/in-circuit split. Findings 5 to 11 and 13 are the active discrepancies.

### `assert` of a non-boolean (finding 2)

Off-circuit, `resolve_operand_bool` demands 0 or 1 (`asBool` in `zkir-ops.k`: `Expected boolean, found: 2`). `std.assert_non_zero` in circuit only excludes zero, modelled by `#nonZero` in the `assert` gate. Case `f02_assert_non_boolean` with input 2 errors on both sides and the gate holds, since the non-zero constraint accepts what `preprocess` rejects. Booleanity is a producer obligation, and the finding is retired.

### `cond_select` on Bytes32 (finding 6)

`selectV` in `zkir-ops.k` returns a value for any pair of the same type, while the gate reports `synthErr("Unsupported cond_select: Bytes32 ? Bytes32")` through `#eqSupported`. Case `f06_cond_select_bytes32` is ok on both sides with the gate synthErr. Completeness. PR #656 partially fixes it (rejected off-circuit); the `JubjubScalar` arm remains open.

### `constrain_eq` on JubjubScalar (finding 7)

`constrainEqV` in `zkir-ops.k` type-checks any pair of the same type and succeeds only when the values are equal (otherwise `cErr("Equality constraint failed")`). The gate reports `synthErr("Unsupported constrain_eq: JubjubScalar == JubjubScalar")`. Case `f07_constrain_eq_jubjub_scalar` with scalars 42 and 42 is ok on both sides with the gate synthErr, a completeness finding partially fixed in PR #656; the in-circuit arm remains open.

### `bytes32_from_low_high` with a foreign `high` (finding 8)

Off-circuit, `bytes32FromLowHighV` accepts any bounded field element. The gate requires `#native(rd(H, M), "bytes32_from_low_high high")` and reports `synthErr: bytes32_from_low_high high: cannot convert Secp256k1Base to Native`. Case `f08_bytes32_from_low_high_foreign_high`: ok on both sides, gate synthErr. Completeness. Fixed in PR #656 and #668.

### `less_than` with a 3-bit bound (finding 4)

Off-circuit `checkBits` rejects 8 (`Bit bound failed: 8 is not 3-bit`), but the gate uses the padded bound `#ltBits(3)` = 4, which a witness holding 8 and 9 would satisfy. Case `f04_less_than_odd_bits` errors on both sides, and the gate is unknown because `%lt` is never written. Soundness in principle, retired as a producer obligation; the Agda model follows the chip.

### secp256k1 reached only through `from_bytes32` (finding 13)

`used_chips` ignores `from_bytes32`, so `#chipFor(T, Chips)` in the gate reports `synthErr: chip not initialised for Secp256k1Base`. Case `f13_chip_gating_from_bytes32`: ok on both sides, gate synthErr. Completeness. Fixed in PR #656 and #667.

### The order-2 Curve25519 point (finding 11)

`(0, -1)` lies on the curve but not in the prime-order subgroup. `Curve25519Subgroup::from_edwards` returns `None`, so both sides error (`#ptToValue` with `notBool inSubgroup(C, Q)` in `zkir-ops.k`) and `#fromCoordsPt` reports `violated("from_coordinates: point is not in the prime-order subgroup")`. Case `f11_curve25519_torsion_point`: error on both sides, gate violated. Availability upstream; the decoder panic is fixed in midnight-circuits 7.2.4, and the in-circuit hint panic remains open.

### Non-canonical secp256k1 input and the commitment gate (finding 5)

Both sides decode limbs encoding `p + 5` as the reduced value 5. The preimage's commitment comes from a `genJob` run over the raw stream, which `preprocess` accepts. In circuit it is recomputed over the canonical re-encoding (`ir_vm.rs`, commitment block), so `commGate` in `zkir-constraints.k` reports `violated("communications commitment differs from poseidon(rand ++ encode(inputs) ++ encode(outputs))")`. Case `f05_noncanonical_foreign_limbs`: ok on both sides, `commGate` violated. Completeness. The panic on limbs at least `2^192` is fixed in midnight-circuits 7.2.4; the silent reduction remains open.

### Other cases

| Case | Behaviour | Expected | Status |
|---|---|---|---|
| `f01_reconstitute_overflow` | `divisor * 2^bits + modulus >= r` errors off-circuit (`#recon4` in `zkir-vm.k`); the gate would wrap modulo `r` | error / error, gate unknown | Retired, undefined behaviour |
| `f03_guard_uncoupled` | an inactive guard yields the default value; the `publicInput` gate never reads the guard | ok / ok, gate holds | By design |
| `f10_reconstitute_bits_256` | `bits > 248` rejected statically (`#checkArity` in `zkir-syntax.k`); the crate errors with `Excessive bit count`, and keygen fails the K8 assertion | wfError / error, circuit preprocess-error | Fixed in PR #656 |
| `f12_ec_mul_generator_p256` | no generator for secp256r1 scalars on either side | error / error, gate synthErr | Design omission, not a divergence |
| `k06_empty_impact_guard` | an empty `impact` still emits `guardGate`; guard 2 is unsatisfiable | error / error, `guardGate` violated | Modelled |
| `k07_alignment_option_offcircuit` | `preprocess` parses option alignments; `alignedBytesCircuit` rejects them as not implemented | ok / ok, gate synthErr | Modelled |

## Injected and stage cases

Eleven cases were added with the circuit oracle. Nine of them perturb one register of the preprocessed memory through the oracle's `--inject` and check the cell the comparison table's walk predicts (`plan-iter3/circuit-comparison-table.md`, D3 to D5); the K run is the honest one, so the gate outcome is that of the honest witness. Two record the widths that `preprocess` rejects while keygen keys.

| Case | Program | Injection | K / Rust, gate | Circuit |
|---|---|---|---|---|
| `e01a_pub_guard1_free` | `public_input` with guard 1, no consumer | `%v` set to 3 | ok / ok, `public_input` holds | accepted (`A`) |
| `e01b_pub_guard1_impact` | the same register read by an `impact` | `%v` set to 3 | ok / ok, `public_input` holds | witness-consistency-error (`W`, `pi_push`) |
| `e02_inv_zero_inject` | `inv` | operand set to 0 | ok / ok, `inv` holds | witness-consistency-error (`W`: the hint stores 0) |
| `e03_b32_high_inject` | `bytes32_from_low_high` | high operand set to 256 | ok / ok, gate holds | panic (`X`, `AssignedByte`) |
| `e03b_b32_low_inject` | `bytes32_from_low_high` | low operand with byte 31 set | ok / ok, gate holds | constraint-failure (`C`) |
| `e04_lt_bound_inject` | `less_than` with `bits` 4 | operand set to 16 | ok / ok, `less_than` holds | panic (`X`, `AssignedBounded`) |
| `e10_align_overflow` | `persistent_hash` over a one-byte atom | operand set to 300 | error / error, `persistent_hash` violated | witness-consistency-error (`W`) |
| `e11_violated_then_synth` | a violated `from_coordinates` followed by a `constrain_eq` on JubjubScalar | none | ok / ok, `from_coordinates` violated | panic (`X`: the synthesis error fires first) |
| `e21a_div_mod_249` | `div_mod_power_of_two` with 249 bits | none | wfError / error, n/a | preprocess-error; keygen keys the program |
| `e21b_reconstitute_249` | `reconstitute_field` with 249 bits | none | wfError / error, n/a | preprocess-error; keygen keys the program |
| `k08_load_constant_jubjub_chip` | K7 above | none | ok / ok, `load_constant` synthErr | panic (`X`, chip) |

## Loader difference: a repeated `0x` prefix

`Operand::deserialize` in `ir.rs` strips one `0x` and hands the rest to `const_hex::decode`, which strips a further `0x`, so `0x0x01` loads as the byte `01`. `immediate` in `tools/zkir_kast.py` requires an even number of hex digits and exits 2. Reproduction: `hex.zkir`

```json
{"version": {"major": 3, "minor": 0},
 "inputs": [{"name": "%a", "type": "Scalar<BLS12-381>"}],
 "outputs": [], "do_communications_commitment": false,
 "instructions": [{"op": "add", "a": "%a", "b": "0x0x01", "output": "%y"}]}
```

with `hex.json` `{"inputs": ["1"], "binding_input": "0"}`: `zkir_run.py hex.zkir hex.json` prints `format error: invalid hex immediate '0x0x01': odd length or non-hex character` and exits 2, and `repos/_build/ledger-92e8bdd3/target/release/zkir-oracle hex.zkir hex.json` returns `%y` encoded `["2"]`. No semantic effect: a program both loaders accept has the same immediates on both.

## K7: `load_constant` of a Jubjub value with no Jubjub input

On the extension surface `used_chips` (midnight-zkir 2ffe2d1) enables the Jubjub chip from the input types and the `public_input` / `private_input` types only, so a program whose only Jubjub value comes from `load_constant` passes `preprocess` on both sides while `assign_constant_incircuit` calls `std_lib.jubjub()`, which panics at synthesis with `ZkStdLibArch must enable jubjub`. The same class as K4. The K gate is `synthErr("chip not initialised for JubjubPoint")`, the target contract fails `chips.gating`, and the case is `k08_load_constant_jubjub_chip` on the extension definition and the 2ffe2d1 oracles: ok on both sides, gate synthErr, circuit outcome `panic` with `ZkStdLibArch must enable jubjub`. The issue text is `plan-iter3/upstream-issues/K7.md`.

## K8: `reconstitute_field` with 0 bits

`reconstitute_field` with `bits: 0` loads on both crates. Off-circuit the divisor bound is `FR_BITS - 0 = 255` bits, which `resolve_operand_bits` rejects as "Excessive bit bound" on every value (`checkBits(native(_), N) => cErr("Excessive bit bound") requires N >=Int #frBits` in `zkir-ops.k`, reached through `#recon3` in `zkir-vm.k`). In circuit `assert_lower_than_fixed(divisor, 1 << 255)` reaches `assign_less_than_pow2` in midnight-circuits `field/decomposition/chip.rs`, whose `assert!((bit_length as u32) < F::NUM_BITS)` fails at key generation, a panic rather than an error. The static check rejects the program:

```k
rule #checkArity(reconstituteField(_, _, 0, _), _) => wfError("reconstitute_field: bits 0 makes the divisor bound 255, an excessive bit bound")
```

and the target contract fails the both-stage obligation `width.reconstitute_field.assertion` with the assertion text (16-compilation-target-contract.md). The same assertion fails at 255 bits, where the modulus bound reaches 255, and above 255, where `FR_BITS - bits` wraps; case `f10_reconstitute_bits_256` reaches it, so the obligation covers bits 0 and every width of 255 or more, while 249 to 254 bits key and are rejected by `preprocess` alone (cases `e21a`, `e21b`). The program is `corpus/handmade-negative/reconstitute_bits_0.zkir`; `evidence/zkir-k-provability-2026-09-06e.txt` records the keygen panic on the assertion for it and for `f10`. Availability, the same class as K5 (a width that only the chip checks, by an assertion); the issue text is `plan-iter3/upstream-issues/K8.md`.

## Summary

| Finding | Off-circuit | In-circuit | Case | Expected | Relevance | Status |
|---|---|---|---|---|---|---|
| K1 | parity of `x` only | exact `(x, y)` | `k01` | ok / ok, violated | trust-base assumption | report upstream |
| K2 | panic on short transcript | register absent | `k02` | panic / panic, unknown | availability | candidate, report upstream |
| K3 | panic on non-canonical Bytes32 | register absent | `k03` | panic / panic, unknown | availability | candidate, report upstream |
| K4 | accepted | Jubjub chip absent | `k04`, `k01b` | ok / ok, synthErr | completeness | extension of finding 13 |
| K5 | bits 253 accepted | padded bound above 253 | `k05` | ok / ok, synthErr | completeness | report upstream |
| K6 | 92e8bdd3 panics, 2ffe2d1 errors | register absent | by hand, `test_bytes32_proof` | panic or error | availability (base only) | reported with K3 |
| K7 | accepted | Jubjub chip absent (extension) | `k08` | ok / ok, synthErr | completeness | issue text written |
| K8 | bits 0 rejected on every value | keygen assertion (also at 255 bits and above) | `handmade-negative/reconstitute_bits_0`, `f10` | wfError / error | availability | candidate, issue text written |
| ext `test_eq` | false | synthesis error | by hand, `--ext` | ok / ok, synthErr | completeness | recorded |
| Finding 2 | cond must be 1 | cond non-zero | `f02` | error / error, holds | producer obligation | retired |
| Finding 6 | accepted | no arm | `f06` | ok / ok, synthErr | completeness | partially fixed, PR #656 |
| Finding 7 | equal values accepted | no arm | `f07` | ok / ok, synthErr | completeness | partially fixed, PR #656 |
| Finding 8 | foreign high accepted | Native required | `f08` | ok / ok, synthErr | completeness | fixed, PR #656, #668 |
| Finding 4 | nominal bound | padded bound | `f04` | error / error, unknown | producer obligation | retired |
| Finding 13 | accepted | chip absent | `f13` | ok / ok, synthErr | completeness | fixed, PR #656, #667 |
| Finding 11 | error | unsatisfiable | `f11` | error / error, violated | availability upstream | partly fixed, 7.2.4 |
| Finding 5 | reduced limbs accepted | canonical re-encoding | `f05` | ok / ok, `commGate` violated | completeness | partly fixed, 7.2.4 |
| `0x0x` prefix | crate loads | preprocessor exits 2 | by hand | load / format error | none | loader difference |
