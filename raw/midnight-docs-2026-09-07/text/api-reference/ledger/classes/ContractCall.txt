# ContractCall

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ContractCall

# Class: ContractCall\<P>

A single contract call segment

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

## Properties[​](#properties "Direct link to Properties")

### address[​](#address "Direct link to address")

```
readonly address: string;
```

The address being called

***

### communicationCommitment[​](#communicationcommitment "Direct link to communicationCommitment")

```
readonly communicationCommitment: string;
```

The communication commitment of this call

***

### entryPoint[​](#entrypoint "Direct link to entryPoint")

```
readonly entryPoint: string | Uint8Array<ArrayBufferLike>;
```

The entry point being called

***

### fallibleTranscript[​](#fallibletranscript "Direct link to fallibleTranscript")

```
readonly fallibleTranscript: 

  | undefined

| Transcript<AlignedValue>;
```

The fallible execution stage transcript

***

### guaranteedTranscript[​](#guaranteedtranscript "Direct link to guaranteedTranscript")

```
readonly guaranteedTranscript: 

  | undefined

| Transcript<AlignedValue>;
```

The guaranteed execution stage transcript

***

### proof[​](#proof "Direct link to proof")

```
readonly proof: P;
```

The proof attached to this call

## Methods[​](#methods "Direct link to Methods")

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns "Direct link to Returns")

`string`
