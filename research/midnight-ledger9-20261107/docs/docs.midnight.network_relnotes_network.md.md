<!-- source: https://docs.midnight.network/relnotes/network.md fetched_at: 2026-11-07T20:48:42Z status: 200 untrusted -->

> For the complete documentation index, see [llms.txt](https://docs.midnight.network/llms.txt)

# Environments and endpoints

Preprod and Mainnet indexer and RPC moved to Blockfrost

The Midnight-hosted Mainnet endpoints `rpc.mainnet.midnight.network` and `indexer.mainnet.midnight.network` were retired at 22:00 UTC on 9 October 2026. [Blockfrost](https://blockfrost.io) is now the primary public provider of the Mainnet indexer and node RPC, and every request needs a Blockfrost **Midnight Mainnet** project token. To move an existing DApp, see [Migrating from the Midnight-hosted endpoints](https://docs.midnight.network/guides/networks-and-environments.md#migrating-from-the-midnight-hosted-mainnet-endpoints).

The Midnight-hosted Preprod endpoints `rpc.preprod.midnight.network` and `indexer.preprod.midnight.network` were retired at 22:00 UTC on 9 October 2026. Blockfrost now provides the Preprod indexer and node RPC too, and every request needs a Blockfrost **Midnight Preprod** project token. A Preprod token doesn't work on Mainnet, or the other way round.

As of 13 October 2026, both retired indexer hostnames answer every request with HTTP `410` and an `ENDPOINT_DECOMMISSIONED` error. Both retired RPC endpoints still answer requests, but they can stop at any time, so don't use them.

The Preview endpoints stay the same.

Midnight uses dedicated network services such as RPC nodes, indexer API, and proof server to power DApp development and interaction across the Preview, Preprod, and Mainnet environments.

## Preview[​](#preview "Direct link to Preview")

The Preview environment is intended for early-stage development and experimentation. Use these endpoints to connect your DApp to Midnight Preview environment.

Preview runs ledger 9. On 6 November 2026, Preview was reset as a new chain with the same endpoints and network ID. The old chain's contracts, balances, and DUST registrations are gone. Request tNIGHT from the faucet again, register for DUST again, and redeploy your contracts.

| Service           | URL                                                       |
| ----------------- | --------------------------------------------------------- |
| Node RPC          | `https://rpc.preview.midnight.network`                    |
| Indexer (GraphQL) | `https://indexer.preview.midnight.network/api/v4/graphql` |
| Faucet UI         | <https://midnight-tmnight-preview.nethermind.dev/>        |
| Block explorers   | <https://preview.midnightexplorer.com/>                   |
|                   | <https://midnight-preview.subscan.io/>                    |
|                   | <https://explorer.1am.xyz/?network=preview>               |

## Preprod[​](#preprod "Direct link to Preprod")

The Preprod environment is designed for final testing before mainnet deployment. Use these endpoints to connect your DApp to Midnight Preprod environment. Blockfrost provides the public Preprod indexer and node RPC: create a **Midnight Preprod** project on [blockfrost.io](https://blockfrost.io) and append `?project_id=<token>` to each Node RPC and Indexer URL. See [Blockfrost: the Preprod and Mainnet indexer and RPC provider](https://docs.midnight.network/guides/networks-and-environments.md#blockfrost-the-mainnet-indexer-and-rpc-provider).

| Service              | URL                                                |
| -------------------- | -------------------------------------------------- |
| Node RPC             | `https://rpc.midnight-preprod.blockfrost.io`       |
| Node RPC (WebSocket) | `wss://rpc.midnight-preprod.blockfrost.io`         |
| Indexer (GraphQL)    | `https://midnight-preprod.blockfrost.io/api/v0`    |
| Indexer (WebSocket)  | `wss://midnight-preprod.blockfrost.io/api/v0/ws`   |
| Faucet UI            | <https://midnight-tmnight-preprod.nethermind.dev/> |
| Block explorers      | <https://preprod.midnightexplorer.com/>            |
|                      | <https://midnight-preprod.subscan.io/>             |
|                      | <https://explorer.1am.xyz/?network=preprod>        |

## Mainnet[​](#mainnet "Direct link to Mainnet")

The Mainnet environment is the production network. Use these endpoints to connect your DApp to Midnight Mainnet. The public Mainnet indexer and node RPC are provided by Blockfrost: create a **Midnight Mainnet** project on [blockfrost.io](https://blockfrost.io) and append `?project_id=<token>` to each Node RPC and Indexer URL. See [Blockfrost: the Preprod and Mainnet indexer and RPC provider](https://docs.midnight.network/guides/networks-and-environments.md#blockfrost-the-mainnet-indexer-and-rpc-provider).

Mainnet runs ledger 9

On 6 November 2026, Mainnet moved to ledger 9 in the ledger 9 network upgrade. Preview runs ledger 9 too, so test your ledger 9 DApp on Preview before you move it to Mainnet. Preprod stays on ledger 8.1.3 until a later upgrade.

The Mainnet endpoints on this page stay the same after the upgrade.

To use Mainnet after the upgrade, your DApp needs Wallet SDK 2.0.0 and Midnight.js 5.0.0. For what changed and what to do, see [Ledger 9 upgrade](https://docs.midnight.network/relnotes/ledger-9.md).

| Service              | URL                                              |
| -------------------- | ------------------------------------------------ |
| Node RPC             | `https://rpc.midnight-mainnet.blockfrost.io`     |
| Node RPC (WebSocket) | `wss://rpc.midnight-mainnet.blockfrost.io`       |
| Indexer (GraphQL)    | `https://midnight-mainnet.blockfrost.io/api/v0`  |
| Indexer (WebSocket)  | `wss://midnight-mainnet.blockfrost.io/api/v0/ws` |
| Block explorers      | <https://midnightexplorer.com/>                  |
|                      | <https://midnight.subscan.io/>                   |
|                      | <https://explorer.1am.xyz>                       |
| cNgD DApp            | <https://midnight-dust-mainnet.nethermind.io/>   |
