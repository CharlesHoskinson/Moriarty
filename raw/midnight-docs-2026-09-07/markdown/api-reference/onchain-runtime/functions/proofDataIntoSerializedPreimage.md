# proofDataIntoSerializedPreimage

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / proofDataIntoSerializedPreimage

# Function: proofDataIntoSerializedPreimage()

```
function proofDataIntoSerializedPreimage(

   input, 

   output, 

   public_transcript, 

   private_transcript_outputs, 

   key_location?): Uint8Array
```

Converts input, output, and transcript information into a proof preimage suitable to pass to a `ProvingProvider`.

The `key_location` parameter is a string used to identify the circuit by proving machinery, for backwards-compatibility, if unset it defaults to `'dummy'`.

## Parameters[​](#parameters "Direct link to Parameters")

### input[​](#input "Direct link to input")

[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)

### output[​](#output "Direct link to output")

[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)

### public\_transcript[​](#public_transcript "Direct link to public_transcript")

[`Op`](/api-reference/onchain-runtime/type-aliases/Op.md)<[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)>\[]

### private\_transcript\_outputs[​](#private_transcript_outputs "Direct link to private_transcript_outputs")

[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)\[]

### key\_location?[​](#key_location "Direct link to key_location?")

`string`

## Returns[​](#returns "Direct link to Returns")

`Uint8Array`
