# UnshieldedOffer

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / UnshieldedOffer

# Class: UnshieldedOffer\<S>

An unshielded offer consists of inputs, outputs, and signatures that authorize the inputs. The data the signatures sign is provided by [Intent.signatureData](/api-reference/ledger/classes/Intent.md#signaturedata).

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### S[​](#s "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

## Properties[​](#properties "Direct link to Properties")

### inputs[​](#inputs "Direct link to inputs")

```
readonly inputs: UtxoSpend[];
```

***

### outputs[​](#outputs "Direct link to outputs")

```
readonly outputs: UtxoOutput[];
```

***

### signatures[​](#signatures "Direct link to signatures")

```
readonly signatures: string[];
```

## Methods[​](#methods "Direct link to Methods")

### addSignatures()[​](#addsignatures "Direct link to addSignatures()")

```
addSignatures(signatures): UnshieldedOffer<S>;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### signatures[​](#signatures-1 "Direct link to signatures")

`string`\[]

#### Returns[​](#returns "Direct link to Returns")

`UnshieldedOffer`<`S`>

***

### eraseSignatures()[​](#erasesignatures "Direct link to eraseSignatures()")

```
eraseSignatures(): UnshieldedOffer<SignatureErased>;
```

#### Returns[​](#returns-1 "Direct link to Returns")

`UnshieldedOffer`<[`SignatureErased`](/api-reference/ledger/classes/SignatureErased.md)>

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

***

### new()[​](#new "Direct link to new()")

```
static new(

   inputs, 

   outputs, 

signatures): UnshieldedOffer<SignatureEnabled>;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### inputs[​](#inputs-1 "Direct link to inputs")

[`UtxoSpend`](/api-reference/ledger/type-aliases/UtxoSpend.md)\[]

##### outputs[​](#outputs-1 "Direct link to outputs")

[`UtxoOutput`](/api-reference/ledger/type-aliases/UtxoOutput.md)\[]

##### signatures[​](#signatures-2 "Direct link to signatures")

`string`\[]

#### Returns[​](#returns-3 "Direct link to Returns")

`UnshieldedOffer`<[`SignatureEnabled`](/api-reference/ledger/classes/SignatureEnabled.md)>
