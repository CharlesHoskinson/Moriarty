# ContractProviders

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / ContractProviders

# Type Alias: ContractProviders\<C, PCK, PS>

> **ContractProviders**<`C`, `PCK`, `PS`> = `MidnightProviders`<`PCK`, `PrivateStateId`, `PS`>

Convenience type for representing the set of providers necessary to use a given contract.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any` = `Contract.Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `Contract.ProvableCircuitId`<`C`> = `Contract.ProvableCircuitId`<`C`>

### PS[​](#ps "Direct link to PS")

`PS` = `Contract.PrivateState`<`C`>
