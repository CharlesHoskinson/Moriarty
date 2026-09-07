# createCircuitCallTxInterface

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / createCircuitCallTxInterface

# Function: createCircuitCallTxInterface()

> **createCircuitCallTxInterface**<`C`>(`providers`, `compiledContract`, `contractAddress`, `privateStateId`): [`CircuitCallTxInterface`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitCallTxInterface.md)<`C`>

Creates a circuit call transaction interface for a contract.

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

### privateStateId[​](#privatestateid "Direct link to privateStateId")

`string` | `undefined`

The identifier of the state of the witnesses of the contract.

## Returns[​](#returns "Direct link to Returns")

[`CircuitCallTxInterface`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitCallTxInterface.md)<`C`>
