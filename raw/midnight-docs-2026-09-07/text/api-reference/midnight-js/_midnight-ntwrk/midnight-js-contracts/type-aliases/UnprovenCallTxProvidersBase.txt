# UnprovenCallTxProvidersBase

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / UnprovenCallTxProvidersBase

# Type Alias: UnprovenCallTxProvidersBase

> **UnprovenCallTxProvidersBase** = `Pick`<[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md), `"zkConfigProvider"` | `"publicDataProvider"` | `"walletProvider"`>

The minimum set of providers needed to create a call transaction, the ZK artifact provider and a wallet. By defining this type, users can choose to omit a private state provider if they're creating a call transaction for a contract with no private state.
