# KeyMaterialProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / KeyMaterialProvider

# Type Alias: KeyMaterialProvider

> **KeyMaterialProvider** = `object`

DApp connector API type for key material retrieval

## Methods[​](#methods "Direct link to Methods")

### getProverKey()[​](#getproverkey "Direct link to getProverKey()")

> **getProverKey**(`circuitKeyLocation`): `Promise`<`Uint8Array`<`ArrayBufferLike`>>

#### Parameters[​](#parameters "Direct link to Parameters")

##### circuitKeyLocation[​](#circuitkeylocation "Direct link to circuitKeyLocation")

`string`

#### Returns[​](#returns "Direct link to Returns")

`Promise`<`Uint8Array`<`ArrayBufferLike`>>

***

### getVerifierKey()[​](#getverifierkey "Direct link to getVerifierKey()")

> **getVerifierKey**(`circuitKeyLocation`): `Promise`<`Uint8Array`<`ArrayBufferLike`>>

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### circuitKeyLocation[​](#circuitkeylocation-1 "Direct link to circuitKeyLocation")

`string`

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`Uint8Array`<`ArrayBufferLike`>>

***

### getZKIR()[​](#getzkir "Direct link to getZKIR()")

> **getZKIR**(`circuitKeyLocation`): `Promise`<`Uint8Array`<`ArrayBufferLike`>>

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### circuitKeyLocation[​](#circuitkeylocation-2 "Direct link to circuitKeyLocation")

`string`

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<`Uint8Array`<`ArrayBufferLike`>>
