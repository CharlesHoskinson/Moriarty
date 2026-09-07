# ContractStateObservableConfig

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / ContractStateObservableConfig

# Type Alias: ContractStateObservableConfig

> **ContractStateObservableConfig** = [`TxIdConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/TxIdConfig.md) | [`BlockHashConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/BlockHashConfig.md) | [`BlockHeightConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/BlockHeightConfig.md) & `object` | [`Latest`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/Latest.md) | [`All`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/All.md)

The configuration for a contract state observable. The corresponding observables may begin at different places (e.g. after a specific transaction identifier / block height) depending on the configuration, but all state updates after the beginning are always included.
