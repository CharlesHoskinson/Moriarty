# signData

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / signData

# Function: signData()

```
function signData(key, data): Signature;
```

Signs arbitrary data with the given signing key.

WARNING: Do not expose access to this function for valuable keys for data that is not strictly controlled!

## Parameters[​](#parameters "Direct link to Parameters")

### key[​](#key "Direct link to key")

[`SigningKey`](/api-reference/compact-runtime/type-aliases/SigningKey.md)

### data[​](#data "Direct link to data")

`Uint8Array`

## Returns[​](#returns "Direct link to Returns")

[`Signature`](/api-reference/compact-runtime/type-aliases/Signature.md)
