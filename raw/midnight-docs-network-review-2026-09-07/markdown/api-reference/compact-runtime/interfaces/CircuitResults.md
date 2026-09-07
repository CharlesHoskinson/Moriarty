# CircuitResults

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / CircuitResults

# Interface: CircuitResults\<PS, R>

The results of the call to a Compact circuit

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PS[​](#ps "Direct link to PS")

`PS` = `any`

### R[​](#r "Direct link to R")

`R` = `any`

## Properties[​](#properties "Direct link to Properties")

### context[​](#context "Direct link to context")

```
context: CircuitContext<PS>;
```

The updated context after the circuit execution, that can be used to inform further runs

***

### gasCost[​](#gascost "Direct link to gasCost")

```
gasCost: RunningCost;
```

The gas consumption of the circuit execution

***

### result[​](#result "Direct link to result")

```
result: R;
```

The primary result, as returned from Compact
