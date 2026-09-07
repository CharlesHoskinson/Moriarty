# ContractStateProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / ContractStateProvider

# Interface: ContractStateProvider

A user-provided interface for fetching the public state of a contract at a given block hash. Used exclusively to retrieve the state of cross-contract call targets at runtime. Assumes state returned is the post-block evaluation contract state.

The `parentBlockHash` value in [CircuitContext](/api-reference/compact-runtime/interfaces/CircuitContext.md) is used for as the `blockHash` argument.

## Methods[​](#methods "Direct link to Methods")

### getContractState()[​](#getcontractstate "Direct link to getContractState()")

```
getContractState(blockHash, address): Promise<ContractState | undefined>;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### blockHash[​](#blockhash "Direct link to blockHash")

`string`

##### address[​](#address "Direct link to address")

`string`

#### Returns[​](#returns "Direct link to Returns")

`Promise`<[`ContractState`](/api-reference/compact-runtime/classes/ContractState.md) | `undefined`>
