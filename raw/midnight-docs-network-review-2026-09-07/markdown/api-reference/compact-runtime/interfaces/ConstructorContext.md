# ConstructorContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / ConstructorContext

# Interface: ConstructorContext\<PS>

Passed to the constructor of a contract. Used to compute the contract's initial ledger state.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PS[​](#ps "Direct link to PS")

`PS` = `any`

## Properties[​](#properties "Direct link to Properties")

### initialPrivateState[​](#initialprivatestate "Direct link to initialPrivateState")

```
initialPrivateState: PS;
```

The private state we would like to use to execute the contract's constructor.

***

### initialZswapLocalState[​](#initialzswaplocalstate "Direct link to initialZswapLocalState")

```
initialZswapLocalState: EncodedZswapLocalState;
```

An initial (usually empty) Zswap local state to use to execute the contract's constructor.
