# createCallTxOptions

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / createCallTxOptions

# Function: createCallTxOptions()

> **createCallTxOptions**<`C`, `PCK`>(`compiledContract`, `circuitId`, `contractAddress`, `privateStateId`, `additionalCoinEncPublicKeyMappings`, `args`): [`CallTxOptions`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptions.md)<`C`, `PCK`>

Creates a [CallTxOptions](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptions.md) object from various data.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `string`

## Parameters[​](#parameters "Direct link to Parameters")

### compiledContract[​](#compiledcontract "Direct link to compiledContract")

`CompiledContract`<`C`, `any`>

### circuitId[​](#circuitid "Direct link to circuitId")

`PCK`

### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

### privateStateId[​](#privatestateid "Direct link to privateStateId")

`string` | `undefined`

### additionalCoinEncPublicKeyMappings[​](#additionalcoinencpublickeymappings "Direct link to additionalCoinEncPublicKeyMappings")

`ReadonlyMap`<`string`, `string`> | `undefined`

### args[​](#args "Direct link to args")

`CircuitParameters`<`C`, `PCK`>

## Returns[​](#returns "Direct link to Returns")

[`CallTxOptions`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptions.md)<`C`, `PCK`>
