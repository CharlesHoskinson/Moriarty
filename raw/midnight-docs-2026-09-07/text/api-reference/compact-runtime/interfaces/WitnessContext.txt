# WitnessContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / WitnessContext

# Interface: WitnessContext\<L, PS>

The external information accessible from within a Compact witness call

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### L[​](#l "Direct link to L")

`L` = `any`

### PS[​](#ps "Direct link to PS")

`PS` = `any`

## Properties[​](#properties "Direct link to Properties")

### contractAddress[​](#contractaddress "Direct link to contractAddress")

```
readonly contractAddress: string;
```

The address of the contract being called

***

### ledger[​](#ledger "Direct link to ledger")

```
readonly ledger: L;
```

The projected ledger state, if the transaction were to run against the ledger state as you locally see it currently

***

### privateState[​](#privatestate "Direct link to privateState")

```
readonly privateState: PS;
```

The current private state for the contract
