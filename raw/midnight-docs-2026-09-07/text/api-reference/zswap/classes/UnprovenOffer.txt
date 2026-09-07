# UnprovenOffer

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / UnprovenOffer

# Class: UnprovenOffer

A [Offer](/api-reference/zswap/classes/Offer.md), prior to being proven

All "shielded" information in the offer can still be extracted at this stage!

## Constructors[​](#constructors "Direct link to Constructors")

### new UnprovenOffer()[​](#new-unprovenoffer "Direct link to new UnprovenOffer()")

```
new UnprovenOffer(): UnprovenOffer
```

#### Returns[​](#returns "Direct link to Returns")

[`UnprovenOffer`](/api-reference/zswap/classes/UnprovenOffer.md)

## Properties[​](#properties "Direct link to Properties")

### deltas[​](#deltas "Direct link to deltas")

```
readonly deltas: Map<string, bigint>;
```

The value of this offer for each token type; note that this may be negative

This is input coin values - output coin values, for value vectors

***

### inputs[​](#inputs "Direct link to inputs")

```
readonly inputs: UnprovenInput[];
```

The inputs this offer is composed of

***

### outputs[​](#outputs "Direct link to outputs")

```
readonly outputs: UnprovenOutput[];
```

The outputs this offer is composed of

***

### transient[​](#transient "Direct link to transient")

```
readonly transient: UnprovenTransient[];
```

The transients this offer is composed of

## Methods[​](#methods "Direct link to Methods")

### merge()[​](#merge "Direct link to merge()")

```
merge(other): UnprovenOffer
```

Combine this offer with another

#### Parameters[​](#parameters "Direct link to Parameters")

##### other[​](#other "Direct link to other")

[`UnprovenOffer`](/api-reference/zswap/classes/UnprovenOffer.md)

#### Returns[​](#returns-1 "Direct link to Returns")

[`UnprovenOffer`](/api-reference/zswap/classes/UnprovenOffer.md)

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(netid): Uint8Array<ArrayBufferLike>
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### netid[​](#netid "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-2 "Direct link to Returns")

`Uint8Array`<`ArrayBufferLike`>

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-3 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw, netid): UnprovenOffer
```

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-4 "Direct link to Returns")

[`UnprovenOffer`](/api-reference/zswap/classes/UnprovenOffer.md)

***

### fromInput()[​](#frominput "Direct link to fromInput()")

```
static fromInput(

   input, 

   type_, 

   value): UnprovenOffer
```

Creates a singleton offer, from an [UnprovenInput](/api-reference/zswap/classes/UnprovenInput.md) and its value vector

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### input[​](#input "Direct link to input")

[`UnprovenInput`](/api-reference/zswap/classes/UnprovenInput.md)

##### type\_[​](#type_ "Direct link to type_")

`string`

##### value[​](#value "Direct link to value")

`bigint`

#### Returns[​](#returns-5 "Direct link to Returns")

[`UnprovenOffer`](/api-reference/zswap/classes/UnprovenOffer.md)

***

### fromOutput()[​](#fromoutput "Direct link to fromOutput()")

```
static fromOutput(

   output, 

   type_, 

   value): UnprovenOffer
```

Creates a singleton offer, from an [UnprovenOutput](/api-reference/zswap/classes/UnprovenOutput.md) and its value vector

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### output[​](#output "Direct link to output")

[`UnprovenOutput`](/api-reference/zswap/classes/UnprovenOutput.md)

##### type\_[​](#type_-1 "Direct link to type_")

`string`

##### value[​](#value-1 "Direct link to value")

`bigint`

#### Returns[​](#returns-6 "Direct link to Returns")

[`UnprovenOffer`](/api-reference/zswap/classes/UnprovenOffer.md)

***

### fromTransient()[​](#fromtransient "Direct link to fromTransient()")

```
static fromTransient(transient): UnprovenOffer
```

Creates a singleton offer, from an [UnprovenTransient](/api-reference/zswap/classes/UnprovenTransient.md)

#### Parameters[​](#parameters-6 "Direct link to Parameters")

##### transient[​](#transient-1 "Direct link to transient")

[`UnprovenTransient`](/api-reference/zswap/classes/UnprovenTransient.md)

#### Returns[​](#returns-7 "Direct link to Returns")

[`UnprovenOffer`](/api-reference/zswap/classes/UnprovenOffer.md)
