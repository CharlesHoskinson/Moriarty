# DustRegistration

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / DustRegistration

# Class: DustRegistration\<S>

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### S[​](#s "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new DustRegistration<S>(

   markerS, 

   nightKey, 

   dustAddress, 

   allowFeePayment, 

signature?): DustRegistration<S>;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### markerS[​](#markers "Direct link to markerS")

`S`\[`"instance"`]

##### nightKey[​](#nightkey "Direct link to nightKey")

`string`

##### dustAddress[​](#dustaddress "Direct link to dustAddress")

`undefined` | `bigint`

##### allowFeePayment[​](#allowfeepayment "Direct link to allowFeePayment")

`bigint`

##### signature?[​](#signature "Direct link to signature?")

`S`

#### Returns[​](#returns "Direct link to Returns")

`DustRegistration`<`S`>

## Properties[​](#properties "Direct link to Properties")

### allowFeePayment[​](#allowfeepayment-1 "Direct link to allowFeePayment")

```
allowFeePayment: bigint;
```

***

### dustAddress[​](#dustaddress-1 "Direct link to dustAddress")

```
dustAddress: undefined | bigint;
```

***

### nightKey[​](#nightkey-1 "Direct link to nightKey")

```
nightKey: string;
```

***

### signature[​](#signature-1 "Direct link to signature")

```
signature: S;
```

## Methods[​](#methods "Direct link to Methods")

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-1 "Direct link to Returns")

`Uint8Array`

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

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize<S>(markerS, raw): DustRegistration<S>;
```

#### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

##### S[​](#s-1 "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### markerS[​](#markers-1 "Direct link to markerS")

`S`\[`"instance"`]

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-3 "Direct link to Returns")

`DustRegistration`<`S`>
