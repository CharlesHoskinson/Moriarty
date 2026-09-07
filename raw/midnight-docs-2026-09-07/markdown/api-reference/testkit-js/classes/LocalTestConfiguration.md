# LocalTestConfiguration

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

Configuration class for local test environment implementing EnvironmentConfiguration

## Implements[​](#implements "Direct link to Implements")

* [`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new LocalTestConfiguration**(`ports`): `LocalTestConfiguration`

Creates a new LocalTestConfiguration instance

#### Parameters[​](#parameters "Direct link to Parameters")

##### ports[​](#ports "Direct link to ports")

[`ComponentPortsConfiguration`](/api-reference/testkit-js/type-aliases/ComponentPortsConfiguration.md)

Object containing port numbers for each component

#### Returns[​](#returns "Direct link to Returns")

`LocalTestConfiguration`

## Properties[​](#properties "Direct link to Properties")

### faucet[​](#faucet "Direct link to faucet")

> `readonly` **faucet**: `string` | `undefined`

Optional URL for the faucet service to obtain test tokens

#### Implementation of[​](#implementation-of "Direct link to Implementation of")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md).[`faucet`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md#faucet)

***

### indexer[​](#indexer "Direct link to indexer")

> `readonly` **indexer**: `string`

URL of the indexer HTTP endpoint

#### Implementation of[​](#implementation-of-1 "Direct link to Implementation of")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md).[`indexer`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md#indexer)

***

### indexerWS[​](#indexerws "Direct link to indexerWS")

> `readonly` **indexerWS**: `string`

WebSocket URL for the indexer service

#### Implementation of[​](#implementation-of-2 "Direct link to Implementation of")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md).[`indexerWS`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md#indexerws)

***

### networkId[​](#networkid "Direct link to networkId")

> `readonly` **networkId**: `string`

Network identifier

#### Implementation of[​](#implementation-of-3 "Direct link to Implementation of")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md).[`networkId`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md#networkid)

***

### node[​](#node "Direct link to node")

> `readonly` **node**: `string`

URL of the blockchain node

#### Implementation of[​](#implementation-of-4 "Direct link to Implementation of")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md).[`node`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md#node)

***

### nodeWS[​](#nodews "Direct link to nodeWS")

> `readonly` **nodeWS**: `string`

WebSocket URL for the blockchain node

#### Implementation of[​](#implementation-of-5 "Direct link to Implementation of")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md).[`nodeWS`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md#nodews)

***

### proofServer[​](#proofserver "Direct link to proofServer")

> `readonly` **proofServer**: `string`

URL of the proof generation server

#### Implementation of[​](#implementation-of-6 "Direct link to Implementation of")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md).[`proofServer`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md#proofserver)

***

### walletNetworkId[​](#walletnetworkid "Direct link to walletNetworkId")

> `readonly` **walletNetworkId**: `string`

Wallet Network identifier

#### Implementation of[​](#implementation-of-7 "Direct link to Implementation of")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md).[`walletNetworkId`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md#walletnetworkid)
