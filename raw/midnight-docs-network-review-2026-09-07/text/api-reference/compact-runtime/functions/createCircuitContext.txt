# createCircuitContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / createCircuitContext

# Function: createCircuitContext()

```
function createCircuitContext<PS>(

   circuitId, 

   contractAddress, 

   coinPublicKeyOrZswapState, 

   contractState, 

   privateState, 

   stateProvider?, 

   gasLimit?, 

   costModel?, 

   time?, 

   parentBlockHash?, 

reentrancyGuard?): CircuitContext<PS>;
```

Entry point for constructing the [CircuitContext](/api-reference/compact-runtime/interfaces/CircuitContext.md) to pass as an argument to a circuit. Always use this function to set up the initial circuit context.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PS[​](#ps "Direct link to PS")

`PS`

## Parameters[​](#parameters "Direct link to Parameters")

### circuitId[​](#circuitid "Direct link to circuitId")

`string`

The name of the circuit being executed.

### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

The address of the contract defining the circuit being executed.

### coinPublicKeyOrZswapState[​](#coinpublickeyorzswapstate "Direct link to coinPublicKeyOrZswapState")

The initial Zswap local state information - used for tracking shielded coin transfers.

`string` | [`EncodedZswapLocalState`](/api-reference/compact-runtime/interfaces/EncodedZswapLocalState.md) | [`EncodedCoinPublicKey`](/api-reference/compact-runtime/interfaces/EncodedCoinPublicKey.md) | [`ZswapLocalState`](/api-reference/compact-runtime/interfaces/ZswapLocalState.md)

### contractState[​](#contractstate "Direct link to contractState")

The initial ledger state to execute the contract again - most often a snapshot fetched from the chain.

[`ContractState`](/api-reference/compact-runtime/classes/ContractState.md) | [`StateValue`](/api-reference/compact-runtime/classes/StateValue.md) | [`ChargedState`](/api-reference/compact-runtime/classes/ChargedState.md)

### privateState[​](#privatestate "Direct link to privateState")

`PS`

The initial witness / private state to execute the contract again - most often a snapshot fetched from local storage.

### stateProvider?[​](#stateprovider "Direct link to stateProvider?")

[`ContractStateProvider`](/api-reference/compact-runtime/interfaces/ContractStateProvider.md)

The provider to use to dynamically fetch on-chain contract state. This is only used to execute cross-contract calls, and is not needed if the circuit being executed does not perform any cross-contract calls.

### gasLimit?[​](#gaslimit "Direct link to gasLimit?")

[`RunningCost`](/api-reference/compact-runtime/type-aliases/RunningCost.md)

The maximum gas this contract should consume.

### costModel?[​](#costmodel "Direct link to costModel?")

[`CostModel`](/api-reference/compact-runtime/classes/CostModel.md)

The model capturing how much ledger operations cost.

### time?[​](#time "Direct link to time?")

`number`

The current time. Used to execute the block time related kernel operations.

### parentBlockHash?[​](#parentblockhash "Direct link to parentBlockHash?")

`string`

The hash of the block the transaction is being built on. Also passed to [ContractStateProvider](/api-reference/compact-runtime/interfaces/ContractStateProvider.md) to fetch the correct contract states when executing cross-contract calls.

### reentrancyGuard?[​](#reentrancyguard "Direct link to reentrancyGuard?")

`boolean`

When `true`, cross-contract calls that re-enter a contract already executing on the call stack (`A -> A`, or `A -> B -> A`) throw instead of running. On by default; pass `false` to opt out.

## Returns[​](#returns "Direct link to Returns")

[`CircuitContext`](/api-reference/compact-runtime/interfaces/CircuitContext.md)<`PS`>
