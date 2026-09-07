# DAppConnectorProvingAPI

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-dapp-connector-proof-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-dapp-connector-proof-provider.md) / DAppConnectorProvingAPI

# Type Alias: DAppConnectorProvingAPI

> **DAppConnectorProvingAPI** = `Pick`<`WalletConnectedAPI`, `"getProvingProvider"`>

Minimal interface required from the DApp Connector wallet.

## Remarks[​](#remarks "Direct link to Remarks")

Picks only WalletConnectedAPI.getProvingProvider | getProvingProvider from the full wallet API, allowing loose coupling between the framework and the wallet implementation.
