# KeyMaterialProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/dapp-connector-api v4.0.1**](/api-reference/dapp-connector.md)

***

[@midnight-ntwrk/dapp-connector-api](/api-reference/dapp-connector/globals.md) / KeyMaterialProvider

# Type Alias: KeyMaterialProvider

> **KeyMaterialProvider** = `object`

Object resolving prover and verifier keys, as well as the ZKIR representation of the circuit. It is almost identical to the one in Midnight.js's `ZKConfigProvider` (<https://github.com/midnightntwrk/midnight-js/blob/main/packages/types/src/zk-config-provider.ts#L25>)

It has separate methods for getting the ZKIR, prover key and verifier key to allow for caching of the keys and to avoid loading the prover key into memory when it is not needed.

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
