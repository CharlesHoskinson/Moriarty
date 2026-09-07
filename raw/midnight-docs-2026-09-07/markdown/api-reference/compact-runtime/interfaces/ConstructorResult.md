# ConstructorResult

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / ConstructorResult

# Interface: ConstructorResult\<PS>

The result of executing a contract constructor.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PS[​](#ps "Direct link to PS")

`PS` = `any`

## Properties[​](#properties "Direct link to Properties")

### currentContractState[​](#currentcontractstate "Direct link to currentContractState")

```
currentContractState: ContractState;
```

The contract's initial ledger (public state).

***

### currentPrivateState[​](#currentprivatestate "Direct link to currentPrivateState")

```
currentPrivateState: PS;
```

The contract's initial private state. Potentially different from the private state passed in [ConstructorContext](/api-reference/compact-runtime/interfaces/ConstructorContext.md).

***

### currentZswapLocalState[​](#currentzswaplocalstate "Direct link to currentZswapLocalState")

```
currentZswapLocalState: EncodedZswapLocalState;
```

The contract's initial Zswap local state. Potentially includes outputs created in the contract's constructor.
