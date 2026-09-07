# createCircuitMaintenanceTxInterface

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / createCircuitMaintenanceTxInterface

# Function: createCircuitMaintenanceTxInterface()

> **createCircuitMaintenanceTxInterface**<`C`, `PCK`>(`providers`, `circuitId`, `compiledContract`, `contractAddress`): [`CircuitMaintenanceTxInterface`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitMaintenanceTxInterface.md)

Creates a [CircuitMaintenanceTxInterface](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitMaintenanceTxInterface.md).

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `string`

## Parameters[​](#parameters "Direct link to Parameters")

### providers[​](#providers "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`, `PCK`>

The providers to use to create and submit transactions.

### circuitId[​](#circuitid "Direct link to circuitId")

`PCK`

The circuit ID the interface is for.

### compiledContract[​](#compiledcontract "Direct link to compiledContract")

`CompiledContract`<`C`, `any`>

### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

The address of the deployed contract for which this interface is being created.

## Returns[​](#returns "Direct link to Returns")

[`CircuitMaintenanceTxInterface`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitMaintenanceTxInterface.md)
