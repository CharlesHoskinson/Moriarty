# sampleSigningKey

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / sampleSigningKey

# Function: sampleSigningKey()

```
function sampleSigningKey(kind?): SigningKey;
```

Randomly samples a [SigningKey](/api-reference/compact-runtime/type-aliases/SigningKey.md). If `kind` is not supplied, assumes `schnorr`.

## Parameters[​](#parameters "Direct link to Parameters")

### kind?[​](#kind "Direct link to kind?")

`SignatureKind`

## Returns[​](#returns "Direct link to Returns")

[`SigningKey`](/api-reference/compact-runtime/type-aliases/SigningKey.md)
