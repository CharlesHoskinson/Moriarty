# zkConfigToProvingKeyMaterial

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / zkConfigToProvingKeyMaterial

# Function: zkConfigToProvingKeyMaterial()

> **zkConfigToProvingKeyMaterial**<`K`>(`zkConfig`): `object`

Converts a ZKConfig object to ProvingKeyMaterial format.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### K[​](#k "Direct link to K")

`K` *extends* `string`

## Parameters[​](#parameters "Direct link to Parameters")

### zkConfig[​](#zkconfig "Direct link to zkConfig")

[`ZKConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ZKConfig.md)<`K`>

## Returns[​](#returns "Direct link to Returns")

`object`

### ir[​](#ir "Direct link to ir")

> **ir**: [`ZKIR`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ZKIR.md) = `zkConfig.zkir`

### proverKey[​](#proverkey "Direct link to proverKey")

> **proverKey**: [`ProverKey`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ProverKey.md) = `zkConfig.proverKey`

### verifierKey[​](#verifierkey "Direct link to verifierKey")

> **verifierKey**: [`VerifierKey`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/VerifierKey.md) = `zkConfig.verifierKey`
