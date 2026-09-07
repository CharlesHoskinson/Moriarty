# SubmitTxProviders

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / SubmitTxProviders

# Type Alias: SubmitTxProviders\<C, PCK>

> **SubmitTxProviders**<`C`, `PCK`> = `Omit`<[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`, `PCK`>, `"privateStateProvider"`>

Providers required to submit an unproven deployment transaction. Since [submitTx](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitTx.md) doesn't manipulate private state, the private state provider can be omitted.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `Contract.ProvableCircuitId`<`C`>
