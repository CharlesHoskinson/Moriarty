[![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)

Hyperliquid Docs](/hyperliquid-docs)

`⌘Ctrl``k`

* [Hyperliquid Docs](/hyperliquid-docs)
* [Builder Tools](/hyperliquid-docs/builder-tools)
* [Support](/hyperliquid-docs/support)

For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). This page is also available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm.md).

Copy

On this page

1. [Hyperliquid Docs](/hyperliquid-docs)
2. [For developers](/hyperliquid-docs/for-developers)

HyperEVM
========

The HyperEVM consists of EVM blocks built as part of Hyperliquid's execution, inheriting all security from HyperBFT consensus. HYPE is the native gas token on the HyperEVM. To move HYPE from HyperCore to HyperEVM, send HYPE to `0x2222222222222222222222222222222222222222`. See the instructions in [Native Transfers](/hyperliquid-docs/for-developers/hyperevm/hypercore-less-than-greater-than-hyperevm-transfers) for more details on how this works.

Note that there are currently no official frontend components of the EVM. Users can build their own frontends or port over existing EVM applications. All interaction with the EVM happens through the JSON-RPC. For example, users can add the chain to their wallets by entering the RPC URL and chain ID. There is currently no websocket JSON-RPC support for the RPC at `rpc.hyperliquid.xyz/evm`but other RPC implementations may support it.

The HyperEVM uses the Cancun hardfork without blobs. In particular, EIP-1559 is enabled on the HyperEVM. Base fees are burned as usual, implemented in the standard way where the burned fees are removed from the total EVM supply. Unlike most other EVM chains, priority fees are also burned because the HyperEVM uses HyperBFT consensus. The burned priority fees are sent to the zero address's EVM balance.

On both mainnet and testnet, HYPE on the HyperEVM has 18 decimals. A few differences between testnet and mainnet HyperEVM are highlighted below:

### Mainnet

Chain ID: 999

JSON-RPC endpoint: `https://rpc.hyperliquid.xyz/evm` for mainnet

### Testnet

Chain ID: 998

JSON-RPC endpoint: `https://rpc.hyperliquid-testnet.xyz/evm`

### Additional notes

Pre-EIP-155 transactions are accepted as with many EVM chains. Users generally should not use wallets that have previously sent pre-EIP-155 transactions.

Error code 10055 represents errors occurring at the boundary between HyperCore and HyperEVM. Examples include nonce errors, insufficient funds, duplicate transaction hashes, or underpriced replacement transactions.

[PreviousHIP-4 deployer actions](/hyperliquid-docs/for-developers/api/hip-4-deployer-actions)[NextDual-block architecture](/hyperliquid-docs/for-developers/hyperevm/dual-block-architecture)

Last updated 1 day ago

* [Mainnet](#mainnet)
* [Testnet](#testnet)
* [Additional notes](#additional-notes)