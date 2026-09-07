# NodeZkConfigProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-node-zk-config-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-node-zk-config-provider.md) / NodeZkConfigProvider

# Class: NodeZkConfigProvider\<K>

Implementation of [ZKConfigProvider](#) that reads the keys and zkIR from the local filesystem.

## Extends[​](#extends "Direct link to Extends")

* [`ZKConfigProvider`](#)<`K`>

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### K[​](#k "Direct link to K")

`K` *extends* `string`

The type of the circuit ID used by the provider.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new NodeZkConfigProvider**<`K`>(`directory`): `NodeZkConfigProvider`<`K`>

#### Parameters[​](#parameters "Direct link to Parameters")

##### directory[​](#directory "Direct link to directory")

`string`

The path to the base directory containing the key and ZKIR subdirectories.

#### Returns[​](#returns "Direct link to Returns")

`NodeZkConfigProvider`<`K`>

#### Overrides[​](#overrides "Direct link to Overrides")

`ZKConfigProvider<K>.constructor`

## Properties[​](#properties "Direct link to Properties")

### directory[​](#directory-1 "Direct link to directory")

> `readonly` **directory**: `string`

The path to the base directory containing the key and ZKIR subdirectories.

## Methods[​](#methods "Direct link to Methods")

### asKeyMaterialProvider()[​](#askeymaterialprovider "Direct link to asKeyMaterialProvider()")

> **asKeyMaterialProvider**(): `KeyMaterialProvider`

#### Returns[​](#returns-1 "Direct link to Returns")

`KeyMaterialProvider`

#### Inherited from[​](#inherited-from "Direct link to Inherited from")

`ZKConfigProvider.asKeyMaterialProvider`

***

### get()[​](#get "Direct link to get()")

> **get**(`circuitId`): `Promise`<`ZKConfig`<`K`>>

Retrieves all zero-knowledge artifacts produced by `compactc` compiler for the given circuit.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### circuitId[​](#circuitid "Direct link to circuitId")

`K`

The circuit ID of the artifacts to retrieve.

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<`ZKConfig`<`K`>>

#### Inherited from[​](#inherited-from-1 "Direct link to Inherited from")

`ZKConfigProvider.get`

***

### getProverKey()[​](#getproverkey "Direct link to getProverKey()")

> **getProverKey**(`circuitId`): `Promise`<`ProverKey`>

[ZKConfigProvider.getProverKey](#)

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### circuitId[​](#circuitid-1 "Direct link to circuitId")

`K`

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<`ProverKey`>

#### Overrides[​](#overrides-1 "Direct link to Overrides")

`ZKConfigProvider.getProverKey`

***

### getVerifierKey()[​](#getverifierkey "Direct link to getVerifierKey()")

> **getVerifierKey**(`circuitId`): `Promise`<`VerifierKey`>

[ZKConfigProvider.getVerifierKey](#)

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### circuitId[​](#circuitid-2 "Direct link to circuitId")

`K`

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<`VerifierKey`>

#### Overrides[​](#overrides-2 "Direct link to Overrides")

`ZKConfigProvider.getVerifierKey`

***

### getVerifierKeys()[​](#getverifierkeys "Direct link to getVerifierKeys()")

> **getVerifierKeys**(`circuitIds`): `Promise`<\[`K`, `VerifierKey`]\[]>

Retrieves the verifier keys produced by `compactc` compiler for the given circuits.

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### circuitIds[​](#circuitids "Direct link to circuitIds")

`K`\[]

The circuit IDs of the verifier keys to retrieve.

#### Returns[​](#returns-5 "Direct link to Returns")

`Promise`<\[`K`, `VerifierKey`]\[]>

#### Inherited from[​](#inherited-from-2 "Direct link to Inherited from")

`ZKConfigProvider.getVerifierKeys`

***

### getZKIR()[​](#getzkir "Direct link to getZKIR()")

> **getZKIR**(`circuitId`): `Promise`<`ZKIR`>

[ZKConfigProvider.getZKIR](#)

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### circuitId[​](#circuitid-3 "Direct link to circuitId")

`K`

#### Returns[​](#returns-6 "Direct link to Returns")

`Promise`<`ZKIR`>

#### Overrides[​](#overrides-3 "Direct link to Overrides")

`ZKConfigProvider.getZKIR`
