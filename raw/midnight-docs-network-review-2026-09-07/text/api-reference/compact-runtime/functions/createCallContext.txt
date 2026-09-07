# createCallContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / createCallContext

# Function: createCallContext()

```
function createCallContext<PS>(

   circuitId, 

   contractAddress, 

   coinPublicKeyOrZswapState, 

   contractState, 

   privateState, 

   maybeTime?, 

   parentBlockHash?, 

caller?): CallContext<PS>;
```

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PS[​](#ps "Direct link to PS")

`PS`

## Parameters[​](#parameters "Direct link to Parameters")

### circuitId[​](#circuitid "Direct link to circuitId")

`string`

### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

### coinPublicKeyOrZswapState[​](#coinpublickeyorzswapstate "Direct link to coinPublicKeyOrZswapState")

`string` | [`EncodedZswapLocalState`](/api-reference/compact-runtime/interfaces/EncodedZswapLocalState.md) | [`EncodedCoinPublicKey`](/api-reference/compact-runtime/interfaces/EncodedCoinPublicKey.md) | [`ZswapLocalState`](/api-reference/compact-runtime/interfaces/ZswapLocalState.md)

### contractState[​](#contractstate "Direct link to contractState")

[`ContractState`](/api-reference/compact-runtime/classes/ContractState.md) | [`StateValue`](/api-reference/compact-runtime/classes/StateValue.md) | [`ChargedState`](/api-reference/compact-runtime/classes/ChargedState.md)

### privateState[​](#privatestate "Direct link to privateState")

`PS`

### maybeTime?[​](#maybetime "Direct link to maybeTime?")

`number`

### parentBlockHash?[​](#parentblockhash "Direct link to parentBlockHash?")

`string`

### caller?[​](#caller "Direct link to caller?")

[`PublicAddress`](/api-reference/compact-runtime/type-aliases/PublicAddress.md)

## Returns[​](#returns "Direct link to Returns")

`CallContext`<`PS`>
