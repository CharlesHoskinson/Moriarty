# ConnectedAPI

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/dapp-connector-api v4.0.1**](/api-reference/dapp-connector.md)

***

[@midnight-ntwrk/dapp-connector-api](/api-reference/dapp-connector/globals.md) / ConnectedAPI

# Type Alias: ConnectedAPI

> **ConnectedAPI** = [`WalletConnectedAPI`](/api-reference/dapp-connector/type-aliases/WalletConnectedAPI.md) & [`HintUsage`](/api-reference/dapp-connector/type-aliases/HintUsage.md)

Connected API. It allows DApp to perform a range ofactions on the wallet after it is connected. Specifically the operations provided are:

* interaction with wallet - [WalletConnectedAPI](/api-reference/dapp-connector/type-aliases/WalletConnectedAPI.md) covers those
* hint usage of methods to the wallet (to help with permissions management)
