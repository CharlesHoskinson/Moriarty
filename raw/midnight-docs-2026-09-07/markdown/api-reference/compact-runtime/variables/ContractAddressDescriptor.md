# ContractAddressDescriptor

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.16.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / ContractAddressDescriptor

# Variable: ContractAddressDescriptor

```
const ContractAddressDescriptor: {

  alignment: Alignment;

  fromValue: {

     bytes: Uint8Array;

  };

  toValue: Value;

};
```

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### alignment()[​](#alignment "Direct link to alignment()")

```
alignment(): Alignment;
```

#### Returns[​](#returns "Direct link to Returns")

[`Alignment`](/api-reference/compact-runtime/type-aliases/Alignment.md)

### fromValue()[​](#fromvalue "Direct link to fromValue()")

```
fromValue(value): {

  bytes: Uint8Array;

};
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### value[​](#value "Direct link to value")

[`Value`](/api-reference/compact-runtime/type-aliases/Value.md)

#### Returns[​](#returns-1 "Direct link to Returns")

```
{

  bytes: Uint8Array;

}
```

##### bytes[​](#bytes "Direct link to bytes")

```
bytes: Uint8Array;
```

### toValue()[​](#tovalue "Direct link to toValue()")

```
toValue(value): Value;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### value[​](#value-1 "Direct link to value")

###### bytes[​](#bytes-1 "Direct link to bytes")

`Uint8Array`

#### Returns[​](#returns-2 "Direct link to Returns")

[`Value`](/api-reference/compact-runtime/type-aliases/Value.md)
