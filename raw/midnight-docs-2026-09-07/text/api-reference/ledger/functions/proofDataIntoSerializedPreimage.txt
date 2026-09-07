# proofDataIntoSerializedPreimage

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / proofDataIntoSerializedPreimage

# Function: proofDataIntoSerializedPreimage()

```
function proofDataIntoSerializedPreimage(

   input, 

   output, 

   public_transcript, 

   private_transcript_outputs, 

   key_location?): Uint8Array;
```

Converts input, output, and transcript information into a proof preimage suitable to pass to a `ProvingProvider`.

The `key_location` parameter is a string used to identify the circuit by proving machinery, for backwards-compatibility, if unset it defaults to `'dummy'`.

## Parameters[​](#parameters "Direct link to Parameters")

### input[​](#input "Direct link to input")

[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)

### output[​](#output "Direct link to output")

[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)

### public\_transcript[​](#public_transcript "Direct link to public_transcript")

[`Op`](/api-reference/ledger/type-aliases/Op.md)<[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)>\[]

### private\_transcript\_outputs[​](#private_transcript_outputs "Direct link to private_transcript_outputs")

[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)\[]

### key\_location?[​](#key_location "Direct link to key_location?")

`string`

## Returns[​](#returns "Direct link to Returns")

`Uint8Array`
