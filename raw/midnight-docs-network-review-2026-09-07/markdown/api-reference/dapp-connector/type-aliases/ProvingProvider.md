# ProvingProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/dapp-connector-api v4.0.1**](/api-reference/dapp-connector.md)

***

[@midnight-ntwrk/dapp-connector-api](/api-reference/dapp-connector/globals.md) / ProvingProvider

# Type Alias: ProvingProvider

> **ProvingProvider** = `object`

Object abstracting the proving functionality It is compatible with Ledger's ProvingProvider (<https://github.com/midnightntwrk/midnight-ledger/blob/main/ledger-wasm/ledger-v6.template.d.ts#L992>)

## Methods[​](#methods "Direct link to Methods")

### check()[​](#check "Direct link to check()")

> **check**(`serializedPreimage`, `keyLocation`): `Promise`<(`bigint` | `undefined`)\[]>

#### Parameters[​](#parameters "Direct link to Parameters")

##### serializedPreimage[​](#serializedpreimage "Direct link to serializedPreimage")

`Uint8Array`

##### keyLocation[​](#keylocation "Direct link to keyLocation")

`string`

#### Returns[​](#returns "Direct link to Returns")

`Promise`<(`bigint` | `undefined`)\[]>

***

### prove()[​](#prove "Direct link to prove()")

> **prove**(`serializedPreimage`, `keyLocation`, `overwriteBindingInput?`): `Promise`<`Uint8Array`<`ArrayBufferLike`>>

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### serializedPreimage[​](#serializedpreimage-1 "Direct link to serializedPreimage")

`Uint8Array`

##### keyLocation[​](#keylocation-1 "Direct link to keyLocation")

`string`

##### overwriteBindingInput?[​](#overwritebindinginput "Direct link to overwriteBindingInput?")

`bigint`

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`Uint8Array`<`ArrayBufferLike`>>
