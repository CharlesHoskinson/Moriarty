# crossContractCall

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / crossContractCall

# Function: crossContractCall()

```
function crossContractCall(

   circuitContext, 

   calleeModule, 

   calleeCircuitId, 

   calleeAddress, 

   calleeIsPure, 

   callerProofData, ...

args): Promise<any>;
```

**`Internal`**

Calls a circuit defined in another contract from the currently executing contract and returns the result.

## Parameters[​](#parameters "Direct link to Parameters")

### circuitContext[​](#circuitcontext "Direct link to circuitContext")

[`CircuitContext`](/api-reference/compact-runtime/interfaces/CircuitContext.md)

The current circuit context.

### calleeModule[​](#calleemodule "Direct link to calleeModule")

`Module`

The callee module containing TS executables.

### calleeCircuitId[​](#calleecircuitid "Direct link to calleeCircuitId")

`string`

The name of the circuit to be called in the contract to be called.

### calleeAddress[​](#calleeaddress "Direct link to calleeAddress")

`string`

The address of the contract to be called.

### calleeIsPure[​](#calleeispure "Direct link to calleeIsPure")

`boolean`

A flag indicating whether the circuit being called is pure.

### callerProofData[​](#callerproofdata "Direct link to callerProofData")

[`PartialProofData`](/api-reference/compact-runtime/interfaces/PartialProofData.md)

The proof data instance created when the caller circuit was initialized.

### args[​](#args "Direct link to args")

...`any`\[]

The arguments to the circuit to be called.

## Returns[​](#returns "Direct link to Returns")

`Promise`<`any`>
