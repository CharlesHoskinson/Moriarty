# Instruction reference

This chapter has one entry for each of the 34 instructions of the base surface, ZKIR v3 at midnight-ledger 92e8bdd3, as the definition implements them. The extension surface (midnight-zkir 2ffe2d1) adds nine instructions (`and`, `or`, `xor`, `concat`, `slice`, `nth`, `reverse`, `load_constant`, `sha512`) and the `Bool`, `Byte` and `Bytes<n>` types; they are described in [09-extension-surface.md](09-extension-surface.md) and do not appear here.

Every instruction is a production of sort `Instr` in `zkir-syntax.k`, module `ZKIR-SYNTAX`. Its `symbol(...)` attribute is the JSON `op`; its arguments follow the field order of the crate's `enum Instruction` (`zkir-v3/src/ir.rs`), with the written identifiers last. `reads(Instr)` and `writes(Instr)` give the operands an instruction resolves, in the order of `IrSource::preprocess`, and the identifiers it defines.

## How to read an entry

The entries follow the order of the `Instr` productions in `zkir-syntax.k`; the heading above each run of entries names its family, so a family can appear more than once. Each entry has the same parts:

| Part | What it states | Source |
|---|---|---|
| Syntax | K constructor, argument sorts and argument names | `zkir-syntax.k` |
| Off-circuit | the witness rule `#exec(...)`, the `zkir-ops.k` function it calls, its type dispatch and every error message | `zkir-vm.k`, `zkir-ops.k`; the crate's `<op>_offcircuit` in `ir_instructions/*.rs` or the arm of `preprocess` in `ir_vm.rs` |
| Gate | the `eval` rule of the emitted constraint, as the sequence of checks it makes, and the chips it needs | `zkir-constraints.k`; the crate's `<op>_incircuit` or the arm of `Relation::circuit` |
| Checks | arity and bit-count checks made before the operation, and the static counterpart in `ZKIR-WF` | `zkir-vm.k`, `zkir-syntax.k` module `ZKIR-WF` |

An entry without a Checks part has no check beyond operand resolution and type dispatch. The corpus programs for each instruction are in the table at the end. Outcomes (holds, violated, synthErr, unknown, unsupported) are defined in [08-constraints-and-verdicts.md](08-constraints-and-verdicts.md), values and `encodeValue` in [04-values-and-encoding.md](04-values-and-encoding.md), field, curve and hash functions in [05-fields-curves-and-hashes.md](05-fields-curves-and-hashes.md), transcripts and the public-input vector in [06-configuration-and-run-lifecycle.md](06-configuration-and-run-lifecycle.md).

## Machinery shared by every entry

### Sequencing

The sequencing rule of `zkir-vm.k` emits the gate of an instruction before running its witness half:

```k
rule <k> I:Instr => #exec(I) ... </k> <constraints> Cs => Cs ListItem(gate(I)) </constraints>
  requires notBool isSpecialEmit(I)
rule <k> #exec(_) => .K ... </k> <status> error(_) </status>
rule <k> #exec(_) => .K ... </k> <status> panic(_) </status>
```

`isSpecialEmit` is true for `impact` and `output` only; their emission rules are in their entries. Once `<status>` has left `ok()`, later witness halves are discarded while their gates are still emitted.

### Off-circuit resolution

`resolve(Operand, Map)` turns an immediate into `native(I)`, reads a variable from `<mem>`, and fails on a missing register with `variable not found: Identifier("x")`, the text of the `idx` closure in `preprocess`. Operands are resolved in `reads` order, so the first failing operand determines the message. The derived resolvers:

| Resolver | Crate counterpart | Accepts | Error messages |
|---|---|---|---|
| `resolveBool` (via `asBool`, `zkir-ops.k`) | `resolve_operand_bool` | `native(0)`, `native(1)` | `Expected boolean, found: V`; `cannot convert T to Native` |
| `asNative` (`zkir-ops.k`) | `try_into` to `Fr` | natives | `cannot convert T to Native` |
| `checkBits(V, N)` (`zkir-ops.k`) | `resolve_operand_bits` with a bound | a native below `2^N`, `N < 255` | `cannot convert T to Native`; `Excessive bit bound` (`N >= 255`); `Bit bound failed: V is not N-bit` |
| `resolveNatives` (`zkir-vm.k`) | the `try_into` chains of the hash and impact arms | natives, one at a time | `cannot convert T to Native` for the first offender |
| `asBytes32` (`zkir-ops.k`) | `try_into` to `[u8; 32]` | `bytes32(B)` | `cannot convert T to Bytes32` |

`T` in a message is the name from `typeName` in `zkir-values.k` (`Native`, `Bytes32`, `JubjubPoint`, `JubjubScalar`, `Secp256k1Point`, `Secp256k1Base`, `Secp256k1Scalar`, `Secp256r1Point`, `Secp256r1Base`, `Secp256r1Scalar`, `Curve25519Point`, `Curve25519Base`, `Curve25519Scalar`). Most witness rules are `#exec(I) => #put(O, f(resolve(A, M), ...))`, binary functions through `#bin`, unary ones through `#un`. `#put` and `#put2` store a `vOk`, turn a `vErr(S)` into `#fail(S)`, which sets `<status>` to `error(S)`, and a `vPanic(S)` into `panic(S)`; `#check` does the same for a `CheckResult`. Every `zkir-ops.k` function dispatches on the constructors of its arguments and ends in an `[owise]` rule with an `Unsupported ...` message, which the entries quote after the supported arms. The messages follow the crate's texts closely, not always to the character.

### In-circuit evaluation

`eval(gate(I), M, Pi, Chips)` in `zkir-constraints.k` reads the final `<mem>` through `rd` and `rdId` and combines outcomes with `#and`, whose first non-`holds` argument wins:

```k
rule #and(holds(), O) => O
rule #and(O, _) => O requires O =/=K holds()
```

The building blocks, with the message each produces; `what` is the label quoted in the entry:

| Block | holds when | otherwise |
|---|---|---|
| `#need(R)` | the register exists | `unknown("register x is not in the witness")` |
| `#native(R, what)` | a native | `synthErr("what: cannot convert T to Native")`; `unknown` if absent |
| `#boolean(R, what)` | `native(0)` or `native(1)` | `violated("what is not boolean: V")`; `synthErr("what: cannot convert T to Native")`; `unknown` if absent |
| `#bits(R, N)` | `N >= 255`, or the native fits in `N` bits | `violated("value does not fit in N bits")` |
| `#eqValues(R1, R2, what)` | both present and equal | `violated(what)`; `unknown` if one is absent |
| `#matches(E, R, what)` | the expected `vOk(V)` equals the register | `synthErr("what: S")` when `E` is `vErr(S)`; `violated(what)`; `unknown` if the register is absent |
| `#chipFor(T, Chips)`, `#chipsForValues(L, Chips)` | the chip of `T` (of each value's type) is in `<chips>` | `synthErr("chip not initialised for T")` |
| `#chipNamed(N, Chips)` | the chip `N` is in `<chips>` | `synthErr("chip not initialised: N")` |
| `#eqSupported(R1, R2, op)` | same type with an in-circuit arm | `synthErr("Unsupported op: T1 == T2")`; for a `Bytes32` pair in `cond_select`, `synthErr("Unsupported cond_select: Bytes32 ? Bytes32")` |

Since `#and` keeps the first non-`holds` outcome, an absent register gives `unknown` only when its check is the first to fail; a width, chip or value check placed earlier in the rule gives `synthErr` or `violated` instead. Each entry lists the checks in the order of its rule.

"Standard" as a gate description means: `#need` on every operand register, `#chipsForValues` on the operand values, then `#matches(f(operands), rdId(O, M), "op output")` with the same `zkir-ops.k` function `f` as off-circuit (through `#bin2` or `#un2`), so the gate is `violated("op output")` when the output differs and `synthErr("op output: Unsupported ...")` for a pair the function rejects, the dispatch of the crate's `<op>_incircuit`.

`<chips>` is filled by `usedChips` (`zkir-constraints.k`, the crate's `used_chips`) from the input types, the `public_input` and `private_input` types, the hash instructions (`hash_to_curve`: `jubjub` and `poseidon`; `transient_hash`: `poseidon`; `persistent_hash`: `sha2_256`; `keccak256`: `keccak_256`) and the commitment flag (`poseidon`). The chip names are `jubjub`, `poseidon`, `sha2_256`, `keccak_256`, `secp256k1`, `p256` and `curve25519`; `chipOfType` maps each type to its chip, `Native` and `Bytes32` to none.

## Summary

| op | reads | writes | chip needed by the gate | off-circuit function | gate |
|---|---|---|---|---|---|
| `encode` | input | outputs | type of the input | `encodeValue` (`zkir-values.k`) | `gate(encode(..))` |
| `assert` | cond | none | none | `asBool` | `gate(assert(..))` |
| `cond_select` | bit, a, b | output | types of a, b | `selectV` | `gate(condSelect(..))` |
| `constrain_bits` | val | none | none | `checkBits` | `gate(constrainBits(..))` |
| `constrain_eq` | a, b | none | types of a, b | `constrainEqV` | `gate(constrainEq(..))` |
| `constrain_to_boolean` | val | none | none | `asBool` | `gate(constrainToBoolean(..))` |
| `copy` | val | output | none | `resolve` (`zkir-vm.k`) | `gate(copy(..))` |
| `impact` | guard, inputs | none | none | `#impact` (`zkir-vm.k`) | `guardGate(g)`, one `piGate` per input |
| `ec_mul` | a, scalar | output | types of a, scalar | `ecMulV` | `gate(ecMul(..))` |
| `ec_mul_generator` | scalar | output | type of scalar | `ecMulGeneratorV` | `gate(ecMulGenerator(..))` |
| `hash_to_curve` | inputs | output | `poseidon`, `jubjub` | `hashToCurve` (`zkir-hash.k`) | `gate(hashToCurve(..))` |
| `into_coordinates` | point | x, y | type of point | `intoCoordinatesV` | `gate(intoCoordinates(..))` |
| `from_coordinates` | x, y | output | types of x, y; `jubjub` for two natives | `fromCoordinatesV` | `gate(fromCoordinates(..))` |
| `into_bytes32` | input | output | type of input | `intoBytes32V` | `gate(intoBytes32(..))` |
| `from_bytes32` | bytes | output | target type | `fromBytes32V` | `gate(fromBytes32(..))` |
| `reverse_bytes` | bytes | output | none | `reverseBytes` (K `BYTES` builtin) | `gate(reverseBytes(..))` |
| `bytes32_into_low_high` | bytes | low, high | none | `bytes32IntoLowHighV` | `gate(bytes32IntoLowHigh(..))` |
| `bytes32_from_low_high` | low, high | output | type of low | `bytes32FromLowHighV` | `gate(bytes32FromLowHigh(..))` |
| `div_mod_power_of_two` | val | outputs (2) | none | `#divMod` (`zkir-vm.k`) | `gate(divModPowerOfTwo(..))` |
| `reconstitute_field` | modulus, divisor | output | none | `#recon` (`zkir-vm.k`) | `gate(reconstituteField(..))` |
| `transient_hash` | inputs | output | `poseidon` | `poseidonHash` (`zkir-hash.k`) | `gate(transientHash(..))` |
| `persistent_hash` | inputs | output | `sha2_256` | `alignedBytes`, then `sha256Bytes` (`zkir-hash.k`) | `gate(persistentHash(..))` |
| `keccak256` | inputs | output | `keccak_256` | `alignedBytes`, then `keccak256Bytes` (`zkir-hash.k`) | `gate(keccak256(..))` |
| `test_eq` | a, b | output | types of a, b | `testEqV` | `gate(testEq(..))` |
| `add` | a, b | output | types of a, b | `addV` | `gate(add(..))` |
| `mul` | a, b | output | types of a, b | `mulV` | `gate(mul(..))` |
| `neg` | a | output | type of a | `negV` | `gate(neg(..))` |
| `inv` | a | output | type of a | `invV` | `gate(inv(..))` |
| `not` | a | output | none | `asBool` | `gate(not(..))` |
| `less_than` | a, b | output | none | `#lt` (`zkir-vm.k`) | `gate(lessThan(..))` |
| `jubjub_scalar_from_native` | native | output | `jubjub` | `#jjScalar` (`zkir-vm.k`) | `gate(jubjubScalarFromNative(..))` |
| `public_input` | guard, if present | output | declared type | `#input` (`zkir-vm.k`) | `gate(publicInput(..))` |
| `private_input` | guard, if present | output | declared type | `#input` (`zkir-vm.k`) | `gate(privateInput(..))` |
| `output` | vals | none | none | `#output` (`zkir-vm.k`) | `outputGate(vals, signature)` |

"Types of a, b" means the chips of the operand values' types, checked by `#chipsForValues`; "none" means the gate calls no chip predicate. A function without a file is in `zkir-ops.k`.

## Encoding

### encode

- Syntax: `encode(Operand, Ids)`: input, outputs.
- Off-circuit: `#encode` computes `encodeValue(V)`; every type encodes, with the lengths of `encodedLen`. A length other than the output count fails with `Unexpected output length of encode instruction: T`; `#bindEncoded` then writes one `native` per identifier.
- Gate: `#need` and `#chipsForValues` on the input; a length mismatch is `synthErr("Unexpected output length of encode instruction")`; each output must equal its element ("encode output").
- Checks: the output count, at run time only; `ZKIR-WF` cannot check it because the input type is dynamic.

## Assertions

### assert

- Syntax: `assert(Operand)`: cond.
- Off-circuit: `#assert(resolveBool(C, M))`; a false condition fails with `Failed direct assertion`, the `bail!` of the `I::Assert` arm.
- Gate: `#native(cond, "assert")`, then `#nonZero`: zero is `violated("assert of zero")`. Booleanity is not enforced in circuit (`std.assert_non_zero`), so a condition of 2 fails off-circuit but holds in the gate: divergence `f02`, see [13-known-divergences.md](13-known-divergences.md).

## Control

### cond_select

- Syntax: `condSelect(Operand, Operand, Operand, String)`: bit, a, b, output.
- Off-circuit: `#sel` resolves the bit with `resolveBool`, then `selectV`: different types fail with `Unsupported cond_select: T1 ? T2`; otherwise `a` for bit 1 and `b` for bit 0, for every type.
- Gate: `#boolean(bit, "cond_select bit")`, `#need` and `#chipsForValues` on a and b, `#eqSupported`, then `#matches` of the selected value ("cond_select output"). `select_incircuit` supports neither `Bytes32` nor `JubjubScalar`: `synthErr("Unsupported cond_select: Bytes32 ? Bytes32")` (divergence `f06`), `synthErr("Unsupported cond_select: JubjubScalar == JubjubScalar")`, and for mixed types `synthErr("Unsupported cond_select: T1 == T2")`.

## Assertions

### constrain_bits

- Syntax: `constrainBits(Operand, Int)`: val, bits.
- Off-circuit: `#check(#bitsCheck(..))`, which applies `checkBits(val, bits)` with its three messages.
- Gate: `bits > 255` is `synthErr("constrain_bits: width N exceeds 255")`, since `assigned_to_le_bits` asserts the width; otherwise `#native(val, "constrain_bits")` and `#bits(val, N)`. At `bits = 255` the gate holds for every native although the witness rejects the instruction.
- Checks: `ZKIR-WF` rejects `bits >= 255` with `constrain_bits: excessive bit bound`.

### constrain_eq

- Syntax: `constrainEq(Operand, Operand)`: a, b.
- Off-circuit: `#check(#eqCheck(..))` calls `constrainEqV`: different types fail with `Unsupported constrain_eq: T1 == T2`, different values of one type with `Equality constraint failed` (the crate appends the values). Every type compares by value.
- Gate: `#need` and `#chipsForValues` on both, `#eqSupported`, then `#eqValues(a, b, "constrain_eq fails")`. `constrain_eq_incircuit` has no `JubjubScalar` arm, so that pair is `synthErr("Unsupported constrain_eq: JubjubScalar == JubjubScalar")` (divergence `f07`); `Bytes32` pairs are supported.

### constrain_to_boolean

- Syntax: `constrainToBoolean(Operand)`: val.
- Off-circuit: `#check(#boolCheck(resolveBool(V, M)))`, only the resolver's errors, as the crate's `drop(resolve_operand_bool(..))`.
- Gate: `#boolean(val, "constrain_to_boolean")` (the crate converts to an `AssignedBit`).

## Control

### copy

- Syntax: `copy(Operand, String)`: val, output.
- Off-circuit: `#put(O, resolve(A, M))`; any type, the only error is `variable not found`.
- Gate: `#eqValues(val, output, "copy output")`; no chip, the crate re-inserts the same cell.

## Transcript

### impact

- Syntax: `impact(Operand, Operands)`: guard, inputs.
- Emission: special. The rule appends `guardGate(G)` and one `piGate(N + i, G, X_i)` per input, `N` being `<piIdx>`, which it advances by the input count, so positions never depend on how far the witness got; an empty `impact` still emits its `guardGate`.
- Off-circuit: `#impact` resolves the guard with `resolveBool`. A false guard appends one zero per input to `<pi>` and records `skipSome(n)` in `<skips>`. A true guard pushes each input in turn (`#impactPush`, `#impactOne`: native or `cannot convert T to Native`, appended to `<pi>`, `<pubInIdx>` advanced; a failure leaves earlier pushes in place), records `skipNone()`, and `#impactCheck` compares the pushed values with `public_transcript_inputs`, failing with `Public transcript input mismatch for input i` (the crate appends both values) when the transcript is shorter or differs.
- Gate: `guardGate(G)` is `#boolean(guard, "impact guard")`. `piGate(i, G, X)` is the same, then `#native(input, "impact")`, then `#piEq`: `pi[i]` equals `X` when the guard is 1 and 0 otherwise (`violated("public input i differs from the guarded value")`; `unknown("public input vector incomplete: witness stopped before index i")` when `<pi>` is shorter), the crate's `select(guard, x, 0)` and `pi_push`. The guard is not coupled to those of `public_input` (`f03`); the empty impact is `k06`.

## Curve

### ec_mul

- Syntax: `ecMul(Operand, Operand, String)`: a, scalar, output.
- Off-circuit: `ecMulV`, four arms: `JubjubPoint x JubjubScalar`, `Secp256k1Point x Secp256k1Scalar`, `Secp256r1Point x Secp256r1Scalar`, `Curve25519Point x Curve25519Scalar`, by `ecMul` of `zkir-curves.k`; any other pair fails with `Unsupported EC multiplication: T1 x T2`.
- Gate: standard, `ecMulV` ("ec_mul output").

### ec_mul_generator

- Syntax: `ecMulGenerator(Operand, String)`: scalar, output.
- Off-circuit: `ecMulGeneratorV`: `JubjubScalar` gives `ecMul(#jubjub, #jubjubGenerator, S)`, `Secp256k1Scalar` gives `ecMul(#secp256k1, #secp256k1Generator, S)`, the `I::EcMulGenerator` arm of `preprocess`; every other type, the `Secp256r1` and `Curve25519` scalars included, fails with `Unsupported EcMulGenerator for scalar of type T`.
- Gate: standard, `ecMulGeneratorV` ("ec_mul_generator output"); the crate's `Error::Synthesis` for the unsupported types (`f12` uses `Secp256r1Scalar`).

## Hashing

### hash_to_curve

- Syntax: `hashToCurve(Operands, String)`: inputs, output.
- Off-circuit: `resolveNatives`, then `jubjubPoint(hashToCurve(L))` (`zkir-hash.k`), the crate's `hash_to_curve`.
- Gate: `#needAll` (`#native(.., "hash input")` on every input), `#chipNamed` for `poseidon` and for `jubjub`, `#matches` of the point ("hash_to_curve output").

## Curve

### into_coordinates

- Syntax: `intoCoordinates(Operand, String, String)`: point, x, y.
- Off-circuit: `intoCoordinatesV` through `#put2`: a `JubjubPoint` gives two natives and a `Curve25519Point` two `Curve25519Base` values (either identity gives `(0, 1)`); a `Secp256k1Point` or `Secp256r1Point` gives two base-field values, except the identity, which fails with `Cannot extract coordinates of the Secp256k1 identity` or `... Secp256r1 identity`; other types fail with `Unsupported coordinate extraction of T`.
- Gate: `#need` and `#chipsForValues` on the point; a Weierstrass identity is `violated("into_coordinates of the identity is unsatisfiable")`, an unsupported type `synthErr` with the message above, and the outputs must equal the coordinates ("first output", "second output").

### from_coordinates

- Syntax: `fromCoordinates(Operand, Operand, String)`: x, y, output.
- Off-circuit: `fromCoordinatesV`. Two natives build a `JubjubPoint` through `jubjubFromXY` (`zkir-curves.k`), the crate's decompression path, which uses only the parity of `x`; pairs of `Secp256k1Base`, `Secp256r1Base` or `Curve25519Base` go through `fromXY`. Off the curve or outside the prime-order subgroup (`inSubgroup`) fails with `Cannot build a Jubjub point`, `Cannot build a Secp256k1Point point`, `Cannot build a Secp256r1Point point` or `Cannot build a Curve25519Point point` (the crate appends the coordinates); any other pair with ``Unsupported `from_coordinates` on (T1, T2)``.
- Gate: `#need` and `#chipsForValues` on both; two natives also need the `jubjub` chip (`#fromCoordsChip`), which `used_chips` does not enable for this instruction (`k01b`). `#fromCoordsGate` then pins the exact `(x, y)` as `point_from_coordinates` does: `violated("from_coordinates: (x, y) is not on the curve")`, `violated("from_coordinates: point is not in the prime-order subgroup")` (`f11`), the unsupported-pair `synthErr`, or `#matches` of the point ("from_coordinates output"). The parity-only path is divergence K1 (`k01`).

## Bytes

### into_bytes32

- Syntax: `intoBytes32(Operand, String)`: input, output.
- Off-circuit: `intoBytes32V`, seven arms: `Native` and the six foreign field types become `intToBytes32(X)`, little-endian; points, `Bytes32` and `JubjubScalar` fail with `Unsupported into_bytes32 for T`.
- Gate: standard, `intoBytes32V` ("into_bytes32 output").

### from_bytes32

- Syntax: `fromBytes32(Operand, IrType, String)`: bytes, type (the crate's `val_t`), output.
- Off-circuit: `asBytes32`, then `fromBytes32V(T, B)`: `Native` and the six foreign field types take the little-endian integer reduced modulo the target field, so non-canonical encodings are accepted; other targets fail with `Unsupported from_bytes32 for type T`.
- Gate: `#need`, `#chipFor(T, Chips)` for the target type, `#matches` of the reduced value ("from_bytes32 output", prefixed to the resolver's or the function's message for a non-`Bytes32` register or an unsupported target). `used_chips` initialises nothing for this instruction, so a program whose only foreign value comes from it gives `synthErr("chip not initialised for T")`: divergence `f13`.

### reverse_bytes

- Syntax: `reverseBytes(Operand, String)`: bytes, output.
- Off-circuit: `asBytes32`, then `bytes32(reverseBytes(B))`; the crate reverses in place.
- Gate: `#need`, `#matches` of the reversal ("reverse_bytes output"; `synthErr("reverse_bytes output: cannot convert T to Bytes32")` for another type). No chip.

### bytes32_into_low_high

- Syntax: `bytes32IntoLowHigh(Operand, String, String)`: bytes, low, high.
- Off-circuit: `asBytes32`, then `bytes32IntoLowHighV`: `low` is the first 31 bytes read little-endian modulo `#r`, `high` is byte 31. No further error exists, in the crate either.
- Gate: `#need`; another type is `synthErr("cannot convert T to Bytes32")`; the outputs must equal the decomposition ("first output", "second output"). No chip.

### bytes32_from_low_high

- Syntax: `bytes32FromLowHigh(Operand, Operand, String)`: low, high, output.
- Off-circuit: `bytes32FromLowHighV`. Two natives with `fitsBits(L, 248)` and `fitsBits(H, 8)` give the bytes of `L` with byte 31 replaced by `H`; two natives outside the bounds fail with `Bytes32FromLowHigh: low operand must fit in 31 bytes (be less than 2^248) and high operand must fit in a single byte (be less than 256)`. Any other pair goes through `#lowHighErr`, the order of the crate's arm: `intoBytes32V` on each operand (`Unsupported into_bytes32 for T`), then byte 31 of the low encoding and bytes 1 to 31 of the high encoding must be zero, with the same bound message.
- Gate: `#need` and `#chipsForValues` on low, `#native(high, "bytes32_from_low_high high")`, `#lowHighBounds` (`violated("bytes32_from_low_high: low operand uses byte 31")`, the crate's `assert_equal_to_fixed(bytes_low[31], 0)`, or `violated("bytes32_from_low_high: high operand is not a byte")`), then `#matches` of the composition ("bytes32_from_low_high output"). A foreign-field high operand passes off-circuit and fails in circuit: divergence `f08`.
- Checks: the two bounds, off-circuit.

## Arithmetic

### div_mod_power_of_two

- Syntax: `divModPowerOfTwo(Operand, Int, Ids)`: val, bits, outputs (quotient, remainder).
- Off-circuit: an output list of length other than 2 fails with `DivModPowerOfTwo requires exactly 2 outputs`; `bits > 248` (`#frBytesStored *Int 8`) with `Excessive bit count`; otherwise `#divMod` resolves the value, native or `cannot convert T to Native`, and stores `highBits(X, N)` and `lowBits(X, N)` (`zkir-field.k`), the crate's `val >> bits` and `val & ((1 << bits) - 1)`.
- Gate: an output list that is not a pair is `synthErr("Unexpected output length of DivModPowerOfTwo instruction")`; otherwise `#native(val, "div_mod_power_of_two")` and `#matches` of quotient and remainder ("div_mod quotient", "div_mod remainder"). No chip.
- Checks: arity and bit count, before the operand is resolved; `ZKIR-WF` checks the same with `div_mod_power_of_two requires exactly 2 outputs` and `div_mod_power_of_two: excessive bit count`.

### reconstitute_field

- Syntax: `reconstituteField(Operand, Operand, Int, String)`: divisor, modulus, bits, output; `reads` gives the modulus first, the order of `preprocess`.
- Off-circuit: `bits > 248` fails with `Excessive bit count` before any operand is read. `#recon` resolves the modulus and applies `checkBits(modulus, bits)`, `#recon3` resolves the divisor and applies `checkBits(divisor, 255 - bits)`, `#recon4` fails with `Reconstituted element overflows field` when `(divisor << bits) + modulus > r - 1` and otherwise stores the sum modulo `#r`.
- Gate: `#native` on both ("reconstitute_field"), `#bits(divisor, 255 - N)`, `#bits(modulus, N)` (the crate's two `assert_lower_than_fixed` calls), then `#matches` of `#recOf`, the sum modulo `r` ("reconstitute_field output"). The gate has no overflow check: after the witness rejects an overflow the output register is absent and the verdict is `unknown` (`f01`); a register holding the wrapped sum would satisfy it.
- Checks: bit count and both bounds; `ZKIR-WF` rejects `bits > 248` with `reconstitute_field: excessive bit count` (`f10`, `bits = 256`).

## Hashing

### transient_hash

- Syntax: `transientHash(Operands, String)`: inputs, output.
- Off-circuit: `resolveNatives`, then `native(poseidonHash(L))` (`zkir-hash.k`), the crate's `transient_hash`.
- Gate: `#needAll`, `#chipNamed("poseidon", ..)`, `#matches` of the hash ("transient_hash output").

### persistent_hash

- Syntax: `persistentHash(Alignment, Operands, String)`: alignment, inputs, output; alignments are described in [03-program-model.md](03-program-model.md).
- Off-circuit: `resolveNatives`, `alignedBytes(Al, L)` (`zkir-ops.k`, the crate's `parse_field_repr` and `binary_repr`), then `sha256Bytes` (`zkir-hash.k`) into a `bytes32`. Every failure of the alignment parser is `Inputs did not match alignment` (the crate appends inputs and alignment): too few elements for a `field` or `bytes` atom, a chunk that does not fit its width, a `compress` atom, an `option` selector that is not a 16-bit integer or exceeds the alternatives, or non-zero padding after the chosen alternative. Surplus trailing elements are ignored.
- Gate: `#needAll`, `#chipNamed("sha2_256", ..)`, `#matches` of the hash of `alignedBytesCircuit(Al, L)` ("persistent_hash output"). The in-circuit decoder (`fab_decode_to_bytes`) differs: an `option` segment gives `synthErr("persistent_hash output: synthesis: in-circuit decoding of alignment options is not yet implemented")` (`k07`), a `compress` atom `synthErr("persistent_hash output: synthesis: Cannot decode compressed value from field elements")`.

### keccak256

- Syntax: `keccak256(Alignment, Operands, String)`: alignment, inputs, output.
- Off-circuit: as `persistent_hash` with `keccak256Bytes` (`zkir-hash.k`) in place of SHA-256 (`#stdHash2(.., Al, "keccak256")`); the same messages.
- Gate: `#needAll`, `#chipNamed("keccak_256", ..)`, `#matches` of the Keccak-256 digest of `alignedBytesCircuit(Al, L)` ("keccak256 output"), with the same two synthesis errors prefixed `keccak256 output: `.

## Comparison

### test_eq

- Syntax: `testEq(Operand, Operand, String)`: a, b, output.
- Off-circuit: `testEqV`: `eqDispatch` has twelve arms, one per type except `JubjubScalar`, comparing by value into `native(1)` or `native(0)`; a `JubjubScalar` or mixed pair fails with `Unsupported test_eq: T1 == T2`.
- Gate: standard, `testEqV` ("test_eq output"), with `#eqSupported` before `#matches`: mixed types and `JubjubScalar` pairs are `synthErr("Unsupported test_eq: T1 == T2")`; `Bytes32` pairs are supported by `test_eq_incircuit`.

## Arithmetic

### add

- Syntax: `add(Operand, Operand, String)`: a, b, output.
- Off-circuit: `addV`, eleven arms: `Native` (`fadd` modulo `#r`), the four point types (`ecAdd`), the six foreign field types (`fadd` modulo their modulus); `Bytes32`, `JubjubScalar` and mixed pairs fail with `Unsupported addition: T1 + T2`.
- Gate: standard, `addV` ("add output"), the dispatch of `add_incircuit`.

### mul

- Syntax: `mul(Operand, Operand, String)`: a, b, output.
- Off-circuit: `mulV`, seven arms: `Native` and the six foreign field types by `fmul`; points and every other pair fail with `Unsupported multiplication: T1 x T2`.
- Gate: standard, `mulV` ("mul output").

### neg

- Syntax: `neg(Operand, String)`: a, output.
- Off-circuit: `negV`, eleven arms: `Native`, the four point types (`ecNeg`), the six foreign field types (`fneg`); `Bytes32` and `JubjubScalar` fail with `Unsupported negation of T`.
- Gate: standard, `negV` ("neg output").

### inv

- Syntax: `inv(Operand, String)`: a, output.
- Off-circuit: `invV`: `native(0)` fails with `cannot invert zero of type Native`, other natives give `finv` modulo `#r`; the six foreign field types go through `#invField`, failing at zero with `cannot invert zero of type T`; points, `Bytes32` and `JubjubScalar` fail with `Unsupported inversion of T`.
- Gate: standard, `invV` ("inv output (a * inv = 1)"), with `#invNonZero` before `#matches`: a zero field element is `violated("inv: a * inv = 1 has no solution at zero")`, a circuit that exists but is unsatisfiable, whatever the output holds.

## Comparison

### not

- Syntax: `not(Operand, String)`: a, output.
- Off-circuit: `resolveBool`, then `#notV` stores `native(1)` for false and `native(0)` for true.
- Gate: `#boolean(a, "not")` (the crate converts to an `AssignedBit`), then `#matches` of `1 - a` ("not output"). No chip.

### less_than

- Syntax: `lessThan(Operand, Operand, Int, String)`: a, b, bits, output.
- Off-circuit: `#lt` resolves `a` and applies `checkBits(a, bits)`, `#lt3` does the same for `b`, `#lt4` stores `native(1)` when `a < b` as integers, else `native(0)`.
- Gate: the chip pads the width to `#ltBits(N) = maxInt(N + N mod 2, 4)` (the crate's `std.lower_than`, `u32::max(bits + bits % 2, 4)`) and `bounded_of_element` asserts it is at most 253, so `#ltBits(N) > 253` is `synthErr("less_than: padded bound P exceeds MAX_BOUND_IN_BITS = 253")` before any register is read; otherwise `#native` on both ("less_than"), `#bits` on both with the padded width, `#matches` of the comparison ("less_than output"). An odd width admits one more bit in circuit (`f04`); widths 253 and 254 pass off-circuit but cannot be built (K5, `k05`).
- Checks: the bound on both operands; `ZKIR-WF` rejects `bits >= 255` with `less_than: excessive bit bound`.

## Curve

### jubjub_scalar_from_native

- Syntax: `jubjubScalarFromNative(Operand, String)`: native, output.
- Off-circuit: `asNative`, then `#jjScalar` stores `jubjubScalar(X modInt #rJ)`, the reduction of `native_to_jubjub_scalar`.
- Gate: `#native(input, "jubjub_scalar_from_native")`, `#chipFor(jubjubScalar(), Chips)`, `#matches` of the reduced scalar ("jubjub_scalar_from_native output"). `used_chips` does not enable the chip for this instruction, so a program whose only Jubjub value is minted here gives `synthErr("chip not initialised for JubjubScalar")`: divergence `k04`.

## Transcript

### public_input

- Syntax: `publicInput(Guard, IrType, String)`: guard (`noGuard()` when the JSON member is absent, `guard(Operand)` otherwise), type, output.
- Off-circuit: `#guardActive` treats no guard as active and resolves one with `resolveBool`. An inactive guard stores `defaultValue(T)` (`zkir-values.k`, the crate's `IrValue::default`) without touching the transcript. An active one takes `encodedLen(T)` elements of `public_transcript_outputs` at `<pubOutIdx>`, decodes them with `decodeStrict` (errors in [04-values-and-encoding.md](04-values-and-encoding.md)) and advances the cursor. A short transcript, a slice panic in the crate, sets `panic("range end index out of range: public transcript outputs too short")`, or in generation mode stores the default and records `needPubOut(T)`.
- Gate: `#need` on the output, `#chipFor(T, Chips)`, `#typed`: the register's type must be `T` (`violated("register has type U, declared T")`), as `assign_incircuit`. The guard plays no part.
- Checks: the transcript length, a panic rather than an error.

### private_input

- Syntax: `privateInput(Guard, IrType, String)`: as `public_input`.
- Off-circuit: the same `#input` rules over `private_transcript` and `<privIdx>`; a short transcript sets `panic("range end index out of range: private transcript too short")` or records `needPriv(T)` in generation mode (K2, `k02`).
- Gate: as `public_input`: register present, chip of `T`, type equal to `T`.
- Checks: the transcript length, a panic.

### output

- Syntax: `output(Operands)`: vals.
- Emission: special. The rule appends `outputGate(Vs, Ts)`, `Ts` being the signature in `<outTypes>`.
- Off-circuit: a list whose length differs from the signature fails with `Output: signature declares N return values but instruction has M`. Then `#output` resolves each operand in order and compares its type with the declared one, failing with `Output position i: signature declares T but operand has runtime type U`; matching values are appended to `<outputs>`, which the communications commitment re-encodes after the run.
- Gate: `outputGate(Vs, Ts)`: arity and per-position mismatches are `synthErr` with the same two texts (the crate's `Error::Synthesis`), an absent register is `unknown`, otherwise the gate holds.
- Checks: arity, then types; `ZKIR-WF` checks the arity with `output: signature declares N return values but instruction has M`.

## Corpus programs by instruction

The table lists the programs under `experiments/zkir-k/corpus/` whose `"op"` members include each instruction. `ledger` is `ledger9-92e8bdd3-tests/`, `handmade` and `negative` are `handmade/` and `handmade-negative/`, `divergence` is `divergence/`, `micro-dao` stands for the six `midnight-zkir-2ffe2d1-precompiles/micro-dao__*.zkir` programs (`advance`, `buyIn`, `cashOut`, `setTopic`, `voteCommit`, `voteReveal`), and `curve_*` for `curve_jubjub`, `curve_secp256k1`, `curve_secp256r1` and `curve_curve25519`. Programs of the same name under `midnight-zkir-2ffe2d1-tests/` use the same ops on the extension surface and are not repeated, except that `test_reverse_bytes_proof` there uses the extension's `reverse` in place of `reverse_bytes` (see [09-extension-surface.md](09-extension-surface.md)).

| op | ledger | handmade, negative | divergence | micro-dao |
|---|---|---|---|---|
| `encode` | `test_ec_proof` | `curve_*`, `native_bytes`, `transcripts`, `transcripts_guard_off` | | |
| `assert` | 19 uses in 13 programs, among them `test_minimal_proof`, `test_jubjub_point_ops`, `test_keygen_and_serialize_eq`, `test_immediate_add_and_cond_select` | `curve_*`, `native_bytes`; negative `immediate_out_of_range` | `f02_assert_non_boolean` | all six |
| `cond_select` | `test_immediate_add_and_cond_select`, `test_immediate_with_public_inputs`, `test_impact_guarded_off_zeroes_public_input`, `test_jubjub_point_cond_select_fails_when_bit_zero`, `test_jubjub_point_ops` | `curve_*`, `native_bytes` | `f06_cond_select_bytes32` | all six |
| `constrain_bits` | `test_immediate_with_public_inputs`, `test_impact_guarded_off_zeroes_public_input` | `native_bytes`; negative `excessive_bits` | | all six |
| `constrain_eq` | 63 uses in 24 programs, among them `test_jubjub_point_constrain_eq_fails_on_unequal`, `test_curve25519_point_constrain_eq_fails_on_unequal`, `test_secp256r1_point_constrain_eq_fails_on_unequal`, `test_immediate_values` | `curve_*`, `native_bytes` | `f07_constrain_eq_jubjub_scalar` | |
| `constrain_to_boolean` | | `native_bytes` | | `voteCommit`, `voteReveal` |
| `copy` | `test_immediate_copy`, `native_via_copy`, `multi_output_native_pair`, `test_invalid_operand_odd_length_hex` | `native_bytes`; negative `reassignment` | | `buyIn`, `cashOut`, `voteCommit`, `voteReveal` |
| `impact` | `test_hash_proof`, `test_immediate_with_public_inputs`, `test_impact_guarded_off_zeroes_public_input` | `transcripts`, `transcripts_guard_off` | `k06_empty_impact_guard` | all six |
| `ec_mul` | `test_ec_proof`, `test_secp256r1_ec_mul_proof`, `test_curve25519_ec_mul_proof` | `curve_*` | | |
| `ec_mul_generator` | `test_ec_proof` | `curve_jubjub`, `curve_secp256k1`, `native_bytes` | `f12_ec_mul_generator_p256`, `k04_jubjub_scalar_from_native_chip` | |
| `hash_to_curve` | `test_htc_proof` | `transient_hash`, `native_bytes` | | |
| `into_coordinates` | `test_coordinates_proof`, `test_secp256r1_coordinates_proof`, `test_curve25519_coordinates_proof` | `curve_*` | `k01_jubjub_from_coordinates_parity_only` | |
| `from_coordinates` | `test_coordinates_proof`, `test_secp256r1_coordinates_proof`, `test_curve25519_coordinates_proof` | `curve_*` | `k01_jubjub_from_coordinates_parity_only`, `k01b_jubjub_from_coordinates_no_chip`, `f11_curve25519_torsion_point` | |
| `into_bytes32` | `test_bytes32_proof`, `test_secp256r1_bytes32_proof`, `test_curve25519_bytes32_proof` | `native_bytes`, `curve_secp256k1`, `curve_secp256r1`, `curve_curve25519` | | |
| `from_bytes32` | `test_bytes32_proof`, `test_secp256r1_bytes32_proof`, `test_curve25519_bytes32_proof` | `native_bytes`, `curve_secp256k1`, `curve_secp256r1`, `curve_curve25519` | `f13_chip_gating_from_bytes32` | |
| `reverse_bytes` | `test_reverse_bytes_proof` | `native_bytes` | `k03_bytes32_input_assertion_panics` | |
| `bytes32_into_low_high` | `test_bytes32_low_high_proof` | `native_bytes` | | all six |
| `bytes32_from_low_high` | `test_bytes32_low_high_proof` | `native_bytes` | `f08_bytes32_from_low_high_foreign_high` | |
| `div_mod_power_of_two` | `test_divmod_proof` | `native_bytes`; negative `divmod_outputs` | | `buyIn`, `cashOut` |
| `reconstitute_field` | `test_divmod_proof` | `native_bytes` | `f01_reconstitute_overflow`, `f10_reconstitute_bits_256` | |
| `transient_hash` | `test_hash_proof` | `transient_hash`, `native_bytes` | | `buyIn`, `cashOut`, `voteCommit`, `voteReveal` |
| `persistent_hash` | `test_std_hashes_proof` | `std_hashes` | `k07_alignment_option_offcircuit` | all six |
| `keccak256` | `test_std_hashes_proof` | `std_hashes` | | |
| `test_eq` | `test_jubjub_point_test_eq_unequal`, `test_immediate_add_and_cond_select`, `test_jubjub_point_ops`, `test_curve25519_proof`, `test_secp256k1_proof`, `test_secp256r1_proof` | `curve_*`, `native_bytes`, `transcripts`, `transcripts_guard_off` | | all six |
| `add` | `test_immediate_add_and_cond_select`, `test_immediate_values`, `test_ec_proof`, `test_secp256k1_proof`, `test_secp256r1_proof`, `test_curve25519_proof` | `curve_*`, `native_bytes`; negative `reassignment`, `undefined_variable` | `f03_guard_uncoupled`, `f05_noncanonical_foreign_limbs`, `f13_chip_gating_from_bytes32`, `k02_transcript_too_short_panics` | `buyIn`, `cashOut` |
| `mul` | `test_native_inv_proof`, `test_secp256k1_proof`, `test_secp256r1_proof`, `test_curve25519_proof` | `native_bytes`, `curve_secp256k1`, `curve_secp256r1`, `curve_curve25519` | | |
| `neg` | `test_jubjub_point_ops`, `test_secp256k1_proof`, `test_secp256r1_proof`, `test_curve25519_proof` | `curve_*`, `native_bytes` | | `cashOut` |
| `inv` | `test_native_inv_proof`, `test_secp256k1_proof`, `test_secp256r1_proof`, `test_curve25519_proof` | `native_bytes`, `curve_secp256k1`, `curve_secp256r1`, `curve_curve25519` | | |
| `not` | `test_jubjub_point_test_eq_unequal` | `native_bytes`, `transcripts`, `transcripts_guard_off` | | |
| `less_than` | | `native_bytes` | `f04_less_than_odd_bits`, `k05_less_than_253_bits_keygen` | `buyIn`, `cashOut`, `voteCommit`, `voteReveal` |
| `public_input` | | `transcripts`, `transcripts_guard_off` | `f03_guard_uncoupled` | all six |
| `private_input` | 16 programs, among them `test_bytes32_proof`, `test_ec_proof`, `test_divmod_proof`, `test_jubjub_point_ops`, `test_reverse_bytes_proof` | `transcripts`, `transcripts_guard_off` | `k02_transcript_too_short_panics` | all six |
| `output` | `native_identity`, `native_via_copy`, `multi_output_native_pair`, `output_arity_mismatch`, `output_operand_type_mismatch` | `native_bytes`, `transcripts`, `transcripts_guard_off` | `f05_noncanonical_foreign_limbs` | `cashOut` |

From the repository root, this command prints every program that uses a given instruction (seven files for `less_than`):

```sh
grep -rlE '"op"\s*:\s*"less_than"' experiments/zkir-k/corpus/
```

`handmade/manifest.json` has a `programs` array whose entries carry `file`, `ops` and a `test_preimage` object. The commands below write the preimage of `native_bytes.zkir` to `/tmp/native_bytes_preimage.json` and run the program on it:

```sh
uv run --group zkir-k python -c 'import json
m = json.load(open("experiments/zkir-k/corpus/handmade/manifest.json"))
p = next(p for p in m["programs"] if p["file"] == "native_bytes.zkir")
json.dump(p["test_preimage"], open("/tmp/native_bytes_preimage.json", "w"))'
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py \
  experiments/zkir-k/corpus/handmade/native_bytes.zkir /tmp/native_bytes_preimage.json
```

The runner prints one JSON object: `status` (`ok` here), `memory` (the `variant`, `type` and `encoded` elements of every register), `pis`, `constraints` (34 here), `verdicts` (the count, also 34), `all_verdicts` (one `[outcome, message, gate]` triple per gate), `violations` (the triples whose outcome is not `holds`, empty here) and `outputs`. [11-tooling-reference.md](11-tooling-reference.md) describes its options and [12-oracles-and-differential-testing.md](12-oracles-and-differential-testing.md) the comparison of the same programs with the oracle.
