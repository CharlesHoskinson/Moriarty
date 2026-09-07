# CoinInfoDescriptor

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.9.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / CoinInfoDescriptor

# Variable: CoinInfoDescriptor

```
const CoinInfoDescriptor: {

  alignment: Alignment;

  fromValue: {

     color: Uint8Array;

     nonce: Uint8Array;

     value: bigint;

  };

  toValue: Value;

};
```

## Type declaration[​](#type-declaration "Direct link to Type declaration")

### alignment()[​](#alignment "Direct link to alignment()")

```
alignment(): Alignment;
```

#### Returns[​](#returns "Direct link to Returns")

[`Alignment`](/api-reference/compact-runtime/type-aliases/Alignment.md)

### fromValue()[​](#fromvalue "Direct link to fromValue()")

```
fromValue(value): {

  color: Uint8Array;

  nonce: Uint8Array;

  value: bigint;

};
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### value[​](#value "Direct link to value")

[`Value`](/api-reference/compact-runtime/type-aliases/Value.md)

#### Returns[​](#returns-1 "Direct link to Returns")

```
{

  color: Uint8Array;

  nonce: Uint8Array;

  value: bigint;

}
```

##### color[​](#color "Direct link to color")

```
color: Uint8Array;
```

##### nonce[​](#nonce "Direct link to nonce")

```
nonce: Uint8Array;
```

##### value[​](#value-1 "Direct link to value")

```
value: bigint;
```

### toValue()[​](#tovalue "Direct link to toValue()")

```
toValue(value): Value;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### value[​](#value-2 "Direct link to value")

###### color[​](#color-1 "Direct link to color")

`Uint8Array`

###### nonce[​](#nonce-1 "Direct link to nonce")

`Uint8Array`

###### value[​](#value-3 "Direct link to value")

`bigint`

#### Returns[​](#returns-2 "Direct link to Returns")

[`Value`](/api-reference/compact-runtime/type-aliases/Value.md)
