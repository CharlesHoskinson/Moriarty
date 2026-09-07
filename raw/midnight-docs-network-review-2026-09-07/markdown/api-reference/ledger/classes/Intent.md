# Intent

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / Intent

# Class: Intent\<S, P, B>

An intent is a potentially unbalanced partial transaction, that may be combined with other intents to form a whole.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### S[​](#s "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

### B[​](#b "Direct link to B")

`B` *extends* [`Bindingish`](/api-reference/ledger/type-aliases/Bindingish.md)

## Properties[​](#properties "Direct link to Properties")

### actions[​](#actions "Direct link to actions")

```
actions: ContractAction<P>[];
```

The action sequence of this intent.

#### Throws[​](#throws "Direct link to Throws")

Writing throws if `B` is [Binding](/api-reference/ledger/classes/Binding.md).

***

### binding[​](#binding "Direct link to binding")

```
readonly binding: B;
```

***

### dustActions[​](#dustactions "Direct link to dustActions")

```
dustActions: undefined | DustActions<S, P>;
```

The DUST interactions made by this intent

#### Throws[​](#throws-1 "Direct link to Throws")

Writing throws if `B` is [Binding](/api-reference/ledger/classes/Binding.md).

***

### fallibleUnshieldedOffer[​](#fallibleunshieldedoffer "Direct link to fallibleUnshieldedOffer")

```
fallibleUnshieldedOffer: undefined | UnshieldedOffer<S>;
```

The UTXO inputs and outputs in the fallible section of this intent.

#### Throws[​](#throws-2 "Direct link to Throws")

Writing throws if `B` is [Binding](/api-reference/ledger/classes/Binding.md), unless the only change is in the signature set.

***

### guaranteedUnshieldedOffer[​](#guaranteedunshieldedoffer "Direct link to guaranteedUnshieldedOffer")

```
guaranteedUnshieldedOffer: undefined | UnshieldedOffer<S>;
```

The UTXO inputs and outputs in the guaranteed section of this intent.

#### Throws[​](#throws-3 "Direct link to Throws")

Writing throws if `B` is [Binding](/api-reference/ledger/classes/Binding.md), unless the only change is in the signature set.

***

### ttl[​](#ttl "Direct link to ttl")

```
ttl: Date;
```

The time this intent expires.

#### Throws[​](#throws-4 "Direct link to Throws")

Writing throws if `B` is [Binding](/api-reference/ledger/classes/Binding.md).

## Methods[​](#methods "Direct link to Methods")

### addCall()[​](#addcall "Direct link to addCall()")

```
addCall(call): Intent<S, PreProof, PreBinding>;
```

Adds a contract call to this intent.

#### Parameters[​](#parameters "Direct link to Parameters")

##### call[​](#call "Direct link to call")

[`ContractCallPrototype`](/api-reference/ledger/classes/ContractCallPrototype.md)

#### Returns[​](#returns "Direct link to Returns")

`Intent`<`S`, [`PreProof`](/api-reference/ledger/classes/PreProof.md), [`PreBinding`](/api-reference/ledger/classes/PreBinding.md)>

***

### addDeploy()[​](#adddeploy "Direct link to addDeploy()")

```
addDeploy(deploy): Intent<S, PreProof, PreBinding>;
```

Adds a contract deploy to this intent.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### deploy[​](#deploy "Direct link to deploy")

[`ContractDeploy`](/api-reference/ledger/classes/ContractDeploy.md)

#### Returns[​](#returns-1 "Direct link to Returns")

`Intent`<`S`, [`PreProof`](/api-reference/ledger/classes/PreProof.md), [`PreBinding`](/api-reference/ledger/classes/PreBinding.md)>

***

### addMaintenanceUpdate()[​](#addmaintenanceupdate "Direct link to addMaintenanceUpdate()")

```
addMaintenanceUpdate(update): Intent<S, PreProof, PreBinding>;
```

Adds a maintenance update to this intent.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### update[​](#update "Direct link to update")

[`MaintenanceUpdate`](/api-reference/ledger/classes/MaintenanceUpdate.md)

#### Returns[​](#returns-2 "Direct link to Returns")

`Intent`<`S`, [`PreProof`](/api-reference/ledger/classes/PreProof.md), [`PreBinding`](/api-reference/ledger/classes/PreBinding.md)>

***

### bind()[​](#bind "Direct link to bind()")

```
bind(segmentId): Intent<S, P, Binding>;
```

Enforces binding for this intent. This is irreversible.

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### segmentId[​](#segmentid "Direct link to segmentId")

`number`

#### Returns[​](#returns-3 "Direct link to Returns")

`Intent`<`S`, `P`, [`Binding`](/api-reference/ledger/classes/Binding.md)>

#### Throws[​](#throws-5 "Direct link to Throws")

If `segmentId` is not a valid segment ID.

***

### eraseProofs()[​](#eraseproofs "Direct link to eraseProofs()")

```
eraseProofs(): Intent<S, NoProof, NoBinding>;
```

Removes proofs from this intent.

#### Returns[​](#returns-4 "Direct link to Returns")

`Intent`<`S`, [`NoProof`](/api-reference/ledger/classes/NoProof.md), [`NoBinding`](/api-reference/ledger/classes/NoBinding.md)>

***

### eraseSignatures()[​](#erasesignatures "Direct link to eraseSignatures()")

```
eraseSignatures(): Intent<SignatureErased, P, B>;
```

Removes signatures from this intent.

#### Returns[​](#returns-5 "Direct link to Returns")

`Intent`<[`SignatureErased`](/api-reference/ledger/classes/SignatureErased.md), `P`, `B`>

***

### intentHash()[​](#intenthash "Direct link to intentHash()")

```
intentHash(segmentId): string;
```

Returns the hash of this intent, for it's given segment ID.

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### segmentId[​](#segmentid-1 "Direct link to segmentId")

`number`

#### Returns[​](#returns-6 "Direct link to Returns")

`string`

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-7 "Direct link to Returns")

`Uint8Array`

***

### signatureData()[​](#signaturedata "Direct link to signatureData()")

```
signatureData(segmentId): Uint8Array;
```

The raw data that is signed for unshielded inputs in this intent.

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### segmentId[​](#segmentid-2 "Direct link to segmentId")

`number`

#### Returns[​](#returns-8 "Direct link to Returns")

`Uint8Array`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-6 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-9 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize<S, P, B>(

   markerS, 

   markerP, 

   markerB, 

raw): Intent<S, P, B>;
```

#### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

##### S[​](#s-1 "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

##### P[​](#p-1 "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

##### B[​](#b-1 "Direct link to B")

`B` *extends* [`Bindingish`](/api-reference/ledger/type-aliases/Bindingish.md)

#### Parameters[​](#parameters-7 "Direct link to Parameters")

##### markerS[​](#markers "Direct link to markerS")

`S`\[`"instance"`]

##### markerP[​](#markerp "Direct link to markerP")

`P`\[`"instance"`]

##### markerB[​](#markerb "Direct link to markerB")

`B`\[`"instance"`]

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-10 "Direct link to Returns")

`Intent`<`S`, `P`, `B`>

***

### new()[​](#new "Direct link to new()")

```
static new(ttl): UnprovenIntent;
```

#### Parameters[​](#parameters-8 "Direct link to Parameters")

##### ttl[​](#ttl-1 "Direct link to ttl")

`Date`

#### Returns[​](#returns-11 "Direct link to Returns")

[`UnprovenIntent`](/api-reference/ledger/type-aliases/UnprovenIntent.md)
