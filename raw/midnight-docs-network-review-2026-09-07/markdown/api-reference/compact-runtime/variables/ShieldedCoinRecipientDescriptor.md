# ShieldedCoinRecipientDescriptor

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.16.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / ShieldedCoinRecipientDescriptor

# Variable: ShieldedCoinRecipientDescriptor

```
const ShieldedCoinRecipientDescriptor: {

  alignment: Alignment;

  fromValue: {

     is_left: boolean;

     left: {

        bytes: Uint8Array;

     };

     right: {

        bytes: Uint8Array;

     };

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

  is_left: boolean;

  left: {

     bytes: Uint8Array;

  };

  right: {

     bytes: Uint8Array;

  };

};
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### value[​](#value "Direct link to value")

[`Value`](/api-reference/compact-runtime/type-aliases/Value.md)

#### Returns[​](#returns-1 "Direct link to Returns")

```
{

  is_left: boolean;

  left: {

     bytes: Uint8Array;

  };

  right: {

     bytes: Uint8Array;

  };

}
```

##### is\_left[​](#is_left "Direct link to is_left")

```
is_left: boolean;
```

##### left[​](#left "Direct link to left")

```
left: {

  bytes: Uint8Array;

};
```

###### left.bytes[​](#leftbytes "Direct link to left.bytes")

```
bytes: Uint8Array;
```

##### right[​](#right "Direct link to right")

```
right: {

  bytes: Uint8Array;

};
```

###### right.bytes[​](#rightbytes "Direct link to right.bytes")

```
bytes: Uint8Array;
```

### toValue()[​](#tovalue "Direct link to toValue()")

```
toValue(value): Value;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### value[​](#value-1 "Direct link to value")

###### is\_left[​](#is_left-1 "Direct link to is_left")

`boolean`

###### left[​](#left-1 "Direct link to left")

{ `bytes`: `Uint8Array`; }

###### left.bytes[​](#leftbytes-1 "Direct link to left.bytes")

`Uint8Array`

###### right[​](#right-1 "Direct link to right")

{ `bytes`: `Uint8Array`; }

###### right.bytes[​](#rightbytes-1 "Direct link to right.bytes")

`Uint8Array`

#### Returns[​](#returns-2 "Direct link to Returns")

[`Value`](/api-reference/compact-runtime/type-aliases/Value.md)
