# createCircuitMaintenanceTxInterfaces

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / createCircuitMaintenanceTxInterfaces

# Function: createCircuitMaintenanceTxInterfaces()

> **createCircuitMaintenanceTxInterfaces**<`C`>(`providers`, `compiledContract`, `contractAddress`): [`CircuitMaintenanceTxInterfaces`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitMaintenanceTxInterfaces.md)<`C`>

Creates a [CircuitMaintenanceTxInterfaces](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitMaintenanceTxInterfaces.md).

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Any`

## Parameters[​](#parameters "Direct link to Parameters")

### providers[​](#providers "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`>

The providers to use to build transactions.

### compiledContract[​](#compiledcontract "Direct link to compiledContract")

`CompiledContract`<`C`, `any`>

The contract to use to execute circuits.

### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

The ledger address of the contract.

## Returns[​](#returns "Direct link to Returns")

[`CircuitMaintenanceTxInterfaces`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitMaintenanceTxInterfaces.md)<`C`>
