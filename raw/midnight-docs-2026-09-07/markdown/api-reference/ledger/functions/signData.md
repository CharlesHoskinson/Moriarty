# signData

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / signData

# Function: signData()

```
function signData(key, data): string;
```

Signs arbitrary data with the given signing key.

WARNING: Do not expose access to this function for valuable keys for data that is not strictly controlled!

## Parameters[​](#parameters "Direct link to Parameters")

### key[​](#key "Direct link to key")

`string`

### data[​](#data "Direct link to data")

`Uint8Array`

## Returns[​](#returns "Direct link to Returns")

`string`
