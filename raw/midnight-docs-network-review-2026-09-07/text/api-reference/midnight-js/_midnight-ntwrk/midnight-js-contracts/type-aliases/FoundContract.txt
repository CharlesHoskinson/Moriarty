# FoundContract

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / FoundContract

# Type Alias: FoundContract\<C>

> **FoundContract**<`C`> = `object`

Base type for a deployed contract that has been found on the blockchain.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

## Properties[​](#properties "Direct link to Properties")

### callTx[​](#calltx "Direct link to callTx")

> `readonly` **callTx**: [`CircuitCallTxInterface`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitCallTxInterface.md)<`C`>

Interface for creating call transactions for a contract.

***

### circuitMaintenanceTx[​](#circuitmaintenancetx "Direct link to circuitMaintenanceTx")

> `readonly` **circuitMaintenanceTx**: [`CircuitMaintenanceTxInterfaces`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CircuitMaintenanceTxInterfaces.md)<`C`>

An interface for creating maintenance transactions for circuits defined in the contract that was deployed.

***

### contractMaintenanceTx[​](#contractmaintenancetx "Direct link to contractMaintenanceTx")

> `readonly` **contractMaintenanceTx**: [`ContractMaintenanceTxInterface`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/interfaces/ContractMaintenanceTxInterface.md)

Interface for creating maintenance transactions for the contract that was deployed.

***

### deployTxData[​](#deploytxdata "Direct link to deployTxData")

> `readonly` **deployTxData**: [`FinalizedDeployTxDataBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedDeployTxDataBase.md)<`C`>

Data for the finalized deploy transaction corresponding to this contract.
