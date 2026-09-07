> For the complete documentation index, see [llms.txt](/llms.txt)

# Detailed API reference

## Top-level exports and native types and functions[​](#top-level-exports-and-native-types-and-functions "Direct link to Top-level exports and native types and functions")

Exporting a type or circuit from the top level of a contract makes its definition visible and usable from the contract's TypeScript API. Exporting a circuit from the top level of a contract additionally makes it one of the contract's endpoints for on-chain transactions.

Many of the the types and functions defined in the standard library are **native** types and functions. These are ones that have special handling of some kind in the Compact compiler. As a consequence of this special handling, they cannot currently be exported from the top level of a contract.

It is a compiler error to try to export these types and functions.

You can, however, export type aliases for native types and export circuits that wrap native functions. For example:

```
import { JubjubPoint as nativeJubjubPoint, ecAdd as nativeEcAdd } from CompactStandardLibrary;



export type JubjubPoint = nativeJubjubPoint;



export pure circuit ecAdd(a: JubjubPoint, b: JubjubPoint): JubjubPoint {

  return nativeEcAdd(a, b);

}
```

The generated TypeScript API will include definitions for `JubjubPoint` and `ecAdd`. Note that the standard library's `ecAdd` is polymorphic (it works with other curve types besides Jubjub) but the exported version only works for `JubjubPoint`.

## Structure types[​](#structure-types "Direct link to Structure types")

### `Maybe`[​](#maybe "Direct link to maybe")

Encapsulates an optionally present value. If `isSome` is `false`, `value` should be `default<T>` by convention.

```
struct Maybe<T> {

  isSome: Boolean;

  value: T;

}
```

### `Either`[​](#either "Direct link to either")

Disjoint union of `A` and `B`. Iff `isLeft` if `true`, `left` should be populated, otherwise `right`. The other should be `default< >` by convention.

```
struct Either<A, B> {

  isLeft: Boolean;

  left: A;

  right: B;

}
```

### `JubjubSchnorrSignature`[​](#jubjubschnorrsignature "Direct link to jubjubschnorrsignature")

A Schnorr signature over the JubJub embedded curve. Contains an announcement point and a scalar response, used with [`jubjubSchnorrVerify`](#jubjubschnorrverify).

```
struct JubjubSchnorrSignature {

  announcement: JubjubPoint;

  response: Field;

}
```

### `JubjubSchnorrSignature`[​](#jubjubschnorrsignature-1 "Direct link to jubjubschnorrsignature-1")

A Schnorr signature over the JubJub embedded curve. Contains an announcement point and a scalar response, used with [`jubjubSchnorrVerify`](#jubjubschnorrverify).

```
struct JubjubSchnorrSignature {

  announcement: JubjubPoint;

  response: Field;

}
```

### `Secp256k1EcdsaSignature`[​](#secp256k1ecdsasignature "Direct link to secp256k1ecdsasignature")

An ECDSA signature over the secp256k1 curve, used with [`secp256k1EcdsaVerify`](#secp256k1ecdsaverify). The `r` and `s` components are `Secp256k1Scalar`s.

```
struct Secp256k1EcdsaSignature {

  r: Secp256k1Scalar;

  s: Secp256k1Scalar;

}
```

### `MerkleTreeDigest`[​](#merkletreedigest "Direct link to merkletreedigest")

The root hash of a Merkle tree, represented by a single `Field`.

```
struct MerkleTreeDigest { field: Field; }
```

### `MerkleTreePathEntry`[​](#merkletreepathentry "Direct link to merkletreepathentry")

An entry in a Merkle tree path, indicating if the path leads left or right, and the root of the sibling node. Primarily used in [`MerkleTreePath`](#merkletreepath)

```
struct MerkleTreePathEntry {

  sibling: MerkleTreeDigest;

  goesLeft: Boolean;

}
```

### `MerkleTreePath`[​](#merkletreepath "Direct link to merkletreepath")

A path in a depth `n` Merkle tree, leading to a leaf of type `T`. Primarily used for [`merkleTreePathRoot`](#merkletreepathroot).

This can be constructed from `witness`es that use the compiler output's `findPathForLeaf` and `pathForLeaf` functions.

```
struct MerkleTreePath<#n, T> {

  leaf: T;

  path: Vector<n, MerkleTreePathEntry>;

}
```

### `ContractAddress`[​](#contractaddress "Direct link to contractaddress")

The address of a contract, used as a recipient in [`sendShielded`](#sendshielded), [`sendImmediateShielded`](#sendimmediateshielded), [`createZswapOutput`](#createzswapoutput), and [`mintShieldedToken`](#mintshieldedtoken).

```
struct ContractAddress { bytes: Bytes<32>; }
```

### `ShieldedCoinInfo`[​](#shieldedcoininfo "Direct link to shieldedcoininfo")

The description of a newly created shielded coin, used in outputting shielded coins, or spending/receiving shielded coins that originate in the current transaction.

`nonce` can be deterministically derived with [`evolveNonce`](#evolvenonce).

Used in:

* [`receiveShielded`](#receiveshielded)
* [`sendImmediateShielded`](#sendimmediateshielded)
* [`mergeCoin`](#mergecoin)
* [`mergeCoinImmediate`](#mergecoinimmediate)
* [`createZswapOutput`](#createzswapoutput)

```
struct ShieldedCoinInfo {

  nonce: Bytes<32>;

  color: Bytes<32>;

  value: Uint<128>;

}
```

### `QualifiedShieldedCoinInfo`[​](#qualifiedshieldedcoininfo "Direct link to qualifiedshieldedcoininfo")

The description of an existing shielded coin in the ledger, ready to be spent.

Used in:

* [`sendShielded`](#sendshielded)
* [`mergeCoin`](#mergeCoin)
* [`mergeCoinImmediate`](#mergecoinimmediate)
* [`createZswapInput`](#createzswapinput)

```
struct QualifiedShieldedCoinInfo {

  nonce: Bytes<32>;

  color: Bytes<32>;

  value: Uint<128>;

  mtIndex: Uint<64>;

}
```

### `ZswapCoinPublicKey`[​](#zswapcoinpublickey "Direct link to zswapcoinpublickey")

The public key used to output a [`ShieldedCoinInfo`](#shieldedcoininfo) to a user, used as a recipient in [`sendShielded`](#sendshielded), [`sendImmediateShielded`](#sendimmediateshielded), and [`createZswapOutput`](#createzswapoutput).

```
struct ZswapCoinPublicKey { bytes: Bytes<32>; }
```

### `ShieldedSendResult`[​](#shieldedsendresult "Direct link to shieldedsendresult")

The output of [`sendShielded`](#sendshielded) and [`sendImmediateShielded`](#sendimmediateshielded), detailing the created shielded coin, and the change from spending the input, if applicable.

```
struct ShieldedSendResult {

  change: Maybe<ShieldedCoinInfo>;

  sent: ShieldedCoinInfo;

}
```

### `UserAddress`[​](#useraddress "Direct link to useraddress")

The public key of a user, used as a recipient in [`sendUnshielded`](#sendunshielded) and [`mintUnshieldedToken`](#mintunshieldedtoken).

```
struct UserAddress { bytes: Bytes<32>; }
```

## Events[​](#events "Direct link to Events")

Events are struct types that can be emitted using an `emit` operation.

### `ShieldedSpend`[​](#shieldedspend "Direct link to shieldedspend")

Shielded coin consumed, new coin created for a user recipient.

Serialized size is 32.

```
struct ShieldedSpend {

  nullifier: Bytes<32> // indexed

}
```

### `ShieldedReceive`[​](#shieldedreceive "Direct link to shieldedreceive")

A contract accepts an incoming shielded coin.

`contractAddress` set when received by a contract, absent for user recipients.

Serialized size is 578.

```
struct ShieldedReceive {

  commitment: Bytes<32>, // indexed

  ciphertext: Maybe<Bytes<512>>,

  contractAddress: Maybe<Bytes<32>>

}
```

### `ShieldedMint`[​](#shieldedmint "Direct link to shieldedmint")

New shielded tokens created.

`tokenType` derived by the consumer from `domainSep` + `ContractLog.address`.

Serialized size is 81.

```
struct ShieldedMint {

  commitment: Bytes<32>, // indexed

  domainSep: Bytes<32>, // indexed

  amount: Maybe<Uint<128>>

}
```

### `ShieldedBurn`[​](#shieldedburn "Direct link to shieldedburn")

Shielded coin sent to the burn address.

Supply tracking — tokens permanently removed from circulation.

Serialized size is 49.

```
struct ShieldedBurn {

  nullifier: Bytes<32>, // indexed

  amount: Maybe<Uint<128>>

}
```

### `UnshieldedSpend`[​](#unshieldedspend "Direct link to unshieldedspend")

Public token sent from a sender.

Serialized size is 145.

```
struct UnshieldedSpend {

  sender: Either<ZswapCoinPublicKey, ContractAddress>, // indexed

  domainSep: Bytes<32>, // indexed

  tokenType: Bytes<32>, // indexed

  amount: Uint<128>

}
```

### `UnshieldedReceive`[​](#unshieldedreceive "Direct link to unshieldedreceive")

Public token sent to a recipient.

Serialized size is 145.

```
struct UnshieldedReceive {

  recipient: Either<ZswapCoinPublicKey, ContractAddress>, // indexed

  domainSep: Bytes<32>, // indexed

  tokenType: Bytes<32>, // indexed

  amount: Uint<128>

}
```

### `UnshieldedMint`[​](#unshieldedmint "Direct link to unshieldedmint")

New unshielded tokens created.

Serialized size is 80.

```
struct UnshieldedMint {

  domainSep: Bytes<32>, // indexed

  tokenType: Bytes<32>, // indexed

  amount: Uint<128>

}
```

### `UnshieldedBurn`[​](#unshieldedburn "Direct link to unshieldedburn")

Unshielded coin sent to the burn address.

Serialized size is 113.

```
struct UnshieldedBurn {

  sender: Either<ZswapCoinPublicKey, ContractAddress>, // indexed

  tokenType: Bytes<32>, // indexed

  amount: Uint<128>

}
```

### `Paused`[​](#paused "Direct link to paused")

Contract operations suspended.

Serialized size is 0.

```
struct Paused {}
```

### `Unpaused`[​](#unpaused "Direct link to unpaused")

Contract operations resumed.

Serialized size is 0.

```
struct Unpaused {}
```

### `Misc`[​](#misc "Direct link to misc")

Miscellaneous event type.

Serialized size is 288.

```
struct Misc {

  name: Bytes<32>,

  payload: Bytes<256>

}
```

## Circuits[​](#circuits "Direct link to Circuits")

### `some`[​](#some "Direct link to some")

Constructs a [`Maybe<T>`](#maybe) containing an element of type `T`

```
circuit some<T>(value: T): Maybe<T>;
```

### `none`[​](#none "Direct link to none")

Constructs a [`Maybe<T>`](#maybe) containing nothing

```
circuit none<T>(): Maybe<T>;
```

### `left`[​](#left "Direct link to left")

Construct an [`Either<A, B>`](#either) containing the `A` item of the disjoint union

```
circuit left<A, B>(value: A): Either<A, B>;
```

### `right`[​](#right "Direct link to right")

Constructs an [`Either<A, B>`](#either) containing the `B` item of the disjoint union

```
circuit right<A, B>(value: B): Either<A, B>;
```

### `transientHash`[​](#transienthash "Direct link to transienthash")

Builtin transient hash compression function

This function is a circuit-efficient compression function from arbitrary values to field elements, which is not guaranteed to persist between upgrades. It should not be used to derive state data, but can be used for consistency checks.

Although this function returns a hash of its inputs, it is not considered sufficient to protect its input from disclosure. If its input contains any value returned from a witness, the program must acknowledge disclosure (via a `disclose` wrapper) if the result can be stored in the public ledger, returned from an exported circuit, or passed to another contract via a cross-contract call.

```
circuit transientHash<T>(value: T): Field;
```

### `transientCommit`[​](#transientcommit "Direct link to transientcommit")

Builtin transient commitment function

This function is a circuit-efficient commitment function over arbitrary types, and a field element commitment opening, to field elements, which is not guaranteed to persist between upgrades. It should not be used to derive state data, but can be used for consistency checks.

Unlike `transientHash`, this function is considered sufficient to protect its input from disclosure, under the assumption that the `rand` argument is sufficiently random. Thus, even if its input contains a value or values returned from one or more witnesses, the program need not acknowledge disclosure (via a `disclose` wrapper) if the result can be stored in the public ledger, returned from an exported circuit, or passed to another contract via a cross-contract call.

```
circuit transientCommit<T>(value: T, rand: Field): Field;
```

### `persistentHash`[​](#persistenthash "Direct link to persistenthash")

Builtin persistent hash compression function

This function is a non-circuit-optimised compression function from arbitrary values to a 256-bit bytestring. It is guaranteed to persist between upgrades, and to consistently use the SHA-256 compression algorithm. It *should* be used to derive state data, and not for consistency checks where avoidable.

The note about disclosing under `transientHash` also applies to this function.

```
circuit persistentHash<T>(value: T): Bytes<32>;
```

### `persistentCommit`[​](#persistentcommit "Direct link to persistentcommit")

Builtin persistent commitment function

This function is a non-circuit-optimised commitment function from arbitrary values representable in Compact, and a 256-bit bytestring opening, to a 256-bit bytestring. It is guaranteed to persist between upgrades, and use the SHA-256 compression algorithm. It *should* be used to derive state data, and not for consistency checks where avoidable.

The note about disclosing under `transientCommit` also applies to this function.

```
circuit persistentCommit<T>(value: T, rand: Bytes<32>): Bytes<32>;
```

### `degradeToTransient`[​](#degradetotransient "Direct link to degradetotransient")

This function "degrades" the output of a [`persistentHash`](#persistenthash) or [`persistentCommit`](#persistentcommit) to a field element, which can then be used in [`transientHash`](#transienthash) or [`transientCommit`](#transientcommit).

```
circuit degradeToTransient(x: Bytes<32>) : Field;
```

### `upgradeFromTransient`[​](#upgradefromtransient "Direct link to upgradefromtransient")

This function "upgrades" a field element to the output of a [`persistentHash`](#persistenthash) or [`persistentCommit`](#persistentcommit).

```
circuit upgradeFromTransient(x: Field): Bytes<32>;
```

### `keccak256`[​](#keccak256 "Direct link to keccak256")

This function hashes its input using the Keccak-256 algorithm. It returns the 32-byte digest.

```
circuit keccak256<T>(value: T): Bytes<32>;
```

### `JubjubPoint`[​](#jubjubpoint "Direct link to jubjubpoint")

This is a native type.

The type of points on the embedded elliptic curve. It represents a pair of affine x- and y-coordinates. The coordinates are native `Field` (BLS12-381) values.

### `JubjubScalar`[​](#jubjubscalar "Direct link to jubjubscalar")

This is a native type.

The type of numeric values between 0 (inclusive) and the order of the prime-order subgroup of the Jubjub embedded elliptic curve (exclusive). It is the type of the scalars used to multiply Jubjub curve points.

The maximum value (one less that the field order) is (decimal) 6554484396890773809930967563523245729705921265872317281365359162392183254198 and (hexadecimal) 0xe7db4ea6533afa906673b0101343b00a6682093ccc81082d0970e5ed6f72cb6.

### `constructJubjubPoint`[​](#constructjubjubpoint "Direct link to constructjubjubpoint")

This is a native circuit.

This function constructs a [`JubjubPoint`](#jubjubpoint) from its x- and y-coordinates. Neither the standard library nor the Compact JavaScript runtime package will actually verify that a constructed point actually lies on the Jubjub curve. The behavior of constructing or operating on an invalid Jubjub curve point is undefined. You will not normally be able to construct proofs involving invalid Jubjub curve points.

```
circuit constructJubjubPoint(x: Field, y: Field): JubjubPoint;
```

### `jubjubPointX`[​](#jubjubpointx "Direct link to jubjubpointx")

This is a native circuit.

This function extracts the x-coordinate from a [`JubjubPoint`](#jubjubpoint).

```
circuit jubjubPointX(pt: JubjubPoint): Field;
```

### `jubjubPointY`[​](#jubjubpointy "Direct link to jubjubpointy")

This is a native circuit.

This function extracts the y-coordinate from a [`JubjubPoint`](#jubjubpoint).

```
circuit jubjubPointY(pt: JubjubPoint): Field;
```

### `Secp256k1Point`[​](#secp256k1point "Direct link to secp256k1point")

This is a native type.

The type of points on the secp256k1 elliptic curve. It represents a pair of affine x- and y-coordinates. The coordinates are `Secp256k1Base` values. Secp256k1 points cannot be created in Compact, but they can be passed as circuit arguments and returned from witness functions. The behavior of operating on an invalid secp256k1 curve point is undefined. You will not normally be able to construct proofs involving invalid secp256k1 curve points.

The (additive) identity point does not have a representation as a pair of coordinates. It is represented in Compact as `default<Secp256k1Point>`.

### `Secp256k1Base`[​](#secp256k1base "Direct link to secp256k1base")

This is a native type.

The type of values between 0 (inclusive) and the order of the base field of the secp256k1 elliptic curve (exclusive). It is the type of the affine coordinates of a point on that curve.

The maximum value (one less than the field order) is (decimal) 115792089237316195423570985008687907853269984665640564039457584007908834671662 and (hexadecimal) 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e.

### `Secp256k1Scalar`[​](#secp256k1scalar "Direct link to secp256k1scalar")

This is a native type.

The type of numeric values betwen 0 (inclusive) and the order of the secp256k1 group (exclusive). This is the type of the scalars used to multiply secp256k1 curve points.

The maximum value (one less than the field order) is (decimal) 115792089237316195423570985008687907852837564279074904382605163141518161494336 and (hexadecimal) 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364140.

### `secp256k1PointX`[​](#secp256k1pointx "Direct link to secp256k1pointx")

This is a native type.

This function extracts the affine x-coordinate from a [`Secp256k1Point`](#secp256k1point).

```
circuit secp256k1PointX(pt: Secp256k1Point): Secp256k1Base;
```

### `secp256k1PointY`[​](#secp256k1pointy "Direct link to secp256k1pointy")

This is a native type.

This function extracts the affine y-coordinate from a [`Secp256k1Point`](#secp256k1point).

```
circuit secp256k1PointY(pt: Secp256k1Point): Secp256k1Base;
```

### `ecAdd`[​](#ecadd "Direct link to ecadd")

This function adds two elliptic curve points. It is polymorphic for the following types:

* [`JubjubPoint`](#jubjubpoint)s
* [`Secp256k1Point`](#secp256k1point)s.

```
circuit ecAdd(a: JubjubPoint, b: JubjubPoint): JubjubPoint;

circuit ecAdd(a: Secp256k1Point, b: Secp256k1Point): Secp256k1Point;
```

### `ecNeg`[​](#ecneg "Direct link to ecneg")

This function negates an elliptic [`JubjubPoint`](#jubjubpoint). On the JubJub twisted Edwards curve, the negation of `(x, y)` is `(-x, y)`.

```
circuit ecNeg(a: JubjubPoint): JubjubPoint;
```

### `ecMul`[​](#ecmul "Direct link to ecmul")

This function multiplies an elliptic curve point by a scalar. It is polymorphic for the following types:

* [`JubjubPoint`](#jubjubpoint)s
* [`Secp256k1Point`](#secp256k1point)s.

```
circuit ecMul(a: JubjubPoint, b: JubjubScalar): JubjubPoint;

circuit ecMul(a: Secp256k1Point, b: Secp256k1Scalar): Secp256k1Point;
```

### `ecMulGenerator`[​](#ecmulgenerator "Direct link to ecmulgenerator")

This function multiplies the primary group generator of a curve by a scalar. It is polymorphic for the following types:

* [`JubjubPoint`](#jubjubpoint)s
* [`Secp256k1Point`](#secp256k1point)s.

```
circuit ecMulGenerator(b: JubjubScalar): JubjubPoint;

circuit ecMulGenerator(b: Secp256k1Scalar): Secp256k1Point;
```

### `neg`[​](#neg "Direct link to neg")

Negates a field element, i.e. returns the value `y` such that `add(x, y)` is `0` in the field. Polymorphic function that works over types:

* `Secp256k1Scalar`
* `Secp256k1Base`

```
circuit neg(x: Secp256k1Scalar): Secp256k1Scalar;

circuit neg(x: Secp256k1Base): Secp256k1Base;
```

### `inv`[​](#inv "Direct link to inv")

Returns the multiplicative inverse of a field element, i.e. the value `y` such that `mul(x, y)` is `1` in the field. Polymorphic function that works over types:

* `Secp256k1Scalar`
* `Secp256k1Base`

```
circuit inv(x: Secp256k1Scalar): Secp256k1Scalar;

circuit inv(x: Secp256k1Base): Secp256k1Base;
```

### `hashToCurve`[​](#hashtocurve "Direct link to hashtocurve")

This function maps arbitrary types to [`JubjubPoint`](#nativepoint)s.

Outputs are guaranteed to have unknown discrete logarithm with respect to the group base, and any other output, but are not guaranteed to be unique (a given input can be proven correct for multiple outputs).

Inputs of different types `T` may have the same output, if they have the same field-aligned binary representation.

```
circuit hashToCurve<T>(value: T): JubjubPoint;
```

### `jubjubSchnorrVerify`[​](#jubjubschnorrverify "Direct link to jubjubschnorrverify")

Verifies a Schnorr signature over the JubJub embedded curve. Takes a message as a vector of `N` field elements, a [`JubjubSchnorrSignature`](#jubjubschnorrsignature), and a verification key (a [`JubjubPoint`](#nativepoint) on the embedded curve). Returns true if the signature is valid; false if the signature does not verify.

To actually enforce that a signature is valid in a Compact circuit, use an `assert` that the result is true.

```
circuit jubjubSchnorrVerify<#N>(

          msg: Vector<N, Field>,

          signature: JubjubSchnorrSignature,

          pk: JubjubPoint

          ): Boolean;
```

### `jubjubSchnorrVerify`[​](#jubjubschnorrverify-1 "Direct link to jubjubschnorrverify-1")

Verifies a Schnorr signature over the JubJub embedded curve. Takes a message as a vector of `n` field elements, a [`JubjubSchnorrSignature`](#jubjubschnorrsignature), and a verification key (a [`JubjubPoint`](#nativepoint) on the embedded curve). Asserts that the signature is valid; fails if the signature does not verify.

```
circuit jubjubSchnorrVerify<#n>(msg: Vector<n, Field>, signature: JubjubSchnorrSignature, vk: JubjubPoint): [];
```

### `secp256k1EcdsaVerify`[​](#secp256k1ecdsaverify "Direct link to secp256k1ecdsaverify")

Verifies an ECDSA signature over the secp256k1 curve. Takes a 32-byte message hash, a [`Secp256k1EcdsaSignature`](#secp256k1ecdsasignature), and a public key (a [`Secp256k1Point`](#secp256k1point)). Returns true if the signature is valid; false otherwise.

The circuit takes `msgHash` as given and does not constrain it to any message. The caller is expected to bind it to the actual message by hashing that message in-circuit (e.g. with [`keccak256`](#keccak256) for Ethereum-style signatures or [`persistentHash`](#persistenthash) for Bitcoin-style ones).

To actually enforce that a signature is valid in a Compact circuit, use an `assert` that the result is true.

```
circuit secp256k1EcdsaVerify(msgHash: Bytes<32>, sig: Secp256k1EcdsaSignature, pk: Secp256k1Point): Boolean;
```

### `secp256k1EthereumAddress`[​](#secp256k1ethereumaddress "Direct link to secp256k1ethereumaddress")

Derives the 20-byte Ethereum-style address of a secp256k1 public key, i.e. the low 20 bytes of the Keccak-256 hash of the [`Secp256k1Point`](#secp256k1point).

```
circuit secp256k1EthereumAddress(pk: Secp256k1Point): Bytes<20>;
```

### `merkleTreePathRoot`[​](#merkletreepathroot "Direct link to merkletreepathroot")

Derives the Merkle tree root of a [`MerkleTreePath`](#merkletreepath), which should match the root of the tree that this path originated from.

```
circuit merkleTreePathRoot<#n, T>(path: MerkleTreePath<n, T>): MerkleTreeDigest;
```

### `merkleTreePathRootNoLeafHash`[​](#merkletreepathrootnoleafhash "Direct link to merkletreepathrootnoleafhash")

Derives the Merkle tree root of a [`MerkleTreePath`](#merkletreepath), which should match the root of the tree that this path originated from. As opposed to [`merkleTreePathRoot`](#merkletreepathroot), this variant assumes that the tree leaves have already been hashed externally.

```
circuit merkleTreePathRootNoLeafHash<#n>(path: MerkleTreePath<n, Bytes<32>>): MerkleTreeDigest;
```

### `nativeToken`[​](#nativetoken "Direct link to nativetoken")

Returns the token type of the native token

```
circuit nativeToken(): Bytes<32>;
```

### `tokenType`[​](#tokentype "Direct link to tokentype")

Transforms a domain separator for the given contract into a globally namespaced token type. A contract can issue tokens for its domain separators, which lets it create new tokens, but due to collision resistance, it cannot mint tokens for another contract's token type. This is used as the `color` field in [`ShieldedCoinInfo`](#shieldedcoininfo) and as arguments to functions like [`sendUnshielded`](#sendunshielded) and [`receiveUnshielded`](#receiveunshielded).

```
circuit tokenType(domainSep: Bytes<32>, contract: ContractAddress): Bytes<32>;
```

### `mintShieldedToken`[​](#mintshieldedtoken "Direct link to mintshieldedtoken")

Creates a new shielded coin, minted by this contract, and sends it to the given recipient. Returns the corresponding [`ShieldedCoinInfo`](#shieldedcoininfo). This requires inputting a unique nonce to function securely, it is left to the user how to produce this. To mint a shielded token to the current contract, pass `right<ZswapCoinPublicKey, ContractAddress>(kernel.self())` as the `recipient`.

```
circuit mintShieldedToken(

  domainSep: Bytes<32>,

  value: Uint<64>,

  nonce: Bytes<32>,

  recipient: Either<ZswapCoinPublicKey, ContractAddress>

): ShieldedCoinInfo;
```

### `evolveNonce`[​](#evolvenonce "Direct link to evolvenonce")

Deterministically derives a [`ShieldedCoinInfo`](#shieldedcoininfo) nonce from a counter index, and a prior nonce.

```
circuit evolveNonce(

  index: Uint<128>,

  nonce: Bytes<32>

): Bytes<32>;
```

### `shieldedBurnAddress`[​](#shieldedburnaddress "Direct link to shieldedburnaddress")

Returns a payment address that guarantees any shielded coins sent to it are burned.

```
circuit shieldedBurnAddress(): Either<ZswapCoinPublicKey, ContractAddress>;
```

### `receiveShielded`[​](#receiveshielded "Direct link to receiveshielded")

Receives a shielded coin, adding a validation condition requiring this coin to be present as an output addressed to this contract, and not received by another call

```
circuit receiveShielded(coin: ShieldedCoinInfo): [];
```

### `sendShielded`[​](#sendshielded "Direct link to sendshielded")

Sends given value from a shielded coin owned by the contract to a recipient. Any change is returned and should be managed by the contract.

Note that this does not currently create coin ciphertexts, so sending to a user public key except for the current user will not lead to this user being informed of the coin they've been sent. To send a shielded token to the current contract, pass `right<ZswapCoinPublicKey, ContractAddress>(kernel.self())` as the `recipient`.

```
circuit sendShielded(input: QualifiedShieldedCoinInfo, recipient: Either<ZswapCoinPublicKey, ContractAddress>, value: Uint<128>): ShieldedSendResult;
```

### `sendImmediateShielded`[​](#sendimmediateshielded "Direct link to sendimmediateshielded")

Like [`sendShielded`](#sendshielded), but for coins created within this transaction

```
circuit sendImmediateShielded(input: ShieldedCoinInfo, target: Either<ZswapCoinPublicKey, ContractAddress>, value: Uint<128>): ShieldedSendResult;
```

### `mergeCoin`[​](#mergecoin "Direct link to mergecoin")

Takes two coins stored on the ledger, and combines them into one

```
circuit mergeCoin(a: QualifiedCoinInfo, b: QualifiedCoinInfo): CoinInfo;
```

### `mergeCoinImmediate`[​](#mergecoinimmediate "Direct link to mergecoinimmediate")

Takes one coin stored on the ledger, and one created within this transaction, and combines them into one

```
circuit mergeCoinImmediate(a: QualifiedCoinInfo, b: CoinInfo): CoinInfo;
```

### `ownPublicKey`[​](#ownpublickey "Direct link to ownpublickey")

Returns the [`ZswapCoinPublicKey`](#zswapcoinpublickey) of the end-user creating this transaction.

```
circuit ownPublicKey(): ZswapCoinPublicKey;
```

### `createZswapInput`[​](#createzswapinput "Direct link to createzswapinput")

Notifies the context to create a new Zswap input originating from this call. Should typically not be called manually, prefer [`sendShielded`](#sendshielded) and [`sendImmediateShielded`](#sendimmediateshielded) instead.

The note about disclosing under `transientHash` also applies to this function.

```
circuit createZswapInput(coin: QualifiedShieldedCoinInfo): [];
```

### `createZswapOutput`[​](#createzswapoutput "Direct link to createzswapoutput")

Notifies the context to create a new Zswap output originating from this call. Should typically not be called manually, prefer [`sendShielded`](#sendshielded) and [`sendImmediateShielded`](#sendimmediateShielded), and [`receiveShielded`](#receiveshielded) instead.

The note about disclosing under `transientHash` also applies to this function.

```
circuit createZswapOutput(coin: ShieldedCoinInfo, recipient: Either<ZswapCoinPublicKey, ContractAddress>): [];
```

### `mintUnshieldedToken`[​](#mintunshieldedtoken "Direct link to mintunshieldedtoken")

Creates a new unshielded coin, minted by this contract, and sends it to the given recipient. Returns the corresponding coin color. To mint an unshielded token to the current contract, pass `left<ContractAddress, UserAddress>(kernel.self())` as the `recipient`.

```
export circuit mintUnshieldedToken(

  domainSep: Bytes<32>,

  value: Uint<64>,

  recipient: Either<ContractAddress, UserAddress>

): Bytes<32>;
```

### `sendUnshielded`[​](#sendunshielded "Direct link to sendunshielded")

Sends the given amount of the given unshielded token (identified by the color) to the given recipient. No change is returned from this function. To send an unshielded token to the current contract, pass `left<ContractAddress, UserAddress>(kernel.self())` as the `recipient`.

```
export circuit sendUnshielded(color: Bytes<32>, amount: Uint<128>, recipient: Either<ContractAddress, UserAddress>): [];
```

### `receiveUnshielded`[​](#receiveunshielded "Direct link to receiveunshielded")

Receives the given amount of the unshielded token identified by the color.

```
circuit receiveUnshielded(color: Bytes<32>, amount: Uint<128>): [];
```

### `unshieldedBalance`[​](#unshieldedbalance "Direct link to unshieldedbalance")

Returns the contract's balance of the unshielded token of the given type. Note that this balance is not updated during contract execution as a result of unshielded sends and receives. It is always fixed to the value provided at the start of execution. Also note that using this function means transaction application will fail unless the token balance at the time of transaction construction is exactly the same as the balance at the time of transaction application. Unless you want to require that, prefer to use the balance comparison functions [`unshieldedBalanceLt`](#unshieldedbalancelt), [`unshieldedBalanceGte`](#unshieldedbalancegte), [`unshieldedBalanceGt`](#unshieldedbalancegt), and [`unshieldedBalanceLte`](#unshieldedbalancelte).

```
circuit unshieldedBalance(color: Bytes<32>): Uint<128>;
```

### `unshieldedBalanceLt`[​](#unshieldedbalancelt "Direct link to unshieldedbalancelt")

Returns true if the unshielded balance of the contract for the given token type is less than the given value.

```
circuit unshieldedBalanceLt(color: Bytes<32>, amount: Uint<128>): Boolean;
```

### `unshieldedBalanceGte`[​](#unshieldedbalancegte "Direct link to unshieldedbalancegte")

Returns true if the unshielded balance of the contract for the given token type is greater than or equal to the given value.

```
circuit unshieldedBalanceGte(color: Bytes<32>, amount: Uint<128>): Boolean;
```

### `unshieldedBalanceGt`[​](#unshieldedbalancegt "Direct link to unshieldedbalancegt")

Returns true if the unshielded balance of the contract for the given token type is greater than the given value.

```
circuit unshieldedBalanceGt(color: Bytes<32>, amount: Uint<128>): Boolean
```

### `unshieldedBalanceLte`[​](#unshieldedbalancelte "Direct link to unshieldedbalancelte")

Returns true if the unshielded balance of the contract for the given token type is less than or equal to the given value.

```
circuit unshieldedBalanceLte(color: Bytes<32>, amount: Uint<128>): Boolean;
```

### `blockTimeLt`[​](#blocktimelt "Direct link to blocktimelt")

Returns true if the current block time is less than the given value.

```
circuit blockTimeLt(time: Uint<64>): Boolean;
```

### `blockTimeGte`[​](#blocktimegte "Direct link to blocktimegte")

Returns true if the current block time is greater than or equal to the given value.

```
circuit blockTimeGte(time: Uint<64>): Boolean;
```

### `blockTimeGt`[​](#blocktimegt "Direct link to blocktimegt")

Returns true if the current block time is greater than the given value.

```
circuit blockTimeGt(time: Uint<64>): Boolean;
```

### `blockTimeLte`[​](#blocktimelte "Direct link to blocktimelte")

Returns true if the current block time is less than or equal to the given value.

```
circuit blockTimeLte(time: Uint<64>): Boolean;
```

### `serialize<T, #n>`[​](#serializet-n "Direct link to serializet-n")

Returns the canonical byte encoding of for a given value of event type. Note that `serialize` can only be instantiated for an event type and its canonical serialized size.

```
circuit serialize<T, #n> (x: T): Bytes<n>;
```

### `deserialize<T, #n>`[​](#deserializet-n "Direct link to deserializet-n")

Reconstructs a value of type event from its canonical byte encoding. Note that `deserialize` can only be instantiated for an event type and its canonical serialized size.

```
circuit deserialize<T, #n> (x: Bytes<n>): T;
```
