# createContractMaintenanceTxInterface

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / createContractMaintenanceTxInterface

# Function: createContractMaintenanceTxInterface()

> **createContractMaintenanceTxInterface**<`C`>(`providers`, `compiledContract`, `contractAddress`): [`ContractMaintenanceTxInterface`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/interfaces/ContractMaintenanceTxInterface.md)

Creates a [ContractMaintenanceTxInterface](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/interfaces/ContractMaintenanceTxInterface.md).

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Any`

## Parameters[​](#parameters "Direct link to Parameters")

### providers[​](#providers "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)

The providers to use to build transactions.

### compiledContract[​](#compiledcontract "Direct link to compiledContract")

`CompiledContract`<`C`, `any`>

### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

The ledger address of the contract.

## Returns[​](#returns "Direct link to Returns")

[`ContractMaintenanceTxInterface`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/interfaces/ContractMaintenanceTxInterface.md)
