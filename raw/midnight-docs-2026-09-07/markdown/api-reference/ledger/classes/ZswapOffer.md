# ZswapOffer

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ZswapOffer

# Class: ZswapOffer\<P>

A full Zswap offer; the zswap part of a transaction

Consists of sets of [ZswapInput](/api-reference/ledger/classes/ZswapInput.md)s, [ZswapOutput](/api-reference/ledger/classes/ZswapOutput.md)s, and [ZswapTransient](/api-reference/ledger/classes/ZswapTransient.md)s, as well as a [deltas](#deltas) vector of the transaction value

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

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
readonly inputs: ZswapInput<P>[];
```

The inputs this offer is composed of

***

### outputs[​](#outputs "Direct link to outputs")

```
readonly outputs: ZswapOutput<P>[];
```

The outputs this offer is composed of

***

### transients[​](#transients "Direct link to transients")

```
readonly transients: ZswapTransient<P>[];
```

The transients this offer is composed of

## Methods[​](#methods "Direct link to Methods")

### merge()[​](#merge "Direct link to merge()")

```
merge(other): ZswapOffer<P>;
```

Combine this offer with another

#### Parameters[​](#parameters "Direct link to Parameters")

##### other[​](#other "Direct link to other")

`ZswapOffer`<`P`>

#### Returns[​](#returns "Direct link to Returns")

`ZswapOffer`<`P`>

***

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
static deserialize<P>(markerP, raw): ZswapOffer<P>;
```

#### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

##### P[​](#p-1 "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### markerP[​](#markerp "Direct link to markerP")

`P`\[`"instance"`]

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-3 "Direct link to Returns")

`ZswapOffer`<`P`>

***

### fromInput()[​](#frominput "Direct link to fromInput()")

```
static fromInput<P>(

   input, 

   type_?, 

value?): ZswapOffer<P>;
```

Creates a singleton offer, from an [ZswapInput](/api-reference/ledger/classes/ZswapInput.md) and its value vector

The `type_` and `value` parameters are deprecated and will be ignored.

#### Type Parameters[​](#type-parameters-2 "Direct link to Type Parameters")

##### P[​](#p-2 "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### input[​](#input "Direct link to input")

[`ZswapInput`](/api-reference/ledger/classes/ZswapInput.md)<`P`>

##### type\_?[​](#type_ "Direct link to type_?")

`string`

##### value?[​](#value "Direct link to value?")

`bigint`

#### Returns[​](#returns-4 "Direct link to Returns")

`ZswapOffer`<`P`>

***

### fromOutput()[​](#fromoutput "Direct link to fromOutput()")

```
static fromOutput<P>(

   output, 

   type_?, 

value?): ZswapOffer<P>;
```

Creates a singleton offer, from an [ZswapOutput](/api-reference/ledger/classes/ZswapOutput.md) and its value vector

The `type_` and `value` parameters are deprecated and will be ignored.

#### Type Parameters[​](#type-parameters-3 "Direct link to Type Parameters")

##### P[​](#p-3 "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### output[​](#output "Direct link to output")

[`ZswapOutput`](/api-reference/ledger/classes/ZswapOutput.md)<`P`>

##### type\_?[​](#type_-1 "Direct link to type_?")

`string`

##### value?[​](#value-1 "Direct link to value?")

`bigint`

#### Returns[​](#returns-5 "Direct link to Returns")

`ZswapOffer`<`P`>

***

### fromTransient()[​](#fromtransient "Direct link to fromTransient()")

```
static fromTransient<P>(transient): ZswapOffer<P>;
```

Creates a singleton offer, from a [ZswapTransient](/api-reference/ledger/classes/ZswapTransient.md)

#### Type Parameters[​](#type-parameters-4 "Direct link to Type Parameters")

##### P[​](#p-4 "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### transient[​](#transient "Direct link to transient")

[`ZswapTransient`](/api-reference/ledger/classes/ZswapTransient.md)<`P`>

#### Returns[​](#returns-6 "Direct link to Returns")

`ZswapOffer`<`P`>
