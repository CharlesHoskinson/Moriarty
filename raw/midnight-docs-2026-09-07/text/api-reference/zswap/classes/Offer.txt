# Offer

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / Offer

# Class: Offer

A full Zswap offer; the zswap part of a transaction

Consists of sets of [Input](/api-reference/zswap/classes/Input.md)s, [Output](/api-reference/zswap/classes/Output.md)s, and [Transient](/api-reference/zswap/classes/Transient.md)s, as well as a [deltas](/api-reference/zswap/classes/Offer.md#deltas) vector of the transaction value

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
readonly inputs: Input[];
```

The inputs this offer is composed of

***

### outputs[​](#outputs "Direct link to outputs")

```
readonly outputs: Output[];
```

The outputs this offer is composed of

***

### transient[​](#transient "Direct link to transient")

```
readonly transient: Transient[];
```

The transients this offer is composed of

## Methods[​](#methods "Direct link to Methods")

### merge()[​](#merge "Direct link to merge()")

```
merge(other): Offer
```

Combine this offer with another

#### Parameters[​](#parameters "Direct link to Parameters")

##### other[​](#other "Direct link to other")

[`Offer`](/api-reference/zswap/classes/Offer.md)

#### Returns[​](#returns "Direct link to Returns")

[`Offer`](/api-reference/zswap/classes/Offer.md)

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(netid): Uint8Array<ArrayBufferLike>
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### netid[​](#netid "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-1 "Direct link to Returns")

`Uint8Array`<`ArrayBufferLike`>

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw, netid): Offer
```

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`Offer`](/api-reference/zswap/classes/Offer.md)
