# createProvingPayload

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / createProvingPayload

# Function: createProvingPayload()

```
function createProvingPayload(

   serializedPreimage, 

   overwriteBindingInput, 

   keyMaterial?): Uint8Array;
```

Creates a payload for proving a specific proof through the proof server

## Parameters[​](#parameters "Direct link to Parameters")

### serializedPreimage[​](#serializedpreimage "Direct link to serializedPreimage")

`Uint8Array`

### overwriteBindingInput[​](#overwritebindinginput "Direct link to overwriteBindingInput")

`undefined` | `bigint`

### keyMaterial?[​](#keymaterial "Direct link to keyMaterial?")

[`ProvingKeyMaterial`](/api-reference/ledger/type-aliases/ProvingKeyMaterial.md)

## Returns[​](#returns "Direct link to Returns")

`Uint8Array`
