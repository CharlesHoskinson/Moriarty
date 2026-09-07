# DeployedContract

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / DeployedContract

# Type Alias: DeployedContract\<C>

> **DeployedContract**<`C`> = [`FoundContract`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FoundContract.md)<`C`> & `object`

Interface for a contract that has been deployed to the blockchain.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### deployTxData[​](#deploytxdata "Direct link to deployTxData")

> `readonly` **deployTxData**: [`FinalizedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedDeployTxData.md)<`C`>

Data resulting from the deployment transaction that created this contract. The information in a deployTxData contains additional private information that does not exist in [FoundContract.deployTxData](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FoundContract.md#deploytxdata) because certain private data is only available to the deployer of a contract.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`
