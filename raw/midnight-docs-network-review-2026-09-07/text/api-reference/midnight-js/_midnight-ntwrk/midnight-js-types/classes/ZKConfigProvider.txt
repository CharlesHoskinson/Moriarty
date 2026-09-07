# ZKConfigProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / ZKConfigProvider

# Abstract Class: ZKConfigProvider\<K>

A provider for zero-knowledge intermediate representations, prover keys, and verifier keys. All three are used by the [ProofProvider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ProofProvider.md) to create a proof for a call transaction. The implementation of this provider depends on the runtime environment, since each environment has different conventions for accessing static artifacts.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### K[​](#k "Direct link to K")

`K` *extends* `string`

The type of the circuit ID used by the provider.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new ZKConfigProvider**<`K`>(): `ZKConfigProvider`<`K`>

#### Returns[​](#returns "Direct link to Returns")

`ZKConfigProvider`<`K`>

## Methods[​](#methods "Direct link to Methods")

### asKeyMaterialProvider()[​](#askeymaterialprovider "Direct link to asKeyMaterialProvider()")

> **asKeyMaterialProvider**(): [`KeyMaterialProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/KeyMaterialProvider.md)

#### Returns[​](#returns-1 "Direct link to Returns")

[`KeyMaterialProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/KeyMaterialProvider.md)

***

### get()[​](#get "Direct link to get()")

> **get**(`circuitId`): `Promise`<[`ZKConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ZKConfig.md)<`K`>>

Retrieves all zero-knowledge artifacts produced by `compactc` compiler for the given circuit.

#### Parameters[​](#parameters "Direct link to Parameters")

##### circuitId[​](#circuitid "Direct link to circuitId")

`K`

The circuit ID of the artifacts to retrieve.

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<[`ZKConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ZKConfig.md)<`K`>>

***

### getProverKey()[​](#getproverkey "Direct link to getProverKey()")

> `abstract` **getProverKey**(`circuitId`): `Promise`<[`ProverKey`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ProverKey.md)>

Retrieves the prover key produced by `compactc` compiler for the given circuit.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### circuitId[​](#circuitid-1 "Direct link to circuitId")

`K`

The circuit ID of the prover key to retrieve.

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<[`ProverKey`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ProverKey.md)>

***

### getVerifierKey()[​](#getverifierkey "Direct link to getVerifierKey()")

> `abstract` **getVerifierKey**(`circuitId`): `Promise`<[`VerifierKey`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/VerifierKey.md)>

Retrieves the verifier key produced by `compactc` compiler for the given circuit.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### circuitId[​](#circuitid-2 "Direct link to circuitId")

`K`

The circuit ID of the verifier key to retrieve.

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<[`VerifierKey`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/VerifierKey.md)>

***

### getVerifierKeys()[​](#getverifierkeys "Direct link to getVerifierKeys()")

> **getVerifierKeys**(`circuitIds`): `Promise`<\[`K`, [`VerifierKey`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/VerifierKey.md)]\[]>

Retrieves the verifier keys produced by `compactc` compiler for the given circuits.

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### circuitIds[​](#circuitids "Direct link to circuitIds")

`K`\[]

The circuit IDs of the verifier keys to retrieve.

#### Returns[​](#returns-5 "Direct link to Returns")

`Promise`<\[`K`, [`VerifierKey`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/VerifierKey.md)]\[]>

***

### getZKIR()[​](#getzkir "Direct link to getZKIR()")

> `abstract` **getZKIR**(`circuitId`): `Promise`<[`ZKIR`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ZKIR.md)>

Retrieves the zero-knowledge intermediate representation produced by `compactc` compiler for the given circuit.

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### circuitId[​](#circuitid-3 "Direct link to circuitId")

`K`

The circuit ID of the ZKIR to retrieve.

#### Returns[​](#returns-6 "Direct link to Returns")

`Promise`<[`ZKIR`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ZKIR.md)>
