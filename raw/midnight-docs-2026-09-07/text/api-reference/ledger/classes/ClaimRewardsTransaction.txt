# ClaimRewardsTransaction

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ClaimRewardsTransaction

# Class: ClaimRewardsTransaction\<S>

A request to allocate rewards, authorized by the reward's recipient

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### S[​](#s "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new ClaimRewardsTransaction<S>(

   markerS, 

   network_id, 

   value, 

   owner, 

   nonce, 

   signature, 

kind?): ClaimRewardsTransaction<S>;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### markerS[​](#markers "Direct link to markerS")

`S`\[`"instance"`]

##### network\_id[​](#network_id "Direct link to network_id")

`string`

##### value[​](#value "Direct link to value")

`bigint`

##### owner[​](#owner "Direct link to owner")

`string`

##### nonce[​](#nonce "Direct link to nonce")

`string`

##### signature[​](#signature "Direct link to signature")

`S`

##### kind?[​](#kind "Direct link to kind?")

[`ClaimKind`](/api-reference/ledger/type-aliases/ClaimKind.md)

#### Returns[​](#returns "Direct link to Returns")

`ClaimRewardsTransaction`<`S`>

## Properties[​](#properties "Direct link to Properties")

### dataToSign[​](#datatosign "Direct link to dataToSign")

```
readonly dataToSign: Uint8Array;
```

The raw data any valid signature must be over to approve this transaction.

***

### kind[​](#kind-1 "Direct link to kind")

```
readonly kind: ClaimKind;
```

The kind of claim being made, either a `Reward` or a `CardanoBridge` claim.

***

### nonce[​](#nonce-1 "Direct link to nonce")

```
readonly nonce: string;
```

The rewarded coin's randomness, preventing it from colliding with other coins.

***

### owner[​](#owner-1 "Direct link to owner")

```
readonly owner: string;
```

The signing key owning this coin.

***

### signature[​](#signature-1 "Direct link to signature")

```
readonly signature: S;
```

The signature on this request.

***

### value[​](#value-1 "Direct link to value")

```
readonly value: bigint;
```

The rewarded coin's value, in atomic units dependent on the currency

Bounded to be a non-negative 64-bit integer

## Methods[​](#methods "Direct link to Methods")

### addSignature()[​](#addsignature "Direct link to addSignature()")

```
addSignature(signature): ClaimRewardsTransaction<SignatureEnabled>;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### signature[​](#signature-2 "Direct link to signature")

`string`

#### Returns[​](#returns-1 "Direct link to Returns")

`ClaimRewardsTransaction`<[`SignatureEnabled`](/api-reference/ledger/classes/SignatureEnabled.md)>

***

### eraseSignatures()[​](#erasesignatures "Direct link to eraseSignatures()")

```
eraseSignatures(): ClaimRewardsTransaction<SignatureErased>;
```

#### Returns[​](#returns-2 "Direct link to Returns")

`ClaimRewardsTransaction`<[`SignatureErased`](/api-reference/ledger/classes/SignatureErased.md)>

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-3 "Direct link to Returns")

`Uint8Array`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-4 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize<S>(markerS, raw): ClaimRewardsTransaction<S>;
```

#### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

##### S[​](#s-1 "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### markerS[​](#markers-1 "Direct link to markerS")

`S`\[`"instance"`]

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-5 "Direct link to Returns")

`ClaimRewardsTransaction`<`S`>

***

### new()[​](#new "Direct link to new()")

```
static new(

   network_id, 

   value, 

   owner, 

   nonce, 

kind): ClaimRewardsTransaction<SignatureErased>;
```

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### network\_id[​](#network_id-1 "Direct link to network_id")

`string`

##### value[​](#value-2 "Direct link to value")

`bigint`

##### owner[​](#owner-2 "Direct link to owner")

`string`

##### nonce[​](#nonce-2 "Direct link to nonce")

`string`

##### kind[​](#kind-2 "Direct link to kind")

[`ClaimKind`](/api-reference/ledger/type-aliases/ClaimKind.md)

#### Returns[​](#returns-6 "Direct link to Returns")

`ClaimRewardsTransaction`<[`SignatureErased`](/api-reference/ledger/classes/SignatureErased.md)>
