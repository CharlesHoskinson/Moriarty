# Values and encoding

Every register of a ZKIR v3 run holds a typed value, and every value crosses the boundary between program and proof as a vector of native field elements: raw inputs, transcript entries, the `encode` instruction and the communications commitment share one encoding. This chapter covers the `Value` sort of `zkir-values.k` (module `ZKIR-VALUES`), `encodeValue`, `decodeValue`, `decodeStrict`, and the rules of `zkir-vm.k` through which values enter and leave a run. The Rust reference is `zkir-v3/src/ir_instructions/encode.rs` at midnight-ledger 92e8bdd3, whose `encode_offcircuit` and `decode_offcircuit` delegate every layout except Bytes32 to the `Instantiable` trait of midnight-circuits 7.2.4 (`as_public_input`, `from_public_input`). A raw element is an integer in [0, r), where r is the order of the BLS12-381 scalar field (`#blsScalarModulus` in `zkir-syntax.k`, `#r` in `zkir-field.k`).

## The Value sort

`zkir-values.k` declares thirteen constructors, one per `IrType` of `zkir-syntax.k` and per variant of the Rust `IrValue`. Each carries a K `Int`, a K `Bytes` or a `Point` of `zkir-curves.k` (`Point ::= pt(Int, Int) | inf()`). Short Weierstrass values (secp256k1, secp256r1) use `inf()`; twisted Edwards values (Jubjub, Curve25519) never carry it, because their identity is `pt(0, 1)`.

| Constructor | Symbol | Payload | `typeOf` | `typeName` | `defaultValue` |
|---|---|---|---|---|---|
| `native(Int)` | `nativeV` | element of [0, r) | `native()` | `Native` | `native(0)` |
| `bytes32(Bytes)` | `bytes32V` | 32 bytes | `bytes32()` | `Bytes32` | 32 zero bytes |
| `jubjubPoint(Point)` | `jubjubPointV` | affine (x, y) over r | `jubjubPoint()` | `JubjubPoint` | `pt(0, 1)` |
| `jubjubScalar(Int)` | `jubjubScalarV` | element of [0, rJ) | `jubjubScalar()` | `JubjubScalar` | `jubjubScalar(0)` |
| `secp256k1Point(Point)` | `secp256k1PointV` | affine over k256P or `inf()` | `secp256k1Point()` | `Secp256k1Point` | `inf()` |
| `secp256k1Base(Int)` | `secp256k1BaseV` | element of [0, k256P) | `secp256k1Base()` | `Secp256k1Base` | 0 |
| `secp256k1Scalar(Int)` | `secp256k1ScalarV` | element of [0, k256N) | `secp256k1Scalar()` | `Secp256k1Scalar` | 0 |
| `secp256r1Point(Point)` | `secp256r1PointV` | affine over p256P or `inf()` | `secp256r1Point()` | `Secp256r1Point` | `inf()` |
| `secp256r1Base(Int)` | `secp256r1BaseV` | element of [0, p256P) | `secp256r1Base()` | `Secp256r1Base` | 0 |
| `secp256r1Scalar(Int)` | `secp256r1ScalarV` | element of [0, p256N) | `secp256r1Scalar()` | `Secp256r1Scalar` | 0 |
| `curve25519Point(Point)` | `curve25519PointV` | affine over c25519P | `curve25519Point()` | `Curve25519Point` | `pt(0, 1)` |
| `curve25519Base(Int)` | `curve25519BaseV` | element of [0, c25519P) | `curve25519Base()` | `Curve25519Base` | 0 |
| `curve25519Scalar(Int)` | `curve25519ScalarV` | element of [0, c25519L) | `curve25519Scalar()` | `Curve25519Scalar` | 0 |

The payload column is the invariant the VM maintains, not a sort constraint: K accepts any `Int` or `Bytes` here, and only the decoders and instruction rules keep values in range. `typeOf` maps a value to its type, `typeName` prints the Rust variant name, as the oracle does, and `tn(V)` in `zkir-ops.k` abbreviates `typeName(typeOf(V))`. `defaultValue` is `IrValue::default`, bound by a guarded `public_input` or `private_input` whose guard is false. The moduli are the constants of `zkir-field.k`; see 05-fields-curves-and-hashes.md.

The extension surface adds `boolV(Bool)`, `byteV(Int)` and `bytesV(Bytes)` in `zkir-ext.k` (module `ZKIR-EXT-VALUES`). A 32-byte string stays a `bytes32(_)` there (`mkBytes`), and `tools/zkir_kast.py` maps `Bytes<32>` to `Bytes32` before any extension type, so every base Bytes32 rule applies under `--ext`. See 09-extension-surface.md.

## Encoding to raw field elements

`encodeValue(Value)` is a total function returning a K `List` of integers. For every value the VM constructs, its length is `encodedLen(typeOf(V))` of `zkir-syntax.k` (`IrType::encoded_len` in `ir_types.rs`). The one exception, `bytes32(B)` with `lengthBytes(B) =/=Int 32`, returns `.List`; that arm exists only to keep the function total.

| Type | Elements | Layout of `encodeValue` |
|---|---|---|
| Native | 1 | `[x]` |
| Bytes32 | 2 | `[int_le(bytes[0..31]), bytes[31]]` |
| JubjubPoint | 2 | `[x, y]` |
| JubjubScalar | 1 | `[s]` |
| Secp256k1Base, Secp256k1Scalar, Secp256r1Base, Secp256r1Scalar, Curve25519Base | 2 | `encField64(x, p)`: two limb groups of the shifted value |
| Curve25519Scalar | 2 | `encForeign(x, c25519L, 51, 5)` |
| Secp256k1Point, Secp256r1Point | 5 | `encField64(x) ++ encField64(y) ++ [is_identity]` |
| Curve25519Point | 4 | `encField64(x) ++ encField64(y)` |

Native encoding is the identity function: `encodeValue(native(X))` is `[X]` (`AssignedNative::as_public_input`). Bytes32 is the hand-written arm of `encode_offcircuit`. The first 31 bytes, read little-endian, form one element (248 bits, always below r), and byte 31 forms a second:

```k
rule encodeValue(bytes32(B)) => ListItem(Bytes2Int(substrBytes(B, 0, 31), LE, Unsigned)) ListItem(B[31]) requires lengthBytes(B) ==Int 32
```

A Jubjub point is its affine pair (`AssignedNativePoint::as_public_input`). A Jubjub scalar is one element. `AssignedScalarOfNativeCurve::as_public_input` packs its 252 bits (`NUM_BITS_SUBGROUP` of `JubjubExtended`) into 254-bit batches, which here means a single batch, and the crate asserts `encoded.len() == 1`.

Foreign field elements follow `AssignedField::as_public_input` in `field/foreign/field_chip.rs`: subtract one ("the unique-zero representation"), split the result into `NB_LIMBS` little-endian limbs of `LOG2_BASE` bits, and pack `F::CAPACITY / LOG2_BASE` limbs into each element. For the BLS12-381 scalar field, `F::CAPACITY` is 254. `encForeign` and `#groupLimbs` do the same:

```k
rule encForeign(X, P, L, NB) => #groupLimbs(fsub(X, 1, P), L, NB, 254 /Int L) requires L >Int 0 andBool P >Int 0
```

The parameters are those of `MultiEmulationParams` in `field/foreign/params.rs` for the BLS12-381 native field:

| Emulated field | `LOG2_BASE` | `NB_LIMBS` | Limbs per element | Element 0 | Element 1 |
|---|---|---|---|---|---|
| secp256k1 base and scalar, secp256r1 base and scalar, Curve25519 base | 64 | 4 | 3 | low 192 bits of x - 1 mod p | high 64 bits |
| Curve25519 scalar | 51 | 5 | 4 | low 204 bits of x - 1 mod L | high 51 bits |

`encField64(X, P)` is `encForeign(X, P, 64, 4)`. Because of the shift, 1 encodes as `[0, 0]` and 0 as the limbs of p - 1. The shift applies to every foreign type, base and scalar alike, and never to Native or JubjubScalar.

A Weierstrass point is `encWPoint(Q, p)`: both encoded coordinates, then an identity flag. `AssignedForeignPoint::as_public_input` takes the identity's coordinates as (0, 0), so the identity encodes as the limbs of p - 1 twice, then 1. A Curve25519 point (`encEPointForeign`) is the two coordinates without a flag. The `inf()` arms of `encodeValue(jubjubPoint(inf()))` and `encEPointForeign(inf(), P)` exist only to keep the functions total.

In the extension, `zkir-ext.k` encodes Bool as `[0]` or `[1]`, Byte as `[b]`, and Bytes<n> as ceil(n / 31) elements of 31 little-endian bytes (`#encChunks`; `BYTES_PER_FIELD_ELEMENT` in the crate); for n = 32 this is the Bytes32 layout.

## Decoding from raw

`decodeValue(List, IrType)` returns a `DecodeResult`:

```k
syntax DecodeResult ::= decOk(Value) | decErr(String) | decPanic(String)
```

`decErr` is a Rust `Err` and becomes the run status `error`. `decPanic` marks a place where the crate aborts and becomes the status `panic`. The VM passes exactly `encodedLen(T)` elements; any other count is `decErr("Failed to decode as T")`.

- Native: the element must lie in [0, r), otherwise `decErr("is not a canonical field element")`. `zkir_run.py` (`fr` in `preimage_term`) and the oracle reject such an integer before a run, as `{n} is not a canonical field element` (K emits the suffix only), so the arm is unreachable through the runners; `unit_values.py` evaluates it as a ZKIR-TEST term.
- Bytes32: the low element must fit in 248 bits and the high in 8 (`fitsBits`), otherwise `decPanic("assertion failed: Bytes32 low element uses byte 31 or high element exceeds a byte")`, because the crate at 92e8bdd3 checks the same conditions with `assert_eq!` and aborts on a well-formed preimage (divergence K3, see 13-known-divergences.md). At 2ffe2d1 there is no assertion. `decode_offcircuit` routes every `Bytes(n)`, n = 32 included, through `decode_bytes`, which returns `None` under the same conditions: an ordinary `Failed to decode` error. Since `Bytes<32>` stays `bytes32()` under `--ext`, a `Bytes<32>` raw input, transcript entry or `load_constant` immediate whose first element is at least 2^248, or whose second is at least 256, is a `panic` in K and an `error` in the extension crate. No recorded extension comparison reaches this case. For every other n, `#decChunks` and `decode_bytes` agree on an error.
- JubjubPoint: `#decJubjub(jubjubFromXY(X, Y))` decompresses from y and the parity of x, as `from_xy` of midnight-circuits does, then requires `inSubgroup(#jubjub, Q)`. Either failure is `decErr("Failed to decode as JubjubPoint")`. The same parity-only rule is divergence K1 on `from_coordinates` (see 13-known-divergences.md): an x of the right parity but the wrong value decodes to the real point on the base surface, and `#canonical` rejects it on the extension because `encodeValue` writes the recovered x.
- JubjubScalar: the element must fit in 254 bits with its low 252 bits below rJ. The result is `jubjubScalar(lowBits(F, 252))`, the `take(NUM_BITS_SUBGROUP)` of `AssignedScalarOfNativeCurve::from_public_input`. Bits 252 and 253 are dropped, so on the base surface such an element decodes to a value whose re-encoding differs from the input.
- Foreign fields: `decForeign(Fs, p, L, NB)` returns the element or -1. It rejects a wrong element count, an element not below 2^(L * limbs per element), and non-zero limbs beyond `NB_LIMBS` (`fitsBits(Acc, L * NB)`). Then it adds one and reduces modulo p (`fadd(Acc, 1, P)`), as `from_public_input` does through `bigint_to_fe`; a reconstructed integer at or above p is accepted and reduced (case `f05_noncanonical_foreign_limbs` in `tools/divergence_tests.py` encodes p + 5). `#decField` turns -1 into `decErr("Failed to decode as " +String typeName(T))`.
- Weierstrass points: `#decWPoint` needs five elements. A final element equal to 1 is the identity whatever the coordinate limbs say (`AssignedForeignPoint::from_public_input`). Otherwise elements 0 and 1 decode x, 2 and 3 decode y, `fromXY` checks the curve equation exactly, and `inSubgroup` is true on these prime-order curves.
- Curve25519 points: four elements, x from 0 and 1, y from 2 and 3, `fromXY(#curve25519, X, Y)` for the exact curve equation, then `inSubgroup` for the order-L check (cofactor 8).

The extension decoders in `zkir-ext.k` accept Bool only as 0 or 1, Byte only below 256, and Bytes<n> only when the unused high bytes of every chunk are zero (`#decChunks`; `decode_bytes` in the crate).

### Strict decoding

`decodeStrict(L, T, Strict)` is `decodeValue` when `Strict` is false; otherwise `#canonical` re-encodes a successful result and demands the original elements:

```k
rule #canonical(decOk(V), L, T) => decErr("The encoded value of type " +String typeName(T) +String " is not in canonical form") requires encodeValue(V) =/=K L
```

This is the check `decode_offcircuit` at 2ffe2d1 performs on every decoded value; 92e8bdd3 does not check. `<strictDecode>` is false by default and set to true by the `job` and `genJob` rules of module `ZKIR-EXT` (priority 30), so the extension decodes strictly everywhere. `load_constant` calls `decodeStrict(#immInts(Es), T, true)` unconditionally. Strict decoding rejects the four non-canonical forms above: Jubjub scalars with bit 252 or 253 set, foreign integers at or above the modulus, identity encodings with arbitrary coordinate limbs, and Jubjub x values of the right parity but the wrong value. `decErr` and `decPanic` pass through `#canonical` unchanged.

## Inputs, transcripts, public inputs and outputs

A run starts from `preimage(inputs, binding_input, comm, private_transcript, public_transcript_inputs, public_transcript_outputs)` in `zkir-vm.k`; every integer in it is a raw element in [0, r).

`#loadInputs` turns raw inputs into memory. For each `typedId(N, T)` of the signature it slices `encodedLen(T)` elements at the cursor, decodes them with `decodeStrict` under `<strictDecode>`, and binds `N` with `#put` before moving on, so a failure leaves the earlier inputs in `<mem>`, as `preprocess` does. Too few elements give `error("Not enough raw inputs: ran out at index i while decoding N")`, and leftover elements give `error("Expected n raw inputs, received m")`; Rust prints the identifier as `Identifier("N")`.

`<pi>` is the public-input vector of the proof and holds raw integers, never values. `#seedPi` places the binding input at position 0 and, under the commitment flag, the commitment at position 1. Each active `impact` then pushes its operands one at a time (`#impactOne`), accepting only `native` values and failing with "cannot convert T to Native" otherwise. The constraint side records the same positions as `bindGate`, `commGate` and `piGate`; see 08-constraints-and-verdicts.md.

`public_input` and `private_input` read the transcripts. `#input` evaluates the guard (`#guardActive`): a false guard binds `defaultValue(T)` and consumes nothing; otherwise `#decodeSlice` takes `encodedLen(T)` elements from `public_transcript_outputs` (cursor `<pubOutIdx>`) or `private_transcript` (cursor `<privIdx>`), decodes them with `decodeStrict`, and advances the cursor. A short transcript gives `panic("range end index out of range: public transcript outputs too short")` or its private counterpart, because `preprocess` slices before checking (divergence K2, see 13-known-divergences.md).

`encode` is the only way a program sees an encoding. `#encode` requires `size(encodeValue(V))` to equal the number of named outputs, otherwise `error("Unexpected output length of encode instruction: T")`, and `#bindEncoded` binds each element as a `native` register. Its gate `#encMatch` in `zkir-constraints.k` states the same equalities in circuit.

`output` checks each operand's `typeOf` against the signature (`#output1`) and appends it to `<outputs>`. Under the commitment flag `#finish` hands the commitment to `#commCheck`, which accepts it when it equals `transientCommit(#preInputs(Pre) #encodeAll(Os), Rand)` and otherwise fails with `Communications commitment mismatch`. That term is the Poseidon hash of the randomness, the raw inputs exactly as given, and the encoded outputs; `#encodeAll` in `zkir-constraints.k` maps `encodeValue` over the output list.

The in-circuit `commGate` (`#commOutcome` in `zkir-constraints.k`) hashes `#encInputs`, the re-encoding of the bound input registers, in place of the raw inputs. Its outcome is `holds` when the hash equals `<pi>` position 1, `violated("communications commitment differs from poseidon(rand ++ encode(inputs) ++ encode(outputs))")` when it differs, `unknown` when the vector is shorter than two, and `synthErr` when the Poseidon chip is not initialised. The two hashes cover different streams exactly when some input's raw elements differ from its re-encoding, which on the base surface is every non-canonical encoding that decodes successfully (the four forms above). Such a preimage, with a commitment computed over the raw stream, passes `#commCheck` while the gate is violated, barring a Poseidon collision; case `f05_noncanonical_foreign_limbs` records this (`commGate:violated`). On the extension surface strict decoding rejects the input in `#loadInputs`, before any commitment is compared.

Both harnesses re-encode memory: the oracle prints `encode_offcircuit` of every register with its variant and type, and `encode_value` in `tools/zkir_run.py` does the same for the K configuration with an independent Python implementation of every layout. `diff_test.py` compares the `encoded` vector and `type` of each register. This comparison covers the types that have no dedicated unit check in the table below.

## Error messages and how the harness classes them

`tools/diff_test.py` maps every error message of both sides to a class (`ERROR_CLASSES`, `err_class`) and requires the classes to agree whenever both statuses are `error` or both are `panic`; a message matching no pattern gets the class `other:` followed by the first 40 characters of the message.

| Message (K) | Origin in K | Origin in Rust | Harness class |
|---|---|---|---|
| `Failed to decode as T` | `decodeValue`, `#decField`, `#decWPoint`, `#decPointResult` | `decode_offcircuit`: `Failed to decode {encoded:?} as {val_t:?}` | `decode` |
| `The encoded value of type T is not in canonical form` | `#canonical` (`decodeStrict`) | `decode_offcircuit` at 2ffe2d1 | `decode` |
| `Not enough raw inputs: ran out at index i while decoding N` | `#loadInputs` | `preprocess` | `decode` |
| `Expected n raw inputs, received m` | `#loadInputs` | `preprocess` | `decode` |
| `is not a canonical field element` | `decodeValue` for `native()` | `zkir-oracle` before `preprocess` (a load error, not an error run) | not reached through the runners |
| `assertion failed: Bytes32 low element uses byte 31 or high element exceeds a byte` | `decPanic` (status `panic`) | `assert_eq!` in `decode_offcircuit` at 92e8bdd3 (process abort, exit 101) | no pattern matches either text, so the two `other:` classes differ and `diff_test.py` would report a disagreement; no recorded comparison ends in a panic |
| `Unexpected output length of encode instruction: T` | `#encode` | `preprocess`, `Encode` arm | `other:Unexpected output length of encode in` on both sides |
| `Communications commitment mismatch` | `#commCheck` | `preprocess` | `commitment` |
| `cannot convert T to Native` | `#impactOne`, `asNative` | `TryFrom<IrValue>` | `type-conversion` |
| `range end index out of range: public transcript outputs too short` and `range end index out of range: private transcript too short` | `#input` (status `panic`) | slice index panic in `preprocess` | `transcript-short` |

The panic paths are exercised by `tools/divergence_tests.py` (cases `k03_bytes32_input_assertion_panics` and `k02_transcript_too_short_panics`), which compares the status on each side and the outcome of one gate, not the error classes.

## Cross-check table

`tools/unit_values.py` evaluates K function terms through the ZKIR-TEST definition and compares them with `enc_foreign` and the curve arithmetic of the same file; 43 of 43 checks pass (`evidence/zkir-k-unit-values-2026-09-05c.txt`).

| Type | `encodedLen` | K encoder rule (`zkir-values.k`) | Rust function (midnight-circuits 7.2.4 unless noted) | Unit checks in `unit_values.py` |
|---|---|---|---|---|
| Native | 1 | `encodeValue(native(X))` | `AssignedNative::as_public_input` | `dec native out of field` |
| Bytes32 | 2 | `encodeValue(bytes32(B))` | `encode_offcircuit`, Bytes32 arm (encode.rs) | `enc bytes32`, `dec bytes32`, `dec bytes32 bad high (Rust assert_eq! panics)` |
| JubjubPoint | 2 | `encodeValue(jubjubPoint(pt(X, Y)))` | `AssignedNativePoint::as_public_input` | `dec jubjubPoint 8G`, `dec jubjubPoint G (not subgroup)` |
| JubjubScalar | 1 | `encodeValue(jubjubScalar(S))` | `AssignedScalarOfNativeCurve::as_public_input` | `dec jubjubScalar`, `dec jubjubScalar too big` |
| Secp256k1Point | 5 | `encWPoint(Q, #k256P)` | `AssignedForeignPoint::as_public_input` | `enc secp256k1Point G`, `dec secp256k1Point G`, `enc secp256k1Point inf`, `dec secp256k1Point inf` |
| Secp256k1Base | 2 | `encField64(X, #k256P)` | `AssignedField::<F, k256::Fp, MEP>::as_public_input` | `enc secp256k1Base`, `enc secp256k1Base 0`, `dec secp256k1Base` |
| Secp256k1Scalar | 2 | `encField64(X, #k256N)` | `AssignedField::<F, k256::Fq, MEP>::as_public_input` | differential harness only |
| Secp256r1Point | 5 | `encWPoint(Q, #p256P)` | `AssignedForeignPoint::as_public_input` | differential harness only |
| Secp256r1Base | 2 | `encField64(X, #p256P)` | `AssignedField::<F, p256::Fp, MEP>::as_public_input` | differential harness only |
| Secp256r1Scalar | 2 | `encField64(X, #p256N)` | `AssignedField::<F, p256::Fq, MEP>::as_public_input` | differential harness only |
| Curve25519Point | 4 | `encEPointForeign(Q, #c25519P)` | `AssignedForeignEdwardsPoint::as_public_input` | `enc curve25519Point B`, `dec curve25519Point B` |
| Curve25519Base | 2 | `encField64(X, #c25519P)` | `AssignedField::<F, curve25519::Fp, MEP>::as_public_input` | differential harness only |
| Curve25519Scalar | 2 | `encForeign(X, #c25519L, 51, 5)` | `AssignedField::<F, curve25519::Scalar, MEP>::as_public_input` | `enc curve25519Scalar`, `dec curve25519Scalar` |

To rerun the unit checks from the repository root:

```sh
uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py
```
