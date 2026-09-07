# createProvingTransactionPayload

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / createProvingTransactionPayload

# Function: ~~createProvingTransactionPayload()~~

```
function createProvingTransactionPayload(transaction, proving_data): Uint8Array;
```

Creates a payload for proving a specific transaction through the proof server

## Parameters[​](#parameters "Direct link to Parameters")

### transaction[​](#transaction "Direct link to transaction")

[`UnprovenTransaction`](/api-reference/ledger/type-aliases/UnprovenTransaction.md)

### proving\_data[​](#proving_data "Direct link to proving_data")

`Map`<`string`, [`ProvingKeyMaterial`](/api-reference/ledger/type-aliases/ProvingKeyMaterial.md)>

## Returns[​](#returns "Direct link to Returns")

`Uint8Array`

## Deprecated[​](#deprecated "Direct link to Deprecated")

Use `Transaction.prove` instead.
